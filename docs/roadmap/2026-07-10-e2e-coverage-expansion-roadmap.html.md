# Rust-Native End-to-End Coverage Expansion Roadmap

**Status**: Remediation plan integrated from overseer review; E2E-COV-007 publish-gate/security closeout complete, E2E-COV.2 and advisory/agentic follow-ups remain open
**Date**: 2026-07-10
**Scope**: CLI, desktop-host, MCP-stdio, lifecycle, and cross-boundary failure/cancellation workflows
**Primary tracker namespace**: `E2E-COV*`

## Executive decision

The repository started with one genuine end-to-end module, `crates/tachi-shell/tests/init_substitution.rs`, with five passing tests. The expansion now adds `crates/tachi-cli/tests/e2e_artifacts.rs`, `crates/tachi-desktop/tests/e2e_command_journey.rs`, and `crates/tachi-mcp/tests/e2e_stdio_journey.rs`, while the init module also composes install, update, and analysis artifact delivery. The critical success journeys exist, the manifest-driven local runner now has terminal full-mode evidence, and the complete cross-boundary failure/cancellation matrix, repeated hosted evidence, advisory act lane, and final publish-gate closeout remain open.

This roadmap expands E2E coverage around stable user-facing boundaries while preserving the existing Rust-native unit and integration pyramid. The work is intentionally staged: freeze the boundary contract first, then execute independent CLI, desktop, and MCP slices in parallel, then compose lifecycle and failure/cancellation flows, and finally enforce coverage evidence.

## Remediation plan from overseer review

The review panel confirmed that the product-facing E2E inventory is a strong critical-flow foundation, but the local aggregate test boundary is not yet a proven deadlock or toolchain defect. The observed mismatch is architectural and operational: GitHub CI runs a route-aware package matrix and shell-suite slices with job timeouts and visible attribution, while the local publish gate still invokes one quiet `cargo test -q`. The remediation therefore preserves test breadth and changes the runner topology and observability before changing toolchains or lowering gates.

### R1 — Make local validation structurally equivalent to repository CI (`E2E-COV-007.3`, P1)

1. Add `.github/ci-test-units.json`, `schemas/ci-test-units.schema.json`, `schemas/ci-run-result.schema.json`, and `scripts/ci-local-runner.sh`. The JSON manifest is the single source for package/all-target and `tachi-shell` suite units; the workflow route job and local runner both consume it. The runner uses the pinned Rust toolchain from `rust-toolchain.toml`, declares an execution stage for each unit, and emits timestamped start/finish and duration records for every unit plus an aggregate summary. A Rust workflow contract test parses both the manifest and workflow to prevent drift; it is not a second untested copy of shell command strings.
2. Preserve `--all-targets`, the complete package set, shell suite slices, and the existing failure semantics. The runner must not replace the workspace matrix with a narrower list of binaries or omit library targets. It must validate package names, test targets, flags, and executable paths against an allowlist and must never use `eval`, `sh -c`, or untrusted shell interpolation.
3. Give each unit an explicit timeout, define SIGINT/SIGTERM escalation, process-tree cleanup, bounded log retention, and deterministic aggregate exit behavior. Create a unique `0700` temporary root, enforce path containment and symlink rejection, redact secret-bearing environment values/arguments, and remove the root on success, failure, timeout, and interruption. The default should be sequential for deterministic local diagnosis; an opt-in bounded parallel mode may be added only after the sequential contract is green.
4. Change `make test` to call `scripts/ci-local-runner.sh --mode local-full`, add `make test-route` for `--mode local-route-equivalent`, and add a workflow-contract test proving both modes remain aligned with the manifest and route policy. Keep `make publish-gate` fail-closed and never call the advisory act lane from it.
5. Verify with focused runner contract tests, each package suite independently, the full local runner, and remote `rust workspace tests` CI. The first unlabeled route-equivalent run passed 8/8 units with 0 failures, timeouts, or cancellations in 320,184 ms wall time. The subsequent full local run passed 8/8 in 294,483 ms. A labeled warm route-equivalent run passed 8/8 in 304,650 ms (compile-and-test: 237,810 ms; test slices: 66,006 ms). A controlled cold route-equivalent run passed 8/8 in 321,636 ms (compile-and-test: 266,987 ms; test slices: 53,842 ms). The terminal local-full run `20260712T173705Z-72397` passed all 8/8 units in 536,162 ms (compile-and-test: 466,327 ms; test slices: 68,906 ms), with zero failures, timeouts, cancellations, and verified cleanup. Hosted run `29175545285` then passed the route plus all five package/all-target and three shell-slice jobs; workflow wall time was about 79 seconds, with hosted job durations from 37 seconds (CLI) to 67 seconds (shell all-targets), and timing artifacts uploaded for all eight units. These local and hosted measurements are directional, not interchangeable; repeated remote samples remain required before closeout.

6. Treat performance and reliability as acceptance evidence across build and CI execution, not as an after-the-fact optimization. Record per-stage and aggregate wall time, cold/warm cache state, toolchain and host provenance, unit pass/fail/timeout/cancellation counts, artifact-validation status, and cleanup status locally. Hosted workflows must publish comparable per-job elapsed summaries and queue-versus-run timing where GitHub exposes it. Compare repeated local and hosted samples without reducing breadth, weakening timeouts, or hiding failures; regressions open a follow-up issue with the affected stage and evidence artifact.

### R2 — Use CI emulation as a diagnostic and workflow-contract aid, not as the release gate

The recommended open-source evaluation order is:

| Candidate | Reality-checked fit | Decision |
|---|---|---|
| [`nektos/act`](https://github.com/nektos/act) | Reads `.github/workflows` and executes jobs through a Docker Engine API. Its own documentation says default images are intentionally incomplete and that container execution differs from GitHub's fully virtualized runners. It supports a custom engine through `DOCKER_HOST`; Podman is therefore viable through its Docker-compatible API, not through a native act/Podman backend. | Adopt as an optional developer diagnostic after the local runner is in place. Prefer rootless Podman on macOS through `podman machine`, with a pinned image, bounded CPU/memory/disk, offline cache mode after warm-up, and no secrets. Add a documented `act` smoke command for a small workflow/job subset; do not make it a required publish gate or claim it proves GitHub parity. |
| [Dagger](https://docs.dagger.io/) | Open-source programmable CI engine with content-addressed caching, containerized execution, tracing, and local/CI consistency. It requires expressing the pipeline as Dagger functions/modules, so it is a second CI implementation rather than a GitHub Actions emulator. | Defer. Reconsider only if the repository intentionally adopts a portable pipeline engine; it is out of scope for resolving this Cargo runner boundary. |
| Firecracker / Cloud Hypervisor microVM | MicroVMs offer stronger isolation and resource controls, but Firecracker requires a Linux host with KVM and is not a Docker Engine API compatible backend for act. On this macOS workstation, Podman already requires a Linux VM, so adding a microVM would create nested virtualization and a second orchestration layer. | Defer for local development. Evaluate only as a separate Linux-hosted security/performance experiment after the Podman lane is benchmarked; it is not an implementation prerequisite for E2E-COV-007.3. |
| GitHub self-hosted runner | Executes jobs on GitHub using managed runner registration; it improves hardware/control but is not offline emulation and still depends on GitHub services. | Not a local emulation solution. Consider only for a future controlled Linux runner if native environment parity becomes a requirement. |

The plan must explicitly test the limits of any emulator: local Rust tests and artifacts are authoritative for this repository, `act` is advisory workflow/action feedback, and GitHub CI remains authoritative for hosted-runner behavior, permissions, SARIF upload, route outputs, concurrency, timeouts, cancellation, step summaries, and artifact processing. Act's known non-parity list must be checked into the plan and tested by omission. No emulator may bypass gitleaks, supply-chain, coverage, or artifact-integrity checks. Act must run with an empty secret set by default; a real `GITHUB_TOKEN`, repository secrets, host sockets, and writable host paths are prohibited in the smoke lane.

### R2a — Podman-backed act lane and resource policy

1. Add a capability probe that checks `act --version`, `podman version`, `podman machine inspect`, Docker API compatibility, architecture, available CPU/memory/disk, and the configured image digest. The probe must fail closed with an actionable skip reason rather than silently falling back to Docker or rootful Podman.
2. On macOS, use a rootless Podman machine as the supported path. Keep the machine profile configurable (`ACT_PODMAN_CPUS`, `ACT_PODMAN_MEMORY_MB`, `ACT_PODMAN_DISK_GB`) and record the chosen profile in the benchmark artifact; do not hard-code a host-specific resource promise before measuring it.
3. Use a small pinned runner image for the smoke lane, `--pull=false` after the first warm run, and `--action-offline-mode` when validating cached actions. The lane must use a temporary artifact directory, a read-only source checkout where act permits it, and no Docker/Podman socket mount inside jobs.
4. Run only deterministic smoke jobs first: workflow listing/schema parsing, the route decision job with a checked-in event fixture, and one representative package/shell slice that invokes the checked-in local runner. Do not use act to validate GitHub Code Scanning ingestion or release-please side effects.
5. Record cold-start, warm-start, peak memory, CPU time, image/cache size, and failure attribution. Establish thresholds from two cold and five warm samples; a performance regression is a follow-up signal, not permission to reduce test breadth or timeout budgets.
6. Provide an explicit fallback: if Podman or act is unavailable, the normal local runner remains usable and the act lane reports `SKIPPED_UNAVAILABLE`. A skipped advisory lane never turns the publish gate green.

MicroVM evaluation is a later Linux-only experiment. It must prove KVM availability, guest/host architecture compatibility, startup and teardown cost, CPU/memory/I/O limits, network isolation, and artifact transfer before it can be compared with rootless Podman. Firecracker's Linux/KVM requirement and lack of a direct Docker Engine API make it unsuitable as the first macOS implementation.

Act smoke explicitly does not validate GitHub permissions, concurrency groups, hosted timeouts/cancellation, step summaries, dynamic route outputs, Code Scanning ingestion, release side effects, or cross-run artifact retrieval. These remain remote-CI-only assertions.

## Delivery hierarchy and traceability

The implementation is organized as `epic → feature/capability → function/task → issue`, with every issue carrying one executable acceptance slice and one test seam:

| Level | Planned unit | Outcome |
|---|---|---|
| Epic | `E2E-COV` | Complete user-facing E2E evidence and publish-gate enforcement without weakening existing gates. |
| Feature/capability | `E2E-COV-007` coverage governance; `E2E-COV-008` local CI-parity execution; `E2E-COV-009` act/Podman advisory emulation; `E2E-COV-010` governed multi-level test evidence | Separates product coverage, local runner reliability, emulation, and test governance so one failure cannot be hidden inside another. |
| Function | Manifest projection, toolchain resolver, command executor, timeout/log reporter, act capability probe, Podman profile, event fixture, resource benchmark, test-evidence collector | Each function has a narrow contract and can be implemented behind a failing unit or integration test. |
| Task/issue | Beads children under `E2E-COV-007` through `E2E-COV-010` | Each child owns one RED/GREEN/REFACTOR slice, documented evidence, and synchronization updates. |

Dependencies are strict: runner manifest contract → local runner → publish-gate wiring → act/Podman smoke lane → performance/security evaluation → final documentation closeout. The act lane cannot block product E2E tests, and the local runner cannot be declared complete from act output.

## Test-driven development and evidence plan

Every implementation issue follows the TDD evidence record: **RED** add one behavior-focused test and run it to capture the expected failure; **GREEN** implement the smallest change and rerun the same test; **REFACTOR** remove duplication while rerunning the focused and sibling suites. No production runner, workflow, or security behavior changes land without a recorded failing test first.

| Test level | Scope and concrete seams | Required evidence |
|---|---|---|
| Unit | Manifest projection from `Cargo.toml`/workflow YAML; stable toolchain command construction; environment allowlist; timeout/budget parsing; log-path sanitization; act/Podman capability classification; benchmark aggregation | RED/GREEN transcript for each helper; malformed input, missing tool, unsupported engine, and boundary-value tests; no shell execution for pure projections. |
| Integration | Parsed `.github/workflows/rust-workspace.yml` vs local runner manifest; package/all-target and shell-suite command parity; fake cargo/act/podman binaries for exit codes, timeouts, signal forwarding, log retention, and cleanup; Beads/doc contract projections | Workflow contract test fails when a package or shell slice drifts; subprocess tests assert argv/env/cwd and bounded temp roots; no real credentials or host socket. |
| Functional | Real `make test`/local runner against all five packages and shell slices under rustup-managed 1.96.1; terminal progress and failure attribution; repeated runs with cold/warm Cargo cache | Package counts and exit statuses match CI manifest; per-unit timeout is observable; failure artifact contains command, unit, elapsed time, and log path; full breadth remains intact. |
| End-to-end | Existing CLI artifact, desktop host, MCP stdio, init/lifecycle, and failure/cancellation journeys; then `act` route/package smoke with a local event fixture and Podman profile | Valid artifacts, cleanup, no partial outputs/secrets, stable request IDs/statuses, and explicit distinction between local E2E success and advisory act success. |
| Agentic | A deterministic agent harness is allowed to select only named repository commands (`make test`, focused suites, `act` smoke, evidence collector), consume structured result JSON, and choose retry/stop/report actions. Use synthetic failures to prove it stops on security/test failures and never supplies secrets or arbitrary shell text. | Golden transcripts, command allowlist tests, bounded retries, result-schema validation, and human-review escalation for skipped/ambiguous gates. Agentic tests must not replace Rust behavior tests or remote CI. |

Required negative cases include missing rustup toolchain, Homebrew-vs-rustup mismatch, unavailable Podman machine, unsupported Docker API, stale workflow package, timeout, SIGINT cancellation, child-process leak, partial artifact, secret-bearing log, cache corruption, and act job that requires an unavailable GitHub service. Each case has a deterministic cleanup assertion and a documented disposition.

### R3 — CodeQL maintenance and upgrade path (`E2E-COV-007.5`, P2)

The workflows currently use the supported `github/codeql-action/*@v4` major for SARIF upload. The checked-in maintenance contract records GitHub release [`v4.37.0`](https://github.com/github/codeql-action/releases/tag/v4.37.0), default bundle CodeQL `2.26.0`, and Node 24 compatibility. `make codeql-maintenance-gate` now fails closed on active v3 references or missing risk-policy evidence. This is not an urgent major-version upgrade; immutable pinning remains an explicitly accepted follow-up risk.

1. Add a contract inventory for every CodeQL action reference, including historical documentation references that must remain explicitly labeled archival rather than current guidance.
2. Require immutable commit pins for active security-sensitive CodeQL action uses, with one documented release mapping to the verified `v4.37.0` release and an owner/cadence for updates. Do not mix v3/v4 or independently pin `upload-sarif` and analysis actions to different releases. If policy owners explicitly retain floating `@v4`, record a risk acceptance with monitoring, rollback, and review ownership before closing `E2E-COV-007.5`.
3. Verify Node 24 compatibility on every runner used by the workflows, especially any future macOS self-hosted runner; GitHub documents Node 24 incompatibility with macOS 13.4 and older. This repository's current CodeQL-uploading jobs run on `ubuntu-latest`, so no macOS migration is implied.
4. Add a scheduled maintenance check that reports the current upstream v4 release and creates an actionable maintenance signal for the existing Beads task, while the publish gate rejects stale v3 references and undocumented historical references only where they appear in active workflow/config surfaces. The workflow is read-only and does not mutate GitHub issues automatically.
5. Validate the update with workflow parsing, SARIF schema validation, a real remote CodeQL/SARIF run, and the existing security/workflow contract gates. A local emulator cannot validate GitHub Code Scanning ingestion; remote CI remains required.
6. SARIF-producing jobs must validate producer/category, repository-contained paths, size/message bounds, redaction, checked-out-commit provenance, and trusted-event permission boundaries before upload. Local act runs never upload SARIF or receive `GITHUB_TOKEN`.

### R4 — Preserve product E2E claims and close the evidence loop

The current four E2E modules cover initialization, CLI artifacts, desktop commands, MCP stdio, and the composed lifecycle. They provide partial failure/cancellation evidence, not a complete matrix. The plan must distinguish “journey covered” from “failure permutation verified” and must not claim comprehensive application E2E coverage until every boundary × failure mode has a named oracle, test file, and status. Every closeout update must reconcile one canonical dated audit snapshot (currently 119 active modules), roadmap, issue cards, BOM, readiness checklist, codemap, and Beads export.

### Remediation exit criteria

- `E2E-COV-007.3` is closed only with terminal local evidence or a deterministic, contract-tested workaround.
- Local test units mirror the CI package/suite surface and expose progress, attribution, timeout, and failure logs.
- `act` smoke validation is available but advisory; Dagger and self-hosted runners are not introduced as accidental scope expansion.
- `act` smoke validation uses an explicit unprivileged profile: rootless Podman via its Docker-compatible API where preflight passes, no privileged/host-network/socket/credential mounts, network disabled by default, empty secrets, ephemeral contained workspaces, pinned image digest, and recorded runtime flags. A missing or incompatible Podman/act environment is `SKIPPED_UNAVAILABLE`, not success.
- The local runner emits machine-readable JSON results and provenance: commit, toolchain/compiler/Cargo paths and versions, target triple, OS/architecture, runtime/image digest, action references/resolved SHAs, features, network/secrets state, unit timing, exit/timeout/signal state, log path, and cleanup result.
- CodeQL active references are all v4, the current release/bundle is recorded, Node 24 runner compatibility is documented, and historical v3 references are labeled or excluded from active policy checks.
- No line, region, branch, security, privacy, supply-chain, or E2E assertion is weakened to make the gate green.

## Evidence baseline

| Evidence | Current state | Consequence |
|---|---|---|
| Coverage audit | 119 active modules: 13 unit, 101 integration, 1 smoke, 4 E2E, 0 support | The E2E denominator is explicitly classified and includes CLI, desktop, MCP, and initialization journeys; 114/96 is retained only as the historical closeout snapshot. |
| Current E2E modules | `crates/tachi-cli/tests/e2e_artifacts.rs`, `crates/tachi-desktop/tests/e2e_command_journey.rs`, `crates/tachi-mcp/tests/e2e_stdio_journey.rs`, and `crates/tachi-shell/tests/init_substitution.rs` | Initialization, CLI artifacts, desktop commands, MCP stdio, init/install/update/analysis lifecycle behavior, and the cross-boundary failure matrix are covered; the 85% nightly branch target is met and publish-gate closeout remains open. |
| Workspace tests | Workspace suites pass; the current audit reports 119 active modules (101 integration, 13 unit, 1 smoke, 4 E2E) | Suite count and coverage-audit module count are different metrics and must remain separate. |
| LLVM coverage | 90.56% lines, 90.22% regions | Current stable gate passes its 85% line threshold; governed nightly branch evidence is 85.15625% (1,408 covered / 210 missed). |
| Branch coverage capability | Pinned stable 1.96.1 rejects `-Z coverage-options=branch`; explicitly pinned nightly 1.99.0 now produces 85.15625% (1,408 branches, 210 missed) when `RUSTC`, `RUSTDOC`, `LLVM_COV`, and `LLVM_PROFDATA` resolve through rustup | E2E-COV-007 meets the requested 85% branch target; retain the separately governed nightly lane and do not silently lower the threshold. |
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
- Do not claim branch-target completion without a reproducible nightly lane; the current lane reaches 85.15625% (1,408 branches, 210 missed) after focused boundary-failure coverage.
- Do not include credentials, private paths, user data, network tokens, or unsanitized generated reports in fixtures or artifacts.

## Target journey matrix

| Journey | Primary boundary | Success oracle | Failure/cancellation oracle | Planned issue |
|---|---|---|---|---|
| Init and personalization | `scripts/init.sh` via shell test harness | Personalized files and modes match baseline | Unmanifested/tracked files remain unchanged; no residual placeholders | Existing `init_substitution`; extend under `E2E-COV-005` |
| CLI analysis and artifacts | `crates/tachi-cli/src/bin/*` and shell facade | Report-data, threats-SARIF, and risk-SARIF outputs are valid and semantically consistent | Invalid args fail closed; no partial output; source URI and status remain stable | `E2E-COV-002` |
| Desktop host command flow | `crates/tachi-desktop` → `tachi-shell` | Command status/stdout/stderr and saved bytes match shared dispatch | Typed errors, path escape rejection, timeout/cancel, and child cleanup | `E2E-COV-003` |
| MCP stdio | `crates/tachi-mcp` stdio transport | Explicit startup and allowlisted request produce validated result | Cancelled/malformed/disallowed requests fail without artifact leakage | `E2E-COV-004` |
| Full adopter lifecycle | init → install/update → analysis | A clean temporary clone reaches a usable analysis artifact | Missing input, failed subprocess, and interrupted lifecycle leave no unsafe residue | `E2E-COV-005` (implemented in init-substitution E2E) |
| Cross-boundary failure/cancel | shell, desktop, CLI, MCP seams | Status/error behavior is explicit across callers | Timeout/cancel is observable and no child process or partial artifact survives | `E2E-COV-006` (partial evidence; matrix completion remains open) |

### Failure/cancellation evidence matrix

“Journey present” and “failure permutation verified” are separate states. The following matrix is the closeout contract for `E2E-COV-006`:

| Boundary | Failure modes | Required oracle | Status |
|---|---|---|---|
| CLI | invalid args/input, output write failure, cancellation | nonzero exit taxonomy, no partial artifact, bounded/redacted stderr, temp-root cleanup | partial; add named tests |
| Desktop | typed command error, path escape, timeout, cancel during child I/O | typed status, descendant liveness poll, artifact-tree snapshot, cleanup | partial; strengthen existing cancel test |
| MCP stdio | blank/malformed/unknown/disallowed request, cancellation | request ID continuity, fail-closed response, no artifact leakage, child cleanup | partial; add process-level cases |
| Lifecycle | missing manifest, install/update failure, interrupted analysis | clone remains bounded, no residual placeholders/secrets, final artifact absent or valid | success path present; failure matrix open |
| Local runner | missing toolchain, timeout, SIGINT/SIGTERM, stale manifest, child leak | JSON result, deterministic aggregate exit, process tree gone, secure logs | planned in `E2E-COV-008` |
| act/Podman | unavailable runtime, unsafe mount/secret, unsupported context, timeout | `SKIPPED_UNAVAILABLE` vs failure, policy rejection, no host access, provenance | planned in `E2E-COV-009` |

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

Add branch coverage collection and reporting to the local evidence path, update the Rust coverage audit and docs, and enforce thresholds only after the measured baseline is reviewed. The stable-toolchain attempt remains unsuitable for measurement because `cargo llvm-cov --workspace --branch --summary-only` reaches the nightly-only `-Z coverage-options=branch` requirement and exits non-zero. The explicitly governed nightly 1.99.0 lane now produces 85.15625% branch coverage after the follow-up uplift slice, meeting the requested target while stable line/region gates remain unchanged. E2E-COV-007 is closed after complete publish/security evidence synchronization. The first enforcement target is:

- line coverage ≥ 85%;
- region coverage ≥ 85%;
- branch coverage ≥ 85%, with the current 85.15625% result recorded and no unexplained regression;
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
