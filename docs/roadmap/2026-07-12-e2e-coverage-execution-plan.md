# E2E Coverage and Publish-Gate Execution Plan

**Status**: Active execution plan; Slices 1–2 complete; next active issue is `E2E-COV-007`
**Baseline**: `main` / `origin/main` at `36f5eaa`
**Last reviewed**: 2026-07-12
**Controlling tracker**: `.beads/issues.jsonl` and the live Beads database

## Current state

Local `main` and `origin/main` are synchronized at `36f5eaa`; no main push is
required before this feature branch. The product E2E foundation is present,
including CLI artifacts, desktop commands, MCP stdio, and initialization /
install / update / analysis journeys. The governed nightly branch result is
85.09% and stable coverage is 90.56% lines / 90.22% regions.

The remaining evidence is narrower than the product journey inventory:

- the aggregate local publish gate now has terminal full-mode runner evidence,
  but the complete uninterrupted `make publish-gate` still needs recording;
- E2E-COV-008 and RT-CI-006.2 now have repeated hosted PR timing,
  artifact-integrity, queue/run, and branch-protection evidence; historical
  failed runs remain visible rather than being removed from the sample;
- security follow-up `E2E-COV.2` remains open for independent GitHub run
  metadata binding and an explicit local-log retention/redaction contract;
- the act/Podman lane is advisory and must remain unavailable-safe and
  security-isolated;
- TDD and agentic replay evidence must be promoted through explicit test-level
  contracts before the E2E epic can close.

## Priority and dependency order

| Order | Beads issue | Priority | Execution decision | Exit evidence |
|---:|---|:---:|---|---|
| 1 | `E2E-COV-007` | P2 | Close only after an uninterrupted local publish-gate run and standalone coverage evidence are proven together. | Stable line/region gate, governed nightly branch gate, security/supply-chain/privacy gates, and synchronized baseline. |
| 2 | `E2E-COV-009.1` | P2 | Implement preflight before any emulation invocation, after E2E-COV-008. | Safe capability result with `SKIPPED_UNAVAILABLE`, versions, digest, architecture, resource profile, and policy checks. |
| 3 | `E2E-COV-009.2` | P2 | Run only after preflight; advisory and opt-in. | Synthetic named-job smoke, cold/warm resource evidence, cleanup, and no hosted-CI claims. |
| 4 | `E2E-COV-010.1` | P2 | Define promotion and TDD evidence contracts before agentic implementation. | AC-to-test mapping and durable RED/GREEN/REFACTOR records across all test levels. |
| 5 | `E2E-COV-010.2` | P2 | Implement deterministic replay after the evidence contract. | Scripted fake model/tool replay for approval, denial, timeout, cancel, circuit breaker, audit correlation, and no live network/model. |
| 6 | `E2E-COV-010` | P2 | Close umbrella only after children and failure matrix are complete. | All child evidence, coverage audit, documentation, security gates, and promotion decision. |
| 7 | `E2E-COV` | P1 | Close epic last. | All journeys, failure/cancellation matrix, coverage, BOM, checklist, codemap, Beads, and publish gates agree. |
| 8 | `RT-CI` | P0 | Close umbrella after its active timing follow-up and synchronized governance evidence are complete. | Route, protection, timing, tracker, and rollback documentation agree. |

`E2E-COV-007.3` is complete and intentionally omitted from the active queue;
its terminal local-runner evidence remains documented in Slice 1 and in the
roadmap/BOM/checklist. The remaining `E2E-COV-007` work is the uninterrupted
aggregate publish-gate recording.

`E2E-COV-008` and `RT-CI-006.2` are closed in Beads. The
merged PR #24 evidence run `29203699709` validated all eight timing artifacts;
the current pull-request timing collector reports workspace 22-run median 85s
and route-observe 23-run median 14s, both with zero queue median. The latest
five PR #24 runs passed; historical failures remain included in the raw sample
and are documented as limitations rather than discarded.

## Slice 1: aggregate local publish-gate boundary — complete

### RED

1. Run `make publish-gate` under the supported rustup toolchain and capture
   the first failing stage, exit status, elapsed time, and environment.
2. Run each manifest unit independently and run `make test` and
   `make test-route` to distinguish product/test failure from aggregate
   orchestration failure.
3. Add or refine a contract test that fails if the publish gate invokes an
   opaque `cargo test -q` path or loses one of the eight canonical units.

### GREEN

1. Keep `.github/ci-test-units.json` as the single unit inventory.
2. Keep `scripts/ci-local-runner.sh` as the bounded, observable executor.
3. Preserve argv allowlists, secure temp roots, containment, redaction,
   timeout/signal/process-tree cleanup, deterministic aggregate exit, and
   stage-aware JSON provenance.
4. Treat dependency/network/tool-cache failures as environment evidence unless
   a repository defect is demonstrated; never weaken the gate to make it pass.

### REFACTOR and verification

Evidence: `make test` run `20260712T173705Z-72397` passed all 8 units in
536,162 ms, with compile/test at 466,327 ms, test slices at 68,906 ms, zero
failures/timeouts/cancellations, and verified cleanup. The first full publish
gate attempt reached this runner after all preceding security and coverage
stages passed; it was interrupted only to isolate the runner evidence. A full
uninterrupted publish-gate recording remains part of `E2E-COV-007` closeout.

Focused workflow/runner tests, all package suites, `make workflow-gate`,
security gates, and coverage gates passed during this slice. The initial
sandboxed publish-gate attempt failed before product validation because the
Cargo advisory database path was read-only; the escalated rerun passed that
stage, proving the first failure was environmental.

## Slice 2: hosted performance and reliability — complete

The merged PR #24 workspace run `29203699709` passed all eight units and its
eight timing artifacts were validated with `make verify-ci-timing-artifacts
RUN_ID=29203699709 COMMIT=auto`. The pull-request timing collector recorded a
22-run workspace median of 85 seconds (79–101 seconds) and a 23-run
route-observe median of 14 seconds (11–17 seconds), with zero queue median in
both samples. The latest five PR #24 workspace/route runs passed; historical
failed workspace runs remain included in the raw GitHub sample and are
documented rather than discarded. Local full/warm/cold/route evidence remains
40/40 successful unit executions, and branch protection is verified with 16
required contexts. `E2E-COV-008` and `RT-CI-006.2` are closed in Beads.

### Slice 2 evidence contract (satisfied)

Collect repeated PR and main samples with queue time separated from execution
time. Validate all eight timing artifacts by run ID, commit provenance, stage,
unit, and duration. Record cold/warm cache state, toolchain, host, pass/fail/
timeout/cancel counts, artifact integrity, cleanup, and reproducibility in the
integration log. Do not treat local wall time as hosted queue time.

## Slice 3: act/Podman advisory lane

Use `nektos/act` only for a synthetic, named workflow/job. Prefer rootless
Podman through its Docker-compatible API after preflight. Reject secrets,
`GITHUB_TOKEN`, privileged mode, host/network/socket/SSH/cloud mounts, and
release/security side effects. Record versions, image digest, architecture,
resource profile, cache mode, resolved action references, and cleanup. A
missing runtime is `SKIPPED_UNAVAILABLE`, not a failed product test. MicroVM
experiments remain deferred to Linux/KVM.

## Slice 4: governed test and agentic evidence

Every implementation issue must record acceptance-criterion-to-test mapping,
RED/GREEN/REFACTOR commands, and promotion status. The agentic harness uses
fixed scripted model/tool responses, bounded replay, allowlisted commands,
deterministic seeds, audit correlation, and explicit approval/denial/cancel /
timeout/circuit-breaker outcomes. No live model or network is permitted.

## Gates and checkpoints

Each slice produces one conventional commit and requests review before the
next slice. A merge checkpoint requires:

```text
git diff --check
cargo fmt --all -- --check
make workflow-gate
make gitleaks-gate
focused cargo tests
make test-route
make llvm-cov
make llvm-cov-nightly-branch
make publish-gate
```

Before merging, update the roadmap status, issue notes/status, `codemap.md`,
`docs/bill-of-materials.html.md`, `docs/publish-readiness-checklist.html.md`,
and `integration_log.jsonl`. Push through a protected PR and observe all
required GitHub checks to terminal state; never use an administrative bypass.
