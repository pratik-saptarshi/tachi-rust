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

if [ -z "$REPO" ]; then
    REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
fi

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
    jq -e --arg commit "$artifact_commit" --arg run_id "$RUN_ID" '
        type == "object"
        and .schema_version == 1
        and .commit == $commit
        and (.duration_ms | type == "number" and . >= 0)
        and (.runner.run_id | tostring) == $run_id
        and (.stage == "compile-and-test" or .stage == "test-slice")
        and (.unit | type == "string" and length > 0)
    ' "${files[0]}" >/dev/null
    verified=$((verified + 1))
done

if [ "$COMMIT" = "auto" ]; then
    COMMIT="$observed_commit"
fi

jq -n --arg repo "$REPO" --arg run_id "$RUN_ID" --arg commit "$COMMIT" --argjson verified "$verified" \
    '{schema_version:1,repository:$repo,run_id:$run_id,commit:$commit,verified_artifacts:$verified,expected_artifacts:8,status:"passed"}'
