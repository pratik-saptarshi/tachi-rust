# Pipeline Reviewer — Phase 3 independent review

Agreement intensity: 30%. Strategy: checklist specialist. Score: **7.5/10**. Recommendation: **targeted corrections requested**; no P0/P1 finding established by this review.

Pinned result: `8df554e884b1e5dd24146111a965597eff5f4779`. Comparison base: `dd3b293d81d358d1ae27424be83b720693539112`. Review mode: Precise. No other reviewer outputs were read.

## Findings

### PIPE-1 — [P2] [DEFECT] [STATICALLY VERIFIED]: new binary tests import Unix APIs on every platform

- Current location: `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:203` (module enabled by `#[cfg(test)]` at line 200; fake executable permission calls at 215–219).
- Trigger: compile this new binary test target on Windows, for example `cargo check -p tachi-cli --tests --target x86_64-pc-windows-msvc` on a provisioned Windows toolchain.
- Consequence: `std::os::unix::fs::PermissionsExt` does not exist for that target, so CLI test compilation fails before portable extraction/classification tests run. The shell executable fixture and `set_mode` call are also Unix-only. Hosted Linux jobs will not catch this.
- Base comparison: `git ls-tree` at dd3b293d confirms the entire new binary is absent. Existing CLI `control_plane_cli.rs:2` guards the analogous Unix import with `#[cfg(unix)]`; its Unix-specific tests are separately gated. This newly added target introduces an unguarded platform dependency.
- Guard search: examined the complete link-monitor module, Cargo manifest, CLI tests and workspace workflow. No `cfg(unix)` surrounds this import/helper/test; Cargo does not disable autobins or this test target. The workspace job uses Ubuntu. This finding concerns test compilation, not an assertion that all Windows runtime paths were previously supported.
- Confidence: **0.98** for the target-specific compile defect. No Windows cross target is installed in the review environment (only x86_64-apple-darwin), so no cross-compile success/failure is claimed.
- Read-only verification_command: `rtk proxy sed -n '198,222p' crates/tachi-cli/src/bin/taxonomy-link-monitor.rs`
- Fix/test: guard the Unix import, fake shell helper, and fake executable test with `cfg(unix)` while retaining portable pure-function tests on all targets; add a Windows test-compilation check for the CLI.

### PIPE-2 — [P2] [DEFECT] [CODE INFERENCE]: manifest can attest source that did not generate the PDFs

- Current location: `crates/tachi-core/src/catalog_drift.rs:242`, `:278`, `:319`, `:329`; CLI accepts arbitrary `--root` at `crates/tachi-cli/src/bin/catalog-drift.rs:10`.
- Trigger: run an already-built catalog-drift binary from revision A against a checkout at revision B whose report builder changed, or use a stale `target/debug/catalog-drift` executable after editing report code. The exposed `--root` interface permits this independently of the build directory.
- Consequence: `rendering_inputs(root)` hashes B source, but `crate::try_build_report_data_typst` runs the A implementation compiled into the executable. The after-render guard only compares the on-disk source before/after rendering, so stable B source passes. Publication records B source hashes beside PDFs assembled with A logic, and subsequent `--check` passes because it only rehashes the source and stored PDF bytes. The manifest therefore does not establish its stated source-to-PDF association in this supported invocation shape.
- Base comparison: catalog-drift and its manifest do not exist at dd3b293d. This is a defect in the newly introduced provenance contract, not a claim that older PDF comparison tests were reproducible in all environments.
- Guard search: examined the full CLI, `regenerate`, `check`, manifest serialization and catalog tests for compiled source identity, embedded build hashes, root/build matching, and executable identity checks. None exists. Typst version pinning, input stability recheck, staging, and rollback are real guards, but do not compare the Rust implementation used for rendering with the source recorded in the manifest. The documented `cargo run` from the same checkout avoids the ordinary stale-binary trigger; it does not enforce the library/CLI contract.
- Confidence: **0.92** for the traced mismatch; **runtime reproduction not performed** because it would require a second executable/source revision and renderer fixture. This is not evidence that the committed PDFs currently have wrong provenance.
- Read-only verification_command: `rtk proxy sed -n '240,335p' crates/tachi-core/src/catalog_drift.rs`
- Fix/test: bind the executable to a build-time digest of the relevant Rust source/build inputs and reject incompatible roots before regeneration, or arrange regeneration through a verified build of the selected root. Add a regression using a binary compiled before a deliberate report-builder change; regeneration must reject it rather than write an apparently current manifest.

## Positive checks and dismissed candidates

- Catalog and permissions workflows restrict token permissions to contents:read, use ordinary pull_request rather than pull_request_target, and include their own workflow plus relevant dependency/setup paths. No newly introduced privileged trigger or secret exposure established.
- Permissions checks are explicitly AC-2 rule-set/document membership checks. Combining allow/ask/deny is consistent with the existing AC-2 shell contract; lack of category-policy validation was not reported as a regression.
- Catalog staging rejects symlinks, validates all render outputs before publishing, verifies pinned Typst identity, detects input drift during rendering, and attempts rollback on ordinary write failure. A failed third render is covered. No claim of missing staging/rollback is made.
- Gitleaks adopter test invokes the scanner with redaction, tests inherited and custom detection and malformed configuration, and is explicitly enabled in CI. Existing scan/final failure checks remain fail-closed.
- Rust sha2 migration keeps fixed-width byte hex; package pins and toolchain versions are updated coherently in the inspected manifests. PostgreSQL workflow uses a fresh service and verifies several positive/negative RLS cases.
- Potential inherited Typst font environment influence and interruption-time publication durability were considered but are not findings: no environment reproduction was available, and ordinary-error rollback is narrower than crash-atomic guarantees.

## Coverage and limitations

Read shared context, Standard report-data trace, root codemap, changed-file inventory and relevant diff/current source. Focused on all changed workflow diffs, Rust setup, catalog check/regeneration, native permissions, adopter scanning, link monitoring, baseline tests, Rust tool/runtime pins, frontend package migrations and PostgreSQL RLS validation. This is not exhaustive review of every changed prompt, taxonomy record, report rendering branch, binary PDF or generated lockfile.

Graph search was used first for orientation; the shared context establishes its stale-source limitation. Direct pinned files and base comparison are authoritative. No product edits, Python, network checks, dependency installations or broad test execution occurred. Typst is absent from PATH; only the local macOS Rust target is installed. Commands above are inspection commands, not claimed behavioral reproductions. Prior memory was used only for graph/workspace orientation, not as current CI evidence.
