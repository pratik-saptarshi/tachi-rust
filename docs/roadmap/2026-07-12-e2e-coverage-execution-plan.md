# E2E Coverage and Publish-Gate Execution Plan

**Status**: Active execution plan; Colima act lane, E2E-COV-010 governance umbrella, fresh publish-gate run, and terminal hosted checks are complete; the broader E2E-COV product-coverage epic remains open pending protected merge and post-merge main verification
**Baseline**: `main` / `origin/main` at `61ba9ee`
**Last reviewed**: 2026-07-13
**Controlling tracker**: `.beads/issues.jsonl` and the live Beads database

## Current state

Local `main` and `origin/main` are synchronized at `61ba9ee`; no main push is
required before this feature branch. The product E2E foundation is present,
including CLI artifacts, desktop commands, MCP stdio, and initialization /
install / update / analysis journeys. The governed nightly branch result is
85.15625% and current stable coverage is 93.24% lines / 92.60% regions; the
90.56% / 90.22% values are historical publish-gate evidence. The live
coverage-audit run reports 119 active modules: 13 unit, 101 integration, 1
smoke, 4 E2E, and 0 support/regression. The 114/96 closeout snapshot is
historical and must not be used for new closeout claims.

The remaining evidence is narrower than the product journey inventory:

- the fresh host-assisted `make publish-gate` completed through the advisory,
  security, coverage, runner, release, and cleanup stages. The terminal
  local-full cleanup receipt is `20260713T045400Z-79992`; the advisory database
  fetched 1,160 records and the live `cargo deny` result was
  `advisories ok, bans ok, licenses ok, sources ok`;
- E2E-COV-008 and RT-CI-006.2 now have repeated hosted PR timing,
  artifact-integrity, queue/run, and branch-protection evidence; historical
  failed runs remain visible rather than being removed from the sample;
- security follow-up `E2E-COV.2` is complete: its hosted
  metadata binding, exact push-ref binding, bounded log, path normalization,
  expanded redaction, retention, 0600 cleanup-receipt, unit/aggregate schema,
  valid/rejected new-format PR fixture, workflow_dispatch, explicit-PR-commit,
  fail-closed cleanup, and tiny-log-cap contracts are implemented and tested;
- the act/Colima lane is advisory and must remain unavailable-safe and
  security-isolated; Colima is the supported local runtime through its CLI and
  Docker-compatible engine, and it does not replace hosted-CI evidence. Podman
  is not part of the active execution path;
- live CLI validation on 2026-07-13 confirmed Colima `0.10.3`, the
  `macOS Virtualization.Framework` provider, Docker API `29.5.2`, 2 CPUs, and
  4 GiB of VM memory. `ACT_SMOKE_RUNTIME=colima make act-smoke` returned
  `READY`; the bounded `route-observe` smoke passed with
  `ACT_SMOKE_NETWORK=host` in 27,260 ms, using the pinned image and verified
  container/artifact/temp cleanup with no hosted side effects. The secure
  `network=none` default correctly fails this synthetic route because its
  checkout step fetches the base ref; that is a documented test limitation,
  not a runtime fallback;
- PR #33's current feature head now has terminal hosted checks with 20 reported
  passes and 0 failures, including CodeQL, gitleaks, supply-chain, workflow,
  route, and package/shell checks. GitHub still reports `MERGEABLE` with
  `mergeStateStatus=BLOCKED`; the active repository secure ruleset includes
  Copilot code review, whose latest attempt returned a quota failure. This is
  a protected-policy blocker, not a test failure; no administrator bypass is
  permitted for this plan.
- deterministic agentic replay evidence is promoted; the act/Colima children
  are complete. The E2E epic remains open only until this fresh gate evidence
  is synchronized into all release artifacts and the protected branch reaches
  its terminal merge state.

### Plan-review-integrator audit and traceability

The 2026-07-12 state audit treated `codemap.md`, this plan, the live Beads
database, `.beads/issues.jsonl`, the publish checklist/BOM, and current branch
artifacts as controlling context. The codebase-memory graph transport was
unavailable during this run, so code discovery used the repository atlas and
direct contract files. The 2026-07-13 continuation re-established live GitHub
access and verified both remote refs and PR state below.

| Finding | Evidence | Category | Disposition |
|---|---|---|---|
| F-01 | The E2E-COV-007 closeout snapshot recorded 114 modules / 96 integration modules, while the current audit now reports 119 / 101. | Correction | Preserve the 114/96 closeout as historical and synchronize all current-state surfaces to the dated 119/101 audit refresh. |
| F-02 | Current governed nightly output is 85.15625%; older docs and Beads notes said 85.09%. | Correction | Synchronized from the terminal gate; the 85% threshold remains unchanged. |
| F-03 | `E2E-COV-007` Beads notes still described the interrupted aggregate gate. | Correction | Resolved by the terminal uninterrupted publish-gate closeout and synchronized Beads/docs evidence. |
| F-04 | `E2E-COV.2` review found incomplete provenance, schema, cleanup, redaction, path, and size contracts. | Security/privacy gap | Closed after metadata/ref/path/log/retention/receipt/unit+aggregate schema and valid/rejected PR-fixture evidence. |
| F-05 | `E2E-COV-009.1/.2` are ready children under an advisory act/Colima lane. | New concern | Add explicit unavailable-safe preflight and no-side-effect benchmark gates; never let them satisfy hosted CI or publish acceptance. |
| F-06 | Earlier remote DNS prevented a fresh `git ls-remote` during planning. | Environment / delivery caveat | Resolved for this checkpoint: live `git ls-remote` confirms `origin/main=61ba9ee` and `origin/chore/e2e-post-merge-plan-sync=4c0cb98`; retain the historical limitation only as prior evidence. |
| F-07 | The earlier container-runtime path required administrator access and was not stable in this Intel host harness. Colima 0.10.3 is already installed. | Environment / architecture correction | Use the Colima CLI and Docker-compatible API as the active local act runtime; record Colima version/provider/socket identity as the closeout dependency. |
| F-08 | Available-runtime execution was previously a deliberate `exit 2` path, so preflight fields and unavailable-safe tests could not prove a real named-job run. | Gap / implementation | Implemented a bounded act runner with explicit Colima runtime selection, image digest, timing, policy flags, side-effect fields, and container cleanup comparison; serial Colima samples now pass. |
| F-09 | `actions/upload-artifact@v4` attempted the hosted artifact API during local act runs despite a local artifact-server flag. | Security/privacy gap | Added an `ACT_SMOKE=true` workflow guard and in-container `route.json` validation; local act no longer calls hosted artifact upload. |
| F-10 | Plan/BOM/checklist text still described the act lane as an unmeasured Docker fallback and carried stale runtime wording. | Documentation correction | Synchronize all publish artifacts from measured Colima CLI/API evidence and preserve the advisory/no-hosted-CI boundary. |
| F-11 | PR #33 has terminal green checks but remains `BLOCKED` under the active secure ruleset because the Copilot review attempt reports quota exhaustion. | Protected delivery blocker | Preserve the green check evidence, record the ruleset/quota condition, keep auto-merge enabled, and do not use an administrator bypass; close the parent only after a protected merge path succeeds and `main` is reverified. |

Actionability: F-01–F-11 pass (0.90–1.00) with full repository context;
F-06 and F-11 are live-command confirmed. No finding was dropped. No
scope-expansion veto was triggered. Security/privacy follow-up F-04 remains
closed with its fail-closed evidence preserved; F-11 remains the sole active
delivery blocker.

## Priority and dependency order

| Order | Beads issue | Priority | Execution decision | Exit evidence |
|---:|---|:---:|---|---|
| 1 | `E2E-COV.2` | P2 | Complete. | Synthetic metadata-binding tests, explicit retention/redaction/cleanup contract, security evidence, no secret persistence, and fail-closed rejection of unsupported rerun-attempt artifact binding are evidenced and Beads-closed. |
| 2 | `E2E-COV-009.1` | P2 | Complete on the installed Colima runtime. | Safe capability result with `SKIPPED_UNAVAILABLE`/`READY`, Colima CLI/version/provider, Docker API/image/resource fields, local endpoint identity, and policy checks. |
| 3 | `E2E-COV-009.2` | P2 | Complete as an advisory Colima smoke lane. | Synthetic named-job smoke, cold/warm timing, image digest, cleanup, and no hosted-CI claims. Two cold runs passed at 32,384/32,864 ms with 556/562 ms pulls; five warm runs passed at 29,753–31,347 ms (median 31,225 ms). |
| 4 | `E2E-COV-010.1` | P2 | Complete. | `docs/testing/tdd-evidence.json` and its Rust contract provide AC-to-test mapping and durable RED/GREEN/REFACTOR records across all test levels; agentic promotion is now passed by .2. |
| 5 | `E2E-COV-010.2` | P2 | Complete; Beads closed. | Final review confirms fixed fake-tool invocation for approval, bounded timeout, cancellation, and circuit-breaker cases; denial remains non-invoked. The harness records transitions, writes an independent 0600 JSONL audit sink, proves descendant cleanup, and uses no live model/network. |
| 6 | `E2E-COV-010` | P2 | Complete; Beads closed. | Child evidence, coverage audit, documentation, security gates, and promotion decision are synchronized; broader product E2E remains tracked by `E2E-COV`. |
| 7 | `E2E-COV` | P1 | Close epic last. | All journeys, failure/cancellation matrix, coverage, BOM, checklist, codemap, Beads, and publish gates agree. |
| 8 | `RT-CI` | P0 | Complete; Beads closure recorded. | Route, protection, timing, tracker, and rollback documentation agree; all seven children are closed. |

`E2E-COV-007` and `E2E-COV-007.3` are complete and intentionally omitted from the active queue;
their terminal local-runner and publish-gate evidence remains documented in the
roadmap/BOM/checklist. The Colima advisory lane and governed multi-level test
evidence are now closed; the remaining work is the broader product journey
coverage in `E2E-COV`.

`E2E-COV-008`, `RT-CI-006.2`, and `E2E-COV-007` are closed in Beads. The
merged PR #24 evidence run `29203699709` validated all eight timing artifacts;
the current pull-request timing collector reports workspace 22-run median 85s
and route-observe 23-run median 14s, both with zero queue median. The latest
five PR #24 runs passed; historical failures remain included in the raw sample
and are documented as limitations rather than discarded.

### Per-Beads execution blueprint and acceptance matrix

The live tracker contains eleven closed child cards and one open parent. This
matrix is the durable execution plan for revalidation, regression response, or
reopening a child; a closed card is not treated as evidence that its acceptance
criteria can be skipped during a merge or publish audit.

| Beads issue | Capability and required behavior | TDD execution slice | Acceptance / exit evidence | Current state |
|---|---|---|---|---|
| `E2E-COV-001` | Freeze the journey matrix, boundary ownership, 119-module baseline, semantic audit rules, and fixture privacy rules. | RED: reject stale/count-only audit claims. GREEN: encode the semantic inventory and privacy contract. REFACTOR: compare plan, issue cards, codemap, and audit output. | `make coverage-audit` reports 119 active modules exactly: 13 unit, 101 integration, 1 smoke, 4 E2E, 0 support/regression; historical counts remain historical. | Closed; revalidate on every coverage-affecting change. |
| `E2E-COV-002` | Drive deterministic architecture input through CLI report-data, threats-SARIF, and risk-scores-SARIF artifacts. | RED: invalid arguments/input must fail and leave no partial output. GREEN: invoke the real CLI and assert semantic artifact projections. REFACTOR: assert stdout/file parity and cleanup. | CLI integration/E2E contracts pass; valid artifacts are schema/semantic-valid, invalid input fails closed, and no partial artifacts remain. | Closed; protected by CLI/package gates. |
| `E2E-COV-003` | Exercise the active desktop host through shared shell dispatch, including preview/save and typed failure paths. | RED: malformed command, path escape, timeout, cancellation, and child leak cases fail. GREEN: headless host dispatch returns status/stdout/stderr and bytes. REFACTOR: preserve command-registry parity and cleanup. | Desktop host contracts prove round-trip bytes, typed errors, path containment, timeout/cancel behavior, and descendant cleanup without requiring a GUI display. | Closed; revalidate with desktop all-targets and host contract gates. |
| `E2E-COV-004` | Exercise MCP stdio startup, request validation, allowlisted dispatch, result metadata, and cancellation. | RED: malformed, unknown, disallowed, and cancelled requests fail closed. GREEN: valid stdio requests return validated tool results. REFACTOR: assert schema/transport/registry parity and no artifact leakage. | MCP package/all-targets and stdio contracts pass; valid responses are typed/validated and unsafe requests do not leak artifacts or secrets. | Closed; protected by the dedicated MCP CI lane. |
| `E2E-COV-005` | Compose a clean temporary clone from init through offline install/update and one real analysis artifact. | RED: interrupted or invalid lifecycle stages leave no unsafe residue. GREEN: run the actual control-plane sequence in a unique temporary clone. REFACTOR: bound cleanup and assert offline determinism. | Temporary clone produces a validated report or SARIF artifact without network access; cleanup is bounded, safe, and verified. | Closed; revalidate when init/install/update contracts change. |
| `E2E-COV-006` | Cover cross-boundary malformed input, disallowed commands, output escape, timeout, cancellation, and child-process cleanup. | RED: each failure/cancel transition must expose the expected typed status and fail the artifact contract. GREEN: implement/verify terminal-state and cleanup behavior. REFACTOR: run sibling CLI, desktop, MCP, and shell regressions. | Shared status/error taxonomy is consistent; no partial artifact, secret-bearing diagnostic, or child process survives failure/cancel. | Closed; required regression matrix before merge. |
| `E2E-COV-007` | Prove uninterrupted branch/line/region coverage and fail-closed publish-gate enforcement. | RED: capture the former interrupted/missing-evidence boundary. GREEN: run the unchanged full gate through advisory, security, coverage, runner, release, and cleanup. REFACTOR: synchronize one receipt across all governance artifacts. | Host-assisted gate receipt `20260713T045400Z-79992`, cargo audit/deny success, current 119-module audit, 93.24% lines / 92.60% regions, and 85.15625% governed nightly branch coverage. | Closed; fresh receipt remains the local publish evidence. |
| `E2E-COV-008` | Make local CI parity observable through a typed manifest-driven package/all-target and shell runner. | RED: opaque aggregate output and signal/timeout/cleanup ambiguity. GREEN: emit per-unit JSON and deterministic aggregate status. REFACTOR: compare manifest units with hosted workflow units and redact paths. | `make test`/`make test-route` preserve CI breadth, timeout/signal/cleanup/redaction semantics, and machine-readable provenance; hosted timing is kept separate from local wall time. | Closed; revalidate against hosted timing artifacts. |
| `E2E-COV.2` | Bind hosted timing artifacts to GitHub metadata and govern local retention/privacy. | RED: reject mismatched workflow/event/ref/head/conclusion and unsafe retention. GREEN: implement explicit provenance, redaction, bounded logs, 0600 receipts, and fail-closed cleanup. REFACTOR: run synthetic accept/reject fixtures and schema checks. | Push and PR provenance, rerun limitation, path normalization, secret redaction, exact log caps, ephemeral retention, cleanup receipt, and no credential persistence are contract-tested and synchronized. | Closed; security veto remains against silent regression. |
| `E2E-COV-009` / `.1` / `.2` | Provide opt-in advisory workflow emulation using the installed Colima CLI and Docker-compatible engine. | RED: unavailable/stopped runtime and unsafe policy inputs are machine-readable and side-effect free. GREEN: preflight Colima and run only the trusted synthetic route job. REFACTOR: measure cold/warm resources, digest, timing, cleanup, and explicit network exception. | `ACT_SMOKE_RUNTIME=colima make act-smoke` is `READY`; live route smoke passes with explicit host networking, pinned image, verified cleanup, and no hosted side effects. Default `network=none` remains safe/unavailable for the checkout-dependent synthetic route. Podman is not used. | Closed; advisory only and never a hosted/publish substitute. |
| `E2E-COV-010` / `.1` / `.2` | Govern unit, integration, functional, E2E, and deterministic agentic evidence with durable RED/GREEN/REFACTOR records. | RED: missing level evidence, fake-tool non-invocation, nondeterministic audit, and cleanup failures. GREEN: use offline scripted fake-tool replay with bounded approval/timeout/cancel/circuit cases. REFACTOR: exact audit correlation, 0600 sink, descendant cleanup, and promotion validator. | `docs/testing/tdd-evidence.json` and focused Rust contracts prove named tests and promotion status; agentic replay uses no live model/network and is promoted `passed`. | Closed; promotion remains independent of hosted CI. |
| `E2E-COV` | Parent closeout: all child journeys, coverage, governance artifacts, security/privacy evidence, and remote state agree. | RED: identify stale artifacts, non-terminal hosted checks, protected-merge blockers, or remote divergence. GREEN: reconcile tracker/docs and monitor terminal CI. REFACTOR: merge, verify post-merge `main`, rerun gates, then close parent. | Only parent is open/ready. Close only after PR #33 terminal checks, protected merge, post-merge main workflows, live refs, and all artifacts agree. | Open; next action is hosted CI monitoring and protected merge progression. |

## Ready-issue execution cards

Each card is a bounded implementation unit. The card must be completed with a
conventional commit, a RED/GREEN/REFACTOR transcript, a focused test result,
updated Beads notes/export, and a codemap/BOM/checklist checkpoint before the
next dependent card is started.

## Continuation execution plan — plan-review-integrator reconciliation

This is the current execution contract for every open Beads issue. Colima is
the supported local act runtime: the Colima CLI proves VM readiness and
provenance, while Docker CLI/API is the engine boundary used by act. The lane
remains advisory and cannot satisfy hosted-CI or publish acceptance by itself.

### 2026-07-13 continuation audit and next-slice traceability

The live Beads audit on `main` at `61ba9ee` confirms that the only active queue
item is the parent E2E epic; the act/Colima issues and dependent `E2E-COV-010`
umbrella are closed. This host has `act` 0.2.89 and Colima 0.10.3 available by
installation, with recorded Docker 29.5.2 CLI/API evidence and serial
cold/warm named-job runs. A stopped Colima VM remains an accurately reported
`SKIPPED_UNAVAILABLE` condition. The active execution path is Colima only;
Podman is not used for the plan's validation or benchmark evidence.

Plan-review traceability for this continuation is:

| Finding | Classification | Action and proof |
|---|---|---|
| Colima CLI/VM/API readiness | Runtime capability | `ACT_SMOKE_RUNTIME=colima` is first-class; `colima version`, `colima status --json`, Docker API compatibility, image digest, and resource profile are recorded. Missing/stopped Colima remains `SKIPPED_UNAVAILABLE`; only Colima evidence is accepted for this plan, and legacy runtime selectors are not closeout evidence. |
| Available-runtime setup failures exited before emitting machine-readable evidence | Correction / reliability gap | Add RED/GREEN contracts for runtime baseline, image pull, and pinned-image integrity failures. `scripts/act-smoke-run.sh` now emits `status=FAILED`, a bounded `failure.stage`, no-workflow side-effect flags, and cleanup verified from actual removal before returning nonzero. |
| Retained evidence could expose platform-specific absolute paths | Security/privacy correction | Normalize arbitrary POSIX absolute paths while preserving URL separators; adversarial contract coverage includes macOS and Linux-style workspace, home, cache, tool, and temporary paths. |
| Existing benchmark and cleanup paths must not regress | Already addressed by regression suite | Run all `act_smoke_run_contract` tests, shell syntax, formatting, diff hygiene, workflow/security gates, and protected remote checks before merge. |
| Documentation and tracker must reflect Colima truth | Governance synchronization | Update this plan, issue notes/export, BOM, publish checklist, codemap, act baseline, and `integration_log.jsonl` from the same checkpoint. |

The next executable slice is terminal hosted-CI monitoring, eligible-review /
protected-merge progression, and post-merge `main` verification. The Colima
lane remains advisory and must not be promoted to hosted-CI or publish
evidence; the fresh publish-gate is the separate local release-quality
evidence. No child Beads implementation issue is currently ready; the parent
remains open as the merge-and-closeout gate.

### E2E-COV-009.1 — capability preflight

**RED:** run `cargo test -p tachi-core --test act_smoke_contract` with missing
runtime shims, stopped/incompatible runtime shims, invalid image references,
and disallowed fallback settings. Each unavailable case must exit 0 with
`SKIPPED_UNAVAILABLE`; policy/schema violations must fail closed.

**GREEN:** `scripts/act-smoke.sh` probes `act`, the selected runtime API,
runtime version, image digest when available, architecture, CPU/memory
profile, and policy fields. Colima is the default and is accepted only after
the Colima CLI and Docker API preflight pass.

**REFACTOR/checkpoint:** run the focused contracts, shell syntax, gitleaks,
and `make act-smoke`; record the runtime kind, Colima provider, and exact
limitation. Close only when the Colima CLI/API evidence and serial cold/warm
samples are recorded; missing or stopped Colima remains unavailable-safe.

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
keep hosted queue time separate from local wall time. Current Colima evidence:
two cold runs passed in 32,384/32,864 ms (556/562 ms image pulls) and five
serial warm runs passed in 29,753–31,347 ms (median 31,225 ms), with 2 CPUs,
4,104,118,272 bytes memory, a 571,284,183-byte image, and
`image_present_before_pull=true` for warm samples.
The Colima runs derived `cache_mode=cold`/ `warm`, normalized the endpoint,
verified the pinned digest and workflow hash, and verified retained-log and
artifact cleanup. All passed
with digest
`sha256:3d98df0137c62626482789b786d4bfe941d139baed30f237ebbabe363ea9bf08`.

### E2E-COV-009 — advisory feature synchronization

Close only after both children have their acceptance evidence, the parent
notes identify Colima provenance truthfully, and the advisory
contract remains excluded from hosted CI, coverage, security, SARIF, and
publish gates. Synchronize `.beads/issues.jsonl`, this plan,
`docs/reports/act-smoke-baseline.md`, BOM, checklist, codemap, and
`integration_log.jsonl` in one checkpoint.

### E2E-COV-010 — governed multi-level and agentic test evidence

The two children are implemented and agentic promotion is passed. The Colima
dependency is closed, and the TDD evidence contract, coverage audit, failure /
cleanup matrix, security gates, and documentation synchronization are recorded
in Beads and the integration log. The broader `E2E-COV` epic remains open for
additional product journeys.

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

### E2E-COV-009.1 — act/Colima capability preflight

**Goal:** make runtime availability and policy safety explicit before invoking
`nektos/act`.

**RED:** add tests for missing `act`, missing/stopped Colima, incompatible Docker API,
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
missing-runtime RED/GREEN contract. The earlier stopped-VM probe on this
Darwin x86_64 host returned `SKIPPED_UNAVAILABLE`, with no workflow invocation,
no SARIF/release/security side effects, empty secrets, and no
privileged/host/socket/credential mounts. The current available-runtime branch
is proven through the Colima CLI/Docker API, image digest, and resource-limit
evidence; legacy runtime selectors remain compatibility-only and are excluded
from plan closeout evidence.

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
with no workflow/SARIF/release/security side effects when Colima is absent.
The Colima runtime now has live CPU/memory/image-size/cache fields and
verified cleanup; the baseline is documented in
`docs/reports/act-smoke-baseline.md`. Colima cold/warm resource evidence is
recorded in the current checkpoint.

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
model/network access is complete; Beads closure is recorded and the broader
product-coverage epic remains open independently.

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
The fresh host-assisted `make publish-gate` closeout completed with local-full
cleanup receipt `20260713T045400Z-79992`; advisory data fetched successfully,
`cargo deny` reported all four policy classes `ok`, and the nightly branch
threshold command exited 0 at the governed 85.15625% value.

Focused workflow/runner tests, all package suites, `make workflow-gate`,
security gates, and coverage gates passed during this slice. The initial
sandboxed publish-gate attempt failed before product validation because the
Cargo advisory database path was read-only; the host-assisted rerun passed
that stage and the complete gate, proving the first failure was environmental.

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

## Slice 3: act/Colima advisory lane

Use `nektos/act` only for a synthetic, named workflow/job. Use Colima through
its Docker-compatible API after CLI/API preflight. Reject secrets,
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
