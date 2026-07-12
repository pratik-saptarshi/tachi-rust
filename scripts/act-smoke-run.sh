#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)"
FIXTURE="${ACT_SMOKE_FIXTURE:-$ROOT_DIR/tests/fixtures/act/pull-request.json}"
WORKFLOW="${ACT_SMOKE_WORKFLOW:-$ROOT_DIR/.github/workflows/ci-route-observe.yml}"
OUTPUT="${ACT_SMOKE_RUN_OUTPUT:-/dev/stdout}"
JOB="${ACT_SMOKE_JOB:-route-observe}"
PREFLIGHT_CREATED=false
if [ -n "${ACT_SMOKE_PREFLIGHT:-}" ]; then
    PREFLIGHT="$ACT_SMOKE_PREFLIGHT"
else
    PREFLIGHT="$(mktemp "${TMPDIR:-/tmp}/tachi-act-preflight.XXXXXX.json")"
    PREFLIGHT_CREATED=true
fi
cleanup() {
    if [ "$PREFLIGHT_CREATED" = true ]; then
        rm -f -- "$PREFLIGHT"
    fi
    return 0
}
trap cleanup EXIT
command -v jq >/dev/null 2>&1 || { echo 'act-smoke-run: jq is required' >&2; exit 2; }
[ -r "$FIXTURE" ] || { echo "act-smoke-run: missing fixture: $FIXTURE" >&2; exit 2; }
[ -r "$WORKFLOW" ] || { echo "act-smoke-run: missing workflow: $WORKFLOW" >&2; exit 2; }
case "$JOB" in *[!A-Za-z0-9._-]*|'') echo 'act-smoke-run: invalid job name' >&2; exit 2 ;; esac
grep -Fqx "  ${JOB}:" "$WORKFLOW" || { echo "act-smoke-run: job is not defined in workflow: $JOB" >&2; exit 2; }
jq -e '.pull_request.number and .pull_request.head.sha and .pull_request.base.ref' "$FIXTURE" >/dev/null || {
    echo "act-smoke-run: invalid synthetic pull-request fixture" >&2
    exit 2
}
ACT_SMOKE_OUTPUT="$PREFLIGHT" "$ROOT_DIR/scripts/act-smoke.sh" >/dev/null
status="$(jq -r '.status' "$PREFLIGHT")"
relative_path() {
    case "$1" in
        "$ROOT_DIR"/*) printf '%s' "${1#"$ROOT_DIR"/}" ;;
        *) printf '%s' "$1" ;;
    esac
}
if [ "$status" != READY ]; then
    payload="$(jq -n --arg status "$status" --arg job "$JOB" --arg fixture "$(relative_path "$FIXTURE")" --arg workflow "$(relative_path "$WORKFLOW")" --argjson preflight "$(cat "$PREFLIGHT")" \
        '{schema_version:1,status:$status,job:$job,workflow:$workflow,event_fixture:$fixture,preflight:$preflight,benchmark:null,side_effects:{workflow_invoked:false,release_or_security_steps:false,sarif_upload:false},cleanup:{verified:true}}')"
else
    echo 'act-smoke-run: available runtime requires explicit execution implementation and review' >&2
    exit 2
fi
if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    umask 077
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    printf '%s\n' "$payload" > "$OUTPUT"
fi
printf 'act-smoke-run status=%s job=%s\n' "$status" "$JOB" >&2
exit 0
