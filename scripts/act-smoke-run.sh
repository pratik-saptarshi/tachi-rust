#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)"
FIXTURE="${ACT_SMOKE_FIXTURE:-$ROOT_DIR/tests/fixtures/act/pull-request.json}"
WORKFLOW="${ACT_SMOKE_WORKFLOW:-$ROOT_DIR/.github/workflows/ci-route-observe.yml}"
OUTPUT="${ACT_SMOKE_RUN_OUTPUT:-/dev/stdout}"
JOB="${ACT_SMOKE_JOB:-route-observe}"
RUNTIME="${ACT_SMOKE_RUNTIME:-podman}"
IMAGE="${ACT_SMOKE_IMAGE:-catthehacker/ubuntu@sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08}"
NETWORK="${ACT_SMOKE_NETWORK:-none}"
RETAIN="${ACT_SMOKE_RETAIN:-false}"
TIMEOUT_SECONDS="${ACT_SMOKE_TIMEOUT_SECONDS:-300}"
MAX_LOG_BYTES="${ACT_SMOKE_MAX_LOG_BYTES:-65536}"
MAX_TIMEOUT_SECONDS=900
MAX_LOG_BYTES_LIMIT=1048576
PREFLIGHT_CREATED=false
RUN_DIR=""
if [ -n "${ACT_SMOKE_PREFLIGHT:-}" ]; then
    PREFLIGHT="$ACT_SMOKE_PREFLIGHT"
else
    PREFLIGHT="$(mktemp "${TMPDIR:-/tmp}/tachi-act-preflight.XXXXXX")"
    PREFLIGHT_CREATED=true
fi
cleanup() {
    if [ -n "$RUN_DIR" ] && [ "$RETAIN" != true ]; then
        rm -rf -- "$RUN_DIR"
    fi
    if [ "$PREFLIGHT_CREATED" = true ]; then
        rm -f -- "$PREFLIGHT"
    fi
    return 0
}
trap cleanup EXIT
case "$RETAIN" in true|false) ;; *) echo 'act-smoke-run: ACT_SMOKE_RETAIN must be true or false' >&2; exit 2 ;; esac
case "$TIMEOUT_SECONDS" in ''|*[!0-9]*) echo 'act-smoke-run: ACT_SMOKE_TIMEOUT_SECONDS must be numeric' >&2; exit 2 ;; esac
[ "$TIMEOUT_SECONDS" -gt 0 ] && [ "$TIMEOUT_SECONDS" -le "$MAX_TIMEOUT_SECONDS" ] || { echo 'act-smoke-run: timeout must be between 1 and 900 seconds' >&2; exit 2; }
case "$MAX_LOG_BYTES" in ''|*[!0-9]*) echo 'act-smoke-run: ACT_SMOKE_MAX_LOG_BYTES must be numeric' >&2; exit 2 ;; esac
[ "$MAX_LOG_BYTES" -gt 0 ] && [ "$MAX_LOG_BYTES" -le "$MAX_LOG_BYTES_LIMIT" ] || { echo 'act-smoke-run: retained log cap must be between 1 and 1048576 bytes' >&2; exit 2; }
command -v jq >/dev/null 2>&1 || { echo 'act-smoke-run: jq is required' >&2; exit 2; }
command -v perl >/dev/null 2>&1 || { echo 'act-smoke-run: perl is required for bounded execution' >&2; exit 2; }
now_ns() {
    perl -MTime::HiRes=time -e 'printf "%.0f\n", time() * 1000000000'
}
[ -r "$FIXTURE" ] || { echo "act-smoke-run: missing fixture: $FIXTURE" >&2; exit 2; }
[ -r "$WORKFLOW" ] || { echo "act-smoke-run: missing workflow: $WORKFLOW" >&2; exit 2; }
command -v realpath >/dev/null 2>&1 || { echo 'act-smoke-run: realpath is required for trusted path validation' >&2; exit 2; }
FIXTURE="$(realpath "$FIXTURE")"
WORKFLOW="$(realpath "$WORKFLOW")"
case "$FIXTURE" in "$ROOT_DIR/tests/fixtures/act/"*) ;; *) echo 'act-smoke-run: fixture must remain under tests/fixtures/act' >&2; exit 2 ;; esac
case "$WORKFLOW" in "$ROOT_DIR/.github/workflows/"*) ;; *) echo 'act-smoke-run: workflow must remain under .github/workflows' >&2; exit 2 ;; esac
case "$JOB" in *[!A-Za-z0-9._-]*|'') echo 'act-smoke-run: invalid job name' >&2; exit 2 ;; esac
grep -Fqx "  ${JOB}:" "$WORKFLOW" || { echo "act-smoke-run: job is not defined in workflow: $JOB" >&2; exit 2; }
jq -e '(.action | type == "string") and (.number | type == "number") and (.pull_request.number | type == "number") and (.pull_request.head.ref | type == "string") and (.pull_request.base.ref | type == "string") and (.pull_request.head.sha | type == "string" and test("^[0-9a-f]{40}$")) and (.pull_request.base.sha | type == "string" and test("^[0-9a-f]{40}$")) and (.repository.full_name == "pratik-saptarshi/tachi-rust")' "$FIXTURE" >/dev/null || {
    echo "act-smoke-run: invalid synthetic pull-request fixture" >&2
    exit 2
}
ACT_SMOKE_OUTPUT="$PREFLIGHT" "$ROOT_DIR/scripts/act-smoke.sh" >/dev/null
jq -e '(.status == "READY" or .status == "SKIPPED_UNAVAILABLE") and (.runtime.kind | type == "string") and (.policy.secrets == "empty") and (.policy.privileged == false) and (.policy.host_mounts == false) and (.policy.socket_mounts == false) and (.side_effects.workflow_invoked == false) and (.side_effects.sarif_upload == false)' "$PREFLIGHT" >/dev/null || {
    echo 'act-smoke-run: preflight failed schema or policy validation' >&2
    exit 1
}
status="$(jq -r '.status' "$PREFLIGHT")"
relative_path() {
    case "$1" in
        "$ROOT_DIR"/*) printf '%s' "${1#"$ROOT_DIR"/}" ;;
        *) printf '%s' "$1" ;;
    esac
}
if [ "$status" != READY ]; then
    result_status="$status"
    payload="$(jq -n --arg status "$status" --arg job "$JOB" --arg fixture "$(relative_path "$FIXTURE")" --arg workflow "$(relative_path "$WORKFLOW")" --argjson preflight "$(cat "$PREFLIGHT")" \
        '{schema_version:1,status:$status,job:$job,workflow:$workflow,event_fixture:$fixture,preflight:$preflight,benchmark:null,side_effects:{workflow_invoked:false,release_or_security_steps:false,sarif_upload:false,artifact_upload:false},cleanup:{verified:true}}')"
else
    case "$RUNTIME" in
        podman|docker) ;;
        *) echo "act-smoke-run: unsupported runtime: $RUNTIME" >&2; exit 2 ;;
    esac
    command -v act >/dev/null 2>&1 || { echo 'act-smoke-run: act is required for available execution' >&2; exit 2; }
    case "$NETWORK" in
        host|none) ;;
        *) echo 'act-smoke-run: network must be host or none' >&2; exit 2 ;;
    esac

    runtime_cmd="$RUNTIME"
    command -v "$runtime_cmd" >/dev/null 2>&1 || { echo "act-smoke-run: runtime is unavailable: $runtime_cmd" >&2; exit 2; }
    RUN_DIR="$(mktemp -d "${TMPDIR:-/tmp}/tachi-act-run.XXXXXX")"
    ACT_LOG="$RUN_DIR/act.jsonl"
    mkdir -m 700 -- "$RUN_DIR/artifacts"
    runtime_endpoint="$(jq -r '.runtime.endpoint // "unreported"' "$PREFLIGHT")"
    if [ "$runtime_endpoint" != unreported ] && [ "$runtime_endpoint" != null ]; then
        export DOCKER_HOST="$runtime_endpoint"
    fi
    runtime_probe_ok=true
    if ! before_containers="$($runtime_cmd ps -aq 2>/dev/null)"; then
        echo 'act-smoke-run: runtime container baseline probe failed; refusing execution' >&2
        exit 1
    fi
    start_ns="$(now_ns)"
    image_pull_ms=0
    image_digest="$(jq -r '.runtime.image_digest' "$PREFLIGHT")"
    if [ -z "$image_digest" ] || [ "$image_digest" = unresolved-before-run ]; then
        image_pull_start_ns="$(now_ns)"
        "$runtime_cmd" pull --platform linux/amd64 "$IMAGE" >/dev/null
        image_pull_end_ns="$(now_ns)"
        image_pull_ms="$(( (image_pull_end_ns - image_pull_start_ns) / 1000000 ))"
        image_digest="$("$runtime_cmd" image inspect "$IMAGE" --format '{{index .RepoDigests 0}}' 2>/dev/null || true)"
    fi
    [ -n "$image_digest" ] || image_digest="unresolved-after-pull"

    set +e
    perl -e '
        my $limit = shift @ARGV;
        my $pid = fork // die "fork failed: $!";
        if ($pid == 0) {
            setpgrp(0, 0);
            exec @ARGV or exit 127;
        }
        $SIG{ALRM} = sub {
            kill "TERM", -$pid;
            select undef, undef, undef, 1;
            kill "KILL", -$pid;
            waitpid($pid, 0);
            exit 124;
        };
        alarm $limit;
        waitpid($pid, 0);
        alarm 0;
        exit(($? >> 8) & 255);
    ' "$TIMEOUT_SECONDS" act pull_request \
        --workflows "$WORKFLOW" \
        --job "$JOB" \
        --eventpath "$FIXTURE" \
        --platform "ubuntu-latest=$IMAGE" \
        --container-daemon-socket=- \
        --secret-file=/dev/null \
        --network="$NETWORK" \
        --container-architecture linux/amd64 \
        --artifact-server-path "$RUN_DIR/artifacts" \
        --artifact-server-addr 127.0.0.1 \
        --artifact-server-port 34567 \
        --pull=false \
        --rm \
        --env=ACT_SMOKE=true \
        --json >"$ACT_LOG" 2>&1
    act_exit=$?
    set -e
    end_ns="$(now_ns)"
    wall_time_ms="$(( (end_ns - start_ns) / 1000000 ))"
    if ! after_containers="$($runtime_cmd ps -aq 2>/dev/null)"; then
        runtime_probe_ok=false
        after_containers=""
    fi
    cleanup_remediation_attempted=false
    if [ "$runtime_probe_ok" = true ] && [ "$before_containers" != "$after_containers" ]; then
        cleanup_remediation_attempted=true
        for container_id in $after_containers; do
            case " $before_containers " in
                *" $container_id "*) ;;
                *) "$runtime_cmd" rm -f "$container_id" >/dev/null 2>&1 || true ;;
            esac
        done
        if ! after_containers="$($runtime_cmd ps -aq 2>/dev/null)"; then
            runtime_probe_ok=false
            after_containers=""
        fi
    fi
    cleanup_verified=false
    containers_verified=false
    [ "$runtime_probe_ok" = true ] && [ "$before_containers" = "$after_containers" ] && containers_verified=true
    temp_dir_removed=false
    temp_dir_retained=false
    if [ "$RETAIN" = true ]; then
        temp_dir_retained=true
        perl -pe 's/(authorization:\s*bearer\s+|(?:api[_-]?key|access[_-]?token|client[_-]?secret|token|password|secret|aws_access_key_id|aws_secret_access_key)["\x27]?\s*[:=]\s*["\x27]?)[^\s",}\x27]+/$1[REDACTED]/ig; s/(gh[pousr]_|github_pat_|sk-[A-Za-z0-9_-]+|AKIA[0-9A-Z]{16})[A-Za-z0-9_-]*/[REDACTED]/g; s/([?&](?:api_key|access_token|client_secret|token)=)[^&\s]+/$1[REDACTED]/ig' "$ACT_LOG" 2>/dev/null | head -c "$MAX_LOG_BYTES" > "$RUN_DIR/act-redacted.log" || true
        rm -f -- "$ACT_LOG"
    else
        run_dir_path="$RUN_DIR"
        rm -f -- "$ACT_LOG"
        rm -rf -- "$RUN_DIR"
        RUN_DIR=""
        [ ! -e "$run_dir_path" ] && temp_dir_removed=true
    fi
    [ "$containers_verified" = true ] && { [ "$temp_dir_removed" = true ] || [ "$temp_dir_retained" = true ]; } && cleanup_verified=true
    benchmark_status="FAILED"
    [ "$act_exit" -eq 0 ] && [ "$runtime_probe_ok" = true ] && benchmark_status="PASSED"
    result_status="$benchmark_status"
    payload="$(jq -n \
        --arg status "$benchmark_status" \
        --arg job "$JOB" \
        --arg fixture "$(relative_path "$FIXTURE")" \
        --arg workflow "$(relative_path "$WORKFLOW")" \
        --arg runtime "$RUNTIME" \
        --arg runtime_endpoint "$runtime_endpoint" \
        --arg image "$IMAGE" \
        --arg image_digest "$image_digest" \
        --arg network "$NETWORK" \
        --arg cache_mode "${ACT_SMOKE_CACHE_MODE:-unknown}" \
        --argjson preflight "$(cat "$PREFLIGHT")" \
        --argjson act_exit "$act_exit" \
        --argjson timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson timed_out "$( [ "$act_exit" -eq 124 ] && printf '%s' true || printf '%s' false )" \
        --argjson containers_verified "$containers_verified" \
        --argjson temp_dir_removed "$temp_dir_removed" \
        --argjson temp_dir_retained "$temp_dir_retained" \
        --argjson cleanup_remediation_attempted "$cleanup_remediation_attempted" \
        --argjson wall_time_ms "$wall_time_ms" \
        --argjson image_pull_ms "$image_pull_ms" \
        --argjson cleanup_verified "$cleanup_verified" \
        '{schema_version:1,status:$status,job:$job,workflow:$workflow,event_fixture:$fixture,preflight:$preflight,runtime:{kind:$runtime,endpoint:$runtime_endpoint,image:$image,image_digest:$image_digest,network:$network,cache_mode:$cache_mode},benchmark:{wall_time_ms:$wall_time_ms,image_pull_ms:$image_pull_ms,act_exit_code:$act_exit,timeout_seconds:$timeout_seconds,timed_out:$timed_out},side_effects:{workflow_invoked:true,release_or_security_steps:false,sarif_upload:false,artifact_upload:false},cleanup:{verified:$cleanup_verified,runtime_containers_verified:$containers_verified,temp_dir_removed:$temp_dir_removed,temp_dir_retained:$temp_dir_retained,remediation_attempted:$cleanup_remediation_attempted}}')"
fi
if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    [ -L "$OUTPUT" ] && { echo 'act-smoke-run: refusing symlink output path' >&2; exit 2; }
    umask 077
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    [ -e "$OUTPUT" ] && chmod 600 "$OUTPUT"
    printf '%s\n' "$payload" > "$OUTPUT"
fi
printf 'act-smoke-run status=%s job=%s\n' "$result_status" "$JOB" >&2
if [ "$result_status" = FAILED ]; then
    exit 1
fi
exit 0
