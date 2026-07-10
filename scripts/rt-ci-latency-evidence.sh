#!/usr/bin/env bash
set -euo pipefail

WORKFLOW="${1:-rust-workspace.yml}"
BRANCH="${2:-main}"
LIMIT="${3:-40}"

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required for remote CI evidence collection: https://cli.github.com/manual" >&2
  exit 1
fi

if ! command -v node >/dev/null 2>&1; then
  echo "node is required for median calculation in rt-ci-latency-evidence.sh" >&2
  exit 1
fi

tmp_json="$(mktemp)"
trap 'rm -f "$tmp_json"' EXIT

if ! gh run list \
  --workflow "$WORKFLOW" \
  --branch "$BRANCH" \
  --status completed \
  --json databaseId,createdAt,startedAt,updatedAt,displayTitle \
  --limit "$LIMIT" > "$tmp_json"; then
  echo "Unable to fetch runs from GitHub Actions (network/auth blocked or repo unavailable)." >&2
  exit 1
fi

if [ ! -s "$tmp_json" ]; then
  echo "No completed runs found for workflow '$WORKFLOW' on branch '$BRANCH'." >&2
  exit 0
fi

node - "$tmp_json" "$WORKFLOW" "$BRANCH" <<'NODE'
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
