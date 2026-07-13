#!/usr/bin/env bash
set -euo pipefail

OUTPUT="${ACT_SMOKE_OUTPUT:-/dev/stdout}"
command -v jq >/dev/null 2>&1 || { echo 'act-smoke: jq is required' >&2; exit 2; }

act_available=false
runtime_available=false
api_compatible=false
act_version="unavailable"
runtime_version="unavailable"
runtime_kind="${ACT_SMOKE_RUNTIME:-podman}"
allow_docker_fallback="${ACT_SMOKE_ALLOW_DOCKER_FALLBACK:-false}"
allow_mutable_image="${ACT_SMOKE_ALLOW_MUTABLE_IMAGE:-false}"
image="${ACT_SMOKE_IMAGE:-catthehacker/ubuntu@sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08}"
image_digest="unresolved-before-run"
cpu_limit="unreported"
memory_limit="unreported"
runtime_endpoint="unreported"
rootless_json=null
reason=""

case "$runtime_kind" in
    podman|docker) ;;
    *)
        echo "act-smoke: unsupported runtime: $runtime_kind" >&2
        exit 2
        ;;
esac

case "$image" in
    ''|*[!A-Za-z0-9./_:@-]*)
        echo "act-smoke: invalid image reference" >&2
        exit 2
        ;;
esac
case "$image" in
    *@sha256:*) ;;
    *)
        [ "$allow_mutable_image" = true ] || { echo 'act-smoke: digest-pinned image required (or explicitly opt into mutable image)' >&2; exit 2; }
        ;;
esac

if command -v act >/dev/null 2>&1 && act_version="$(act --version 2>/dev/null | head -n 1)" && [ -n "$act_version" ]; then
    act_available=true
fi

if [ "$runtime_kind" = docker ] && [ "$allow_docker_fallback" != true ]; then
    reason="Docker fallback is opt-in; set ACT_SMOKE_ALLOW_DOCKER_FALLBACK=true explicitly"
elif command -v "$runtime_kind" >/dev/null 2>&1; then
    if [ "$runtime_kind" = podman ]; then
        runtime_version="$(podman version --format '{{.Client.Version}}' 2>/dev/null || true)"
        info_json="$(podman info --format json 2>/dev/null || true)"
        rootless_value="$(printf '%s' "$info_json" | jq -r 'if .host.security.rootless != null then .host.security.rootless elif .Host.Security.Rootless != null then .Host.Security.Rootless else "unreported" end' 2>/dev/null || true)"
        if [ "$rootless_value" = true ]; then
            rootless_json=true
        elif [ "$rootless_value" = false ]; then
            rootless_json=false
        fi
        runtime_endpoint="$(podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}' 2>/dev/null || true)"
        [ -n "$runtime_endpoint" ] && runtime_endpoint="unix://$runtime_endpoint"
        [ -n "$runtime_endpoint" ] || runtime_endpoint="$(printf '%s' "$info_json" | jq -r '.host.remoteSocket.path // .host.remoteSocket.address // .Host.RemoteSocket.Path // .Host.RemoteSocket.Address // empty' 2>/dev/null || true)"
        case "$runtime_endpoint" in
            /*) runtime_endpoint="unix://$runtime_endpoint" ;;
        esac
        [ -n "$runtime_endpoint" ] || runtime_endpoint="${DOCKER_HOST:-unreported}"
    else
        runtime_version="$(docker version --format '{{.Server.Version}}' 2>/dev/null || true)"
        info_json="$(docker info --format '{{json .}}' 2>/dev/null || true)"
        runtime_endpoint="$(docker context inspect --format '{{(index .Endpoints "docker").Host}}' 2>/dev/null || true)"
        [ -n "$runtime_endpoint" ] || runtime_endpoint="${DOCKER_HOST:-unreported}"
    fi
    if [ -n "$runtime_version" ] && printf '%s' "$info_json" | jq -e . >/dev/null 2>&1; then
        if [ "$runtime_kind" = podman ] && [ "$rootless_json" != true ]; then
            reason="podman rootless mode is unavailable or unverified"
        fi
        case "$runtime_endpoint" in
            unix:///*) ;;
            *) [ -n "$reason" ] || reason="$runtime_kind endpoint is not a local unix socket" ;;
        esac
        if [ -n "$reason" ]; then
            :
        elif [ "$runtime_endpoint" = unreported ] || [ -z "$runtime_endpoint" ]; then
            reason="$runtime_kind engine is available but its Docker-compatible endpoint is unresolved"
        else
            api_compatible=true
            runtime_available=true
            cpu_limit="$(printf '%s' "$info_json" | jq -r '.NCPU // .Host.CPUs // "unreported"')"
            memory_limit="$(printf '%s' "$info_json" | jq -r '.MemTotal // .Host.MemTotal // "unreported"')"
            image_digest="$("$runtime_kind" image inspect "$image" --format '{{index .RepoDigests 0}}' 2>/dev/null || true)"
            [ -n "$image_digest" ] || image_digest="unresolved-before-run"
        fi
    else
        reason="$runtime_kind is installed but its engine/API is unavailable"
    fi
else
    reason="$runtime_kind is unavailable"
fi

status="READY"
if [ "$act_available" != true ]; then
    reason="act is unavailable"
elif [ -z "$reason" ]; then
    reason="${runtime_kind} and act are available for a separately gated synthetic smoke"
fi
if [ "$act_available" != true ] || [ "$runtime_available" != true ] || [ "$api_compatible" != true ]; then
    status="SKIPPED_UNAVAILABLE"
    [ -n "$reason" ] || reason="act and the selected runtime/API are required; no workflow job was invoked"
fi

payload="$(jq -n \
    --arg status "$status" \
    --arg reason "$reason" \
    --arg act_version "$act_version" \
    --arg runtime_kind "$runtime_kind" \
    --arg runtime_version "$runtime_version" \
    --arg podman_version "$( [ "$runtime_kind" = podman ] && printf '%s' "$runtime_version" || printf '%s' unavailable )" \
    --argjson podman_available "$( [ "$runtime_kind" = podman ] && printf '%s' "$runtime_available" || printf '%s' false )" \
    --arg image "$image" \
    --arg image_digest "$image_digest" \
    --arg runtime_endpoint "$runtime_endpoint" \
    --argjson rootless "$rootless_json" \
    --arg arch "$(uname -m)" \
    --arg os "$(uname -s)" \
    --argjson act_available "$act_available" \
    --argjson runtime_available "$runtime_available" \
    --argjson api_compatible "$api_compatible" \
    --arg cpu_limit "$cpu_limit" \
    --arg memory_limit "$memory_limit" \
    '{schema_version:1,status:$status,reason:$reason,runtime:{kind:$runtime_kind,act_available:$act_available,act_version:$act_version,runtime_available:$runtime_available,runtime_version:$runtime_version,api_compatible:$api_compatible,podman_available:$podman_available,podman_version:$podman_version,endpoint:$runtime_endpoint,rootless:$rootless,image:$image,image_digest:$image_digest,os:$os,architecture:$arch},policy:{secrets:"empty",privileged:false,host_mounts:false,socket_mounts:false,ssh_or_cloud_credentials:false,network:"disabled-by-default; explicit-synthetic-host-only"},side_effects:{workflow_invoked:false,release_or_security_steps:false,sarif_upload:false,artifact_upload:false},resource_profile:{cpu_limit:$cpu_limit,memory_limit:$memory_limit}}')"

if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    [ -L "$OUTPUT" ] && { echo 'act-smoke: refusing symlink output path' >&2; exit 2; }
    umask 077
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    [ -e "$OUTPUT" ] && chmod 600 "$OUTPUT"
    printf '%s\n' "$payload" > "$OUTPUT"
fi

printf 'act-smoke status=%s\n' "$status" >&2
exit 0
