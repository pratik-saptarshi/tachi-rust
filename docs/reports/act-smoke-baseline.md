# act/Podman advisory smoke baseline

Status: partial-green on the 2026-07-12 Darwin x86_64 host.

## Unavailable-safe Podman result

The default `make act-smoke` / `make act-smoke-run` path remains
`SKIPPED_UNAVAILABLE` when rootless Podman is not available. It invokes no
workflow and records empty secrets, no privileged/host/socket/SSH/cloud
mounts, no SARIF or artifact upload, no release/security steps, and verified
wrapper cleanup.

The official Podman v5.8.5 Intel package was checksum-verified, but system
installation requires administrator credentials in this environment. A
user-local Podman machine booted its Fedora CoreOS guest but its API readiness
did not remain stable under this host harness, so no Podman measurement is
claimed.

## Explicit Docker/Colima fallback evidence

The fallback is opt-in only:

```text
ACT_SMOKE_RUNTIME=docker
ACT_SMOKE_ALLOW_DOCKER_FALLBACK=true
ACT_SMOKE_IMAGE=catthehacker/ubuntu@sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08
```

Colima 0.10.3 provided a Docker-compatible Linux/x86_64 engine with 2 CPUs
and 4,104,118,272 bytes of memory at a user-local Colima Docker socket
(endpoint identity verified; the user-specific absolute path is intentionally
redacted). `act` 0.2.89 ran only the synthetic
`route-observe` job using `catthehacker/ubuntu:act-latest` at digest
`sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08`.

| Cache mode | Result | Image pull | Job wall time | Cleanup | Hosted side effects |
|---|---|---:|---:|---|---|
| cold-1 | `PASSED` | 646 ms | 13,710 ms | verified | none |
| cold-2 | `PASSED` | 542 ms | 13,727 ms | verified | none |
| warm (n=6) | `PASSED` | 0 ms | 13,024–13,470 ms (median 13,258.5 ms) | verified for all 6 | none |

The wrapper defaults to `network=none`; these route measurements explicitly
set `ACT_SMOKE_NETWORK=host` because the synthetic route step performs a
read-only `git fetch`. The local workflow sets `ACT_SMOKE=true`, skips `actions/upload-artifact`,
and validates `route.json` inside the container. The invocation uses empty
secrets, `--container-daemon-socket=-`, no privileged mode, `--rm`, and an
explicit synthetic `host` network because the route job performs a read-only
`git fetch`. These results demonstrate act behavior only; they do not close
the Podman-specific Beads issues or satisfy hosted CI, coverage, security,
SARIF, or publish acceptance.

The Docker fallback repeatability checkpoint is complete at two cold and six
warm samples. Required next evidence is the same sample shape on a stable
rootless Podman Docker-compatible API, plus CPU/memory/image/cache provenance
and cleanup observations.
