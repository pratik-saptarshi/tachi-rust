# Implementation Backlog

**Last Updated**: 2026-10-04
**Purpose**: navigation hub for Rust implementation tracks and completed roadmap records
**Scope**: roadmap sequencing, issue-pack pointers, and task-template guidance

## Backlog Navigation

- [Rust-native upstream milestone delivery (2026-10-04)](./2026-10-04-upstream-milestone-delivery.md) — epic `RT-3zm`, three separately gated PR phases, including PR #42 review remediation.

- Active desktop host: `crates/tachi-desktop`
- [Beads issue mirror snapshot](../../.beads/issues.jsonl)
- [AISVS Dependabot remediation roadmap](./2026-06-23-aisvs-dependabot-remediation-roadmap.html.md)
- [GTK-Free Desktop GUI Host roadmap](./2026-06-26-gtk-free-desktop-gui-host-roadmap.html.md)
- [GTK-Free Desktop GUI Host issue cards](./2026-06-26-gtk-free-desktop-gui-host-issue-cards.md)
- [Rust-native E2E coverage expansion roadmap](./2026-07-10-e2e-coverage-expansion-roadmap.html.md)
- [Rust-native E2E coverage expansion issue cards](./2026-07-10-e2e-coverage-expansion-issue-cards.md)
- [Dependency and Runtime Upgrade Closeout plan](./2026-10-04-dependency-runtime-upgrade-closeout-v1.md)
- [PR #39 review and closeout remediation](./2026-10-04-pr39-remediation-v1.md)

## Current Status Snapshot

- Dependency/runtime upgrade closeout is active under the 2026-10-04 plan.
  Baseline reconciliation, pin verification, and local validation are complete.
  Separate commits and the PR are complete; all 16 required checks on PR #39
  passed and GitHub reports it mergeable. Live PostgreSQL migration evidence
  and merge remain open gates.

- PR #39 review remediation is active under GitHub issue #40 and Beads epic
  `RT-bz6`. The follow-up fixes the PostgreSQL 18 volume path, Supabase env
  template, user-table RLS, live Prisma migration evidence, and pending CodeQL
  ruleset bypass notice. PR #41 review also identified and corrected stale
  Rust toolchain values in the root codemap. The seven-child Beads hierarchy
  tracks all review action items.

- Live Beads audit on 2026-10-04: 13 open, 0 in progress, 0 blocked, and 197
  closed issues; 5 open issues belong to the dependency/runtime closeout
  hierarchy `RT-bbi` and children `.1` through `.4`, and 7 belong to the PR #39
  remediation hierarchy `RT-bz6` and children `.1` through `.7`.
- `RT-CI`, `E2E-COV`, and `DT-GUI` hierarchies are closed. Their plans and
  issue cards below are retained as completed execution records; future work
  should use new issues rather than reopening completed cards without review.
- The checked-in Beads export remains available at `../../.beads/issues.jsonl`.
- Dependency/runtime upgrade tracker state: `RT-bbi` is the active epic for
  [the 2026-10-04 closeout plan](./2026-10-04-dependency-runtime-upgrade-closeout-v1.md).
  Its children cover baseline/pin reconciliation, Rust and release gates,
  scaffold/Prisma validation, and docs/protected delivery. The export contains
  205 issues before the remediation hierarchy was created.
- PR #39 remediation tracker state: GitHub issue #40 anchors Beads epic
  `RT-bz6`; eight children cover PostgreSQL 18 volume migration, Supabase env
  setup, profile RLS, live PostgreSQL CI, CodeQL ruleset behavior,
  documentation/export synchronization, root atlas Rust version accuracy, and
  runnable scaffold Compose command paths.
  Hosted PostgreSQL migration and RLS assertions passed in run
  [37191445486](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37191445486).
  PR #41 final-head checks after commit `d027adb` require a live status refresh
  before merge readiness is reported. The refreshed export contains 214 issues.

## Archive Records

- [Adversarial Architecture and Test Quality Roadmap](./2026-06-22-adversarial-architecture-test-quality-roadmap.html.md)
- [Rust/Tauri Parity Remediation Roadmap](./2026-06-21-rust-tauri-parity-remediation-roadmap.html.md)
- [Rust/Tauri Parity Issue Cards](./2026-06-21-rust-tauri-parity-issue-cards.md)
- [Archived Docs Workflow-Version Inventory](./2026-06-21-archived-docs-workflow-version-inventory.md)
- [Archived Docs Workflow-Version Sweep Roadmap](./2026-06-21-archived-docs-workflow-version-sweep-roadmap.html.md)
- [Archived Docs Workflow-Version Sweep Issue Cards](./2026-06-21-archived-docs-workflow-version-sweep-issue-cards.md)
- [SARIF Open Issues remediation roadmap](./2026-06-27-sarif-open-issues-remediation-roadmap.html.md)
- [SARIF Open Issues remediation issue cards](./2026-06-27-sarif-open-issues-remediation-issue-cards.md)
- [Standalone MCP Server Roadmap](./2026-06-25-standalone-mcp-server-roadmap.html.md)
- [Standalone MCP Server Issue Cards](./2026-06-25-standalone-mcp-server-issue-cards.md)
- [Rust Toolchain Upgrade roadmap](./2026-07-05-rust-toolchain-upgrade-roadmap.html.md)
- [Rust Toolchain Upgrade issue cards](./2026-07-05-rust-toolchain-upgrade-issue-cards.md)
- [Rust/Tauri Migration Issue Pack](./2026-06-04-rust-tauri-issue-pack.md)
- [Rust/Tauri Implementation Roadmap](./2026-06-08-rust-tauri-only-roadmap.md)
- [Rust/Tauri Implementation Issue Cards](./2026-06-08-rust-tauri-only-issue-cards.md)

## Security Track Record

- [AISVS Dependabot remediation issue cards](./2026-06-23-aisvs-dependabot-remediation-issue-cards.md)
- [GTK-Free Desktop GUI Host roadmap](./2026-06-26-gtk-free-desktop-gui-host-roadmap.html.md)
- [GTK-Free Desktop GUI Host issue cards](./2026-06-26-gtk-free-desktop-gui-host-issue-cards.md)
- Epic: `RT-00i` AISVS framework and Dependabot remediation
- Completed GitHub issue mirrors: `RT-bu7` / `gh-2` dynamic SARIF
  `baselineRunId`, `RT-0zv` / `gh-6` SARIF logical-location kind compliance
- Feature issues: `RT-00i.1` typed AISVS control registry, `RT-00i.3`
  AISVS C01-C04, `RT-00i.4` AISVS C09-C12, and `RT-00i.6` CI/docs
  readiness are closed.
- Completed remediation slices: `RT-00i.2` remediate glib/tauri advisory,
  `RT-00i.2.5` retire the buildable `src-tauri` adapter, and `RT-00i.5`
  AISVS C05-C08 control cluster.
- Decision issues: `RT-00i.7` gtk/glib compatibility decision for Dependabot alert (closed decision note)
- Historical follow-up: the future workspace recheck was removed after the
  workspace dependency proof showed no `glib` package.

## CI Track Record

- [Tachi-Rust CI execution plan](../tachi-rust-ci-execution-plan.md)
- [Tachi-Rust CI Beads issue cards](../tachi-rust-ci-beads-issue-cards.md)
- [Tachi-Rust CI review panel](../tachi-rust-ci-review-panel.md)
- Epic: `RT-CI` Rust CI orchestration and delta-routing hardening
- Closed phase-1 guardrails: `RT-CI-002`, `RT-CI-002.1`, `RT-CI-002.1.1`,
  `RT-CI-002.2`, and `RT-CI-002.3`
- The `RT-CI` umbrella and all child issues are closed. Hosted timing and
  route evidence remain documented in the CI baseline and BOM.
- Tracker state: live Beads writes are exported to `../../.beads/issues.jsonl`
  after each slice; these CI plans and issue cards are completed records.

## E2E Coverage Track Record

- [Rust-native E2E coverage expansion roadmap](./2026-07-10-e2e-coverage-expansion-roadmap.html.md)
- [Rust-native E2E coverage expansion issue cards](./2026-07-10-e2e-coverage-expansion-issue-cards.md)
- Epic: `E2E-COV` Rust-native end-to-end coverage expansion — closed.
- Completed sequence: baseline contract → CLI/Desktop/MCP boundary slices →
  lifecycle and resilience composition → branch/line/region publish evidence.
- Latest recorded evidence (2026-07-13): four E2E modules (`crates/tachi-cli/tests/e2e_artifacts.rs`,
  `crates/tachi-desktop/tests/e2e_command_journey.rs`,
  `crates/tachi-mcp/tests/e2e_stdio_journey.rs`, and
  `crates/tachi-shell/tests/init_substitution.rs`). The historical closeout
  snapshot is 114 modules (13 unit, 96 integration, 1 smoke, 4 E2E); the later
  audit refresh is 119 modules (13 unit, 101 integration, 1 smoke, 4 E2E).
  At that audit, coverage was 93.24% lines / 92.60% regions; governed nightly
  coverage was 85.15625% branches (1,408 total / 210 missed).

## Rust Toolchain Modernization Track

- [Rust Toolchain Upgrade roadmap](./2026-07-05-rust-toolchain-upgrade-roadmap.html.md)
- [Rust Toolchain Upgrade issue cards](./2026-07-05-rust-toolchain-upgrade-issue-cards.md)
- Epic: `RT-TC` Rust toolchain modernization is closed.
- Done: `RT-TC-001` pinned toolchain, Rust `1.96` MSRV metadata, and path
  proof; `RT-TC-002` fail-closed supply-chain gates.
- Priority `0`: all currently materialized RT-TC P0 prerequisites are closed;
  future canary work should proceed only under a new issue with current
  validation evidence.
- Priority `1`: `RT-TC-003` converted workflow/reporting assertions to
  semantic YAML, workspace-derived, keyed JSON, and parsed rendering
  projections; `RT-TC-004` added pinned `cargo-hack`/`cargo-llvm-cov`
  manual/scheduled canaries; `RT-TC-005` recorded the standalone-adapter
  decision that is now superseded by completed `RT-00i.2.5` retirement of the
  buildable `src-tauri` surface.
- Priority `2`: `RT-TC-006` is implemented by
  [ADR-046](../architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md),
  which defers `smol-rs` runtime crates to a future MCP or desktop
  async-runtime feature with benchmarks, cancellation/shutdown tests,
  compatibility evidence, dependency diff, and rollback plan.
- Sequencing: P0 security and CI reproducibility work may run before
  non-blocking MCP/runtime follow-ups when it improves release confidence.
  Future security blockers should be tracked with new issues and current
  evidence.
- Tracker state: the approved remote schema migration has been applied and
  pushed, the live `RT-TC` hierarchy now exists, and `.beads/issues.jsonl` has
  been exported. Future toolchain updates should open a new Beads hierarchy and
  leave the closed RT-TC issue-card file as historical source text.

## Standalone MCP Track

- [Standalone MCP server roadmap](./2026-06-25-standalone-mcp-server-roadmap.html.md)
- [Standalone MCP server issue cards](./2026-06-25-standalone-mcp-server-issue-cards.md)
- Epic: `MCP-001` Standalone MCP server
- Feature issues: `MCP-001.1` canonical contract extraction, `MCP-001.2` MCP transport and tool layer, `MCP-001.3` transport runtime hardening, `MCP-001.4` docs and release integration, `MCP-001.5` portability envelope
- Task issues: `MCP-001.3.1` correlation and cancellation, `MCP-001.3.2` auth guard, `MCP-001.3.3` cleanup and timeout, `MCP-001.4.1` docs publish alignment, `MCP-001.4.2` BOM/release alignment, `MCP-001.4.3` CI evidence lane, `MCP-001.5.1` portability matrix
- Tracker state: all `MCP-001*` issues are closed in the Beads export. The
  roadmap remains a canonical historical source and future MCP work should open
  a new issue hierarchy instead of reusing closed tracker cards.

## Planning Reconciliation

- Live Beads state is authoritative for issue status; roadmap files remain
  authoritative for scope and acceptance criteria.
- The 2026-10-04 live audit reports five open/ready issues, all in `RT-bbi`.
  Earlier open-work snapshots in dated records are historical and must not be
  treated as current.
- After any live tracker write, run `bd export -o .beads/issues.jsonl` and
  update this backlog snapshot in the same commit.

## Historical Local Branch Reconciliation (2026-07-12 snapshot)

- At the time of this snapshot, `main` equaled `origin/main` at `b70e0f4` and
  PR #25 carried the timing-closeout corrections. These branch and PR details
  are historical and do not describe the current checkout.
- One worktree is present: `/Volumes/dev/Git-SCM/tachi-rust` on
  `feat/e2e-ci-timing-closeout` while PR #25 is under protected review.
- `feat/tauri-minimal-features` is a stale pre-main desktop branch. Its
  portable temp-root test changes and GTK-free desktop host work are already
  represented on `main`; direct merge would reintroduce obsolete toolchain,
  SARIF, MCP, and adapter state.
- `feat/rt009-publish-gate-bom-readiness` is a stale publish-gate branch from
  the earlier RT-009/RT-010 planning pass. Its useful publish/BOM/status
  concerns have been superseded by the current BOM, publish checklist, RT-TC
  closeout, and this backlog reconciliation; direct merge would rewind current
  roadmaps and gates.
- GitHub PR #10 (`issue-5`) and PR #11 (`issue-6`) were closed as superseded.
  The shell containment behavior from #10 is covered on `main` by
  `crates/tachi-shell/tests/control_plane.rs`, and the SARIF kind contract from
  #11 is covered by `crates/tachi-core/src/sarif_common.rs` plus the
  `risk_scores` and `threats_sarif` tests. The remote branches may remain as
  historical refs, but their PRs are no longer active merge candidates.
- Do not blindly merge either stale branch. Delete or archive local stale
  branches only with explicit operator approval.

## Canonical Sources

- [Adversarial Architecture and Test Quality Roadmap](./2026-06-22-adversarial-architecture-test-quality-roadmap.html.md)
- [Adversarial Architecture and Test Quality Issue Cards](./2026-06-22-adversarial-architecture-test-quality-issue-cards.md)
- [AISVS Dependabot remediation roadmap](./2026-06-23-aisvs-dependabot-remediation-roadmap.html.md)
- [AISVS Dependabot remediation issue cards](./2026-06-23-aisvs-dependabot-remediation-issue-cards.md)
- [GTK-Free Desktop GUI Host roadmap](./2026-06-26-gtk-free-desktop-gui-host-roadmap.html.md)
- [GTK-Free Desktop GUI Host issue cards](./2026-06-26-gtk-free-desktop-gui-host-issue-cards.md)
- [SARIF Open Issues remediation roadmap](./2026-06-27-sarif-open-issues-remediation-roadmap.html.md)
- [SARIF Open Issues remediation issue cards](./2026-06-27-sarif-open-issues-remediation-issue-cards.md)
- [Rust Toolchain Upgrade roadmap](./2026-07-05-rust-toolchain-upgrade-roadmap.html.md)
- [Rust Toolchain Upgrade issue cards](./2026-07-05-rust-toolchain-upgrade-issue-cards.md)
- [Rust/Tauri parity remediation roadmap](./2026-06-21-rust-tauri-parity-remediation-roadmap.html.md)
- [Rust/Tauri parity issue cards](./2026-06-21-rust-tauri-parity-issue-cards.md)
- [Standalone MCP Server Roadmap](./2026-06-25-standalone-mcp-server-roadmap.html.md)
- [Standalone MCP Server Issue Cards](./2026-06-25-standalone-mcp-server-issue-cards.md)
- [Archived Docs Workflow-Version Sweep Roadmap](./2026-06-21-archived-docs-workflow-version-sweep-roadmap.html.md)
- [Archived Docs Workflow-Version Sweep Issue Cards](./2026-06-21-archived-docs-workflow-version-sweep-issue-cards.md)
- [Rust/Tauri migration issue pack](./2026-06-04-rust-tauri-issue-pack.md)

For an active track, its roadmap is the canonical sequencing document. The
issue cards are the copy-paste execution templates that become Beads issues.
The archived issue pack remains the tracker-neutral historical baseline.
Completed roadmap slices remain linked as historical records after their
tracker cards close.

## Backlog Shape

Work is organized as:

`Epic -> Feature -> Capability -> Task -> Function`

- `Epic` states the migration outcome.
- `Feature` groups the work by crate or user-facing concern.
- `Capability` defines the behavior that must exist.
- `Task` is the smallest TDD-driven slice that can be completed and validated.
- `Function` names the concrete function, command, fixture, or test seam.

## Stage Map

1. Phase 0: parity harness
1. Phase 1: critical parity closure
1. Phase 2: output contract parity
1. Phase 3: CI and release hardening
1. Phase 4: exceed `tachi`

Each phase is a hard gate. Do not start the next phase until the current phase
has passed its exit criteria and validation matrix.

## TDD Policy

- Write the failing test before the production change.
- Verify the test fails for the intended reason before editing code.
- Keep the implementation slice minimal until the test passes.
- Validate the slice at the function, task, capability, feature, and epic
  levels.
- Repeat the red -> green -> refactor cycle for every slice.
- Do not batch unrelated work into a single Beads item.

## Validation Matrix

| Work type | Minimum proof | Preferred seam |
|---|---|---|
| Parser work | Unit tests plus integration fixtures | Parser module and malformed fixture set |
| CLI and config work | Command-level tests plus config parsing tests | `tachi-cli` entrypoints |
| Desktop host work | Host-parity tests plus desktop smoke checks | `crates/tachi-desktop` command boundary |
| Reporting work | Output-shape checks plus snapshot-style regression tests | `tachi-core` builders |
| Performance work | Benchmark or criterion gate | Hot-path functions and regressions |
| Docs work | Readability, consistency, and link checks | Roadmap and onboarding docs |

## Dependency Rules

- Parity harness work must land before any bridge, output, or release-hardening
  slice.
- Desktop host exposure must land before schema validation and desktop-only UX.
- Output goldens and parser regression tests must stabilize before release
  artifact work.
- Release hardening must land before desktop-specific differentiators.
- Keep dependencies at the capability or feature level when possible. Do not
  create task graphs that mirror every internal callsite.

## Beads Issue Template

Use this format when converting roadmap slices into Beads issues:

```md
Epic:
Feature:
Capability:
Task:
Function:
Dependencies:
Acceptance criteria:
Validation:
Implementation owner:
Stage label:
Next test seam:
Notes:
```

## Usage Order

1. Read the active roadmap to understand the intended sequencing.
1. Use the issue cards for tracker-ready Beads issue text.
1. Use the archived issue pack only for provenance and historical context.
1. Execute the task with TDD and validate the phase gate before advancing.
