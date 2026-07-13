# act/Colima advisory smoke baseline

Status: green advisory runtime evidence on the 2026-07-13 Darwin x86_64 host.

The available-runtime wrapper uses Colima by default. Its fail-closed
structured-failure contract produces a `FAILED` JSON result for runtime
baseline, image-pull, or image-integrity failures, with a bounded failure
stage, `workflow_invoked=false`, truthful cleanup, and a nonzero process
exit. This keeps diagnostics machine-readable without treating an unavailable
or unhealthy runtime as a passed smoke.
Retained logs normalize arbitrary POSIX absolute paths (including macOS and
Linux workspace, home, cache, tool, and temporary roots) while preserving URL
separators.

## Colima CLI/API result

`colima version` reports 0.10.3 on Darwin x86_64 with Docker runtime.
`colima status --json` reports the macOS Virtualization.Framework provider,
2 CPUs, 4,294,967,296 bytes of memory, and a local Docker socket. Docker
29.5.2 responds through that socket, and act 0.2.89 is available. The
published evidence stores only the normalized endpoint identity
`local-unix-socket`, never the user-specific socket path.

The default `make act-smoke` / `make act-smoke-run` path uses Colima and
returns `SKIPPED_UNAVAILABLE` if the CLI, VM, or Docker API is unavailable.
Unavailable mode invokes no workflow and records empty secrets, no
privileged/host/socket/SSH/cloud mounts, no SARIF or artifact upload, no
release/security steps, and verified wrapper cleanup.

## Serial Colima evidence

The active runtime command is:

```text
ACT_SMOKE_RUNTIME=colima
ACT_SMOKE_IMAGE=catthehacker/ubuntu@sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08
# explicit synthetic route-fetch exception for the read-only route job
ACT_SMOKE_NETWORK=host
```

Colima 0.10.3 provided the Docker-compatible Linux/x86_64 engine with 2 CPUs
and 4,104,118,272 bytes of memory. act 0.2.89 ran only the synthetic
`route-observe` job using the pinned image digest
`sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08`.

| Cache mode | Result | Image pull | Job wall time | Cleanup | Hosted side effects |
|---|---|---:|---:|---|---|
| cold-1 | `PASSED` | 556 ms | 32,384 ms | verified | none |
| cold-2 | `PASSED` | 562 ms | 32,864 ms | verified | none |
| warm (n=5) | `PASSED` | 0 ms | 29,753–31,347 ms (median 31,225 ms) | verified for all 5 | none |

All samples recorded Colima runtime kind, pinned image digest, workflow hash,
2 CPUs, 4,104,118,272-byte memory profile, 571,284,183-byte image, local
endpoint identity, and verified cleanup/log/artifact handling. The wrapper
defaults to `network=none`; these route measurements explicitly set
`ACT_SMOKE_NETWORK=host` because the synthetic route step performs a
read-only `git fetch`. The local workflow sets `ACT_SMOKE=true`, skips
`actions/upload-artifact`, and validates `route.json` inside the
container. These results demonstrate act behavior only; they do not satisfy
hosted CI, coverage, security, SARIF, or publish acceptance.
