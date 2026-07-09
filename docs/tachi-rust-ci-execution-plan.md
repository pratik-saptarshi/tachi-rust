# Tachi-Rust CI Execution Plan

**Date**: 2026-07-09  
**Source input**: `/Volumes/dev/Git-SCM/tachi-rust/docs/ci-improvement-plan.html`  
**Status**: proposed execution package; not yet written back to the repository  
**Primary objective**: reduce wasted pull-request CI time without weakening `tachi-rust`'s existing SARIF, supply-chain, cross-platform, and contract-specific guarantees

## Repo-State Reconciliation

- The source plan is currently an untracked draft on `main`.
- The Rust toolchain modernization hierarchy (`RT-TC*`) is already closed in Beads and must not be reused as the live tracker for this work.
- The backlog snapshot says future CI/toolchain work should open a new Beads hierarchy and keep `.beads/issues.jsonl`, the backlog snapshot, and the roadmap synchronized.
- The current workflow surface already contains protected specialist or privileged lanes:
  - `rust-clippy.yml`
  - `rust-supply-chain.yml`
  - `gitleaks.yml`
  - `tachi-pytest.yml`
  - `tachi-mmdc-preflight.yml`
  - `rust-feature-coverage-canary.yml`
  - `release-please.yml`
  - `fuzz-mutation-audit.yml`
- `rust-workspace.yml` still runs an unfiltered `pull_request` package matrix across all active workspace crates plus dedicated `tachi-shell` suite slices.
- Several workflow contracts are docs-sensitive today, so `docs_only` must distinguish passive docs from active contract surfaces such as `README.md`, selected `docs/**`, `.aod/**`, Typst templates, and workflow-adjacent command docs.

## Executive Judgment

Adopt the intent of the source plan, but change the rollout shape.

The current draft is directionally right about delta routing, fast-fail lanes, and PR concurrency. The main refinement is that `tachi-rust` should not jump directly from full-matrix PR validation to active narrowing. It needs:

- a new tracker hierarchy,
- a baseline freeze and contract-realignment stage,
- protected-workflow invariants,
- a versioned route-policy manifest,
- shadow-mode route proof,
- dependency-closure-aware package routing,
- and a required-check migration step before any lane is renamed or replaced.

## Non-Negotiables

- Preserve fail-closed `clippy` + SARIF behavior.
- Preserve `cargo audit` / `cargo deny` supply-chain gates.
- Preserve `gitleaks` SARIF upload semantics.
- Preserve `tachi-pytest` cross-platform init coverage.
- Preserve `tachi-mmdc-preflight` missing-renderer contract coverage.
- Preserve `release-please` and other privileged workflow semantics explicitly.
- Keep `main`, release refs, shared manifests, lockfiles, workflow files, active-doc contract surfaces, and unknown routing states on full mode.
- Never let the router silently reduce coverage; unknown or incomplete data must widen coverage, not narrow it.

## New Beads Hierarchy

- `Epic`: `RT-CI` / Rust CI orchestration and delta-routing hardening
- `Feature set`:
  - `RT-CI-001` baseline freeze and contract realignment
  - `RT-CI-002` fast-fail guardrails
  - `RT-CI-003` protected workflow contracts
  - `RT-CI-004` route-policy manifest and shadow mode
  - `RT-CI-005` controlled narrowing for passive docs and dependency-closure package routing
  - `RT-CI-006` shared setup and telemetry
  - `RT-CI-007` release policy, required-check migration, and closeout

## Phase Plan

### Phase 0 - Baseline Freeze And Contract Realignment

Purpose: reopen CI work on a fresh hierarchy, freeze the current contract, and capture the baseline required to judge optimization safely.

Work:

- Create `RT-CI` issue cards and export them into live Beads only after operator approval.
- Capture the current workflow inventory and matrix behavior.
- Classify each workflow as `required`, `specialist-required`, `privileged`, or `advisory`.
- Inventory current required-check and branch-protection expectations before renaming or splitting any lane.
- Update workflow contract tests so they can intentionally evolve away from the current unfiltered `rust-workspace` PR shape.
- Record the current wall-clock baseline for:
  - passive-docs PR shape,
  - dependency-closure crate-local change shape,
  - shared-manifest / lockfile shape,
  - workflow-change shape.
- Record the current package matrix and shell suite slices as the pre-router contract.
- Stabilize any advisory baseline needed for trustworthy rollout evidence before routing semantics change.

Acceptance criteria:

- The new hierarchy does not reuse `RT-TC*` identifiers.
- The repository has an explicit required/advisory workflow map and required-check migration note.
- Baseline evidence includes workflow list, matrix list, and at least one sample duration snapshot per PR shape.
- `crates/tachi-core/tests/workflow_ci_gates.rs` is ready to represent the routed-core-CI contract instead of the current unfiltered PR contract.
- `implementation-backlog.md` is ready to reference the new hierarchy once tracker writes happen.

Validation:

- `ls .github/workflows`
- `cargo test -p tachi-core --test workflow_ci_gates`
- `bd ready --json`
- duration capture from the current GitHub Actions runs or local operator notes

### Phase 1 - Fast-Fail Guardrails

Purpose: land the lowest-risk CI cost reductions before any routing logic changes execution scope.

Work:

- Add `concurrency` groups to PR-facing workflows so superseded runs cancel on the same ref.
- Add a dedicated workflow-parse lane using `actionlint`.
- Add an isolated `cargo fmt --check` lane for Rust-facing changes.
- Keep these lanes small, early, and visually obvious in the Actions graph.
- Do not narrow `rust-workspace.yml` yet; this phase adds early-fail signals, not routing.

Acceptance criteria:

- Superseded PR pushes cancel obsolete in-flight runs for PR-facing workflows.
- A malformed workflow file fails in a dedicated lane before heavy Rust package matrices start.
- A formatting-only regression fails in a dedicated lane even if `rust-workspace.yml` still exists unchanged during the first rollout step.

Validation:

- workflow contract test covering `concurrency`, parse-lane triggers, and fmt-lane commands
- broken-workflow fixture or semantic workflow assertion
- rustfmt drift fixture or workflow-level command assertion

### Phase 2 - Protected Workflow Contracts

Purpose: freeze the workflows whose semantics must remain invariant before routing is allowed to influence core CI breadth.

Work:

- Define a `Protected Workflow Contracts` table for:
  - `rust-clippy.yml`
  - `gitleaks.yml`
  - `rust-supply-chain.yml`
  - `tachi-pytest.yml`
  - `tachi-mmdc-preflight.yml`
  - `rust-feature-coverage-canary.yml`
  - `release-please.yml`
  - `fuzz-mutation-audit.yml`
- For SARIF-producing lanes, preserve exact invariants:
  - `security-events: write` where required,
  - `if: always()` upload behavior,
  - fail-closed status re-emission after upload,
  - any checksum or full-history requirements already present.
- Add trigger-contract tests for specialist workflows, not just existence or pinned-toolchain tests.
- Add a reusable CI security policy:
  - immutable action pinning for third-party actions,
  - default least-privilege permissions,
  - no `pull_request_target`,
  - no `secrets: inherit` by default,
  - governance review required for `.github/**` and `.github/actions/**`.

Acceptance criteria:

- Every protected workflow has a named invariant set for triggers, permissions, and non-negotiable behavior.
- Specialist workflows gain explicit trigger-contract test coverage where it is currently missing.
- Routing work is blocked until the protected-workflow contract table and tests are in place.

Validation:

- `cargo test -p tachi-core --test workflow_ci_gates`
- trigger-surface assertions for `tachi-pytest` and `tachi-mmdc-preflight`
- SARIF and permission assertions for `rust-clippy`, `gitleaks`, and release automation lanes

### Phase 3 - Route Policy Manifest And Shadow Mode

Purpose: make route decisions visible and testable before those decisions are allowed to narrow coverage.

Work:

- Define routing from a versioned policy manifest rather than scattered shell conditionals.
- Split the routing rollout into four gates:
  - `3A` route-spec manifest plus fixture tests,
  - `3B` shadow mode with artifacts only,
  - `3C` passive-docs narrowing,
  - `3D` dependency-closure package routing.
- Define the routing taxonomy:
  - `passive_docs_only`
  - `active_docs_contract`
  - `crate_local_dependency_closure`
  - `shared_shell`
  - `shared_manifest`
  - `lockfile`
  - `workflow_or_ci_script`
  - `release_or_mainline`
  - `unknown_fallback`
- Treat the following as automatic full-mode or specialist-lane escalation inputs:
  - `.github/**`
  - `.github/actions/**`
  - `Cargo.lock`
  - root/workspace `Cargo.toml`
  - shared scripts
  - `Makefile`
  - `.aod/**`
  - active contract docs and templates
  - release automation files
  - unmatched paths
- Add a router job or script that computes changed files from PR base/head or push before/after SHA.
- Emit both human-readable summaries and a machine-readable `route.json` artifact containing:
  - changed files,
  - matched rules,
  - unmatched files,
  - impacted crates after dependency closure,
  - selected workflows,
  - route mode,
  - fallback reason.
- Keep the router in read-only mode first: report decisions, but continue running the existing full PR matrix.
- Add fixtures for passive docs, active docs, leaf crate, shared crate, multi-crate, shell-only, lockfile, workflow, shared-script, `.aod` template, and unknown-path change sets.
- Make the router/orchestrator job the stable required check name so branch protection always has a durable top-level signal.

Acceptance criteria:

- Router outputs classify all required change shapes.
- Unknown inputs or missing diff data emit `unknown_fallback` and require full mode.
- Dependency closure is computed for impacted crate routing; changed-crate membership alone is not sufficient.
- The observe-only router can be enabled without changing the current package matrix behavior.
- Every shadow-mode run publishes `route.json` and a summary with `route`, `reason`, and `fallback reason`.

Validation:

- unit or fixture tests for routing vectors
- workflow CI gate assertions for router job presence, artifact emission, and output fields
- sampled observe-only PR runs showing visible route summaries and artifacts
- shadow-mode exit gate evidence: at least 10 shadow-mode PRs with zero missed specialist triggers and zero narrower-than-current routes, signed off by the security/governance owner

### Phase 4 - Controlled Narrowing

Purpose: turn on safe narrowing only after protected-workflow invariants and shadow-mode route evidence are already proven.

Work:

- Step `4A`: enable narrowing only for `passive_docs_only`.
- Step `4B`: enable dependency-closure-aware package routing for crate-local changes.
- Drive `rust-workspace.yml` package matrix selection from router outputs only after the route manifest, fixture coverage, and shadow-mode exit gate are green.
- Keep `shell-tests` as dedicated semantic slices; only narrow them where the route contract proves it is safe.
- Keep full mode for:
  - all escalated route classes,
  - release refs,
  - `main`,
  - route uncertainty,
  - emergency override.
- Add an emergency full-CI override via manual input or repo variable.
- Add job-summary mode labels such as `mode=passive_docs_only`, `mode=dependency_closure`, `mode=full_fallback`.

Acceptance criteria:

- Passive-docs PRs do not run the full package matrix.
- Crate-local changes run the impacted crate dependency closure plus required shared lanes.
- Shared manifest, lockfile, workflow, active-doc, or diff ambiguity forces full mode.
- Operators can tell which routing mode was used from the workflow summary and `route.json`.
- The emergency override forces full mode regardless of router output.

Validation:

- workflow CI gate tests for matrix narrowing semantics
- route-fixture tests for package selection
- pilot PR evidence showing passive-docs and dependency-closure narrowing without specialist-lane regressions
- override proof showing forced full mode

### Phase 5 - Shared Setup And Telemetry

Purpose: reduce duplicated setup cost after the routing model is already stable and false-negative risk has been audited.

Work:

- Extract repeated Rust setup, cache, toolchain proof, and OS package installation into a reusable composite action or reusable workflow.
- Keep lane-specific commands local to each workflow so failure ownership remains obvious.
- Add elapsed runtime summaries per heavy lane.
- Capture pre-router vs post-router median duration evidence for the first ten successful PR runs per narrowed route shape.
- Delay this phase until the router has survived shadow mode and initial narrowing without route regressions.

Acceptance criteria:

- Shared setup is defined once and consumed by PR-facing Rust workflows.
- Timing summaries are emitted for the heavy Rust lanes.
- Failure localization is not reduced by the setup refactor.
- Routing and setup reuse can be diagnosed independently.

Validation:

- workflow gate tests for shared setup reuse
- job summaries showing duration evidence
- evidence table comparing pre-router and post-router median PR durations

### Phase 6 - Release Policy, Required-Check Migration, And Closeout

Purpose: codify where delta routing applies and where the repository must always stay broad.

Work:

- Keep `main`, release refs, scheduled canaries, supply-chain scans, security lanes, and route uncertainty on explicit full mode.
- Add rollout criteria for promoting shadow-mode routing to default PR behavior.
- Add rollback criteria: any route misclassification, skipped specialist lane, or unexpected false green forces reversion to full mode.
- Add a required-check migration step: old and new checks dual-run until branch protection is updated and verified; no protected check disappears in the same PR that introduces its replacement.
- Update `implementation-backlog.md` and any roadmap/backlog references once live tracker writes happen.

Acceptance criteria:

- Release and `main` execution remains equal to or broader than current validation breadth.
- Rollback conditions are explicit and operator-readable.
- Required-check migration is staged and verified before lane renames or removals.
- Closeout evidence includes route-contract tests, protected-workflow preservation proof, and post-rollout timing metrics.

Validation:

- workflow contract tests for full-mode enforcement on `main` and release refs
- sampled `main` / release / manual runs proving full mode
- docs and backlog sync review

## TDD Execution Model

1. Write the failing route, trigger, or workflow contract test first.
2. Prove the failure is for the intended reason.
3. Implement the smallest workflow or script change that turns the test green.
4. Refactor only after the route contract, protected-workflow invariants, fallback behavior, and workflow summary remain explicit.
5. Export Beads state and update backlog/docs in the same change when tracker writes are live.

## Capability-Level Red -> Green -> Refactor Matrix

| Capability | Red | Green | Refactor |
|---|---|---|---|
| PR concurrency | Add workflow test asserting missing `concurrency` on PR-facing workflows. | Add branch/ref concurrency groups and prove superseded runs cancel. | Normalize concurrency naming across workflows. |
| Parse gate | Introduce broken workflow fixture or semantic assertion that currently reaches heavy jobs. | Add `actionlint` lane that fails before heavy execution. | Move parse invocation into shared CI helper only after behavior is stable. |
| Formatting gate | Add rustfmt drift fixture or command assertion showing formatting still depends on heavy lanes. | Add isolated fmt lane and prove drift fails there only. | Align naming and summaries after contract tests stabilize. |
| Protected workflows | Add failing tests showing specialist triggers, SARIF semantics, or permissions could drift. | Preserve exact trigger, permission, and fail-closed invariants for protected workflows. | Add anti-consolidation comments and ownership notes. |
| Route policy manifest | Add route fixtures for passive docs, active docs, lockfile, workflow, shared script, `.aod`, and unknown states. | Emit expected route outputs while still running full matrices. | Extract router logic into a tested script or action. |
| Shadow mode artifact | Add failing test proving route decision artifacts are absent or incomplete. | Emit `route.json` plus explicit summary fields on every routed PR. | Stabilize required-check naming and artifact schema. |
| Dependency-closure routing | Add failing tests asserting dependency closure is not computed for impacted crates. | Use router outputs to select impacted crates plus downstream dependents. | Tune package grouping once route correctness is stable. |
| Shared setup | Add workflow test proving repeated setup remains duplicated. | Introduce reusable setup without changing lane-specific commands. | Remove leftover duplication once routing and setup tests stay green. |

## Risk Register

| Risk | Why it matters | Mitigation |
|---|---|---|
| Docs-only false positive | Documentation paths may still affect active-doc runtime guidance or workflow docs contracts. | Distinguish passive docs from active contract docs; only passive-doc routes can narrow package execution. |
| Crate-impact undercount | Shared crates can affect downstream packages even when only one crate changed. | Compute impacted crates as reverse-dependency closure, not changed-crate membership alone. |
| Specialist workflow bypass | A central router can accidentally replace path-filtered contract workflows. | Freeze specialist workflows as protected standalone lanes and add semantic trigger tests. |
| Operator blind spots | If route decisions are not visible, a false green can look normal. | Emit route mode, impacted crates, and fallback reasons in job summaries and `route.json`. |
| Premature optimization | Narrowing before route proof can create untestable confidence gaps. | Require observe-only mode before active narrowing. |
| Required-check drift | Splitting or renaming lanes can break branch protection or silently weaken merge gates. | Dual-run old and new checks until branch protection is updated and verified. |

## Rollback Rules

- Revert to full PR matrix immediately if any route fixture mismatch appears.
- Revert to full PR matrix immediately if a specialist workflow fails to trigger on its contract surface.
- Revert to full PR matrix immediately if an unknown diff shape narrows coverage instead of widening it.
- Revert to full PR matrix immediately if required-check migration removes or renames a protected signal prematurely.
- Do not continue a partial rollout after a route-classification incident; land the rollback first, then reopen the affected Beads task.

## SMART Acceptance Criteria

- **Specific**: implement PR concurrency, fast-fail parse/fmt lanes, protected-workflow contract tests, route-policy shadow mode, then controlled passive-docs and dependency-closure package narrowing.
- **Measurable**: reduce median wall-clock time for passive-docs and dependency-closure PRs by at least 35% across the first 10 successful post-rollout runs of each shape.
- **Measurable**: 100% of malformed workflow edits fail in the parse lane before heavy Rust package execution starts.
- **Measurable**: 100% of route-fixture vectors produce the expected routing mode and escalation behavior.
- **Measurable**: 10 shadow-mode PRs complete with zero missed specialist triggers and zero narrower-than-current routes.
- **Achievable**: land Phases 1-3 without touching release breadth or removing any protected workflow.
- **Relevant**: preserve `tachi-rust`'s stronger governance posture while removing avoidable PR wait time.
- **Time-bound**: finish Phases 0-3 within 5 working days, Phase 4 pilot rollout within 8 working days, Phases 5-6 and evidence closeout within the following release-preparation window.

## Traceability Summary

| Source finding | Refined disposition | Plan action |
|---|---|---|
| `CI-01` delta routing | Must-fix, but only after protected-workflow freeze and shadow-mode proof | Phases 3-4 |
| `CI-02` parse + fmt fast-fail lanes | Must-fix | Phase 1 |
| `CI-03` preserve clippy/SARIF rigor | Preserve as invariant | Phase 2 |
| `CI-04` keep path-filtered specialist workflows | Preserve as invariant | Phase 2 |
| `CI-05` reduce duplicated setup | Bundle after routing stability | Phase 5 |
| `CI-06` PR concurrency cancellation | Must-fix, low-risk first slice | Phase 1 |

## Final Recommendation

Proceed, but as a new `RT-CI` execution package rather than an extension of `RT-TC`.

The safest rollout order is:

1. baseline freeze and required-check inventory,
2. concurrency + parse/fmt fast-fail lanes,
3. protected-workflow contract freeze,
4. route-policy manifest and shadow mode,
5. constrained passive-docs then dependency-closure narrowing,
6. shared setup and timing evidence,
7. release-policy and required-check closeout.
