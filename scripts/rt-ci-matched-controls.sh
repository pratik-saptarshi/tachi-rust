#!/usr/bin/env bash
set -euo pipefail

WORKFLOW="${1:-rust-workspace.yml}"
LIMIT="${2:-100}"

if [ "${RT_CI_USE_RTK:-false}" = "true" ]; then
  if ! command -v rtk >/dev/null 2>&1; then
    echo "rtk is required when RT_CI_USE_RTK=true" >&2
    exit 1
  fi
  gh() { rtk gh "$@"; }
fi

for command_name in gh jq; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "$command_name is required for matched-control evidence collection" >&2
    exit 1
  fi
done

repo="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
runs_file="$(mktemp)"
candidates_file="$(mktemp)"
tmp_root="$(mktemp -d)"
trap 'rm -f "$runs_file" "$candidates_file"; rm -rf "$tmp_root"' EXIT

printf '[]\n' > "$candidates_file"
if ! gh run list --workflow "$WORKFLOW" --limit "$LIMIT" \
  --json databaseId,attempt,conclusion,event,createdAt,startedAt,updatedAt,headBranch,url \
  > "$runs_file"; then
  echo "Unable to list workflow runs for '$WORKFLOW'." >&2
  exit 1
fi

record_candidate() {
  local json="$1"
  jq --argjson candidate "$json" '. + [$candidate]' "$candidates_file" > "$candidates_file.new"
  mv "$candidates_file.new" "$candidates_file"
}

while IFS= read -r run_json; do
  base="$(jq -c '{run_id:(.databaseId|tostring),run_attempt:.attempt,event,conclusion,head_branch:.headBranch,created_at:.createdAt,started_at:.startedAt,completed_at:.updatedAt,run_url:.url}' <<<"$run_json")"
  event="$(jq -r '.event' <<<"$run_json")"
  case "$event" in
    pull_request|workflow_dispatch) ;;
    *) continue ;;
  esac

  conclusion="$(jq -r '.conclusion // ""' <<<"$run_json")"
  attempt="$(jq -r '.attempt // 0' <<<"$run_json")"
  reason=""
  if [ "$conclusion" != "success" ]; then
    reason="run conclusion is not success"
  elif [ "$attempt" != "1" ]; then
    reason="rerun attempt is ambiguous"
  fi

  run_id="$(jq -r '.databaseId' <<<"$run_json")"
  route_file=""
  if [ -z "$reason" ]; then
    run_dir="$tmp_root/$run_id"
    mkdir -p "$run_dir"
    if ! gh run download "$run_id" --name route-decision --dir "$run_dir" >/dev/null 2>&1; then
      reason="route decision artifact is missing or unavailable"
    else
      route_file="$(find "$run_dir" -type f -name route.json -print -quit)"
      if [ -z "$route_file" ]; then
        reason="route decision artifact has no route.json"
      fi
    fi
  fi

  if [ -n "$reason" ]; then
    record_candidate "$(jq -cn --argjson base "$base" --arg reason "$reason" '$base + {eligible:false,rejection_reason:$reason}')"
    continue
  fi

  route="$(cat "$route_file")"
  route_run_id="$(jq -r '.run_id // ""' <<<"$route")"
  route_attempt="$(jq -r '.run_attempt // 0' <<<"$route")"
  route_event="$(jq -r '.event // ""' <<<"$route")"
  head_sha="$(jq -r '.head_sha // ""' <<<"$route")"
  workflow_file_revision="$(jq -r '.workflow_file_revision // ""' <<<"$route")"
  execution_workflow_file_revision="$(jq -r '.execution_workflow_file_revision // ""' <<<"$route")"
  execution_sha="$(jq -r '.execution_sha // ""' <<<"$route")"
  route_tree_sha="$(jq -r '.tree_sha // ""' <<<"$route")"
  control_tree_sha="$(jq -r '.control_tree_sha // ""' <<<"$route")"
  control_pr_number="$(jq -r '.control_pr_number // 0' <<<"$route")"
  control_pr_head_sha="$(jq -r '.control_pr_head_sha // ""' <<<"$route")"
  classifier_revision="$(jq -r '.classifier_revision // ""' <<<"$route")"
  path_producer_revision="$(jq -r '.path_producer_revision // ""' <<<"$route")"
  runner_definition="$(jq -r '.runner_definition // ""' <<<"$route")"
  route_mode="$(jq -r '.mode // ""' <<<"$route")"
  forced="$(jq -r '.force_full_requested // false' <<<"$route")"

  reason=""
  if [ "$route_run_id" != "$run_id" ] || [ "$route_attempt" != "$attempt" ] || [ "$route_event" != "$event" ]; then
    reason="route artifact run provenance does not match the workflow run"
  elif [ -z "$head_sha" ] || [ -z "$workflow_file_revision" ] || [ -z "$execution_workflow_file_revision" ] || [ "$workflow_file_revision" != "$execution_workflow_file_revision" ] || [ -z "$execution_sha" ] || [ -z "$route_tree_sha" ] || [ -z "$classifier_revision" ] || [ -z "$path_producer_revision" ] || [ -z "$runner_definition" ]; then
    reason="route artifact is missing source, execution-tree, workflow-content, classifier, path-producer, or runner provenance"
  elif [ "$control_pr_number" != "0" ] && { [ "$event" != "pull_request" ] || [ "$execution_sha" != "$control_tree_sha" ] || [ "$control_pr_number" != "$(jq -r '.pr_number // 0' <<<"$route")" ] || [ "$control_pr_head_sha" != "$head_sha" ] || [ -z "$control_pr_head_sha" ] || [ "$route_mode" != "full_pr_matrix" ] || [ "$forced" != "true" ]; }; then
    reason="forced-full PR-label control did not execute its recorded PR merge commit, PR head, and full route"
  elif [ "$event" = "workflow_dispatch" ]; then
    reason="manual dispatches are not matched controls; use the pull_request ci-full-control label"
  elif [ "$event" = "pull_request" ] && [ "$control_pr_number" = "0" ] && [ "$route_mode" != "passive_docs_only" ] && [ "$route_mode" != "dependency_closure" ]; then
    reason="PR route is not an eligible optimized shape"
  fi

  if [ -n "$reason" ]; then
    record_candidate "$(jq -cn --argjson base "$base" --argjson route "$route" --arg reason "$reason" '$base + {route:$route,eligible:false,rejection_reason:$reason}')"
    continue
  fi

  tree_sha="$route_tree_sha"
  runner_inventory="$(jq -cn --arg definition "$runner_definition" '{route:$definition}')"
  runner_dir="$run_dir/runners"
  mkdir -p "$runner_dir"
  if ! gh run download "$run_id" --pattern 'runner-provenance-*' --dir "$runner_dir" >/dev/null 2>&1 \
    && { [ "$route_mode" != "passive_docs_only" ] || [ "$event" != "pull_request" ]; }; then
    record_candidate "$(jq -cn --argjson base "$base" --argjson route "$route" '$base + {route:$route,eligible:false,rejection_reason:"measured-job runner provenance artifacts are missing or unavailable"}')"
    continue
  fi
  runner_files=("")
  while IFS= read -r runner_file; do
    runner_files+=("$runner_file")
  done < <(find "$runner_dir" -type f -name runner.json -print)
  if [ "${#runner_files[@]}" -le 1 ] && { [ "$route_mode" != "passive_docs_only" ] || [ "$event" != "pull_request" ]; }; then
    record_candidate "$(jq -cn --argjson base "$base" --argjson route "$route" '$base + {route:$route,eligible:false,rejection_reason:"measured-job runner provenance is empty"}')"
    continue
  fi
  for runner_file in "${runner_files[@]}"; do
    [ -n "$runner_file" ] || continue
    runner_json="$(cat "$runner_file")"
    runner_key="$(jq -r '.job_key // ""' <<<"$runner_json")"
    runner_value="$(jq -r '.runner_definition // ""' <<<"$runner_json")"
    if [ -z "$runner_key" ] || [ -z "$runner_value" ]; then
      reason="measured-job runner provenance artifact is incomplete"
      break
    fi
    runner_inventory="$(jq -c --arg key "$runner_key" --arg value "$runner_value" '. + {($key):$value}' <<<"$runner_inventory")"
  done
  if [ -n "$reason" ]; then
    record_candidate "$(jq -cn --argjson base "$base" --argjson route "$route" --arg reason "$reason" '$base + {route:$route,eligible:false,rejection_reason:$reason}')"
    continue
  fi

  if ! durations="$(jq -cn \
    --arg created_at "$(jq -r '.createdAt' <<<"$run_json")" \
    --arg started_at "$(jq -r '.startedAt' <<<"$run_json")" \
    --arg completed_at "$(jq -r '.updatedAt' <<<"$run_json")" \
    '($created_at|fromdateiso8601) as $created | ($started_at|fromdateiso8601) as $started | ($completed_at|fromdateiso8601) as $completed | {queue_duration_ms:(($started-$created)*1000),execution_duration_ms:(($completed-$started)*1000)}')"; then
    record_candidate "$(jq -cn --argjson base "$base" --argjson route "$route" '$base + {route:$route,eligible:false,rejection_reason:"run timestamps are missing or invalid"}')"
    continue
  fi

  record_candidate "$(jq -cn \
    --argjson base "$base" \
    --argjson route "$route" \
    --arg tree_sha "$tree_sha" \
    --argjson durations "$durations" \
    --argjson runner_inventory "$runner_inventory" \
    '{run_id:$base.run_id,run_attempt:$base.run_attempt,event:$base.event,conclusion:$base.conclusion,head_branch:$base.head_branch,created_at:$base.created_at,started_at:$base.started_at,completed_at:$base.completed_at,run_url:$base.run_url,pr_number:$route.pr_number,head_sha:$route.head_sha,execution_sha:$route.execution_sha,tree_sha:$tree_sha,control_pr_number:$route.control_pr_number,control_tree_sha:$route.control_tree_sha,control_pr_head_sha:$route.control_pr_head_sha,route_mode:$route.mode,route_reason:$route.reason,changed_paths:$route.changed_paths,policy_version:$route.policy_version,force_full_requested:$route.force_full_requested,workflow_file_revision:$route.workflow_file_revision,classifier_revision:$route.classifier_revision,path_producer_revision:$route.path_producer_revision,runner_definition:$route.runner_definition,runner_inventory:$runner_inventory,queue_duration_ms:$durations.queue_duration_ms,execution_duration_ms:$durations.execution_duration_ms,eligible:true}')"
done < <(jq -c '.[] | select(.event == "pull_request" or .event == "workflow_dispatch")' "$runs_file")

jq -n --slurpfile candidates "$candidates_file" '{candidates:$candidates[0]}' \
  | jq -f "$(dirname "$0")/rt-ci-matched-controls.jq"
