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
  and (.network == false) and (.max_iterations | type == "number" and . >= 3 and . <= 32)
  and (.timeout_seconds | type == "number" and . >= 1 and . <= 30)
  and (.allowlisted_commands == ["printf", "echo"])
  and (.cases | type == "array" and length == 5)
  and ((.cases | map(.id) | length) == (.cases | map(.id) | unique | length))
  and (.cases | map(.id) | sort == ["approval", "cancel", "circuit_breaker", "denial", "timeout"])
  and (all(.cases[]; (.expected | IN("approved", "denied", "timed_out", "cancelled", "blocked"))))
  and (all($root.cases[]; . as $case | if $case.id == "denial" then ($case.expected == "denied" and ($root.allowlisted_commands | index($case.tool) == null)) else ($root.allowlisted_commands | index($case.tool) != null) end))
' "$FIXTURE" >/dev/null || { echo 'agentic-replay: unsafe or invalid fixture' >&2; exit 2; }

FAKE_TOOL="$ROOT_DIR/tests/fixtures/agentic/fake-tool.sh"
[ -x "$FAKE_TOOL" ] || { echo 'agentic-replay: missing scripted fake tool' >&2; exit 2; }
MAX_ITERATIONS="$(jq -r '.max_iterations' "$FIXTURE")"
TIMEOUT_SECONDS="$(jq -r '.timeout_seconds' "$FIXTURE")"
EXECUTION_TMP="$(mktemp "${TMPDIR:-/tmp}/agentic-replay-execution.XXXXXXXX")"
TOOL_OUTPUT="$(mktemp "${TMPDIR:-/tmp}/agentic-replay-tool.XXXXXXXX")"
EXECUTION_NEXT_TMP=""
chmod 0600 "$EXECUTION_TMP" "$TOOL_OUTPUT"
printf '[]\n' > "$EXECUTION_TMP"
AUDIT_TMP=""
cleanup_temp_files() {
    rm -f -- "$EXECUTION_TMP" "$TOOL_OUTPUT" "$EXECUTION_NEXT_TMP" "$AUDIT_TMP"
}
trap cleanup_temp_files EXIT

record_execution() {
    local id="$1" invoked="$2" status="$3" attempts="$4"
    EXECUTION_NEXT_TMP="$(mktemp "${TMPDIR:-/tmp}/agentic-replay-execution-next.XXXXXXXX")"
    chmod 0600 "$EXECUTION_NEXT_TMP"
    jq --arg id "$id" --arg status "$status" --arg tool "scripted-fake-tool" \
        --argjson invoked "$invoked" --argjson attempts "$attempts" \
        '. + [{id:$id,tool:$tool,invoked:$invoked,status:$status,attempts:$attempts}]' \
        "$EXECUTION_TMP" > "$EXECUTION_NEXT_TMP"
    mv -- "$EXECUTION_NEXT_TMP" "$EXECUTION_TMP"
    EXECUTION_NEXT_TMP=""
}

terminate_tree() {
    local root="$1" signal="$2" child
    for child in $(pgrep -P "$root" 2>/dev/null || true); do
        terminate_tree "$child" "$signal"
    done
    kill -"$signal" "-$root" 2>/dev/null || true
    kill -"$signal" "$root" 2>/dev/null || true
}

while IFS= read -r case_id; do
    case "$case_id" in
        approval)
            "$FAKE_TOOL" approval >"$TOOL_OUTPUT" 2>&1 || { echo 'agentic-replay: approval fake tool failed' >&2; exit 1; }
            record_execution "$case_id" true approved 1
            ;;
        denial)
            record_execution "$case_id" false denied 0
            ;;
        timeout)
            "$FAKE_TOOL" timeout >"$TOOL_OUTPUT" 2>&1 &
            tool_pid=$!
            deadline=$(( $(date +%s) + TIMEOUT_SECONDS ))
            timeout_hit=false
            while kill -0 "$tool_pid" 2>/dev/null; do
                if [ "$(date +%s)" -ge "$deadline" ]; then
                    terminate_tree "$tool_pid" TERM
                    sleep 0.1
                    terminate_tree "$tool_pid" KILL
                    timeout_hit=true
                    break
                fi
                sleep 0.05
            done
            wait "$tool_pid" 2>/dev/null || true
            [ "$timeout_hit" = true ] || { echo 'agentic-replay: timeout fake tool completed unexpectedly' >&2; exit 1; }
            record_execution "$case_id" true timed_out 1
            ;;
        cancel)
            "$FAKE_TOOL" cancel >"$TOOL_OUTPUT" 2>&1 &
            tool_pid=$!
            sleep 0.1
            terminate_tree "$tool_pid" TERM
            sleep 0.1
            wait "$tool_pid" 2>/dev/null || true
            terminate_tree "$tool_pid" KILL
            record_execution "$case_id" true cancelled 1
            ;;
        circuit_breaker)
            attempts=0
            while [ "$attempts" -lt 3 ] && [ "$attempts" -lt "$MAX_ITERATIONS" ]; do
                "$FAKE_TOOL" circuit_breaker >"$TOOL_OUTPUT" 2>&1 || true
                attempts=$((attempts + 1))
            done
            [ "$attempts" -eq 3 ] || { echo 'agentic-replay: circuit breaker budget too small' >&2; exit 1; }
            record_execution "$case_id" true blocked "$attempts"
            ;;
        *)
            echo "agentic-replay: unsupported case: $case_id" >&2
            exit 2
            ;;
    esac
done < <(jq -r '.cases[].id' "$FIXTURE")

payload="$(jq -c '
  . as $root
  | {schema_version:1,audit_sink:"audit.jsonl",promotion_status:"skipped",promotion_note:"harness present but not promotion-ready until E2E-COV-010.2 behavioral replay review",model:$root.model,seed:$root.seed,network_policy:"deny",network_used:false,max_iterations:$root.max_iterations,timeout_seconds:$root.timeout_seconds,
     cases:[$root.cases[] | . as $case | (if $case.id == "approval" and ($root.allowlisted_commands | index($case.tool) != null) then "approved" elif $case.id == "denial" and ($root.allowlisted_commands | index($case.tool) == null) then "denied" elif $case.id == "timeout" then "timed_out" elif $case.id == "cancel" then "cancelled" elif $case.id == "circuit_breaker" then "blocked" else "invalid" end) as $outcome | (if $case.id == "approval" then ["approved","executing","completed"] elif $case.id == "denial" then ["denied"] elif $case.id == "timeout" then ["executing","timed_out"] elif $case.id == "cancel" then ["executing","cancelled"] elif $case.id == "circuit_breaker" then ["executing","circuit_open","blocked"] else [] end) as $transitions | (first($executions[0][] | select(.id == $case.id))) as $execution | {id:$case.id,status:(if $outcome == $case.expected and $execution.status == $outcome then "passed" else "failed" end),outcome:$outcome,expected:$case.expected,audit_id:("audit-" + ($root.seed|tostring) + "-" + $case.id),audit_events:([{event:"request",id:$case.id},{event:"decision",outcome:$outcome}] + ($transitions | map({event:"transition",transition:.})) + [{event:"result",correlated_to:("audit-" + ($root.seed|tostring) + "-" + $case.id)}]),tool:$case.tool,command_executed:$execution.invoked,execution:$execution}]}
  | . + {status:(if (.cases | all(.status == "passed")) then "passed" else "failed" end)}
' --slurpfile executions "$EXECUTION_TMP" "$FIXTURE")"

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
if [ "$OUTPUT" != /dev/stdout ]; then
    reject_symlink_path "$OUTPUT"
fi
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
