# E2E Coverage and Publish-Gate Execution Plan

**Status**: Active execution plan; E2E-COV-007, E2E-COV.2, E2E-COV-010.1, and E2E-COV-010.2 complete; E2E-COV-009.1/.2 and the E2E-COV-010 umbrella remain open
**Baseline**: `main` / `origin/main` at `61ba9ee`
**Last reviewed**: 2026-07-13
**Controlling tracker**: `.beads/issues.jsonl` and the live Beads database

## Current state

Local `main` and `origin/main` are synchronized at `61ba9ee`; no main push is
required before this feature branch. The product E2E foundation is present,
including CLI artifacts, desktop commands, MCP stdio, and initialization /
install / update / analysis journeys. The governed nightly branch result is
85.15625% and stable coverage is 90.56% lines / 90.22% regions. The live
coverage-audit run reports 119 active modules: 13 unit, 101 integration, 1
smoke, 4 E2E, and 0 support/regression. The 114/96 closeout snapshot is
historical and must not be used for new closeout claims.

The remaining evidence is narrower than the product journey inventory:

- the complete uninterrupted `make publish-gate` exited 0 with terminal
  full-mode runner evidence and E2E-COV-007 is closed in Beads;
- E2E-COV-008 and RT-CI-006.2 now have repeated hosted PR timing,
  artifact-integrity, queue/run, and branch-protection evidence; historical
  failed runs remain visible rather than being removed from the sample;
- security follow-up `E2E-COV.2` is complete: its hosted
  metadata binding, exact push-ref binding, bounded log, path normalization,
  expanded redaction, retention, 0600 cleanup-receipt, unit/aggregate schema,
  valid/rejected new-format PR fixture, workflow_dispatch, explicit-PR-commit,
  fail-closed cleanup, and tiny-log-cap contracts are implemented and tested;
- the act/Podman lane is advisory and must remain unavailable-safe and
  security-isolated; a Docker/Colima fallback is measured separately and does
  not close Podman-specific acceptance;
- deterministic agentic replay evidence is promoted; the E2E epic remains open
  only while the act lane and its dependent umbrella synchronization remain
  incomplete.

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
| F-01 | The E2E-COV-007 closeout snapshot recorded 114 modules / 96 integration modules, while the current audit now reports 119 / 101. | Correction | Preserve the 114/96 closeout as historical and synchronize all current-state surfaces to the dated 119/101 audit refresh. |
| F-02 | Current governed nightly output is 85.15625%; older docs and Beads notes said 85.09%. | Correction | Synchronized from the terminal gate; the 85% threshold remains unchanged. |
| F-03 | `E2E-COV-007` Beads notes still described the interrupted aggregate gate. | Correction | Resolved by the terminal uninterrupted publish-gate closeout and synchronized Beads/docs evidence. |
| F-04 | `E2E-COV.2` review found incomplete provenance, schema, cleanup, redaction, path, and size contracts. | Security/privacy gap | Closed after metadata/ref/path/log/retention/receipt/unit+aggregate schema and valid/rejected PR-fixture evidence. |
| F-05 | `E2E-COV-009.1/.2` are ready children under an advisory act/Podman lane. | New concern | Add explicit unavailable-safe preflight and no-side-effect benchmark gates; never let them satisfy hosted CI or publish acceptance. |
| F-06 | Earlier remote DNS prevented a fresh `git ls-remote` during planning. | Environment / delivery caveat | Resolved: current `main` and `origin/main` are synchronized at `61ba9ee`; no stale PR state is used for this slice. |
| F-07 | Homebrew's current Podman formula rejects this Intel Mac, while the official v5.8.5 amd64 package requires administrator installation; the user-local VM boot/API path is not stable in this host harness. | Environment / architecture constraint | Keep Podman evidence open; use an explicitly labeled Colima/Docker fallback for act behavior and measurements, never relabel it as Podman proof. |
| F-08 | Available-runtime execution was previously a deliberate `exit 2` path, so preflight fields and unavailable-safe tests could not prove a real named-job run. | Gap / implementation | Implemented a bounded act runner with explicit runtime selection, image digest, timing, policy flags, side-effect fields, and container cleanup comparison; remaining repeated-sample and Podman gates stay open. |
| F-09 | `actions/upload-artifact@v4` attempted the hosted artifact API during local act runs despite a local artifact-server flag. | Security/privacy gap | Added an `ACT_SMOKE=true` workflow guard and in-container `route.json` validation; local act no longer calls hosted artifact upload. |
| F-10 | Plan/BOM/checklist text still claimed the act lane was entirely unmeasured and carried stale baseline/review wording. | Documentation correction | Synchronize all publish artifacts from the measured Docker fallback and preserve explicit Podman limitations. |

Actionability: F-01–F-05 pass (0.90–0.99) with full repository context;
F-06 passes with a context caveat because remote state could not be refreshed.
No finding was dropped. No scope-expansion veto was triggered. Security/privacy
follow-up F-04 remains a P2 implementation item with a security veto against
silent deferral.

## Priority and dependency order

| Order | Beads issue | Priority | Execution decision | Exit evidence |
|---:|---|:---:|---|---|
| 1 | `E2E-COV.2` | P2 | Complete. | Synthetic metadata-binding tests, explicit retention/redaction/cleanup contract, security evidence, no secret persistence, and fail-closed rejection of unsupported rerun-attempt artifact binding are evidenced and Beads-closed. |
| 2 | `E2E-COV-009.1` | P2 | Implemented and revalidated; Podman-specific capability remains open. | Safe capability result with `SKIPPED_UNAVAILABLE`/`READY`, runtime/API/image/resource fields, explicit Docker fallback opt-in, and policy checks. **Partial-green:** Docker/Colima is measured; official Intel Podman VM/API evidence remains unavailable. |
| 3 | `E2E-COV-009.2` | P2 | Implemented as an advisory, explicitly labeled Docker fallback; Podman samples remain. | Synthetic named-job smoke, cold/warm timing, image digest, cleanup, and no hosted-CI claims. **Partial-green:** two cold runs (13,710/13,727 ms; 646/542 ms pulls) and six warm runs (13,024–13,470 ms; median 13,258.5 ms) passed on Colima; Podman measurements remain. |
| 4 | `E2E-COV-010.1` | P2 | Complete. | `docs/testing/tdd-evidence.json` and its Rust contract provide AC-to-test mapping and durable RED/GREEN/REFACTOR records across all test levels; agentic promotion is now passed by .2. |
| 5 | `E2E-COV-010.2` | P2 | Complete; Beads closure ready. | Final review confirms fixed fake-tool invocation for approval, bounded timeout, cancellation, and circuit-breaker cases; denial remains non-invoked. The harness records transitions, writes an independent 0600 JSONL audit sink, proves descendant cleanup, and uses no live model/network. |
| 6 | `E2E-COV-010` | P2 | Close umbrella only after children and failure matrix are complete. | All child evidence, coverage audit, documentation, security gates, and promotion decision. |
| 7 | `E2E-COV` | P1 | Close epic last. | All journeys, failure/cancellation matrix, coverage, BOM, checklist, codemap, Beads, and publish gates agree. |
| 8 | `RT-CI` | P0 | Complete; Beads closure recorded. | Route, protection, timing, tracker, and rollback documentation agree; all seven children are closed. |

`E2E-COV-007` and `E2E-COV-007.3` are complete and intentionally omitted from the active queue;
their terminal local-runner and publish-gate evidence remains documented in the
roadmap/BOM/checklist. The current remaining work is the advisory act/Podman
lane (with its measured Docker fallback clearly separated) and the dependent
E2E-COV-010 umbrella.

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

## Continuation execution plan — plan-review-integrator reconciliation

This is the current execution contract for every open Beads issue. It is
deliberately explicit about partial-green evidence: a Docker-compatible
fallback demonstrates act behavior, but it cannot satisfy a Podman-specific
acceptance criterion or close the dependency chain by inference.

### 2026-07-13 continuation audit and next-slice traceability

The live Beads audit on `main` at `61ba9ee` confirms that the active queue is
the E2E epic plus `E2E-COV-009`, `E2E-COV-009.1`, `E2E-COV-009.2`, and the
dependent `E2E-COV-010` umbrella. The two child issues are implementation-
green on the Docker/Colima fallback but remain open because the selected
acceptance is runtime-specific: this host has `act` 0.2.89, no `podman`
executable or Podman machine, and a working Colima Docker 29.5.2 fallback.
No Docker result is promoted to Podman evidence.

Plan-review traceability for this continuation is:

| Finding | Classification | Action and proof |
|---|---|---|
| Podman binary/API unavailable on the current Darwin x86_64 host | Environment constraint | Retain `SKIPPED_UNAVAILABLE`; do not close `E2E-COV-009.1/.2`, and preserve the Docker fallback as non-equivalent. Re-test the capability before every closeout attempt. |
| Available-runtime setup failures exited before emitting machine-readable evidence | Correction / reliability gap | Add RED/GREEN contracts for runtime baseline, image pull, and pinned-image integrity failures. `scripts/act-smoke-run.sh` now emits `status=FAILED`, a bounded `failure.stage`, no-workflow side-effect flags, and cleanup verified from actual removal before returning nonzero. |
| Retained evidence could expose platform-specific absolute paths | Security/privacy correction | Normalize arbitrary POSIX absolute paths while preserving URL separators; adversarial contract coverage includes macOS and Linux-style workspace, home, cache, tool, and temporary paths. |
| Existing benchmark and cleanup paths must not regress | Already addressed by regression suite | Run all `act_smoke_run_contract` tests, shell syntax, formatting, diff hygiene, workflow/security gates, and protected remote checks before merge. |
| Documentation and tracker must reflect partial-green truth | Governance synchronization | Update this plan, issue notes/export, BOM, publish checklist, codemap, act baseline, and `integration_log.jsonl` from the same checkpoint. |

The next executable slice is therefore the structured setup-failure and
retained-evidence correction, followed by review and protected merge. The Podman capability
slice remains a host-dependent follow-up and cannot be closed by synthetic
runtime shims or Docker compatibility alone.

### E2E-COV-009.1 — capability preflight

**RED:** run `cargo test -p tachi-core --test act_smoke_contract` with missing
runtime shims, stopped/incompatible runtime shims, invalid image references,
and disallowed fallback settings. Each unavailable case must exit 0 with
`SKIPPED_UNAVAILABLE`; policy/schema violations must fail closed.

**GREEN:** `scripts/act-smoke.sh` probes `act`, the selected runtime API,
runtime version, image digest when available, architecture, CPU/memory
profile, and policy fields. Podman remains the default. Docker/Colima is only
accepted when `ACT_SMOKE_ALLOW_DOCKER_FALLBACK=true` is explicit.

**REFACTOR/checkpoint:** run the focused contracts, shell syntax, gitleaks,
and `make act-smoke`; record the runtime kind, rootless indicator, and exact
limitation. Close only when a rootless Podman machine/API evidence record
exists, or retain open with the Docker fallback clearly marked non-equivalent.

### E2E-COV-009.2 — named-job smoke and resource benchmark

**RED:** prove the wrapper rejects untrusted fixture/workflow paths, malformed
fixtures, unsafe runtime selection, and a hosted artifact-upload attempt.
Prove unavailable mode cannot invoke a workflow or side-effect job.

**GREEN:** `scripts/act-smoke-run.sh` hard-allows only
`ci-route-observe.yml`/`route-observe` with an explicit synthetic event,
empty secrets, a sanitized act environment, a local-unix runtime endpoint, no
daemon-socket mount, no privileged mode, bounded network declaration,
`ACT_SMOKE=true`, a verified pinned runner image, and `--rm`. It emits
status, normalized runtime/image digest, image-pull time, wall time, derived
cache/resource fields, side-effect flags, and cleanup/log comparison. The
workflow skips hosted artifact upload in local mode and validates `route.json`
in-container.

**REFACTOR/checkpoint:** collect at least two cold and five warm samples for
the available runtime, preserve raw machine-readable results outside the
repository unless sanitized, update the baseline/BOM/checklist/codemap, and
keep hosted queue time separate from local wall time. Current measured sample:
Docker/Colima cold runs 13,710/13,727 ms (646/542 ms image pulls) and six warm
runs 13,024–13,470 ms (median 13,258.5 ms); later cached runs passed in
27,636 ms, 13,249 ms, 14,309 ms, and 13,732 ms with 2 CPUs, 4,104,118,272
bytes memory, a 571,284,183-byte image, and `image_present_before_pull=true`.
The hardened runs derived `cache_mode=warm`, normalized the endpoint,
verified the pinned digest and workflow hash, and verified retained-log and
artifact cleanup. All passed
with digest
`sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08`.

### E2E-COV-009 — advisory feature synchronization

Close only after both children have their acceptance evidence, the parent
notes identify Podman versus Docker fallback truthfully, and the advisory
contract remains excluded from hosted CI, coverage, security, SARIF, and
publish gates. Synchronize `.beads/issues.jsonl`, this plan,
`docs/reports/act-smoke-baseline.md`, BOM, checklist, codemap, and
`integration_log.jsonl` in one checkpoint.

### E2E-COV-010 — governed multi-level and agentic test evidence

The two children are implemented and agentic promotion is passed. Keep the
umbrella open while its declared dependency on E2E-COV-009 is open. On the
next checkpoint, re-run the TDD evidence contract, coverage audit, failure /
cleanup matrix, security gates, and remote CI; then close only if the canonical
coverage baseline and all five test levels remain synchronized.

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
coverage >=85%, the historical 114-module closeout plus current 119-module
audit reconciliation, gitleaks, dependency policy,
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
with no workflow/SARIF/release/security side effects when Podman is absent.
The Docker/Colima fallback now has live CPU/memory/image-size/cache fields and
verified cleanup; the baseline is documented in
`docs/reports/act-smoke-baseline.md`. Podman-specific cold/warm resource
evidence remains open.

### E2E-COV-010.1/.2 — TDD promotion and deterministic agentic replay

`E2E-COV-010.1` is complete: `docs/testing/tdd-evidence.json` defines the
AC-to-test matrix, durable RED/GREEN/REFACTOR records, and
pass/fail/skipped/inconclusive semantics across unit, integration, functional,
E2E, and agentic levels. `E2E-COV-010.2` now has an offline deterministic
fake-tool replay harness in `scripts/agentic-replay.sh`,
`tests/fixtures/agentic/fake-tool.sh`, and
`tests/fixtures/agentic/replay.json`; final code review passed the child. The
harness actually invokes the fixed fake tool for approval,
bounded timeout, cancellation, and circuit-breaker cases while denial remains
non-invoked. It records
explicit approval, denial, timeout, cancellation, and circuit-breaker
transitions, writes an independent 0600 JSONL audit sink, and cross-checks
deterministic audit correlation. Bounded fake-tool execution without live
model/network access is complete; remaining work is umbrella synchronization
and closure after the act/Podman dependency lane is resolved.

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
`GITHUB_TOKEN`, privileged mode, host/socket/SSH/cloud mounts, and
release/security side effects. Network is `none` by default; `host` networking
is permitted only as an explicit synthetic-test exception for the route job's
read-only `git fetch`, and must be recorded in the result. Record versions,
the bound endpoint, image digest, architecture, resource profile, cache mode,
resolved action references, and cleanup. A missing runtime is
`SKIPPED_UNAVAILABLE`, not a failed product test. MicroVM experiments remain
deferred to Linux/KVM.

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
