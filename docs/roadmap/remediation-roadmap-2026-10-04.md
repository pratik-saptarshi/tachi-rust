# Main-48h Adversarial Remediation Roadmap — 2026-10-04

Status: **planning and baseline audit complete; code remediation not started**. Epic: **RT-aha**.
Nine confirmed P2 corrections (A1–A9) and three separate nonblocking P3
clarify/defer decisions (A10–A12). No closed issue is reopened or reused.

## Baseline and evidence

- Reviewed main: `8df554e884b1e5dd24146111a965597eff5f4779`; comparison base: `dd3b293d81d358d1ae27424be83b720693539112`.
- Panel window: 2026-10-02 21:37:02 UTC–2026-10-04 21:37:02 UTC;
  verdict REQUEST CHANGES, score 6.5/10, medium overall confidence.
- [Panel report](../reviews/2026-10-04-main-48h/review_panel_report.md),
  [process and evidence](../reviews/2026-10-04-main-48h/review_panel_process.md),
  [dashboard](../reviews/2026-10-04-main-48h/review_panel_report.html).
  The report and linked review evidence are archived with this plan so
  their local evidence links survive delivery. Report Markdown hard breaks use
  backslashes instead of trailing spaces; the evidence content is unchanged.
  Transient generator scripts and the raw changes.diff are excluded; reproduce
  the latter with git diff between the comparison base and reviewed SHA. Private /tmp probe paths in that
  historical record are not durable test fixtures; recreate them in Rust tests.
- Isolated branch: `docs/main-48h-remediation-roadmap`, worktree
  `.worktrees/main-48h-remediation-roadmap`, created at the reviewed SHA.
  The original dirty checkout and its stale export are preserved. Future fixes
  branch from this reviewed baseline/planning branch, with upstream changes
  reconciled explicitly before protected delivery.
- Setup audit on 2026-10-04: live Beads had **239 records: 236 closed,
  3 deferred, 0 open, 0 in progress** before creation. Its database is shared
  across worktrees. The original dirty export still listed five open RT-bbi
  records; that is a stale snapshot, not authority to reopen them. This work
  adds 19 records, yielding 258 total: 19 open, 236 closed, 3 deferred.
  New IDs and all existing records are exported to this worktree's
  [Beads mirror](../../.beads/issues.jsonl).
  Relative to the reviewed commit's dated export, this also synchronizes the
  already-closed live records `RT-3zm` and `RT-3zm.3`; their closure predates
  this roadmap. No historical issue is closed by the remediation setup.
- The local origin/main ref matches the reviewed SHA. No new network freshness
  check was performed for this planning task; the panel's fresh-main evidence
  belongs to its capture time. Refresh remote main and required checks at delivery.

## Adoption and boundaries

All P2 items are bounded corrections to behavior already adopted in the
[Rust-native upstream roadmap](../feature-roadmap-2026-10-04.md), not new
upstream feature imports. A1/A5/A6 harden assets and report rendering;
A2/A3/A7 repair authoritative evidence and attribution; A4 preserves MAESTRO
truthfulness; A8/A9 repair the adopted citation monitor. Missing or ambiguous
MAESTRO evidence remains `not_evaluated`, never inferred `clean`.

A10–A12 are contract decisions, not confirmed broken promises. Current-catalog
interpretation, executable/source identity and companion scope are distinct.
No automatic historical remapping, attestation subsystem or expanded PDF checker
is required without an explicit contract decision and separately scoped work.
All remediation, fixtures and domain validation remain Rust-native. No new
Python interpreter requirement or indirect application/build/codegen/test/
baseline/docs/CI Python execution chain is acceptable. Existing native Typst,
curl and minimal command orchestration remain within the established boundary.

## Key-delivery milestones and PR gates

The open Beads hierarchy is delivered as six reviewable milestones. Each
milestone ends in its own PR with a Conventional Commit history and evidence
linked from this roadmap. PR #45 established the planning baseline; this
follow-on PR delivers the Milestone 0 audit. Later milestones use separate
branches and PRs against the latest integrated predecessor.

At the start of every milestone, query all open PRs with `rtk gh` and inspect
the current head, required-check rollup, merge state, and review threads. Fix
failures and comments on the branch attached to the affected PR before relying
on it as a base. Enable protected auto-merge only after required checks are
terminal and accepted by GitHub protection, review threads are resolved,
platform evidence is present, and GitHub reports a mergeable head. Inspect any
NEUTRAL or SKIPPED status and its underlying jobs; do not infer acceptance.
Never bypass protection. Recheck the PR after every follow-up commit; pending
or failed checks block auto-merge.

| Milestone | Beads | Delivery PR boundary | Exit validation |
|---|---|---|---|
| 0 — Baseline and Rust-only tooling audit | RT-aha.1 | Follow-on PR after PR #45: dependency/tooling audit and synchronized tracker snapshot | Reviewed SHA and audit receipts recorded; no new Python execution path; docs and required hosted checks pass |
| 1 — Report evidence integrity | RT-aha.2; A2, A3, A4, A7 | One PR for the four report/parser corrections | Each trigger has a pre-change RED regression, corrected GREEN behavior, valid-input regression, and focused phase checks |
| 2 — Asset safety and PDF resilience | RT-aha.3; A1, A5, A6 | One PR for safe asset cleanup, image fallback, and report-root containment | Symlink and path-containment regressions pass; pinned Typst builds the affected tree and chain PDFs |
| 3 — Link monitor portability and correctness | RT-aha.4; A8, A9 | One PR for platform-safe test helpers and curl transport precedence | Portable classifier and loopback exhaustion tests pass; Windows test-target compilation is evidenced |
| 4 — Advisory contract decisions | RT-aha.5; A10, A11, A12 | Separate nonblocking decision PR, limited to contract wording or explicit deferrals | All three decisions record rationale, scope, and revisit triggers; this milestone does not block P2 delivery |
| 5 — Integrated P2 delivery and closeout | RT-aha.6 | Final evidence PR after Milestones 1–3 are integrated | Focused and workspace suites, formatting, Clippy, docs/workflow gates, pinned Typst, Windows evidence, and terminal required hosted checks are recorded |

Milestone 0 records the reviewed baseline, archives evidence, creates the
hierarchy, and closes the dependency audit only after its receipts below are
committed. The nine P2 leaves depend on RT-aha.1. Edit-order dependencies are
A2 → A3 → A7 (shared report/parser surface), A1 → A5 → A6 (asset resolver),
and A8 → A9 (monitor helpers); these reduce overlap without reclassifying
independent findings. A4 can proceed independently after the audit.

Each phase feature closes only after all listed children and phase validation
are complete. Milestone 5 / RT-aha.6 depends on the audit, all nine P2 fixes,
and milestones 1–3. It has no dependency on milestone 4 or any P3 card. P2
delivery may close while advisories remain open; the epic closes only after
A10–A12 have explicit decisions.

### Milestone 0 dependency/tooling audit evidence

- Reviewed source: `8df554e884b1e5dd24146111a965597eff5f4779`; current Rust
  environment: rustc 1.99.0 (b940084d7), Cargo 1.99.0 (5f94df478).
- `cargo metadata --locked --format-version 1`: exit 0; 90 locked packages
  across the five workspace members. The locked graph contains zero
  Python/PyO3/CPython/RustPython packages.
- `cargo tree --locked --edges normal,build,dev --prefix none`: exit 0.
  No dependency or lockfile changes are proposed by the current nine fixes;
  the planning PR also changes no Cargo manifest, workflow, or lockfile.
- Existing tooling is recorded separately: `.github/workflows/tachi-mmdc-preflight.yml`
  already sets up Python 3.14 for its renderer-absence preflight. This
  pre-existing workflow is outside the remediation changes; no new Python
  interpreter, indirect execution chain, action, or package is introduced.
  Reopen the audit if an implementation changes manifests or executable CI.
- On PR #45 head `46a5e979535be4cbf4b11847375b5a7fa5436c07`, 16 required
  checks succeeded and the required `CodeQL` aggregate was NEUTRAL; its three
  language analyses completed successfully. GitHub accepted that terminal state
  and protected-auto-merged the PR at `293c1ed028ad7bb8ac32eab571b0a748cf3dbe8a`
  on 2026-10-05 01:14 UTC, with both review threads resolved. At Milestone 0
  start, PR #26 was the only open PR: all 24 rollups were terminal without
  failures, all 17 required checks passed, and auto-merge was enabled. It was
  reported DIRTY at the initial check and is outside this roadmap’s base chain.
- Local validation: `make docs-version-gate docs-archive-version-gate
  workflow-gate gitleaks-gate supply-chain-gate` exited 0. Cargo audit and deny
  loaded 1,290 RustSec advisories and reported advisories, bans, licenses, and
  sources all OK.
- Follow-on PR #46 completed Milestone 0 and merged by protected auto-merge at
  `c0d27e83a0dd006fedabc5f176d7aee324b0226b` on 2026-10-05 01:43 UTC. Its final
  head `7d30864cbf769c6f991b12b3d437512e9b456401` has 20 passed checks, zero
  failures, and two skipped statuses; all required checks are accepted. The
  underlying Rust and JavaScript/TypeScript CodeQL jobs succeeded; CodeQL and
  Clippy aggregate statuses were NEUTRAL. Three review comments were fixed on
  that PR's branch and all three threads were resolved before merge.
- Beads `RT-aha.1` closed after this evidence was committed. The exported
  snapshot contains 258 issues: 237 closed, 3 deferred, and 18 open. The open
  hierarchy remains independently actionable; the P3 cards do not block P2.
- Phase 1 completed on PR #47 (`docs/main-48h-phase1`) and merged by protected
  squash auto-merge at `2026-10-05T02:35:23Z` as
  `f4b7fe1f08ce4e1abb820f159ac129c4eef55129`. All 17 required hosted checks
  passed. The first hosted attempt had one unrelated `ci_local_runner_contract`
  timeout-trap failure; the focused test passed locally and hosted retry attempt
  2 passed. The A4 review thread was replied to and resolved before merge.
  Beads Phase 1 children and feature `RT-aha.2` were closed with these receipts.

### Milestone 1 report-evidence integrity — local validation

At Phase 1 start after PR #46 merged, PR #26 was the only open PR. Its 23
checks were successful, one check was skipped, and none failed; its existing
protected auto-merge remained enabled. GitHub continued to report the unrelated
PR #26 branch as DIRTY/CONFLICTING with `main`, so it remains an explicit
mergeability blocker outside this phase's branch scope.

The pre-change regressions reproduced each phase trigger against the integrated
main baseline: the report-document contract run failed for both inventory-only
and ungrouped residual controls as well as unrelated risk-table rows;
`cargo test -p tachi-core --test maestro
duplicate_summary_rows_preserve_positive_maestro_evidence -- --nocapture`
failed when the later zero row erased count 2; and
`cargo test -p tachi-core --test parsers
nested_source_attribution_defaults_relationship_to_primary -- --nocapture`
failed with the missing `relationship` field. The completed-empty assessment,
canonical risk row, explicit attribution, and malformed-attribution contracts
remain covered by neighboring tests.

The fixes now require parsed residual findings or a valid completed-empty
assessment before Tier 1 selection, validate risk-table columns and nonempty
finding IDs/severities through the checked report API, retain the higher
positive MAESTRO duplicate, and default nested attribution to `primary`. Final
focused GREEN results after PR review follow-up: `report_document_contract`
14/14, `maestro` 6/6, `maestro_evaluation_states` 6/6, and `parsers` 22/22.
`cargo test --workspace --all-targets -q`
exited 0; one Gitleaks-specific test was skipped by its declared workflow-only
contract. Workspace Clippy with
`-D warnings`, `cargo fmt --all -- --check`, the documentation version/archive
gates, workflow gate, Gitleaks scan, and `catalog-drift --check` all passed.

Because the infographic source is included in catalog render-input hashes, all
registered PDFs and the manifest were regenerated using Typst 0.15.1
(`9dfd3a08`) from the official x86_64 macOS asset whose SHA-256 was verified as
`7f9fdd9584866245de9a79e0add8f9236fae6f40a8a45e2c4771ccc14db4e0fa`. PDF
comparisons and the offline catalog check pass. PR #47 merged via protected
squash auto-merge at `2026-10-05T02:35:23Z` as
`f4b7fe1f08ce4e1abb820f159ac129c4eef55129`; all 17 required hosted checks
passed. The first hosted attempt had one unrelated runner timeout-trap failure;
focused local and hosted retry attempt 2 passed. The A4 review thread was
replied to and resolved before merge. Phase 1 Beads cards are closed with these
receipts.

PR #47 review follow-up: the equal-count duplicate trigger was reproduced with
`L1 | 2 | Low` followed by `L1 | 2 | Critical`; the parser selected Low before
the fix. RED was also confirmed for `L2 | 0 | Clean` versus
`L2 | 0 | Not evaluated` in both row orders, where one order incorrectly
selected Clean. The reducer now chooses the more severe equal-count positive
evidence and deterministically resolves conflicting zero-count states to
NotEvaluated. The focused `maestro` suite passes 6/6, including both row-order
permutations; `maestro_evaluation_states` passes 6/6, `report_document_contract`
14/14, and `parsers` 22/22. The full workspace test run exited 0 with the
declared workflow-only Gitleaks test skipped; workspace Clippy and the catalog
drift check pass. The source and regenerated manifest are on PR #47's branch;
the review thread is resolved and protected auto-merge completed.

The code and regression tests are committed on PR #47 as
`eb504feda82768c1f6344a82218dd0e74bee4e2b`
(`fix(infographic): reconcile duplicate MAESTRO evidence`).

### Milestone 2 asset safety and PDF resilience — start evidence

Phase 2 started from merged main `f4b7fe1f08ce4e1abb820f159ac129c4eef55129`.
At this boundary, PR #26 is the only open PR. Its 17 required checks pass with
no failures; its remaining non-required check contexts are successful or
neutral. Protected auto-merge remains enabled, while GitHub reports `DIRTY`
against `main` (base head `cb567d3235eee0d45081b099a3c6df7c98f12eb7`). This
unrelated conflict is recorded and remains untouched.

The Phase 2 RED/GREEN regressions now cover each trigger. A1's baseline
symlink case failed because cleanup removed the regular JPG target retained by
the PNG symlink; after the guard, `cargo test -p tachi-core --test assets`
passes 5/5, including independent duplicate cleanup. A5's pre-fix test selected
the zero-byte preferred PNG despite a valid JPEG fallback. The corrected shared
resolver decodes PNG/JPEG candidates and validates SVG candidates as bounded,
renderable report diagrams, so empty/corrupt preferred files fall through to
later usable formats; all-invalid candidates leave attack trees in the Mermaid
text path and chains without an image. `report_document_contract` passes 22/22.
Its pinned Typst 0.15.1 compilation test compiled four cases: valid preferred
PNG, empty PNG with JPEG fallback, corrupt PNG with SVG fallback, and
semantically invalid SVG with the no-image fallback. A6's RED
experiment selected outside-root SVGs through both metadata and heading IDs;
the GREEN test rejects those IDs, rejects symlink escapes, and retains a valid
in-root S-1 image. The code uses Rust-native image decoding (`image` 0.25.10,
PNG/JPEG features only) and XML parsing (`roxmltree` 0.21.1); no Python chain was
introduced. Final local evidence: `cargo test --workspace --all-targets -q`
exited 0 with one declared workflow-only Gitleaks test ignored; workspace
Clippy with `-D warnings`, formatting, catalog drift, documentation/version/
archive and workflow gates, Gitleaks, and `make supply-chain-gate` all passed.
The supply-chain gate initially flagged duplicate transitive `miniz_oxide`
patch versions; `Cargo.lock` now pins compatible `flate2` 1.1.9 so the ban
gate passes without an exception. Hosted PR checks remain pending.

The automated PR #48 review identified a second A5 gap: well-formed XML with
zero dimensions, the wrong namespace, or invalid drawing semantics could still
be selected. RED reproduced that selection. A full renderer was not compatible
with the repository's Apache/MIT-only dependency policy, so the correction uses
the Apache/MIT `svgtypes` grammar crate and a conservative Rust-native validator
for report-diagram geometry. It checks namespace and bounded positive viewport,
viewBox, shape dimensions, SVG path/point syntax, visibility, and nonempty
supported drawing content; unsupported drawing elements and external image,
use, foreignObject, or script nodes fail closed to Mermaid. Five regressions
cover zero viewport, wrong namespace, malformed path, zero-sized shape, and a
hidden-only shape. The pinned Typst test compiles the semantically invalid SVG
through the no-image fallback. The focused document suite, 653-test workspace,
Clippy, format, catalog, and supply-chain gates pass. Review remediation source
commit: `fb98ba57eaf9156beb087e7c9d8ee13a7b5cb6ee` on PR #48's branch; the review
thread is replied to and resolved. PR #48's final head
`67072440978c8b65798f1ca0071994d5ce249409` passed 16 required checks, with no
failures and one skipped check, then protected squash-merged at
`2026-10-05T04:04:03Z` as `324f7ea7560216e978941571bb7d13523f682355`. Beads
`RT-aha.3.1`–`.3.3` and parent `RT-aha.3` are closed with receipts.

Phase 2 code commits on `docs/main-48h-phase2`: A1 is
`1d8ed47b29788c8b9ce8b4901475c7a8e4aa0af6`; A5/A6 and dependency/catalog
updates are `6501b51cef9225730458537a75092979df5a0825`; PR #48 review fix is
`fb98ba57eaf9156beb087e7c9d8ee13a7b5cb6ee`. Beads feature `RT-aha.3` and its
three finding cards are closed with the merge and check receipts above.

### Milestone 3 link-monitor portability and correctness — start evidence

Phase 3 starts from PR #48 merge `324f7ea7560216e978941571bb7d13523f682355`.
PR #26 is the only open PR at this boundary. Its head is
`ce7535c3129ea5c21302fddc66c211b107ea83b4`, recorded base is
`cb567d3235eee0d45081b099a3c6df7c98f12eb7`, auto-merge is enabled, and all 17
required checks pass with no failures. GitHub returned merge state `UNKNOWN` at
the baseline snapshot; that state is recorded without treating it as clean or
conflicted. Phase 3 implementation stays on `docs/main-48h-phase3` and is
isolated from the original dirty checkout.

Beads at this boundary contains 258 issues: 246 closed, 0 in progress, 9 open,
2 blocked, and 3 deferred. A8 remains a P2 correction requiring a real Windows
test-target compile receipt; A9 remains blocked on A8 and requires a loopback
redirect-exhaustion regression. Both preserve the monitor's public status and
exit contracts.

### Milestone 3 implementation and local validation

Source commit `1782b9f36c755fe34391b7fa5bc51a9d88a67910` implements A8 and A9.
The A8 RED reproduction used the Windows test target before the guard: direct
`rustc --test --target x86_64-pc-windows-msvc` compilation failed because
`std::os::unix::fs` and `Permissions::set_mode` are unavailable for that
target. The Unix permissions import, executable fake-curl helper, and its
subprocess test are now `cfg(unix)`; portable classifier tests remain active
for Windows. The exact monitor test source then compiled successfully as a
Windows test harness using `rustc --edition=2021 --crate-name
taxonomy_link_monitor --test --target x86_64-pc-windows-msvc --emit=metadata`
with the target's existing `serde_json` metadata.

The full Cargo Windows command,
`cargo check -p tachi-cli --bin taxonomy-link-monitor --tests --target
x86_64-pc-windows-msvc`, remains blocked before the monitor crate by the
pre-existing unconditional `.process_group(0)` call at
`crates/tachi-shell/src/commands/script_executor.rs:105`. The isolated monitor
test-harness compile passes; full package-target verification is recorded as
unverified until that unrelated Windows production compile blocker is fixed.

A9 RED was reproduced twice before the classifier fix: `classify(302, true)`
returned `healthy`, and a real local curl probe exhausted redirects with curl
exit 47, retained HTTP 302 and reported `healthy`. The regression uses an
ephemeral `127.0.0.1` listener, redirects to itself, and stops and joins its
server thread after the HEAD/GET probes. After the fix, the focused monitor
suite passed 12/12, including the real curl exhaustion case and portable
successful 2xx/3xx, HTTP-error, transport-error and HEAD-to-GET cases. The full
workspace suite passed 654 tests with one ignored test; workspace Clippy,
formatting, workflow action, documentation-version, archive-version, and
supply-chain gates passed. The supply-chain run loaded 1,290 advisories and
reported advisories, bans, licenses, and sources OK across 112 locked crates.
No manifest or lockfile dependency changes were made. Hosted PR checks remain
pending at this roadmap snapshot.

## Action-item mapping

| Panel | Priority | Beads | Phase | Adoption classification |
|---|---|---|---|---|
| A1 | P2 | RT-aha.3.1 | 2 | Existing defect / Rust-native correction |
| A2 | P2 | RT-aha.2.1 | 1 | Existing defect / Rust-native correction |
| A3 | P2 | RT-aha.2.2 | 1 | Existing defect / Rust-native correction |
| A4 | P2 | RT-aha.2.3 | 1 | Existing defect / Rust-native correction |
| A5 | P2 | RT-aha.3.2 | 2 | Existing defect / Rust-native correction |
| A6 | P2 | RT-aha.3.3 | 2 | Existing defect / Rust-native correction |
| A7 | P2 | RT-aha.2.4 | 1 | Existing defect / Rust-native correction |
| A8 | P2 | RT-aha.4.1 | 3 | Existing defect / Rust-native correction |
| A9 | P2 | RT-aha.4.2 | 3 | Existing defect / Rust-native correction |
| A10 | P3 advisory | RT-aha.5.1 | 4 | Contract clarification / optional deferral |
| A11 | P3 advisory | RT-aha.5.2 | 4 | Contract clarification / optional deferral |
| A12 | P3 advisory | RT-aha.5.3 | 4 | Contract clarification / optional deferral |

## Issue cards

Every card below is also stored in Beads with both roadmap and panel paths.
All cards are open at setup. Acceptance is a future closeout contract, not a
claim that implementation or checks have run. Record exact command, source SHA,
environment/version, exit status and receipt path for RED, GREEN and regression.

### A1 — RT-aha.3.1 — A1: preserve bytes reachable through retained symlinks

Priority: P2; type: bug; parent: RT-aha.3. Blocking dependencies: RT-aha.1.

**Production/test boundary:** crates/tachi-core/src/assets.rs cleanup decision; tests/assets.rs. Symlink creation helpers are platform-gated; ordinary duplicate tests remain portable.

**Acceptance:** Correctly named PNG symlink pointing to a mislabeled JPG with PNG bytes remains readable after cleanup. Conservatively skip dependent cleanup or prove retained independence before unlink. No sole backing bytes are lost.

- **RED:** Construct the retained PNG symlink to the candidate JPG, invoke opt-in cleanup and assert retained path readability and byte identity; reviewed code deletes the backing file.
- **GREEN:** Regression retains readable original bytes; independent ordinary duplicate still removes the mislabeled candidate.
- **Regression:** Cover symlink chains/dependent target, missing/different sibling, non-cleanup mode and safe regular duplicates. Record symlink platform capability instead of silently counting skips as passes.

### A2 — RT-aha.2.1 — A2: require completed controls before replacing findings

Priority: P2; type: bug; parent: RT-aha.2. Blocking dependencies: RT-aha.1.

**Production/test boundary:** crates/tachi-core/src/report_data.rs and compensating_controls.rs; tests/report_document_contract.rs and report_data.rs. Separate inventory presence from assessment completion.

**Acceptance:** A Missing/None control without residual assessment and populated residual rows outside severity subsections must each retain known High S-1 with diagnostic or return a checked error. Completed-empty assessments still select the authoritative tier, with control metadata intact. Preserve panel P1 dissent while implementing adopted P2.

- **RED:** Add two independent negative fixtures with valid High S-1: incomplete control inventory; populated but unconsumed residual table. Show pre-change successful zero-finding output violates the assertion.
- **GREEN:** Both negative fixtures prove checked failure or retained finding/count; existing legitimate completed-empty fixture remains successful with its metadata.
- **Regression:** Cover populated completed controls, empty/missing optional input, coverage-only inventory and Tier 2/Tier 3 selection; run report_document_contract and report_data.

### A3 — RT-aha.2.2 — A3: validate risk rows before promoting Tier 2

Priority: P2; type: bug; parent: RT-aha.2. Blocking dependencies: RT-aha.1, RT-aha.2.1.

**Production/test boundary:** crates/tachi-core/src/report_data.rs tier promotion and parsers/findings.rs scored-row validation; report_document_contract/report_data/parsers tests.

**Acceptance:** Unrelated headers and canonical rows missing identity or severity cannot replace valid S-1 with an anonymous finding or erase High counts. Use checked rejection or diagnosed lower-tier retention; valid scored rows still promote.

- **RED:** Pair valid High S-1 with unrelated scored headers, then canonical headers with missing ID or severity cells; assert identity/count preservation or checked error.
- **GREEN:** Each malformed case is rejected or preserves lower-tier evidence; canonical scored input selects Tier 2 with correct IDs, severities and totals.
- **Regression:** Run valid scored, empty optional, completed controls precedence and A2 regressions; cover malformed rows mixed with valid rows.

### A4 — RT-aha.2.3 — A4: preserve positive duplicate MAESTRO evidence

Priority: P2; type: bug; parent: RT-aha.2. Blocking dependencies: RT-aha.1.

**Production/test boundary:** crates/tachi-core/src/infographic.rs layer reduction and reconciliation; tests/infographic_payload.rs.

**Acceptance:** Summary-only L1 count 2 High plus L1 count 0 Clean cannot become clean/count 0 in either order. Reject conflicts or merge conservatively with positive precedence; normalize aliases consistently and never blindly sum duplicate counts.

- **RED:** Reproduce summary-only positive-then-clean overwrite; include reversed order and equivalent normalized layer aliases.
- **GREEN:** Conflicts produce checked errors or preserved positive count/state and most-exposed layer; identical duplicates do not inflate totals.
- **Regression:** Cover unique clean, findings, not_applicable and not_evaluated rows, positive versus not_applicable, absent evidence and detailed finding reconciliation.

### A5 — RT-aha.3.2 — A5: skip unusable optional images and continue fallback

Priority: P2; type: bug; parent: RT-aha.3. Blocking dependencies: RT-aha.1, RT-aha.3.1.

**Production/test boundary:** crates/tachi-core/src/report_data.rs shared image resolver/assets helper and templates/tachi/security-report/{attack-path,attack-chain}.typ; report/PDF fixture tests.

**Acceptance:** Zero-byte and nonempty corrupt preferred PNGs do not mask usable later JPG/SVG candidates. No usable candidate yields tree text fallback or chain no-image layout. Actual Typst 0.15.1 compilation succeeds in each corrected case; nonzero size alone is not proof of usability.

- **RED:** Replay empty preferred tree PNG with Mermaid source and actual Typst decode failure. Add corrupt nonempty first candidate plus valid later candidate and a separate chain fixture.
- **GREEN:** Prove chosen usable image, later-format fallback and all-invalid no-image behavior in bindings and successful pinned compiler output; test tree and chain separately.
- **Regression:** Compile valid preferred PNG/JPG/SVG and missing-image fixtures. Preserve tree raw-source fallback and chain layout (chains have no Mermaid text fallback). Run report_document_contract/report_data/assets.

### A6 — RT-aha.3.3 — A6: contain tree image resolution within report root

Priority: P2; type: bug; parent: RT-aha.3. Blocking dependencies: RT-aha.1, RT-aha.3.2.

**Production/test boundary:** crates/tachi-core/src/attack_trees.rs metadata/heading ID ingress and report_data.rs shared resolver; private report-root and neighboring image fixtures.

**Acceptance:** Reject unsafe IDs or omit unsafe bindings with a clear diagnostic. Validate intended ID grammar and canonical resolved containment within report root, including symlink escape. Neither metadata nor heading IDs may bind neighboring images.

- **RED:** Exercise ../../ ID with neighboring valid SVG through metadata and Attack Tree heading branches; assert no outside image binding. Add absolute path and symlink escape cases.
- **GREEN:** Traversal, absolute/platform path forms and symlink-to-outside all reject or omit; valid ID resolves usable in-root image, with no outside image-path emitted.
- **Regression:** Cover safe IDs, nested allowed paths if contract permits, dangling links, in-root symlinks and A5 alternative-format fallbacks; compile safe report fixture. Do not claim demonstrated disclosure or RCE.

### A7 — RT-aha.2.4 — A7: honor primary default in nested attribution

Priority: P2; type: bug; parent: RT-aha.2. Blocking dependencies: RT-aha.1, RT-aha.2.2.

**Production/test boundary:** crates/tachi-core/src/parsers/findings.rs nested deserialization; schemas/finding.yaml contract; parser and report tests.

**Acceptance:** Absent nested relationship defaults to primary exactly as flat form does. Explicit valid values survive; invalid/empty values remain subject to existing validation. Preserve absent-versus-empty attribution behavior.

- **RED:** Create equivalent flat/nested attribution without relationship; demonstrate nested missing-field failure against successful flat primary.
- **GREEN:** Both parse successfully to equivalent primary attribution; CLI/report behavior stays consistent.
- **Regression:** Test explicit primary/other schema-valid relationships, unknown and empty relationship, absent attribution and explicit empty collection; run parsers and report contracts.

### A8 — RT-aha.4.1 — A8: isolate Unix-only monitor test helpers

Priority: P2; type: bug; parent: RT-aha.4. Blocking dependencies: RT-aha.1.

**Production/test boundary:** crates/tachi-cli/src/bin/taxonomy-link-monitor.rs cfg(test) imports, permissions helper and fake-command tests. Production monitor semantics stay stable.

**Acceptance:** Guard Unix-only imports/helpers and their tests or replace with a portable fixture. Platform-neutral classifier tests remain compiled everywhere. Windows test-target compilation has an actual successful target/runner receipt before closure.

- **RED:** Record source dependency on std::os::unix::fs::PermissionsExt and obtain a pre-change Windows test-target compile failure if target/runner is available; otherwise mark runtime reproduction unverified.
- **GREEN:** Run local monitor tests and cargo check -p tachi-cli --bin taxonomy-link-monitor --tests --target x86_64-pc-windows-msvc; a Unix-only pass is insufficient.
- **Regression:** Verify Windows classifier tests are present (not hidden behind cfg(unix)); retain Unix fake-executable tests and host --all-targets compilation.

### A9 — RT-aha.4.2 — A9: prioritize curl failure over residual redirect status

Priority: P2; type: bug; parent: RT-aha.4. Blocking dependencies: RT-aha.1, RT-aha.4.1.

**Production/test boundary:** crates/tachi-cli/src/bin/taxonomy-link-monitor.rs probe/classifier; portable table tests and Rust-owned loopback HTTP server fixture using real curl.

**Acceptance:** Failed process/transport with residual 3xx, especially curl exit 47 plus HTTP302, cannot classify healthy. Preserve diagnostic and intentional informational command exit. Successful 2xx/3xx retains documented behavior.

- **RED:** Add failed-302 classifier case; serve endless redirects on 127.0.0.1 ephemeral port and reproduce real HEAD then GET exhaustion with curl error and incorrectly healthy output.
- **GREEN:** Classifier and loopback integration return nonhealthy needs-review/transient status, retain error and expected informational exit; bound server lifetime and clean up process/thread.
- **Regression:** Test successful 2xx/3xx, ordinary HTTP failures, transport failure without status and HEAD-to-GET fallback. No public network dependency; run all monitor tests on supported hosts.

### A10 — RT-aha.5.1 — A10: clarify or defer versionless taxonomy policy

Priority: P3; type: decision; parent: RT-aha.5. Blocking dependencies: none.

**Production/test boundary:** schemas/finding.yaml, taxonomy documentation and historical fixtures only if edition-aware support is explicitly chosen.

**Acceptance:** Record explicit current-catalog resolution with caller legacy review obligation, or defer with rationale/revisit trigger. Edition-aware preservation is conditional on an explicit contract and separately scoped implementation; never guess historical ID remaps.

- **RED:** Document the historical/current LLM05 meaning collision and absence of an edition field without claiming a proven historical regeneration promise.
- **GREEN:** Publish contract wording or a named deferral decision; if edition support is selected, define historical/current fixture expectations before code.
- **Regression:** Confirm current-catalog inputs remain valid and no advisory dependency blocks P2 delivery; historical tests are conditional on selected support.

### A11 — RT-aha.5.2 — A11: clarify or defer same-source builder precondition

Priority: P3; type: decision; parent: RT-aha.5. Blocking dependencies: none.

**Production/test boundary:** Catalog regeneration invocation/provenance documentation; catalog_drift.rs/CLI identity checks only if cross-revision generation is explicitly supported.

**Acceptance:** State builder and selected source root must use the same revision with prescribed Cargo invocation, or record defer rationale/revisit trigger. Arbitrary cross-revision support requires a separate identity/mismatch contract and test before expansion.

- **RED:** Record supplied-root hashing versus compiled report builder composition; stale-binary mismatch consequence remains unexecuted.
- **GREEN:** Document exact supported same-checkout invocation or explicit deferral; if broader support is chosen, design binary-A/source-B rejection evidence.
- **Regression:** Preserve same-source catalog regeneration, input-stability and rollback contracts; do not invent mandatory attestation scope.

### A12 — RT-aha.5.3 — A12: clarify or defer companion PDF checker scope

Priority: P3; type: decision; parent: RT-aha.5. Blocking dependencies: none.

**Production/test boundary:** Catalog checker documentation and conditional companion registration/hashing; existing registered baseline integrity contract stays authoritative.

**Acceptance:** Explicitly state --check covers registered baselines rather than every companion, or defer with rationale/revisit trigger. If companion integrity is adopted, separately scope registration/hashing and companion-only corruption regression.

- **RED:** Record panel fixture: changed companion with intact registered baselines passes --check; no current committed corruption is claimed.
- **GREEN:** Publish checker scope or explicit deferral; conditional expansion must detect companion-only corruption while intact outputs pass.
- **Regression:** Retain registered-baseline corruption detection and deterministic regeneration; P3 remains nonblocking.

### epic — RT-aha — Main-48h adversarial remediation roadmap

Priority: P2; type: epic; parent: none. Blocking dependencies: none.

**Production/test boundary:** Coordinate the pinned main review, nine bounded Rust corrections and three independent contract decisions. No automatic taxonomy migration or attestation expansion.

**Acceptance:** All A1–A12 have new generated IDs, evidence links, boundaries and validation. P2 delivery closes only with its required evidence; advisory decisions may remain open after P2 delivery. Epic closure additionally requires explicit A10–A12 decisions.

- **RED:** Inventory all twelve panel items and stale tracker/export state before setup.
- **GREEN:** Validate complete issue mapping, acyclic dependencies, archived panel links and isolated baseline.
- **Regression:** Preserve all prior issue records and original dirty work; no closed issue is reused.

### audit — RT-aha.1 — Audit Rust-only dependency and tooling boundary

Priority: P2; type: task; parent: RT-aha. Blocking dependencies: none.

**Production/test boundary:** Cargo runtime/dev/build dependencies, build scripts, test fixtures, code generation, baseline regeneration, docs tools and CI actions. This planning change adds no dependency or executable pipeline; archived panel scripts are evidence only.

**Acceptance:** Record before/after manifests and lockfiles plus transitive tooling paths for every proposed dependency. Reject new Python interpreters and indirect Python execution chains. Document preexisting tooling separately. Review again if later implementation adds a dependency.

- **RED:** Inventory Cargo metadata/tree, build scripts and relevant workflow commands at the reviewed SHA; identify any proposed Python-bearing path before adoption.
- **GREEN:** Record a Rust-native choice or no-new-dependency result for each fix; verify adopted transitive build/test/CI paths need no Python.
- **Regression:** Run existing supply-chain/workflow gates; compare lockfiles and CI pins. Missing network/advisory data is unverified, not clean.

### p1 — RT-aha.2 — Phase 1: report evidence integrity

Priority: P2; type: feature; parent: RT-aha. Blocking dependencies: none.

**Production/test boundary:** Report selection, MAESTRO evidence reduction and attribution parsing; Rust parser/report tests.

**Acceptance:** A2, A3, A4 and A7 meet focused RED/GREEN and valid-input contracts; phase validation is recorded. Missing evidence stays not_evaluated.

- **RED:** Capture each child trigger independently against 8df554e.
- **GREEN:** Run focused report, infographic and parser suites after all four corrections.
- **Regression:** Run phase closeout checks and semantic report goldens without masking changed finding counts.

### p2 — RT-aha.3 — Phase 2: asset safety and PDF resilience

Priority: P2; type: feature; parent: RT-aha. Blocking dependencies: none.

**Production/test boundary:** Cleanup and report-root image resolution, attack-tree/chain consumers and private image/PDF fixtures.

**Acceptance:** A1, A5 and A6 preserve reachable bytes, usable fallbacks and root containment; pinned Typst 0.15.1 compiles tree and chain cases.

- **RED:** Reproduce symlink-backed cleanup, empty preferred image and path-valued ID selection independently.
- **GREEN:** Run assets/report suites and actual pinned Typst compilation of good, fallback and no-image cases.
- **Regression:** Verify ordinary duplicate cleanup and safe in-root image selection; keep chain no-image behavior distinct from tree text fallback.

### p3 — RT-aha.4 — Phase 3: link monitor portability and correctness

Priority: P2; type: feature; parent: RT-aha. Blocking dependencies: none.

**Production/test boundary:** taxonomy-link-monitor test helper platform guards and transport/status classification; no change to informational exit semantics.

**Acceptance:** A8 and A9 pass portable classifiers, Unix helpers, loopback redirect exhaustion and Windows test-target compilation.

- **RED:** Capture failed curl with residual HTTP 302; compile the reviewed test target for Windows where available.
- **GREEN:** Run monitor tests locally and cargo check -p tachi-cli --bin taxonomy-link-monitor --tests --target x86_64-pc-windows-msvc on a provisioned target or Windows runner.
- **Regression:** Keep platform-neutral classifiers enabled on Windows; retain successful HTTP and HEAD-to-GET cases.

### p4 — RT-aha.5 — Phase 4: advisory contract decisions

Priority: P3; type: feature; parent: RT-aha. Blocking dependencies: none.

**Production/test boundary:** A10–A12 documentation/contract decisions only. Broader production changes require an explicit supported contract.

**Acceptance:** Each advisory records clarify or defer, rationale, contract wording, and conditional follow-up validation. No advisory blocks P2 delivery.

- **RED:** Record current ambiguity and bounded evidence for each advisory.
- **GREEN:** Review three independent decisions and link exact contract docs or a deferral with revisit trigger.
- **Regression:** Verify no blocking dependency from the P2 delivery task or P2 fixes reaches these advisory cards.

### delivery — RT-aha.6 — Verify and deliver P2 remediation with synchronized evidence

Priority: P2; type: task; parent: RT-aha. Blocking dependencies: RT-aha.1, RT-aha.2, RT-aha.3, RT-aha.4, RT-aha.3.1, RT-aha.2.1, RT-aha.2.2, RT-aha.2.3, RT-aha.3.2, RT-aha.3.3, RT-aha.2.4, RT-aha.4.1, RT-aha.4.2.

**Production/test boundary:** Roadmap, Beads export, local/hosted check receipts and protected delivery of A1–A9. A10–A12 and Phase 4 do not block this task.

**Acceptance:** Record reviewed/implementation/test SHAs, RED/GREEN/regression receipts, platform/compiler versions, required local results and terminal required hosted checks for the final head. Synchronize roadmap/export and preserve advisory state. Pending, unavailable or stale checks cannot be passed.

- **RED:** Enumerate missing receipts and required check names from the live protected branch rules; separate baseline/environment failures from regressions.
- **GREEN:** Verify focused suites, cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --workspace --all-targets, docs/workflow gates, pinned Typst compilation and Windows target evidence. All required hosted results are terminal for the delivered head.
- **Regression:** Recheck changed-head checks after any fix, catalog-drift only when reporting/catalog inputs change, and export/document status consistency. Record final freshness and unresolved limitations; never infer CI success from a clean diff.

## Validation and closeout protocol

1. Add a minimal Rust regression for each independent trigger at the reviewed
   source revision, before changing production behavior. Record a failing
   assertion attributable to the defect; compiler/tool/network failure is not
   a successful RED reproduction. Panel findings are input evidence, not a
   substitute for new regression receipts. A8's reviewed source diagnosis is
   confirmed, but actual Windows reproduction remains unverified until run.
2. Make the bounded correction, run the same test to GREEN, then sibling and
   valid-input tests. For A2 and A3, checked errors and diagnosed retained
   evidence are acceptable; silent successful evidence loss is not. Record
   the selected behavior in the issue before closure.
3. For every phase, record focused tests plus formatting, workspace Clippy and
   tests, relevant docs/workflow checks and terminal required hosted checks.
   Shared commands, from the implementation worktree:

   ```sh
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace --all-targets
   make docs-version-gate docs-archive-version-gate
   make workflow-gate
   git diff --check
   ```

   Focused commands use existing Rust integration targets, extended with each
   card's new cases:

   ```sh
   cargo test -p tachi-core --test report_document_contract --test report_data --test parsers --test infographic_payload --test reporting_goldens
   cargo test -p tachi-core --test assets --test report_document_contract --test report_data
   cargo test -p tachi-cli --bin taxonomy-link-monitor
   cargo check -p tachi-cli --bin taxonomy-link-monitor --tests --target x86_64-pc-windows-msvc
   ```

4. A5/A6 PDF receipts must show native **Typst 0.15.1 (9dfd3a08)**, generated
   report bindings, compile command/root, exit status and output artifact for
   both trees and chains. Recreate the panel's
   [minimal pair](../reviews/2026-10-04-main-48h/state/phase_13_point_2_image.md)
   using copied private templates and Rust-owned fixtures. Tree fallback is raw
   Mermaid source; chains have a no-image layout and no Mermaid text fallback.
   Exercise empty and corrupt data, later valid formats, all-invalid and valid
   preferred candidates. Guard symlink fixture creation by platform capability.
5. Report/core rendering changes invalidate catalog rendering fingerprints.
   Run `cargo run --locked -p tachi-cli --bin catalog-drift -- --check`.
   When fingerprints legitimately change, use the documented same-checkout
   `--regenerate-baselines --typst /absolute/path/to/typst` flow, then the
   offline check and `cargo test -p tachi-core --test backward_compatibility`
   with that pinned executable on PATH. Review regenerated outputs; do not
   accept a blanket golden update in place of preserving findings.
6. The dependency task covers runtime, development, build, test and CI tools,
   including transitive build scripts and wrappers. Capture manifest/lockfile
   deltas, Cargo dependency evidence, relevant workflow pins and supply-chain
   gate output. A clean source-language inventory alone is insufficient.
7. At delivery, refresh remote main and branch rules, record final commit SHA,
   required check names, run URLs, terminal conclusions and review/merge state.
   A pending aggregate, skipped required check, unavailable Windows runner or
   missing Typst environment remains **unverified**. Separate existing baseline
   or environment failures from regressions. Recheck when the tested head changes.
8. Update this roadmap and export Beads from the shared live database into the
   implementation worktree. Verify old issue records are preserved and all IDs,
   priorities, dependencies, decisions and completion claims match. Close P2
   delivery independently of optional advisory implementation.

## Setup verification and remaining work

This change implements the roadmap/worklist request only. Production fixes,
new RED/GREEN runs, full workspace checks, Windows compilation, pinned Typst
remediation compilation and hosted delivery remain **not run / unverified**.
Panel runtime claims are preserved with their original limits: A6 establishes
outside-image selection, not executed disclosure; A8 is a source-target
diagnosis; A5's actual compile failure covers the exercised empty tree image.
A10–A12 still await their explicit decisions.

Planning validation completed on 2026-10-04 against this worktree's final export:

| Check | Observed result |
|---|---|
| Beads export and content validation | 258 records; 19 new open cards; all 239 preexisting live records unchanged |
| Hierarchy and action mapping | 1 epic, 4 features, 9 P2 bugs, 3 P3 decisions, 2 shared tasks; all 12 actions mapped |
| Dependency validation | `bd dep cycles`: no cycles; every mandatory dependency present; no P3 reachable through P2 delivery blocking edges |
| Evidence links and provenance | 58 relative links in roadmap/report resolve; 56 archived files match source evidence except documented report hard-break normalization |
| Original checkout preservation | Original status, tracked binary diff and dirty Beads export match the before snapshots |
| Documentation checks | `make docs-version-gate docs-archive-version-gate`: both passed |
| Whitespace | `git diff --check` passed; explicit no-index whitespace checks passed for all 57 new files (roadmap plus archived evidence) |
| Publication freshness | `git fetch origin main` on 2026-10-04 confirmed origin/main still at the reviewed SHA; planning branch starts with no upstream divergence |
| Secret scan | Gitleaks directory scan of the archived review evidence passed with no leaks |

The structural checks used temporary local Node scripts for authoring/inspection;
no script or dependency was added to the project's execution pipeline. Full Rust,
workflow, Windows, Typst and hosted checks are future remediation gates, not
claimed passes for this documentation/tracker change.

## Publication and artifact retention

Publish this planning deliverable from `docs/main-48h-remediation-roadmap` in
two Conventional Commit slices: `docs(review)` for the panel evidence archive,
then `docs(roadmap)` for the phased worklist, navigation and live Beads export.
The PR description records commit IDs, local validation and current hosted
status. Publishing this plan does not close any `RT-aha` remediation issue.

Retain the panel report, HTML dashboard, chronological process, individual
review/verification records, reviewed commit/path inventories and gate receipt.
The `state/` directory is durable review evidence linked from the report, not
an execution cache. Do not commit temporary authoring/check scripts, copied
before-state exports, raw diff bundles, scratch issue maps, probe directories,
locks or command logs. Transient helpers created specifically for this planning
task are removed after validation; original reviewer fixtures remain outside
this worktree and are not publication artifacts.
