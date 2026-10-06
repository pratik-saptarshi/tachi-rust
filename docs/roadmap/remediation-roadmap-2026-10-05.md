# RT-CI Route Remediation Roadmap — 2026-10-05

Status: **Phase 0 planning and Phase 1 routing are complete; Phase 2 onward and
matched-control timing remain open.** Phase 1 merged in PR #58 at
`c9460aa8550e4bfb064e7dcdfcc322b30f4032e8`. Source baseline: adversarial review of
`10339cc8f586fdf0050c01bd2d4889f7709906e6`.

> **Source note:** The original panel report file was not present in the
> repository or available temporary directories during this integration. The
> companion archive records the findings as summarized in the supplied panel
> plan; it is an attributed summary, not a verbatim reconstruction of the
> missing report. Finding confidence is therefore context-limited until the
> original is attached or recovered.

## Objective and decision contract

Remediate four P2 route-coverage gaps, retain two nonblocking P3 evidence and
tooling advisories, and collect valid matched timing controls for the two
optimized route shapes. The historical audit's zero eligible candidates is
bounded to its retained sample and does not establish whether the timing target
passed.

The open Beads issue `RT-0sd.2` remains the sole timing acceptance tracker. Its
method is updated to compare successful optimized PR runs against successful
forced-full `workflow_dispatch` controls on ten distinct, identical code trees
per route shape. For both `passive-docs` and `dependency-closure`, the optimized
execution median must be no more than 65% of the matched full-control execution
median. Queue and execution medians are reported separately. Insufficient pairs
or a missed threshold leave `RT-0sd.2` open and do not support a completion
claim.

## Delivery phases and PR boundaries

| Phase | Scope and tracker | Delivery boundary and exit gate |
|---|---|---|
| **0 — Baseline, evidence, tracker contract** | Create the fresh remediation epic and phase/finding issues; update `RT-0sd.2`; archive the panel summary; publish this roadmap and the integration-log record. | Completed in planning PR #57 (`777bdb5`), merged at `ddd74f0a97197771a8eb0b70f6a4ec9aff828d9f`. Open PR #56 holds the separate route cohort audit and is being synchronized to the merged base. |
| **1 — Path and documentation-contract routing** | P2-F01: preserve rename source and destination paths. P2-F04: treat `docs/testing/tdd-evidence.json` as active input to its owning contract. | Completed in PR #58 (`c71c08b`), merged at `c9460aa8550e4bfb064e7dcdfcc322b30f4032e8`. Both rename endpoints participate in route classification; TDD evidence changes execute their owner contract. Focused and all-targets Rust tests, workflow/docs gates, formatting, shell syntax, and terminal hosted checks passed. |
| **2 — Dependency closure and repository contracts** | P2-F02: include `tachi-mcp` in shell reverse-dependency tests. P2-F03: run compact repository-wide manifest/toolchain/policy contracts independently of package routing. | One implementation PR. Shell changes include all reverse Cargo dependencies, including MCP. Repository contract inputs trigger the compact contract job regardless of package route. |
| **3 — Matched timing controls** | Implement a read-only collector and matched-run contract under `RT-0sd.2`. P3-F05 and P3-F06 remain separately tracked and nonblocking. | One implementation/evidence PR. Collector handles success, failure, cancellation, reruns, absent route evidence, and mismatched tree SHA. Start the ten-tree cohorts only after Phases 1 and 2 and the collector are merged. |
| **4 — Integrated validation and closeout** | Reconcile the roadmap, panel archive, Beads records/export, integration log, and raw matched-run evidence. | Closeout PR only after the timing acceptance passes and all records agree. Report actual valid pair counts per shape. If either shape has fewer than ten valid pairs or misses the threshold, record the gap and keep `RT-0sd.2` open. |

### Commit and PR progression

Use a dedicated worktree and branch per milestone, based on a freshly verified
`origin/main`; preserve the current checkout and `.beads.gate.lock`. Make a
separate Conventional Commit for each milestone and open a reviewable PR at each
boundary above. Do not combine Phase 1 and Phase 2 implementation changes or
fold measurement closeout into their code PRs. Fix review comments on the branch
attached to that PR. Enable auto-merge only after required checks are terminal
and successful, review threads are resolved, and GitHub reports the PR
mergeable. Never bypass protection. If the remote cannot be refreshed, stop
before choosing or publishing a PR base and record that limit.

## Beads worklist

The new epic and all phase/finding issues use generated IDs. IDs below are
filled after creation and mirrored in `.beads/issues.jsonl`.

| Finding / phase | Priority and disposition | Refined acceptance and test plan | Beads |
|---|---|---|---|
| Phase 0 — planning/evidence contract | Delivery task | Roadmap, source-attributed panel archive, linked issue hierarchy, updated timing acceptance, export, and integration log are consistent. Verify IDs and dependencies against live Beads and validate JSONL records. | `RT-0vf.1` |
| P2-F01 — preserve both rename paths | P2; gap, Phase 1 | **RED:** In a temporary Git repo, rename a crate source/test file into passive docs and show the route input contains only the destination. **GREEN:** Feed both old and new paths to routing and select full or the applicable crate closure. **Regression:** crate-to-docs, cross-crate rename, delete/add, ordinary passive-doc change; unknown paths remain full. | `RT-0vf.2.1` |
| P2-F02 — include MCP in shell reverse dependencies | P2; gap, Phase 2 | **RED:** A `tachi-shell` change omits `tachi-mcp`. **GREEN:** Route closure contains every reverse Cargo dependency, including MCP. **Regression:** Compare maintained route mapping to Cargo metadata; verify multi-crate selection deduplicates; run MCP behavioral tests for a shell change. | `RT-0vf.3.1` |
| P2-F03 — route repository-wide contracts by their inputs | P2; coverage gap, Phase 2 | **RED:** An MCP manifest-only change selects MCP but skips `workflow_ci_gates`, which checks every crate manifest. **GREEN:** Compact repository-contract job runs for workspace manifests, toolchain, and contract-policy inputs independently of package routing. **Regression:** An MCP manifest violation fails the contract job; an unrelated crate change runs scoped tests plus the compact contract. | `RT-0vf.3.2` |
| P2-F04 — route test-owned TDD evidence as active input | P2; gap, Phase 1 | **RED:** A sole `docs/testing/tdd-evidence.json` change selects passive docs and skips its test. **GREEN:** The owning JSON contract runs for this path. **Regression:** Invalid JSON and missing required evidence fail; valid passive prose stays optimized; active/shared docs stay full. | `RT-0vf.2.2` |
| P3-F05 — define timing verifier route boundary | P3; nonblocking advisory | Document that the existing verifier validates all eight full-matrix artifacts and does not verify narrowed runs. If used for route-aware evidence, define route-derived artifact expectations. Preserve successful full-matrix verification and prove missing artifacts in a narrowed run cannot be reported as a pass. This advisory does not gate P2 delivery or timing acceptance unless needed by the agreed collector. | `RT-0vf.6` |
| P3-F06 — preserve route-audit replay inputs | P3; nonblocking evidence improvement | Save candidate run IDs, PR/head and tree SHAs, changed paths, route reasons, and exact classifier revision in a compact manifest. Replay reproduces the reported 37-candidate classification or explicitly records unavailable source artifacts. Do not generalize beyond retained candidates. This advisory does not gate P2 delivery or timing acceptance unless its evidence is required by the agreed acceptance. | `RT-0vf.7` |
| Phase 1 — path and documentation-contract routing | P2 delivery | Both P2-F01 and P2-F04 focused RED/GREEN tests and adjacent regressions pass; workflow/docs contracts pass; required hosted checks are terminal and successful. | `RT-0vf.2` |
| Phase 2 — dependency and repository contracts | P2 delivery | P2-F02 and P2-F03 focused RED/GREEN tests pass; Cargo metadata comparison and manifest-failure regression pass; relevant workspace tests, Clippy, workflow/docs gates, and required hosted checks pass. | `RT-0vf.3` |
| Phase 3 — matched controls | P2 timing acceptance | For each route shape, collect ten distinct PR code trees. Each tree has a successful routed PR run and successful forced-full `workflow_dispatch` control on identical tree SHA and comparable workflow revision/runner definition. Capture run ID, PR/head, event, attempt, route decision, created/started/completed timestamps, queue duration, and execution duration. Exclude failed, cancelled, rerun-ambiguous, missing-route, and mismatched-tree pairs. Report queue and execution medians separately; optimized execution median is <=65% of full-control median for each shape. Otherwise keep `RT-0sd.2` open. | `RT-0vf.4` and `RT-0sd.2` |
| Phase 4 — integrated closeout | Closeout | Roadmap, review archive, raw evidence, Beads export, and integration log reconcile. Actual cohort counts and calculation are reproducible from retained inputs. Do not close timing acceptance on fewer than ten pairs per route shape or a failed threshold. | `RT-0vf.5` |

### Timing acceptance update for `RT-0sd.2`

The existing [2026-10-05 timing plan](2026-10-05-rt-ci-timing-evidence-followup.md) and initial issue acceptance compare pre-router with post-router
cohorts. Replace that criterion with matched full controls as described above;
retain the issue, its history, and existing collector work. Require ten distinct
identical code trees per route shape, successful routed PR and forced-full
control runs on each tree, comparable workflow/runner definitions, explicit
exclusion rules, queue/execution separation, and a <=65% optimized execution
median. The 37-candidate historical audit remains sample-bounded context, not a
substitute cohort or a performance result.

`RT-0sd.2` depends on completion of the Phase 1 routing, Phase 2 repository
coverage, and Phase 3 matched-control collector issues. It does not depend on
the P3 advisory issues. The new epic is related to `RT-0sd.2`; it does not
replace or close it.

## Validation and release gates

For each P2 issue, run its current regression first (RED), make the focused fix
(GREEN), then run adjacent valid-input regressions. Phase validation includes
focused Rust test targets, workflow contract tests, formatting, workflow/docs
gates, workspace tests and Clippy as appropriate, and terminal required hosted
results. Keep environment/baseline failures distinct from regressions.

The forced-full control workflow must use read-only permissions and receive no
PR secrets. Exercise collector handling for success, failure, cancellation,
reruns, missing route data, and tree mismatch. Pairing must be deterministic
and preserve enough raw metadata to reproduce every exclusion and median.

## Panel finding traceability

| Panel finding | Disposition | Plan location | Tracker |
|---|---|---|---|
| P2-F01 rename endpoint loss | Bundle as required Phase 1 implementation | Beads worklist; Phase 1 | `RT-0vf.2.1` |
| P2-F02 omitted MCP dependency closure | Bundle as required Phase 2 implementation | Beads worklist; Phase 2 | `RT-0vf.3.1` |
| P2-F03 missing repository-wide manifest contracts | Bundle as required Phase 2 implementation | Beads worklist; Phase 2 | `RT-0vf.3.2` |
| P2-F04 test-owned TDD evidence treated as passive | Bundle as required Phase 1 implementation | Beads worklist; Phase 1 | `RT-0vf.2.2` |
| P3-F05 verifier route scope | Defer as nonblocking advisory with explicit decision/test contract | P3 advisory issue | `RT-0vf.6` |
| P3-F06 replayable audit inputs | Defer as nonblocking evidence improvement | P3 advisory issue | `RT-0vf.7` |
| Zero eligible historical candidates | Informational, bounded to retained sample; not a defect or target result | Objective, timing contract, Phase 3 | Existing `RT-0sd.2` |

## Integration summary

Total findings: **7** (six panel findings plus the bounded historical-evidence
claim) | Must-fix: **0 P0/P1** | Bundle: **4 P2** | Defer: **2 P3** | Info:
**1 bounded evidence statement**.

Classification follows `plan-review-integrator`: all six findings have
actionability 1.0 and match the supplied structured summary, but context
coverage is **partial** because the original panel report was unavailable.
The user-provided, decision-complete plan resolves the dispositions and timing
method; preserve the context caveat in the report archive and integration log.
No disputed P0/P1 finding or governance escalation was present.

**Final Recommendation:** Applied with caveats.

**Dissent Ledger:** none. No panel-level disagreement was included in the
available summary.

### Action items

| Priority | Owner | Action | Source |
|---|---|---|---|
| P2 | Implementer | Deliver Phase 1 rename-path and TDD evidence routing with RED/GREEN regressions. | P2-F01, P2-F04 |
| P2 | Implementer | Deliver Phase 2 MCP reverse-dependency and repository-contract routing. | P2-F02, P2-F03 |
| P2 | Implementer | Deliver read-only matched-control collector and gather ten valid pairs per route shape. | `RT-0sd.2` |
| P3 | Implementer/reviewer | Preserve verifier boundary and replay inputs as separately tracked advisory work. | P3-F05, P3-F06 |
| P2 | Reviewer | Recheck remote PR state and terminal protections at every milestone. | Phase 0–4 |

## Current verification and limitations

- Live Beads retains `RT-0sd.2` open with the matched-control acceptance;
  Phase 1 issue `RT-0vf.2` and children `.2.1`/`.2.2` closed after PR #58 merged.
  Phase 2 `RT-0vf.3` is next. The issue JSONL export is regenerated during the
  current tracker synchronization.
- PR #57 delivered the planning baseline; PR #58 delivered Phase 1. GitHub
  checks were inspected through `rtk gh` at the Phase 1/2 boundary. PR #56's
  checks passed, but its branch was behind after Phase 1 and was synchronized
  by merging refreshed `origin/main` into its attached worktree. Its auto-merge
  request remains protected by normal review and branch rules.
- The worktree contains the unrelated pre-existing untracked
  `.beads.gate.lock`; preserve it.
- At planning time, the referenced cohort-audit report was absent from the
  current checkout. Open PR #56 now carries
  [`docs/reports/rt-ci-route-cohort-audit-2026-10-05.md`](../reports/rt-ci-route-cohort-audit-2026-10-05.md).
  It records 37 reconstructed candidates, zero eligible historical candidates
  for either route, and no route-specific timing medians. This supports only the
  sample-bounded historical observation; it is not the original adversarial
  panel report and does not replace matched controls.
- Phase 1 is complete. P2-F02/P2-F03 and matched-control timing are not yet
  implemented or accepted. The historical zero-candidate result remains
  bounded to the retained sample and is not a timing result.
