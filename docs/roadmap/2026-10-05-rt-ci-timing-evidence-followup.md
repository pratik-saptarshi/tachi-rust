# RT-CI Route-specific Timing Evidence Follow-up — 2026-10-05

Status: **Phases 0, 1, and 2a complete; Phase 2b timing acceptance remains open**.
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
| 0 — Reconcile the existing review and tracker state | RT-0sd | Planning/update commits on PR #26's attached branch | PR #26 head and hosted jobs inspected; review findings fixed on that branch; Beads records and export reconciled |
| 1 — Make timing collection success-only and reproducible | RT-0sd.1 — complete in PR #26 | PR #26, merged as `62dbed10` | Mixed success/failure behavioral regression proves failed completions do not affect sample size or medians; focused Rust contract suite and hosted required checks pass |
| 2a — Make the planned route shapes reachable | RT-0sd.3 — complete | PR #54, merged as `9d5b2733420bf5511a12c9a04cb36172937cd864` | Main-target PRs can select passive-docs and dependency-closure modes; direct protected-ref events and all unsafe/unknown routes remain full; behavioral route tests and hosted required checks pass |
| 2b — Prove route-specific reduction | RT-0sd.2 — open; prerequisite RT-0sd.3 complete | Evidence PR with raw run links and reproducible calculation | At least ten successful pre-router and ten successful post-router PR runs for each route shape; report queue and run medians separately; both route shapes meet the planned 35% reduction; otherwise leave open |
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
Valid pre-router sample counts are therefore zero for `passive-docs` and
`dependency-closure`. At the time of the audit, post-router narrowed-mode
counts were also zero because the gate routed all PRs to full mode. PR #54
corrected and merged the classifier; new route-specific samples can now be
collected. This does not relax the original ten-run cohorts or 35% reduction
threshold.

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

## Phase 3 — Delivery and closure

Synchronize the dated baseline, closeout notes, execution plan, backlog, root
atlas, live Beads, and `.beads/issues.jsonl` with exact source SHAs and
reproducible run links. The follow-up epic remains open until the four cohorts
and both reduction targets pass. Any proposed change to the target requires an
explicit contract decision and a separately reviewed plan update; it is not
implied by existing aggregate samples.
