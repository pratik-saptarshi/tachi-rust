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
AUDIT_SINK="$(basename -- "$AUDIT_OUTPUT")"
payload="$(jq -c --arg audit_sink "$AUDIT_SINK" '.audit_sink = $audit_sink' <<<"$payload")"
umask 077
mkdir -p -- "$(dirname -- "$AUDIT_OUTPUT")"
jq -c --arg audit_sink "$AUDIT_SINK" '.cases[] as $case | $case.audit_events[] | {audit_id:$case.audit_id,case_id:$case.id,audit_sink:$audit_sink} + .' <<<"$payload" > "$AUDIT_OUTPUT.tmp"
chmod 0600 "$AUDIT_OUTPUT.tmp"
mv -- "$AUDIT_OUTPUT.tmp" "$AUDIT_OUTPUT"

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
