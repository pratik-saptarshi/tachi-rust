# E2E Coverage Expansion Beads Issue Cards

**Source plan**: [Rust-Native End-to-End Coverage Expansion Roadmap](./2026-07-10-e2e-coverage-expansion-roadmap.html.md)
**Namespace**: `E2E-COV*`
**Status**: ready for live Beads creation

## Epic

### E2E-COV — Rust-native end-to-end coverage expansion

- **Type**: epic
- **Goal**: expand true E2E evidence from initialization to CLI, desktop, MCP, lifecycle, and resilience journeys without weakening deterministic or security gates.
- **Dependencies**: none; follows the closed RT-CI work but does not depend on remote RT-CI closeout evidence.
- **Acceptance**: all child journeys have executable tests, coverage audit classification, branch/line/region evidence, synchronized docs, and green publish/security gates.
- **Validation**: roadmap-linked Beads tree, `bd ready --json`, workspace tests, coverage audit, `cargo llvm-cov --branch`, gitleaks, `make publish-gate`.
- **Priority**: P1

## Child cards

### E2E-COV-001 — Freeze journey contracts and E2E baseline

- **Type**: task
- **Dependencies**: none
- **Acceptance**:
  - journey matrix and boundary ownership are recorded;
  - current 109-module / one-E2E baseline is captured;
  - semantic coverage-audit tests distinguish module inventory from test-suite count;
  - deterministic fixture and privacy rules are documented.
- **Validation**: `cargo test -p tachi-core --test coverage_audit --test coverage_catalog --test reporting_goldens`; `cargo run -q -p tachi-cli --bin coverage-audit`.
- **Next test seam**: red audit assertion for the new E2E contract inventory.
- **Priority**: P1

### E2E-COV-002 — CLI analysis to report and SARIF artifacts

- **Type**: feature
- **Dependencies**: `E2E-COV-001`
- **Acceptance**:
  - a real CLI invocation consumes a deterministic architecture fixture;
  - report-data, threats-SARIF, and risk-scores-SARIF outputs validate semantically;
  - output-file bytes match stdout where the contract requires parity;
  - invalid arguments and invalid input fail closed without partial artifacts;
  - the test is classified as E2E or a documented integration boundary, not double-counted.
- **Validation**: focused CLI E2E suite, SARIF schema tests, artifact byte/parity checks, `cargo test -p tachi-cli --all-targets`.
- **Next test seam**: red test for one end-to-end CLI artifact journey.
- **Priority**: P1

### E2E-COV-003 — Desktop host command journey

- **Type**: feature
- **Dependencies**: `E2E-COV-001`
- **Acceptance**:
  - the active desktop host invokes shared shell dispatch through its public host boundary;
  - status/stdout/stderr and preview/save bytes match direct shared dispatch;
  - path escape, typed failure, timeout, and cancellation behavior are observable;
  - no child process or partial artifact survives failure/cancel.
- **Validation**: focused desktop E2E suite plus `cargo test -p tachi-desktop --all-targets` and shell bridge tests.
- **Next test seam**: red host-level command round-trip test.
- **Priority**: P1

### E2E-COV-004 — MCP stdio request to tool result

- **Type**: feature
- **Dependencies**: `E2E-COV-001`
- **Acceptance**:
  - explicit stdio startup is required;
  - a valid request reaches the allowlisted tool and returns validated output metadata;
  - malformed, unknown, disallowed, and cancelled requests fail closed;
  - request IDs and artifact cleanup remain observable across the transport boundary.
- **Validation**: focused MCP E2E suite plus contract snapshot, schema, session-policy, stdio, and tool-registration tests.
- **Next test seam**: red process-level stdio request/response test using a local child process.
- **Priority**: P1

### E2E-COV-005 — Full init/install/update/analysis lifecycle

- **Type**: feature
- **Dependencies**: `E2E-COV-002`, `E2E-COV-003`, `E2E-COV-004`
- **Acceptance**:
  - a unique temporary clone runs initialization and reaches the analysis boundary;
  - install/update control-plane behavior is exercised without network access;
  - a final report or SARIF artifact is produced and validated;
  - cleanup leaves the temporary clone and subprocess state bounded and safe.
- **Validation**: lifecycle E2E suite, init matrix, CLI/desktop/MCP focused suites, workspace tests.
- **Next test seam**: red clean-clone lifecycle scenario.
- **Priority**: P1

### E2E-COV-006 — Cross-boundary failure and cancellation matrix

- **Type**: feature
- **Dependencies**: `E2E-COV-002`, `E2E-COV-003`, `E2E-COV-004`
- **Acceptance**:
  - timeout, cancellation, malformed input, disallowed command, output escape, and child-process cleanup are tested at user-facing boundaries;
  - status/error taxonomy is consistent across CLI, desktop, and MCP callers;
  - no partial artifact or secret-bearing diagnostic is emitted.
- **Validation**: failure/cancel E2E suite, typed error tests, gitleaks, workspace tests.
- **Next test seam**: red cancellation test with process-liveness assertion.
- **Priority**: P1

### E2E-COV-007 — Coverage evidence and publish-gate enforcement

- **Type**: feature
- **Dependencies**: `E2E-COV-005`, `E2E-COV-006`
- **Acceptance**:
  - `cargo llvm-cov --branch` produces a recorded branch baseline;
  - line, region, and branch thresholds are explicit and enforced without silent lowering;
  - coverage-audit counts, roadmap, BOM, checklist, and codemap agree;
  - full publish, security, privacy, supply-chain, and workspace gates pass.
- **Validation**: `cargo llvm-cov --workspace --branch --summary-only` under a separately governed nightly lane or an explicitly documented deferral, `make coverage-audit`, `make publish-gate`, gitleaks, docs contract tests.
- **Next test seam**: red threshold/documentation drift assertion.
- **Priority**: P2
