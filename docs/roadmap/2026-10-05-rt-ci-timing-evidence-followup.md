# RT-CI Route-specific Timing Evidence Follow-up — 2026-10-05

Status: **Phase 0 complete; route-specific performance acceptance remains open**.
This follow-up addresses the unresolved post-merge timing review on PR #26
(comment `4177119281`). The original RT-CI epic and `RT-CI-006.2` remain closed
for their recorded implementation and aggregate-sample scope; this plan tracks
the stricter route-specific comparison required by the execution plan.

## Evidence gap

The dated PR sample records 22 workspace runs and 23 route-observe runs, but
does not separate pre-router and post-router medians for the `passive-docs` and
`dependency-closure` route shapes. It therefore does not establish the planned
35% median reduction for either shape. The existing latency collector requests
completed runs without requesting or filtering `conclusion`, so failed runs can
enter its sample. These aggregate numbers remain historical context, not proof
that the route-specific acceptance target passed.

## Milestones and Beads

| Milestone | Beads | Delivery boundary | Exit validation |
|---|---|---|---|
| 0 — Reconcile the existing review and tracker state | RT-0sd | This planning/update PR on the branch attached to PR #26 | Current PR head, all prior hosted jobs and unresolved review threads inspected; live Beads records and checked-in export reconciled |
| 1 — Make timing collection success-only and reproducible | RT-0sd.1 | Focused collector/test PR | A behavioral regression fixture mixes successful and failed completed runs; only successful PR runs affect sample size and medians; focused Rust contract suite and docs/workflow gates pass |
| 2 — Prove route-specific reduction | RT-0sd.2; depends on RT-0sd.1 | Evidence PR with raw run links and reproducible calculation | At least ten successful pre-router and ten successful post-router PR runs for each route shape; report queue and run medians separately; both route shapes meet the planned 35% reduction or remain explicitly incomplete |
| 3 — Synchronize delivery records | RT-0sd closeout | Final evidence/doc PR | Baseline, closeout, execution plan, codemap, backlog, live Beads, and JSONL export agree; close only after Phase 2 evidence passes |

At the start of each milestone, inspect every open PR's current head, checks,
mergeability, and review threads through `rtk gh`. Fix findings on their attached
PR branch. Enable auto-merge only after required checks are terminal and
successful, review threads are resolved, and GitHub accepts the head as
mergeable. A neutral aggregate is evaluated together with its underlying jobs.

## Phase 1 — Collector contract

Request `conclusion` in the `gh run list` fields, filter to `success` before
computing queue/run samples, and report when no successful runs remain. Add a
hermetic behavioral regression with successful and failed completed runs and
assert the failed run cannot alter sample size or medians. Preserve the
existing workflow/event/branch filters and branch-protection output.

## Phase 2 — Route-specific performance proof

Collect pre-router and post-router cohorts separately for `passive-docs` and
`dependency-closure`. Identify each run by workflow run ID, PR, event,
conclusion, route decision, route shape, created/started/completed timestamps,
and the cohort used. Exclude failed, cancelled, rerun-ambiguous, missing-route,
and otherwise incomparable runs. Require ten valid successes in each of the
four cohorts. Report medians and ranges for both queue time and execution time;
compare execution medians using the same workflow/lane definition. The
post-router median must be at most 65% of the pre-router median for each route
shape. If available history cannot supply the cohorts, keep this milestone
open and continue collecting real successful PR runs; do not synthesize PRs or
present pooled totals as route-specific proof.

## Phase 3 — Delivery and closure

Synchronize the dated baseline, closeout notes, execution plan, backlog, root
atlas, live Beads, and `.beads/issues.jsonl` with exact source SHAs and
reproducible run links. The follow-up epic remains open until the four cohorts
and both reduction targets pass. Any proposed change to the target requires an
explicit contract decision and a separately reviewed plan update; it is not
implied by existing aggregate samples.
