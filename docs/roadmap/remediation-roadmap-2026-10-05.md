# RT-CI Route Remediation Roadmap — 2026-10-05

Status: **Phases 0–3 are delivered; post-merge review corrections for Phase 3
are in a protected follow-up, and matched-control timing acceptance remains
open.** Phase 1 merged in PR #58 at
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
| **2 — Dependency closure and repository contracts** | P2-F02: include `tachi-mcp` in shell reverse-dependency tests. P2-F03: run compact repository-wide manifest/toolchain/policy contracts independently of package routing. | Completed in PR #59, merged at `a8f4930c`. Shell changes include all reverse Cargo dependencies, including MCP. Repository contract inputs trigger the compact contract job regardless of package route. |
| **3 — Matched timing controls** | Implement a read-only collector and matched-run contract under `RT-0sd.2`. P3-F05 and P3-F06 remain separately tracked and nonblocking. | Initial collector merged in PR #60. Protected follow-up fixes pin every measured job and timing artifact to the route job's verified immutable execution SHA, require matching PR/control commits, retain controlled PR/head provenance, and remove dependency caching from `rust-workspace.yml` for both routed runs and controls so measured setup stays comparable. Matched controls also skip the repository-contract job, which is outside the timing artifact set; ordinary PR runs and non-control manual dispatches retain input-based repository-contract coverage. Start the ten-tree cohorts only after follow-up hosted checks pass. |
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
| Phase 3 — matched controls | P2 timing acceptance | For each route shape, collect ten distinct PR code trees. Each successful routed PR run is paired with a successful forced-full `workflow_dispatch` run on the exact same execution commit SHA and Git tree. The route job alone resolves the mutable PR merge ref, verifies the requested commit and controlled PR head, then publishes the verified execution SHA; every measured downstream job checks out that immutable SHA. Require identical workflow-file blob content and route classifier/path-producer revisions; compare the route runner and every measured PR job's OS, image version, and architecture against the corresponding full-control job. Capture run ID, controlled PR number/head SHA, event, attempt, route decision, execution SHA/tree, created/started/completed timestamps, queue duration, and execution duration. Exclude failed, cancelled, rerun-ambiguous, missing-provenance, and mismatched-commit/tree/workflow/runner pairs. Report queue and execution medians separately; optimized execution median is <=65% of full-control median for each shape. Otherwise keep `RT-0sd.2` open. | `RT-0vf.4` and `RT-0sd.2` |
| Phase 4 — integrated closeout | Closeout | Roadmap, review archive, raw evidence, Beads export, and integration log reconcile. Actual cohort counts and calculation are reproducible from retained inputs. Do not close timing acceptance on fewer than ten pairs per route shape or a failed threshold. | `RT-0vf.5` |

### Timing acceptance update for `RT-0sd.2`

The existing [2026-10-05 timing plan](2026-10-05-rt-ci-timing-evidence-followup.md) and initial issue acceptance compare pre-router with post-router
cohorts. Replace that criterion with matched full controls as described above;
retain the issue, its history, and existing collector work. Require ten distinct
identical code trees per route shape, successful routed PR and forced-full
control runs on each identical execution commit SHA and tree, captured
controlled PR number/head SHA, identical workflow-file content and route
classifier/path-producer revisions, comparable runner images for the route job
and every measured PR job, explicit exclusion rules, queue/execution separation,
and a <=65% optimized execution median. The 37-candidate historical audit remains
sample-bounded context, not a substitute cohort or a performance result.

`RT-0sd.2` depends on completion of the Phase 1 routing, Phase 2 repository
coverage, and Phase 3 matched-control collector issues. It does not depend on
the P3 advisory issues. The new epic is related to `RT-0sd.2`; it does not
replace or close it.

The matched-control collector is invoked with `RT_CI_USE_RTK=true make rt-ci-matched-controls`.
It reads completed `rust-workspace.yml` runs and their route and per-job runner
artifacts. Each run records the commit and Git tree actually checked out. To
create a control, dispatch `rust-workspace.yml` against the trusted default
branch (`main`), with `force_full_ci=true`, `control_pr_number=<PR number>`, and
`control_tree_sha=<the routed run's github.sha>`. The control checks out that
PR's merge ref and fails if it no longer resolves to the expected commit.
The route job is the only job that resolves the mutable PR merge ref; after it
verifies the requested merge commit and records the controlled PR head SHA, all
measured downstream jobs check out the route job's immutable execution SHA. The
collector requires the PR and control execution commit SHAs and Git trees to
match, and retains the controlled PR number, head SHA, and requested commit in
each control candidate and pair. It compares the workflow-file blob ID and
route script blob IDs, then verifies that every measured PR job has the same
runner OS, image version, and architecture in the full control. It emits matched pairs, excluded
candidates, run provenance, and separate queue/execution medians, ratios, and
per-shape acceptance state. Timing artifacts use the route artifact's verified
execution SHA and controlled PR head; the verifier binds dispatch evidence to
that route artifact. Control jobs load the Rust setup action from
`github.workflow_sha`, disable dependency caching, and do not persist checkout
credentials while executing PR code. The workflow grants only `contents: read`
and `pull-requests: read` and consumes no PR secrets. Keep the JSON output with
the closeout evidence.
`insufficient_pairs` is an expected open state and leaves `RT-0sd.2` open.

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
| PR #60 Codex comment 4192299099: mutable merge ref re-resolved by measured jobs | Fix in protected follow-up; downstream jobs use verified immutable SHA from route output | Phase 3 timing contract and review closeout | `RT-0vf.4`, `RT-0sd.2` |
| PR #60 Codex comment 4192299102: same tree could pair different execution commits | Fix matcher and regression fixture; require exact `execution_sha` equality | Phase 3 timing contract and review closeout | `RT-0vf.4`, `RT-0sd.2` |
| PR #60 Codex comment 4192299107: control drops controlled PR provenance | Capture controlled PR head at dispatch and retain PR number/head/commit in candidates and pairs | Phase 3 timing contract and review closeout | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 CodeQL alert 28: untrusted PR code could write a default-branch Rust cache | Require controls to dispatch the trusted default-branch workflow; load a setup action from `github.workflow_sha` that contains no cache action; disable checkout credential persistence; skip the unmeasured repository-contract job only for matched controls | Phase 3 control security contract | `RT-0vf.4`, `RT-0sd.2` |
| PR #61 Codex comment 4192392232: control timing artifacts record dispatch SHA | Write the route-verified execution SHA and controlled PR head into timing artifacts; make the verifier bind workflow_dispatch evidence to the route artifact | Phase 3 provenance and verifier tests | `RT-0vf.4`, `RT-0sd.2` |

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
  Phase 1 (`RT-0vf.2` and children `.2.1`/`.2.2`) closed after PR #58 and Phase 2
  (`RT-0vf.3` and children `.3.1`/`.3.2`) closed after PR #59. Phase 3
  `RT-0vf.4` is in progress. The JSONL export is synchronized in the current
  Phase 3 branch.
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
  fields. PR #61 review then identified default-branch cache-poisoning risk when
  controls run PR code and timing artifacts that still named the dispatch SHA.
  The follow-up requires controls to dispatch the trusted default-branch
  workflow; it loads a cache-free Rust setup action from that workflow revision
  for both routed runs and controls, disables persisted checkout credentials,
  and skips only the
  unmeasured repository-contract job on matched controls. It writes the verified
  execution SHA and controlled PR head into timing artifacts, and makes the
  verifier validate control evidence against the route artifact. The earlier
  smoke run remains non-pair evidence.
