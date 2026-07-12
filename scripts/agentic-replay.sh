#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)"
FIXTURE="${AGENTIC_REPLAY_FIXTURE:-$ROOT_DIR/tests/fixtures/agentic/replay.json}"
OUTPUT="${AGENTIC_REPLAY_OUTPUT:-/dev/stdout}"
AUDIT_OUTPUT="${AGENTIC_REPLAY_AUDIT_OUTPUT:-}"
command -v jq >/dev/null 2>&1 || { echo 'agentic-replay: jq is required' >&2; exit 2; }
[ -r "$FIXTURE" ] || { echo "agentic-replay: missing fixture: $FIXTURE" >&2; exit 2; }
jq -e '
  . as $root |
  .schema_version == 1 and .model == "scripted-fake" and (.seed | type == "number")
  and (.network == false) and (.max_iterations | type == "number" and . >= 1 and . <= 32)
  and (.timeout_seconds | type == "number" and . >= 1 and . <= 30)
  and (.allowlisted_commands == ["printf", "echo"])
  and (.cases | type == "array" and length == 5)
  and ((.cases | map(.id) | length) == (.cases | map(.id) | unique | length))
  and (.cases | map(.id) | sort == ["approval", "cancel", "circuit_breaker", "denial", "timeout"])
  and (all(.cases[]; (.expected | IN("approved", "denied", "timed_out", "cancelled", "blocked"))))
  and (all($root.cases[]; . as $case | if $case.id == "denial" then ($case.expected == "denied" and ($root.allowlisted_commands | index($case.tool) == null)) else ($root.allowlisted_commands | index($case.tool) != null) end))
' "$FIXTURE" >/dev/null || { echo 'agentic-replay: unsafe or invalid fixture' >&2; exit 2; }

payload="$(jq -c '
  . as $root
  | {schema_version:1,audit_sink:"audit.jsonl",promotion_status:"skipped",promotion_note:"harness present but not promotion-ready until E2E-COV-010.2 behavioral replay review",model:$root.model,seed:$root.seed,network_policy:"deny",network_used:false,max_iterations:$root.max_iterations,timeout_seconds:$root.timeout_seconds,
     cases:[$root.cases[] | . as $case | (if $case.id == "approval" and ($root.allowlisted_commands | index($case.tool) != null) then "approved" elif $case.id == "denial" and ($root.allowlisted_commands | index($case.tool) == null) then "denied" elif $case.id == "timeout" then "timed_out" elif $case.id == "cancel" then "cancelled" elif $case.id == "circuit_breaker" then "blocked" else "invalid" end) as $outcome | (if $case.id == "approval" then ["approved","executing","completed"] elif $case.id == "denial" then ["denied"] elif $case.id == "timeout" then ["executing","timed_out"] elif $case.id == "cancel" then ["executing","cancelled"] elif $case.id == "circuit_breaker" then ["executing","circuit_open","blocked"] else [] end) as $transitions | {id:$case.id,status:(if $outcome == $case.expected then "passed" else "failed" end),outcome:$outcome,expected:$case.expected,audit_id:("audit-" + ($root.seed|tostring) + "-" + $case.id),audit_events:([{event:"request",id:$case.id},{event:"decision",outcome:$outcome}] + ($transitions | map({event:"transition",transition:.})) + [{event:"result",correlated_to:("audit-" + ($root.seed|tostring) + "-" + $case.id)}]),tool:$case.tool,command_executed:false}]}
  | . + {status:(if (.cases | all(.status == "passed")) then "passed" else "failed" end)}
' "$FIXTURE")"

if [ -z "$AUDIT_OUTPUT" ]; then
    if [ "$OUTPUT" != /dev/stdout ]; then
        AUDIT_OUTPUT="$OUTPUT.audit.jsonl"
    else
        mkdir -p -- "$ROOT_DIR/target"
        AUDIT_OUTPUT="$ROOT_DIR/target/agentic-replay-audit-$$.jsonl"
    fi
fi
umask 077
reject_symlink_path() {
    local path="$1"
    [ ! -L "$path" ] || { echo "agentic-replay: symlink output paths are not allowed: $path" >&2; exit 2; }
}
reject_symlink_path "$OUTPUT"
reject_symlink_path "$AUDIT_OUTPUT"
if [ "$OUTPUT" != /dev/stdout ]; then
    mkdir -p -- "$(dirname -- "$OUTPUT")"
fi
mkdir -p -- "$(dirname -- "$AUDIT_OUTPUT")"
canonical_path() {
    local path="$1" dir base
    dir="$(dirname -- "$path")"
    base="$(basename -- "$path")"
    dir="$(CDPATH= cd -- "$dir" 2>/dev/null && pwd -P)" || return 1
    printf '%s/%s\n' "$dir" "$base"
}
output_path="$(canonical_path "$OUTPUT")" || { echo 'agentic-replay: cannot resolve output path' >&2; exit 2; }
audit_path="$(canonical_path "$AUDIT_OUTPUT")" || { echo 'agentic-replay: cannot resolve audit path' >&2; exit 2; }
[ "$output_path" != "$audit_path" ] || { echo 'agentic-replay: output and audit paths must differ' >&2; exit 2; }
AUDIT_TMP=""
cleanup_audit_tmp() {
    [ -z "$AUDIT_TMP" ] || rm -f -- "$AUDIT_TMP"
}
trap cleanup_audit_tmp EXIT
AUDIT_TMP="$(mktemp "$AUDIT_OUTPUT.tmp.XXXXXXXX")" || { echo 'agentic-replay: cannot create secure audit temporary file' >&2; exit 2; }
chmod 0600 "$AUDIT_TMP"
jq -c '.cases[] as $case | $case.audit_events[] | {audit_id:$case.audit_id,case_id:$case.id,audit_sink:"audit.jsonl"} + .' <<<"$payload" > "$AUDIT_TMP"
mv -- "$AUDIT_TMP" "$AUDIT_OUTPUT"
AUDIT_TMP=""

if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    umask 077
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    printf '%s\n' "$payload" > "$OUTPUT"
fi
status="$(jq -r '.status' <<<"$payload")"
printf 'agentic-replay status=%s cases=5 network=disabled\n' "$status" >&2
[ "$status" = passed ]
