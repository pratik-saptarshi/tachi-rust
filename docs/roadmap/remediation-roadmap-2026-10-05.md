# RT-CI Route Remediation Roadmap — 2026-10-05

Status: **Phases 0–3 and both P3 advisories are delivered; post-merge review
corrections merged in PR #62 (`d81f9962`), P3-F05 closed in PR #64, P3-F06
closed in PR #65. The 2026-10-06 cohort has ten identity-matched pairs per
shape, but the collector's run-level "execution" includes downstream matrix
job queueing. Neither shape has an accepted timing result until queue and
execution are separated. Phase 4 remains incomplete, and `RT-0sd.2` stays
open.** Phase 1
merged in PR #58 at
`c9460aa8550e4bfb064e7dcdfcc322b30f4032e8`; Phase 2 merged in PR #59 at
`a8f4930caeede037ffdd58b32a1f160f481cd461`. Source baseline: adversarial review
of `10339cc8f586fdf0050c01bd2d4889f7709906e6`.

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
forced-full `pull_request` control runs initiated with the `ci-full-control`
label on ten distinct, identical code trees per route shape. For both
`passive-docs` and `dependency-closure`, the optimized
execution median must be no more than 65% of the matched full-control execution
median. Queue and execution medians are reported separately. Insufficient pairs
or a missed threshold leave `RT-0sd.2` open and do not support a completion
claim.

## Delivery phases and PR boundaries

| Phase | Scope and tracker | Delivery boundary and exit gate |
|---|---|---|
| **0 — Baseline, evidence, tracker contract** | Create the fresh remediation epic and phase/finding issues; update `RT-0sd.2`; archive the panel summary; publish this roadmap and the integration-log record. | Completed in planning PR #57 (`777bdb5`), merged at `ddd74f0a97197771a8eb0b70f6a4ec9aff828d9f`. Open PR #56 holds the separate route cohort audit and is being synchronized to the merged base. |
| **1 — Path and documentation-contract routing** | P2-F01: preserve rename source and destination paths. P2-F04: treat `docs/testing/tdd-evidence.json` as active input to its owning contract. | Completed in PR #58 (`c71c08b`), merged at `c9460aa8550e4bfb064e7dcdfcc322b30f4032e8`. Both rename endpoints participate in route classification; TDD evidence changes execute their owner contract. Focused and all-targets Rust tests, workflow/docs gates, formatting, shell syntax, and terminal hosted checks passed. |
| **2 — Dependency closure and repository contracts** | P2-F02: include `tachi-mcp` in shell reverse-dependency tests. P2-F03: run compact repository-wide manifest/toolchain/policy contracts independently of package routing. | Completed in PR #59, merged at `a8f4930c`. Shell changes include all reverse Cargo dependencies, including MCP. Repository contract inputs trigger the compact contract job regardless of package route. |
| **3 — Matched timing controls** | Implement a read-only collector and matched-run contract under `RT-0sd.2`. P3-F05 verifier-boundary and P3-F06 replay advisories are nonblocking and delivered in PRs #64 and #65. | Initial collector merged in PR #60. PR #61 merged the immutable execution SHA, matched commit, PR/head provenance, and cache-scope protections. Post-merge corrections merged in PR #62 (`d81f9962`): trusted provenance is separate and uploaded before PR scripts; artifacts with multiple JSON objects are rejected; label controls have isolated concurrency; ignored labels cannot satisfy the stable required check. PR #65 closes P3-F06. The 2026-10-06 cohort has ten identity-matched pairs per shape, but run-level duration includes downstream job queueing; timing acceptance remains unproven pending per-job queue/execution separation. |
| **4 — Integrated validation and closeout** | Reconcile the roadmap, panel archive, Beads records/export, integration log, and raw matched-run evidence. | Closeout PR only after the timing acceptance passes and all records agree. Report actual valid pair counts per shape. If either shape has fewer than ten valid pairs or misses the threshold, record the gap and keep `RT-0sd.2` open. |
| **5 — Timing metric correction and follow-up** | Correct the collector's timing split tracked by `RT-0sd.4`; then determine whether dependency-closure needs cost remediation without reducing selected package or repository-contract coverage. | Separate implementation PR. Capture each measured job's creation, start, and completion timestamps; report workflow queue, per-job queue, and execution separately with a deterministic aggregation. Reprocess the existing cohort only if complete source timestamps are recoverable; otherwise collect a fresh matched cohort. Optimize dependency-closure only if the corrected execution metric misses the existing <=65% threshold. |

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

The two nonblocking P3 advisories are separate review milestones after the
collector: `RT-0vf.6` delivers the verifier-boundary decision and narrowed-run
regression; `RT-0vf.7` delivers the route-audit manifest and replay contract.
Each milestone checks open PRs and CI at its start and gets its own PR. The
existing Phase 4 closeout remains a separate final PR and stays blocked by
`RT-0sd.2` until its matched-cohort acceptance passes. The observed
dependency-closure miss has a separate follow-up milestone, `RT-0sd.4`; do not
lower the threshold or count the current cohort as a Phase 4 pass.

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
| P3-F06 — preserve route-audit replay inputs | P3; nonblocking, delivered | PR #65 preserves all 37 candidates and the classifier snapshot; a reviewed manifest digest rejects tampered route inputs and provenance. Replay result remains bounded to the retained sample. | `RT-0vf.7` closed |
| Phase 1 — path and documentation-contract routing | P2 delivery | Both P2-F01 and P2-F04 focused RED/GREEN tests and adjacent regressions pass; workflow/docs contracts pass; required hosted checks are terminal and successful. | `RT-0vf.2` |
| Phase 2 — dependency and repository contracts | P2 delivery | P2-F02 and P2-F03 focused RED/GREEN tests pass; Cargo metadata comparison and manifest-failure regression pass; relevant workspace tests, Clippy, workflow/docs gates, and required hosted checks pass. | `RT-0vf.3` |
| Phase 3 — matched controls | P2 timing acceptance | For each route shape, collect ten distinct PR code trees. Pair each successful optimized PR run with a successful full-route `pull_request` run triggered by adding `ci-full-control` to that same PR, with identical execution commit SHA and Git tree. Both runs use the immutable merge SHA supplied by the event; every measured job checks out that SHA. Require identical workflow-file blob content and route classifier/path-producer revisions; compare the route runner and every measured job's OS, image version, and architecture. Capture run ID, PR/head SHA, event, attempt, route decision, execution SHA/tree, created/started/completed timestamps, queue duration, and execution duration. Exclude failed, cancelled, rerun-ambiguous, missing-provenance, and mismatched-commit/tree/workflow/runner pairs. Report queue and execution medians separately; optimized execution median is <=65% of full-control median for each shape. Otherwise keep `RT-0sd.2` open. | `RT-0vf.4` and `RT-0sd.2` |
| Phase 4 — integrated closeout | Closeout | Roadmap, review archive, raw evidence, Beads export, and integration log reconcile. Actual cohort counts and calculation are reproducible from retained inputs. Do not close timing acceptance on fewer than ten pairs per route shape or a failed threshold. | `RT-0vf.5` |
| Phase 5 — timing metric correction and follow-up | P1; timing evidence integrity | **RED:** The current collector sets queue to workflow `createdAt`→`startedAt` and execution to workflow `startedAt`→`updatedAt`; this places matrix-job runner waits inside execution. The reported 193.6% dependency-closure ratio is workflow wall-time only and cannot establish an execution threshold miss. **GREEN:** Capture measured-job `created_at`, `started_at`, and `completed_at`; report workflow queue, per-job queue, and per-job execution separately under a documented deterministic aggregation. Reprocess current runs only if all required source timestamps can be recovered; otherwise collect ten fresh distinct successful matched trees per shape on comparable workflow/runner definitions. **Regression:** Fixtures prove post-start job wait affects queue but not execution, while job runtime affects execution; route closure and repository-contract triggering remain unchanged; invalid/incomplete job timing is excluded. Only then compare optimized execution median to the unchanged <=65% threshold; optimize dependency-closure if it still misses. | `RT-0sd.4` |

### Matched-control result — 2026-10-06

The final collector snapshot is
[`rt-ci-matched-control-final-2026-10-06.json`](../reports/rt-ci-matched-control-final-2026-10-06.json)
(SHA-256 `dbe6a1b53203ea495660293f1106c91b3d932b1557109c89b24d54c808bad132`;
source collector SHA-256 `2a98c3cf359fc94f06a9b4460832cf29cdf266297fbc7e72f3a1283ca22a3443`);
the run mapping and limitations are in the
[matched-control readout](../reports/rt-ci-matched-control-final-2026-10-06.md).
It contains 87 candidate workflow runs and 20 valid pairs: ten `passive-docs`
and ten `dependency-closure`, all on distinct PR trees with matching execution
SHA/tree, workflow revision, runner definition, and successful attempts.

| Route shape | Optimized workflow wall median | Full-control workflow wall median | Ratio | Interpretation |
|---|---:|---:|---:|---|
| `passive-docs` | 16,500 ms | 122,000 ms | 13.5% | Informational only |
| `dependency-closure` | 257,500 ms | 133,000 ms | 193.6% | Informational only |

Workflow queue medians are 0 ms at the one-second source timestamp resolution.
They omit downstream job queueing. For example, the PR #80 control workflow
started at 10:48:36Z, while measured jobs started between 10:49:26Z and
10:52:36Z. These run-level ratios therefore do not establish either a pass or
a threshold miss. Keep `RT-0sd.2` and `RT-0vf.5` open and Phase 4 incomplete.
`RT-0sd.4` tracks corrected per-job queue/execution instrumentation and
re-evaluation; optimize only if the accepted execution metric still misses.

### Timing acceptance update for `RT-0sd.2`

The existing [2026-10-05 timing plan](2026-10-05-rt-ci-timing-evidence-followup.md) and initial issue acceptance compare pre-router with post-router
cohorts. Replace that criterion with matched full controls as described above;
retain the issue, its history, and existing collector work. Require ten distinct
identical code trees per route shape, successful optimized PR and label-triggered
full-route PR runs on each identical execution commit SHA and tree, captured
controlled PR number/head SHA, identical workflow-file content and route
classifier/path-producer revisions, comparable runner images for the route job
and every measured PR job, explicit exclusion rules, queue/execution separation,
and a <=65% optimized execution median. The 37-candidate historical audit remains
sample-bounded context, not a substitute cohort or a performance result.

`RT-0sd.2` depends on completion of the Phase 1 routing, Phase 2 repository
coverage, Phase 3 matched-control collector issues, and the Phase 5
dependency-closure timing remediation in `RT-0sd.4` before a passing replacement
cohort can satisfy closeout. It does not depend on the P3 advisory issues. The
new epic is related to `RT-0sd.2`; it does not replace or close it.

The matched-control collector is invoked with `RT_CI_USE_RTK=true make rt-ci-matched-controls`.
It reads completed `rust-workspace.yml` runs and their route and per-job runner
artifacts. Each run records the commit and Git tree actually checked out. To
create a control, add the `ci-full-control` label to the PR. That label event
forces the `pull_request` workflow through `full_pr_matrix` while keeping code
execution in the PR cache scope. Remove the label after the control finishes to
return later events to optimized routing. The event's immutable `github.sha`
is the merge commit used by all measured jobs; the route artifact records the
PR number and head SHA from the event payload without a token-bearing API call.
The collector requires optimized and control execution SHAs, Git trees, PR
numbers, and PR head SHAs to match. It compares the workflow-file blob ID and
route script blob IDs, then verifies that every measured PR job has the same
runner OS, image version, and architecture in the full control. It emits matched pairs, excluded
candidates, run provenance, and separate queue/execution medians, ratios, and
per-shape acceptance state. Timing artifacts use the route artifact's verified
execution SHA and controlled PR head; the verifier binds full-control evidence
to that route artifact. Control jobs load the Rust setup action from
`github.workflow_sha`, disable dependency caching, and do not persist checkout
credentials while executing PR code. The workflow grants only `contents: read`
and consumes no PR secrets or PR API token. Keep the JSON output with
the closeout evidence.
`insufficient_pairs` is an expected open state and leaves `RT-0sd.2` open.

### P3-F05 verifier boundary

`scripts/verify-ci-timing-artifacts.sh` is a full-matrix timing verifier. It
requires all eight package and shell timing artifacts, in addition to trusted
route and provenance artifacts, for every accepted run. A narrowed route that
does not produce the complete matrix therefore fails closed; the script does
not derive expected artifacts from the selected route and must not be used to
verify narrowed-run completeness. A future route-aware verifier must define
and test its expected artifact set from the trusted route decision before it
can report a narrowed run as verified. This P3 advisory is nonblocking for the
P2 routing work and matched-control acceptance.

### P3-F06 replayable route-audit inputs

[`rt-ci-route-cohort-audit-replay-manifest-2026-10-05.json`](../reports/rt-ci-route-cohort-audit-replay-manifest-2026-10-05.json)
preserves all 37 retained candidates with route/workspace run IDs, PR number,
PR head and head-tree SHAs, changed paths, reconstructed route mode/reason,
route artifact digest, and the classifier source commit/blob. Its tree field is
the Git tree of the PR head commit; it does not claim the historical workspace
job checked out that tree instead of GitHub's synthetic merge commit. The
classifier snapshot is stored with the manifest so replay works without a
network fetch or old Git history. `scripts/replay-rt-ci-route-cohort-audit.sh`
first verifies the manifest against a reviewed SHA-256 digest pinned in the
script, then verifies the classifier snapshot hash and replays each candidate
offline. This rejects altered paths, route results, run/head identities, or
classifier provenance as changes to the retained audit input. The regression
tamper cases cover route reason, changed paths, head identity, and classifier
provenance. The replay reproduces 37 `full_pr_matrix` classifications: 33
active/shared inputs and 4 unknown non-documentation paths. The result remains
bounded to those retained candidates and is not a timing result.

Post-merge review of PR #61 identified two additional integrity and workflow
isolation requirements, tracked in follow-up PR #62. The workflow uploads
trusted route provenance as a separate artifact before running any PR-controlled
route scripts; it does not merge trusted fields into the script-generated
`route.json` afterward. The collector and verifier download both artifacts and
merge trusted provenance over route-decision fields outside PR code execution.
Both collector and verifier require exactly one JSON object from each artifact
before merging; multiple top-level documents are rejected.
Label-triggered events use separate concurrency namespaces for full controls,
ignored labels, and ordinary CI. Only the `ci-full-control` label runs the route
job, and `unlabeled` is not a workflow trigger. Ignored-label runs also use a
distinct, non-required check name so a skipped job cannot report success for the
stable required context. These corrections do not change the matched timing
acceptance; keep `RT-0sd.2` open until its cohort gates pass.

PR #62 merged as `d81f99629b77c6873be168a65ed052aedc9fcc4b` after 25 hosted
checks passed, no checks failed, and the single skipped check remained
nonblocking. Both PR #62 review threads were resolved. Earlier inventories had
zero eligible pairs. The final 2026-10-06 sample has ten identity-matched pairs
per shape, but its workflow wall-time intervals include post-start matrix-job
queueing. The resulting ratios are informational, not accepted pass/miss
outcomes; `RT-0sd.2` remains open pending corrected metrics.

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

For P3-F05, retain the successful full-matrix verifier fixture and add a
negative fixture in which a dependency-closure run publishes only its selected
package artifacts. The fixed eight-artifact verifier must reject that run. This
proves its full-matrix boundary; it does not claim route-aware verification.

For P3-F06, replay all 37 saved path sets with the pinned classifier snapshot;
fail if the classifier hash, candidate data, mode, reason, or changed paths do
not match. Verify the output states that the result is bounded historical
eligibility evidence and does not represent matched timing performance.

## Panel finding traceability

| Panel finding | Disposition | Plan location | Tracker |
|---|---|---|---|
| P2-F01 rename endpoint loss | Bundle as required Phase 1 implementation | Beads worklist; Phase 1 | `RT-0vf.2.1` |
| P2-F02 omitted MCP dependency closure | Bundle as required Phase 2 implementation | Beads worklist; Phase 2 | `RT-0vf.3.1` |
| P2-F03 missing repository-wide manifest contracts | Bundle as required Phase 2 implementation | Beads worklist; Phase 2 | `RT-0vf.3.2` |
| P2-F04 test-owned TDD evidence treated as passive | Bundle as required Phase 1 implementation | Beads worklist; Phase 1 | `RT-0vf.2.2` |
| P3-F05 verifier route scope | Implemented as full-matrix-only boundary with a narrowed-artifact rejection regression; nonblocking advisory merged in PR #64 | P3 advisory section and contract test | `RT-0vf.6` closed |
| P3-F06 replayable audit inputs | Delivered as a nonblocking manifest and offline replay in PR #65; keep the 37-candidate result sample-bounded | P3 advisory section, replay manifest, classifier snapshot, and replay contract | `RT-0vf.7` closed |
| Zero eligible historical candidates | Informational, bounded to retained sample; not a defect or target result | Objective, timing contract, Phase 3 | Existing `RT-0sd.2` |
| PR #60 Codex comment 4192299099: mutable merge ref re-resolved by measured jobs | Fix in protected follow-up; downstream jobs use verified immutable SHA from route output | Phase 3 timing contract and review closeout | `RT-0vf.4`, `RT-0sd.2` |
| PR #60 Codex comment 4192299102: same tree could pair different execution commits | Fix matcher and regression fixture; require exact `execution_sha` equality | Phase 3 timing contract and review closeout | `RT-0vf.4`, `RT-0sd.2` |
| PR #60 Codex comment 4192299107: control drops controlled PR provenance | Capture controlled PR head at dispatch and retain PR number/head/commit in candidates and pairs | Phase 3 timing contract and review closeout | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 CodeQL alert 28: workflow_dispatch can execute untrusted code in default-branch cache scope | Run controls as label-triggered `pull_request` events, remove cache actions from the workflow, and avoid token-bearing API calls | Phase 3 control security contract | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 Codex comment 4192392232: control timing artifacts record dispatch SHA | Write the event's route-verified execution SHA and controlled PR head into timing artifacts; bind full-control evidence to the route artifact | Phase 3 provenance and verifier tests | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 Codex comment 4192479376: token-bearing provenance ran after PR-controlled route scripts | Remove the PR API token entirely; derive PR/head provenance from the pull_request event payload before running route scripts | Phase 3 token-isolation ordering contract | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 Copilot comment 4192629142: post-script provenance merge can be poisoned through PR-controlled shell environment changes | Upload trusted provenance before route scripts as a separate artifact; merge it over route-decision fields only in the trusted collector and verifier | Phase 3 artifact trust boundary and collector/verifier regressions | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 Copilot comment 4192629148: label events can cancel ordinary required CI and unrelated labels can start route work | Remove the `unlabeled` trigger, skip the route job for labels other than `ci-full-control`, and isolate ordinary, control, and ignored-label concurrency groups | Phase 3 trigger/concurrency workflow contract | `RT-0vf.4`, `RT-0sd.2` |
| PR #62 Codex comment 4192736938: skipped ignored-label job reports success for the stable required check | Give ignored-label events a distinct non-required check name, while preserving the required name for ordinary and full-control runs; retain the route-job skip and concurrency isolation | Phase 3 trigger/status-check contract | `RT-0vf.4`, `RT-0sd.2` |
| PR #62 Codex comment 4192802658: multiple route JSON documents can make `jq -s` merge a forged document instead of trusted provenance | Require exactly two slurped JSON objects before merging; reject malformed or multi-document route/provenance artifacts | Phase 3 collector and verifier artifact validation tests | `RT-0vf.4`, `RT-0sd.2` |
| PR #90 Codex comment 4194686655: workflow-level execution interval includes downstream matrix job queueing | Mark current workflow wall-time ratios informational; collect per-job creation/start/completion timestamps and calculate queue/execution separately before evaluating or optimizing dependency-closure | Matched-control readout and Phase 5 acceptance | `RT-0sd.2`, `RT-0sd.4`, `RT-0vf.5` |

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
| P3 | Implementer/reviewer | Preserve and replay the route-audit inputs under the exact classifier revision; keep conclusions bounded to retained candidates. | P3-F06 |
| P2 | Reviewer | Recheck remote PR state and terminal protections at every milestone. | Phase 0–4 |

## Current verification and limitations

- Live Beads retains `RT-0sd.2` open with the matched-control acceptance;
  Phase 1 (`RT-0vf.2` and children `.2.1`/`.2.2`) closed after PR #58 and Phase 2
  (`RT-0vf.3` and children `.3.1`/`.3.2`) closed after PR #59. Phase 3
  collector issue `RT-0vf.4` closed after PR #62 merged and its hosted checks
  passed. This closes implementation delivery only: the refreshed read-only
  earlier collector baselines had zero eligible pairs. The final 2026-10-06
  cohort has ten identity-matched pairs per shape but workflow wall time still
  includes internal matrix-job queueing; `RT-0sd.2` remains open pending
  corrected execution metrics. P3-F05 `RT-0vf.6` closed after PR #64 merged at
  `430ef76b`; its focused verifier tests and hosted workflows passed, and its
  Codex review thread was resolved. P3-F06 `RT-0vf.7` closed after PR #65
  merged at `2d3ff33d`; its replay digest fix and tamper tests passed, all nine
  hosted workflows succeeded on final head `b2d70167`, and all review threads
  were resolved.
- PR #57 delivered the planning baseline; PR #58 delivered Phase 1; PR #59
  delivered Phase 2. GitHub
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
- Phase 2 local verification passed: Cargo metadata reverse-dependency
  comparison, route deduplication, compact contract selection for manifests,
  toolchain and source changes, passive-doc exclusion, active route/workflow
  contract tests, all 459 `tachi-core` all-target tests (1 ignored), all 21
  `tachi-mcp` tests, Clippy, formatting, shell syntax, workflow/docs gates and
  diff checks. PR #59's hosted core, MCP, CLI, shell, desktop and shell-slice
  tests, repository-wide contracts, CodeQL Rust, workflow parse, rustfmt,
  supply-chain, route-observe and Gitleaks checks succeeded; its protected
  squash auto-merge completed at `a8f4930c`. `RT-0sd.2` remains open. The
  historical zero-candidate result remains bounded to the retained sample and
  is not a timing result.
- Phase 3 is being implemented in `fix/rt-ci-route-phase-3-matched-control`.
  The `rust-workspace.yml` route job now records per-run provenance and uploads
  a route artifact for PR and dispatch events. `make rt-ci-matched-controls`
  reports matched pairs, exclusions, and separate medians. Focused collector
  tests cover successful pairs, failures, cancellation, reruns, missing route
  artifacts, tree mismatch, and incompatible workflow/runner definitions; 33
  `workflow_ci_gates` tests pass, along with the full `tachi-core` suite,
  Clippy, workflow/docs gates, formatting, and shell syntax checks. The live
  read-only inventory of the latest 40 workflow runs found 26 PR/dispatch
  candidates: 20 successful runs had no route artifact because they predate
  this change, 6 were unsuccessful, and there were 0 valid pairs for either
  shape. This sample does not establish timing performance; `RT-0sd.2` remains
  open pending ten valid pairs per shape and the <=65% execution-median ratio.
  P3-F05 documents that `scripts/verify-ci-timing-artifacts.sh` verifies the
  full eight-artifact matrix and cannot verify narrowed-run completeness. The
  focused contract retains full-matrix success and rejects a narrowed
  dependency-closure fixture missing the matrix artifacts; local validation
  passed before the separate P3-F05 delivery PR was opened.
  The P3-F05 milestone closed in PR #64 at `430ef76b20a9fe839a2165c8a349a348402513dd`.
  All nine workflow runs succeeded on final PR head
  `ab1ba2e7e7509a5240344aa4e28ad4c7ce9b8f37`; Codex comment 4193166301 was
  corrected, replied to, and resolved. GitHub rejected the auto-merge request
  because the PR was already `clean`; it was squash-merged through GitHub after
  checks passed. The historical timing pair count remains zero for both shapes.
- The P3-F06 replay manifest at
  [`docs/reports/rt-ci-route-cohort-audit-replay-manifest-2026-10-05.json`](../reports/rt-ci-route-cohort-audit-replay-manifest-2026-10-05.json)
  now records all 37 PR/head/head-tree identities, run IDs, changed paths,
  route reasons, artifact digests, and exact classifier commit/blob. A pinned
  reviewed SHA-256 digest anchors the complete manifest before replay; the
  saved classifier snapshot hash is also verified, and offline replay returns 37
  `full_pr_matrix` results (33 active/shared inputs, 4 unknown paths). Its Rust
  contract rejects tampered route reason, changed paths, head identity, and
  classifier provenance. The focused contract passes 2/2, targeted Clippy
  passes with warnings denied, and formatting and shell syntax pass. This is
  bounded historical route eligibility evidence, not timing evidence. PR #65
  merged at `2d3ff33d72053086160b23926a5cb6189c16732a` after nine hosted
  workflows succeeded on final head
  `b2d7016717c5f080fb928a1ce940114e5b607027`; Codex comment 4193406716 and both
  resulting Clippy code-scanning threads were resolved. GitHub rejected the
  auto-merge mutation as unstable, then accepted the normal protected squash
  merge after checks passed. `RT-0vf.7` is closed.
  The current complete collector output is archived at
  [`docs/reports/rt-ci-matched-control-inventory-2026-10-06.json`](../reports/rt-ci-matched-control-inventory-2026-10-06.json)
  (SHA-256 `25daf5b1f60adca111e6e4a0723c883dd0a04e541b08b181b88d562e8627595a`).
  It scans the latest 100 `rust-workspace.yml` runs and retains 74 PR/dispatch
  candidates: 44 lack an available route artifact, 10 lack trusted provenance,
  9 are unsuccessful, 1 has an ambiguous rerun attempt, and 10 have a full
  route rather than an optimized shape. There are zero eligible route samples,
  zero full controls, and zero matched pairs. Both route shapes remain
  `insufficient_pairs`; queue and execution medians are null. The supplemental
  API query also found the latest 11 PR workspace runs were full route. This
  bounded scan does not prove the timing threshold and does not replace future
  collection of ten distinct valid pairs per shape.
  The first inventory PR attempt (#67) was closed without merge after GitHub
  reported its branch update had bypassed a no-force-push rule. No main-branch
  content was changed by that attempt. The evidence commit was reapplied to a
  fresh branch with an ordinary push for PR #68; only the replacement PR can
  deliver this snapshot.
  A successful read-only forced-full dispatch (run `37419572327`) then exercised
  provenance upload and collector ingestion. The collector classified it as an
  unmatched control, with no compatible optimized PR run; the pair count remains
  zero. That earlier smoke predates exact merge-ref validation and per-job runner
  artifacts, so it is not eligible under the strengthened evidence contract.
  Its route artifact, run/tree IDs, runner image, workflow and classifier
  revisions, and timestamps are retained in
  [`docs/reports/rt-ci-matched-control-dispatch-smoke-2026-10-06.json`](../reports/rt-ci-matched-control-dispatch-smoke-2026-10-06.json).
  Review of PR #60 identified three initial evidence-validity gaps: workflow
  commit SHAs differ across PR and dispatch events despite identical workflow
  content; dispatch controls must target the actual PR merge commit; and
  measured jobs can receive different runner images. PR #60 merged with fixes
  for workflow-content comparison, actual execution-tree capture, exact PR merge
  ref validation, and per-job runner provenance. Three additional Codex comments
  posted immediately after merge exposed mutable-ref re-resolution by downstream
  jobs, missing exact execution-commit equality in the matcher, and dropped
  controlled-PR head provenance. The protected follow-up adds immutable
  downstream checkouts, exact commit matching, and captured controlled PR/head
  fields. PR #61 review identified default-branch cache-poisoning risk when
  controls run PR code and timing artifacts that still named the dispatch SHA.
  Matched controls now use a `pull_request` run triggered by the `ci-full-control`
  label, keeping execution in the PR cache scope; `rust-workspace.yml` uses a
  cache-free Rust setup action for both route shapes. The workflow consumes no
  PR token, and trusted provenance is captured before route scripts execute.
  Timing artifacts record the verified event merge SHA and controlled PR head;
  the verifier validates control evidence against the route artifact. The earlier
  dispatch smoke run remains non-pair evidence.
