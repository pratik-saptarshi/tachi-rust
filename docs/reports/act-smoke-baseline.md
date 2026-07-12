# act/Podman advisory smoke baseline

Status: `SKIPPED_UNAVAILABLE` on the 2026-07-12 Darwin x86_64 host.

`make act-smoke-run` validates the synthetic pull-request fixture and the
named `route-observe` job contract in `ci-route-observe.yml`, then consumes the
read-only preflight. Because
neither `act` nor rootless Podman is installed, it does not invoke a workflow
and records no benchmark timing. The result explicitly records empty secrets,
no privileged/host/socket/SSH/cloud mounts, no SARIF upload, no release or
security steps, and verified wrapper cleanup.

Available-runtime cold/warm startup, CPU, memory, image/cache identity, and
Podman Docker-API compatibility remain unmeasured and must not be inferred
from this skipped result. The lane is advisory and cannot satisfy hosted CI,
coverage, security, SARIF, or publish acceptance.
