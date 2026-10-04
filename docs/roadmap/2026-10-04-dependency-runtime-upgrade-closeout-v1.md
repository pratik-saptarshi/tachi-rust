# Dependency and Runtime Upgrade Closeout

**Date:** 2026-10-04
**Status:** PR #39 is mergeable; all required checks passed at the implementation head
**Scope:** Rust workspace, frontend scaffolds, CI/runtime pins, supply-chain and release evidence.

## Baseline and preservation

- Preserve the existing E2E closeout commit and every current worktree change.
- The selected upstream baseline is `origin/main` at `35be8778c3aacea179224676d42c45a6f50a33b7` (PR #38, taxonomy and safe image cleanup).
- Review the full diff before staging. Keep E2E closeout, dependency/runtime implementation, and documentation synchronization in separate reviewable Conventional Commit slices.
- Create a new Beads hierarchy for this upgrade. Do not reopen or reuse completed issue IDs. Export `.beads/issues.jsonl` and refresh this backlog in the documentation slice.

## Work plan

1. **Reconcile baseline.** Refresh `origin/main`; compare the complete branch and worktree with it. Preserve the pre-existing E2E documentation reconciliation separately from upgrade changes. Reconcile overlapping dependency and documentation edits against PR #37/#38 before staging.
2. **Verify pins.** Confirm toolchain, direct Cargo and npm packages, GitHub Actions, container images, and helper tools against their authoritative release sources, using `rtk` for network checks. Keep exact direct pins and committed lockfiles; update recorded values if sources show drift.
3. **Finish compatibility.** Complete Prisma validation against PostgreSQL when a local server is available. Record command output and environmental limits; schema validation or generated SQL alone does not prove a live migration.
4. **Run validation.** Run locked Rust formatting, workspace Clippy, supply-chain/security gates, and workspace tests. Reproduce the known runner-contract failures and shell init-test stall before classifying them. For all three frontend scaffolds run clean lockfile installation, lint, tests, and build; validate Prisma generation and migration SQL. Run workflow, scaffold dependency, and release-readiness gates after package checks.
5. **Synchronize evidence.** Update the BOM, publish checklist, codemap, backlog, tracker snapshot, and dated upgrade record with exact command results, versions, and any baseline/environment blockers.
6. **Protected delivery.** Separate the E2E, implementation, and documentation commits; push the reviewed branch, open a PR under repository CI guidance, and wait for all required protected checks before declaring merge readiness.

## Acceptance criteria

- Rust formatting, Clippy, security/dependency gates, and workspace tests pass, or independently reproduced baseline failures are identified with exact evidence.
- Each frontend scaffold installs from its committed lockfile and passes lint, tests, and build. Prisma validation, client generation, and migration SQL checks pass; live PostgreSQL migration status is explicit.
- Workflow, scaffold, and release-readiness gates pass. Protected PR checks are terminal and green before merge readiness is reported.
- Roadmap, Beads export, backlog, BOM, checklist, codemap, and dated upgrade record agree on scope and evidence.

## Execution log

- Refreshed `origin/main` on 2026-10-04 to `35be8778c3aacea179224676d42c45a6f50a33b7` (PR #38). The original `chore/e2e-post-merge-closeout` checkout and its existing user changes remain intact. Work is isolated in `.worktrees/dependency-runtime-upgrade-closeout` on `chore/dependency-runtime-upgrade-closeout`, created from that current main commit.
- Preserved and reviewed the complete changeset. The E2E documentation reconciliation is a separate commit slice from the dependency/runtime implementation and synchronized status/evidence documents. The new Beads hierarchy is epic `RT-bbi` with children `.1` through `.4`; `.beads/issues.jsonl` contains 205 issues (5 open/ready, 0 in progress, 0 blocked, 197 closed).
- Registry and upstream release checks ran through `rtk`. Exact Cargo and npm pins were checked; both Vite `npm outdated` reports were empty, and Next.js reported only Prisma 8.0.0-rc.19 above the selected latest stable Prisma 7.10.0. Rust 1.99.0 is the active stable toolchain. GitHub release metadata confirmed upload-artifact v7.0.1, checkout v7.0.1, setup-python v7.0.0, and Typst setup v5.3.0. The Postgres 18 Alpine manifest and pinned Cargo helper releases were also checked. Upload-artifact workflows were advanced to v7 and the route workflow’s trusted-content SHA-256 was refreshed.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `cargo test --workspace --locked` pass. `make workflow-gate`, `make scaffold-dependency-gate` (6/6), `make supply-chain-gate` (1,290 advisories; advisories, bans, licenses, and sources all pass), `make release-gate`, and the full `make publish-gate` pass on this branch. The final publish gate measured 85.08174386920983% nightly branch coverage, above the 85% threshold. Focused offline tests were added for the PR #38 taxonomy link monitor after the current-main baseline first measured 83.88%.
- The runner-contract timeout case failed under nightly instrumentation with a one-second fake-shell timeout because its TERM trap had not started. The fixture timeout is now five seconds; all four runner-contract tests pass under nightly coverage and the stable workspace suite. The shell `init_constitution` tests pass (2/2); no shell init-test stall was reproduced.
- Each of the three frontends completed clean lockfile installation, lint, configured tests, and production build. All three have no test files yet; Vitest is configured with `--passWithNoTests`. Next.js also passes Prisma client generation, `prisma validate`, migration SQL generation (schema, users table, primary key, and unique email index), and production build. Live PostgreSQL migration is unavailable: `pg_isready` is not installed, localhost port 5432 is closed, and the Colima Docker socket is inaccessible.
- Workflow, scaffold, RustSec, and secret-scan gates pass in the aggregate publish gate. Git diff whitespace validation passes. Package/build outputs were removed after validation and are not part of the changeset.
- PR #39 is open at `https://github.com/pratik-saptarshi/tachi-rust/pull/39`. At implementation head `a3de8c7ed9445b05b32d202db50ab4ef858f9eb6`, GitHub reports all 16 required branch-protection contexts successful; the check summary has 23 successes, 0 failures, and one skipped/cancelled optional job. GitHub reports the PR as mergeable, with no required review configured. The PR is not merged.
- The protected-check snapshot above predates this final documentation-only synchronization commit. Verify the required check rollup again against the final PR head before reporting merge readiness. Live PostgreSQL migration evidence remains unverified because the local server/runtime is unavailable.

## Delivery record

The changes are split into these reviewed commit slices:

- E2E and desktop roadmap reconciliation: `5b00f22`.
- Dependency/runtime implementation and scaffold updates: `59e69da`.
- Documentation, backlog, BOM, checklist, codemap, and Beads export: this closeout commit.

The branch uses the Beads scope `RT-bbi`; repository guidance prefers a
zero-padded GitHub issue number, but no matching GitHub issue was found during
the read-only issue search. PR #39 is open and mergeable with all 16 required
contexts passing at the implementation head. This closeout has not been merged.
