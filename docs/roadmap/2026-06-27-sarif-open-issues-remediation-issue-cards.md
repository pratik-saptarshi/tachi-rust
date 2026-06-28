# SARIF Open Issues Remediation Issue Cards

**Last Updated**: 2026-06-27
**Status**: Beads-ready execution blueprint for GitHub issues `#2` and `#6`
**Source**: [SARIF Open Issues Remediation Roadmap](./2026-06-27-sarif-open-issues-remediation-roadmap.html.md)

These cards are TDD-first. Each card maps to a measurable closure condition for
one remaining open GitHub issue and its Beads mirror.

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

### RT-sarifepic / SARIF-OPEN-001 - Close remaining SARIF contract issues

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: Remaining GitHub issue closure for `#2` and `#6`
- `Capability`: SARIF output contract correctness and downstream consumer compatibility
- `Task`: close the two residual SARIF defects without changing unrelated output shapes
- `Function`: `crates/tachi-core/src/sarif_common.rs`, `crates/tachi-core/src/threats_sarif.rs`, `crates/tachi-core/src/risk_scores.rs`
- `Dependencies`: current `origin/main` at or after `1796bb6`
- `Acceptance criteria`:
  - GitHub issues `#2` and `#6` are objectively remediated and closed.
  - Beads mirrors `RT-bu7` and `RT-0zv` are closed with traceability.
  - Threat and risk-score SARIF outputs preserve finding IDs, source URIs, and severity properties.
  - `make publish-gate` and remote PR checks pass.
- `Validation`: `make publish-gate`, `gh pr checks <PR> --watch --interval 10`
- `Implementation owner`: `tachi-core`
- `Stage label`: SARIF contract closure
- `Next test seam`: `crates/tachi-core/tests/{risk_scores,threats_sarif}.rs`
- `Priority`: 1
- `Notes`: Do not batch unrelated SARIF schema or report-format changes into this epic.

## RT-bu7 / GitHub #2

### RT-bu7a - Add red tests for caller-supplied baseline run identity

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: Dynamic SARIF baseline run identity
- `Capability`: prove the current frozen baseline defect
- `Task`: add failing tests showing threat and risk SARIF cannot currently emit a caller-supplied non-static `baselineRunId`
- `Function`: `crates/tachi-core/tests/risk_scores.rs`, `crates/tachi-core/tests/threats_sarif.rs`
- `Dependencies`: none
- `Acceptance criteria`:
  - A threat SARIF test fails on current `main` because an unchanged finding cannot emit a supplied baseline run ID.
  - A risk-score SARIF test fails on current `main` for the same reason.
  - Both tests preserve current empty `baselineRunId` expectations for `NEW` findings.
  - The failure message names GitHub `#2` / Beads `RT-bu7`.
- `Validation`: focused failing tests before implementation, then green after `RT-bu7b`
- `Implementation owner`: `tachi-core`
- `Stage label`: Stage 0 red
- `Next test seam`: `baselineRunId` assertions in SARIF builder tests
- `Priority`: 1
- `Notes`: Keep this card test-only until the red failure is captured.

### RT-bu7b - Thread explicit SARIF run context through both builders

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: Dynamic SARIF baseline run identity
- `Capability`: shared run identity contract
- `Task`: replace the frozen baseline helper with an explicit shared SARIF run context or equivalent caller-provided value
- `Function`: `crates/tachi-core/src/sarif_common.rs`, `crates/tachi-core/src/threats_sarif.rs`, `crates/tachi-core/src/risk_scores.rs`
- `Dependencies`: `RT-bu7a`
- `Acceptance criteria`:
  - Unchanged findings in threat SARIF emit the exact caller-supplied `baselineRunId`.
  - Unchanged findings in risk-score SARIF emit the exact same caller-supplied `baselineRunId`.
  - New findings in both pipelines still emit an empty `baselineRunId` unless an explicit ADR changes that contract.
  - No static frozen value is required by the builder API for non-test callers.
  - Existing source URI, rule ID, severity, and fingerprint fields remain stable except for the intended baseline field.
- `Validation`: `cargo test -p tachi-core --test risk_scores --offline`, `cargo test -p tachi-core --test threats_sarif --offline`
- `Implementation owner`: `tachi-core`
- `Stage label`: Stage 1 green
- `Next test seam`: shared SARIF context helper
- `Priority`: 1
- `Notes`: Prefer a small context type over adding independent string arguments to every helper.

### RT-bu7c - Prove command-level baseline propagation

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: Dynamic SARIF baseline run identity
- `Capability`: CLI/shell caller integration
- `Task`: prove the command surface supplies or derives the baseline run identity consistently for generated SARIF artifacts
- `Function`: `crates/tachi-shell/src/command_use_cases.rs`, `crates/tachi-cli/tests/control_plane_cli.rs`, `crates/tachi-shell/tests/tauri_bridge.rs`
- `Dependencies`: `RT-bu7b`
- `Acceptance criteria`:
  - `risk-scores-sarif` command output includes the expected non-static baseline for unchanged findings.
  - `threats-sarif` command output includes the same baseline for unchanged findings.
  - Command-level tests prove the value is derived from caller/input context, not from the frozen helper.
  - Generated SARIF remains schema-valid.
- `Validation`: `cargo test -p tachi-cli --test control_plane_cli --offline`, `cargo test -p tachi-shell --test tauri_bridge --offline`
- `Implementation owner`: `tachi-shell`
- `Stage label`: Stage 1 command integration
- `Next test seam`: SARIF command output tests
- `Priority`: 1
- `Notes`: If no existing CLI argument should expose this, derive from source artifact parent/run directory and document the rule.

## RT-0zv / GitHub #6

### RT-0zva - Add red tests for non-standard logicalLocation.kind

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: SARIF logical location kind compliance
- `Capability`: prove the current non-standard `data-store` emission
- `Task`: add failing tests showing `Data Store` currently emits a non-standard logical-location kind in both SARIF pipelines
- `Function`: `crates/tachi-core/tests/risk_scores.rs`, `crates/tachi-core/tests/threats_sarif.rs`
- `Dependencies`: none
- `Acceptance criteria`:
  - A risk-score SARIF test fails on current `main` when asserting no `data-store` kind is emitted.
  - A threat SARIF test fails on current `main` when asserting the same.
  - Tests assert parity across both pipelines, not just absence in one builder.
  - The failure message names GitHub `#6` / Beads `RT-0zv`.
- `Validation`: focused failing tests before implementation, then green after `RT-0zvb`
- `Implementation owner`: `tachi-core`
- `Stage label`: Stage 0 red
- `Next test seam`: logical-location kind assertions
- `Priority`: 1
- `Notes`: The tests should not require a broader SARIF schema overhaul.

### RT-0zvb - Omit or spec-map unsupported logical-location kinds

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: SARIF logical location kind compliance
- `Capability`: standards-compatible logical-location emission
- `Task`: change the shared mapping so unsupported DFD classes omit `kind` or map to a documented SARIF-compatible value
- `Function`: `crates/tachi-core/src/sarif_common.rs`, `crates/tachi-core/src/threats_sarif.rs`, `crates/tachi-core/src/risk_scores.rs`
- `Dependencies`: `RT-0zva`
- `Acceptance criteria`:
  - `Data Store` no longer emits `kind: "data-store"` in threat SARIF.
  - `Data Store` no longer emits `kind: "data-store"` in risk-score SARIF.
  - Threat and risk-score outputs remain field-shape compatible for `name` and `fullyQualifiedName`.
  - Any emitted `kind` value is backed by a code comment or test naming the intended SARIF-compatible contract.
  - Existing reporting goldens are updated only for the intended logical-location field changes.
- `Validation`: `cargo test -p tachi-core --test risk_scores --offline`, `cargo test -p tachi-core --test threats_sarif --offline`, `cargo test -p tachi-core --test reporting_goldens --offline`
- `Implementation owner`: `tachi-core`
- `Stage label`: Stage 2 green
- `Next test seam`: shared logical location builder
- `Priority`: 1
- `Notes`: Prefer omission over inventing a quasi-standard value if the SARIF spec does not define a suitable DFD class.

## Closure Cards

### RT-sarifgate - Publish gate and tracker closure

- `Epic`: `RT-sarifepic` / SARIF-OPEN-001
- `Feature`: Release and tracker closure
- `Capability`: prove remediation and close remote/local trackers
- `Task`: run release gates, merge PR, close GitHub and Beads issues with evidence
- `Function`: `Makefile`, `.github/workflows/*`, `bd export`, GitHub issue comments
- `Dependencies`: `RT-bu7c`, `RT-0zvb`
- `Acceptance criteria`:
  - `cargo fmt --all -- --check` passes.
  - `cargo test --workspace --all-targets --offline` passes.
  - `cargo clippy --workspace --all-features --all-targets -- -D warnings` passes.
  - `make publish-gate` passes.
  - `make llvm-cov` reports at least 85% line coverage.
  - PR checks pass remotely.
  - GitHub issues `#2` and `#6` are closed with exact commit, PR, and validation references.
  - Beads issues `RT-bu7` and `RT-0zv` are closed and `.beads/issues.jsonl` is exported.
- `Validation`: local gates plus `gh pr checks <PR> --watch --interval 10`
- `Implementation owner`: release owner
- `Stage label`: Stage 3 publish
- `Next test seam`: publish gate
- `Priority`: 1
- `Notes`: Do not close trackers until remote `origin/main` contains the merged remediation.
