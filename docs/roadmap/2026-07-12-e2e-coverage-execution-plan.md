# E2E Coverage and Publish-Gate Execution Plan

**Status**: Active execution plan
**Baseline**: `main` / `origin/main` at `fdecc9f`
**Last reviewed**: 2026-07-12
**Controlling tracker**: `.beads/issues.jsonl` and the live Beads database

## Current state

Local `main` and `origin/main` are synchronized at `fdecc9f`; no main push is
required before this feature branch. The product E2E foundation is present,
including CLI artifacts, desktop commands, MCP stdio, and initialization /
install / update / analysis journeys. The governed nightly branch result is
85.09% and stable coverage is 90.56% lines / 90.22% regions.

The remaining evidence is narrower than the product journey inventory:

- the aggregate local publish gate must reach a terminal result or use a
  deterministic, contract-tested runner explanation;
- E2E-COV-008 needs repeated hosted PR timing/reliability evidence and final
  documentation reconciliation;
- RT-CI-006.2 remains open until representative PR timing samples are
  collected, even though mainline timing and branch protection are verified;
- the act/Podman lane is advisory and must remain unavailable-safe and
  security-isolated;
- TDD and agentic replay evidence must be promoted through explicit test-level
  contracts before the E2E epic can close.

## Priority and dependency order

| Order | Beads issue | Priority | Execution decision | Exit evidence |
|---:|---|:---:|---|---|
| 1 | `E2E-COV-007.3` | P1 | Execute first. Diagnose the local aggregate boundary and make the runner contract the canonical local path without weakening gates. | Terminal `make publish-gate`, or a deterministic documented workaround with contract tests and preserved breadth. |
| 2 | `E2E-COV-008` | P1 | Reconcile runner, hosted artifact, timing, queue, cache, reliability, and cleanup evidence. | 8/8 local units across labeled cache states, hosted artifact verification, repeated timing sample, and synchronized docs/tracker. |
| 3 | `RT-CI-006.2` | P2 | Keep open while PR timing sample remains below the acceptance target. | Representative PR samples, separated queue/run medians, variance, and no unexplained reliability regression. |
| 4 | `E2E-COV-007` | P2 | Close only after local publish-gate and coverage evidence are proven together. | Stable line/region gate, governed nightly branch gate, security/supply-chain/privacy gates, and synchronized baseline. |
| 5 | `E2E-COV-009.1` | P2 | Implement preflight before any emulation invocation. | Safe capability result with `SKIPPED_UNAVAILABLE`, versions, digest, architecture, resource profile, and policy checks. |
| 6 | `E2E-COV-009.2` | P2 | Run only after preflight; advisory and opt-in. | Synthetic named-job smoke, cold/warm resource evidence, cleanup, and no hosted-CI claims. |
| 7 | `E2E-COV-010.1` | P2 | Define promotion and TDD evidence contracts before agentic implementation. | AC-to-test mapping and durable RED/GREEN/REFACTOR records across all test levels. |
| 8 | `E2E-COV-010.2` | P2 | Implement deterministic replay after the evidence contract. | Scripted fake model/tool replay for approval, denial, timeout, cancel, circuit breaker, audit correlation, and no live network/model. |
| 9 | `E2E-COV-010` | P2 | Close umbrella only after children and failure matrix are complete. | All child evidence, coverage audit, documentation, security gates, and promotion decision. |
| 10 | `E2E-COV` | P1 | Close epic last. | All journeys, failure/cancellation matrix, coverage, BOM, checklist, codemap, Beads, and publish gates agree. |
| 11 | `RT-CI` | P0 | Close umbrella after its active timing follow-up and synchronized governance evidence are complete. | Route, protection, timing, tracker, and rollback documentation agree. |

## Slice 1: aggregate local publish-gate boundary

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

Run focused workflow/runner tests, all package suites, `make test-route`,
`make workflow-gate`, security gates, coverage gates, and then the complete
publish gate with a writable cargo advisory cache.

## Slice 2: hosted performance and reliability

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
