#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)"
FIXTURE="${ACT_SMOKE_FIXTURE:-$ROOT_DIR/tests/fixtures/act/pull-request.json}"
PREFLIGHT="${ACT_SMOKE_PREFLIGHT:-$(mktemp "${TMPDIR:-/tmp}/tachi-act-preflight.XXXXXX.json")}"
OUTPUT="${ACT_SMOKE_RUN_OUTPUT:-/dev/stdout}"
JOB="${ACT_SMOKE_JOB:-route}"
trap 'rm -f "$PREFLIGHT"' EXIT
command -v jq >/dev/null 2>&1 || { echo 'act-smoke-run: jq is required' >&2; exit 2; }
[ -r "$FIXTURE" ] || { echo "act-smoke-run: missing fixture: $FIXTURE" >&2; exit 2; }
jq -e '.pull_request.number and .pull_request.head.sha and .pull_request.base.ref' "$FIXTURE" >/dev/null || {
    echo "act-smoke-run: invalid synthetic pull-request fixture" >&2
    exit 2
}
"$ROOT_DIR/scripts/act-smoke.sh" > "$PREFLIGHT"
status="$(jq -r '.status' "$PREFLIGHT")"
if [ "$status" != READY ]; then
    payload="$(jq -n --arg status "$status" --arg job "$JOB" --arg fixture "tests/fixtures/act/pull-request.json" --argjson preflight "$(cat "$PREFLIGHT")" \
        '{schema_version:1,status:$status,job:$job,event_fixture:$fixture,preflight:$preflight,benchmark:null,side_effects:{workflow_invoked:false,release_or_security_steps:false,sarif_upload:false},cleanup:{verified:true}}')"
else
    echo 'act-smoke-run: available runtime requires explicit execution implementation and review' >&2
    exit 2
fi
if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    umask 077
    printf '%s\n' "$payload" > "$OUTPUT"
fi
printf 'act-smoke-run status=%s job=%s\n' "$status" "$JOB" >&2
exit 0
