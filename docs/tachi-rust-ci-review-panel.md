# Tachi-Rust CI Plan Review Panel

**Date**: 2026-07-09  
**Scope reviewed**: `/Volumes/dev/Git-SCM/tachi-rust/docs/ci-improvement-plan.html` against live workflow, backlog, and Beads conventions  
**Method**: overseer-style parallel expert review with feasibility, governance, tracker-design, and devil's-advocate lenses

## Panel Verdict

Proceed with the CI improvement initiative, but not in the original order.

The panel agreed on the high-level direction: delta-aware PR CI is a good fit for `tachi-rust` if it preserves fail-closed security workflows, cross-platform specialist lanes, and full release/mainline validation. The panel also agreed that the source plan was not yet tracker-ready and was too optimistic about when narrowing could safely turn on.

## Consensus Findings

1. A new tracker hierarchy is required.
   - The closed `RT-TC*` namespace should remain historical.
   - The plan should open a fresh `RT-CI*` hierarchy.

2. Protected workflows must be frozen before routing narrows anything.
   - `rust-clippy.yml`, `gitleaks.yml`, `rust-supply-chain.yml`, `tachi-pytest.yml`, `tachi-mmdc-preflight.yml`, `rust-feature-coverage-canary.yml`, `release-please.yml`, and `fuzz-mutation-audit.yml` need explicit invariant coverage.

3. The router must be policy-driven and shadowed first.
   - A versioned route-policy manifest is safer than scattered workflow conditionals.
   - Observe-only mode with artifacted route decisions must precede active narrowing.

4. “Docs-only” is too blunt for this repo.
   - Passive docs and active contract docs must be separated.
   - `README.md`, selected `docs/**`, `.aod/**`, templates, and workflow-adjacent docs can still be contract-bearing.

5. Crate routing must use dependency closure.
   - Routing the changed crate alone is unsafe when downstream crates depend on `tachi-core` or `tachi-shell`.

## Key Refinements Adopted

- Added a new Phase 0 for baseline freeze, required/advisory workflow mapping, branch-protection inventory, and workflow gate realignment.
- Moved protected-workflow preservation ahead of routing activation.
- Split routing into route-spec, shadow mode, passive-docs narrowing, and dependency-closure narrowing.
- Added required-check migration rules so renamed or split lanes do not silently weaken merge protection.
- Added `route.json` as a durable machine-readable route artifact.
- Added an emergency full-CI override.
- Refined the Beads hierarchy to use tracker-ready `RT-CI-001` through `RT-CI-007` feature IDs with per-slice fields.

## Findings By Lens

### Feasibility

- The original adoption order was internally reversed: active narrowing was described before `rust-workspace.yml` was actually integrated with router outputs.
- The source plan promised outcomes that the current `workflow_ci_gates.rs` contract would still block.
- The rollout needed explicit evidence gates rather than only calendar targets.

### Governance

- The original plan omitted protected treatment for `release-please.yml` and `fuzz-mutation-audit.yml`.
- SARIF and security workflows need exact invariant preservation, not just broad “retain behavior” language.
- Shared-setup reuse needs its own security policy.

### Tracker Design

- The source plan was roadmap-shaped, not Beads-shaped.
- Broad phases bundled too many unrelated changes for TDD-sized cards.
- The plan needed per-slice fields such as dependencies, validation, owner, stage label, and next test seam.

### Devil's Advocate

- “Docs-only” could misroute active contract surfaces.
- Route decisions needed a durable artifact, not only summaries.
- Impact analysis had to account for dependency closure and non-crate global surfaces like `.aod/**`, `Makefile`, and workflow-adjacent scripts/docs.

## Output Impact

The review panel findings were integrated into:

- [tachi-rust-ci-execution-plan.md](/Users/neo/Documents/Codex/2026-07-09/new-chat/outputs/tachi-rust-ci-execution-plan.md)
- [tachi-rust-ci-beads-issue-cards.md](/Users/neo/Documents/Codex/2026-07-09/new-chat/outputs/tachi-rust-ci-beads-issue-cards.md)

## Residual Risks

- The plan still needs live tracker writes and repo edits before it becomes the canonical backlog.
- Branch protection and required-check migration need operator verification at implementation time.
- Advisory or scheduled workflow health should be rechecked live before rollout starts.
