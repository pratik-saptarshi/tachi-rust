# Rust-Native End-to-End Coverage Expansion Roadmap

**Status**: Proposed for Beads materialization and TDD execution
**Date**: 2026-07-10
**Scope**: CLI, desktop-host, MCP-stdio, lifecycle, and cross-boundary failure/cancellation workflows
**Primary tracker namespace**: `E2E-COV*`

## Executive decision

The repository started with one genuine end-to-end module, `crates/tachi-shell/tests/init_substitution.rs`, with five passing tests. The expansion now adds `crates/tachi-cli/tests/e2e_artifacts.rs`, `crates/tachi-desktop/tests/e2e_command_journey.rs`, and `crates/tachi-mcp/tests/e2e_stdio_journey.rs`, while the init module also composes install, update, and analysis artifact delivery. Initialization, CLI artifacts, desktop commands, MCP stdio, the full local lifecycle, and the cross-boundary failure matrix are now covered; coverage-governance and branch-evidence work remains open.

This roadmap expands E2E coverage around stable user-facing boundaries while preserving the existing Rust-native unit and integration pyramid. The work is intentionally staged: freeze the boundary contract first, then execute independent CLI, desktop, and MCP slices in parallel, then compose lifecycle and failure/cancellation flows, and finally enforce coverage evidence.

## Evidence baseline

| Evidence | Current state | Consequence |
|---|---|---|
| Coverage audit | 112 active modules: 13 unit, 94 integration, 1 smoke, 4 E2E, 0 support | The E2E denominator is explicitly classified and includes CLI, desktop, MCP, and initialization journeys. |
| Current E2E modules | `crates/tachi-cli/tests/e2e_artifacts.rs`, `crates/tachi-desktop/tests/e2e_command_journey.rs`, `crates/tachi-mcp/tests/e2e_stdio_journey.rs`, and `crates/tachi-shell/tests/init_substitution.rs` | Initialization, CLI artifacts, desktop commands, MCP stdio, init/install/update/analysis lifecycle behavior, and the cross-boundary failure matrix are covered; coverage-governance and branch evidence remain open. |
| Workspace tests | 468 tests pass across 111 test suites | Suite count and coverage-audit module count are different metrics and must remain separate. |
| LLVM coverage | 89.77% lines, 89.25% regions | Current gate passes its 85% line threshold; branch coverage is not currently reported. |
| Branch coverage capability | Pinned stable 1.96.1 rejects `-Z coverage-options=branch`; explicitly pinned nightly 1.99.0 produces a 77.49% baseline (1,408 branches, 317 missed) when `RUSTC` and `RUSTDOC` resolve through rustup | E2E-COV-007 must govern the nightly lane and uplift branch coverage to the requested 85% target; no lower threshold may be silently substituted. |
| Security/privacy | Local gitleaks 8.30.1 scan passes; fixtures are local/synthetic | New E2E fixtures must remain deterministic, redaction-safe, and offline by default. |

## Goals

1. Prove the critical CLI journey: validated architecture input → Rust analysis boundary → report-data and SARIF artifacts → stable output/exit semantics.
2. Prove the desktop-host journey through the active `crates/tachi-desktop` host and shared shell dispatch, including success, typed failure, artifact save, and cancellation behavior.
3. Prove the MCP stdio journey: explicit startup → request parsing → allowlisted tool dispatch → in-band result or artifact metadata → clean shutdown/cancellation.
4. Prove the adopter lifecycle: init → install/update control-plane path → analysis/artifact command, using isolated temporary clones and no network dependency.
5. Prove user-facing failure and cancellation boundaries without leaked child processes, partial artifacts, secrets, or ambiguous exit codes.
6. Add repeatable branch, line, region, unit, integration, smoke, and E2E evidence to the publish gate without weakening existing security or supply-chain checks.

## Non-goals and safety boundaries

- Do not introduce a browser framework or a GUI automation dependency for the GTK-free native host; test the host boundary through its real Rust entrypoints and deterministic fixtures.
- Do not require live GitHub, package registries, MCP servers, or external renderers for deterministic pull-request E2E tests.
- Do not duplicate business logic inside tests. E2E tests invoke the same CLI, shell, desktop, and MCP boundaries used by production callers.
- Do not treat golden-file equality as the only oracle; assert semantic output contracts, artifact bytes where byte stability is intentional, exit/status behavior, and cleanup invariants.
- Do not claim branch-target completion until the nightly branch baseline is reproducible and reaches the requested 85% threshold; the current 77.49% baseline is evidence, not completion.
- Do not include credentials, private paths, user data, network tokens, or unsanitized generated reports in fixtures or artifacts.

## Target journey matrix

| Journey | Primary boundary | Success oracle | Failure/cancellation oracle | Planned issue |
|---|---|---|---|---|
| Init and personalization | `scripts/init.sh` via shell test harness | Personalized files and modes match baseline | Unmanifested/tracked files remain unchanged; no residual placeholders | Existing `init_substitution`; extend under `E2E-COV-005` |
| CLI analysis and artifacts | `crates/tachi-cli/src/bin/*` and shell facade | Report-data, threats-SARIF, and risk-SARIF outputs are valid and semantically consistent | Invalid args fail closed; no partial output; source URI and status remain stable | `E2E-COV-002` |
| Desktop host command flow | `crates/tachi-desktop` → `tachi-shell` | Command status/stdout/stderr and saved bytes match shared dispatch | Typed errors, path escape rejection, timeout/cancel, and child cleanup | `E2E-COV-003` |
| MCP stdio | `crates/tachi-mcp` stdio transport | Explicit startup and allowlisted request produce validated result | Cancelled/malformed/disallowed requests fail without artifact leakage | `E2E-COV-004` |
| Full adopter lifecycle | init → install/update → analysis | A clean temporary clone reaches a usable analysis artifact | Missing input, failed subprocess, and interrupted lifecycle leave no unsafe residue | `E2E-COV-005` (implemented in init-substitution E2E) |
| Cross-boundary failure/cancel | shell, desktop, CLI, MCP seams | Status/error behavior is explicit across callers | Timeout/cancel is observable and no child process or partial artifact survives | `E2E-COV-006` (implemented across CLI/Desktop/MCP E2E suites) |

## Phased execution plan

### Phase 0 — Contract freeze and baseline (`E2E-COV-001`)

Document the journey matrix, fixture policy, boundary ownership, artifact oracles, and current branch/line/region/module baselines. Add semantic coverage-audit assertions for the E2E inventory and make the distinction between test suites and audit modules explicit.

**Exit criteria**: roadmap and issue cards are linked from the backlog/BOM; a red test exists for each new E2E classification or contract; no existing E2E behavior changes.

### Phase 1 — Independent boundary slices (parallel wave)

These slices can be implemented independently after Phase 0:

- `E2E-COV-002` CLI analysis/artifact journey.
- `E2E-COV-003` desktop-host command journey.
- `E2E-COV-004` MCP stdio request/response journey.

Each slice follows red test → minimal production wiring only if required → green focused suite → semantic review → conventional checkpoint. Tests must use repository fixtures and temporary roots with unique process/counter suffixes.

### Phase 2 — Composition and resilience

- `E2E-COV-005` composes init/install/update with one real analysis/artifact path in an isolated clone.
- `E2E-COV-006` adds cross-boundary failure, timeout, cancellation, partial-write, and child-process cleanup scenarios, reusing the typed error/status contracts established in Phase 1. The current matrix is covered across CLI, desktop, and MCP E2E suites; E2E-COV-007 remains for coverage evidence and publish enforcement.

Phase 2 must not hide failures behind retries. Each scenario has one deterministic setup, one invocation, and explicit cleanup assertions. `E2E-COV-005` now proves the local init → install → update → SARIF path in an isolated sparse clone; `E2E-COV-006` remains the follow-up matrix for broader cross-boundary failure and cancellation consistency.

### Phase 3 — Coverage governance and publish gate (`E2E-COV-007`)

Add branch coverage collection and reporting to the local evidence path, update the Rust coverage audit and docs, and enforce thresholds only after the measured baseline is reviewed. The stable-toolchain attempt remains unsuitable for measurement because `cargo llvm-cov --workspace --branch --summary-only` reaches the nightly-only `-Z coverage-options=branch` requirement and exits non-zero. An explicit nightly 1.99.0 run now produces a 77.49% branch baseline after seven uplift slices, but the requested 85% target is not met. E2E-COV-007 must govern that nightly coverage-only lane and uplift the uncovered branches before enforcement. The first enforcement target is:

- line coverage ≥ 85%;
- region coverage ≥ 85%;
- branch coverage ≥ 85%, with the current 77.49% baseline recorded and no unexplained regression;
- every active critical journey has at least one E2E test;
- unit/integration/smoke/E2E classifications remain synchronized with the audit binary;
- full workspace, security, privacy, supply-chain, and publish gates remain green.

## TDD operating loop

For every issue:

1. Write a failing test at the narrowest user-facing boundary that proves the missing behavior.
2. Run only that test and capture the failure string and missing contract.
3. Implement the smallest production change that makes the behavior green without duplicating domain logic.
4. Run the focused suite, then sibling boundary suites, then the workspace suite.
5. Run formatting, diff hygiene, Clippy, gitleaks, coverage, and the relevant publish gate.
6. Request a semantic code review focused on behavior, security/privacy, cleanup, and regression risk.
7. Update the roadmap/issue note, BOM/checklist, and `codemap.md`; create one conventional checkpoint commit.

## Parallel-wave review protocol

Independent work may be reviewed in parallel, but merge order is deterministic:

1. `E2E-COV-001` contract/baseline.
2. `E2E-COV-002`, `E2E-COV-003`, and `E2E-COV-004` focused slices, reviewed independently.
3. `E2E-COV-005` lifecycle composition.
4. `E2E-COV-006` resilience.
5. `E2E-COV-007` coverage/publish enforcement.

No later issue closes while a dependency has only a plan claim instead of executable test evidence.

## Rollback and stop conditions

- If an E2E test becomes network-dependent, replace the external dependency with a local fixture or explicit manual-only gate.
- If a failure leaves children or partial artifacts, stop the wave and fix cleanup before adding scenarios.
- If coverage falls below threshold, keep the failing evidence and open a focused follow-up; do not lower the threshold silently.
- If a route or host contract changes, update its semantic contract tests before updating E2E expectations.

## Definition of done

The roadmap is complete only when all `E2E-COV*` issues are closed, the audit reports the intended E2E inventory, branch/line/region evidence is recorded, full workspace/publish/security gates pass, the BOM and readiness checklist identify the new journeys, and the resulting commits are merged through the feature branch into `main` with remote CI evidence.
