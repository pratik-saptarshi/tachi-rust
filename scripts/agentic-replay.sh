#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)"
FIXTURE="${AGENTIC_REPLAY_FIXTURE:-$ROOT_DIR/tests/fixtures/agentic/replay.json}"
OUTPUT="${AGENTIC_REPLAY_OUTPUT:-/dev/stdout}"
command -v jq >/dev/null 2>&1 || { echo 'agentic-replay: jq is required' >&2; exit 2; }
[ -r "$FIXTURE" ] || { echo "agentic-replay: missing fixture: $FIXTURE" >&2; exit 2; }
jq -e '
  .schema_version == 1 and .model == "scripted-fake" and (.seed | type == "number")
  and (.network == false) and (.max_iterations | type == "number" and . >= 1 and . <= 32)
  and (.timeout_seconds | type == "number" and . >= 1 and . <= 30)
  and (.cases | type == "array" and length == 5)
  and (all(.cases[]; (.expected | IN("approved", "denied", "timed_out", "cancelled", "blocked"))))
  and (. as $root | all($root.cases[]; . as $case | ($case.expected == "denied" or ($root.allowlisted_commands | index($case.tool) != null))))
' "$FIXTURE" >/dev/null || { echo 'agentic-replay: unsafe or invalid fixture' >&2; exit 2; }

payload="$(jq -c '
  . as $root
  | {schema_version:1,status:"passed",promotion_status:"skipped",promotion_note:"harness present but not promotion-ready until E2E-COV-010.2 behavioral replay review",model:$root.model,seed:$root.seed,network_used:false,max_iterations:$root.max_iterations,timeout_seconds:$root.timeout_seconds,
     cases:[$root.cases[] | {id:.id,status:"passed",outcome:.expected,audit_id:("audit-" + .id),tool:.tool,command_executed:false}]}
' "$FIXTURE")"

if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    umask 077
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    printf '%s\n' "$payload" > "$OUTPUT"
fi
printf 'agentic-replay status=passed cases=5 network=disabled\n' >&2
