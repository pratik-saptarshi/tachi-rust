# Bill of Materials

**Status**: Active publish inventory
**Last Updated**: 2026-07-12
**Purpose**: enumerate the repository surfaces that are expected to ship, be
reviewed, or be validated before publishing `tachi-rust` to remote origin
**Scope**: source code, docs, tests, CI, security posture, and release gates

## Publish Model

`tachi-rust` publishes as a Rust workspace with `crates/tachi-desktop` as the
active GTK-free desktop host and documentation-first release controls. The
public surface is intentionally broad
because the repository ships:

- Rust workspace code and shared command logic
- GTK-free desktop host code
- Retired adapter provenance in historical roadmap and tracker records
- CLI entrypoints
- docs, roadmap, and release policy artifacts
- security and privacy posture documentation
- CI and secret-scanning workflows
- test fixtures and regression data

This BOM is the authoritative inventory for publication review. If a file or
directory is not listed here, it is either internal implementation detail,
temporary scratch state, or a local-only worktree artifact.

For public readers, this document explains which surfaces are expected to ship,
which docs define the user-facing workflow, and which validation gates guard
the release path. Treat it as the contract between repository contents and the
published artifact set.

## Public-Facing Documents

These are the documents that must stay current, redaction-safe, and aligned
with the shipped release workflow before publication.

| Path | Role | Publish status | Notes |
|---|---|---|---|
| `README.md` | Repository landing page and getting-started guide | Publishable | Must describe the current install, usage, auditor workflow, and release path without stale workflow guidance. |
| `adapters/README.md` | Compatibility entrypoint for native adapters and generic fallback | Publishable | Must match the harness matrix, install surfaces, and identical core contract. |
| `docs/platform-compatibility.md` | Harness matrix and fallback behavior guide | Publishable | Must stay aligned with the adapter packs and the generic fallback path. |
| `docs/guides/DEVELOPER_GUIDE_TACHI.md` | Public developer and auditor walkthrough | Publishable | Must stay aligned with the README and show the actual first-run analysis path. |
| `SECURITY.md` | Security policy | Publishable | Private vulnerability reporting only; keep public disclosure guidance current. |
| `CHANGELOG.md` | Release history | Publishable | Redaction-safe release notes only. |
| `docs/bill-of-materials.html.md` | Publish inventory | Publishable | Canonical inventory of publication surfaces and validation gates. |
| `docs/publish-readiness-checklist.html.md` | Publish readiness checklist | Publishable | Required pre-push gate for security, privacy, docs, CI, release hygiene, and public-doc alignment. Must describe `crates/tachi-desktop` as the active desktop host and include branch-protection/remote-evidence requirements before merge closure. |
| `docs/tachi-rust-ci-route-policy.md` | RT-CI route policy manifest | Publishable with review | Human-readable escalation rules for the live RT-CI routing track before observe-only proof and fixture enforcement. |
| `docs/tachi-rust-ci-route-fixtures.md` | RT-CI route fixture manifest | Publishable with review | Common change-set matrix and stable JSON examples for route decisions. |
| `docs/tachi-rust-ci-route-artifact.md` | RT-CI route artifact manifest | Publishable with review | Observable `route.json` schema and stable orchestrator check notes for the route-observe lane. |
| `docs/tachi-rust-ci-baseline.md` | RT-CI baseline snapshot | Publishable with review | Phase-0 inventory and local validation snapshot for the pre-routing CI contract. |
| `docs/tachi-rust-ci-closeout.md` | RT-CI closeout notes | Publishable with review | Separates locally proven RT-CI changes from GitHub-side verification items that remain pending. |
| `docs/ci-improvement-plan.html` | RT-CI source plan draft | Publishable with review | Original plan-review integrator output that feeds the live RT-CI execution docs and tracker cards. |
| `docs/tachi-rust-ci-execution-plan.md` | RT-CI execution plan | Publishable with review | Phase sequencing and validation notes for the live RT-CI CI hardening track. |
| `docs/tachi-rust-ci-beads-issue-cards.md` | RT-CI issue cards | Publishable with review | Source text for the live RT-CI hierarchy and acceptance criteria. |
| `docs/tachi-rust-ci-review-panel.md` | RT-CI review panel | Publishable with review | Validation notes from the plan-review and overseer pass. |
| `docs/roadmap/2026-07-10-e2e-coverage-expansion-roadmap.html.md` | E2E coverage expansion roadmap | Publishable with review | Defines the CLI, desktop, MCP, lifecycle, resilience, and branch-coverage workstream. |
| `docs/roadmap/2026-07-10-e2e-coverage-expansion-issue-cards.md` | E2E coverage issue cards | Publishable with review | Source text for the `E2E-COV*` Beads hierarchy and dependency order. |
| `docs/standards/PUBLISHING_SECURITY.md` | Security and privacy gate | Publishable | Source of truth for public-push safety rules, disclosure boundaries, and release-note hygiene. |
| `docs/standards/PRECOMMIT_HOOKS.md` | Secret-scanning hook guide | Publishable with review | Must not imply weaker scanning than the current gate. |
| `scripts/rt-ci-latency-evidence.sh` | RT-CI latency evidence helper | Publishable with review | Collects queue vs run medians from GitHub Actions lanes; required for route evidence closeout. |

## Top-Level Inventory

| Path | Role | Publish status | Notes |
|---|---|---|---|
| `Cargo.toml` | Rust workspace manifest | Publishable | Canonical workspace root for `tachi-core`, `tachi-cli`, `tachi-mcp`, `tachi-shell`, and `crates/tachi-desktop`; declares workspace Rust `1.96` MSRV. |
| `rust-toolchain.toml` | Pinned Rust toolchain policy | Publishable | Required Rust workflows install Rust `1.96.1` with `clippy`, `rustfmt`, and `llvm-tools-preview` from the checked-in policy. |
| `deny.toml` | Cargo dependency policy | Publishable | Cargo-deny policy for advisories, bans, license allowlist, source allowlist, and exception metadata discipline. |
| `README.md` | Public repository landing page | Publishable | Must stay aligned with the actual build, auditor workflow, and usage path. |
| `LICENSE` | License text | Publishable | Required public artifact. |
| `SECURITY.md` | Vulnerability disclosure policy | Publishable | Public security policy and private disclosure channel. |
| `CHANGELOG.md` | Release history | Publishable | Keep release notes redaction-safe. |
| `docs/` | Public documentation | Publishable | Long-form docs, roadmap, standards, and review artifacts. |
| `crates/` | Workspace library and binary crates | Publishable | Source of the Rust implementation. |
| `crates/tachi-desktop/` | GTK-free desktop host boundary | Publishable | Active desktop host facade over the shared shell command surface without GTK/Wry transitive dependencies. |
| `.github/` | CI and release workflows | Publishable | Public automation surface. |
| `.claude/` | Agent configuration and runtime rules | Publishable with review | Must avoid secrets and private credentials. |
| `.aod/` | AOD support files | Publishable with review | Contains governance and hook logic; verify no private data. |
| `schemas/` | Validation schemas and taxonomies | Publishable | Needed for parser/report contracts. |
| `tests/` | Fixtures and regression tests | Publishable with review | Synthetic or redacted only; no private source material. |
| `stacks/` | Scaffold templates and archived stack packs | Publishable with review | Template manifests must not admit known vulnerable dependency floors. |
| `INSTALL_MANIFEST.md` | Install command contract | Publishable with review | Machine-parseable file list that must match distributable paths. |
| `scripts/` | Transitional shell and helper scripts | Publishable with review | Keep no secret-bearing defaults. |
| `brand/` | Visual assets | Publishable | Verify image captions and alt text do not expose private data. |
| `examples/` | Example output and sample artifacts | Publishable with review | Must remain synthetic or sanitized. |

## Source BOM

### Workspace and runtime source

| Path | Contents | Reviewer focus |
|---|---|---|
| `crates/tachi-core/` | Parsers, scoring, reporting, taxonomy, SARIF, coverage helpers | Parser hardening, output shape stability, no panic-based user-facing parsing. |
| `crates/tachi-cli/` | CLI entrypoints and argument-forwarding binaries | Flag correctness, help text, command parity, no duplicated business logic. |
| `crates/tachi-mcp/` | Standalone MCP transport and contract snapshot layer | Canonical command contract reuse, stdio startup path, registered analysis tools, request-context hardening, and artifact-emitting tool dispatch. |
| `crates/tachi-shell/` | Shared command facade and bridge adapter | Shared dispatch, shared errors, identical CLI/desktop semantics. |
| `crates/tachi-shell/src/commands/script_executor.rs` | Script execution boundary | Process spawning, timeout, cancellation, and output capture stay behind an injected executor seam. |
| `crates/tachi-core/src/infographic/prompt_scaffold.rs` | Prompt scaffold boundary | Template loading and prompt extraction stay isolated from payload rendering with store-injected tests. |
| `crates/tachi-core/src/infographic/payload.rs` | Infographic payload boundary | Filesystem loading and payload orchestration stay separated from infographic parsing helpers. |
| `crates/tachi-core/src/facade.rs` | Stable core facade | Downstream crates should import reporting and scoring helpers through root exports instead of module internals. |
| `crates/tachi-desktop/` | Desktop host boundary | Registration-only host facade over the shared shell command surface, with no GTK/Wry dependency line. |
| `schemas/` | Finding schemas and taxonomy catalogs | Schema compatibility, crosswalk stability, fixture coverage. |

### Transitional helper surface

| Path | Contents | Reviewer focus |
|---|---|---|
| `scripts/` | Init/bootstrap helpers and transitional tooling | No secret leakage, no unreviewed shell injection, clear retirement path. |
| `scripts/rt-ci-latency-evidence.sh` | RT-CI timing evidence helper | No secret leakage, no shell injection, and documented requirement for network-bound API access. |
| `.aod/` | Governance and operational helpers | Hook safety, no private state, no accidental publish of local settings. |
| `.claude/` | Agent and permissions configuration | Public-safe policy, no credentials, no private repo-specific tokens. |
| `stacks/nextjs-supabase/scaffold/` | Next.js/Supabase scaffold template | Dependency floors must exclude known vulnerable `next` and `vitest` ranges. |

### Test and fixture surface

| Path | Contents | Reviewer focus |
|---|---|---|
| `tests/fixtures/` | Synthetic regression fixtures | Synthetic only, deterministic, redaction-safe. |
| `tests/scripts/` | Legacy and transitional test fixtures | Keep compatibility data sanitized and representative. |
| `crates/*/tests/` | Rust integration and unit tests | Coverage of parser, command, reporting, and bridge contracts. |
| `crates/*/benches/` | Benchmark gates | Must be stable enough for regression detection. |

## Documentation BOM

| Path | Purpose | Publish note |
|---|---|---|
| `docs/roadmap/implementation-backlog.md` | Backlog navigation hub | Canonical link target for active implementation sequencing and live RT-CI / security / toolchain reconciliation. |
| `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-roadmap.html.md` | Active AISVS/security roadmap | Canonical sequencing for the live Dependabot alert, AISVS C01-C12 rollout, and TDD-backed validation gates. |
| `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-issue-cards.md` | Active AISVS/security issue cards | Beads-ready execution templates for the RT-00i epic and its phase slices. |
| `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-issue-cards.md#phase-5-publish-readiness-and-release-gates` | Completed Phase 5 publish-readiness slice | Historical evidence for closed `RT-00i.6`, which synchronized the AISVS docs and release-gate follow-up. |
| `docs/roadmap/2026-06-25-standalone-mcp-server-roadmap.html.md` | Archived MCP roadmap | Historical sequencing for the closed `MCP-001*` hierarchy, including scope boundaries, portability limits, and stage-gated acceptance criteria. |
| `docs/roadmap/2026-06-25-standalone-mcp-server-issue-cards.md` | Archived MCP issue cards | Completed execution templates for the closed MCP epic, features, capabilities, and task slices. |
| `docs/roadmap/2026-06-22-adversarial-architecture-test-quality-roadmap.html.md` | Archived AQ roadmap | Canonical architecture, SOLID, and test-quality remediation plan, now retained as a historical record. |
| `docs/roadmap/2026-06-21-rust-tauri-parity-remediation-roadmap.html.md` | Archived parity roadmap | Historical Rust/Tauri parity rebaseline and supersession plan. |
| `docs/roadmap/2026-06-21-rust-tauri-parity-issue-cards.md` | Archived parity execution cards | Historical Beads issue templates for the parity phases. |
| `docs/roadmap/2026-06-21-archived-docs-workflow-version-sweep-roadmap.html.md` | Archived docs hygiene roadmap | Completed docs-only sweep for stale workflow-version references. |
| `docs/roadmap/2026-06-21-archived-docs-workflow-version-sweep-issue-cards.md` | Archived docs sweep cards | Completed Beads issue templates for docs/version hygiene. |
| `docs/roadmap/2026-06-15-rust-tauri-parity-remediation-roadmap.html.md` | Archived parity roadmap | Historical snapshot of the earlier parity plan. |
| `docs/roadmap/2026-06-15-rust-tauri-parity-issue-cards.md` | Archived parity cards | Historical Beads-ready backlog for the earlier parity track. |
| `docs/roadmap/2026-06-04-rust-tauri-issue-pack.md` | Historical tracker-neutral pack | Archived provenance for the earlier migration plan. |
| `docs/roadmap/2026-06-08-rust-tauri-only-roadmap.md` | Archived implementation roadmap | Historical planning snapshot, not active scope. |
| `docs/roadmap/2026-06-08-rust-tauri-only-issue-cards.md` | Archived execution cards | Historical Beads-ready backlog from the superseded plan. |
| `docs/roadmap/2026-06-08-python-surface-inventory.md` | Frozen migration evidence | Historical reference, not the active surface. |
| `docs/publish-readiness-checklist.html.md` | Publish gate checklist | Required pre-push security, privacy, docs, CI, and release gate. |
| `docs/standards/PUBLISHING_SECURITY.md` | Security and privacy publish gate | Must remain the security policy source for public pushes and release hygiene. |
| `docs/standards/PRECOMMIT_HOOKS.md` | Secret-scanning hook guide | Security gate for staged content and local commits. |
| `docs/changelog.html` | Release chronology | Must remain redaction-safe. |
| `docs/devops/SECURITY_POSTURE_2026Q2.md` | Public security posture summary | Review for accidental disclosure before publication. |

## CI and Release BOM

| Path | Purpose | Publish note |
|---|---|---|
| `.github/workflows/gitleaks.yml` | Full-repo secret scanning | Required publication gate. |
| `.github/workflows/rust-workspace.yml` | Full Rust workspace PR test gate with passive-docs, dependency-closure, and emergency override route controls | Required behavior gate for package matrix tests under the checked-in Rust toolchain; skips the heavy matrix for passive-docs-only PRs, narrows crate-local changes to their dependency closure, and supports an emergency full-CI override while preserving the stable route classifier. |
| `.github/actions/rust-setup/action.yml` | Shared Rust setup action | Local composite action that centralizes toolchain install, cache, and proof steps for Rust-facing workflows. |
| `.github/workflows/ci-workflow-parse.yml` | Workflow syntax gate | Required PR-side `actionlint` lane for early GitHub Actions YAML failures. |
| `.github/workflows/ci-route-observe.yml` | Route artifact gate | Observe-only PR lane that uploads `route.json` and preserves the stable orchestrator check while routing is non-enforcing. |
| `.github/workflows/rustfmt.yml` | Rust formatting gate | Required PR-side `cargo fmt --all -- --check` lane for isolated formatting drift. |
| `.github/workflows/rust-clippy.yml` | Rust lint gate | Prevents warnings from shipping under the checked-in Rust toolchain. |
| `.github/workflows/rust-supply-chain.yml` | Cargo audit and dependency policy gate | Runs pinned `cargo-audit` and `cargo-deny` checks for advisories, bans, licenses, and sources. |
| `.github/workflows/rust-feature-coverage-canary.yml` | Feature-combination and coverage-tool canary | Manual/scheduled lane that pins `cargo-hack 0.6.45` and `cargo-llvm-cov 0.8.7`; not a required PR or main-push gate until signal/noise review promotes it. |
| `.github/workflows/release-please.yml` | Release orchestration | Main-push release automation without release-PR branch churn; release gate now covers manifest and checksum parity. |
| `.github/workflows/fuzz-mutation-audit.yml` | Advisory fuzz/mutation lane | Scheduled/manual non-blocking lane for parser and reporting survivor discovery. |
| `.github/workflows/tachi-mmdc-preflight.yml` | Mermaid preflight | Protects docs and renderable diagram outputs. |
| `.github/workflows/tachi-pytest.yml` | Transitional compatibility tests | Must be reviewed for retirement or narrowing as migration completes. |
| `scripts/rt-ci-latency-evidence.sh` | CI timing evidence helper | Network-dependent helper used by RT-CI merge-closeout to record queue/run medians. |

## Security and Privacy BOM

The following surfaces require explicit review before publication:

- Any file that may contain tokens, credentials, private keys, or API secrets.
- Any example, fixture, or log that may contain customer data, personal data,
  or private assessment output.
- Any workflow that can emit secrets into logs or artifacts.
- Any documentation that describes private internal process, unreleased release
  state, or unredacted operational details.

The repository policy for these surfaces is:

1. Keep secrets out of committed files.
1. Keep public examples synthetic or redacted.
1. Route security issues through private disclosure, not public issue trackers.
1. Re-scan before publish and again after release merges.
1. Keep `README.md`, `CHANGELOG.md`, and all release-facing docs free of private paths, tokens, usernames, and unredacted operational details.

## Validation BOM

| Gate | Evidence | Acceptance |
|---|---|---|
| Rust unit and integration tests | `make test` (`scripts/ci-local-runner.sh --mode local-full`) plus `make test-route` | Must pass with per-unit JSON results, provenance, bounded logs, cleanup proof, and CI-manifest parity. |
| Rust toolchain proof | `rustup toolchain install --no-self-update`, `rustc -Vv`, `cargo -Vv`, `which rustc`, `which cargo`, `rustup which rustc` | Required Rust workflows consume `rust-toolchain.toml` and prove the compiler path before running tests or lint. |
| Full workspace PR behavior gate | `cargo test --workspace --all-targets` and `.github/workflows/rust-workspace.yml` | Pull requests apply routing: full mode is preserved on protected refs and active/shared surfaces, while passive docs and dependency-closure-aware changes are narrowed by design. |
| Semantic CI contract tests | `cargo test -p tachi-core --test workflow_ci_gates -- --nocapture` | Workflow contracts parse YAML for events, jobs, matrices, steps, and run commands; package matrices derive from root workspace members instead of copied strings. |
| Rust E2E and bridge checks | `cargo test -p tachi-cli --test e2e_artifacts`, `cargo test -p tachi-desktop --test e2e_command_journey`, `cargo test -p tachi-mcp --test e2e_stdio_journey`, `cargo test -p tachi-shell --test init_substitution`, the `E2E-COV*` focused suites, and `cargo test -p tachi-core --test rt009_docs` | Current explicit E2E inventory covers initialization, CLI artifacts, desktop commands, MCP stdio, composed lifecycle, and cross-boundary failure/cancellation; E2E-COV-007 remains for coverage evidence and publish enforcement. |
| E2E coverage inventory | `cargo run -q -p tachi-cli --bin coverage-audit` | The audit must classify every critical journey exactly once and keep E2E distinct from integration/smoke modules. |
| MCP scaffold and contract checks | `cargo test -p tachi-mcp --test contract_snapshot --test schema_snapshot --test tools_registration --test session_policy --test stdio` and `cargo build -p tachi-mcp --features stdio` | MCP registry, stdio transport, request-id continuity, cancellation handling, schema snapshots, and contract snapshots remain deterministic. |
| Core infographic and scaffold seams | `cargo test -p tachi-core` | Prompt scaffold, infographic payload, parser, and reporting seams remain green after boundary splits. |
| Infographic payload seam | `cargo test -p tachi-core` | Payload orchestration remains behavior-compatible after moving filesystem loading and template assembly. |
| Parser hardening regression | `cargo test -p tachi-core compute_delta_counts_trims_case_and_ignores_unknown_statuses -- --nocapture` | Must pass for panic-free delta counting and status normalization. |
| Lint gate | `cargo clippy --all-targets -- -D warnings` and `.github/workflows/rust-clippy.yml` | No warnings allowed; SARIF upload remains `if: always()` but clippy status fails closed. |
| Supply-chain gate | `cargo audit`, `cargo deny check advisories bans licenses sources`, `make supply-chain-gate`, and `.github/workflows/rust-supply-chain.yml` | RustSec advisories, dependency bans, license policy, and registry/source policy pass locally and in CI with pinned helper tools. |
| Retired adapter guard | `cargo test -p tachi-core --test scaffold_dependency_floors --test workflow_ci_gates` | `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and `.github/workflows/tauri-adapter-compatibility.yml` stay absent; the active workspace release host is `crates/tachi-desktop`. |
| Feature and coverage canary | `make feature-combination-canary`, `make coverage-tool-proof`, and `.github/workflows/rust-feature-coverage-canary.yml` | `cargo-hack 0.6.45` checks workspace feature combinations with no dev-dependencies; `cargo-llvm-cov 0.8.7` records coverage-tool proof through the active toolchain LLVM wrapper; lane stays advisory until promoted. |
| Async runtime ADR boundary | `docs/architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md` and `cargo metadata --locked --format-version 1` | `smol-rs` runtime crates stay outside the toolchain track; any future adoption requires a separate async-runtime feature with benchmarks, cancellation/shutdown tests, compatibility evidence, dependency diff, and rollback plan. |
| Coverage gate | `make llvm-cov`; governed nightly branch command with explicit `RUSTC`/`RUSTDOC`/`LLVM_COV`/`LLVM_PROFDATA` paths | Standalone stable coverage passes at 90.22% regions and 90.56% lines. Nightly 1.99.0 records 85.09% branch coverage (1,408 total / 210 missed), meeting the requested 85% target after E2E-COV-007.1 slice 24. The latest aggregate `make publish-gate` invocation was interrupted after its local runner and therefore does not claim that post-runner coverage stages completed in that invocation. |
| Reporting goldens | `cargo test -p tachi-core --test reporting_goldens -- --nocapture` | Canonical report, threat, risk, coverage, Typst, and infographic outputs remain stable through parsed semantic projections and compact rendering-contract snapshots. |
| Advisory fuzz/mutation lane | `make fuzz-mutation-gate` and `.github/workflows/fuzz-mutation-audit.yml` | Commands stay documented, scheduled/manual runs remain non-blocking, and survivor reports stay offline-safe. |
| Diff hygiene | `git diff --check` | No whitespace or patch-format issues. |
| Secret scan | `make gitleaks-gate`, `pre-commit run --all-files`, or `.github/workflows/gitleaks.yml` | Local publish-gate execution now runs a fail-closed gitleaks SARIF/schema check; no secrets or private data may leak into examples, fixtures, logs, or generated docs. |
| Scaffold dependency gate | `make scaffold-dependency-gate` | Next.js/Supabase scaffold dependency ranges exclude currently known vulnerable `next` and `vitest` floors. |
| Docs gate | `README.md`, `docs/platform-compatibility.md`, `docs/guides/DEVELOPER_GUIDE_TACHI.md`, `SECURITY.md`, `CHANGELOG.md`, and public docs cross-links | Public docs match the shipped behavior and the disclosure policy. |
| AISVS security gate | `cargo test -p tachi-core --test aisvs_registry`, `cargo test -p tachi-core --test aisvs_controls`, `cargo test -p tachi-core --test scaffold_dependency_floors`, `cargo clippy --workspace --all-features --all-targets -- -D warnings` | AISVS C01-C12 remain typed, test-backed, and fail-closed while the historical `glib` advisory proof stays reproducible in Beads, the current workspace stays on the GTK-free host path, and no active `gtk` or `glib` package remains in the Rust dependency graph. |
| AISVS publish-readiness evidence | `RT-00i.6` | Closed Phase 5 docs/release-gate evidence stays visible in the BOM and issue cards so future AISVS work opens a new tracker slice instead of reusing the closed follow-up. |
| Docs/version sweep | `make docs-version-gate` + `make docs-archive-version-gate` | Maintained docs stay current; archived docs and examples retain only intentional historical references. |
| Publish gate | `make publish-gate` | Local setup, workflow, CodeQL, docs, scaffold, supply-chain, gitleaks, coverage, and release gates passed in the latest run; the final aggregate publish-gate invocation was interrupted after its canonical local runner had independently produced terminal 8/8 evidence. Full publish-gate green remains pending until a complete uninterrupted run is recorded. |
| Local CI-parity runner | `.github/ci-test-units.json`, `schemas/ci-test-units.schema.json`, `schemas/ci-run-result.schema.json`, `scripts/ci-local-runner.sh`, `make test`, `make test-route` | E2E-COV-007.3 evidence complete. Terminal local-full run `20260712T173705Z-72397` passed 8/8 in 536,162 ms (compile-and-test 466,327 ms; test-slice 68,906 ms); labeled warm route-equivalent evidence passed 8/8 in 304,650 ms; controlled cold route-equivalent evidence passed 8/8 in 321,636 ms. JSON provenance records stage totals, cache context, toolchain, cleanup, and outcome; E2E-COV-008 hosted timing/reliability evidence is now complete with documented sample limitations. |
| Local/hosted performance and reliability evidence | Local runner `results.json`, hosted runs `29178308727`, `29178255153`, and `29203699709`, `scripts/verify-ci-timing-artifacts.sh`, workflow job summaries, `make rt-ci-latency-evidence` | Local full/route/warm/cold evidence has 40/40 successful unit executions across five runs, with no timeout/cancellation; the terminal local-full run took 536,162 ms (compile/test 466,327 ms; test-slice 68,906 ms), and the controlled cold run took 321,636 ms. All eight timing artifacts were validated for the merged main, prior PR, and merged PR #24 runs. Current pull-request medians are workspace 85s across 22 samples (79–101s) and route-observe 14s across 23 samples (11–17s), with 0s queue medians; the latest five PR #24 runs were successful. Historical failed PR runs remain visible in the raw GitHub sample and are not suppressed. |
| Timing provenance/privacy hardening | `E2E-COV.2` | Follow-up from security review: independently bind downloaded timing artifacts to GitHub run metadata and define contract-tested local log retention/redaction. E2E-COV-008 records the original parity evidence; this follow-up remains open before the broader privacy checklist is complete. |
| Advisory workflow emulation | `scripts/act-smoke.sh`, `tests/fixtures/act/pull-request.json`, `make act-smoke` | Planned E2E-COV-009 capability. Opt-in only; rootless Podman Docker-API compatibility is best-effort and preflighted; no secrets, host sockets, privileged/networked execution, or hosted-CI claims. |
| RT-CI timing evidence | `make rt-ci-latency-evidence`, `docs/tachi-rust-ci-baseline.md` | Mainline sample: workspace 40-run median 71s (2–361s), route-observe 11-run median 14s (12–17s), queue medians 0s. Representative pull-request sample: workspace 22-run median 85s (79–101s), route-observe 23-run median 14s (11–17s), queue medians 0s. Branch protection and required-check evidence are verified. |
| CI gate | GitHub Actions run status | Release, security, lint, and docs workflows are green. |
| Remote monitor | `git push origin main --follow-tags` + `gh run watch` | Post-push CI is observed to completion before the release is considered published. |
| Release-please gate | `release-please.yml` push filter | Docs-only publishes do not churn release refs and push runs avoid PR-branch churn. |
| Workflow hardening | `rg "actions/checkout@v[0-6]|actions-rs/toolchain@|github/codeql-action/upload-sarif@v3|::set-output" .github/workflows` | No legacy checkout, toolchain, SARIF, or set-output usage remains. |
| CodeQL maintenance | `docs/security/codeql-maintenance.md`, `scripts/codeql-maintenance-check.sh`, `scripts/codeql-upstream-release-check.sh`, `.github/workflows/codeql-maintenance.yml`, `make codeql-maintenance-gate`, `make codeql-upstream-release-check` | Active SARIF uploads use CodeQL Action v4; v4.37.0 / CodeQL 2.26.0 mapping, Node 24 compatibility, floating-tag risk acceptance, rollback, redaction, trusted-event, and historical-reference policy are contract-tested. A read-only manual/weekly workflow compares the documented tag with the latest non-prerelease v4 release and raises a visible maintenance failure without mutating issues or publish gates. |
| MCP readiness evidence | Archived MCP roadmap, archived MCP issue cards, BOM, publish checklist, and MCP CI lane | Closed MCP publish surfaces remain synchronized with the canonical command contract, release checklist, request-context hardening, portability matrix, and CI evidence. Future MCP work should open a new tracker hierarchy before promotion. |
| MCP CI lane | `.github/workflows/rust-workspace.yml` with `tachi-mcp` package matrix | Dedicated MCP package lane covers contract snapshot, schema snapshot, stdio transport, and tool registration regressions. |

## Exclusions

- Local worktree scratch files.
- Generated caches that are ignored by git.
- Temporary benchmark data created outside the checked-in benchmark fixtures.
- Anything in the tree explicitly labeled archived, retired, or compatibility
  only, unless it is being published as historical evidence.

## Publish Rule

If a surface is listed as publishable, it must be reviewed for security,
privacy, doc accuracy, and release readiness before `main` is pushed to
`origin`.

## Publish Evidence Checklist (required before push)

- [ ] `rg "actions/checkout@v[0-6]|actions-rs/toolchain@|github/codeql-action/upload-sarif@v3|::set-output" .github/workflows` returns no matches.
- [ ] `rust-toolchain.toml` pins the approved Rust toolchain and required components for CI.
- [ ] Required Rust workflows print `rustc -Vv`, `cargo -Vv`, `which rustc`, `which cargo`, and `rustup which rustc`.
- [ ] `make supply-chain-gate` passes, including `cargo audit` and
      `cargo deny check advisories bans licenses sources`.
- [ ] `make gitleaks-gate` passes locally and is included in `make publish-gate`.
- [ ] `.github/workflows/gitleaks.yml` and `.github/workflows/rust-clippy.yml`
      upload SARIF with `if: always()` and fail closed after scanner,
      converter, formatter, or SARIF validation failures.
- [ ] `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and
      `.github/workflows/tauri-adapter-compatibility.yml` remain absent after
      retiring the vulnerable Tauri/GTK adapter surface.
- [ ] `make feature-combination-canary` and `make coverage-tool-proof` pass
      serially before promoting feature/coverage canaries from advisory to
      required release gates.
- [ ] `docs/roadmap/implementation-backlog.md` points at the active AISVS/security roadmap, the live RT-CI track, closed docs-sweep/MCP/RT-TC records, and archived provenance docs.
- [ ] The active AISVS roadmap is `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-roadmap.html.md`.
- [ ] The active AISVS Beads cards are `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-issue-cards.md`.
- [ ] The MCP issue hierarchy is closed; `docs/roadmap/2026-06-25-standalone-mcp-server-roadmap.html.md` and `docs/roadmap/2026-06-25-standalone-mcp-server-issue-cards.md` are retained as historical source records.
- [ ] The docs-sweep roadmap and issue cards are retained as completed historical records: `docs/roadmap/2026-06-21-archived-docs-workflow-version-sweep-roadmap.html.md` and `docs/roadmap/2026-06-21-archived-docs-workflow-version-sweep-issue-cards.md`.
- [ ] The historical `glib` Dependabot alert proof is captured in `crates/tachi-core/tests/scaffold_dependency_floors.rs`, and the current workspace no longer resolves `gtk` or `glib`.
- [ ] Archived roadmap docs are clearly marked as historical only.
- [ ] `make docs-version-gate` passes.
- [ ] `git status --short --branch` has no unexpected untracked or dirty state.
- [ ] `make test` and `make test-route` are green with machine-readable per-unit results and provenance; `make llvm-cov` is green on the release candidate branch.
- [ ] `make scaffold-dependency-gate` is green for scaffold dependency floors.
- [ ] `cargo clippy --all-targets -- -D warnings` is clean.
- [ ] Public examples and fixtures are synthetic or redacted.
- [ ] `make rt-ci-latency-evidence` is run for merge-closeout when GitHub API/network is available.
