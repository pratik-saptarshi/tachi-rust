# Tachi-Rust CI Closeout Notes

**Status**: draft closeout evidence for RT-CI
**Purpose**: separate locally proven RT-CI changes from external verification
items that still require GitHub access

## Proven Locally

- Route policy manifest and route artifact contracts exist and are covered by
  workflow contract tests.
- Passive-docs narrowing, dependency-closure routing, and the emergency full-CI
  override are implemented in `rust-workspace.yml`.
- Protected refs (`main`, `release/*`, and tags) are forced to full mode.
- Shared Rust setup is centralized in `.github/actions/rust-setup/action.yml`.
- Heavy Rust-facing workflows emit elapsed runtime summaries.
- Phase 0 baseline inventory and local validation snapshot are recorded in
  `docs/tachi-rust-ci-baseline.md`.
- Warm local timing comparison exists for the same workflow test on
  `origin/main` (`real 0.58s`) and the current branch (`real 1.39s`), but it
  is not a substitute for live PR median evidence.
- Beads export and issue notes are updated after each slice and can be used by
  release operators when evidence gaps remain.
- On 2026-07-10, the full workspace test gate passed 468 tests across 111
  suites; the standalone coverage gate passed at 84.77% regions and 85.25%
  lines. The combined publish gate had one transient workspace-parallel test
  failure that did not reproduce in the workspace or coverage-only reruns.
- The local gitleaks 8.30.1 scan passed with no leaks; this does not replace
  the required GitHub gitleaks workflow result.

## Still Pending External Verification

- Live GitHub Actions timing evidence: repeated PR-specific timing evidence for
  pre-router vs post-router median PR
  durations. The current mainline sample is recorded in
  `docs/tachi-rust-ci-baseline.md`; collect PR/event-filtered samples via
  `make rt-ci-latency-evidence` when representative PR runs exist.
- Branch-protection verification that the required-check migration is safe to
  finalize. The current collector reports `main` unprotected (HTTP 404), so
  publish closure must remain fail-closed until repository governance is
  configured and rechecked.
- Post-push monitoring of `main` after a publish step.

## Latest Attempted Remote Evidence Pull (2026-07-11)

- `make rt-ci-latency-evidence` and direct API checks were executed from this
  branch with elevated network privileges:
  - `branch_protection=pratik-saptarshi/tachi-rust/main: unavailable`
    (`gh api .../branches/main/protection` returns HTTP 404: branch is not
    currently protected).
  - `rust-workspace.yml` PR-side median evidence command (`pull_request` event):
    `sample_size=2`, `run_med_ms=350000`, `queue_med_ms=0`,
    `run_range_ms=93000..607000`.
  - `ci-route-observe.yml` PR-side evidence command (`pull_request` event):
    `sample_size=1`, `run_med_ms=16000`, `queue_med_ms=0`,
    `run_range_ms=16000..16000`.
- Route-observe artifact evidence was downloaded from PR run
  `29091065279` (`ci route observe`); it reports:
  - `mode=observe_only`
  - `selected_lanes=[\"full-pr-matrix\"]`
  - `escalation_reasons=[\"active docs or shared surface touched\"]`
- Legacy `rust-workspace.yml` PR evidence from run `29091065263` reports
  `mode=full_pr_matrix` and `reason=protected ref stays full mode`.
- Evidence confirms the evidence collection path is now functional and
  synchronized with docs and artifacts.
- Current mainline median collection: `rust-workspace.yml` sample size 40,
  run median 71 seconds, queue median 0 seconds; `ci-route-observe.yml` sample
  size 11, run median 14 seconds, queue median 0 seconds. Branch protection is
  enabled and the required-check API response matches the documented contract.

## Publish-Readiness Guardrails Before Merge Closure

- `make publish-gate` must pass before any branch merge intended to close RT-CI.
- Mainline remote evidence must include both:
  - stable full-mode coverage for protected refs (`main`, release refs, tags, and
    lockfile/workflow changes), and
  - route-observe artifact emission for non-forced docs-only PRs.
- Required-check migration notes in `docs/tachi-rust-ci-execution-plan.md` must
  match the exact route policy currently in use.
- `docs/tachi-rust-ci-baseline.md` must continue to include the latest local and
  remote timing notes, with queue and run time separated.

## Evidence Links

- [Baseline snapshot](./tachi-rust-ci-baseline.md)
- [Execution plan](./tachi-rust-ci-execution-plan.md)
- [Route policy](./tachi-rust-ci-route-policy.md)
- [Route artifact](./tachi-rust-ci-route-artifact.md)
