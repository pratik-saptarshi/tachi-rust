# E2E Coverage and Publish-Gate Execution Plan

**Status**: Active execution plan; E2E-COV-007 and E2E-COV.2 complete; E2E-COV-009.1/.2 are partial-green
**Baseline**: `main` / `origin/main` at `36f5eaa`
**Last reviewed**: 2026-07-12
**Controlling tracker**: `.beads/issues.jsonl` and the live Beads database

## Current state

Local `main` and `origin/main` are synchronized at `36f5eaa`; no main push is
required before this feature branch. The product E2E foundation is present,
including CLI artifacts, desktop commands, MCP stdio, and initialization /
install / update / analysis journeys. The governed nightly branch result is
85.15625% and stable coverage is 90.56% lines / 90.22% regions. The live
coverage-audit run reports 114 active modules: 13 unit, 96 integration, 1
smoke, 4 E2E, and 0 support/regression. The older 113/95 baseline remains
historical and must not be used for new closeout claims.

The remaining evidence is narrower than the product journey inventory:

- the complete uninterrupted `make publish-gate` now exited 0 with terminal
  full-mode runner evidence; E2E-COV-007 is ready for Beads closure after this
  synchronized documentation checkpoint;
- E2E-COV-008 and RT-CI-006.2 now have repeated hosted PR timing,
  artifact-integrity, queue/run, and branch-protection evidence; historical
  failed runs remain visible rather than being removed from the sample;
- security follow-up `E2E-COV.2` is complete: its hosted
  metadata binding, exact push-ref binding, bounded log, path normalization,
  expanded redaction, retention, 0600 cleanup-receipt, unit/aggregate schema,
  and valid/rejected new-format PR fixture contracts are implemented and
  tested;
- the act/Podman lane is advisory and must remain unavailable-safe and
  security-isolated;
- TDD and agentic replay evidence must be promoted through explicit test-level
  contracts before the E2E epic can close.

### Plan-review-integrator audit and traceability

The 2026-07-12 state audit treated `codemap.md`, this plan, the live Beads
database, `.beads/issues.jsonl`, the publish checklist/BOM, and current branch
artifacts as controlling context. The codebase-memory graph transport was
unavailable during this run, so code discovery used the repository atlas and
direct contract files; GitHub DNS was also unavailable for a live `ls-remote`
refresh and is recorded as an environment limitation, not as proof of remote
divergence.

| Finding | Evidence | Category | Disposition |
|---|---|---|---|
| F-01 | Live audit reports 114 modules / 96 integration modules while historical roadmap text said 113 / 95. | Correction | Synchronized in the E2E-COV-007 closeout pass; historical counts remain labeled. |
| F-02 | Current governed nightly output is 85.15625%; older docs and Beads notes said 85.09%. | Correction | Synchronized from the terminal gate; the 85% threshold remains unchanged. |
| F-03 | `E2E-COV-007` Beads notes still describe the interrupted aggregate gate. | Gap | Keep open until a terminal uninterrupted `make publish-gate` result is recorded. |
| F-04 | `E2E-COV.2` review found incomplete provenance, schema, cleanup, redaction, path, and size contracts. | Security/privacy gap | Closed after metadata/ref/path/log/retention/receipt/unit+aggregate schema and valid/rejected PR-fixture evidence. |
| F-05 | `E2E-COV-009.1/.2` are ready children under an advisory act/Podman lane. | New concern | Add explicit unavailable-safe preflight and no-side-effect benchmark gates; never let them satisfy hosted CI or publish acceptance. |
| F-06 | Remote DNS prevented a fresh `git ls-remote`; PR #26 is open and CI is still pending. | Environment / delivery caveat | Retry outside the sandbox before any remote-sync or merge claim; keep PR #26 separate from this implementation branch. |

Actionability: F-01–F-05 pass (0.90–0.99) with full repository context;
F-06 passes with a context caveat because remote state could not be refreshed.
No finding was dropped. No scope-expansion veto was triggered. Security/privacy
follow-up F-04 remains a P2 implementation item with a security veto against
silent deferral.

## Priority and dependency order

| Order | Beads issue | Priority | Execution decision | Exit evidence |
|---:|---|:---:|---|---|
| 1 | `E2E-COV.2` | P2 | Complete. | Synthetic metadata-binding tests, explicit retention/redaction/cleanup contract, security evidence, and no secret persistence are evidenced and Beads-closed. |
| 2 | `E2E-COV-009.1` | P2 | Implement preflight before any emulation invocation; runtime is currently unavailable-safe. | Safe capability result with `SKIPPED_UNAVAILABLE`, versions, digest, architecture, resource profile, and policy checks. **Partial-green:** read-only unavailable-safe preflight landed; available-runtime API/digest/resource probes remain. |
| 3 | `E2E-COV-009.2` | P2 | Run only after preflight; advisory and opt-in. | Synthetic named-job smoke, cold/warm resource evidence, cleanup, and no hosted-CI claims. **Partial-green:** fixture-driven unavailable-safe wrapper and baseline landed; available-runtime measurements remain. |
| 4 | `E2E-COV-010.1` | P2 | Define promotion and TDD evidence contracts before agentic implementation. | AC-to-test mapping and durable RED/GREEN/REFACTOR records across all test levels. |
| 5 | `E2E-COV-010.2` | P2 | Implement deterministic replay after the evidence contract. | Scripted fake model/tool replay for approval, denial, timeout, cancel, circuit breaker, audit correlation, and no live network/model. |
| 6 | `E2E-COV-010` | P2 | Close umbrella only after children and failure matrix are complete. | All child evidence, coverage audit, documentation, security gates, and promotion decision. |
| 7 | `E2E-COV` | P1 | Close epic last. | All journeys, failure/cancellation matrix, coverage, BOM, checklist, codemap, Beads, and publish gates agree. |
| 8 | `RT-CI` | P0 | Close umbrella after its active timing follow-up and synchronized governance evidence are complete. | Route, protection, timing, tracker, and rollback documentation agree. |

`E2E-COV-007` and `E2E-COV-007.3` are complete and intentionally omitted from the active queue;
their terminal local-runner and publish-gate evidence remains documented in the
roadmap/BOM/checklist. The current remaining work is the advisory act/Podman
lane and agentic evidence.

`E2E-COV-008`, `RT-CI-006.2`, and `E2E-COV-007` are closed in Beads. The
merged PR #24 evidence run `29203699709` validated all eight timing artifacts;
the current pull-request timing collector reports workspace 22-run median 85s
and route-observe 23-run median 14s, both with zero queue median. The latest
five PR #24 runs passed; historical failures remain included in the raw sample
and are documented as limitations rather than discarded.

## Ready-issue execution cards

Each card is a bounded implementation unit. The card must be completed with a
conventional commit, a RED/GREEN/REFACTOR transcript, a focused test result,
updated Beads notes/export, and a codemap/BOM/checklist checkpoint before the
next dependent card is started.

### E2E-COV-007 — uninterrupted publish-gate enforcement

**Goal:** prove the complete fail-closed local publish gate, not merely its
manifest runner.

**RED:** run `make publish-gate` on the supported rustup toolchain and capture
the first nonzero stage, elapsed time, run ID, coverage-audit count, and output
paths. If the gate passes, the RED evidence is the pre-existing Beads/docs
claim that the uninterrupted result was missing; do not manufacture a failure.

**GREEN:** rerun the unchanged gate with bounded output capture and preserve
all security, supply-chain, docs, coverage, release, and cleanup stages. A
passing result must include stable line/region coverage, nightly branch
coverage >=85%, 114-module audit reconciliation, gitleaks, dependency policy,
and terminal local runner results with zero failures/timeouts/cancellations.

**REFACTOR:** update `integration_log.jsonl`, `.beads/issues.jsonl`, the
roadmap, issue cards, BOM, readiness checklist, and `codemap.md` from the same
captured result. Close `E2E-COV-007` only when every acceptance field is
evidenced; otherwise record the exact failed stage and keep it open.

### E2E-COV.2 — timing provenance and local artifact privacy

**Goal:** prevent self-consistent but misbound hosted artifacts and prevent
local logs from retaining secrets or unnecessary machine-specific data.

**RED:** add focused contract tests that reject mismatched workflow/event/ref/
head/conclusion metadata and expose the current implicit retention/cleanup
contract. Tests must use synthetic GitHub metadata and fake runner output; no
network, credentials, or live artifacts are required.

**GREEN:** implement explicit metadata binding for push head provenance and PR
synthetic-merge provenance; define a retention mode with documented default,
redaction of secrets/absolute paths where feasible, secure permissions, and
verified cleanup. Preserve diagnostic usefulness and fail closed on unverifiable
metadata.

**REFACTOR:** run the focused contract tests, gitleaks, JSON/schema validation,
and local runner tests; update security notes, BOM, checklist, codemap, Beads,
and integration log. The current partial-green slice proves exact push refs,
path-normalized metadata, bounded logs, expanded credential redaction, and
truthful retained cleanup state and durable ephemeral receipts. Do not close the issue until
  valid new-format PR fixtures cover accept/reject paths.

### E2E-COV-009.1 — act/Podman capability preflight

**Goal:** make runtime availability and policy safety explicit before invoking
`nektos/act`.

**RED:** add tests for missing `act`, missing Podman, incompatible Docker API,
unsafe mounts/secrets, and unavailable architecture/runtime. Each unavailable
case must produce `SKIPPED_UNAVAILABLE`, while policy violations are failures.

**GREEN:** implement a read-only preflight that reports versions, API
compatibility, image/runtime identity, architecture, resource limits, and
policy decisions. Default to empty secrets, no privileged mode, no host or
socket mounts, no SSH/cloud credentials, and network disabled unless an
explicit synthetic test requires otherwise.

**REFACTOR:** validate shell quoting, JSON output, redaction, deterministic
exit semantics, and macOS/Linux behavior. Record the result without making the
advisory lane a required CI or publish gate.

Current TDD checkpoint: `scripts/act-smoke.sh` and `make act-smoke` pass the
missing-runtime RED/GREEN contract. On this Darwin x86_64 host the result is
`SKIPPED_UNAVAILABLE`, with no workflow invocation, no SARIF/release/security
side effects, empty secrets, and no privileged/host/socket/credential mounts.
The available-runtime branch remains open for Podman Docker-API compatibility,
image digest, and resource-limit evidence.

### E2E-COV-009.2 — advisory smoke/resource benchmark

**Goal:** execute one named synthetic workflow/job only after a passing
preflight and collect reproducible cold/warm resource evidence.

**RED:** add a fixture-driven test proving the command rejects missing or
unsafe preflight state and cannot upload SARIF, call release/security steps, or
use live secrets.

**GREEN:** run the named job with the synthetic pull-request event and record
startup/wall time, CPU/memory where supported, image/cache identity,
provenance, cleanup, and `SKIPPED_UNAVAILABLE` when the runtime is absent.

**REFACTOR:** compare cold/warm samples without conflating local and hosted
queue time, publish an advisory baseline, and retain the no-side-effect
contract in workflow tests and the readiness checklist.

Current TDD checkpoint: `make act-smoke-run` validates the synthetic fixture
and named `route-observe` job definition in `ci-route-observe.yml`, consumes
preflight, and records `SKIPPED_UNAVAILABLE`
with no workflow/SARIF/release/security side effects on this host. The baseline
is documented in `docs/reports/act-smoke-baseline.md`; available-runtime
cold/warm resource evidence remains open.

### E2E-COV-010.1/.2 — TDD promotion and deterministic agentic replay

`E2E-COV-010.1` defines the AC-to-test matrix, durable RED/GREEN/REFACTOR
records, and pass/fail/skipped/inconclusive semantics across unit, integration,
functional, E2E, and agentic levels. `E2E-COV-010.2` then implements a fixed,
scripted fake model/tool replay with bounded iterations, deterministic seed,
allowlisted commands, audit correlation, and approval/denial/timeout/cancel/
circuit-breaker cases. No live model or network is permitted. The evidence
contract must land before replay implementation.

### Umbrella closeout: E2E-COV-010, E2E-COV, RT-CI

Close each umbrella only after all child cards are closed, the coverage audit
and threshold evidence agree, all docs and Beads exports are synchronized, and
the protected remote checks have reached terminal success. An advisory act
lane, a local-only result, or a stale documentation count cannot substitute for
the required product or hosted evidence.

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

Historical evidence: `make test` run `20260712T173705Z-72397` passed all 8
units in 536,162 ms, with compile/test at 466,327 ms, test slices at 68,906 ms,
zero failures/timeouts/cancellations, and verified cleanup. The first full
publish-gate attempt reached this runner after all preceding security and
coverage stages passed; it was interrupted only to isolate the runner evidence.
The later uninterrupted `make publish-gate` closeout is recorded above and in
the E2E-COV-007 Beads note.

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
