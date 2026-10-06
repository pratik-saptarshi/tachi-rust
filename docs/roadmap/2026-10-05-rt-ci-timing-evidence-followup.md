# RT-CI Route-specific Timing Evidence Follow-up — 2026-10-05

Status: **Phases 0, 1, and 2a complete; matched-control timing acceptance remains open**.
This follow-up addresses the unresolved post-merge timing review on PR #26
(comment `4177119281`). The original RT-CI epic and `RT-CI-006.2` remain closed
for their recorded implementation and aggregate-sample scope; this plan tracks
the stricter route-specific comparison required by the execution plan.

## Evidence gap

The dated PR sample records 22 workspace runs and 23 route-observe runs, but
does not establish the optimized-to-full-control execution median for either
`passive-docs` or `dependency-closure`. It therefore does not establish the
required <=65% ratio for either shape. The existing latency collector requests
completed runs without requesting or filtering `conclusion`, so failed runs can
enter its sample. These aggregate numbers remain historical context, not proof
that the route-specific acceptance target passed.

## Milestones and Beads

| Milestone | Beads | Delivery boundary | Exit validation |
|---|---|---|---|
| 0 — Reconcile the existing review and tracker state | RT-0sd | Planning/update commits on PR #26's attached branch | PR #26 head and hosted jobs inspected; review findings fixed on that branch; Beads records and export reconciled |
| 1 — Make timing collection success-only and reproducible | RT-0sd.1 — complete in PR #26 | PR #26, merged as `62dbed10` | Mixed success/failure behavioral regression proves failed completions do not affect sample size or medians; focused Rust contract suite and hosted required checks pass |
| 2a — Make the planned route shapes reachable | RT-0sd.3 — complete | PR #54, merged as `9d5b2733420bf5511a12c9a04cb36172937cd864` | Main-target PRs can select passive-docs and dependency-closure modes; direct protected-ref events and all unsafe/unknown routes remain full; behavioral route tests and hosted required checks pass |
| 2b — Prove route-specific reduction with matched controls | RT-0sd.2 — open; prerequisite RT-0sd.3 complete | Evidence PR with matched run IDs and reproducible calculation | For each route shape, ten distinct identical code trees each have a successful routed PR run and successful forced-full `workflow_dispatch` control on the same tree SHA and comparable workflow/runner definition. Report queue and execution medians separately; optimized execution median is at most 65% of the full-control median for both shapes; otherwise leave open |
| 3 — Synchronize delivery records | RT-0sd closeout | Final evidence/doc PR | Baseline, closeout, execution plan, codemap, backlog, live Beads, and JSONL export agree; close only after Phase 2b evidence passes |

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

### Route-eligibility audit (2026-10-05)

The required cohorts cannot be collected from the current route implementation.
Both `.github/workflows/rust-workspace.yml` and
`.github/workflows/ci-route-observe.yml` are triggered for PRs targeting
`main`, then pass `github.base_ref` (`main`) to `is_protected_ref` before
classifying changed paths. This forces every main-target PR into full mode.
The same active-contract regular expression also matches every path under
`crates/tachi-*` before the dependency-closure branch, making that branch
unreachable for crate changes. The Rust workspace route output recorded in run
`29091065263` and the observe-only route artifact from PR run `29210900411`
confirm `reason=protected ref stays full mode`.

The Rust workspace workflow began on 2026-06-23. Across the available history,
the only successful PR run before the route rollout commit
`52a0f8e4d603b5bef1f5d058d1db69f3ce56e702` (2026-07-09 20:07 CDT) is run
`28757002103` for PR #14. That PR mixes workflow, manifest, active-document,
and Rust crate changes, so it is not comparable to either target route shape.
The retained history therefore contains zero eligible narrowed-route
candidates for `passive-docs` and `dependency-closure`; these bounded counts do
not establish whether a timing target passed. The acceptance was revised to
matched controls because a historical pre-router cohort cannot be populated
from future runs. For each optimized route shape, pair a successful routed PR
run with a successful forced-full `workflow_dispatch` control on the same
distinct code tree, using comparable workflow revision and runner definition.
The identical-tree controls provide a collectable full-matrix baseline without
synthetic PRs. PR #54 corrected and merged the classifier; `RT-0sd.2` remains
open until each shape has ten valid pairs and the optimized execution median
is at most 65% of its full-control median.

### Phase 1 receipt

The success-only collector fix landed on PR #26 head `562666a5e0e025050a55d757ec723285fbea0085`
and merged as `62dbed10a4389215a0e4f01aa08c1df68bdeb857`. The focused
`workflow_ci_gates` suite passed 29 tests; hosted Rust workspace, supply-chain,
Clippy, CodeQL, Gitleaks, rustfmt, workflow parsing, catalog drift, init matrix,
route artifact, and PostgreSQL migration/RLS checks all completed successfully.
PR #26's four remaining review threads were replied to and resolved.

### Phase 2a — Make the planned route shapes reachable

Route classification must use the event's execution ref for protected-ref
enforcement, not the PR's protected base branch. Remove crate directories from
the active-contract matcher so crate-local changes reach dependency-closure
selection. Keep a single shared classifier for the enforced Rust workflow and
the observe-only artifact, with behavioral regressions for passive docs,
crate closures, active docs, shared paths, unknown paths, protected push refs,
and the emergency override. Keep branch protection in force; GitHub reports a
conditionally skipped required job as successful, so skipped package matrices
remain compatible with the required-check contract.

Collect matched pairs separately for `passive-docs` and `dependency-closure`.
Each pair consists of a successful routed PR run and a successful forced-full
`workflow_dispatch` control on the same exact tree SHA, with comparable
workflow revision and runner definition. Use ten distinct PR code trees per
shape. Identify each run by workflow run ID, PR/head, event, attempt, route
decision, tree SHA, workflow/runner definition, and created/started/completed
timestamps. Exclude failed, cancelled, rerun-ambiguous, missing-route,
mismatched-tree, and otherwise incomparable pairs. Report queue and execution
medians separately. The optimized execution median must be at most 65% of the
matched full-control execution median for both shapes. Keep `RT-0sd.2` open if
either shape has fewer than ten valid pairs or misses the threshold; do not
present the bounded historical candidate sample as timing evidence.

### Phase 2a local validation receipt (2026-10-05)

- `ci_route_classifier`: 3/3 behavioral tests passed; `workflow_ci_gates`:
  29/29 passed, including the corrected distinction between direct protected
  refs and PRs targeting `main`.
- `act_smoke_run_contract`: 9/9 passed after refreshing the trusted SHA-256
  for the edited `ci-route-observe.yml` workflow.
- `cargo test --locked --workspace --all-targets -q` passed, including the
  loopback redirect-exhaustion regression; `cargo clippy --locked --workspace
  --all-targets -- -D warnings` passed.
- `cargo fmt --all -- --check`, `bash -n scripts/ci-route-classifier.sh`,
  `make workflow-gate docs-version-gate docs-archive-version-gate`, and
  `git diff --check` passed.
- PR #54 merged at `2026-10-05T06:44:42Z` as
  `9d5b2733420bf5511a12c9a04cb36172937cd864` from head
  `86638be79a36429bb5c179cf33715152fc2c65d8`. All 17 required branch
  protection contexts passed; every reported check was terminal (23 success,
  one neutral advisory Clippy status). No formal review or inline comments
  remained. `RT-0sd.3` is closed. These results do not satisfy the four live
  timing cohorts or close `RT-0sd.2`.

### Supplemental historical cohort audit (2026-10-05)

The expanded history scan paired successful route-observe and workspace PR
runs by exact final PR head SHA from the July rollout through the start of
PR #54's classifier correction. It found 37 unique candidates with both
successful workflow runs and a downloadable route artifact containing
`changed_paths`. Reclassifying those paths with the corrected shared
classifier produced `full_pr_matrix` for all 37: 33 touched active/shared
documentation and 4 included unknown non-documentation paths. The scan
therefore confirms no eligible narrowed-route candidates in the retained
historical sample. The latest
post-correction PR run available, PR #55, also selected full mode because it
touched active/shared documentation; it is not a matched timing pair.
The complete run links, SHAs, timestamps, and reconstructed reasons are
recorded in [`the route cohort audit`](../reports/rt-ci-route-cohort-audit-2026-10-05.md).
The historical sample is not generalized to future matched controls. `RT-0sd.2`
remains open under the updated per-shape ten-pair, <=65% acceptance.

## Phase 3 — Delivery and closure

Synchronize the dated baseline, closeout notes, execution plan, backlog, root
atlas, live Beads, and `.beads/issues.jsonl` with exact source SHAs and
reproducible matched-run links. The follow-up epic remains open until both
route shapes have ten valid pairs and meet the execution-median threshold. Any
further target change requires an explicit contract decision and a separately
reviewed plan update; it is not implied by existing aggregate samples.
