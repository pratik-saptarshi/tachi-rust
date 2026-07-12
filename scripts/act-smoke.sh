#!/usr/bin/env bash
set -euo pipefail

OUTPUT="${ACT_SMOKE_OUTPUT:-/dev/stdout}"
command -v jq >/dev/null 2>&1 || { echo 'act-smoke: jq is required' >&2; exit 2; }

act_available=false
podman_available=false
act_version="unavailable"
podman_version="unavailable"

if command -v act >/dev/null 2>&1 && act_version="$(act --version 2>/dev/null | head -n 1)" && [ -n "$act_version" ]; then
    act_available=true
fi
if command -v podman >/dev/null 2>&1 && podman_version="$(podman version --format '{{.Client.Version}}' 2>/dev/null)" && [ -n "$podman_version" ]; then
    podman_available=true
fi

status="READY"
reason="act and Podman are available for a separately gated synthetic smoke"
if [ "$act_available" != true ] || [ "$podman_available" != true ]; then
    status="SKIPPED_UNAVAILABLE"
    reason="act and rootless Podman are required; no workflow job was invoked"
fi

payload="$(jq -n \
    --arg status "$status" \
    --arg reason "$reason" \
    --arg act_version "$act_version" \
    --arg podman_version "$podman_version" \
    --arg arch "$(uname -m)" \
    --arg os "$(uname -s)" \
    --argjson act_available "$act_available" \
    --argjson podman_available "$podman_available" \
    '{schema_version:1,status:$status,reason:$reason,runtime:{act_available:$act_available,act_version:$act_version,podman_available:$podman_available,podman_version:$podman_version,os:$os,architecture:$arch},policy:{secrets:"empty",privileged:false,host_mounts:false,socket_mounts:false,ssh_or_cloud_credentials:false,network:"disabled-unless-explicit-synthetic-test"},side_effects:{workflow_invoked:false,release_or_security_steps:false,sarif_upload:false},resource_profile:{cpu_limit:"unreported",memory_limit:"unreported"}}')"

if [ "$OUTPUT" = /dev/stdout ]; then
    printf '%s\n' "$payload"
else
    umask 077
    mkdir -p -- "$(dirname -- "$OUTPUT")"
    printf '%s\n' "$payload" > "$OUTPUT"
fi

printf 'act-smoke status=%s\n' "$status" >&2
exit 0
