#!/usr/bin/env bash
set -euo pipefail

RUN_ID="${1:-}"
COMMIT="${2:-}"
REPO="${GH_REPO:-}"
OUTPUT_DIR="${3:-}"

if [ -z "$RUN_ID" ] || [ -z "$COMMIT" ]; then
    echo "usage: $0 RUN_ID COMMIT|auto [OUTPUT_DIR]" >&2
    exit 2
fi
command -v gh >/dev/null 2>&1 || { echo "gh is required" >&2; exit 2; }
command -v jq >/dev/null 2>&1 || { echo "jq is required" >&2; exit 2; }

EXPECTED_WORKFLOW="${CI_TIMING_EXPECTED_WORKFLOW:-rust workspace tests}"
ALLOW_LEGACY="${CI_TIMING_ALLOW_LEGACY_ARTIFACTS:-0}"

if [ -z "$REPO" ]; then
    REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
fi

# Artifact self-consistency is insufficient evidence: bind the download to the
# completed, successful GitHub run before accepting any timing payload.
# The hosted producer records GITHUB_REF as runner.ref; the verifier checks
# that serialized ref below so push and pull-request provenance cannot be
# substituted across event types.
run_metadata="$(gh run view "$RUN_ID" --repo "$REPO" --json databaseId,workflowName,event,status,conclusion,headBranch,headSha,attempt)" || {
    echo "FAIL: unable to retrieve GitHub run metadata for $RUN_ID" >&2
    exit 1
}
jq -e \
    --arg run_id "$RUN_ID" \
    --arg workflow "$EXPECTED_WORKFLOW" \
    '.databaseId | tostring == $run_id' \
    <<<"$run_metadata" >/dev/null || {
    echo "FAIL: GitHub run metadata ID does not match requested run" >&2
    exit 1
}
jq -e \
    --arg workflow "$EXPECTED_WORKFLOW" \
    '(.workflowName == $workflow)
     and (.status == "completed")
     and (.conclusion == "success")
     and (.event == "push" or .event == "pull_request")
     and (.attempt | type == "number" and . >= 1)' \
    <<<"$run_metadata" >/dev/null || {
    echo "FAIL: GitHub run metadata is not an accepted successful timing run" >&2
    exit 1
}

run_event="$(jq -r '.event' <<<"$run_metadata")"
run_attempt="$(jq -r '.attempt' <<<"$run_metadata")"
run_head_sha="$(jq -r '.headSha' <<<"$run_metadata")"
run_head_branch="$(jq -r '.headBranch' <<<"$run_metadata")"

root="${OUTPUT_DIR:-$(mktemp -d)}"
cleanup() {
    if [ -z "${OUTPUT_DIR:-}" ]; then
        rm -rf -- "$root"
    fi
}
trap cleanup EXIT
mkdir -p -- "$root"
chmod 0700 "$root"

artifacts=(
    ci-timing-package-tachi-core
    ci-timing-package-tachi-mcp
    ci-timing-package-tachi-cli
    ci-timing-package-tachi-shell
    ci-timing-package-tachi-desktop
    ci-timing-shell-shell-smoke
    ci-timing-shell-shell-init
    ci-timing-shell-shell-integration
)

verified=0
observed_commit=""
for artifact in "${artifacts[@]}"; do
    destination="$root/$artifact"
    mkdir -p -- "$destination"
    gh run download "$RUN_ID" --repo "$REPO" --name "$artifact" --dir "$destination" >/dev/null
    files=("$destination"/*.json)
    [ "${#files[@]}" -eq 1 ] || {
        echo "FAIL: $artifact did not contain exactly one JSON result" >&2
        exit 1
    }
    case "$artifact" in
        ci-timing-package-*)
            expected_unit="cargo-test-${artifact#ci-timing-package-}"
            expected_stage="compile-and-test"
            ;;
        ci-timing-shell-*)
            expected_unit="shell-tests-${artifact#ci-timing-shell-}"
            expected_stage="test-slice"
            ;;
        *)
            echo "FAIL: unknown timing artifact mapping: $artifact" >&2
            exit 1
            ;;
    esac
    artifact_commit="$(jq -r '.commit // empty' "${files[0]}")"
    if [ -z "$artifact_commit" ]; then
        echo "FAIL: $artifact has no commit provenance" >&2
        exit 1
    fi
    if [ "$COMMIT" = "auto" ]; then
        if [ -z "$observed_commit" ]; then
            observed_commit="$artifact_commit"
        elif [ "$artifact_commit" != "$observed_commit" ]; then
            echo "FAIL: timing artifacts disagree on commit provenance" >&2
            exit 1
        fi
    elif [ "$artifact_commit" != "$COMMIT" ]; then
        echo "FAIL: $artifact commit provenance does not match expected commit" >&2
        exit 1
    fi
jq -e --arg commit "$artifact_commit" --arg run_id "$RUN_ID" --arg event "$run_event" --arg workflow "$EXPECTED_WORKFLOW" --arg head_sha "$run_head_sha" --arg head_branch "$run_head_branch" --arg legacy "$ALLOW_LEGACY" --arg unit "$expected_unit" --arg stage "$expected_stage" --argjson attempt "$run_attempt" '
        type == "object"
        and .schema_version == 1
        and .commit == $commit
        and (.duration_ms | type == "number" and . >= 0)
        and (.runner.run_id | tostring) == $run_id
        and (.runner.event == $event)
        and (.runner.attempt | tonumber) == $attempt
        and .stage == $stage
        and .unit == $unit
        and (
            ($legacy == "1" and (.runner.workflow_name? == null))
            or (
                .runner.workflow_name == $workflow
                and .runner.head_sha == $commit
                and .runner.source_head_sha == $head_sha
                and (if $event == "push"
                     then (.runner.ref == ("refs/heads/" + $head_branch)
                           or .runner.ref == ("refs/tags/" + $head_branch))
                     else (.runner.ref | test("^refs/pull/[0-9]+/merge$"))
                     end)
            )
        )
    ' "${files[0]}" >/dev/null
    verified=$((verified + 1))
done

if [ "$COMMIT" = "auto" ]; then
    COMMIT="$observed_commit"
fi

if [ "$run_event" = "push" ] && [ "$COMMIT" != "$run_head_sha" ]; then
    echo "FAIL: push timing artifacts do not match the GitHub run head SHA" >&2
    exit 1
fi

jq -n --arg repo "$REPO" --arg run_id "$RUN_ID" --arg commit "$COMMIT" --argjson verified "$verified" \
    '{schema_version:1,repository:$repo,run_id:$run_id,commit:$commit,verified_artifacts:$verified,expected_artifacts:8,status:"passed"}'
