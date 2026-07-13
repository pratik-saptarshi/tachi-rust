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
TRUSTED_WORKFLOW_SHA256=beadd4fd229e19aca457e20515c56594f38285b461d52d42f185686dffa64665
PREFLIGHT_CREATED=false
RUN_DIR=""
CLEANUP_VERIFIED=true
if [ -n "${ACT_SMOKE_PREFLIGHT:-}" ]; then
    PREFLIGHT="$ACT_SMOKE_PREFLIGHT"
else
    PREFLIGHT="$(mktemp "${TMPDIR:-/tmp}/tachi-act-preflight.XXXXXX")"
    PREFLIGHT_CREATED=true
fi
cleanup() {
    local cleanup_ok=true
    if [ -n "$RUN_DIR" ] && [ "$RETAIN" != true ]; then
        if rm -rf -- "$RUN_DIR" && [ ! -e "$RUN_DIR" ]; then
            RUN_DIR=""
        else
            cleanup_ok=false
        fi
    fi
    if [ "$PREFLIGHT_CREATED" = true ]; then
        if ! rm -f -- "$PREFLIGHT" || [ -e "$PREFLIGHT" ]; then
            cleanup_ok=false
        fi
    fi
    CLEANUP_VERIFIED="$cleanup_ok"
    return 0
}
trap cleanup EXIT
case "$RETAIN" in true|false) ;; *) echo 'act-smoke-run: ACT_SMOKE_RETAIN must be true or false' >&2; exit 2 ;; esac
case "$TIMEOUT_SECONDS" in ''|*[!0-9]*) echo 'act-smoke-run: ACT_SMOKE_TIMEOUT_SECONDS must be numeric' >&2; exit 2 ;; esac
[ "$TIMEOUT_SECONDS" -gt 0 ] && [ "$TIMEOUT_SECONDS" -le "$MAX_TIMEOUT_SECONDS" ] || { echo 'act-smoke-run: timeout must be between 1 and 900 seconds' >&2; exit 2; }
case "$MAX_LOG_BYTES" in ''|*[!0-9]*) echo 'act-smoke-run: ACT_SMOKE_MAX_LOG_BYTES must be numeric' >&2; exit 2 ;; esac
[ "$MAX_LOG_BYTES" -gt 0 ] && [ "$MAX_LOG_BYTES" -le "$MAX_LOG_BYTES_LIMIT" ] || { echo 'act-smoke-run: retained log cap must be between 1 and 1048576 bytes' >&2; exit 2; }
mutable_image_opt_in="$(printenv ACT_SMOKE_ALLOW_MUTABLE_IMAGE || true)"
case "$IMAGE" in
    *@sha256:*) ;;
    *) [ "$mutable_image_opt_in" = true ] || { echo 'act-smoke-run: digest-pinned image required (or explicitly opt into mutable image)' >&2; exit 2; } ;;
esac
command -v jq >/dev/null 2>&1 || { echo 'act-smoke-run: jq is required' >&2; exit 2; }
command -v perl >/dev/null 2>&1 || { echo 'act-smoke-run: perl is required for bounded execution' >&2; exit 2; }
now_ns() {
    perl -MTime::HiRes=time -e 'printf "%.0f\n", time() * 1000000000'
}
[ -r "$FIXTURE" ] || { echo "act-smoke-run: missing fixture: $FIXTURE" >&2; exit 2; }
[ -r "$WORKFLOW" ] || { echo "act-smoke-run: missing workflow: $WORKFLOW" >&2; exit 2; }
command -v realpath >/dev/null 2>&1 || { echo 'act-smoke-run: realpath is required for trusted path validation' >&2; exit 2; }
command -v shasum >/dev/null 2>&1 || { echo 'act-smoke-run: shasum is required for workflow integrity validation' >&2; exit 2; }
FIXTURE="$(realpath "$FIXTURE")"
WORKFLOW="$(realpath "$WORKFLOW")"
case "$FIXTURE" in "$ROOT_DIR/tests/fixtures/act/"*) ;; *) echo 'act-smoke-run: fixture must remain under tests/fixtures/act' >&2; exit 2 ;; esac
case "$WORKFLOW" in "$ROOT_DIR/.github/workflows/"*) ;; *) echo 'act-smoke-run: workflow must remain under .github/workflows' >&2; exit 2 ;; esac
case "$JOB" in *[!A-Za-z0-9._-]*|'') echo 'act-smoke-run: invalid job name' >&2; exit 2 ;; esac
if [ "$WORKFLOW" != "$ROOT_DIR/.github/workflows/ci-route-observe.yml" ] || [ "$JOB" != route-observe ]; then
    echo 'act-smoke-run: workflow/job is outside the route-observe allowlist' >&2
    exit 2
fi
[ ! -e "$ROOT_DIR/.actrc" ] || { echo 'act-smoke-run: repository .actrc is not allowed for governed execution' >&2; exit 2; }
workflow_sha256="$(shasum -a 256 "$WORKFLOW" | awk '{print $1}')"
[ "$workflow_sha256" = "$TRUSTED_WORKFLOW_SHA256" ] || { echo 'act-smoke-run: trusted route workflow content hash mismatch' >&2; exit 2; }
grep -Fqx "  ${JOB}:" "$WORKFLOW" || { echo "act-smoke-run: job is not defined in workflow: $JOB" >&2; exit 2; }
jq -e '(.action | type == "string") and (.number | type == "number") and (.pull_request.number | type == "number") and (.pull_request.head.ref | type == "string") and (.pull_request.base.ref | type == "string") and (.pull_request.head.sha | type == "string" and test("^[0-9a-f]{40}$")) and (.pull_request.base.sha | type == "string" and test("^[0-9a-f]{40}$")) and (.repository.full_name == "pratik-saptarshi/tachi-rust")' "$FIXTURE" >/dev/null || {
    echo "act-smoke-run: invalid synthetic pull-request fixture" >&2
    exit 2
}
ACT_SMOKE_OUTPUT="$PREFLIGHT" "$ROOT_DIR/scripts/act-smoke.sh" >/dev/null
jq -e --arg image "$IMAGE" '(.status == "READY" or .status == "SKIPPED_UNAVAILABLE") and (.runtime.kind | type == "string") and (.runtime.image == $image) and (.policy.secrets == "empty") and (.policy.privileged == false) and (.policy.host_mounts == false) and (.policy.socket_mounts == false) and (.side_effects.workflow_invoked == false) and (.side_effects.sarif_upload == false)' "$PREFLIGHT" >/dev/null || {
    echo 'act-smoke-run: preflight failed schema or policy validation' >&2
    exit 1
}
status="$(jq -r '.status' "$PREFLIGHT")"
preflight_payload="$(jq '.runtime.endpoint = (if (.runtime.endpoint | startswith("unix://")) then "local-unix-socket" else "redacted" end)' "$PREFLIGHT")"
relative_path() {
    case "$1" in
        "$ROOT_DIR"/*) printf '%s' "${1#"$ROOT_DIR"/}" ;;
        *) printf '%s' "$1" ;;
    esac
}
write_result() {
    local result_payload="$1"
    if [ "$OUTPUT" = /dev/stdout ]; then
        printf '%s\n' "$result_payload"
        return 0
    fi
    [ -L "$OUTPUT" ] && { echo 'act-smoke-run: refusing symlink output path' >&2; return 2; }
    umask 077
    local output_dir
    output_dir="$(dirname -- "$OUTPUT")"
    mkdir -m 700 -p -- "$output_dir"
    local output_dir_mode
    output_dir_mode="$(stat -c '%a' "$output_dir" 2>/dev/null || stat -f '%Lp' "$output_dir" 2>/dev/null || true)"
    case "$output_dir_mode" in
        ''|*[!0-7]*) echo 'act-smoke-run: unable to verify output directory permissions' >&2; return 2 ;;
    esac
    (( (0$output_dir_mode & 077) == 0 )) || { echo 'act-smoke-run: output directory must not be group/world accessible' >&2; return 2; }
    [ -e "$OUTPUT" ] && chmod 600 "$OUTPUT"
    printf '%s\n' "$result_payload" > "$OUTPUT"
}
emit_setup_failure() {
    local stage="$1"
    local reason="$2"
    cleanup
    payload="$(jq -n \
        --arg status FAILED \
        --arg job "$JOB" \
        --arg fixture "$(relative_path "$FIXTURE")" \
        --arg workflow "$(relative_path "$WORKFLOW")" \
        --arg stage "$stage" \
        --arg reason "$reason" \
        --argjson cleanup_verified "$CLEANUP_VERIFIED" \
        --argjson preflight "$preflight_payload" \
        '{schema_version:1,status:$status,job:$job,workflow:$workflow,event_fixture:$fixture,preflight:$preflight,benchmark:null,failure:{stage:$stage,reason:$reason},side_effects:{workflow_invoked:false,release_or_security_steps:false,sarif_upload:false,artifact_upload:false},cleanup:{verified:$cleanup_verified}}')"
    write_result "$payload"
}
if [ "$status" != READY ]; then
    result_status="$status"
    payload="$(jq -n --arg status "$status" --arg job "$JOB" --arg fixture "$(relative_path "$FIXTURE")" --arg workflow "$(relative_path "$WORKFLOW")" --argjson preflight "$preflight_payload" \
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
    act_home="$RUN_DIR/home"
    mkdir -m 700 -- "$act_home"
    runtime_endpoint="$(jq -r '.runtime.endpoint // "unreported"' "$PREFLIGHT")"
    case "$runtime_endpoint" in
        unix:///*) ;;
        *) echo 'act-smoke-run: runtime endpoint must be a local unix socket' >&2; exit 1 ;;
    esac
    if [ "$runtime_endpoint" != unreported ] && [ "$runtime_endpoint" != null ]; then
        export DOCKER_HOST="$runtime_endpoint"
    fi
    runtime_endpoint_label=local-unix-socket
    runtime_probe_ok=true
    if ! before_containers="$($runtime_cmd ps -aq 2>/dev/null)"; then
        echo 'act-smoke-run: runtime container baseline probe failed; refusing execution' >&2
        emit_setup_failure runtime-baseline 'runtime container baseline probe failed' || exit 2
        exit 1
    fi
    start_ns="$(now_ns)"
    image_pull_ms=0
    cpu_count_raw="$(jq -r '.resource_profile.cpu_limit // "unreported"' "$PREFLIGHT")"
    if [[ "$cpu_count_raw" =~ ^[0-9]+$ ]]; then
        cpu_count="$cpu_count_raw"
    else
        cpu_count=null
    fi
    memory_total_raw="$(jq -r '.resource_profile.memory_limit // "unreported"' "$PREFLIGHT")"
    if [[ "$memory_total_raw" =~ ^[0-9]+$ ]]; then
        memory_total_bytes="$memory_total_raw"
    else
        memory_total_bytes=null
    fi
    image_present_before_pull=false
    if "$runtime_cmd" image inspect "$IMAGE" >/dev/null 2>&1; then
        image_present_before_pull=true
    fi
    if [ "$image_present_before_pull" = true ]; then
        cache_mode=warm
    else
        cache_mode=cold
    fi
    image_digest="$(jq -r '.runtime.image_digest' "$PREFLIGHT")"
    if [ "$image_present_before_pull" = false ] || [ -z "$image_digest" ] || [ "$image_digest" = unresolved-before-run ]; then
        image_pull_start_ns="$(now_ns)"
        if ! "$runtime_cmd" pull --platform linux/amd64 "$IMAGE" >/dev/null; then
            echo 'act-smoke-run: runtime image pull failed; refusing workflow execution' >&2
            emit_setup_failure image-pull 'runtime image pull failed' || exit 2
            exit 1
        fi
        image_pull_end_ns="$(now_ns)"
        image_pull_ms="$(( (image_pull_end_ns - image_pull_start_ns) / 1000000 ))"
        image_digest="$("$runtime_cmd" image inspect "$IMAGE" --format '{{index .RepoDigests 0}}' 2>/dev/null || true)"
    fi
    [ -n "$image_digest" ] || image_digest="unresolved-after-pull"
    actual_image_digest="$("$runtime_cmd" image inspect "$IMAGE" --format '{{index .RepoDigests 0}}' 2>/dev/null || true)"
    case "$IMAGE" in
        *@sha256:*)
            if [ "$actual_image_digest" != "$IMAGE" ]; then
                echo 'act-smoke-run: runtime image digest does not match the requested pinned image' >&2
                emit_setup_failure image-integrity 'runtime image digest does not match requested pinned image' || exit 2
                exit 1
            fi
            image_digest="$actual_image_digest"
            ;;
        *)
            [ -n "$actual_image_digest" ] && image_digest="$actual_image_digest"
            ;;
    esac
    image_size_raw="$("$runtime_cmd" image inspect "$IMAGE" --format '{{.Size}}' 2>/dev/null || true)"
    if [[ "$image_size_raw" =~ ^[0-9]+$ ]]; then
        image_size_bytes="$image_size_raw"
    else
        image_size_bytes=null
    fi

    set +e
    act_tmpdir="$(printenv TMPDIR || printf '%s' /tmp)"
    act_docker_host="$(printenv DOCKER_HOST || true)"
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
        my $status = $?;
        exit(($status & 127) ? 128 + ($status & 127) : (($status >> 8) & 255));
    ' "$TIMEOUT_SECONDS" env -i PATH="$PATH" HOME="$act_home" TMPDIR="$act_tmpdir" DOCKER_HOST="$act_docker_host" act pull_request \
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
        # Do not remove newly observed containers: without a runtime-owned
        # label, a concurrent unrelated container cannot be distinguished from
        # an act leftover safely. Report cleanup failure instead.
        cleanup_remediation_attempted=false
    fi
    cleanup_verified=false
    containers_verified=false
    [ "$runtime_probe_ok" = true ] && [ "$before_containers" = "$after_containers" ] && containers_verified=true
    temp_dir_removed=false
    temp_dir_retained=false
    logs_verified=false
    artifacts_removed=false
    if rm -rf -- "$RUN_DIR/artifacts"; then
        [ ! -e "$RUN_DIR/artifacts" ] && artifacts_removed=true
    fi
    if [ "$RETAIN" = true ]; then
        temp_dir_retained=true
        if perl -pe 's/(authorization:\s*bearer\s+|(?:api[_-]?key|access[_-]?token|client[_-]?secret|token|password|secret|aws_access_key_id|aws_secret_access_key)["\x27]?\s*[:=]\s*["\x27]?)[^\s",}\x27]+/$1[REDACTED]/ig; s/(gh[pousr]_|github_pat_|sk-[A-Za-z0-9_-]+|AKIA[0-9A-Z]{16})[A-Za-z0-9_-]*/[REDACTED]/g; s/([?&](?:api_key|access_token|client_secret|token)=)[^&\s]+/$1[REDACTED]/ig; s#(?<![:/A-Za-z0-9])/(?:[A-Za-z0-9._-]+/)*[A-Za-z0-9._-]+#[PATH]#g' "$ACT_LOG" 2>/dev/null | head -c "$MAX_LOG_BYTES" > "$RUN_DIR/act-redacted.log"; then
            retained_log_bytes="$(wc -c < "$RUN_DIR/act-redacted.log" | tr -d ' ')"
            if [ -f "$RUN_DIR/act-redacted.log" ] && [ "$retained_log_bytes" -le "$MAX_LOG_BYTES" ]; then
                logs_verified=true
            fi
        fi
        rm -f -- "$ACT_LOG"
    else
        run_dir_path="$RUN_DIR"
        rm -f -- "$ACT_LOG"
        rm -rf -- "$RUN_DIR"
        RUN_DIR=""
        [ ! -e "$run_dir_path" ] && temp_dir_removed=true
        logs_verified=true
    fi
    [ "$containers_verified" = true ] && { [ "$temp_dir_removed" = true ] || [ "$temp_dir_retained" = true ]; } && [ "$logs_verified" = true ] && [ "$artifacts_removed" = true ] && cleanup_verified=true
    benchmark_status="FAILED"
    [ "$act_exit" -eq 0 ] && [ "$runtime_probe_ok" = true ] && [ "$cleanup_verified" = true ] && benchmark_status="PASSED"
    result_status="$benchmark_status"
    payload="$(jq -n \
        --arg status "$benchmark_status" \
        --arg job "$JOB" \
        --arg fixture "$(relative_path "$FIXTURE")" \
        --arg workflow "$(relative_path "$WORKFLOW")" \
        --arg runtime "$RUNTIME" \
        --arg runtime_endpoint "$runtime_endpoint_label" \
        --arg image "$IMAGE" \
        --arg image_digest "$image_digest" \
        --arg network "$NETWORK" \
        --argjson cpu_count "$cpu_count" \
        --arg cache_mode "$cache_mode" \
        --argjson preflight "$preflight_payload" \
        --argjson act_exit "$act_exit" \
        --argjson timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson timed_out "$( [ "$act_exit" -eq 124 ] && printf '%s' true || printf '%s' false )" \
        --argjson containers_verified "$containers_verified" \
        --argjson logs_verified "$logs_verified" \
        --argjson artifacts_removed "$artifacts_removed" \
        --argjson temp_dir_removed "$temp_dir_removed" \
        --argjson temp_dir_retained "$temp_dir_retained" \
        --argjson cleanup_remediation_attempted "$cleanup_remediation_attempted" \
        --argjson wall_time_ms "$wall_time_ms" \
        --argjson image_pull_ms "$image_pull_ms" \
        --argjson memory_total_bytes "$memory_total_bytes" \
        --arg workflow_sha256 "$workflow_sha256" \
        --argjson image_size_bytes "$image_size_bytes" \
        --argjson image_present_before_pull "$image_present_before_pull" \
        --argjson cleanup_verified "$cleanup_verified" \
        '{schema_version:1,status:$status,job:$job,workflow:$workflow,event_fixture:$fixture,preflight:$preflight,runtime:{kind:$runtime,endpoint:$runtime_endpoint,image:$image,image_digest:$image_digest,network:$network,cache_mode:$cache_mode,workflow_sha256:$workflow_sha256},benchmark:{wall_time_ms:$wall_time_ms,image_pull_ms:$image_pull_ms,cpu_count:$cpu_count,memory_total_bytes:$memory_total_bytes,image_size_bytes:$image_size_bytes,image_present_before_pull:$image_present_before_pull,act_exit_code:$act_exit,timeout_seconds:$timeout_seconds,timed_out:$timed_out},side_effects:{workflow_invoked:true,release_or_security_steps:false,sarif_upload:false,artifact_upload:false},cleanup:{verified:$cleanup_verified,logs_verified:$logs_verified,artifacts_removed:$artifacts_removed,runtime_containers_verified:$containers_verified,temp_dir_removed:$temp_dir_removed,temp_dir_retained:$temp_dir_retained,remediation_attempted:$cleanup_remediation_attempted}}')"
fi
write_result "$payload" || exit 2
printf 'act-smoke-run status=%s job=%s\n' "$result_status" "$JOB" >&2
if [ "$result_status" = FAILED ]; then
    exit 1
fi
exit 0
