# Tachi-Rust CI Baseline Snapshot

**Status**: baseline reference for RT-CI Phase 0
**Purpose**: record the pre-routing CI contract, required checks, and local
validation snapshot so later routing changes can be compared against a stable
reference

## Workflow Inventory

- `ci-workflow-parse.yml`
- `rustfmt.yml`
- `rust-clippy.yml`
- `rust-supply-chain.yml`
- `rust-workspace.yml`
- `ci-route-observe.yml`
- `release-please.yml`
- `gitleaks.yml`
- `rust-feature-coverage-canary.yml`
- `fuzz-mutation-audit.yml`
- `tachi-pytest.yml`
- `tachi-mmdc-preflight.yml`

## Required-Check Map

- `route decision and stable orchestrator check`
- `cargo fmt --all -- --check`
- `actionlint` parse gate
- `rust-clippy analyze`
- `cargo audit and cargo deny`
- `Upload analysis results to GitHub` for clippy SARIF
- `Upload SARIF to GitHub Code Scanning` for gitleaks

## Matrix Inventory

- Package matrix: `tachi-core`, `tachi-mcp`, `tachi-cli`, `tachi-shell`,
  `tachi-desktop`
- Shell matrix slices:
  - `shell-smoke`
  - `shell-init`
  - `shell-integration`

## Local Validation Snapshot

- `cargo test -p tachi-core --test workflow_ci_gates`
- Result: 23 tests passed
- `git diff --check`
- Result: clean

## Elapsed Runtime Summaries

- Heavy Rust-facing workflows now emit elapsed runtime summaries with
  `elapsed_ms` notes to `GITHUB_STEP_SUMMARY`.
- Covered jobs:
  - `rust-workspace.yml` package matrix and shell slices
  - `rust-clippy.yml`
  - `rustfmt.yml`
  - `rust-supply-chain.yml`

## Timing Notes

### Current hosted mainline sample (2026-07-11)

The current `make rt-ci-latency-evidence` run collected the following live
mainline sample:

- `rust-workspace.yml`: sample size 40, median run `67,000 ms`, queue median
  `0 ms`, run range `2,000..361,000 ms`, queue range `0..0 ms`.
- `ci-route-observe.yml`: sample size 5, median run `16,000 ms`, queue median
  `0 ms`, run range `12,000..16,000 ms`, queue range `0..0 ms`.

These are hosted workflow medians, not local wall-time equivalents. The
collector also failed its branch-protection check because
`pratik-saptarshi/tachi-rust/main` is currently unprotected (GitHub HTTP 404).
That governance failure remains a separate publish-gate blocker; it does not
invalidate the timing measurements.

- PR-side timing evidence was collected via GitHub Actions APIs in this session.
  Representative command lines:

  ```bash
  ./scripts/rt-ci-latency-evidence.sh "rust-workspace.yml,ci-route-observe.yml" main 40 pull_request
  ```

  - `rust-workspace.yml`: `sample_size=2`, `run_med_ms=350000`,
    `queue_med_ms=0`, `run_range_ms=93000..607000`, `queue_range_ms=0..0`.
  - `ci-route-observe.yml`: `sample_size=1`, `run_med_ms=16000`,
    `queue_med_ms=0`, `run_range_ms=16000..16000`, `queue_range_ms=0..0`.

  The session did not collect a full 10-run PR representative sample yet.

- Suggested GitHub median evidence command set (run once a feature branch has
  remote visibility):

  ```bash
  # Collect latest completed runs and derive queue/run medians.
  make rt-ci-latency-evidence
  ```

  The helper emits:

  - `workflow=<workflow-name>`
  - `branch=<branch-name>`
  - `sample_size=<N>`
  - `run_med_ms=<ms>`
  - `queue_med_ms=<ms>`
  - `queue_range_ms=<min>..<max>`
  - `run_range_ms=<min>..<max>`

- Keep queue time and run time separated in notes so route narrowing impact is
  not masked by workflow scheduling delays.

- live PR-run timing evidence required by the original baseline plan remains the
  blocker for final RT-CI timing closure.

## Route-Observe Evidence Snapshot

- PR `29091065279` (`ci-route-observe`) artifact output shows:
  - `mode`: `observe_only`
  - `selected_lanes`: `["full-pr-matrix"]`
  - `escalation_reasons`: `["active docs or shared surface touched"]`
- PR `29091065263` (`rust-workspace`) route output indicates `mode=full_pr_matrix`
  and `reason=protected ref stays full mode`, with `packages` unchanged from
  `["tachi-core","tachi-mcp","tachi-cli","tachi-shell","tachi-desktop"]`.

## Local Timing Snapshot

- `cargo test -p tachi-core --test workflow_ci_gates`
- Result: `real 1.62s`, `user 0.12s`, `sys 0.08s`
- This is the post-change local validation timing available from the current
  session, and it should be compared with future networked PR-run evidence.

## Warm Timing Comparison

- `origin/main` warm run: `real 0.58s`, `user 0.08s`, `sys 0.08s`
- Current branch warm run: `real 1.39s`, `user 0.10s`, `sys 0.08s`
- These are local workflow-test measurements, not GitHub PR medians, and they
  are recorded here as a reproducible before/after comparison for the RT-CI
  timing track.

## Notes

- This snapshot intentionally records the contract that was present before the
  routing changes in the current feature branch.
- The execution plan and publish checklist should remain synchronized with this
  snapshot as the RT-CI rollout progresses.
