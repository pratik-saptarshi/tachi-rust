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
- Result: 22 tests passed
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

- Local network access to GitHub Actions is unavailable in this session, so the
  live PR-run timing evidence required by the original baseline plan could not
  be refreshed here.
- The next networked CI observation should capture per-shape durations for:
  passive docs, dependency-closure crate-local, lockfile, and workflow-change
  PRs.

- Suggested GitHub median evidence command set (run once a feature branch has
  remote visibility):

  ```bash
  # Collect the latest completed PR runs with queue/execution timing fields.
  gh run list \
    --workflow rust-workspace.yml \
    --branch "$BRANCH" \
    --status completed \
    --limit 40 \
    --json databaseId,createdAt,startedAt,completedAt,status,conclusion,displayTitle,headBranch,name \
    > runs.json

  # Optional: join with python to derive queue_ms and run_ms and compute medians.
  python - <<'PY'
  import json
  from datetime import datetime, timezone
  from statistics import median

  def ts(value):
      return datetime.fromisoformat(value.replace("Z", "+00:00")).replace(
          tzinfo=timezone.utc
      ).timestamp()

  runs = json.load(open("runs.json")) or []
  queue_ms = []
  run_ms = []
  for run in runs:
      if not run.get("startedAt") or not run.get("completedAt"):
          continue
      queue_ms.append((ts(run["startedAt"]) - ts(run["createdAt"])) * 1000)
      run_ms.append((ts(run["completedAt"]) - ts(run["startedAt"])) * 1000)

  if queue_ms and run_ms:
      print(f"queue_ms_median={median(queue_ms)}")
      print(f"run_ms_median={median(run_ms)}")
  else:
      print("queue_ms_median=")
      print("run_ms_median=")
  PY
  ```

- Keep queue time and run time separated in notes so route narrowing impact is
  not masked by workflow scheduling delays.

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
