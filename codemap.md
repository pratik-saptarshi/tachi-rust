# Repository Atlas: tachi-rust

## Project Responsibility

`tachi-rust` is the Rust-native implementation track for Tachi threat-modeling workflows. The current canonical path is the Rust workspace: `tachi-core` owns parsing and report data, `tachi-cli` exposes command-line entrypoints, `tachi-mcp` owns the standalone MCP transport and registered analysis tools, `tachi-shell` provides shared command handlers, and `crates/tachi-desktop` owns the active GTK-free desktop host. The former `src-tauri` adapter is retired from the active dependency surface.

The repository is still migrating away from the original Python ecosystem. Remaining Python scripts, pytest suites, and FastAPI stack scaffolds are tracked as transitional surfaces in `docs/roadmap/2026-06-08-python-surface-inventory.md`.

## System Entry Points

| Entry Point | Responsibility |
|---|---|
| `Cargo.toml` | Workspace manifest for `crates/tachi-core`, `crates/tachi-cli`, `crates/tachi-mcp`, `crates/tachi-shell`, and `crates/tachi-desktop`, with workspace Rust `1.96` MSRV metadata. |
| `rust-toolchain.toml` | Repository Rust toolchain policy pinned to `1.96.1` with `clippy`, `rustfmt`, and `llvm-tools-preview`; required Rust workflows install it and print compiler path/version proof. |
| `deny.toml` | Cargo dependency policy for advisories, bans, licenses, source registries, and exception metadata expectations. |
| `crates/tachi-core/src/lib.rs` | Core Rust library export surface for parsers, report data, coverage-attestation payloads, SARIF builders, taxonomy, coverage audit, infographic payloads, and attack-chain Mermaid generation, including the executive-architecture overlay path. |
| `crates/tachi-cli/src/bin/*.rs` | Rust CLI binaries for init/install/update/bootstrap, report-data, infographic-data, SARIF generation, and coverage audit. |
| `crates/tachi-shell/src/commands.rs` | Shared command layer used by CLI-style flows, the GTK-free desktop host, and transitional adapter paths. |
| `crates/tachi-desktop/src/main.rs` | Active native desktop host entrypoint, including headless smoke mode and macOS AppKit launch path. |
| `crates/tachi-desktop/src/lib.rs` | Active desktop host facade, command registry parity, typed invoke validation, offline-cache helpers, release-artifact manifest helpers, and shared shell dispatch integration. |
| `Makefile` | Validation shortcuts, including the Rust coverage gate via `make llvm-cov`, dependency policy via `make supply-chain-gate`, advisory feature/coverage canaries via `make feature-combination-canary` and `make coverage-tool-proof`, and scaffold dependency-floor gate via `make scaffold-dependency-gate`. |
| `.github/workflows/release-please.yml` | Main-push release automation using release-please with direct tag/release creation and no release-PR churn. |
| `docs/platform-compatibility.md` | Public compatibility matrix and setup landing page for canonical core plus harness-specific shims/fallbacks. |
| `docs/roadmap/` | Canonical migration roadmap, issue cards, merge plan, and Python-surface inventory. |

## Directory Map

| Directory | Responsibility Summary | Detailed Map |
|---|---|---|
| `crates/` | Rust workspace packages and their dependency boundaries. | [View map](crates/codemap.md) |
| `crates/tachi-core/` | Domain and data-transformation core. It parses generated threat-model artifacts, computes MAESTRO and coverage views, builds report data, emits SARIF payloads, owns the Rust coverage-audit catalog, and exposes a stable facade over internal utilities. | [View map](crates/tachi-core/codemap.md) |
| `crates/tachi-core/src/infographic/` | Infographic payload orchestration, template parsing, MAESTRO rendering, and executive-architecture overlays. | [View map](crates/tachi-core/src/infographic/codemap.md) |
| `crates/tachi-core/src/parsers/` | Structured parsing for findings, source attribution, agentic patterns, project metadata, and summaries. | [View map](crates/tachi-core/src/parsers/codemap.md) |
| `crates/tachi-cli/` | Thin CLI binary layer. Binaries parse flags, call shared core/shell functions, and write files or stdout. | [View map](crates/tachi-cli/codemap.md) |
| `crates/tachi-cli/src/bin/` | Executable entrypoints for control-plane, reporting, SARIF, infographic, and coverage-audit commands. | [View map](crates/tachi-cli/src/bin/codemap.md) |
| `crates/tachi-mcp/` | Standalone MCP transport with contract snapshots, registered analysis tools, request-id propagation, cancellation, and policy checks. | [View map](crates/tachi-mcp/codemap.md) |
| `crates/tachi-shell/` | Shared command facade that aligns desktop and CLI semantics and enforces bounded, root-contained execution. | [View map](crates/tachi-shell/codemap.md) |
| `crates/tachi-shell/src/commands/` | Runtime execution adapters, process lifecycle controls, and output/progress handling behind the shell registry. | [View map](crates/tachi-shell/src/commands/codemap.md) |
| `crates/tachi-desktop/` | Active GTK-free native desktop host routing through `tachi-shell`, with typed invocation, cache, artifact, and app-state boundaries. | [View map](crates/tachi-desktop/codemap.md) |
| `schemas/` | Finding schema and taxonomy catalogs used by parser, source-attribution, coverage, AISVS, and crosswalk validation tests. |
| `.claude/` | Agent, command, skill, and reference content inherited from the original Tachi workflow. This is data/configuration for threat-modeling behavior, not Rust runtime code. |
| `.aod/` | AOD shell helpers, templates, and governance memory. Some shell helpers remain under Rust test coverage while migration continues. |
| `tests/scripts/` | Transitional pytest suite and fixtures. RT-011 progressively ports high-signal coverage into Rust tests and removes retired pytest modules. |
| `tests/fixtures/` | Frozen fixture copies and baseline trees used for compatibility checks. These are excluded from active coverage-audit counts. |
| `scripts/` | Transitional Python runtime scripts from the original implementation. RT-012 tracks porting remaining canonical behavior into Rust; the standalone SARIF generators, infographic extractor, pagination smoke scaffolds, and attack-chain extraction pytest have already moved to Rust CLI binaries and tests, RT-013 now routes desktop `infographic-data` through the shared Rust payload builder, and the active architecture system-design README now points at the Rust CLI extractors. |
| `stacks/` | Legacy Python/FastAPI and frontend scaffolds. RT-014 tracks retirement or archival after Rust/Tauri parity is stable; the stack index now describes the retired FastAPI packs generically. |
| `docs/` | Public project documentation. Roadmap and product planning documents live under `docs/roadmap/`; testing status lives under `docs/testing/`; archived security-review guidance for retired FastAPI scaffolds lives in `docs/security/`; the root `README.md` now treats the old FastAPI packs and legacy `make test` note as archived guidance, the Rust init matrix workflow has replaced the pytest matrix, `docs/platform-compatibility.md` centralizes the harness matrix and fallback path, live examples use Rust/Tauri or shell tooling instead of Python pretty-printers, and the pre-commit / CLAUDE organization / permissions guidance now avoids Python-specific installation, test-run, and runtime-example wording. |

## Roadmap Control Surfaces

| Track | Current Direction |
|---|---|
| Rust toolchain modernization | `docs/roadmap/2026-07-05-rust-toolchain-upgrade-roadmap.html.md` is the completed historical roadmap for the closed `RT-TC` Beads hierarchy. `RT-TC-001` landed the repository toolchain pin and workflow proof; `RT-TC-002` added fail-closed audit, deny, gitleaks, and clippy SARIF policy gates; `RT-TC-003` converted workflow/reporting tests to semantic YAML, workspace-derived, parsed rendering, and keyed JSON projections; `RT-TC-004` added pinned `cargo-hack` / `cargo-llvm-cov` manual-scheduled canaries; `RT-TC-005` first resolved `src-tauri` as standalone evidence, then `RT-00i.2.5` retired that adapter from the active dependency surface to unblock the live GTK/GLib advisory; `RT-TC-006` is implemented by `docs/architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md`, which defers `smol-rs` runtime crates to a separate async-runtime feature with benchmarks and cancellation/shutdown tests. |
| RT-CI live CI hardening | `docs/tachi-rust-ci-execution-plan.md`, `docs/tachi-rust-ci-beads-issue-cards.md`, `docs/tachi-rust-ci-review-panel.md`, `docs/tachi-rust-ci-route-policy.md`, `docs/tachi-rust-ci-route-fixtures.md`, and `docs/tachi-rust-ci-closeout.md` define the live `RT-CI` hierarchy for PR concurrency, workflow parse, rustfmt, protected trigger contracts, route-policy escalation, route fixtures, dependency-closure routing, shared Rust setup, protected-ref enforcement, and privileged SARIF invariants. The branch carries the dedicated `ci-workflow-parse.yml` and `rustfmt.yml` gates plus the workflow contract audit in `crates/tachi-core/tests/workflow_ci_gates.rs`; the current telemetry slice adds protected/unknown-route full-mode assertions, multi-workflow queue-vs-run median evidence, and explicit branch-protection evidence requirements. |
| E2E coverage expansion | `docs/roadmap/2026-07-10-e2e-coverage-expansion-roadmap.html.md`, `docs/roadmap/2026-07-10-e2e-coverage-expansion-issue-cards.md`, and the `E2E-COV*` Beads hierarchy define the active plan for CLI, desktop-host, MCP-stdio, lifecycle, failure/cancellation, and branch-coverage evidence. The Rust-owned E2E inventory covers CLI artifacts, desktop host commands, MCP stdio, initialization, the composed init/install/update/analysis lifecycle, and the cross-boundary failure matrix; the 85% nightly branch target is met and publish-gate closeout remains open. |
| Codemap automation state | `.slim/codemap.json` tracks the core Rust/configuration surface (67 files), with hierarchical maps under `crates/` and each active workspace package. Tests and documentation remain excluded from hash-based change detection. |

## Rust Data And Control Flow

1. CLI binaries in `crates/tachi-cli/src/bin/` parse command arguments and delegate to Rust libraries.
2. Shared business logic runs in `tachi-core` modules:
   - `parsers.rs` parses project names, threat findings, markdown tables, source attribution, and agentic patterns.
   - `attack_chains.rs` parses attack-chain artifacts and renders MAESTRO-aligned Mermaid diagrams for Rust-native parity tests.
   - `coverage_attestation.rs` builds the per-finding and per-framework coverage-attestation data for the report pipeline, including raw-versus-in-scope taxonomy counts and filtering helpers.
   - `report_data.rs` builds Typst payload data for report assembly and reuses the already-read threats content for project-name parsing on the hot path.
   - `infographic.rs` builds JSON payloads, MAESTRO visual data, and the executive-architecture overlay path.
   - `risk_scores.rs`, `threats_sarif.rs`, and `sarif_common.rs` build SARIF exports.
   - `coverage_taxonomy.rs` centralizes coverage and MAESTRO taxonomy labels.
   - `coverage_audit.rs` classifies active test modules by unit, integration, smoke, E2E, and support/regression families.
3. `tachi-shell` exposes reusable command functions for shell and desktop paths through the stable `tachi_core::facade` surface, which now carries the test-facing artifacts/assets/attack-chain/mmdc and compensating-control helpers, plus the stable reporting exports now rehomed behind root facade re-exports.
4. `crates/tachi-desktop` exposes the active desktop host and calls the shared shell dispatch path directly while preserving command output shape, artifact behavior, progress/cancellation handling, and app-state visibility.
5. The retired `src-tauri` Cargo manifest and lockfile stay absent so the active workspace desktop path remains GTK-free and Dependabot does not track a stale adapter lockfile.

## Testing And Validation

| Level | Current Rust-Native Surface |
|---|---|
| Unit | Rust unit tests; current audit shows 2 Rust unit modules and 0 remaining Python unit modules. |
| Integration | Rust integration tests under `crates/*/tests`; current audit includes the desktop host parity tests, scaffold dependency-floor audit, workflow CI gate audit, issue-template TDD contract audit, retired-adapter guard tests, the typed control-plane boundary audit, the RT-CI trigger/permission contract audit, the route-policy manifest contract audit, and the route-fixture manifest contract audit, while the init-substitution E2E boundary is Rust-owned. |
| Smoke | Transitional smoke modules tracked by `tachi-core::coverage_audit`; current audit shows 1 Rust smoke canary and 0 remaining Python smoke modules. |
| E2E | Critical init, CLI analysis-to-artifact, desktop host command, MCP stdio, composed init/install/update/analysis lifecycle, and cross-boundary failure/cancellation flows are explicitly classified or exercised by `crates/tachi-core/tests/coverage_audit.rs` and the Rust E2E suites; E2E-COV-007 retains coverage-governance and branch evidence as the remaining work. |
| Coverage | `make llvm-cov` is the stable release-quality gate: 90.22% regions / 90.56% lines, with the configured 85% line threshold passing. The governed nightly 1.99.0 lane now records 85.09% branch coverage (1,408 branches / 210 missed) after deterministic CLI, desktop, MCP stdio, and shell-bridge failure-edge coverage; E2E-COV-007.1 meets its branch target while publish-gate synchronization remains. The current audit has 113 active modules, 95 Rust integration modules, 13 Rust unit modules, 1 Rust smoke module, 4 Rust E2E modules, and 0 support/regression modules. |

The publish gate now includes `make scaffold-dependency-gate`, which runs the
Rust-native `scaffold_dependency_floors` integration test against the real
Next.js/Supabase scaffold manifest so known vulnerable `next` and `vitest`
lower bounds cannot be reintroduced.
The publish/release path now also uses `release-please` on push to `main`,
with the workflow configured to skip release-PR churn and create the tag /
GitHub Release directly.

Primary validation commands:

```bash
cargo fmt --check
git diff --check
cargo test -q
cargo test --workspace --all-targets
cargo test -p tachi-desktop --all-targets
cargo clippy --all-targets -- -D warnings
make llvm-cov
cargo run -q -p tachi-cli --bin coverage-audit
```

## Migration Map

| Roadmap Card | Current Direction |
|---|---|
| RT-011 | Complete: migrate remaining pytest coverage into Rust tests using TDD. Keep explicit unit, integration, smoke, and E2E classification visible through `coverage-audit`. |
| RT-012 | Complete: port remaining Python runtime behavior into Rust modules and CLI binaries, especially report extraction, infographic output handling, executive-architecture infographic parity, and remaining report/SARIF payload parity. |
| RT-013 | Complete: keep Tauri shell thin by routing desktop behavior through shared Rust command handlers; `infographic-data` now flows through the shared Rust payload builder. |
| RT-014 | Complete: retire Python packaging, pytest-only guidance, and FastAPI stack scaffolds after parity is complete; the security-review FastAPI scaffold note is archived guidance only, the active architecture and devops README/CI/env-var summaries now point at Rust CLI extractor commands and the Rust init matrix, and the pre-commit / CLAUDE organization / permissions guidance no longer recommends Python-package installation, pytest wording, or Python-runtime environment examples, with the devops local pre-commit path now using a package-manager install example. |
| RT-015 | Optimize Rust path for speed and reliability after the Python runtime path is no longer canonical, with `AOD_INIT_TRACE=1` timing markers in `scripts/init.sh`, a cleanup-phase trace before self-delete, millisecond slowest-phase trace summaries, same-clone cold/warm precommit sample support, and single-read report-data assembly. |

## Dependency Notes

The RT-CI telemetry slice adds `scripts/rt-ci-latency-evidence.sh` and the
`make rt-ci-latency-evidence` target for queue-vs-run medians across the
workspace and route-observe workflows; the helper also records branch-
protection availability and fails when required remote evidence is missing.

Codemap dependency analysis now treats `scripts/tachi_parsers` as retired. The dead `tests/scripts` init helper package has been retired and the init precommit matrix now lives in `crates/tachi-shell/tests/init_precommit_matrix.rs`; `scripts/init.sh` now exposes opt-in `AOD_INIT_TRACE=1` timing markers plus a cleanup-phase trace and millisecond slowest-phase summaries for the slow-init refinement, scopes placeholder substitution to manifest-backed personalized files plus the constitution clean template, and the workspace inventory now reports no active Python files. The manifest-path cache helper now lives in `crates/tachi-shell/tests/init_manifest_paths.rs` and `scripts/init.sh` reuses the cached personalized path list across substitution and residual scanning. `crates/tachi-core/src/report_data.rs` now reuses the already-read threats content when deriving project metadata so Typst assembly avoids a duplicate filesystem read on the hot path. The security review now treats the remaining FastAPI `SECRET_KEY` note as archived scaffold guidance, not active runtime advice. The stack index and architecture tech-stack table now describe the retired FastAPI packs generically while keeping their historical paths for reference. The init trace harness can now collect cold and warm samples, with and without precommit, in the same clone by restoring `scripts/init.sh` between runs, and the active pre-commit, CLAUDE organization, and permissions guidance now keeps installation, testing, and runtime-example wording package-manager-only or Rust-native. The Rust init matrix workflow now replaces the pytest matrix, and the active devops README/env-var summaries, smoke guide, orchestration skill doc, root README, and getting-started / eval-convention live docs now use Rust/Tauri or shell tooling instead of Python pretty-printers or legacy pytest guidance. The main workspace test workflow now splits into package-sized jobs to stay inside the GitHub runner window. Rust work should continue moving any remaining parser-like behavior into `tachi-core` or `tachi-shell` with Rust tests. The FastAPI Alembic scaffold `env.py` files, backend test-package scaffolding, backend app runtime trees, backend scaffold packaging manifests, backend Alembic scaffold directories/manifests, root pytest support package, and top-level Python packaging manifests (`pyproject.toml`, `requirements-dev.txt`) are also retired and should stay out of the active Python surface inventory.

## Agent Guidance

- Before changing code, read this file and the relevant Rust module/test files.
- Local branch audit on 2026-07-05 found `feat/tauri-minimal-features` and
  `feat/rt009-publish-gate-bom-readiness` stale against `main`; do not merge
  them directly without a fresh reconciliation pass and explicit deletion or
  archival approval.
- Prefer small TDD slices with one retired Python surface per commit.
- Use `.worktrees/` for isolated branches; it is ignored by git.
- Treat roadmap documents under `docs/roadmap/` as the canonical migration status.
- Keep `README.md` at repository root and move non-root Markdown documentation under `docs/` unless it is standard project metadata.

## Recent AQ Slices

- RT-jwj: `crates/tachi-shell/src/commands.rs` now keeps control-plane
  script discovery inside the repo root, and the containment regression in
  `crates/tachi-shell/tests/tauri_bridge.rs` still proves bootstrap routing
  works when the root-local script exists.
- RT-bu7: `crates/tachi-core/src/sarif_common.rs`,
  `crates/tachi-core/src/{threats_sarif,risk_scores}.rs`, and
  the SARIF goldens/tests now share the baseline run ID helper so new
  findings stay empty while existing findings emit the same frozen baseline
  marker across both pipelines.
- RT-qz9: `crates/tachi-core/src/{threats_sarif,risk_scores}.rs` and
  `crates/tachi-shell/src/command_use_cases.rs` now thread the threats source
  URI from CLI input into SARIF builders, with goldens and shell tests proving
  the emitted artifact path is no longer hardcoded.
- RT-oui: `crates/tachi-core/src/report_extraction.rs` now truncates
  executive narrative text on a UTF-8 character boundary, with
  `crates/tachi-core/tests/extractor_contract_fixes.rs` covering the
  multibyte boundary regression.
- RT-0zv: `crates/tachi-core/src/sarif_common.rs`,
  `crates/tachi-core/src/risk_scores.rs`,
  `crates/tachi-core/src/threats_sarif.rs`, and
  `crates/tachi-core/tests/{risk_scores,threats_sarif}.rs` now share the
  canonical `logicalLocation.kind` mapping so threat and risk SARIF stay in
  parity on `data-store`.
- AQ-021: closed historical evidence. Its reusable typed validation, offline
  cache, release-artifact, and registry tests now live under
  `crates/tachi-desktop`; the buildable `src-tauri` adapter surface is retired
  by RT-00i.2.5.
- AQ-020: closed. The Phase 1 desktop boundary now covers least-privilege
  config/capabilities, typed argument policy, root-contained IO, bounded
  process execution, and typed desktop errors through the closed AQ-021 through
  AQ-025 task set.
- Coverage gate: the E2E-COV-007.1 slice adds CLI artifact/error, desktop
  headless/schema/offline, MCP stdio startup, and shell-bridge cancellation /
  artifact-failure cases. The governed nightly 1.99.0 lane now reports 85.09%
  branch coverage (1,408 total / 210 missed), while stable `make llvm-cov`
  remains at 90.56% lines / 90.22% regions. `make gitleaks-gate` is now a
  fail-closed member of `make publish-gate`.
- RT-00i.5.1: `schemas/aisvs.yaml`, `schemas/taxonomy/aisvs.yaml`,
  `crates/tachi-shell/tests/tauri_bridge.rs`, and the public docs now ship
  the AISVS schema/catalog slice with bridge coverage for report-data,
  infographic-data, threats-sarif, and risk-scores-sarif dispatch paths.
- PR reconciliation: GitHub PR #10 (`issue-5`) and PR #11 (`issue-6`) are
  closed as superseded by `main`. Shell script containment is now guarded by
  `crates/tachi-shell/tests/control_plane.rs`; SARIF logical-location kind
  behavior is centralized in `crates/tachi-core/src/sarif_common.rs` and
  covered by `risk_scores` / `threats_sarif` tests.
- AQ-001: tracker now has phase-0 Beads children `AQ-011`, `AQ-012`, and
  `AQ-013` materialized from the architecture/test-quality roadmap, and
  their workflow/template proofs are validated and closed.
- AQ-054.4: parser roundtrip seed coverage now exists in
  `crates/tachi-core/tests/parsers.rs` as a regression landing zone for the
  advisory fuzz/mutation lane.
- AQ-040: `crates/tachi-core/tests/facade_api.rs` now exercises the stable
  core facade surface, proving downstream crates can compile against the root
  exports instead of module internals.
- AQ-042: `crates/tachi-core/src/infographic/maestro_templates.rs` and
  `crates/tachi-core/tests/infographic_payload.rs` now cover the split MAESTRO
  template assembly and executive-architecture payload seams.
- AQ-043: `crates/tachi-core/src/coverage_attestation.rs`,
  `crates/tachi-core/src/infographic.rs`, and
  `crates/tachi-core/tests/coverage_attestation_in_scope.rs` now route
  taxonomy/template resolution through injected stores with fake-provider
  coverage.
- AQ-033: bootstrap control-plane argument shaping was extracted into
  `bootstrap_control_plane_args`, and `crates/tachi-shell/src/tauri_bridge.rs`
  now reuses the helper instead of duplicating the bootstrap prefix logic.
- PR reconciliation: `crates/tachi-shell/tests/control_plane.rs` now locks the
  current workspace-root script resolution against stale PR #10 regressions:
  nested package manifests still use workspace scripts, and ancestor scripts
  outside the repo root are not executed.
- AQ-043: `crates/tachi-core/src/infographic.rs` now routes template loading
  through an injected `PromptScaffoldStore`, letting tests swap in a fake
  store while the filesystem adapter preserves current behavior.
- AQ-043: `crates/tachi-core/src/coverage_attestation.rs` now routes taxonomy
  loading through an injected `TaxonomyStore`, with fake-store tests covering
  the aggregate path while the filesystem adapter keeps the current output.
- AQ-042: `crates/tachi-core/src/infographic/executive_architecture.rs`
  now owns executive-architecture payload assembly and its direct unit test,
  trimming the parent infographic module without changing golden output.
- AQ-051: `crates/tachi-core/src/parsers/findings.rs`,
  `crates/tachi-core/src/risk_scores.rs`, and
  `crates/tachi-core/src/threats_sarif.rs` now carry source-level tests for
  malformed recommendations, empty/missing sections, sentinel normalization,
  and classifier prefix precedence.
- AQ-052: `crates/tachi-core/tests/reporting_goldens.rs` now uses semantic
  projections for SARIF and infographic contracts, keeping compact
  fixture-local snapshots instead of full-envelope equality everywhere.
- AQ-053: `crates/tachi-core/tests/property_quality.rs` now uses `proptest`
  for normalization, coverage math, source-attribution order, and malformed
  parser inputs, giving the property-quality lane a no-network baseline.
- AQ-054: `docs/testing/fuzz-mutation-audit.md`,
  `docs/reports/fuzz-mutation-baseline.md`,
  `.github/workflows/fuzz-mutation-audit.yml`, and `make fuzz-mutation-gate`
  now define an offline advisory fuzz/mutation lane with a repo test guard.
- AQ-055: `docs/platform-compatibility.md`, the adapter README family, and
  `.github/workflows/release-please.yml` now document the harness-agnostic
  compatibility matrix and direct push-driven release workflow.
- AQ-050: `crates/tachi-core/tests/property_quality.rs`,
  `crates/tachi-core/tests/publishing_security_docs.rs`,
  `docs/testing/fuzz-mutation-audit.md`, and
  `docs/reports/fuzz-mutation-baseline.md` now document the non-blocking
  property/fuzz/mutation lanes and their promotion criteria.
- AQ-001: `docs/roadmap/2026-06-22-adversarial-architecture-test-quality-roadmap.html.md`,
  `docs/roadmap/2026-06-22-adversarial-architecture-test-quality-issue-cards.md`,
  and the associated AQ task set are now complete and retained as the
  historical architecture/test-quality record.
- RT-00i: `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-roadmap.html.md`,
  `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-issue-cards.md`,
  `docs/bill-of-materials.html.md`,
  `docs/publish-readiness-checklist.html.md`,
  `crates/tachi-core/src/aisvs.rs`, `crates/tachi-core/tests/scaffold_dependency_floors.rs`,
  and `Cargo.lock` now anchor the glib alert remediation plan and phased
  AISVS 1.0 control rollout, with the typed control registry foundation,
  `AisvsRegistry::validation_commands`, C01-C12 typed control policies,
  explicit per-control validation commands, and the reproducible glib
  advisory proof landed locally. The Beads tracker now includes leaf tasks for
  the typed registry, sanitized error model, Send+Sync proof,
  publish-readiness sync,
  and the gtk/glib compatibility decision/follow-up lane.
- MCP-001: `docs/roadmap/2026-06-25-standalone-mcp-server-roadmap.html.md`,
  `docs/roadmap/2026-06-25-standalone-mcp-server-issue-cards.md`,
  `docs/bill-of-materials.html.md`,
  `docs/publish-readiness-checklist.html.md`, and `docs/platform-compatibility.md`
  now anchor the standalone MCP planning track, with the semantic core split
  into ported analysis surfaces, explicit control-plane exclusions, and a
  release/documentation gate that keeps the MCP contract aligned with the
  canonical command registry. `crates/tachi-mcp` now carries the Stage 1
  transport/tool implementation plus request-id propagation, cancellation
  handling, the policy allowlist seam, and the cleanup hook. The `MCP-001*`
  Beads hierarchy is closed; future MCP work should open a new tracker
  hierarchy.
- AQ-042: `crates/tachi-core/src/infographic/maestro_templates.rs`
  now owns MAESTRO template assembly and a direct unit test, separating the
  layer-summary rendering from the parent infographic module.
- AQ-033: `crates/tachi-shell/src/commands/runtime_helpers.rs` now owns
  stream capture and final output assembly, trimming pure output-handling code
  out of `commands.rs` with direct helper coverage.
- AQ-033: `crates/tachi-shell/src/commands/script_executor.rs` now owns
  system script spawning, timeout, and cancellation behind an injected
  `ScriptExecutor` seam, while `ScriptOutputSink` owns output finalization
  with fake-request and fake-sink coverage.
- AQ-033: `crates/tachi-shell/src/command_use_cases.rs` now owns the shell
  domain conversions for coverage audit, report data, infographic payloads,
  threats SARIF, and risk scores SARIF, leaving `commands.rs` as a slimmer
  registry/runtime adapter.
- AQ-033: `crates/tachi-shell/src/commands.rs` only orchestrates the registry
  and delegates to executor, runtime helper, and progress seams; bridge and
  CLI parity tests now justify closing the boundary split slice.
- AQ-034: `src-tauri/tests/bridge.rs`,
  `crates/tachi-shell/tests/tauri_bridge.rs`, and
  `crates/tachi-cli/tests/control_plane_cli.rs` now cover CLI/Tauri command
  parity and command-shape checks for the registered shell surface.
- AQ-043: `crates/tachi-core/src/infographic.rs` and
  `crates/tachi-core/src/coverage_attestation.rs` now route template and
  taxonomy loading through injected stores, with filesystem-equivalence tests
  preserving the existing outputs.
- AQ-042: `crates/tachi-core/src/infographic/prompt_scaffold.rs` now owns the
  prompt scaffold loading and parsing seam, with filesystem and injected-store
  tests preserving the existing infographic payload surface.
- AQ-042: `crates/tachi-core/src/infographic/payload.rs` now owns infographic
  payload orchestration and filesystem loading, leaving the parent module on
  pure data, parsing, and scoring helpers.
- AQ-034: `src-tauri/tests/schema.rs` now exercises all registered command
  success paths and output validation markers, while CLI control-plane parity
  tests remain green in `crates/tachi-cli/tests/control_plane_cli.rs`.
