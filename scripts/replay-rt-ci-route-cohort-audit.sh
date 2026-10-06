#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
manifest="${1:-$repo_root/docs/reports/rt-ci-route-cohort-audit-replay-manifest-2026-10-05.json}"

command -v jq >/dev/null 2>&1 || { echo "jq is required" >&2; exit 2; }

jq -e '
  .schema_version == 1
  and .audit_date == "2026-10-05"
  and .classification_scope == "retained 37 candidates only; route eligibility evidence, not a timing result"
  and (.classifier_replay_policy_version | type == "string" and length > 0)
  and (.classifier_blob_sha | test("^[0-9a-f]{40}$"))
  and (.candidates | length == 37)
  and all(.candidates[];
    (.pr_number | type == "number")
    and (.route_run_id | type == "string" and length > 0)
    and (.workspace_run_id | type == "string" and length > 0)
    and (.head_sha | test("^[0-9a-f]{40}$"))
    and (.head_tree_sha | test("^[0-9a-f]{40}$"))
    and (.changed_paths | type == "array" and length > 0)
    and (.route_mode | type == "string" and length > 0)
    and (.route_reason | type == "string" and length > 0)
  )
' "$manifest" >/dev/null || {
    echo "FAIL: route-audit manifest is missing required bounded candidate data" >&2
    exit 1
}

classifier_path="$(jq -r '.classifier_snapshot_path' "$manifest")"
classifier="$repo_root/$classifier_path"
expected_blob="$(jq -r '.classifier_blob_sha' "$manifest")"
actual_blob="$(git hash-object "$classifier")"
if [ "$actual_blob" != "$expected_blob" ]; then
    echo "FAIL: classifier snapshot blob does not match the recorded source revision" >&2
    exit 1
fi

temp_dir="$(mktemp -d)"
cleanup() {
    rm -rf -- "$temp_dir"
}
trap cleanup EXIT
chmod 0700 "$temp_dir"

replayed=0
while IFS= read -r candidate; do
    pr_number="$(jq -r '.pr_number' <<<"$candidate")"
    route_run_id="$(jq -r '.route_run_id' <<<"$candidate")"
    event="$(jq -r '.event' <<<"$candidate")"
    ref="$(jq -r '.ref' <<<"$candidate")"
    changed_paths_file="$temp_dir/changed-paths.txt"
    jq -r '.changed_paths[]' <<<"$candidate" > "$changed_paths_file"

    actual="$(bash "$classifier" "$event" "$ref" "$changed_paths_file" false false)" || {
        echo "FAIL: classifier replay failed for PR #$pr_number, route run $route_run_id" >&2
        exit 1
    }
    actual_mode="$(jq -r '.mode' <<<"$actual")"
    expected_mode="$(jq -r '.route_mode' <<<"$candidate")"
    actual_reason="$(jq -r '.reason' <<<"$actual")"
    expected_reason="$(jq -r '.route_reason' <<<"$candidate")"
    actual_policy_version="$(jq -r '.policy_version' <<<"$actual")"
    expected_policy_version="$(jq -r '.classifier_replay_policy_version' "$manifest")"
    actual_paths="$(jq -cS '.changed_paths' <<<"$actual")"
    expected_paths="$(jq -cS '.changed_paths' <<<"$candidate")"

    if [ "$actual_mode" != "$expected_mode" ] || [ "$actual_reason" != "$expected_reason" ] || [ "$actual_policy_version" != "$expected_policy_version" ] || [ "$actual_paths" != "$expected_paths" ]; then
        echo "FAIL: replay differs for PR #$pr_number, route run $route_run_id" >&2
        exit 1
    fi
    replayed=$((replayed + 1))
done < <(jq -c '.candidates[]' "$manifest")

[ "$replayed" -eq 37 ] || {
    echo "FAIL: replayed $replayed candidates; expected 37 retained candidates" >&2
    exit 1
}

jq -n \
    --arg classifier_commit "$(jq -r '.source_commit' "$manifest")" \
    --arg classifier_blob "$actual_blob" \
    --arg classifier_policy_version "$(jq -r '.classifier_replay_policy_version' "$manifest")" \
    --argjson replayed "$replayed" \
    --argjson full_count "$(jq '[.candidates[] | select(.route_mode == "full_pr_matrix")] | length' "$manifest")" \
    --argjson passive_docs_count "$(jq '[.candidates[] | select(.route_mode == "passive_docs_only")] | length' "$manifest")" \
    --argjson dependency_closure_count "$(jq '[.candidates[] | select(.route_mode == "dependency_closure")] | length' "$manifest")" \
    '{schema_version:1,status:"passed",candidates_replayed:$replayed,classifier_commit:$classifier_commit,classifier_blob_sha:$classifier_blob,classifier_policy_version:$classifier_policy_version,route_counts:{full_pr_matrix:$full_count,passive_docs_only:$passive_docs_count,dependency_closure:$dependency_closure_count},scope:"retained historical candidates only; not timing evidence"}'
