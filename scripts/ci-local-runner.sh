#!/usr/bin/env bash
# Manifest-driven local CI runner. Commands are always executed as argv arrays.
set -u
set -m

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)"
MANIFEST="${CI_LOCAL_MANIFEST:-$ROOT_DIR/.github/ci-test-units.json}"
RESULT_SCHEMA="${CI_LOCAL_RESULT_SCHEMA:-$ROOT_DIR/schemas/ci-run-result.schema.json}"
CLEANUP_SCHEMA="${CI_LOCAL_CLEANUP_SCHEMA:-$ROOT_DIR/schemas/ci-cleanup-receipt.schema.json}"
AGGREGATE_SCHEMA="${CI_LOCAL_AGGREGATE_SCHEMA:-$ROOT_DIR/schemas/ci-run-aggregate.schema.json}"
MODE="local-full"
OUTPUT_DIR="${CI_LOCAL_OUTPUT_DIR:-$ROOT_DIR/target/ci-local-results}"
CACHE_CONTEXT="${CI_LOCAL_CACHE_STATE:-unknown}"
RETENTION="${CI_LOCAL_RETENTION:-ephemeral}"
MAX_LOG_BYTES="${CI_LOCAL_MAX_LOG_BYTES:-1048576}"
RUN_ID="$(date -u +%Y%m%dT%H%M%SZ)-$$"
RUN_DIR=""
CLEANUP_RECEIPT=""
INTERRUPTED=0
RUN_START_MS=""

usage() {
    cat <<'EOF'
Usage: scripts/ci-local-runner.sh [--mode local-full|local-route-equivalent]
       [--output-dir DIRECTORY]

Environment: CI_LOCAL_RETENTION=ephemeral|retain (default: ephemeral),
CI_LOCAL_MAX_LOG_BYTES (default: 1048576).
EOF
}

fail() { printf 'ci-local-runner: %s\n' "$1" >&2; exit 2; }

while [ "$#" -gt 0 ]; do
    case "$1" in
        --mode) [ "$#" -ge 2 ] || fail "--mode requires a value"; MODE="$2"; shift 2 ;;
        --output-dir) [ "$#" -ge 2 ] || fail "--output-dir requires a value"; OUTPUT_DIR="$2"; shift 2 ;;
        --help|-h) usage; exit 0 ;;
        *) fail "unknown argument: $1" ;;
    esac
done

case "$MODE" in local-full|local-route-equivalent) ;; *) fail "unsupported mode: $MODE" ;; esac
case "$RETENTION" in ephemeral|retain) ;; *) fail "unsupported retention mode: $RETENTION" ;; esac
case "$MAX_LOG_BYTES" in ''|*[!0-9]*) fail "CI_LOCAL_MAX_LOG_BYTES must be a positive integer" ;; esac
[ "$MAX_LOG_BYTES" -gt 0 ] || fail "CI_LOCAL_MAX_LOG_BYTES must be positive"
command -v jq >/dev/null 2>&1 || fail "jq is required"
[ -r "$MANIFEST" ] || fail "missing manifest: $MANIFEST"
[ -r "$RESULT_SCHEMA" ] || fail "missing result schema: $RESULT_SCHEMA"
[ -r "$CLEANUP_SCHEMA" ] || fail "missing cleanup receipt schema: $CLEANUP_SCHEMA"
[ -r "$AGGREGATE_SCHEMA" ] || fail "missing aggregate result schema: $AGGREGATE_SCHEMA"
jq -e '(.units | type == "array" and length > 0) and (.units | map(.id) | length == (unique | length))' "$MANIFEST" >/dev/null || fail "invalid manifest"

mkdir -p -- "$OUTPUT_DIR" || fail "cannot create output directory"
chmod 0700 "$OUTPUT_DIR" || fail "cannot secure output directory"
CLEANUP_RECEIPT="$OUTPUT_DIR/cleanup-$RUN_ID.json"
RUN_DIR="$OUTPUT_DIR/run-$RUN_ID"
mkdir -- "$RUN_DIR" || fail "cannot create run directory"
chmod 0700 "$RUN_DIR" || fail "cannot secure run directory"

cleanup() {
    [ -z "$RUN_DIR" ] && return
    if [ "$RETENTION" = ephemeral ]; then
        local verified=false
        if rm -rf -- "$RUN_DIR" && [ ! -e "$RUN_DIR" ]; then
            verified=true
        else
            printf 'ci-local-runner: failed to remove ephemeral run directory\n' >&2
        fi
        if [ -n "$CLEANUP_RECEIPT" ]; then
            local receipt_ok=true
            if ! jq -n --arg run_id "$RUN_ID" --arg retention "$RETENTION" --argjson verified "$verified" \
                '{schema_version:1,run_id:$run_id,retention:$retention,verified:$verified}' \
                > "$CLEANUP_RECEIPT.tmp"; then
                receipt_ok=false
            elif ! validate_against_schema "$CLEANUP_SCHEMA" "$CLEANUP_RECEIPT.tmp" >/dev/null; then
                receipt_ok=false
            elif ! chmod 0600 "$CLEANUP_RECEIPT.tmp"; then
                receipt_ok=false
            elif ! mv "$CLEANUP_RECEIPT.tmp" "$CLEANUP_RECEIPT"; then
                receipt_ok=false
            fi
            if [ "$receipt_ok" != true ]; then
                printf 'ci-local-runner: failed to write cleanup receipt\n' >&2
                exit 1
            fi
        fi
        [ "$verified" = true ] || exit 1
    fi
}
# SIGINT and SIGTERM request cooperative cancellation; the child is terminated
# and the result records cancellation or timeout instead of hiding the signal.
trap cleanup EXIT
on_signal() { INTERRUPTED=1; }
trap on_signal INT TERM

toolchain_json() {
    local active rustc_path rustc_version
    active="$(rustup show active-toolchain 2>/dev/null || printf '%s' unavailable)"
    rustc_path="$(rustup which rustc 2>/dev/null || printf '%s' unavailable)"
    rustc_version="$(rustc -Vv 2>/dev/null || printf '%s' unavailable)"
    rustc_path="$(basename -- "$rustc_path")"
    jq -n --arg active "$active" --arg path "$rustc_path" --arg version "$rustc_version" \
        '{active_toolchain:$active,rustc_path:$path,rustc_version:$version}'
}
TOOLCHAIN_JSON="$(toolchain_json)"

terminate_tree() {
    local root="$1" signal="$2" child
    # Bash job control gives each background job a process group on supported
    # hosts; signal the group first so grandchildren cannot outlive a timeout.
    kill -"$signal" "-$root" 2>/dev/null || true
    for child in $(pgrep -P "$root" 2>/dev/null || true); do
        terminate_tree "$child" "$signal"
    done
    kill -"$signal" "$root" 2>/dev/null || true
}

redact_log() {
    if [ -n "${CI_LOCAL_SECRET:-}" ]; then
        command -v perl >/dev/null 2>&1 || fail "perl is required when CI_LOCAL_SECRET is set"
        CI_LOCAL_SECRET="$CI_LOCAL_SECRET" perl -0pi -e 's/\Q$ENV{CI_LOCAL_SECRET}\E/[REDACTED]/g' "$1"
    fi
    if command -v perl >/dev/null 2>&1; then
        perl -0pi -e 's/(Bearer\s+|gh[pousr]_|github_pat_)[A-Za-z0-9_\-\.]+/$1[REDACTED]/gi; s/(AKIA|ASIA)[A-Z0-9]{16}/$1[REDACTED]/g; s/(AWS_SECRET_ACCESS_KEY|AWS_SESSION_TOKEN|GOOGLE_APPLICATION_CREDENTIALS|AZURE_CLIENT_SECRET)\s*[=:]\s*[^\s]+/$1=[REDACTED]/gi; s/(Authorization:\s*Basic\s+)[A-Za-z0-9+\/=]+/$1[REDACTED]/gi; s/-----BEGIN [^-]+-----.*?-----END [^-]+-----/[REDACTED]/gs' "$1"
    fi
    if [ "$(wc -c < "$1")" -gt "$MAX_LOG_BYTES" ]; then
        local marker='[log truncated]'
        local marker_bytes=$(( ${#marker} + 1 ))
        if [ "$MAX_LOG_BYTES" -le "$marker_bytes" ]; then
            head -c "$MAX_LOG_BYTES" "$1" > "$1.tmp"
        else
            local keep_bytes=$(( MAX_LOG_BYTES - marker_bytes ))
            head -c "$keep_bytes" "$1" > "$1.tmp"
            printf '\n%s' "$marker" >> "$1.tmp"
        fi
        mv -- "$1.tmp" "$1"
    fi
}

redact_metadata() {
    if command -v perl >/dev/null 2>&1; then
        CI_LOCAL_SECRET="${CI_LOCAL_SECRET:-}" perl -pe 's/\Q$ENV{CI_LOCAL_SECRET}\E/[REDACTED]/g if length $ENV{CI_LOCAL_SECRET}; s/(Bearer\s+|gh[pousr]_|github_pat_)[A-Za-z0-9_\-\.]+/[REDACTED]/gi; s/(AKIA|ASIA)[A-Z0-9]{16}/[REDACTED]/g; s/(Authorization:\s*Basic\s+)[A-Za-z0-9+\/=]+/[REDACTED]/gi' <<<"$1"
    else
        printf '%s\n' "$1"
    fi
}

validate_against_schema() {
    local schema="$1" value="$2"
    jq -e --slurpfile schema "$schema" '
        def type_ok($expected; $value):
            if ($expected | type) == "array" then
                any($expected[]; . == ($value | type) or (. == "integer" and ($value | type) == "number" and ($value | floor) == $value))
            elif $expected == "integer" then
                ($value | type) == "number" and ($value | floor) == $value
            else
                ($value | type) == $expected
            end;
        def validate($schema; $value):
            (if ($schema.const? != null) then $value == $schema.const else true end)
            and (if ($schema.enum? != null) then any($schema.enum[]; . == $value) else true end)
            and (if ($schema.type? != null) then type_ok($schema.type; $value) else true end)
            and (if ($schema.minLength? != null) then ($value | length) >= $schema.minLength else true end)
            and (if ($schema.minItems? != null) then ($value | length) >= $schema.minItems else true end)
            and (if ($schema.minimum? != null) then $value >= $schema.minimum else true end)
            and (if ($schema.type? == "array") then all($value[]; validate($schema.items; .)) else true end)
            and (if ($schema.type? == "object") then
                ($schema.properties // {}) as $properties
                | all(($schema.required // [])[]; . as $key | ($value | has($key)))
                and (if $schema.additionalProperties == false then all(($value | to_entries)[]; .key as $key | ($properties | has($key))) else true end)
                and all(($properties | to_entries)[]; . as $property | if ($value | has($property.key)) then validate($properties[$property.key]; $value[$property.key]) else true end)
            else true end);
        validate($schema[0]; .)
    ' "$value" >/dev/null || fail "result does not conform to schema: $value"
}

sanitize_arg() {
    local sanitized
    case "$1" in
        "$ROOT_DIR"/*) sanitized="<repo>/${1#"$ROOT_DIR"/}" ;;
        /*) sanitized="<path>/$(basename -- "$1")" ;;
        *) sanitized="$1" ;;
    esac
    redact_metadata "$sanitized"
}

validate_unit_result() {
    local required allowed
    required="$(jq -c '.required' "$RESULT_SCHEMA")"
    allowed="$(jq -c '.properties | keys' "$RESULT_SCHEMA")"
    jq -e --argjson required "$required" --argjson allowed "$allowed" '
        . as $object
        | (all($required[]; . as $key | ($object | has($key))))
        and (all($object | to_entries[]; .key as $key | ($allowed | index($key))))
    ' "$1" >/dev/null || fail "result schema required/properties validation failed: $1"
    jq -e '
        .schema_version == 1
        and (.run_id | type == "string" and length > 0)
        and (.unit | type == "string" and length > 0)
        and (.stage == "compile-and-test" or .stage == "test-slice")
        and (.status | IN("passed", "failed", "timed_out", "cancelled", "skipped_unavailable"))
        and (.duration_ms | type == "number" and . >= 0)
        and (.retention | IN("ephemeral", "retain"))
        and (.cleanup.verified | type == "boolean")
        and (.cleanup.retention == .retention)
    ' "$1" >/dev/null || fail "result schema validation failed: $1"
}

now_ms() {
    if command -v perl >/dev/null 2>&1; then
        perl -MTime::HiRes -e 'printf "%.0f", Time::HiRes::time() * 1000'
    else
        printf '%s000' "$(date +%s)"
    fi
}
RUN_START_MS="$(now_ms)"

run_unit() {
    local encoded="$1" unit_json id stage timeout_seconds log_path result_path exit_path started finished duration status exit_code signal start_ms
    local -a argv=()
    unit_json="$(printf '%s' "$encoded" | base64 --decode 2>/dev/null || printf '%s' "$encoded" | base64 -D)"
    id="$(jq -r '.id' <<<"$unit_json")"
    stage="$(jq -r '.stage' <<<"$unit_json")"
    timeout_seconds="$(jq -r '.timeout_seconds' <<<"$unit_json")"
    log_path="$RUN_DIR/$id.log"
    result_path="$RUN_DIR/$id.json"
    exit_path="$RUN_DIR/.exit-$id"
    while IFS= read -r arg; do argv+=("$arg"); done < <(jq -r '.argv[]' <<<"$unit_json")
    [ "${#argv[@]}" -gt 0 ] || return 2
    [ "${argv[0]}" = cargo ] || { printf 'rejected executable\n' > "$log_path"; return 1; }
    started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    start_ms="$(now_ms)"
    (
        "${argv[@]}" > "$log_path" 2>&1
        rc=$?
        printf '%s\n' "$rc" > "$exit_path.tmp"
        mv "$exit_path.tmp" "$exit_path"
        exit "$rc"
    ) &
    local pid=$!
    local deadline=$(( $(date +%s) + timeout_seconds ))
    exit_code=0
    signal="null"
    status=""
    while [ ! -f "$exit_path" ]; do
        if [ "$INTERRUPTED" -eq 1 ]; then terminate_tree "$pid" TERM; status=cancelled; signal=15; break; fi
        if [ "$(date +%s)" -ge "$deadline" ]; then
            terminate_tree "$pid" TERM
            sleep 1
            terminate_tree "$pid" KILL
            status=timed_out
            signal=9
            break
        fi
        sleep 1
    done
    wait "$pid" 2>/dev/null || exit_code=$?
    if [ -f "$exit_path" ]; then
        exit_code="$(cat "$exit_path")"
    fi
    [ -n "$status" ] || { [ "$exit_code" -eq 0 ] && status=passed || status=failed; }
    finished="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    duration=$(( $(now_ms) - start_ms ))
    redact_log "$log_path"
    rm -f -- "$exit_path" "$exit_path.tmp"
    local safe_argv_json
    safe_argv_json="$(printf '%s\n' "${argv[@]}" | while IFS= read -r arg; do sanitize_arg "$arg"; printf '\n'; done | jq -R -s 'split("\n") | map(select(length > 0))')"
    jq -n --arg run_id "$RUN_ID" --arg id "$id" --arg stage "$stage" --arg mode "$MODE" --arg cache_context "$CACHE_CONTEXT" \
        --arg status "$status" --arg started "$started" --arg finished "$finished" \
        --arg log "$log_path" --argjson exit_code "$exit_code" --argjson signal "$signal" \
        --argjson argv "$safe_argv_json" \
        --argjson toolchain "$TOOLCHAIN_JSON" --argjson duration "$duration" --arg retention "$RETENTION" \
        '{schema_version:1,run_id:$run_id,mode:$mode,unit:$id,stage:$stage,cache_context:$cache_context,argv:$argv,toolchain:$toolchain,status:$status,started_at:$started,finished_at:$finished,duration_ms:$duration,exit_code:$exit_code,signal:$signal,log_path:($id + ".log"),retention:$retention,cleanup:{verified:($retention == "ephemeral"),retention:$retention},redactions:["environment values are not copied to result JSON","machine paths normalized","credential patterns redacted","log size bounded"]}' \
        > "$result_path"
    validate_unit_result "$result_path"
    validate_against_schema "$RESULT_SCHEMA" "$result_path"
    [ "$status" = passed ]
}

overall=0
while IFS= read -r encoded; do
    [ "$INTERRUPTED" -eq 0 ] || break
    run_unit "$encoded" || overall=1
done < <(jq -r --arg mode "$MODE" '.units[] | select(.modes | index($mode)) | @base64' "$MANIFEST")

jq -s --arg run_id "$RUN_ID" --arg mode "$MODE" --arg cache_context "$CACHE_CONTEXT" --argjson started_ms "$RUN_START_MS" --argjson finished_ms "$(now_ms)" \
    --arg retention "$RETENTION" \
    '{schema_version:1,run_id:$run_id,mode:$mode,cache_context:$cache_context,retention:$retention,started_ms:$started_ms,finished_ms:$finished_ms,total_duration_ms:($finished_ms-$started_ms),unit_count:length,passed:(map(select(.status=="passed"))|length),failed:(map(select(.status=="failed"))|length),timed_out:(map(select(.status=="timed_out"))|length),cancelled:(map(select(.status=="cancelled"))|length),stages:(group_by(.stage)|map({stage:.[0].stage,unit_count:length,duration_ms:(map(.duration_ms)|add),failed:(map(select(.status!="passed"))|length)})),cleanup:{verified:($retention == "ephemeral"),retention:$retention},results:.}' \
    "$RUN_DIR"/*.json > "$RUN_DIR/results.json"
jq -e '.schema_version == 1 and (.retention | IN("ephemeral", "retain")) and ((.results | length) == .unit_count)' "$RUN_DIR/results.json" >/dev/null || fail "aggregate result schema validation failed"
validate_against_schema "$AGGREGATE_SCHEMA" "$RUN_DIR/results.json"
printf 'run_id=%s\nmode=%s\nresults=results.json\n' "$RUN_ID" "$MODE"
exit "$overall"
