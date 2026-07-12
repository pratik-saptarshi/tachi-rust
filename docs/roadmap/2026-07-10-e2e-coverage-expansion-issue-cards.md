# E2E Coverage Expansion Beads Issue Cards

**Source plan**: [Rust-Native End-to-End Coverage Expansion Roadmap](./2026-07-10-e2e-coverage-expansion-roadmap.html.md)
**Namespace**: `E2E-COV*`
**Status**: live Beads hierarchy; overseer remediation integrated; E2E-COV-007.1 branch target met, E2E-COV-007.3 terminal local runner evidence complete, aggregate publish-gate closeout pending

## Epic

### E2E-COV — Rust-native end-to-end coverage expansion

- **Type**: epic
- **Goal**: expand true E2E evidence from initialization to CLI, desktop, MCP, lifecycle, and resilience journeys without weakening deterministic or security gates.
- **Dependencies**: none; consumes the existing RT-CI workflow contract and does not make historical RT-CI tracker status a substitute for current local/remote evidence.
- **Acceptance**: all child journeys have executable tests, coverage audit classification, branch/line/region evidence, synchronized docs, and green publish/security gates.
- **Validation**: roadmap-linked Beads tree, `bd ready --json`, workspace tests, coverage audit, `cargo llvm-cov --branch`, gitleaks, `make publish-gate`.
- **Priority**: P1

## Hierarchy and evidence contract

The tracker uses the following refinement: `E2E-COV` epic → capability features (`E2E-COV-007` through `E2E-COV-010`) → boundary functions → implementation tasks/issues. Each issue must name its production boundary, test seam, owner, dependency, RED command/failure, GREEN command, regression command, and synchronized documentation surfaces.

The canonical baseline is the dated output of `cargo run -q -p tachi-cli --bin coverage-audit`: **113 active modules** — 13 unit, 95 integration, 1 smoke, 4 E2E, 0 support/regression — with four E2E modules. Older 109/110/112-module references are historical and must not be copied into new acceptance criteria.

| Capability | User-facing function | Current boundary | Required evidence | Feature |
|---|---|---|---|---|
| CLI artifact generation | architecture input → report/SARIF artifacts | `tachi-cli` / `tachi-core` | semantic artifact, exit, partial-write, and cleanup assertions | `E2E-COV-002` |
| Desktop dispatch | host command → shared shell → status/artifact | `tachi-desktop` / `tachi-shell` | typed status, timeout/cancel, descendant cleanup | `E2E-COV-003` |
| MCP stdio | request → allowlisted tool → response | `tachi-mcp` | request ID, malformed/disallowed/cancelled response, cleanup | `E2E-COV-004` |
| Lifecycle composition | init → install/update → analysis | `scripts/init.sh` / shell / CLI | isolated clone, offline control-plane, final artifact | `E2E-COV-005` |
| Failure cleanup | timeout/cancel/error → safe terminal state | all user-facing boundaries | process liveness, artifact tree, redacted diagnostics | `E2E-COV-006` |
| Local CI parity | CI matrix/slices → local observable units | `Makefile` / `scripts/ci-local-runner.*` | parsed manifest parity, JSON result/provenance, bounded runner | `E2E-COV-008` |
| Workflow emulation | workflow/event/job → advisory local result | `act` + Podman API | opt-in smoke, empty secrets, isolation/resource/provenance | `E2E-COV-009` |
| Test governance | RED/GREEN evidence → promotion decision | Rust tests/docs/Beads | level-specific TDD evidence and agentic replay | `E2E-COV-010` |

### Required issue evidence template

Every new or reopened card must include:

- **RED**: exact focused command, expected failure string, and why the pre-change behavior is insufficient;
- **GREEN**: minimal implementation command and focused pass output;
- **REFACTOR**: duplication/design review plus focused and sibling regression commands;
- **Promotion**: unit → integration → functional → E2E → remote CI evidence as applicable;
- **Safety**: secret/network policy, temp-root containment, cleanup/liveness assertion, and provenance artifact;
- **Sync**: roadmap, BOM, readiness checklist, codemap, Beads export, and integration log update.

## Child cards

### E2E-COV-001 — Freeze journey contracts and E2E baseline

- **Type**: task
- **Dependencies**: none
- **Acceptance**:
  - journey matrix and boundary ownership are recorded;
  - current dated 113-module / four-E2E baseline is captured from the coverage-audit binary;
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
  - no child process or partial artifact survives failure/cancel, proven by bounded liveness polling and artifact-tree comparison.
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
  - no partial artifact or secret-bearing diagnostic is emitted, proven by artifact policy and redaction/gitleaks assertions.
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

### E2E-COV-008 — Local CI-parity execution capability

- **Type**: feature
- **Dependencies**: `E2E-COV-007.3` blocks closeout; consumes the workflow contract from RT-CI.
- **Capability**: replace opaque local `cargo test -q` with a typed, observable, rustup-pinned runner that mirrors both full and route-equivalent CI surfaces.
- **Acceptance**:
  - `scripts/ci-local-runner.sh` and a checked-in typed manifest/result schema exist;
  - workflow YAML parsing derives and compares package/all-target and all three shell-suite slices;
  - runner emits per-unit JSON with unit, argv, toolchain, target, start/end/duration, exit/signal/timeout, log path, cleanup, and pass/fail fields;
  - each unit declares a build/test stage and the aggregate result records total duration, stage totals, cold/warm cache context, and pass/fail/timeout/cancellation counts;
  - command construction uses allowlisted argv arrays and rejects shell metacharacters, `eval`, `sh -c`, unexpected executables, duplicate units, and stale packages;
  - unique `0700` temp roots, containment/symlink checks, redaction, bounded logs, process-tree cleanup, and deterministic aggregate exit behavior are contract-tested;
  - `make test` uses the runner, while `make publish-gate` remains fail-closed and does not call act.
- **Functions/tasks**: manifest projection, rustup provenance collector, argv executor, timeout/signal supervisor, result writer, cleanup verifier, workflow drift contract.
- **Child issues**: `E2E-COV-008.1` manifest/schema parity; `E2E-COV-008.2` runner safety/observability; `E2E-COV-008.3` Make/docs integration.
  - **Validation**: RED/GREEN unit and integration tests, executable fake-cargo argv/redaction/timeout/descendant-cleanup tests in `ci_local_runner_contract.rs`, real five-package functional run, repeated cold/warm measurements, `make workflow-gate`, focused workspace contracts, `make verify-ci-timing-artifacts` for all eight hosted artifacts, comparable hosted job/queue timing, and remote package-matrix CI. For pull requests, artifact commit provenance is the synthetic merge commit exposed to the workflow, not necessarily the API run `headSha`.
- **Priority**: P1

### E2E-COV-009 — Advisory act workflow emulation with Podman

- **Type**: feature
- **Dependencies**: `E2E-COV-008`; does not block product E2E or publish-gate acceptance.
- **Capability**: provide fast local workflow/action wiring feedback without treating emulation as GitHub-hosted proof.
- **Acceptance**:
  - one opt-in `make act-smoke`/`scripts/act-smoke.sh` command targets a named workflow/job and synthetic event fixture;
  - rootless Podman through its Docker-compatible API is preferred only after capability preflight; unsupported or unavailable environments return `SKIPPED_UNAVAILABLE` distinctly from failure;
  - no privileged, host-network, host-filesystem, Docker/Podman socket, SSH-agent, cloud credential, repository secret, or real `GITHUB_TOKEN` access is permitted; network is disabled by default;
  - runner image digest, act/Podman versions, architecture, CPU/memory/disk profile, cache mode, isolation flags, repository commit, and action references/resolved SHAs are recorded;
  - cold/warm performance samples record startup, wall time, CPU, peak memory, image/cache size, and cleanup; thresholds are baselined before enforcement;
  - act results cannot satisfy hosted-CI, CodeQL/SARIF-ingestion, release, security, coverage, or publish acceptance by themselves.
- **Functions/tasks**: capability probe, Podman machine profile, event fixture, smoke selector, resource sampler, provenance/result reporter, secret/network policy contract.
- **Child issues**: `E2E-COV-009.1` act/Podman capability preflight; `E2E-COV-009.2` advisory smoke and resource benchmark.
- **MicroVM boundary**: Firecracker/Cloud Hypervisor is deferred to a Linux/KVM-only experiment; it is not a drop-in act backend and is not required on macOS.
- **Validation**: RED/GREEN script-contract tests, Podman smoke when available, unavailable-runtime test, isolation-policy test, and remote CI comparison.
- **Priority**: P2

### E2E-COV-010 — Governed multi-level and agentic test evidence

- **Type**: feature
- **Dependencies**: `E2E-COV-002` through `E2E-COV-009` as applicable.
- **Capability**: make TDD evidence and promotion decisions reproducible across unit, integration, functional, E2E, and agentic test layers.
- **Acceptance**:
  - every implementation issue records RED/GREEN/REFACTOR commands and output plus AC/test mapping;
  - unit, integration, functional, E2E, and failure/cancellation matrices have named tests and promotion gates;
  - required agentic tests use scripted fake models/tools, deterministic seeds/replays, bounded iterations/timeouts, allowlisted commands, denied-tool/approval/circuit-breaker/cancellation cases, and no live model/network calls;
  - golden result/provenance schemas and human escalation rules distinguish pass, fail, skipped-unavailable, and inconclusive;
  - coverage-audit and planning surfaces consume one dated baseline.
- **Functions/tasks**: TDD evidence parser, test-level catalog, failure matrix, fake agent/tool harness, replay comparator, promotion gate, canonical baseline synchronizer.
- **Child issues**: `E2E-COV-010.1` TDD evidence and test-level promotion contract; `E2E-COV-010.2` deterministic agentic replay harness.
- **Validation**: contract tests, focused Rust suites, coverage audit, documentation tests, gitleaks, and remote CI.
- **Priority**: P2

### E2E-COV-007.1 — Raise nightly branch coverage to 85 percent

- **Type**: task
- **Dependencies**: child of `E2E-COV-007`
- **Acceptance**:
  - the governed nightly 1.99.0 branch lane reaches at least 85%;
  - stable line/region thresholds remain unchanged and green;
  - newly exercised desktop, shell-bridge, CLI, and error paths have focused tests;
  - Beads, roadmap, BOM, checklist, and codemap report one consistent baseline.
- **Current evidence after uplift slice 24**: 85.09% branch coverage, 1,408 total branches / 210 missed, measured with explicit nightly `RUSTC`, `RUSTDOC`, `LLVM_COV`, and `LLVM_PROFDATA` paths. Slice 24 adds deterministic CLI help/error and artifact-write cases, desktop headless/schema/offline cases, MCP stdio blank-line/startup cases, shell-bridge cancellation and artifact failure edges, and a fail-closed local gitleaks publish target. Stable line/region gates remain 90.56% / 90.22%.
- **Validation**: nightly branch report, focused boundary tests, `make llvm-cov`, gitleaks, and `make publish-gate`.

### E2E-COV-007.3 — Resolve aggregate local publish-gate test runtime boundary

- **Type**: task
- **Dependencies**: child of `E2E-COV-007`
- **Acceptance**:
  - the aggregate local test phase reaches a terminal green result, or a deterministic documented runner workaround is contract-tested;
  - no coverage, security, or privacy gate is weakened;
  - Beads, codemap, BOM, and publish checklist record the final evidence.
- **Current evidence**: crate-level suites pass independently (core 372, shell 78, CLI 31, desktop 46, MCP 20); terminal local-full run `20260712T173705Z-72397` passed all 8 manifest units in 536,162 ms (compile-and-test 466,327 ms; test slices 68,906 ms), with zero failures, timeouts, cancellations, and verified cleanup. The manifest-driven runner is the documented bounded replacement for the previously opaque aggregate `cargo test -q` path.
- **Validation**: `make publish-gate`, package-level regression suites, workflow contract tests, and synchronized release documentation.
- **Priority**: P1

#### Remediation acceptance refinement from overseer review

- Use `.github/ci-test-units.json` as the canonical manifest and `scripts/ci-local-runner.sh --mode local-full|local-route-equivalent` as the observable runner; do not replay workflow command strings with `eval` or `sh -c`.
- Require JSON result/provenance records, rustup path/version proof, secure `0700` temp roots, containment/symlink checks, redaction, bounded logs, timeout/signal/process-tree cleanup, and deterministic aggregate exit semantics.
- Add workflow-contract tests for package membership, all-target flags, shell-suite membership, route modes, duplicate/stale units, and manifest drift.
- Evaluate [`nektos/act`](https://github.com/nektos/act) only as an opt-in advisory workflow/action smoke tool. Prefer rootless Podman through its Docker-compatible API when preflight passes, but treat compatibility as best-effort; no secrets, privileged mode, host/network/socket/credential mounts, or hosted-CI claims. Defer Dagger and self-hosted runners as separate architecture decisions.
- Close only after terminal local evidence or a deterministic, contract-tested environment workaround is documented in the roadmap, BOM, readiness checklist, codemap, and Beads export.

### E2E-COV-007.5 — CodeQL v4 maintenance and release verification

- **Type**: task
- **Dependencies**: none
- **Current evidence**: active workflows use `github/codeql-action/upload-sarif@v4`; upstream currently lists `v4.37.0` with CodeQL bundle `2.26.0`, released 2026-07-08.
- **Acceptance**:
  - all active CodeQL action references remain on one supported v4 release line;
  - the release/bundle version and Node 24 runner requirement are recorded;
  - stale v3 references are absent from active workflows, while historical docs are labeled as archival;
  - any immutable SHA pin is verified and updated atomically across all CodeQL action uses;
  - SARIF producer/category, repository-contained paths, size/message bounds, redaction, checked-out-commit provenance, and trusted-event permission boundaries are contract-tested;
  - workflow parsing, SARIF validation, and remote CodeQL/SARIF execution pass.
- **Validation**: `make codeql-maintenance-gate`, `make codeql-upstream-release-check`, workflow contract gate, `rg` inventory, SARIF schema checks, remote GitHub CI/CodeQL, and the scheduled/manual read-only upstream-release workflow; current immutable SHA pinning remains an explicitly tracked follow-up risk.
- **Priority**: P2
