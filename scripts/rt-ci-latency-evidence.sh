#!/usr/bin/env bash
set -euo pipefail

WORKFLOWS="${1:-rust-workspace.yml}"
BRANCH="${2:-main}"
LIMIT="${3:-40}"
EVENT_FILTER="${4:-}"

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "$cmd is required for RT-CI evidence collection: https://github.com" >&2
    exit 1
  fi
}

require_cmd gh
require_cmd node
require_cmd jq

collect_workflow_medians() {
  local workflow="$1"
  local tmp_json
  local -a query
  tmp_json="$(mktemp)"
  trap 'rm -f "$tmp_json" "$tmp_json.new"' RETURN

  # run list query for workflow timing medians
  query=(
    "gh"
    "run"
    "list"
    "--workflow"
    "$workflow"
    "--status"
    "completed"
    "--json"
    "databaseId,createdAt,startedAt,updatedAt,displayTitle,event,headBranch"
    "--limit"
    "$LIMIT"
  )

  if [ -z "$EVENT_FILTER" ] && [ "$workflow" != "ci-route-observe.yml" ] && [ -n "$BRANCH" ]; then
    query+=(--branch "$BRANCH")
  fi

  if [ -n "$EVENT_FILTER" ]; then
    query+=(--jq "map(select(.event == \"$EVENT_FILTER\"))")
  fi

  if ! "${query[@]}" > "$tmp_json"; then
    echo "Unable to fetch runs for workflow '$workflow' on branch '$BRANCH'." >&2
    echo "check your internet connection, auth scope, or workflow availability."
    return 1
  fi

  if [ ! -s "$tmp_json" ] || [ "$(cat "$tmp_json")" = "[]" ]; then
    echo "No completed runs found for workflow '$workflow' on branch '$BRANCH'." >&2
    return 0
  fi

  if [ "$EVENT_FILTER" = "pull_request" ]; then
    jq -c "map(select(.event == \"pull_request\"))" "$tmp_json" > "$tmp_json.new"
    if [ -s "$tmp_json.new" ]; then
      mv "$tmp_json.new" "$tmp_json"
    else
      rm -f "$tmp_json.new"
    fi
  fi

  if [ ! -s "$tmp_json" ] || [ "$(cat "$tmp_json")" = "[]" ]; then
    echo "No completed runs found for workflow '$workflow' with event '${EVENT_FILTER:-all}' on branch '$BRANCH'." >&2
    return 0
  fi

  node - "$tmp_json" "$workflow" "$BRANCH" <<'NODE'
const fs = require("fs");

const runs = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const workflow = process.argv[3];
const branch = process.argv[4];

const parsed = runs
  .map((run) => {
    if (!run.createdAt || !run.startedAt || !run.updatedAt) {
      return null;
    }
    const queueMs = new Date(run.startedAt).getTime() - new Date(run.createdAt).getTime();
    const runMs = new Date(run.updatedAt).getTime() - new Date(run.startedAt).getTime();
    return { ...run, queue_ms: queueMs, run_ms: runMs };
  })
  .filter(Boolean);

if (parsed.length === 0) {
  console.log(`No runs with timing data found for workflow='${workflow}', branch='${branch}'.`);
  process.exit(0);
}

const queueMs = parsed.map((run) => run.queue_ms).sort((a, b) => a - b);
const runMs = parsed.map((run) => run.run_ms).sort((a, b) => a - b);

const median = (arr) => {
  if (arr.length === 0) return null;
  const mid = Math.floor(arr.length / 2);
  if (arr.length % 2 === 1) return arr[mid];
  return (arr[mid - 1] + arr[mid]) / 2;
};

const maybeMinMax = (arr) => `${arr[0]}..${arr[arr.length - 1]}`;

console.log(`workflow=${workflow}`);
console.log(`branch=${branch}`);
console.log(`sample_size=${parsed.length}`);
console.log(`run_med_ms=${median(runMs).toFixed(0)}`);
console.log(`queue_med_ms=${median(queueMs).toFixed(0)}`);
console.log(`queue_range_ms=${maybeMinMax(queueMs)}`);
console.log(`run_range_ms=${maybeMinMax(runMs)}`);
NODE
}

check_branch_protection() {
  local repo
  local protection_json
  local protection_err
  local required

  repo="$(gh repo view --json nameWithOwner -q .nameWithOwner || printf "<unknown>")"
  if [ -z "$repo" ] || [ "$repo" = "null" ]; then
    repo="<unknown>"
  fi
  protection_json="$(mktemp)"
  protection_err="$(mktemp)"
  trap 'rm -f "$protection_json" "$protection_err"' RETURN

  if ! gh api "repos/${repo}/branches/${BRANCH}/protection" > "$protection_json" 2> "$protection_err"; then
    echo "branch_protection=${repo}/${BRANCH}: unavailable"
    cat "$protection_err" >&2
    return 1
  fi

  required="$(jq -r '.required_status_checks.contexts // [] | sort | join(",")' "$protection_json")"
  echo "branch_protection=${repo}/${BRANCH}: enabled"
  if [ -n "$required" ]; then
    echo "branch_protection_required_checks=${required}"
  else
    echo "branch_protection_required_checks=<none>"
  fi
}

FAILED=0
FAILED_WF=0

echo "RT-CI evidence run"
echo "branch=${BRANCH}"
echo "workflows=${WORKFLOWS}"
echo "---"

for workflow in $(echo "$WORKFLOWS" | tr ',' '\n' | tr ' ' '\n' | sed '/^$/d'); do
  if ! collect_workflow_medians "$workflow"; then
    FAILED_WF=$((FAILED_WF + 1))
  fi
  echo "---"
done

if ! check_branch_protection; then
  FAILED=1
fi

if [ "$FAILED_WF" -gt 0 ]; then
  FAILED=1
fi

if [ "$FAILED" -eq 1 ]; then
  exit 1
fi
