# Tachi-Rust CI Closeout Notes

**Status**: original RT-CI closeout is complete; route-specific timing evidence
remains open under Beads `RT-0sd`
**Purpose**: distinguish locally proven RT-CI changes, hosted governance
evidence, and post-closeout operational monitoring

## Proven Locally

- Route policy manifest and route artifact contracts exist and are covered by
  workflow contract tests.
- `scripts/ci-route-classifier.sh` is the shared source for passive-docs,
  dependency-closure, and full-matrix decisions used by both routing workflows.
- Direct protected-ref runs (`main`, `release/*`, and tags), active contracts,
  shared surfaces, uncertainty, and emergency full-CI overrides remain full
  mode. Protected refs (`main`, `release/*`, and tags) are forced to full mode.
  This applies when they are the execution ref; a PR targeting `main` can
  still be narrowed.
- Shared Rust setup is centralized in `.github/actions/rust-setup/action.yml`.
- Heavy Rust-facing workflows emit elapsed runtime summaries.
- Phase 0 baseline inventory and local validation snapshot are recorded in
  `docs/tachi-rust-ci-baseline.md`.
- Warm local timing comparison exists for the same workflow test on
  `origin/main` (`real 0.58s`) and the current branch (`real 1.39s`).
- Beads export and issue notes are updated after each slice and can be used by
  release operators when evidence gaps remain.
- On 2026-07-10, the full workspace test gate passed 468 tests across 111
  suites; the standalone coverage gate passed at 84.77% regions and 85.25%
  lines. The combined publish gate had one transient workspace-parallel test
  failure that did not reproduce in the workspace or coverage-only reruns.
- The local gitleaks 8.30.1 scan passed with no leaks; this does not replace
  the required GitHub gitleaks workflow result.

## Route classification and measurement status (2026-10-05)

The route-specific timing audit found that both PR workflows treated
`github.base_ref=main` as the protected execution ref, forcing every PR to run
the full matrix. Their active-contract pattern also matched every `crates/`
path before dependency-closure routing. The correction centralizes route
classification, evaluates the actual execution ref, and exposes
`route_mode`/`selected_packages` in the observe-only artifact. Behavioral
regressions cover passive docs, package closure, and full-mode fallbacks.
Passive-docs narrowing applies only to a main-target PR whose changed paths are
limited to passive documentation; direct protected-ref runs remain full mode.

The correction merged in PR #54 as `9d5b2733420bf5511a12c9a04cb36172937cd864`
after all required hosted contexts passed; Beads `RT-0sd.3` is closed. The
historical pre-router history contains no comparable route cohorts, so the
ten-run comparison and 35% target remain open under `RT-0sd.2`. See the
[route-specific timing plan](roadmap/2026-10-05-rt-ci-timing-evidence-followup.md).

## Route-specific timing acceptance (open follow-up)

- Live GitHub Actions timing evidence for the planned performance comparison is
  not complete. The dated aggregate samples do not provide separate pre-router
  and post-router cohorts for `passive-docs` and `dependency-closure`.
- The open follow-up [RT-CI timing evidence plan](./roadmap/2026-10-05-rt-ci-timing-evidence-followup.md)
  requires ten successful PR runs in each route/cohort combination and a
  minimum 35% reduction in each route shape's median execution time. Queue and
  run times remain separate, and failed or route-unknown runs are excluded.

## Operational Monitoring (post-closeout)

- Continue collecting representative PR-specific timing samples via
  `make rt-ci-latency-evidence`; after the route-specific acceptance is met,
  additional samples are operational monitoring. Current pooled samples are
  recorded in `docs/tachi-rust-ci-baseline.md` and are not completion evidence.
- Branch-protection verification was refreshed on 2026-10-04: `main` requires
  17 strict status contexts, including CodeQL and the PostgreSQL migration/RLS
  workflow. Admin enforcement and linear history are enabled; force pushes and
  branch deletion are disabled. Recheck after future policy changes.
- Post-push monitoring of `main` after a publish step.

## Dated Remote Evidence

- The representative PR timing sample below was captured on 2026-07-12:
  - `rust-workspace.yml` PR-side median evidence command (`pull_request` event):
    `sample_size=22`, `run_med_ms=85000`, `queue_med_ms=0`,
    `run_range_ms=79000..101000`.
  - `ci-route-observe.yml` PR-side evidence command (`pull_request` event):
    `sample_size=23`, `run_med_ms=14000`, `queue_med_ms=0`,
    `run_range_ms=11000..17000`.
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
  enabled; the 2026-10-04 live response confirms 17 strict required contexts.

## Historical Publish-Readiness Guardrails (satisfied)

- These gates were required before RT-CI closure. The RT-CI epic and all seven
  children are now closed in Beads.
- `make publish-gate` passed before RT-CI closure.
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
