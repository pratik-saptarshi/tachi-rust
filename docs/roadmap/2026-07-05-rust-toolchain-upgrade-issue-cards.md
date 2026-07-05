# Rust Toolchain Upgrade Issue Cards

**Last Updated**: 2026-07-05
**Status**: Beads-backed execution blueprint; live `RT-TC` hierarchy created
after the approved remote schema migration.
**Source**: [Rust Toolchain Upgrade Roadmap](./2026-07-05-rust-toolchain-upgrade-roadmap.html.md)

These cards are the source text for the live RT-TC Beads hierarchy. The remote
schema migration was run with explicit approval before tracker writes.

## Card Format

- `Epic`
- `Feature`
- `Capability`
- `Task`
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

### RT-TC / Rust toolchain modernization

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: reproducible Rust compiler, Cargo, CI, and supply-chain gates
- `Capability`: pin required PR behavior while retaining latest-stable drift visibility
- `Task`: execute RT-TC-001 through RT-TC-006 in priority order
- `Function`: `rust-toolchain.toml`, `.github/workflows/*`, `Cargo.toml`,
  `deny.toml`, `gitleaks.yml`, `crates/tachi-core/tests/workflow_ci_gates.rs`
- `Dependencies`: active desktop security blockers remain release-critical;
  RT-TC P0 work may run first when it directly improves release confidence.
- `Acceptance criteria`:
  - Required PR gates use a checked-in pinned compiler policy.
  - Supply-chain gates fail closed and preserve SARIF upload evidence.
  - CI contract tests are semantic and workspace-derived where possible.
  - Transitional `src-tauri` status is explicit before adapter gates are required.
  - Deferred async-runtime decisions stay outside the compiler-upgrade blast radius.
- `Validation`: `cargo test -p tachi-core --test workflow_ci_gates`,
  `git diff --check`, `bd ready --json`
- `Implementation owner`: release/toolchain owner
- `Stage label`: roadmap execution package
- `Next test seam`: workflow CI gate tests and supply-chain workflow fixtures
- `Priority`: 0
- `Notes`: Use these cards as the source for future Beads creation after schema
  migration; run `bd export -o .beads/issues.jsonl` after live tracker writes.

## P0 Security And Reproducibility

### RT-TC-001 - Pin Rust stable toolchain and normalize path proof

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: reproducible Rust compiler and Cargo baseline
- `Capability`: required PR gates use the same compiler locally and in CI
- `Task`: add `rust-toolchain.toml`, normalize Homebrew/rustup path proof, and
  update required Rust workflows to consume the checked-in policy
- `Function`: `rust-toolchain.toml`, `.github/workflows/rust-workspace.yml`,
  `.github/workflows/rust-clippy.yml`, developer prerequisites
- `Dependencies`: none
- `Acceptance criteria`:
  - `rust-toolchain.toml` pins the current approved stable release and includes
    `clippy`, `rustfmt`, and `llvm-tools-preview`.
  - Required Rust workflows no longer depend only on floating `stable`.
  - CI prints `rustc -Vv`, `cargo -Vv`, `which rustc`, `which cargo`, and
    `rustup which rustc`.
  - Local path drift between Homebrew and rustup is either resolved or recorded
    as harmless with command evidence.
  - Workspace MSRV policy is explicit before release docs claim a public floor.
- `Validation`: `rustup which rustc`, `which rustc`, `cargo -Vv`,
  `cargo fmt --all -- --check`, `cargo test --workspace --all-targets`
- `Implementation owner`: release/toolchain owner
- `Stage label`: P0 reproducibility
- `Next test seam`: workflow CI gate test for pinned toolchain setup and proof
- `Priority`: 0
- `Notes`: Re-check the latest stable Rust release before implementation; the
  roadmap's `1.96.1` target is date-stamped 2026-07-05.

### RT-TC-002 - Add fail-closed supply-chain gates

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: dependency-security controls
- `Capability`: audit, policy, secret scanning, and SARIF conversion failures
  fail closed while preserving upload evidence
- `Task`: add `cargo audit`, `cargo deny`, SARIF validation, and gitleaks/clippy
  status capture to release and PR lanes
- `Function`: `.github/workflows/*`, `deny.toml`, `gitleaks.yml`,
  `crates/tachi-core/tests/workflow_ci_gates.rs`
- `Dependencies`: `RT-TC-001`
- `Acceptance criteria`:
  - `cargo audit` and `cargo deny check advisories bans licenses sources` are
    runnable locally and represented in CI policy.
  - Gitleaks uploads SARIF with `if: always()` but fails the job after upload
    when findings or scanner errors occur.
  - Clippy SARIF helper tools are pinned or SHA-pinned, and SARIF JSON is
    structurally validated before upload.
  - Pipeline statuses for scanner, converter, formatter, and upload-adjacent
    commands are captured instead of hidden by pipes.
  - Registry/source/license exceptions require owner, expiry, and remediation
    issue metadata.
- `Validation`: `cargo audit`, `cargo deny check advisories bans licenses sources`,
  `cargo test -p tachi-core --test workflow_ci_gates`
- `Implementation owner`: security/release owner
- `Stage label`: P0 supply chain
- `Next test seam`: workflow gate tests for fail-closed SARIF paths
- `Priority`: 0
- `Notes`: Do not allow SARIF upload success to mask a failed scanner or converter.

## P1 CI Confidence And Adapter Boundaries

### RT-TC-003 - Make CI tests semantic and order-insensitive

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: stable CI contract tests
- `Capability`: workflow and report assertions validate behavior, not incidental
  line order
- `Task`: replace raw text/order assertions with YAML, TOML, JSON, SARIF, and
  sorted/keyed semantic comparisons where order is not product behavior
- `Function`: `crates/tachi-core/tests/workflow_ci_gates.rs`,
  `crates/tachi-core/tests/reporting_goldens.rs`, root `Cargo.toml`
- `Dependencies`: none
- `Acceptance criteria`:
  - Workflow tests parse YAML and assert semantic sets for jobs, permissions,
    package matrices, and required commands.
  - Package matrix assertions derive active packages from the root workspace.
  - JSON/SARIF checks parse structured data before comparing fields.
  - Remaining exact full-output goldens are documented as rendering contracts.
  - `src-tauri` is excluded from required workspace matrix checks unless its
    member/exclude/retire decision makes it active.
- `Validation`: `cargo test -p tachi-core --test workflow_ci_gates`,
  `cargo test -p tachi-core --test reporting_goldens`
- `Implementation owner`: test/CI owner
- `Stage label`: P1 semantic tests
- `Next test seam`: workflow matrix helper and SARIF projection helpers
- `Priority`: 1
- `Notes`: This card directly reduces recurring brittle-review failures.
  Implemented by parsing workflow YAML into job, event, matrix, step, and run
  projections; deriving the required package matrix from root workspace
  members; parsing coverage and Typst rendered output for semantic fields; and
  comparing infographic JSON arrays as sorted/keyed projections where order is
  not a product contract.

### RT-TC-004 - Add pinned cargo-hack and cargo-llvm-cov proof lanes

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: feature and coverage canaries
- `Capability`: detect feature-combination and coverage-tool drift without
  expanding the required PR blast radius prematurely
- `Task`: add pinned `cargo-hack` and explicit `cargo-llvm-cov` provisioning,
  version proof, and canary commands
- `Function`: `.github/workflows/*`, `Makefile`, documentation prerequisites
- `Dependencies`: `RT-TC-001`
- `Acceptance criteria`:
  - `cargo-hack` install/provisioning is pinned and version-printed.
  - First canary runs `cargo hack check --workspace --each-feature --no-dev-deps`.
  - Any bounded powerset lane documents package/feature exclusions with owner,
    expiry, and reason.
  - `cargo-llvm-cov` provisioning records version evidence and verifies
    `llvm-tools-preview` availability.
  - Canary promotion to required PR gate is a later decision based on signal/noise.
- `Validation`: `cargo hack --version`,
  `cargo hack check --workspace --each-feature --no-dev-deps`,
  `cargo llvm-cov --version`, `make llvm-cov`
- `Implementation owner`: CI/release owner
- `Stage label`: P1 canary tooling
- `Next test seam`: workflow gate test for pinned install/version evidence
- `Priority`: 1
- `Notes`: Prefer canary first; do not let tool installation churn block the
  primary compiler pin slice. Implemented as a manual/scheduled advisory lane
  with pinned `cargo-hack 0.6.45`, pinned `cargo-llvm-cov 0.8.7`, local
  Makefile proof targets with exact version checks, a post-`cargo-hack`
  manifest diff guard before coverage, and no PR/main-push trigger until
  signal/noise review.

### RT-TC-005 - Resolve src-tauri adapter status before enforcement

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: active host versus transitional adapter clarity
- `Capability`: required gates only cover surfaces with explicit support status
- `Task`: decide whether `src-tauri` is a workspace member, explicitly excluded
  standalone adapter, or retired compatibility surface before adding adapter
  validation as a required gate
- `Function`: root `Cargo.toml`, `src-tauri/Cargo.toml`,
  `.github/workflows/*`, `crates/tachi-core/tests/workflow_ci_gates.rs`
- `Dependencies`: active desktop host roadmap and GTK-free security decisions
- `Acceptance criteria`:
  - `src-tauri` status is documented as member, exclude/standalone, or retired.
  - Required root workspace gates do not accidentally include an undecided adapter.
  - If retained, adapter validation has a standalone `cargo metadata --locked`
    and `cargo check --manifest-path src-tauri/Cargo.toml` lane.
  - Active workspace remains GTK-free and dependency-floor proof fails loudly
    on GTK/glib regressions.
  - Adapter validation is not a release blocker until the status decision lands.
- `Validation`: `cargo metadata --manifest-path src-tauri/Cargo.toml --locked --format-version 1`,
  `cargo check --manifest-path src-tauri/Cargo.toml`,
  `cargo tree -p tauri --manifest-path src-tauri/Cargo.toml`
- `Implementation owner`: desktop/adapter owner
- `Stage label`: P1 adapter boundary
- `Next test seam`: workflow matrix exclusion/member assertion
- `Priority`: 1
- `Notes`: Do not silently convert transitional compatibility into a required
  release surface. Implemented as an explicitly excluded standalone adapter
  with its own lockfile, local `make tauri-adapter-check`, and a
  manual/scheduled compatibility workflow.

## P2 Deferred ADR Boundary

### RT-TC-006 - Document deferred smol-rs async-runtime ADR boundary

- `Epic`: `RT-TC` / Rust toolchain modernization
- `Feature`: async-runtime decision hygiene
- `Capability`: keep runtime-crate adoption out of the compiler upgrade blast radius
- `Task`: document that `smol-rs` runtime crates are deferred until a separate
  MCP or desktop async-runtime ADR has benchmarks and cancellation/shutdown tests
- `Function`: roadmap ADR section, future MCP/desktop async design docs
- `Dependencies`: none
- `Acceptance criteria`:
  - Toolchain upgrade scope explicitly excludes runtime replacement or async
    primitive adoption.
  - Future `smol-rs` evaluation requires a concrete feature, benchmarks, and
    cancellation/shutdown regression tests.
  - No `smol-rs` dependency is added as part of the toolchain pin or security gates.
  - `taiki-e` CI tooling decisions remain separate from runtime crate decisions.
- `Validation`: docs review plus dependency diff showing no new runtime crates
- `Implementation owner`: architecture owner
- `Stage label`: P2 ADR boundary
- `Next test seam`: none until a future async-runtime feature is opened
- `Priority`: 2
- `Notes`: This is a guardrail card, not an implementation dependency.

## Beads Creation Notes

The live Beads hierarchy was created from the card text:

- `RT-TC` priority `0`
- `RT-TC-001` priority `0`
- `RT-TC-002` priority `0`
- `RT-TC-003` priority `1`
- `RT-TC-004` priority `1`
- `RT-TC-005` priority `1`
- `RT-TC-006` priority `2`

After future live tracker writes, run `bd export -o .beads/issues.jsonl` and
include the export in the same commit.
