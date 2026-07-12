# Publish Readiness Checklist

**Status**: Active release gate
**Last Updated**: 2026-07-12
**Purpose**: confirm `tachi-rust` is ready to publish to `origin/main`
**Scope**: security, privacy, docs, tests, coverage, CI, and release hygiene

Use this checklist before publishing to GitHub or cutting a release. The
active desktop host is `crates/tachi-desktop`; the former `src-tauri` adapter is
retired from the active dependency surface.

## 0. Canonical publish sequence

- [ ] `pre-commit run --all-files` or the equivalent `gitleaks` scan passes.
- [ ] `make gitleaks-gate` passes and is included in `make publish-gate`; the
      local gate validates SARIF shape and propagates scanner failures.
- [ ] `make publish-gate` passes on the release candidate branch.
- [ ] `cargo test -p tachi-shell` passes after the script executor boundary
      slice and coverage-invariant cleanup.
- [ ] `cargo test -p tachi-mcp --test contract_snapshot --test schema_snapshot --test tools_registration --test session_policy --test stdio`
      passes and the MCP scaffold builds with `cargo build -p tachi-mcp --features stdio`.
- [ ] `crates/tachi-mcp/tests/tools_registration.rs` and
      `crates/tachi-mcp/tests/stdio.rs` cover tool allowlisting, artifact
      emission, and stdio request/response handling.
- [ ] `crates/tachi-mcp/tests/session_policy.rs` covers request-id continuity
      and cancellation handling without artifact leakage.
- [ ] `make scaffold-dependency-gate` passes before publishing scaffold or template changes.
- [ ] `make fuzz-mutation-gate` passes and `.github/workflows/fuzz-mutation-audit.yml` remains scheduled/manual and non-blocking.
- [ ] The `E2E-COV*` roadmap and Beads hierarchy are synchronized with the
      current E2E journey matrix and no critical user-facing boundary is
      represented only by a unit or integration test.
- [ ] The explicit E2E inventory is current: CLI artifact, desktop command,
      MCP stdio, initialization, composed init/install/update/analysis
      lifecycle, and cross-boundary failure/cancellation journeys have
      focused suites; E2E-COV-007 remains for coverage evidence and publish
      enforcement.
- [ ] `cargo run -q -p tachi-cli --bin coverage-audit` reports the intended
      E2E inventory, with no double-counting between integration, smoke, and
      true end-to-end categories.
- [ ] Feature and coverage canaries are run serially when promoted for release
      evidence: `make feature-combination-canary`, then `make coverage-tool-proof`.
- [ ] `git push origin main --follow-tags` is the intended publish command.
- [ ] `gh run list --branch main --limit 10` is ready for post-push monitoring.
- [ ] `gh run watch <run-id>` will be used until the publish workflow completes.

## 1. Repository hygiene

- [ ] `git status --short --branch` is clean except for intentional, reviewed work.
- [ ] `git diff --check` passes.
- [ ] No untracked scratch files, temporary exports, or local-only artifacts are
      present in the publish set.
- [ ] The branch name and commit messages are conventional and self-explanatory.
- [ ] `make docs-version-gate` and `make docs-archive-version-gate` pass after
      documentation updates included in the release slice.

## 2. Security and privacy

- [ ] No secrets, API keys, tokens, private keys, or credentials exist in the
      commit range to be published.
- [ ] No personal data, customer data, or private assessment output appears in
      examples, fixtures, screenshots, logs, or generated docs.
- [ ] Public-facing examples are synthetic, redacted, or already committed as
      safe fixtures.
- [ ] Security issues that are not safe for public disclosure stay in private
      vulnerability reporting.
- [ ] The BOM at [bill-of-materials.html.md](./bill-of-materials.html.md) was
      reviewed for any sensitive surfaces that need redaction.
- [ ] `README.md`, `SECURITY.md`, `CHANGELOG.md`, and the public docs under
      `docs/` do not leak private paths, credentials, internal-only status, or
      unreleased operational details.

## 3. Secret scanning

- [ ] `pre-commit run --all-files` passes, or the equivalent gitleaks command has
      been run successfully.
- [ ] The secret scan covers committed examples, fixtures, docs, generated
      reports, and workflow files.
- [ ] `.github/workflows/gitleaks.yml` is present and matches the local secret
      scan policy.
- [ ] Any legitimate placeholder or fixture match is documented and justified.
- [ ] No new warnings were introduced by hook configuration changes.
- [ ] `rg "actions/checkout@v[0-6]|actions-rs/toolchain@|github/codeql-action/upload-sarif@v3|::set-output" .github/workflows` returns no matches.
- [x] `make codeql-maintenance-gate` passes; active SARIF uploads use CodeQL Action v4 with the documented v4.37.0 / CodeQL 2.26.0 mapping and explicit floating-tag risk acceptance.
- [x] `make codeql-upstream-release-check` passes against the public release API; `.github/workflows/codeql-maintenance.yml` provides the read-only weekly/manual upstream-release signal.
- [ ] Any future immutable CodeQL SHA pin is verified against its upstream release and updated atomically across all active references.

## 4. Rust validation

- [ ] `make test` passes through the manifest-driven local runner, with JSON result/provenance output, per-unit timeout and cleanup evidence; `make test-route` also passes or its deterministic environment limitation is documented.
- [ ] The local runner preserves all five package/all-target units and all three `tachi-shell` suite slices from `.github/ci-test-units.json`; `cargo test -q` is not used as the opaque publish-gate runner.
- [ ] Local runner evidence records per-stage and aggregate build/test duration, cold/warm cache context, toolchain/host provenance, pass/fail/timeout/cancellation counts, artifact validation, and cleanup status.
- [x] Local route-equivalent evidence records 8/8 passed units with zero failure/timeout/cancellation outcomes: initial 320,184 ms, full 294,483 ms, labeled warm 304,650 ms, and controlled cold 321,636 ms (compile/test 266,987 ms; test slices 53,842 ms). Hosted comparison and artifact-download verification remain required.
- [x] Hosted timing artifact integrity is verified for main run `29178308727` and PR run `29178255153`: eight artifacts each, commit/run/stage/unit/duration fields validated by `make verify-ci-timing-artifacts`; PR evidence uses the synthetic merge commit embedded in `GITHUB_SHA` artifacts.
- [ ] Hosted CI evidence records comparable per-job elapsed timing and queue-versus-run timing where available; local and hosted measurements are clearly labeled and not treated as interchangeable.
- [x] Current mainline timing evidence is recorded: workspace 40-run median 71s and route-observe 11-run median 14s, both with 0s queue median.
- [x] Repository branch protection is enabled and required checks are verified; `gh api repos/pratik-saptarshi/tachi-rust/branches/main/protection` returns the documented route, security, formatting, package, and shell contexts with strict up-to-date enforcement, linear history, conversation resolution, and force-push/deletion protection.
- [x] Hosted workspace run `29175545285` passed route, five package/all-target, and three shell-slice jobs; timing artifacts were produced for all eight units and job durations ranged from 37s to 67s. Queue/run medians and repeated samples remain open.
- [ ] Repeated local and hosted observations show no unexplained reliability regression, leaked child process, partial artifact, or nondeterministic aggregate exit; any limitation has an owner, issue, and rollback/mitigation note.
- [ ] `make act-smoke` is opt-in and advisory only. If used, rootless Podman Docker-API preflight, image digest, empty secrets, disabled network, no host/socket mounts, resource profile, and `SKIPPED_UNAVAILABLE`/failure distinction are recorded.
- [ ] `Cargo.toml` declares workspace `rust-version = "1.96"` and active
      crates inherit it with `rust-version.workspace = true`.
- [ ] `rust-toolchain.toml` pins the approved release toolchain and includes
      `clippy`, `rustfmt`, and `llvm-tools-preview`.
- [ ] Required Rust workflows install the checked-in toolchain policy and print
      `rustc -Vv`, `cargo -Vv`, `which rustc`, `which cargo`, and
      `rustup which rustc`.
- [ ] `cargo audit` passes locally or in the equivalent
      `.github/workflows/rust-supply-chain.yml` gate.
- [ ] `cargo deny check advisories bans licenses sources` passes against
      `deny.toml`, and any future license/source exceptions include owner,
      expiry, issue, and remediation metadata.
- [ ] `cargo test --workspace --all-targets` passes or the equivalent
      `.github/workflows/rust-workspace.yml` PR gate is green.
- [ ] `.github/workflows/rust-workspace.yml` skips the heavy matrix for
      passive-docs-only PRs, narrows crate-local changes to dependency
      closure, and keeps the route classifier stable.
- [ ] `.github/actions/rust-setup/action.yml` centralizes the shared Rust
      toolchain, cache, and proof steps used by Rust-facing workflows.
- [ ] The required-check migration note in
      `docs/tachi-rust-ci-execution-plan.md` names the old matrix checks, the
      new stable route checks, and the rollback rule for protected refs.
- [ ] `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and the adapter
      compatibility workflow remain absent after retiring the vulnerable
      Tauri/GTK adapter surface.
- [ ] `cargo hack --version` reports `0.6.45`, `cargo llvm-cov --version`
      reports `0.8.7`, and `make feature-combination-canary` passes before any
      feature-combination canary is promoted to a required PR gate.
- [ ] Parser hardening regression tests pass, including delta-count normalization and panic-free status handling.
- [ ] Reporting goldens pass with semantic projections for coverage, report,
      threat, risk, and infographic outputs.
- [ ] Workflow CI gates pass with YAML-parsed event/job/matrix/step assertions
      and workspace-derived package matrix checks.
- [ ] [ADR-046](./architecture/02_ADRs/ADR-046-async-runtime-adoption-boundary.md)
      remains the async-runtime boundary: no `smol-rs` runtime dependency is
      added without a separate MCP or desktop feature, benchmarks,
      cancellation/shutdown tests, compatibility evidence, dependency diff, and
      rollback plan.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.
- [ ] `make llvm-cov` passes and the coverage floor remains above the project
      threshold; latest local run on 2026-07-11 measured 90.56% lines / 90.22%
      regions, with the configured 85% line threshold passing.
- [ ] The governed nightly branch command produces at least 85% branch
      coverage. Current evidence is 85.09% (1,408 total / 210 missed) on
      nightly 1.99.0 using explicit rustup-resolved compiler and LLVM tools.
- [ ] Any benchmark or regression gate referenced by the roadmap has its current
      baseline recorded.
- [ ] `INSTALL_MANIFEST.md` only references files/directories that exist in the
      repository and expected install command paths.
- [ ] `make scaffold-dependency-gate` passes and blocks scaffold ranges that admit
      currently known vulnerable `next` or `vitest` versions.
- [ ] `make fuzz-mutation-gate` passes, the advisory fuzz/mutation workflow stays
      manual or scheduled, and its baseline report remains offline-safe.

## 5. Documentation readiness

- [ ] `README.md` matches the actual build, install, usage, and release path.
- [ ] `docs/platform-compatibility.md` matches the current harness matrix, support levels, install surfaces, and fallback behavior.
- [ ] `docs/guides/DEVELOPER_GUIDE_TACHI.md` matches the public README and
      explains the first analysis flow in plain language.
- [ ] `README.md`, `docs/platform-compatibility.md`, and
      `docs/guides/DEVELOPER_GUIDE_TACHI.md` describe the same standalone MCP
      server build, run, and validation contract.
- [ ] `adapters/README.md` matches the compatibility matrix and the canonical
      core contract.
- [ ] `crates/tachi-mcp/` is reflected in the BOM, install manifest, and
      release notes where the standalone MCP scaffold is public-facing.
- [ ] MCP request-context hardening is reflected in the BOM and checklist so
      Stage 2 transport policy stays visible while the server grows.
- [ ] `SECURITY.md` matches the current private-reporting and privacy policy.
- [ ] `CHANGELOG.md` is redaction-safe and reflects only releasable notes.
- [ ] `docs/roadmap/implementation-backlog.md` points at the archived AQ
      roadmap, active AISVS/security roadmap, live RT-CI execution track,
      closed docs-sweep/MCP/RT-TC records, and archived provenance docs.
- [ ] `docs/tachi-rust-ci-execution-plan.md`, `docs/tachi-rust-ci-beads-issue-cards.md`,
      and `docs/tachi-rust-ci-review-panel.md` stay synchronized with the live
      RT-CI Beads hierarchy and the checked-in workflow gate changes.
- [ ] `docs/tachi-rust-ci-baseline.md` stays synchronized as the Phase 0
      baseline snapshot and local validation record for RT-CI.
- [ ] `docs/tachi-rust-ci-closeout.md` stays synchronized as the current RT-CI
      closeout note, separating local proofs from pending GitHub-side checks.
- [ ] `docs/ci-improvement-plan.html` stays synchronized as the source plan
      draft that feeds the RT-CI execution docs and tracker cards.
- [ ] `docs/tachi-rust-ci-route-policy.md` stays synchronized with the
      routing-policy rules used by the RT-CI route and observe-only phases.
- [ ] `docs/tachi-rust-ci-route-fixtures.md` stays synchronized with the
      route matrix and stable JSON examples used by the RT-CI fixture tests.
- [ ] `docs/tachi-rust-ci-route-artifact.md` stays synchronized with the
      emitted `route.json` schema and the stable orchestrator check contract.
- [ ] The AQ roadmap is archived at
      `docs/roadmap/2026-06-22-adversarial-architecture-test-quality-roadmap.html.md`.
- [ ] The active AISVS roadmap is
      `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-roadmap.html.md`.
- [ ] The active AISVS scope and issue-card source is
      `docs/roadmap/2026-06-23-aisvs-dependabot-remediation-issue-cards.md`.
- [ ] The MCP roadmap and issue-card files are retained as historical records
      for the closed `MCP-001*` hierarchy:
      `docs/roadmap/2026-06-25-standalone-mcp-server-roadmap.html.md` and
      `docs/roadmap/2026-06-25-standalone-mcp-server-issue-cards.md`.
- [ ] Closed `RT-00i.6` remains the historical publish-readiness evidence for
      the AISVS roadmap; future AISVS release-gate deltas open new tracker
      slices instead of reusing the closed follow-up.
- [ ] The docs-sweep roadmap and issue-card files are retained as completed
      historical records:
      `docs/roadmap/2026-06-21-archived-docs-workflow-version-sweep-roadmap.html.md`
      and `docs/roadmap/2026-06-21-archived-docs-workflow-version-sweep-issue-cards.md`.
- [ ] The archived roadmap docs are clearly marked historical only.
- [ ] The roadmap and issue cards reflect the current phase sequencing, with
      closed AQ work retained as historical context.
- [ ] The historical `glib` Dependabot proof is captured in
      `crates/tachi-core/tests/scaffold_dependency_floors.rs`, and the
      current workspace no longer resolves `gtk` or `glib`.
- [ ] The AISVS registry exposes stable per-control validation commands, and
      the docs reference the registry-level contract instead of only the
      individual test names.
- [ ] The DOC-00X documentation-update plan remains separate from the parity
      and docs-sweep tracks.
- [ ] `docs/bill-of-materials.html.md` and `docs/publish-readiness-checklist.html.md`
      agree on the publish gate, security surfaces, RT-CI workflow surfaces,
      and remote publication flow.
- [ ] `docs/bill-of-materials.html.md` includes the MCP roadmap and MCP issue
      cards as historical records for the closed `MCP-001*` hierarchy.
- [ ] The public README, compatibility doc, developer guide, BOM, and
      publish-security checklist describe the same install, analysis, adapter,
      and release workflow.
- [ ] Golden update policy is documented: semantic projections first, compact
      fixture-local snapshots second, and full-envelope equality only when a
      schema contract truly requires it.
- [ ] The shell executor seam is documented in the BOM and covered by focused
      shell crate tests.
- [ ] The infographic prompt scaffold seam is documented in the BOM and
      covered by focused core crate tests.
- [ ] The infographic payload seam is documented in the BOM and covered by
      focused core crate tests.
- [ ] `make docs-version-gate` passes.
- [ ] `make docs-archive-version-gate` passes.
- [ ] Public docs do not promise unsupported features or outdated workflows.
- [ ] `tachi-core` reporting and scoring consumers compile against root facade exports instead of module internals.
- [ ] Release notes, changelog entries, and user-facing examples are current and
      redaction-safe.

## 6. CI and GitHub readiness

- [ ] `.github/workflows/gitleaks.yml` is green for the publish branch.
- [ ] `.github/workflows/rust-workspace.yml` is green and is not
      path-filtered on pull requests.
- [ ] `.github/workflows/rust-workspace.yml` consumes `rust-toolchain.toml`
      instead of floating on `stable`.
- [ ] `.github/workflows/rust-workspace.yml` completes within the runner window via its package-sized test matrix.
- [ ] `.github/workflows/rust-workspace.yml` includes `tachi-mcp` in the
      package matrix and runs the MCP validation suite.
- [ ] `.github/workflows/rust-clippy.yml` is green.
- [ ] `.github/workflows/rust-clippy.yml` fails closed on warnings while still
      uploading SARIF with `if: always()`.
- [ ] `.github/workflows/rust-clippy.yml` consumes `rust-toolchain.toml`
      instead of overriding the repo pin with floating `stable`.
- [ ] `.github/workflows/rust-supply-chain.yml` is green and runs pinned
      `cargo-audit` and `cargo-deny` versions.
- [ ] `.github/workflows/ci-workflow-parse.yml` is green and fails broken
      workflow YAML before heavy jobs start.
- [ ] `.github/workflows/ci-route-observe.yml` is green and uploads a
      `route.json` artifact while keeping the orchestrator check stable.
- [ ] `.github/workflows/rustfmt.yml` is green and isolates formatting drift
      from the full workspace matrix.
- [ ] `.github/workflows/rust-feature-coverage-canary.yml` stays manual or
      scheduled, installs pinned `cargo-hack` / `cargo-llvm-cov`, prints tool
      versions, and is not a required PR/main-push gate until reviewed.
- [ ] `.github/workflows/gitleaks.yml` fails closed after SARIF upload when
      scanner execution or SARIF validation fails.
- [ ] The latest main-push Actions run does not emit Node 20 deprecation warnings from the updated workflows.
- [ ] `gh api repos/:owner/:repo/branches/:branch/protection` is checked and
      documented when required-check migration is active. A 404 is a blocker until
      branch protection-based required-check migration is confirmed by policy.
- [ ] `.github/workflows/release-please.yml` ignores docs-only and roadmap-only
      pushes and does not churn release-PR branches on main pushes.
- [ ] `.github/workflows/tachi-mmdc-preflight.yml` is green.
- [ ] `.github/workflows/tachi-pytest.yml` is either retired or scoped strictly to
      transitional compatibility with a documented deprecation plan.
- [ ] The docs/version gate is green on the current branch.
- [ ] The release artifact gate and checksum matrix pass via `make publish-gate`.
- [ ] MCP roadmap, issue cards, BOM, and publish checklist remain synchronized
      as closed MCP evidence before any future MCP release promotion opens a
      new tracker hierarchy.
- [ ] RT-CI roadmap, issue cards, BOM, and publish checklist remain synchronized
      as the live CI-hardening track evolves.
- [ ] The scaffold dependency-floor audit passes via `make scaffold-dependency-gate`
      and is included in `make publish-gate`.
- [ ] Any release workflow required for the branch has succeeded or is queued
      without failures.
- [ ] GitHub Actions status was checked after the last merge or rebase.
- [ ] No workflow emits secrets, private paths, or private data into logs.

## 7. Remote publication

- [ ] The branch to publish is up to date with the intended base branch.
- [ ] `gh run list --workflow rust-workspace.yml --branch main --status completed --limit 5`
      is checked after merge and before release claim.
- [ ] `gh run watch <run-id>` completes with a green `rust-workspace.yml`,
      `ci-route-observe.yml` (if scoped), and `release-please.yml` sequence.
- [ ] All required-check migration evidence required by RT-CI remains in the
      local closeout notes and the associated Beads issue note trail.
- [ ] The publish commit history is linear or intentionally merged.
- [ ] The push target is `origin/main` or a clearly named release branch.
- [ ] `make publish-gate` runs clean on the branch being published, including
      workflow drift, scaffold dependency-floor, and release artifact parity checks.
- [ ] The post-push CI monitor command is ready, for example:

```bash
gh run list --branch main --limit 10
gh run watch <run-id>
```
- [ ] The CI monitor runs until the relevant release, lint, security, and docs
      jobs all finish successfully.

## 8. Publish decision

- [ ] The repo is safe to publish.
- [ ] The repo is documented well enough for an outside reader to use.
- [ ] The repo is passing the required validation gates.
- [ ] The repo can be pushed to `origin` without exposing secrets or private
      material.

## Required exit criteria

Do not push to `origin/main` until all of the following are true:

1. Repository hygiene passes.
2. Security and privacy pass.
3. Secret scanning passes.
4. Rust validation passes.
5. Documentation is current.
6. GitHub Actions status is green or understood.

If any item fails, fix the failing gate first and rerun the checklist from the
top.
