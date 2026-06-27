# SARIF Open Issues Remediation Roadmap

**Date**: 2026-06-27
**Scope**: GitHub origin issues `#2` and `#6`, Beads mirrors `RT-bu7` and `RT-0zv`
**Execution model**: `$plan-review-integrator` TDD refinement with Beads traceability
**Status**: active plan, ready for implementation
**Source context**: `crates/tachi-core/src/sarif_common.rs`, `crates/tachi-core/src/threats_sarif.rs`, `crates/tachi-core/src/risk_scores.rs`, `crates/tachi-core/tests/{risk_scores,threats_sarif,reporting_goldens}.rs`, GitHub issues `#2` and `#6`

## Executive Summary

The GitHub issue sweep closed remediated issues `#3`, `#4`, `#5`, and `#7`.
Two P1 SARIF contract issues remain open:

- `#2` / `RT-bu7`: `baselineRunId` is now symmetric across threat and risk
  SARIF outputs, but the value is still frozen in `sarif_common::baseline_run_id`.
- `#6` / `RT-0zv`: `logicalLocation.kind` is now shared across threat and risk
  SARIF outputs, but the shared mapping still emits `data-store`, which the
  issue identifies as non-standard.

This roadmap treats both as verified must-fix work because each affects SARIF
consumer correlation semantics. The implementation should not widen the command
surface unnecessarily and must preserve current CLI, shell, desktop, and MCP
adapter behavior.

## Plan-Review Integration Summary

| ID | Severity | Source | Summary | Category | Disposition |
|---|---:|---|---|---|---|
| PRI-F01 | HIGH | GitHub `#2`, Beads `RT-bu7` | Frozen SARIF `baselineRunId` remains after partial symmetry fix | Must-fix | Add dynamic run identity contract with regression tests |
| PRI-F02 | HIGH | GitHub `#6`, Beads `RT-0zv` | `logicalLocation.kind` still emits non-standard `data-store` | Must-fix | Replace/omit non-standard kind values with SARIF-compatible behavior |

Total findings: 2 | Must-fix: 2 | Bundle: 0 | Defer: 0 | Info: 0

Final Recommendation: Applied with caveats.

Dissent Ledger: none.

## Governance Gates

Both findings trigger the Security/Data Integrity Veto because SARIF output feeds
security dashboards, differential alerting, and downstream policy gates.

No finding triggers the Scope Expansion Veto. The remediation must stay inside
the existing SARIF builder and command-input seams unless tests prove the caller
contract requires a narrowly scoped argument.

## Stage 0 - Contract Discovery and Red Tests

**Goal**: prove the two residual defects with failing tests before production
code changes.

| Mapping | Value |
|---|---|
| Epic | `SARIF-OPEN-001` |
| Feature | `SARIF-OPEN-001.0` Residual SARIF contract proof |
| Capability | Reproduce remaining GitHub issue conditions on current `main` |
| Tasks | `RT-bu7a`, `RT-0zva` |
| Functions / seams | `sarif_common::baseline_run_id`, `build_threats_sarif`, `build_risk_scores_sarif`, SARIF golden tests |

Acceptance criteria:

- A focused test fails because `baselineRunId` cannot vary per run when the
  caller supplies a new run identity.
- A focused test fails because `Data Store` currently produces a non-standard
  `logicalLocation.kind` value.
- The failing tests identify the current behavior without changing unrelated
  SARIF fields.

Validation:

- `cargo test -p tachi-core --test risk_scores baseline_run_id_uses_supplied_run_identity --offline`
- `cargo test -p tachi-core --test threats_sarif baseline_run_id_uses_supplied_run_identity --offline`
- `cargo test -p tachi-core --test risk_scores logical_location_kind_omits_non_standard_data_store_kind --offline`
- `cargo test -p tachi-core --test threats_sarif logical_location_kind_omits_non_standard_data_store_kind --offline`

## Stage 1 - Dynamic Baseline Run Identity

**Goal**: close `#2` / `RT-bu7` by replacing the frozen baseline identity with an
explicit, testable contract shared by both SARIF pipelines.

Recommended implementation shape:

1. Add a small typed or borrowed SARIF options struct in `sarif_common.rs`, for
   example `SarifRunContext { baseline_run_id: Option<String> }`.
2. Thread the context through `build_threats_sarif` and `build_risk_scores_sarif`
   without changing result shapes unrelated to `baselineRunId`.
3. Preserve current CLI behavior with a deterministic fallback only when no
   caller value is supplied.
4. Add command-level coverage proving CLI/shell callers can supply or derive the
   run identity from the actual input artifact/run directory.
5. Update reporting goldens so both SARIF pipelines use identical semantics for
   new and unchanged findings.

Closure criteria:

- Existing findings in both SARIF outputs emit the supplied non-empty
  `baselineRunId`.
- New findings in both SARIF outputs preserve the current empty
  `baselineRunId` behavior, unless an ADR explicitly changes that contract.
- The static helper is either removed or renamed to make fallback behavior
  explicit, for example `fallback_baseline_run_id`.
- GitHub issue `#2` is closed with links to tests, PR, and final command output.
- Beads issue `RT-bu7` is closed with the same evidence and exported to
  `.beads/issues.jsonl`.

Validation:

- `cargo test -p tachi-core --test risk_scores --offline`
- `cargo test -p tachi-core --test threats_sarif --offline`
- `cargo test -p tachi-core --test reporting_goldens --offline`
- `cargo test -p tachi-shell --test tauri_bridge --offline`
- `cargo test -p tachi-cli --test control_plane_cli --offline`

## Stage 2 - SARIF Logical Location Kind Compliance

**Goal**: close `#6` / `RT-0zv` by making `logicalLocation.kind` SARIF-compliant
and shared across both pipelines.

Recommended implementation shape:

1. Replace `logical_location_kind_for_dfd_type(dfd_type) -> &'static str` with a
   function that can return no kind, for example
   `logical_location_kind_for_dfd_type(dfd_type) -> Option<&'static str>`.
2. Emit `kind` only when the mapping is known to be SARIF-compatible.
3. Treat `Data Store` as omitted unless a spec-backed standard value is
   documented in code and tests.
4. Keep `name` and `fullyQualifiedName` stable so alert correlation remains
   anchored even when `kind` is omitted.
5. Update both threat and risk-score tests to assert identical output shape for
   data stores.

Closure criteria:

- Threat and risk SARIF emit the same logical-location shape for `Data Store`.
- No emitted `logicalLocation.kind` value is one of the issue-flagged
  non-standard values: `data`, `data-store`.
- Tests prove the output remains SARIF schema-valid after omitting or replacing
  the kind.
- GitHub issue `#6` is closed with links to tests, PR, and final command output.
- Beads issue `RT-0zv` is closed with the same evidence and exported to
  `.beads/issues.jsonl`.

Validation:

- `cargo test -p tachi-core --test risk_scores --offline`
- `cargo test -p tachi-core --test threats_sarif --offline`
- `cargo test -p tachi-core --test reporting_goldens --offline`
- `cargo test -p tachi-cli --test control_plane_cli --offline`

## Stage 3 - Publish Gate and Remote Closure

**Goal**: prove no SARIF contract regressions and close remote trackers.

Acceptance criteria:

- `make publish-gate` passes on the feature branch.
- `make llvm-cov` remains at or above 85% line coverage.
- `gh pr checks --watch` is green for the remediation PR.
- `origin/main` contains the merge commit and post-merge workflows pass.
- GitHub issues `#2` and `#6` are closed with traceability comments.
- Beads issues `RT-bu7` and `RT-0zv` are closed and exported.
- `codemap.md`, BOM, and publish readiness docs are updated only if behavior or
  release gates changed.

Validation:

- `cargo fmt --all -- --check`
- `cargo test --workspace --all-targets --offline`
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- `make publish-gate`
- `cargo tree -i glib --locked --target all`
- `cargo tree -i gtk --locked --target all`
- `gh pr checks <PR> --watch --interval 10`

## Ranked Action Items

| Priority | Owner | Action | Source |
|---|---|---|---|
| P1 | implementer | Add failing tests for supplied dynamic `baselineRunId` across both SARIF builders | PRI-F01 |
| P1 | implementer | Thread explicit SARIF run context through threat/risk builders and command callers | PRI-F01 |
| P1 | implementer | Add failing tests proving `data-store` is no longer emitted as `logicalLocation.kind` | PRI-F02 |
| P1 | implementer | Change logical-location kind emission to omit or spec-map unsupported DFD kinds | PRI-F02 |
| P1 | reviewer | Verify all SARIF golden snapshots and command-level tests still preserve field-level correlation | PRI-F01, PRI-F02 |
| P2 | release owner | Close GitHub/Beads issues with exact PR and validation evidence after merge | PRI-F01, PRI-F02 |
