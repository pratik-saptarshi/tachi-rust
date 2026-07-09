# Tachi-Rust CI Improvement Beads Issue Cards

**Date**: 2026-07-09  
**Source plan**: [tachi-rust-ci-execution-plan.md](/Users/neo/Documents/Codex/2026-07-09/new-chat/outputs/tachi-rust-ci-execution-plan.md)  
**Intended tracker namespace**: `RT-CI*`  
**Status**: proposed issue-card source text; not yet applied to live Beads

## Card Format

- `Epic`
- `Feature`
- `Capability`
- `Task`
- `User Story`
- `Function`
- `Dependencies`
- `Acceptance criteria`
- `Validation`
- `Implementation owner`
- `Stage label`
- `Next test seam`
- `Priority`
- `Notes`

## Epic

### RT-CI / Rust CI orchestration and delta-routing hardening

- `Epic`: `RT-CI` / Rust CI orchestration and delta-routing hardening
- `Feature`: fast-fail lanes, protected workflow contracts, delta-aware routing, required-check migration, and rollout evidence
- `Capability`: reduce wasted PR CI runtime without weakening governance, SARIF, cross-platform, or contract-specific validation
- `Task`: execute `RT-CI-001` through `RT-CI-007` in order
- `User Story`: As a maintainer, I want PR CI to narrow safely for low-risk changes while remaining full and explicit for releases, shared surfaces, active-doc contracts, and uncertain diffs.
- `Function`: `.github/workflows/*.yml`, route-policy manifest, routing helper script/action, `crates/tachi-core/tests/workflow_ci_gates.rs`, backlog/docs sync surfaces
- `Dependencies`: new hierarchy must be opened separately from the closed `RT-TC*` work
- `Acceptance criteria`:
  - The repo has a fresh CI hierarchy instead of reusing closed toolchain cards.
  - Protected workflow contracts are frozen before delta routing narrows execution.
  - Delta routing is proven in observe-only mode before it narrows execution.
  - Mainline and release refs always remain on explicit full mode.
- `Validation`: `cargo test -p tachi-core --test workflow_ci_gates`, `bd ready --json`, route-fixture evidence
- `Implementation owner`: CI/release owner
- `Stage label`: roadmap execution package
- `Next test seam`: workflow gate tests plus routing fixtures
- `Priority`: 0
- `Notes`: Export `.beads/issues.jsonl` and update `implementation-backlog.md` in the same change when live tracker writes happen.

## Feature 1 - Baseline Freeze And Contract Realignment

### RT-CI-001 - Baseline freeze and contract realignment

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-001` baseline freeze and contract realignment
- `Capability`: establish a measurable and governable baseline before CI semantics change
- `Task`: create the hierarchy, capture workflow inventory, classify required/advisory lanes, and record pre-router baseline timings
- `User Story`: As an operator, I want baseline evidence and a current contract map so we can prove the optimization helped and avoid weakening required checks.
- `Function`: `.github/workflows`, `docs/roadmap/implementation-backlog.md`, `.beads/issues.jsonl`
- `Dependencies`: none
- `Acceptance criteria`:
  - workflow inventory is recorded,
  - required-check / branch-protection expectations are recorded,
  - package and shell suite matrices are recorded,
  - baseline timing evidence exists for passive-docs, dependency-closure crate-local, lockfile, and workflow-change shapes.
- `Validation`: workflow inventory snapshot, `cargo test -p tachi-core --test workflow_ci_gates`, baseline evidence note
- `Implementation owner`: CI/release owner
- `Stage label`: P0 bootstrap
- `Next test seam`: none; documentation and evidence capture
- `Priority`: 0
- `Notes`: this card is the required entry point before any narrowing work starts.

#### RT-CI-001.1 - Realign workflow CI gate contract

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-001`
- `Capability`: contract tests can intentionally evolve from the current unfiltered PR behavior
- `Task`: update `workflow_ci_gates.rs` so routed-core-CI changes are represented deliberately instead of incidentally
- `User Story`: As a CI maintainer, I want the contract tests to block accidental CI weakening while still allowing planned orchestration changes.
- `Function`: `crates/tachi-core/tests/workflow_ci_gates.rs`
- `Dependencies`: `RT-CI-001`
- `Acceptance criteria`:
  - the current tests are inventoried by purpose,
  - future routed-core-CI semantics have explicit failing seams,
  - specialist workflow invariants are separated from `rust-workspace` breadth assumptions.
- `Validation`: targeted `cargo test -p tachi-core --test workflow_ci_gates`
- `Implementation owner`: test/CI owner
- `Stage label`: P0 contract
- `Next test seam`: workflow CI gate helpers
- `Priority`: 0
- `Notes`: prerequisite to route-aware CI changes.

## Feature 2 - Fast-Fail Guardrails

### RT-CI-002 - Add fast-fail CI guardrails

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-002` fast-fail guardrails
- `Capability`: cheap failures land before heavy matrices
- `Task`: add PR concurrency, workflow parsing, and formatting isolation
- `User Story`: As a contributor, I want obvious syntax and formatting failures to end quickly instead of consuming the full Rust matrix.
- `Function`: `.github/workflows/rust-workspace.yml`, new parse/fmt workflows, workflow CI gate tests
- `Dependencies`: `RT-CI-001.1`
- `Acceptance criteria`:
  - obsolete PR runs cancel,
  - malformed workflow edits fail before heavy Rust jobs,
  - formatting-only regressions fail in a dedicated lane.
- `Validation`: workflow gate tests plus sampled canceled-run evidence
- `Implementation owner`: CI maintainer
- `Stage label`: P0 guardrails
- `Next test seam`: workflow semantics for `concurrency`, parse triggers, and fmt commands
- `Priority`: 0
- `Notes`: lowest-risk compute reduction slice.

#### RT-CI-002.1 - PR concurrency discipline

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-002`
- `Capability`: branch/ref-level cancellation for superseded PR runs
- `Task`: add consistent `concurrency` groups to PR-facing workflows
- `User Story`: As a reviewer, I do not want outdated PR runs consuming queue time after a newer push exists.
- `Function`: PR-facing workflow YAML files
- `Dependencies`: `RT-CI-001.1`
- `Acceptance criteria`:
  - all PR-facing workflows define consistent concurrency semantics,
  - superseded pushes cancel earlier in-flight runs on the same PR ref.
- `Validation`: workflow CI gate assertions plus sampled Actions evidence
- `Implementation owner`: CI maintainer
- `Stage label`: P0.1
- `Next test seam`: workflow CI gate helper for `concurrency`
- `Priority`: 0
- `Notes`: apply before route logic so it cannot change test breadth.

#### RT-CI-002.1.1 - Add semantic concurrency test coverage

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-002`
- `Capability`: concurrency is protected from future drift
- `Task`: add or extend workflow tests to assert PR-facing workflows declare the required concurrency contract
- `User Story`: As a maintainer, I want concurrency policy locked by tests so future workflow refactors do not silently remove it.
- `Function`: `crates/tachi-core/tests/workflow_ci_gates.rs`
- `Dependencies`: `RT-CI-002.1`
- `Acceptance criteria`: the workflow tests fail if required PR-facing workflows lose their concurrency configuration
- `Validation`: targeted `cargo test -p tachi-core --test workflow_ci_gates`
- `Implementation owner`: test/CI owner
- `Stage label`: P0.1
- `Next test seam`: semantic YAML parse assertions
- `Priority`: 0
- `Notes`: failing test first.

#### RT-CI-002.2 - Workflow parse gate

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-002`
- `Capability`: workflow syntax and expression failures stop before heavy jobs
- `Task`: add an `actionlint` workflow or early lane
- `User Story`: As a maintainer, I want broken workflow edits to fail in under two minutes.
- `Function`: new `ci-workflow-parse` workflow, workflow fixtures/tests
- `Dependencies`: `RT-CI-001.1`
- `Acceptance criteria`:
  - broken workflow syntax fails the parse lane,
  - the parse lane starts before heavy package execution.
- `Validation`: workflow gate tests and a deliberately broken workflow fixture
- `Implementation owner`: CI maintainer
- `Stage label`: P0.2
- `Next test seam`: parse-lane trigger and command assertions
- `Priority`: 0
- `Notes`: keep lane small and readable in the graph.

#### RT-CI-002.3 - Isolated Rust formatting gate

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-002`
- `Capability`: formatting drift fails independently of package test matrices
- `Task`: add dedicated `cargo fmt --check` workflow or early lane
- `User Story`: As a contributor, I want formatting drift to fail fast without waiting for unrelated tests.
- `Function`: new `rust-fmt` workflow, workflow gate tests
- `Dependencies`: `RT-CI-001.1`
- `Acceptance criteria`:
  - formatting drift fails in an isolated lane,
  - lane trigger scope covers Rust source/tests/manifests appropriately.
- `Validation`: workflow gate tests and rustfmt drift fixture
- `Implementation owner`: CI maintainer
- `Stage label`: P0.3
- `Next test seam`: format-lane trigger and command assertions
- `Priority`: 0
- `Notes`: preserve the package matrix for behavioral failures only.

## Feature 3 - Protected Workflow Contracts

### RT-CI-003 - Freeze protected workflow contracts

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-003` protected workflow contracts
- `Capability`: specialist and privileged workflows are frozen before routing can influence breadth
- `Task`: define invariants, permissions, and trigger contracts for protected workflows
- `User Story`: As a governance owner, I want routing work to be unable to bypass or weaken specialist and privileged workflows.
- `Function`: protected workflow YAML files, workflow gate tests, roadmap/backlog notes
- `Dependencies`: `RT-CI-001.1`, `RT-CI-002`
- `Acceptance criteria`:
  - protected workflows have explicit invariant tables,
  - trigger-contract tests exist for docs-sensitive specialist workflows,
  - SARIF and privileged-workflow invariants are explicit and testable.
- `Validation`: `cargo test -p tachi-core --test workflow_ci_gates`
- `Implementation owner`: security/governance owner
- `Stage label`: P0.4
- `Next test seam`: trigger-contract assertions
- `Priority`: 0
- `Notes`: do not allow routing-driven narrowing before this feature is complete.

#### RT-CI-003.1 - Specialist trigger-contract tests

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-003`
- `Capability`: docs-sensitive and path-filtered specialist workflows are protected by semantic tests
- `Task`: add trigger-contract assertions for `tachi-pytest` and `tachi-mmdc-preflight`
- `User Story`: As a maintainer, I want docs and template surfaces that carry runtime contracts to stay protected during CI refactors.
- `Function`: `tachi-pytest.yml`, `tachi-mmdc-preflight.yml`, workflow gate tests
- `Dependencies`: `RT-CI-003`
- `Acceptance criteria`: contract-surface changes that must and must not trigger each workflow are asserted explicitly
- `Validation`: trigger-surface assertions
- `Implementation owner`: governance/test owner
- `Stage label`: P0.4
- `Next test seam`: workflow trigger parser assertions
- `Priority`: 0
- `Notes`: protects active-doc and template-sensitive surfaces.

#### RT-CI-003.2 - SARIF and privileged workflow invariant lock

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-003`
- `Capability`: SARIF-producing and privileged workflows keep exact governance invariants
- `Task`: lock down `rust-clippy`, `gitleaks`, `rust-supply-chain`, `release-please`, and `fuzz-mutation-audit` invariants
- `User Story`: As a security owner, I want permissions, upload behavior, and privileged workflow assumptions to remain stable during refactors.
- `Function`: corresponding workflow YAML files and workflow gate tests
- `Dependencies`: `RT-CI-003`
- `Acceptance criteria`: invariants for permissions, upload behavior, and special checkout/download behavior are testable
- `Validation`: workflow gate assertions
- `Implementation owner`: security/governance owner
- `Stage label`: P0.4
- `Next test seam`: SARIF and privileged-workflow assertions
- `Priority`: 0
- `Notes`: includes `security-events: write`, `if: always()`, and checksum/full-history requirements.

## Feature 4 - Route Policy Manifest And Shadow Mode

### RT-CI-004 - Define route policy manifest and observe-only proof

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-004` route policy manifest and observe-only proof
- `Capability`: route decisions are visible, testable, and safe before they narrow execution
- `Task`: implement the router in report-only mode with a versioned policy manifest and fixture coverage
- `User Story`: As a maintainer, I want to see exactly what the router would do before trusting it to change CI breadth.
- `Function`: router script/action, route policy manifest, `rust-workspace.yml`, workflow gate tests, routing fixtures
- `Dependencies`: `RT-CI-003`
- `Acceptance criteria`:
  - route manifest covers passive docs, active docs, crate dependency closure, shared surfaces, release/mainline, and unknown fallback,
  - route outputs are emitted in logs, summaries, and `route.json`,
  - the existing matrix remains unchanged while the router is observe-only.
- `Validation`: routing fixtures, workflow gate tests, observe-only run evidence
- `Implementation owner`: CI/release owner
- `Stage label`: P1 shadow
- `Next test seam`: routing vector fixture suite
- `Priority`: 1
- `Notes`: no narrowing in this feature.

#### RT-CI-004.1 - Encode full-mode escalation rules

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-004`
- `Capability`: risky or ambiguous surfaces always widen to full mode
- `Task`: encode escalation rules for `.github/**`, `.github/actions/**`, lockfiles, root manifests, shared scripts, `.aod/**`, release automation, active docs, and unknown paths
- `User Story`: As an operator, I want ambiguous or high-blast-radius changes to force full validation automatically.
- `Function`: route manifest, routing fixtures
- `Dependencies`: `RT-CI-003`
- `Acceptance criteria`: all escalation vectors map to full mode or the relevant specialist lane and are visible in `route.json`
- `Validation`: route-fixture suite
- `Implementation owner`: CI/release owner
- `Stage label`: P1 shadow
- `Next test seam`: escalation-vector fixtures
- `Priority`: 1
- `Notes`: this is the core safety invariant for the router.

#### RT-CI-004.2 - Add route-fixture contract tests

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-004`
- `Capability`: route decisions are protected from regression
- `Task`: add fixture cases for passive docs, active docs, leaf crate, shared crate, lockfile, workflow, shell-only, `.aod`, and unknown shapes
- `User Story`: As a maintainer, I want route decisions to be deterministic and reviewable.
- `Function`: routing test module and/or workflow fixture harness
- `Dependencies`: `RT-CI-004.1`
- `Acceptance criteria`: each fixture asserts route mode, impacted packages or dependency closure, specialist lanes, and fallback reason
- `Validation`: targeted routing tests
- `Implementation owner`: test/CI owner
- `Stage label`: P1 shadow
- `Next test seam`: route fixture harness
- `Priority`: 1
- `Notes`: include active-doc-sensitive fixtures explicitly.

#### RT-CI-004.3 - Emit route artifact and stable orchestrator check

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-004`
- `Capability`: every routed PR leaves a durable decision record
- `Task`: emit `route.json` and make the router/orchestrator job the stable required check name
- `User Story`: As a reviewer, I want to tell the difference between an intentionally skipped path and a false-negative route.
- `Function`: router job, workflow summaries, artifact upload
- `Dependencies`: `RT-CI-004.2`
- `Acceptance criteria`: each routed PR publishes the artifact and the required orchestrator signal
- `Validation`: sampled observe-only runs
- `Implementation owner`: CI maintainer
- `Stage label`: P1 shadow
- `Next test seam`: artifact and summary assertions
- `Priority`: 1
- `Notes`: include `route`, `reason`, and `fallback reason` in the summary.

## Feature 5 - Controlled Narrowing

### RT-CI-005 - Enable constrained delta-aware execution

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-005` controlled narrowing
- `Capability`: the workspace matrix narrows only for approved low-risk route shapes
- `Task`: connect `rust-workspace.yml` to router outputs after observe-only proof
- `User Story`: As a contributor, I want passive-docs and dependency-closure PRs to finish substantially faster without hiding risky changes.
- `Function`: `.github/workflows/rust-workspace.yml`, router outputs, package-selection tests
- `Dependencies`: `RT-CI-004`
- `Acceptance criteria`:
  - passive-docs PRs avoid the full package matrix,
  - dependency-closure route selects impacted crates plus downstream dependents,
  - shared surfaces and unknown routes remain full mode,
  - route mode is visible in workflow summaries and `route.json`.
- `Validation`: package-selection tests plus pilot PR evidence
- `Implementation owner`: CI maintainer
- `Stage label`: P1.1
- `Next test seam`: matrix selection and summary assertions
- `Priority`: 1
- `Notes`: first active narrowing slice; rollback immediately on any misroute.

#### RT-CI-005.1 - Passive-docs narrowing

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-005`
- `Capability`: passive docs can skip package-heavy work without skipping governance-sensitive checks
- `Task`: enable narrowing only for passive-docs routes first
- `User Story`: As a documentation contributor, I want faster feedback for passive docs while active docs still keep contract coverage.
- `Function`: router mode handling, workflow conditionals
- `Dependencies`: `RT-CI-004`
- `Acceptance criteria`: passive-docs route skips full package matrix but preserves required governance and specialist checks
- `Validation`: passive-docs route fixtures plus sampled PR evidence
- `Implementation owner`: CI maintainer
- `Stage label`: P1.1
- `Next test seam`: passive-docs route fixtures
- `Priority`: 1
- `Notes`: do not combine with dependency-closure routing in the same activation step.

#### RT-CI-005.2 - Dependency-closure workspace routing

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-005`
- `Capability`: crate-local routes cover downstream dependency closure instead of only the changed crate
- `Task`: implement closure-aware package selection for `rust-workspace.yml`
- `User Story`: As a maintainer, I want a `tachi-core` or `tachi-shell` change to validate the packages that depend on it.
- `Function`: router package mapping, workflow matrix generation
- `Dependencies`: `RT-CI-004`
- `Acceptance criteria`: known crate-local fixtures select the expected dependency closure and no unsafe narrower set
- `Validation`: routing fixtures and workflow gate assertions
- `Implementation owner`: CI maintainer
- `Stage label`: P1.2
- `Next test seam`: package-closure fixture table
- `Priority`: 1
- `Notes`: shared-shell and shared-manifest escalation still override this route.

#### RT-CI-005.3 - Emergency full-CI override

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-005`
- `Capability`: operators can force safety mode regardless of route output
- `Task`: add manual or variable-driven full-mode override
- `User Story`: As a release owner, I want a one-step escape hatch if routing confidence is shaken.
- `Function`: router job, workflow conditionals
- `Dependencies`: `RT-CI-004`
- `Acceptance criteria`: override path forces full mode and is visible in the route summary and artifact
- `Validation`: override proof run
- `Implementation owner`: CI/release owner
- `Stage label`: P1.2
- `Next test seam`: override assertions
- `Priority`: 1
- `Notes`: required before broad rollout.

## Feature 6 - Shared Setup And Telemetry

### RT-CI-006 - Extract shared setup and runtime telemetry

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-006` shared setup and telemetry
- `Capability`: repeated setup cost drops without reducing failure localization
- `Task`: extract shared setup and add timing summaries after routing semantics stabilize
- `User Story`: As a DevEx maintainer, I want CI setup to be easier to maintain and easier to optimize with real timing data.
- `Function`: reusable composite action or workflow, workflow summaries, CI docs
- `Dependencies`: `RT-CI-005`
- `Acceptance criteria`:
  - shared setup is defined once,
  - heavy lanes emit timing summaries,
  - failure ownership remains lane-specific.
- `Validation`: workflow gate tests and summary evidence
- `Implementation owner`: DevEx maintainer
- `Stage label`: P1.3
- `Next test seam`: shared-setup usage assertions
- `Priority`: 1
- `Notes`: do not start until route correctness is stable.

#### RT-CI-006.1 - Shared Rust setup action

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-006`
- `Capability`: common toolchain/cache/bootstrap logic is centralized
- `Task`: create a reusable setup unit for checkout-adjacent Rust proof, cache, and OS package steps where appropriate
- `User Story`: As a maintainer, I want one place to update common Rust CI setup.
- `Function`: reusable setup action/workflow and consuming workflows
- `Dependencies`: `RT-CI-005`
- `Acceptance criteria`: PR-facing Rust workflows consume one shared setup definition
- `Validation`: workflow CI gate assertions
- `Implementation owner`: DevEx maintainer
- `Stage label`: P1.3
- `Next test seam`: shared setup invocation assertions
- `Priority`: 1
- `Notes`: keep lane-specific run commands out of the shared layer.

#### RT-CI-006.2 - Timing evidence and median reduction proof

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-006`
- `Capability`: optimization decisions are backed by measured run-time evidence
- `Task`: emit timing summaries and compare pre-router vs post-router medians for the first ten successful passive-docs and dependency-closure runs
- `User Story`: As a release owner, I want evidence that the new CI shape is materially faster and still safe.
- `Function`: workflow summaries, rollout evidence note, backlog closeout docs
- `Dependencies`: `RT-CI-005`
- `Acceptance criteria`: median timing evidence is captured and compared against baseline
- `Validation`: workflow summary exports and evidence note
- `Implementation owner`: DevEx maintainer
- `Stage label`: P2.2
- `Next test seam`: summary-format assertions if automated
- `Priority`: 2
- `Notes`: required for closeout, not for first pilot enablement.

## Feature 7 - Release Policy And Closeout

### RT-CI-007 - Enforce full mode for release/mainline and close out evidence

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-007` rollout, release policy, required-check migration, and evidence closeout
- `Capability`: delta routing is bounded to PR optimization only; release confidence never narrows
- `Task`: codify full-mode enforcement and update roadmap/backlog references after rollout evidence is complete
- `User Story`: As a release owner, I want PR optimization without any ambiguity about release or `main` coverage breadth.
- `Function`: workflow conditionals, branch protection settings, backlog docs, Beads export
- `Dependencies`: `RT-CI-005`, `RT-CI-006`
- `Acceptance criteria`:
  - `main`, release refs, scheduled security/canary lanes, and unknown states always run full mode,
  - old and new required checks dual-run until branch protection is updated and verified,
  - rollback triggers are documented,
  - backlog/docs/tracker references are synchronized.
- `Validation`: workflow tests, sampled full-mode runs, docs review
- `Implementation owner`: release/toolchain owner
- `Stage label`: P2 closeout
- `Next test seam`: full-mode event assertions
- `Priority`: 2
- `Notes`: this is the final closeout gate for the hierarchy.

#### RT-CI-007.1 - Full-mode enforcement

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-007`
- `Capability`: release and `main` coverage cannot be narrowed by route logic
- `Task`: assert full-mode behavior for `main`, release refs, scheduled lanes, and route uncertainty
- `User Story`: As a release owner, I want the safety path to be automatic and obvious.
- `Function`: workflow routing conditions, workflow gate tests
- `Dependencies`: `RT-CI-005`
- `Acceptance criteria`: route conditions force full mode for all protected events and ambiguity cases
- `Validation`: workflow gate tests and sampled runs
- `Implementation owner`: release/toolchain owner
- `Stage label`: P2
- `Next test seam`: event-to-mode assertions
- `Priority`: 2
- `Notes`: this is the last blocker before hierarchy closure.

#### RT-CI-007.2 - Required-check migration and backlog synchronization

- `Epic`: `RT-CI`
- `Feature`: `RT-CI-007`
- `Capability`: branch protection, backlog, and tracker agree on the live CI state
- `Task`: dual-run required checks during migration, then export `.beads/issues.jsonl`, update `implementation-backlog.md`, and archive the issue-card source when the hierarchy closes
- `User Story`: As a future maintainer, I want one authoritative live status and one safe required-check migration path.
- `Function`: branch protection settings, `.beads/issues.jsonl`, `docs/roadmap/implementation-backlog.md`, archival roadmap references
- `Dependencies`: all prior `RT-CI*` work
- `Acceptance criteria`: protected check migration is verified before old checks disappear; tracker export and docs snapshot are updated in the same change
- `Validation`: branch-protection review, `bd export -o .beads/issues.jsonl`, docs review, `bd ready --json`
- `Implementation owner`: roadmap/tracker owner
- `Stage label`: closeout
- `Next test seam`: none
- `Priority`: 2
- `Notes`: mirror the repo's existing backlog hygiene rules.
