# Rust Toolchain Upgrade Roadmap

**Date**: 2026-07-05
**Scope**: Rust, Cargo, rustup, workspace CI, dependency-security gates, and
review brittleness for `tachi-rust`
**Status**: completed; retained as the historical execution roadmap for the
closed `RT-TC` Beads hierarchy

> Supersession note (2026-07-05): RT-TC-005 resolved `src-tauri` as a
> standalone adapter during the toolchain hardening track. Active security work
> in `RT-00i.2.5` now supersedes that decision by retiring the buildable
> adapter manifest, lockfile, and workflow to close the remaining GTK/GLib
> advisory surface.

## Executive Summary

The repo has been upgraded to the pinned Rust `1.96.1` toolchain deliberately,
without letting required CI float silently. The `RT-TC` epic and `RT-TC-001`
through `RT-TC-006` are closed in Beads. This document is retained as the
historical execution plan and acceptance record for the toolchain pin,
fail-closed supply-chain gates, semantic CI tests, advisory canaries,
standalone `src-tauri` adapter boundary, and ADR-046 async-runtime deferral.

Future compiler, supply-chain, or runtime-adoption work should open a new Beads
hierarchy and keep `.beads/issues.jsonl`, the backlog snapshot, BOM, and publish
checklist synchronized in the same change.

## Source Verification

| Fact | Source | Plan impact |
|---|---|---|
| Rust `1.96.1` was published on 2026-06-30. | `https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/` | Pin and validate against `1.96.1` before updating CI policy. Re-check this URL before implementation if a newer stable release exists. |
| Rust `1.96.1` fixes Cargo HTTP retry/timeout behavior, a MIR miscompilation, and three libssh2 CVEs in Cargo. | `https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/` | Treat upgrade as P1 supply-chain/security work. |
| Rust `1.96.0` included Cargo fixes for CVE-2026-5223 and CVE-2026-5222 for third-party registries. | `https://blog.rust-lang.org/2026/05/28/Rust-1.96.0/` | Add registry and lockfile review gates even if crates.io-only today. |
| rustup `1.29.0` was published on 2026-03-12. | `https://blog.rust-lang.org/2026/03/12/Rustup-1.29.0/` | Local rustup is current; document self-update check instead of forcing a repo file change. |
| `smol-rs` publishes async runtime primitives such as `smol`, `async-channel`, `blocking`, `async-io`, `polling`, and `async-task`. | `https://github.com/smol-rs` | Deferred by [ADR-046](../architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md); evaluate only if a future MCP or desktop async-runtime feature supplies benchmarks plus cancellation and shutdown tests. |
| `taiki-e` publishes Rust CI tools including `cargo-llvm-cov`, `cargo-hack`, `install-action`, `pin-project`, and `portable-atomic`. | `https://github.com/taiki-e` | Adopt `cargo-hack`/`cargo-llvm-cov` workflow patterns where they reduce feature or coverage drift. |

## Pre-Implementation Repo Baseline

This table records the baseline that shaped the original execution plan. It is
not the current repository state; the completed `RT-TC` issue hierarchy and the
backlog snapshot are authoritative for post-implementation status.

| Area | Current state | Risk |
|---|---|---|
| Workspace | `Cargo.toml` members are `tachi-core`, `tachi-cli`, `tachi-mcp`, `tachi-shell`, and `tachi-desktop`. | Active runtime path is Rust-native; toolchain changes hit every crate. |
| Transitional Tauri | `src-tauri/Cargo.toml` is explicitly excluded from the root workspace and retains its own lockfile. | Validate through `make tauri-adapter-check` and the manual/scheduled adapter workflow while `crates/tachi-desktop` remains the active host. |
| Toolchain pin | No checked-in `rust-toolchain.toml`. | CI and local runs can diverge based on date and machine state. |
| Package MSRV | Crates declare `edition = "2021"` and no `rust-version`. | Consumers cannot tell the supported compiler floor. |
| CI | `rust-workspace.yml`, `rust-clippy.yml`, and `tachi-pytest.yml` use `dtolnay/rust-toolchain@stable` or otherwise install stable Rust. | Freshness is good, reproducibility is weaker; inventory every Rust workflow before claiming CI is pinned. |
| Security gates | `gitleaks.yml`, Dependabot-related docs, `scaffold_dependency_floors`, and workflow-version gates exist. | Missing first-class `cargo audit` / `cargo deny` policy; gitleaks currently needs fail-closed handling. |
| Desktop dependency floor | Current `cargo tree -i glib` and `cargo tree -e features -i gtk` exit non-zero because no matching packages exist in the active workspace graph. | Preserve this as a wrapped absence proof; raw `cargo tree -i` is not a green gate when absence is expected. |
| Local toolchain | `rustc --version` and `cargo --version` return Homebrew `1.96.0`, while rustup's active stable reports `rustc 1.95.0`. | Add path/toolchain proof before any CI or docs claim that the repo is running the pinned compiler. |
| Beads | The approved remote schema migration has been applied and pushed. The live `RT-TC` hierarchy exists and `.beads/issues.jsonl` is exported. | Keep future tracker updates paired with `bd export -o .beads/issues.jsonl`. |

## Review Findings

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| R1-F01 | P1 | Floating stable CI makes failures time-dependent and hard to bisect. | Add `rust-toolchain.toml` pinning `1.96.1`, then keep a scheduled latest-stable probe as a separate drift lane. |
| R1-F02 | P1 | Cargo security fixes in `1.96.0`/`1.96.1` affect dependency-fetch behavior and bundled libssh2. | Add explicit supply-chain validation: `cargo audit`, registry assumptions, lockfile review, and Cargo version evidence. |
| R1-F03 | P1 | Workspace crates lack `rust-version`, so MSRV is implicit. | Decide MSRV policy after the `1.96.1` upgrade compiles; either set all active crates to `1.96` or document no public MSRV until first stable release. |
| R1-F04 | P2 | Exact ordered golden assertions are brittle where order is not product behavior. | Convert remaining non-contractual order checks to semantic assertions using maps, sets, sorted projections, or JSON object comparisons. |
| R1-F05 | P1 | Tauri compatibility sits under the root workspace tree but is neither a member nor excluded, so standalone validation is brittle. | First decide member vs exclude/retire, then add adapter validation if it remains publishable. |
| R1-F06 | P2 | External repos can help CI depth, but adopting runtime crates during a toolchain upgrade expands blast radius. | Adopt `taiki-e` CI tools selectively; defer `smol-rs` runtime crates until an async-runtime feature demands them. |
| R1-F07 | P1 | Gitleaks SARIF upload should not mask secret findings. | Keep upload `if: always()`, but capture the scanner exit code and fail the job after upload. |
| R1-F08 | P1 | Clippy SARIF tooling is unpinned and converter failures are not first-class. | Install pinned tool versions with `--locked`, validate generated SARIF JSON, and capture all pipeline statuses. |
| R1-F09 | P1 | Workflow tests use exact text fragments for YAML structure. | Parse workflow YAML and assert semantic sets instead of line/order-specific strings. |
| R1-F10 | P1 | Local Homebrew and rustup toolchains disagree, so version checks can pass while CI uses a different compiler path. | Add an explicit `rustup which rustc`, `which rustc`, and `cargo -Vv` proof step before pin rollout. |

## Upgrade Policy

1. Pin the repository compiler with `rust-toolchain.toml`:

   ```toml
   [toolchain]
   channel = "1.96.1"
   components = ["clippy", "rustfmt", "llvm-tools-preview"]
   profile = "minimal"
   ```

1. Keep CI reproducible by replacing `dtolnay/rust-toolchain@stable` with the
   checked-in toolchain file in PR gates. Required Rust workflows should either
   use `dtolnay/rust-toolchain@master` without a `toolchain:` override so it
   reads `rust-toolchain.toml`, or run `rustup toolchain install` from the
   checked-in file directly. Each Rust workflow must print `rustc -Vv`,
   `cargo -Vv`, and `rustup which rustc` after setup.
1. Add a scheduled `latest-stable` canary that runs `rustup update stable` and
   `cargo +stable test --workspace --all-targets`, but does not replace the
   pinned PR gate until a planned bump lands.
1. Treat `rustup` as an operator prerequisite: require `rustup >= 1.29.0` in
   docs/check scripts, but do not couple repo reproducibility to rustup
   self-update side effects.
1. Do not migrate to a newer Rust edition in this slice. Keep `edition = "2021"`
   unless a separate edition migration has failing-first tests and a public API
   review.

## Security Requirements

- Install the exact pinned toolchain locally with
  `rustup toolchain install 1.96.1 --profile minimal --component clippy --component rustfmt --component llvm-tools-preview`,
  then verify:
  - `rustc +1.96.1 --version` returns `1.96.1`.
  - `cargo +1.96.1 --version` returns `1.96.1`.
  - `rustup show active-toolchain` resolves through the repo pin when run from
    the workspace root.
  - `rustup --version` remains at least `1.29.0`.
  - `rustup which rustc`, `which rustc`, and `cargo -Vv` agree on the intended
    toolchain source, or the mismatch is documented and harmless.
- Run `cargo update` only when needed to refresh lockfile metadata under the
  pinned compiler; isolate it to an explicit dependency-refresh PR and review
  `Cargo.lock` as a supply-chain artifact.
- Run required PR/release checks with `--locked` where applicable:
  `cargo metadata --locked`, `cargo test --locked`, and `cargo clippy --locked`.
- Add `cargo audit` to the release gate. If advisory DB access is unavailable
  in CI, make that failure explicit rather than silently passing.
- Add `cargo deny check advisories bans licenses sources` with a minimal
  `deny.toml`:
  - allow only expected registries/sources,
  - deny yanked crates,
  - warn or deny duplicate major versions according to blast radius,
  - record license policy before enforcement.
- Re-run dependency-floor proof after every toolchain/dependency bump:
  - `cargo tree -i glib`
  - `cargo tree -e features -i gtk`
  - `cargo tree -p tauri --manifest-path src-tauri/Cargo.toml` after
    `src-tauri` membership/exclusion is resolved
- Make `gitleaks.yml` fail closed: scanner findings must fail the job after
  SARIF upload, not pass through `continue-on-error: true`.
- Pin cargo-installed CI tools with `cargo install --locked --version ...`, or
  use a SHA-pinned installer action after a documented trust decision.
- Treat every `cargo audit`/`cargo deny` ignore or allowlist entry as a tracked
  exception with advisory/license/source ID, owner, expiry date, and remediation
  issue.
- Preserve least-privilege GitHub Actions permissions: `contents: read` by
  default, `security-events: write` only for SARIF upload jobs.
- For required release/security workflows, prefer SHA-pinned third-party
  actions with a scheduled update lane over mutable tags.
- Review `Cargo.lock` with explicit diff rules: no unexpected registry/source
  changes, no new git dependencies, no yanked/advisory crates, no unexplained
  duplicate major versions, and no unrelated lockfile churn in toolchain-only
  PRs.

## CI Brittleness Reduction

### Order-Insensitive Review Rule

For output tests, make order a contract only when user-visible behavior depends
on order. Otherwise:

- parse JSON/SARIF before comparing;
- parse GitHub Actions YAML before asserting workflow structure;
- compare objects by stable keys such as `ruleId`, `fingerprints`, `level`, or
  command name;
- sort vectors before assertion when order is incidental;
- prefer `BTreeMap`/`BTreeSet` or sorted projections in tests when the product
  contract is membership rather than sequence;
- compare section presence, counts, and semantic fields instead of raw full-text
  output where formatting is not the behavior under test;
- keep a small number of canonical full-output goldens for rendering smoke only.
- move long manual suite definitions into a checked-in machine-readable manifest
  before expanding the matrix further.

### CI Structure

| Gate | Command | Purpose |
|---|---|---|
| Toolchain proof | `rustc --version && cargo --version && rustup --version` | Confirms pinned compiler and installer baseline. |
| Toolchain path proof | `which rustc && which cargo && rustup which rustc && rustup run 1.96.1 rustc --version && rustup run 1.96.1 cargo --version` | Prevents Homebrew/rustup path drift from hiding the compiler actually used. |
| Format | `cargo fmt --all -- --check` | Keeps rustfmt deterministic under the pinned toolchain. |
| Workspace tests | `cargo test --workspace --all-targets` | Full active workspace correctness. |
| Package matrix | existing `cargo test -p <package> --all-targets` matrix | Keeps failure localization and runner time bounded. |
| Feature matrix | `cargo hack check --workspace --each-feature --no-dev-deps`, then bounded powerset canary | Starts with low-noise per-feature drift detection; bounded powerset is not a substitute for test-target builds because `--no-dev-deps` skips dev-dependency interactions. |
| Lint | `cargo clippy --workspace --all-features --all-targets -- -D warnings` | Fails closed on warnings. |
| SARIF validation | parse `rust-clippy-results.sarif` and assert `version == "2.1.0"` plus non-empty `runs` | Ensures converter/formatter failures do not masquerade as successful lint uploads. |
| Audit | `cargo audit` | Catches known RustSec advisories. |
| Policy | `cargo deny check advisories bans licenses sources` | Enforces registry/source/license policy. |
| Coverage | existing `make llvm-cov` / `cargo llvm-cov` lane | Keeps current coverage floor visible. |
| Desktop floor | wrapper/test around `cargo tree -i glib` and `cargo tree -e features -i gtk` | Proves the old GTK/glib floor stays absent while treating "package did not match any packages" as pass and any discovered GTK/glib package as fail. |
| Tauri adapter | resolve `src-tauri` membership/exclusion, then run `cargo metadata --manifest-path src-tauri/Cargo.toml --locked --format-version 1` and `cargo check --manifest-path src-tauri/Cargo.toml` | Separates transitional compatibility from active workspace host without a broken gate. |

## External Repo Evaluation

| Source | Decision | Rationale | Validation |
|---|---|---|---|
| `taiki-e/cargo-hack` | Adopted in manual/scheduled CI canary; PR gate promotion remains a later decision. | Directly reduces feature-combination brittleness and catches hidden default-feature assumptions. | Pins `cargo-hack 0.6.45`, prints version proof, and runs `cargo hack check --workspace --each-feature --no-dev-deps`; bounded powerset promotion remains deferred until exclusions have owner/expiry/reason metadata. |
| `taiki-e/cargo-llvm-cov` | Adopted explicitly in manual/scheduled CI canary and local proof target. | Repo already has `make llvm-cov`; using the established tool keeps coverage behavior standard. | Pins `cargo-llvm-cov 0.8.7`, records `cargo llvm-cov --version`, and uses `scripts/llvm-cov.sh` to verify active-toolchain LLVM tools. |
| `taiki-e/install-action` | Consider for installing cargo tools in CI. | Can reduce shell install drift for cargo-binstall-style tools, but adds a third-party action trust decision. | Use only with pinned version/SHA and least permissions; compare against `cargo install --locked`. |
| `taiki-e/pin-project` | Defer. | Useful for custom `Future`/pin projection; no current evidence the upgrade needs it. | Re-evaluate only if async internals require manual pin projection. |
| `taiki-e/portable-atomic` | Defer. | Useful for target portability, not a toolchain upgrade need today. | Re-evaluate if supporting weaker atomic targets becomes a release requirement. |
| `smol-rs/smol` and runtime crates | Defer under [ADR-046](../architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md). | Runtime replacement or async introduction is outside the toolchain upgrade blast radius. | Evaluate only for a concrete MCP/desktop async-runtime feature with benchmarks, cancellation tests, shutdown tests, compatibility evidence, dependency diff, and rollback plan. |
| `smol-rs/async-channel` | Defer under [ADR-046](../architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md). | Could be useful if command progress/events need async MPMC channels, but adopting runtime primitives during a toolchain upgrade expands blast radius. | Re-evaluate only in a separate async-runtime ADR with race/cancellation tests and no regression in CLI/Tauri command parity. |
| `smol-rs/blocking` / `async-io` / `polling` | Defer under [ADR-046](../architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md). | Useful primitives, but adopting them during a compiler bump would confound failures. | Consider only in a separate async-runtime ADR. |

## Phased Plan

### Phase 0 - Baseline and Failing Proof

- Add a toolchain-version test or script assertion that currently fails on
  `rustc 1.96.0` when the policy expects `1.96.1`.
- Add a toolchain-path assertion that fails when unqualified `rustc`/`cargo`
  and rustup's active compiler do not agree, unless a local override explains
  the mismatch.
- Inventory every Rust-related workflow before edits:
  `rg "rust-toolchain|rustc|cargo|cargo install|rustup" .github/workflows`.
- Capture current dependency-floor output for `glib`, `gtk`, and transitional
  Tauri separately.
- Capture current CI commands and package matrix behavior.
- Capture current `gitleaks.yml`, clippy SARIF, and `src-tauri` workspace-status
  failures as first-slice remediation inputs.

**Validation**:

```bash
rustc --version
cargo --version
rustup --version
which rustc
which cargo
rustup which rustc
rustup run 1.96.1 rustc --version
rustup run 1.96.1 cargo --version
rg "rust-toolchain|rustc|cargo|cargo install|rustup" .github/workflows
cargo +1.96.1 metadata --locked --format-version 1 --no-deps
./scripts/assert-gtk-glib-absent.sh
cargo metadata --manifest-path src-tauri/Cargo.toml --locked --format-version 1 # expected failing proof until workspace membership/exclusion is resolved
```

### Phase 1 - Pin and Upgrade Toolchain

- Add `rust-toolchain.toml` pinned to `1.96.1` with `clippy` and `rustfmt`.
- Include `llvm-tools-preview` so coverage uses toolchain-local LLVM tools.
- Pin or provision `cargo-llvm-cov`, run `cargo llvm-cov --version`, and
  verify the pinned toolchain contains `llvm-tools-preview`.
- Update developer docs and prerequisites to name the pinned compiler and
  minimum rustup version.
- Decide and document MSRV:
  - if public support starts now, add `[workspace.package] rust-version =
    "1.96"` and set `rust-version.workspace = true` in all five active crates;
  - otherwise add a release-note caveat that no MSRV is promised before the
    first stable release line.

**Validation**:

```bash
rustup toolchain install 1.96.1 --profile minimal --component clippy --component rustfmt --component llvm-tools-preview
rustc +1.96.1 -Vv
cargo +1.96.1 -Vv
which rustc
which cargo
rustup which rustc
rustup run 1.96.1 rustc --version
rustup run 1.96.1 cargo --version
cargo +1.96.1 metadata --locked --format-version 1
cargo +1.96.1 fmt --all -- --check
cargo +1.96.1 test --locked --workspace --all-targets
cargo +1.96.1 clippy --locked --workspace --all-features --all-targets -- -D warnings
```

### Phase 2 - Harden Supply Chain

- Add `cargo audit` and `cargo deny` policy files/gates.
- Add pinned CI install/cache steps for `cargo-deny`, `cargo-hack`,
  `clippy-sarif`, and `sarif-fmt` using `cargo install --locked --version ...`
  or a SHA-pinned installer action after a documented trust decision.
- Fix `gitleaks.yml` so findings fail the job after SARIF upload.
- Pin clippy SARIF helper installs or replace them with a SHA-pinned installer.
- Review `Cargo.lock` under the pinned compiler with
  `cargo +1.96.1 metadata --locked --format-version 1`,
  `cargo +1.96.1 tree --locked`, and `git diff --exit-code Cargo.lock` after
  no-op checks. Allow `cargo update` only in a separate dependency-change
  commit.
- Capture the exit status for `cargo clippy`, `clippy-sarif`, `tee`, and
  `sarif-fmt`; any non-zero status fails after SARIF has been written and
  structurally validated.
- Use `set -o pipefail` or split the clippy SARIF pipeline into explicit
  intermediate files. Validate SARIF JSON before upload, upload with
  `if: always()`, then fail after upload if any stage failed.
- Make third-party registry assumptions explicit because recent Cargo CVEs are
  registry/cache related even when crates.io is unaffected.
- Preserve the GTK/glib floor proof as a release-gate requirement.

**Validation**:

```bash
cargo audit
cargo deny check advisories bans licenses sources
./scripts/assert-gtk-glib-absent.sh
make scaffold-dependency-gate
```

### Phase 3 - Reduce CI Brittleness

- Add `cargo hack` as a canary for feature combinations.
- Convert order-sensitive assertions that do not represent a user contract into
  semantic checks.
- Convert workflow CI tests from raw `text.contains(...)` checks to YAML parsing
  with set-based assertions for matrices, steps, and permissions.
- Parse root `Cargo.toml` workspace members and workflow YAML, then assert
  `matrix.include[*].package` equals active workspace package names, excluding
  only explicitly documented standalone or retired surfaces such as `src-tauri`.
- Keep `coverage_audit_render_matches_canonical_inventory_golden` and Typst
  prefix tests as rendering contracts; convert infographic `findings_ids`,
  `top_findings`, `severity_distribution`, and `maestro_layer_distribution` to
  sorted/keyed projections unless UI order is documented as product behavior.
- Add clippy SARIF validation for file existence, non-empty JSON, SARIF version,
  and `runs`.
- Keep only true rendering contracts as exact goldens.
- Split slow and flaky lanes into PR-required vs scheduled canary categories.

**Validation**:

```bash
cargo hack check --workspace --each-feature --no-dev-deps
cargo hack check --workspace --feature-powerset --no-dev-deps --depth 1
cargo test -p tachi-core --test reporting_goldens
cargo test -p tachi-core --test workflow_ci_gates
```

### Phase 4 - CI Workflow Rollout

- Update `rust-workspace.yml` and `rust-clippy.yml` to consume the checked-in
  toolchain policy.
- Add a scheduled latest-stable canary with non-mutating reporting.
- Make the latest-stable canary explicitly non-blocking for PR merge. It should
  run `rustup update stable` plus `cargo +stable test --workspace --all-targets`,
  write a check summary or issue with `rustc`/`cargo` versions on failure, and
  never mutate `Cargo.lock`.
- Keep SARIF upload `if: always()` while preserving fail-closed clippy status.
- Pin or lock cargo tool installation commands where practical.
- Resolve `src-tauri` as one of: workspace member, workspace excluded standalone
  adapter, or retired compatibility surface. Do not add a required adapter gate
  until this decision is implemented.

**Validation**:

```bash
make workflow-gate
make docs-version-gate
cargo test -p tachi-core --test workflow_ci_gates
```

### Phase 5 - Release Gate and Closeout

- Run the publish gate under the pinned toolchain.
- Record version, dependency tree, audit, deny, clippy, and coverage evidence in
  the publish checklist/BOM.
- Only then bump release docs or tags.

**Validation**:

```bash
make publish-gate
```

## Beads-Ready Issue Slices

### RT-TC-001 - Pin Rust stable toolchain to 1.96.1

- `Epic`: Rust toolchain modernization
- `Capability`: reproducible compiler and Cargo baseline
- `Task`: add `rust-toolchain.toml`, update docs, and prove local/CI use
- `Acceptance criteria`:
  - `rust-toolchain.toml` pins `1.96.1`.
  - CI no longer relies only on floating `stable` for required PR gates.
  - Local proof captures both toolchain version and toolchain path.
  - Toolchain proof appears in docs or CI logs.
  - Workspace MSRV policy is explicit: either `[workspace.package]
    rust-version = "1.96"` plus `rust-version.workspace = true` in active
    crates, or a documented no-public-MSRV lane.
- `Validation`: `rustc --version`, `cargo fmt --all -- --check`,
  `cargo test --workspace --all-targets`

### RT-TC-002 - Add supply-chain audit and policy gates

- `Epic`: Rust toolchain modernization
- `Capability`: dependency-security controls
- `Task`: add `cargo audit`, `cargo deny`, and lockfile review guidance
- `Acceptance criteria`:
  - Audit and deny checks are runnable locally and in CI.
  - Gitleaks fails closed after SARIF upload.
  - Clippy SARIF helper tools are pinned and SARIF output is structurally
    validated.
  - Registry/source assumptions are explicit.
  - GTK/glib floor proof remains part of release evidence.
- `Validation`: `cargo audit`, `cargo deny check advisories bans licenses sources`,
  `./scripts/assert-gtk-glib-absent.sh`

### RT-TC-003 - Make CI less order-sensitive

- `Epic`: Rust toolchain modernization
- `Capability`: stable, semantically meaningful validation
- `Task`: convert incidental ordering assertions to semantic comparisons
- `Acceptance criteria`:
  - Tests document when order is a product contract.
  - SARIF/JSON assertions parse data before comparing.
  - Workflow assertions parse YAML and compare semantic sets.
  - Package matrices are derived from root workspace members instead of copied
    raw strings.
  - Remaining exact goldens are limited to rendering contracts.
- `Validation`: `cargo test -p tachi-core --test reporting_goldens`
  and `cargo test -p tachi-core --test workflow_ci_gates`
- `Notes`: Implemented with YAML job/event/matrix/step projections,
  workspace-derived package matrix assertions, parsed coverage and Typst
  fields, and keyed/sorted JSON projections for infographic data where order is
  not the contract.

### RT-TC-004 - Add feature-combination canary

- `Epic`: Rust toolchain modernization
- `Capability`: feature matrix confidence
- `Task`: introduce `cargo hack` canary using taiki-e tooling
- `Acceptance criteria`:
  - Feature combinations are checked without hand-maintained matrix sprawl.
  - Any exclusions are documented with package, feature, owner, expiry, and
    reason.
  - Canary graduates to PR-required only after signal/noise is acceptable.
- `Validation`: `cargo install cargo-hack --locked --version <pinned-version>`,
  `cargo hack --version`,
  `cargo hack check --workspace --locked --feature-powerset --no-dev-deps --depth 1`

### RT-TC-005 - Separate transitional Tauri compatibility validation

- `Epic`: Rust toolchain modernization
- `Capability`: active host vs compatibility adapter clarity
- `Task`: validate `src-tauri` separately from the root workspace
- `Acceptance criteria`:
  - `src-tauri` is either a workspace member, explicitly excluded standalone
    adapter, or retired.
  - Active workspace remains GTK-free.
  - Transitional adapter has its own compile lane if still publishable.
  - Dependency-floor regressions fail loudly.
- `Validation`: `cargo metadata --manifest-path src-tauri/Cargo.toml --locked --format-version 1`,
  `cargo check --manifest-path src-tauri/Cargo.toml`,
  `cargo tree -p tauri --manifest-path src-tauri/Cargo.toml`

## Review Integration Traceability

| Finding | Priority | Disposition | Plan update |
|---|---|---|---|
| Floating stable CI and ambient toolchain checks make failures time-dependent. | P1 | Must-fix | Pin `rust-toolchain.toml`, inventory all Rust workflows, require `rustc -Vv`/`cargo -Vv`/`rustup which rustc`, and keep latest-stable as non-blocking canary. |
| Homebrew and rustup toolchains disagree locally. | P1 | Must-fix | Added path normalization proof and explicit `rustup run 1.96.1` validation. |
| Raw GTK/glib absence commands exit non-zero when the dependency is absent. | P1 | Must-fix | Replaced raw `cargo tree -i` gates with `./scripts/assert-gtk-glib-absent.sh` wrapper/test requirement. |
| Cargo lockfile and supply-chain gates need fail-closed behavior. | P1 | Must-fix | Added `--locked` metadata/test/clippy policy, pinned tool installs, audit/deny exception metadata, and no-op lockfile diff proof. |
| Gitleaks and clippy SARIF upload paths can mask scanner/converter failures. | P1 | Must-fix | Added saved scanner exit-code flow, SARIF validation, pipeline status capture, and post-upload failure requirement. |
| `src-tauri` metadata is currently an expected failure until workspace status is decided. | P1 | Fixed | Resolved `src-tauri` as an explicitly excluded standalone adapter with its own lockfile, local `make tauri-adapter-check`, and a manual/scheduled compatibility workflow. |
| Workflow and golden tests are too order/text sensitive. | P1/P2 | Bundle | Added YAML/TOML parsing, workspace-member matrix comparison, keyed/sorted data assertions, and exact-golden limits. |
| `taiki-e` tooling can reduce CI drift but needs pinned install proof. | P2 | Fixed | Added pinned manual/scheduled `cargo-hack 0.6.45` and `cargo-llvm-cov 0.8.7` canary lane, local proof targets, version evidence, and promotion guardrails. |
| `smol-rs` runtime crates expand blast radius for a compiler upgrade. | P2 | Fixed | Deferred all `smol-rs` adoption through [ADR-046](../architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md), which requires a future feature, benchmarks, cancellation/shutdown tests, compatibility evidence, dependency diff, and rollback plan. |

## Final Recommendation

Proceed in small TDD-first slices. The upgrade is low code-change risk because
the active workspace is already Rust-native, but it is medium supply-chain risk
because the local machine has Homebrew/rustup toolchain drift and recent Cargo
fixes affect registry/cache behavior and bundled SSH dependencies. Pin `1.96.1`,
normalize toolchain execution, add security gates, then reduce test brittleness
before broadening feature or async runtime scope.
