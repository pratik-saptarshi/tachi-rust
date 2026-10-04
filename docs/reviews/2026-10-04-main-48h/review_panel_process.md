# Overseer review panel process history

Pinned main: `8df554e884b1e5dd24146111a965597eff5f4779`. Window: 2026-10-02 21:37:02 UTC – 2026-10-04 21:37:02 UTC.

Full state artifacts are reproduced verbatim below. Subagent progress messages and short handoff summaries are operational metadata; the full written outputs are authoritative. No product changes were made. Opus was unavailable; the inherited model was used consistently. Runtime capacity required two isolated batches per six-person phase. Devil’s Advocate Phase 3 needed one completion retry after an unfinished probe; the final artifact is complete. No mandatory review phase was dropped. The local reference prompt file contains compact skeletons, so SKILL.md supplies the detailed schema.


---

# Persona Profiles Registry

<!-- Source artifact: state/personas.md -->

# Persona Profiles Registry

All agents use the same inherited available model. The skill's requested Opus model and Agent-specific plugin types are unavailable in this runtime; this is an explicitly adapted Overseer run. Shared model biases remain a limitation. Three child slots require two isolated batches for six independent reviewers; no reviewer sees another's Phase 3 output before debate.

| Slug | Persona | Expertise / lens | Reasoning | Agreement | Phases |
|---|---|---|---|---|---|
| correctness | Correctness Hawk | Rust parsing, data integrity, edge cases | Systematic enumeration | 30% | 3, 4, 5, 7 |
| architecture | Architecture Critic | Cross-boundary contracts and compatibility | Backward reasoning | 50% | 3, 4, 5, 7 |
| security | Security Auditor | Filesystem safety, auth and isolation | Adversarial simulation | 30% | 3, 4, 5, 7 |
| devils_advocate | Devil's Advocate | Challenge tests, assumptions and happy paths | Analogical reasoning | 20% | 3, 4, 5, 7 |
| code_quality | Code Quality Auditor | Exact source/config and taxonomy semantics | Systematic enumeration | 40% | 3, 4, 5, 7 |
| pipeline | Pipeline Reviewer | CI, dependencies, failure behavior and operational reliability | Checklist verification | 30% | 3, 4, 5, 7 |

Support roles: data-flow tracer (Phase 2, boundary certificates); completeness auditor (Phase 8, overlooked cases); citation verifier (Phase 10, factual evidence); severity verifier (Phase 11, current defect impact and safeguards); tier advisor and focused verification specialists when needed (12–13); neutral judge (14); independent judge-output verifier (14.5); HTML rendering specialist (15.3). Support agents do not vote as panelists.

Classification: code/implementation review with supporting prose, hence Precise code/config findings and Exhaustive prose analysis. Code Quality Auditor is mandatory for code; Pipeline Reviewer is the signal specialist for CI/infra changes. Security and architecture cover the SQL/auth and reliability signals without exceeding six reviewers. Standard trace, one run, default Correctness/Completeness/Quality/Edge Cases criteria.


---

# Phase 1: Setup

<!-- Source artifact: state/context.md -->

# Phase 1: Context Brief

## Codebase State
Review root: `/Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004`.
Pinned fresh origin/main: `8df554e884b1e5dd24146111a965597eff5f4779`; detached isolated worktree, zero commits behind at capture. Original dirty checkout is out of scope and must be preserved.
Window: 2026-10-02 21:37:02 UTC through 2026-10-04 21:37:02 UTC, using committer timestamps of changes merged onto main. Base: `dd3b293d81d358d1ae27424be83b720693539112`. Seven commits: PRs 35, 37, 38, 41, 43, 44, 42. See commits.txt and changed-files.txt. Review current resulting behavior, not already corrected intermediate defects. Prove introduction/worsening against base.

## System Documentation Found
Read root codemap.md first; crate codemaps describe core -> shell facade -> CLI/desktop/MCP. README identifies active Rust threat modeling/security report harness; Python/FastAPI references may be archived scaffold history. CLAUDE.md describes generic framework methodology; this is a review, no feature implementation, PR, or tracker changes requested. Roadmaps under docs/roadmap and feature-roadmap-2026-10-04.md preserve feature contracts. No Python execution is permitted in this review.

## Referenced Files and Scope
changes.diff contains review text, excluding binary PDFs, generated npm lockfiles, generated SARIF companions, large crosswalk catalog data, catalog manifest and Beads export to keep the shared bundle below 20,000 lines; those are still available on disk for targeted inspection. Prioritize changed Rust parsing/report/SARIF/MAESTRO, catalog publication and assets, active prompts/taxonomy semantics, new CI and permission scanners, dependency/runtime pins, and Next.js/Supabase/Prisma frontend migration and tenant isolation. Inspect relevant unchanged callers/guards/tests when necessary. Do not claim all files were exhaustively reviewed.

## Safety Mechanisms
Core checked report builder propagates attribution parse errors; legacy API emits a Typst panic. MAESTRO has shared typed states. Catalog regeneration stages renders and checks inputs before publication with rollback; inspect actual boundaries rather than assuming atomic guarantees. Desktop/MCP have containment and policy checks. Gitleaks defaults are inherited. Prisma RLS migration and verification script exist. Verify these guards before claiming missing safety. No changes to branch protection are in scope.

## Knowledge Mining Results
Memory registry identifies Rust core/shell/CLI/MCP/desktop, isolated worktree convention, evidence-based readiness and graph exclusions. Graph queried first: it returns older build_report_data_typst (51-line implementation) inconsistent with pinned source, so use it for orientation only; new symbols/CLI/tests/config require direct pinned file reads. Do not cite stale graph snippets as ground truth. Current earlier CI passing is context, not proof of correctness.

## Domain Checklist
Code correctness: every fallback and empty/malformed state; identity, citation and asset preservation; serialization/template contracts. Security: root containment, symlinks, untrusted strings, authentication/session refresh and tenant isolation. Reliability: staged output and failure rollback, concurrency, deterministic hashes, offline checks. CI: triggers, pins, permissions, meaningful negative tests. Taxonomy: contextual meaning and historical exceptions. Temporal claims: count every affected event across the full interval.

## Review Mode and Protocol
Mixed scope: Precise for code/config, Exhaustive for prose. Standard data-flow trace of the highest-complexity report generation path. Single run. Six personas: correctness (30%, systematic enumeration), architecture (50%, backward reasoning), security (30%, adversarial simulation), devils_advocate (20%, analogy), code_quality (40%, systematic enumeration), pipeline (30%, checklist). Independent reviews receive same context and trace and must not read other reviewer files before debate. Runtime permits three child agents concurrently, so independent phases run in two isolated batches. Opus is unavailable; all agents use inherited available model consistently. Record this adaptation and correlated-model limitation; do not pretend Opus was used.

## Context Gaps and Constraints
No production data/live agent evaluation; binaries excluded from source review. Focus is new regressions in this 48-hour range, not general debt. External domain claims must be checked with primary sources before high severity. Shell commands must start with rtk (general commands rtk proxy); git/gh via rtk. Do not mutate product code or use Python. Only write assigned review state artifacts; use private temporary fixtures for reproduction. You are not alone; do not revert others' work. Findings require file:line, concrete trigger, observed/traceable consequence, missing-guard search, base comparison, confidence, proposed regression and fix. No manufactured severity or minimum finding quota.


---

# Reviewed commits

<!-- Source artifact: state/commits.txt -->

8df554e884b1e5dd24146111a965597eff5f4779 2026-10-04T14:40:56Z docs: finalize phased upstream roadmap delivery evidence (#42)
c933cd4bb6b7a99a1e07ce56f940dbccab0d6cc4 2026-10-04T13:57:02Z feat(reporting): deliver native upstream reporting and automation milestone (#44)
b7cdd03a3ad7625082eb84ff4bc43ba8e8295744 2026-10-04T13:32:01Z fix(taxonomy): deliver OWASP 2026 semantic milestone (#43)
cb567d3235eee0d45081b099a3c6df7c98f12eb7 2026-10-04T09:50:42Z fix(040): address PR #39 review and closeout findings (#41)
35be8778c3aacea179224676d42c45a6f50a33b7 2026-10-04T00:15:32Z feat: adopt upstream taxonomy parity and safe image cleanup (#38)
ddd19ef852e99f805eed23c0062a1b932ba7f462 2026-10-03T22:52:21Z feat(taxonomy): adopt upstream framework and reporting updates (#37)
cc2f18e1bc2afe6d8fd63f9ccc608718204847ce 2026-10-03T20:09:49Z chore(deps-dev): bump the npm_and_yarn group across 2 directories with 1 update (#35)


---

# Phase 2: Data Flow Trace

<!-- Source artifact: state/phase_2_data_flow.md -->

> **Persona Profile — phase_2_data_flow.md**
> Data Flow Tracer; boundary schemas and composition certificates; Phase 2. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 2 — Standard data-flow trace

Pinned source: `8df554e884b1e5dd24146111a965597eff5f4779`; comparison base: `dd3b293d81d358d1ae27424be83b720693539112`. Root is the isolated `overseer-main-20261004` worktree. All citations below are relative to that root and refer to current pinned source. This is one critical-path trace, not an independent reviewer verdict. Candidates require reviewer validation and are not automatically P0.

## Path and external inputs

`report-data` arguments → shell checked wrapper → core `threats.md` parsing → source attribution and taxonomy aggregation → optional risk/control tier selection → MAESTRO, scope, remediation and attack artifacts → Typst bindings → CLI file/stdout publication → Typst report template consumers.

External inputs: CLI target/template/output paths and cleanup flag; UTF-8 Markdown artifacts (`threats.md`, `risk-scores.md`, `compensating-controls.md`, `threat-report.md`, `attack-chains.md`, attack-tree files); taxonomy YAML; report and brand image bytes; filesystem availability. This command does not compile the PDF. Template consumption is the next boundary; catalog regeneration is a separate publication path outside this Standard trace.

Graph discovery used `search_graph` and outbound `trace_path(build_report_data_typst, data_flow, depth=2)`. Its 51-line builder is stale and excludes the new checked builder and document transforms. Direct pinned source and base diff are authoritative. Read the Phase 1 context, root atlas, and Overseer Phase 2 instructions. No product changes, runtime executions, or test-pass claims are made here.

## Boundary certificates

### 1. CLI `parse_args` / `main`

Source: `crates/tachi-cli/src/bin/report-data.rs:8-89`.

- Input schema: arbitrary OS argument strings decoded by `std::env::args`; required target/template `PathBuf`, optional output `PathBuf`, cleanup boolean default false. These are directly caller controlled.
- Transform: recognized flags consume their next argument; missing required flag/value or unknown flag gives usage error / exit 2. `main` calls checked wrapper before cleanup and output. Checked error is stderr / exit 1.
- Output schema: arguments are syntactically present paths, with no existence guarantee. Successful core output becomes bytes written with `fs::write`, or stdout. Parent directory creation and write errors return exit 1.
- Composition: invalid attribution prevents output publication and cleanup. Output is Typst source, not proof of successful compilation. Direct file replacement is non-atomic; this already existed at base and is not a new regression. Cleanup is new and occurs after generation but before output write, so failure to publish does not roll it back.

### 2. `try_report_data_result` / `render_report_data_result`

Source: `crates/tachi-shell/src/command_use_cases.rs:19-58`.

- Input: target/template `&Path` from CLI.
- Transform: checked core result is wrapped without filtering; rendering clones the string.
- Output: `Result<ReportDataResult { typst: String }, String>` → identical Typst text.
- Composition: error propagation is preserved on this CLI path. Legacy `report_data_result` remains separate; core compatibility builder returns a Typst `#panic` on invalid input (`report_data.rs:19-24`). Prefix-only `validate_report_data_result` is not invoked by this CLI. It is not a compiler or full schema validator.

### 3. `try_build_report_data_typst`

Source: `crates/tachi-core/src/report_data.rs:26-100`.

- Input: caller-selected target/template directories.
- Transform: read `threats.md`; NotFound becomes empty content; other I/O errors propagate. Resolve project name, parse findings, then detect images. Build raw-threat attribution rows and aggregate framework coverage. Taxonomy directory is template ancestor 3 plus `schemas/taxonomy`; existing directory overrides compiled workspace fallback. Append base bindings, attribution, MAESTRO and document data.
- Output: `Result<String,String>` containing all Typst bindings; source text is reused for threat-derived branches.
- Composition: parse failures are now fail-closed before image detection, correcting baseline `unwrap_or_default`. Missing threats remains intentionally accepted. Optional artifact reads deeper in the graph still swallow errors, so the checked API guarantees valid primary parsing, not successful reads of every artifact.

### 4. `parse_threats_findings` / attribution extraction

Source: `crates/tachi-core/src/parsers/findings.rs:293-339,404-550`.

- Input: untrusted Markdown text.
- Transform: parse explicit attribution first, even without recommendation rows. Read `## 7. Recommended Actions`; skip rows without a usable finding ID. Populate component/threat/risk/mitigation strings; likelihood and impact become the em dash sentinel. Normalize pattern and optional delta status. Match explicit attribution by exact finding ID; absent mapping may use explicit OWASP-reference column evidence.
- Output: `Result<Vec<ThreatFinding>,String>`, each with strings plus `Option<Vec<SourceAttributionRecord { taxonomy,id,relationship }>>`.
- Composition: absent attribution and explicit empty vector remain distinct. Explicit empty vector wins over OWASP fallback. Nested YAML validates finding-ID syntax, supported taxonomy, supported relationship and nonblank ID. Catalog membership is a different function, `validate_source_attribution` (`findings.rs:342-375`), and the report builder does not call it. This separation already existed for older attribution inputs; nested YAML and explicit OWASP fallback extend accepted inputs in this window.
- Row stability: recommendation rows define raw finding membership; no cross-document ID reconciliation is guaranteed.

### 5. Coverage projection and aggregation

Source: `crates/tachi-core/src/coverage_attestation.rs:98-182,204-279`; `report_data.rs:428-521`.

- Input: raw `Vec<ThreatFinding>` and filesystem taxonomy store.
- Transform: one attestation row per raw finding, lowercased severity and mapped framework-reference buckets. Iterate catalog records for framework matrices; primary references imply covered, related/derived imply partial, otherwise gap. Coverage denominator is in-scope catalog rows; zero denominator is handled explicitly. Render reference groups and matrix items to Typst tuples.
- Output: raw finding identity is preserved in per-finding rows; aggregates are per taxonomy record, not per threat. Coverage is intentionally a different cardinality from the findings table.
- Composition: these rows continue to describe raw threats even when document details later select risk/residual findings. Reviewers should distinguish intentional inherent-threat attribution from an actual identity mismatch. Unknown IDs can appear in displayed references while matching no catalog record; catalog validation is not part of this path.

### 6. `render_document_data` tier selection

Source: `crates/tachi-core/src/report_data.rs:105-225,261-282`; `parsers/findings.rs:158-178`; `compensating_controls.rs:64-174`.

- Input: raw threat text/findings plus optional sibling files and template assets.
- Transform: initialize Tier 3 raw findings (`id,component,threat,likelihood,impact,risk_level,mitigation`). Parse optional risk file; any nonempty returned rows select Tier 2 and replace findings (`id,component,threat,composite_score,severity,cvss,exploitability`). Parse controls file; `has_control_assessment` selects Tier 1 and replaces findings (`id,component,threat,residual_score,residual_severity,control_status,recommendation`).
- Controls parser groups severity sections, deduplicates IDs, and recalculates severity from numeric score where available. Assessment evidence includes nonempty findings, controls or coverage matrix, a canonical residual-table header, or complete numeric coverage metadata (`report_data.rs:285-365`). A completed assessment may intentionally have zero residual findings.
- Output: tagged union encoded as JSON map with integer `data-source-tier`. Severity counts and total are recomputed from the selected findings, not stale raw counts (`261-276`). Component distribution also uses selected findings.
- Composition: Typst receives matching tier-specific keys (`templates/tachi/security-report/main.typ:405-409`, `findings-detail.typ:182-213`). Risk parser accepts arbitrary row maps and defaults absent columns to empty strings; selection validates only nonempty row count. See candidate DF-1. Full replacement does not verify that selected IDs cover raw IDs; this is a contract assumption, not automatically a defect.

### 7. `build_remediation_actions`

Source: `crates/tachi-core/src/report_data.rs:197-225`; `report_extraction.rs:137-191`.

- Input: selected findings projected into `RemediationFinding`, tier, controls flag, parsed optional threat report.
- Transform: Tier 1 uses residual severity, recommendations and control status; other tiers depend on threat-report remediation evidence, using scored severity/threat text for Tier 2 or raw risk/mitigation for Tier 3. SLA derives from severity.
- Output: optional action vector; builder converts None to empty Typst array. No rows are synthesized when helper declines to build actions.
- Composition: missing action fields are represented as empty strings at JSON projection. Existing helper policy can omit actions without threat-report evidence even if findings exist; this helper behavior predates this change, while invoking it from the complete document builder is new. Do not claim actions are unconditionally guaranteed.

### 8. MAESTRO evaluation and grouping

Source: `crates/tachi-core/src/infographic.rs:250-307,443-515`; `maestro_coverage.rs:35-58`; `report_data.rs:153-154,395-405`.

- Input: threat Markdown coverage table and per-finding layer records.
- Transform: normalize layer labels; parse count as Option; provide seven canonical absent-layer rows with NotEvaluated; retain additional layer keys. Positive counts force Findings; zero plus explicit clean evidence permits Clean; explicit nonapplicability permits NotApplicable; otherwise NotEvaluated. Observed per-finding rows raise layer count to max(declared, observed), override state to Findings, and may raise severity. Group findings by normalized layer for display.
- Output: `MaestroData` with distribution, observed rows, heatmap, most-exposed layer, presence flag; Typst state strings and human labels are emitted together.
- Composition: both report coverage and grouped findings derive from the same extractor, preserving state semantics. Component-layer mapping alone does not establish evaluation. A declared count exceeding observed rows is retained rather than silently reduced; count is not promised to equal rendered detail membership. Missing count is temporarily numeric zero but retains NotEvaluated state, preserving meaningful missingness.

### 9. Assets, attack trees/chains, cleanup

Source: `crates/tachi-core/src/assets.rs:35-85,128-168`; `report_data.rs:226-260`.

- Input: caller-selected target directory, IDs parsed from threat artifacts, image bytes and filesystem entries.
- Transform: report images use signature detection, prefer correct extension, and may copy mislabeled bytes to corrected sibling. New cleanup deletes only a mislabeled file with byte-identical sibling; errors are warnings. Trees derive from raw findings plus files; chains are filtered to `surfaced`. Their image lookup tries exact/lowercase ID and png/jpg/svg using `is_file`.
- Output: booleans and template-relative paths, tree dictionaries and surfaced-chain dictionaries.
- Composition: deliberate chain filtering changes count; it is not accidental loss. Top-level image existence means detectable nonempty PNG/JPEG, while new tree/chain image existence means only regular-file presence. See DF-2. Cleanup preserves a referenced corrected sibling under a stable filesystem; concurrent mutations are not synchronized.

### 10. Typst serialization, templates and publication

Source: `crates/tachi-core/src/report_data.rs:368-392,408-425,524-527`; `templates/tachi/security-report/main.typ:405-420`; `crates/tachi-cli/src/bin/report-data.rs:24-45`.

- Input: serde JSON scalar/array/map values and image/reference strings.
- Transform: Null→`none`; arrays→trailing-comma tuples (including singleton); maps→quoted-key dictionaries, empty map→`(:)`; booleans/numbers retain literals. Strings escape backslashes and quotes. Fixed binding names are generated by code, not interpolated from arbitrary metadata keys.
- Output: Typst source; template reads selected tier and findings, gates attribution on flag plus nonempty row vector, and renders empty findings explicitly.
- Composition: string escaping is one-way source serialization; the Typst parser is the intended back-transform. No numerical log/exp or timezone conversion exists. No temporal filtering occurs: assessment date is copied text, not a range computation. CLI writes bytes after generation, without running Typst or checking referenced images are decodable.

## Concrete violation candidates for independent reviewers

| ID | Trigger and composition consequence | Introduction / guard status | Suggested confirmation |
|---|---|---|---|
| DF-1 | Valid raw High finding plus `risk-scores.md` whose `## 2. Scored Threat Table` contains a nonempty unrelated/malformed table. Parser emits one row with empty ID/severity because missing columns default; `render_document_data` treats nonempty as valid Tier 2, replaces real findings, and emits zero severity counts with a blank finding. | New composition at `report_data.rs:164-168`; permissive parser at `findings.rs:158-178` predates window. Existing controls-stub guard does not guard risk rows. Source trace supports mechanism; no runtime reproduction in this phase. | Minimal fixture with header `Unexpected` and one row. Assert Tier 3/raw IDs survive or checked API rejects invalid risk input. Validate required columns and IDs before selecting Tier 2. |
| DF-2 | Tree or surfaced chain has a zero-byte/corrupt image sibling. New lookup accepts `is_file`, sets `has-image=true`, and passes bad path into template image rendering instead of text fallback; downstream PDF can fail. | New `report_data.rs:227-247,255-259`; stronger top-level image validation is elsewhere, not applied by this closure. Verify actual template image consumer before severity assignment. | Add zero-byte `S-1-attack-tree.png` for a parsed tree; compile resulting report and check fallback. Gate these images consistently with usable asset policy. |
| DF-3 | Existing but unreadable/invalid-UTF-8 optional risk/control artifact is treated exactly as absent; a checked command can succeed with a lower tier and silently changed risk posture. | New optional reads in `render_document_data` (`162,172`), unlike checked primary read at `31-35`. Lower-tier fallback may be intended for missing files; non-NotFound failures are not distinguished. | Reviewer assess contract and reproduce permission/UTF-8 failure. Propagate non-NotFound errors if optional artifact presence asserts authoritative tier. No severity assigned here. |

These are source-supported candidates, not independently reproduced findings. Do not inflate candidate status into P0. Do not flag intentionally completed empty controls as automatic data loss: explicit tests cover that contract (`crates/tachi-core/tests/report_document_contract.rs:249,311`).

## Transform completeness / invariant summary

| Forward transform | Consumer/back-transform | Status |
|---|---|---|
| Markdown rows → raw/risk/control structs | Tier-specific Typst dictionary keys | Keys align; risk validity gate is candidate DF-1. |
| Raw findings → selected tier | Counts/components/actions recomputed from selected rows | Internal selected-tier counts preserved; cross-document membership assumed. |
| Source-attribution records → buckets and catalog classification | Attestation references / framework matrices | Identity retained; catalog membership validation is separate. |
| MAESTRO Option count/evidence → typed state → string/label | MAESTRO template | Missing vs zero clean distinction preserved. |
| JSON strings/arrays/maps → Typst literals | Typst import and page functions | Required shape preserved by static inspection; no runtime compile claim. |
| Image bytes → extension/path → image renderer | File read / image decode | Top-level guarded; tree/chain candidate DF-2. |
| String → output file/stdout | Later Typst invocation | Write errors propagated; publication atomicity unchanged baseline. |

Clean traced compositions: checked primary parse errors propagate end to end; explicit empty attribution is preserved; severity/count/component recalculation tracks selected tier; MAESTRO missingness is not converted to clean; only surfaced chains publish; cleanup requires identical retained sibling. Temporal interval checks are not applicable to this path.


---

# correctness — Phase 3

<!-- Source artifact: state/reviewer_correctness_phase_3.md -->

> **Persona Profile — correctness**
> Correctness Hawk; Rust logic/edge cases; systematic enumeration; 30% agreement; phases 3/4/5/7. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Correctness Hawk — Phase 3 independent review

Score: **6.5/10**. Agreement intensity: 30%. Strategy: systematic enumeration. Mode: Precise for code. Recommendation: **request changes** for the three bounded report-generation regressions below. No P0/P1 claimed.

Reviewed pinned main `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112`. Read shared context, data-flow trace, root atlas, Overseer Phase 3 instructions and RTK instructions. No other reviewer output was opened and no findings were exchanged with reviewers. Available inherited model used; Opus was unavailable, so correlated-model limitations remain.

## Findings

### COR-1 [P2] Malformed scored table replaces valid threats with a blank finding

**Current citations:** `crates/tachi-core/src/report_data.rs:164-168`; `crates/tachi-core/src/parsers/findings.rs:158-178`; severity/count consequences at `crates/tachi-core/src/report_data.rs:268-276`; rendered consumer at `templates/tachi/security-report/main.typ:405-409`.

**Trigger:** Keep a real High S-1 finding in threats.md, then provide risk-scores.md containing:

```markdown
## 2. Scored Threat Table

| Unexpected |
|---|
| pending |
```

**Observed consequence:** Checked report-data succeeds, selects Tier 2, changes high-count from 1 to 0, and replaces the real finding with one dictionary whose id, component, threat, score and severity are all empty. A failed/intermediate generator artifact therefore silently suppresses substantive report content.

**Guard search:** Inspected the entire checked builder, risk parser and table parser, plus report_document_contract tests. The only risk-tier guard is nonempty vector length. parse_markdown_table emits a map for this row and parse_risk_scores_findings defaults every missing required field to empty. The control-stub guard is specific to the other tier. No risk header, identity or severity validation intervenes before replacement. Primary threats attribution validation does not validate this file.

**Introduction evidence:** Base `report_data.rs` had no render_document_data, risk-file selection or replacement. The permissive parser is preexisting, but this window newly composes its arbitrary nonempty output into the authoritative report findings list. `rtk git diff --no-compact dd3b293d81d358d1ae27424be83b720693539112 HEAD -- crates/tachi-core/src/report_data.rs` shows the new selection block.

**Confidence:** High. Runtime reproduced using a binary built from the pinned worktree, not the original dirty checkout.

**verification_command** (read-only for the prepared fixture; prints generated bindings):

```sh
rtk proxy /private/tmp/overseer-correctness-build/debug/report-data --target-dir /private/tmp/overseer-correctness-DpxDFG/malformed-risk --template-dir /Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004/templates/tachi/security-report
```

**Fix/regression:** Validate the scored-table column contract and nonempty finding identity/severity before selecting Tier 2; reject invalid input through the checked API or retain the earlier tier with an explicit diagnostic. Add a regression that a valid S-1 never becomes an anonymous empty finding after this malformed optional artifact appears. Also cover canonical headers plus empty/malformed data cells.

### COR-2 [P2] Controls header recognition treats unparsed nonempty residual tables as zero findings

**Current citations:** `crates/tachi-core/src/report_data.rs:306-340` (header-only acceptance); `crates/tachi-core/src/report_data.rs:172-177` (full replacement); `crates/tachi-core/src/compensating_controls.rs:126-130` (parser only visits named severity subsections); `templates/tachi/security-report/findings-detail.typ:190-203` (empty findings display).

**Trigger:** A valid threats.md contains High S-1; compensating-controls.md has a residual table directly under `## 2. Coverage Matrix` with all six required columns and a populated S-1 row, but no `### High Residual Severity` subsection. Example:

```markdown
## 2. Coverage Matrix

| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
|---|---|---|---|---|---|
| S-1 | Gateway | Impersonation | 8 | High | Missing |
```

**Observed consequence:** CLI succeeds and emits Tier 1, has-compensating-controls=true, total-findings=0, high-count=0 and findings=(). The template consequently displays “No findings to display” despite the nonempty threat and residual rows. This is not a complaint about intentionally supported completed empty assessments: the input contains a real residual row that the different parser contract failed to read.

**Guard search:** Inspected has_control_assessment end-to-end, parse_compensating_controls_md and the completed-empty/control-stub tests. The guard scans any matching header/separator inside Coverage Matrix, while the parser scans only four exact severity headings. The guard never proves that the recognized table is actually empty or that every data row was consumed. The checked API catches primary attribution errors, not this mismatch. The published output schema normally groups by severity; unsupported grouping should fail or preserve the earlier tier, not be silently reclassified as a completed zero-finding assessment.

**Introduction evidence:** Both the broad header acceptance and the report's Tier 1 replacement were added after dd3b293d. The subsection-only parser predates the window, but the new guard/consumer combination creates this silent success path. This shares a tier-validation theme with COR-1, but a fix confined to risk-row validation does not fix the separate controls completion detector.

**Confidence:** High for the reproduced data loss; Medium-High for exposure frequency because canonical generated files normally include severity headings.

**verification_command** (read-only for prepared fixture):

```sh
rtk proxy /private/tmp/overseer-correctness-build/debug/report-data --target-dir /private/tmp/overseer-correctness-DpxDFG/unparsed-controls --template-dir /Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004/templates/tachi/security-report
```

**Fix/regression:** Make recognition and extraction share the same table contract, or require zero data rows before classifying an unparsed residual table as an empty assessment. Preserve the existing legitimate empty-assessment behavior. Add the populated flat-table fixture and a changed/unsupported severity-heading fixture; require either extraction of S-1 or a checked error, never successful zero-finding output.

### COR-3 [P2] Empty attack-tree images override the usable source fallback

**Current citations:** `crates/tachi-core/src/report_data.rs:227-246`; analogous chain use at `crates/tachi-core/src/report_data.rs:255-258`; direct image consumer `templates/tachi/security-report/attack-path.typ:69-81`.

**Trigger:** A parsed High S-1 attack-tree Markdown file contains valid Mermaid source, while `attack-trees/S-1-attack-tree.png` exists with zero bytes (for example, interrupted image generation). The fixture contains exactly this case.

**Observed/traceable consequence:** report-data succeeds and emits has-image=true pointing at the zero-byte file. The template enters image(...) and bypasses its Mermaid raw-source fallback. A zero-byte PNG is not a usable image, so downstream PDF generation is handed invalid mandatory image input instead of the available fallback. I verified the generated binding and source branch; I did not run Typst compilation in this phase.

**Guard search:** Read the image lookup closure, attack-tree parser, attack-path template and top-level image helpers. The new closure checks only is_file and stops at the first candidate, so it can also mask a later valid JPG/SVG. The nonempty/signature checks used for other report image stems in assets.rs are not called here. The template tests only the boolean and a nonempty path; it has no decoder-error recovery. The attack parser checks severity and Mermaid availability, which does not validate the sibling image.

**Introduction evidence:** Base report_data.rs did not bind attack trees/chains. This window introduced the is_file selection closure and has-image binding; it also added the explicit raw Mermaid fallback to attack-path.typ. Thus the new path prevents its own fallback on partial image artifacts.

**Confidence:** High for binding/branch behavior; downstream compile failure is source-traced, not runtime-compiled.

**verification_command** (read-only for prepared fixture):

```sh
rtk proxy /private/tmp/overseer-correctness-build/debug/report-data --target-dir /private/tmp/overseer-correctness-DpxDFG/empty-tree-image --template-dir /Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004/templates/tachi/security-report
```

**Fix/regression:** Skip unusable images while searching candidates, with format-appropriate validation for supported raster/SVG files. At minimum reject empty files and keep the text fallback. Add empty PNG, invalid PNG, empty PNG plus valid alternate extension, and normal image cases. Confirm a malformed optional diagram cannot prevent the report from compiling.

## Verification performed

- Built pinned CLI offline successfully: `rtk cargo build --offline --locked -p tachi-cli --bin report-data --target-dir /private/tmp/overseer-correctness-build -j 2`. Cargo reported 31 crates compiled and completion in 42.86 seconds.
- Private fixture directory: `/private/tmp/overseer-correctness-DpxDFG`. Baseline emitted Tier 3, one High finding S-1; all three candidate outputs are saved as actual.typ under their scenario directories. No product files changed.
- Initial fixture invocation used incorrect --target flag and exited 2; corrected to the source-declared --target-dir before collecting the successful reproductions above. No conclusion relies on the failed invocation.
- Knowledge graph queried first and confirmed stale/missing new report symbols; current source and base diffs supplied finding evidence. Memory registry was orientation only (graph exclusions), not evidence for runtime behavior.

## Rejected or deferred candidates

- Do not flag zero MAESTRO counts as clean: the shared typed classifier correctly requires explicit clean evidence and observed findings override stale summaries.
- Do not flag all empty controls as invalid: existing tests deliberately preserve completed empty assessments; COR-2 is specifically an unparsed nonempty table.
- Do not flag raw attribution rows differing from selected residual/risk findings without a cross-document identity contract. Their inherent-threat coverage role can be intentional.
- Primary attribution errors now propagate through the checked CLI and infographic builder; the legacy report API explicitly emits panic source. The absence of catalog-membership validation is not established as a new regression.
- Optional UTF-8/read failures and partial tier identity reconciliation merit later design scrutiny, but I do not add findings without a stronger new-contract argument.
- MAESTRO header fence exclusion, observed evidence count/severity updates, state rendering, singleton Typst tuple serialization, nested attribution parsing, SARIF finding identity attachment and surfaced-chain filtering were inspected without a stronger additional current regression established.

## Coverage limits

Focused independent review covered Rust report composition, parser/schema boundaries, selected MAESTRO and SARIF changes, downstream Typst consumers and relevant tests, with three real CLI fixtures. Did not exhaust frontend/scaffold auth or database behavior, CI/workflow triggers, catalog publication transactions, all adapters/prompts or every changed file. Did not run the full test suite, production agent generation, or compile PDFs. Findings are bounded to the pinned main range; no issue is asserted merely from stale graph output or an unverified historical note.


---

# architecture — Phase 3

<!-- Source artifact: state/reviewer_architecture_phase_3.md -->

> **Persona Profile — architecture**
> Architecture Critic; cross-boundary contracts; backward reasoning; 50% agreement; phases 3/4/5/7. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 3 — Architecture Critic

Agreement intensity: 50%. Reasoning: backward from trustworthy report output, consistent adapter results, and complete baseline publication. Review mode: Precise for implementation/configuration. Score: **7/10**. Recommendation: **Request changes for report input validation; retain the architectural direction.**

Reviewed resulting revision `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112`, covering the seven-commit window. No other independent reviewer output was read. No product files were modified. Findings below are source-traced defects, not claims of runtime reproduction or a passing test suite.

## Desired outcomes and backward trace

The final findings cards, counts, component distribution, and remediation must all describe a valid selected assessment. Tracing backward from `main.typ:405-409` reaches one selection point in `report_data.rs:162-181`. Deriving all downstream summaries from the selected rows is a good improvement, but that selection point needs a validity contract: a nonempty parser result is not evidence of a valid assessment.

A failed image render must preserve the useful attack-path analysis. Tracing backward from `attack-path.typ:72-80` reaches a second asset-selection policy, implemented independently from `assets.rs::choose_image`. It treats mere file existence as successful image availability and therefore bypasses the new Mermaid fallback.

Checked errors are propagated consistently through the CLI, shell bridge, and MCP path. Catalog regeneration stages inputs and outputs, rejects renderer failures, checks source drift before publication, snapshots destination bytes, and attempts rollback including the failing destination. These safeguards are real; I am not claiming render failure corrupts existing baselines or that publication promises crash-safe atomicity.

## Findings

### ARC-1 — P1 — A malformed optional risk table replaces valid threat findings with blank rows

**Labels:** [CODE-TRACE] [DEFINITE-DEFECT] [PRECISE]. Confidence: **High** for the data-loss mechanism; severity assumes reports are relied on for security prioritization. No runtime reproduction performed.

**Exact source:** `crates/tachi-core/src/report_data.rs:164-168` selects Tier 2 and replaces the full findings vector whenever parsed risk rows are nonempty. `crates/tachi-core/src/parsers/findings.rs:164-178` constructs a `RiskScoreFinding` from every table row, defaulting missing ID, severity, component, and score columns to empty strings. `crates/tachi-core/src/report_data.rs:268-276` subsequently recomputes counts from those replacement rows. The consumer at `templates/tachi/security-report/main.typ:405-409` receives those rows directly; `findings-detail.typ:182-213` does not reject invalid rows.

**Concrete trigger:** A valid `threats.md` containing a High `S-1` recommendation, alongside this syntactically readable `risk-scores.md`:

```markdown
## 2. Scored Threat Table

| Unexpected |
| --- |
| generator failure |
```

`parse_markdown_table` produces a nonempty map, the risk parser produces one all-empty finding, and `render_document_data` emits `data-source-tier = 2`, `has-risk-scores = true`, one blank finding, and zero Critical/High/Medium/Low counts. The valid `S-1` disappears from the detail table and component distribution. The checked command still succeeds. Attribution may still list the raw finding, so this can also create an internally inconsistent report rather than an obvious failure.

**Base comparison:** The permissive risk parser predates the window. The defect is its new composition into the canonical report builder: base `report_data.rs` never reads `risk-scores.md` or replaces the findings vector, whereas the reviewed revision adds `render_document_data` and this selection branch. This is not a claim that the baseline already emitted a complete, correct findings document.

**Safeguards considered:** Primary threat parsing is checked before generation; that does not validate the optional risk file. Controls have a dedicated `has_control_assessment` guard and control-stub regressions in `report_document_contract.rs`; neither applies to the risk branch. Final count recomputation is internally correct for the wrong rows and therefore cannot detect the loss. I searched the report contract tests and relevant parser/renderer paths for malformed risk-table validation and found no guard on this path.

**Read-only verification_command:** `rtk proxy sh -c 'sed -n "158,179p" crates/tachi-core/src/parsers/findings.rs; sed -n "161,181p" crates/tachi-core/src/report_data.rs; sed -n "261,276p" crates/tachi-core/src/report_data.rs'`

**Fix and regression:** Validate the required header/row contract before selecting a tier, ideally returning a typed validated assessment and distinguishing absent, incomplete, and invalid inputs. The checked API should reject malformed existing risk data with its filename, or deliberately retain Tier 3 with a visible diagnostic. Add a regression for the fixture above asserting that a successful result never drops `S-1` or reports zero High findings. Also test partially populated rows and invalid severities; checking only nonempty IDs is insufficient.

### ARC-2 — P2 — An empty attack-tree image disables the text fallback and breaks PDF generation

**Labels:** [CODE-TRACE] [DEFINITE-DEFECT] [PRECISE]. Confidence: **High** for the producer/consumer mismatch. The Typst compilation failure is inferred from feeding an empty image file to the explicit `image` call; no compilation was run in this phase.

**Exact source:** `crates/tachi-core/src/report_data.rs:227-239` selects the first `png`, `jpg`, or `svg` path satisfying only `is_file()`. Lines `243-247` set `has-image` from that selection. `templates/tachi/security-report/attack-path.typ:69-80` calls `image()` whenever that flag is true and only renders the Mermaid source in the other branch. The same lookup is reused for chains at `report_data.rs:255-259`.

**Concrete trigger:** A parsed High/Critical attack tree for `S-1`, with valid Mermaid source, plus an empty `attack-trees/S-1-attack-tree.png` left by a failed diagram command. That file wins selection even if a valid JPG exists. The generated binding sets `has-image=true`, making the PDF consumer attempt to decode empty bytes instead of using available text. Report-data generation itself returns success, but PDF rendering fails.

**Base comparison:** Base `report_data.rs` does not bind attack trees or chains. This window adds both the image-selection closure and canonical attack-tree output, activating this failure path. The template's image call existed previously, but the new producer does not establish its usable-image precondition.

**Safeguards considered:** `assets.rs:128-168` rejects empty candidates and uses image signatures for the top-level report images. That function is not used by this closure. The checked report-data API does not compile Typst or validate these bytes. The newly added Mermaid branch is a useful fallback but is unreachable for an existing unusable file.

**Read-only verification_command:** `rtk proxy sh -c 'sed -n "227,259p" crates/tachi-core/src/report_data.rs; sed -n "69,81p" templates/tachi/security-report/attack-path.typ; sed -n "128,168p" crates/tachi-core/src/assets.rs'`

**Fix and regression:** Centralize image usability/selection rather than maintain separate top-level and attack-diagram policies. Skip empty/invalid candidates and continue to other formats; preserve SVG support explicitly. Add an integration fixture with valid Mermaid source and empty PNG, asserting the generated report selects the fallback and compiles. Add empty-PNG-plus-valid-JPG coverage to prove selection continues instead of stopping at the first file.

## Architectural strengths and non-findings

- The checked report API is threaded through the affected CLI, shell bridge, and MCP paths. The legacy API deliberately emits an unrenderable panic document on error; I found no error swallowing in the newly updated adapter handoffs themselves.
- MAESTRO report coverage and grouped findings use the same extractor, and explicit evaluation state preserves the distinction between clean and not evaluated. I did not treat inherent-attribution counts versus residual-findings counts as automatically erroneous; those describe different projections by design.
- Catalog publication includes staging, renderer-version checking, complete-PDF checks, source fingerprint rechecks, and ordinary-error rollback. Crash recovery, interprocess serialization, and stale executable provenance remain broader limitations, not promoted here without a demonstrated in-scope failure.
- The Next.js middleware-to-proxy migration preserves the existing authentication and cookie logic; the Prisma migration adds own-profile RLS and explicit PostgreSQL checks. The existing privileged Prisma connection is not, on its own, proof of a new cross-tenant endpoint exposure. I found no concrete frontend migration regression in the inspected files.

## Coverage and limits

Read the shared context/data-flow trace and atlas; inspected relevant changed implementation and tests for report building, parser contracts, attack-tree rendering, source attribution/SARIF propagation, MAESTRO integration, catalog staging/publication, shell/MCP adapters, and Next.js/Supabase/Prisma migration. Graph discovery was attempted first; the shared context warns it describes an older source state, so pinned source is authoritative. The memory registry was used only for discovery/exclusion guidance (`MEMORY.md:58-67`), not as proof of current behavior.

This was not exhaustive review of every changed line, generated lockfile, taxonomy record, binary PDF, prompt, or CI workflow. I did not execute the full workspace tests, a browser workflow, PostgreSQL migration, or renderer. Read-only commands support the cited control flow; proposed reproductions and regressions remain to be executed by a later verification phase. No temporal range filtering was found in the inspected runtime paths; the review's 48-hour window is a commit-selection criterion, not application date filtering.


---

# security — Phase 3

<!-- Source artifact: state/reviewer_security_phase_3.md -->

> **Persona Profile — security**
> Security Auditor; filesystem/auth trust boundaries; adversarial simulation; 30% agreement; phases 3/4/5/7. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 3 independent review — Security Auditor

Score: **7.5/10**. Agreement intensity: 30%. Strategy: adversarial simulation. Mode: Precise. Recommendation: **request targeted corrections** for two P2 regressions. No P0/P1 vulnerability established.

Reviewed pinned main `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112` in `.worktrees/overseer-main-20261004`. Read context, Phase 2 trace, codemap, relevant shared diff sections, actual source/consumers/tests, and Overseer Phase 3 instructions. Did not read other reviewers. Graph discovery returned stale source positions; pinned files are authoritative. Memory informed cleanup-contract discovery only; findings were rechecked against current source and base.

## Findings

## SEC-1 [P2] [EXISTING_DEFECT] Cleanup deletes the sole backing image when the retained sibling is a symlink

- **Location:** `crates/tachi-core/src/assets.rs:43` and `:51`; equality helper `:61-64`. CLI invocation: `crates/tachi-cli/src/bin/report-data.rs:24-26`.
- **Trigger:** A PNG image is stored as `threat-risk-funnel.jpg`, and correctly named `threat-risk-funnel.png` is a relative symbolic link to that JPG. Invoke report-data with `--cleanup-mislabeled-images`.
- **Consequence:** Detection selects the PNG symlink because its bytes match its extension. Both `image_format` and `fs::read` follow it, so byte comparison succeeds. Cleanup removes the JPG regular file, leaving the retained PNG dangling. The only image bytes disappear; the generated image binding now references a nonexistent image. This is permanent local data loss.
- **Guard search:** Inspected `detect_images`, `choose_image`, `image_format`, `files_are_identical`, and CLI order; searched assets/report/CLI tests for symlink checks. There is no `symlink_metadata` or retained-file-independence guard. Catalog regeneration has separate symlink rejection, but this CLI path does not call it. Equality, opt-in flag, fixed stems, and nonfatal deletion errors exist; none prevent this successful harmful unlink. Hard links do not reproduce this defect because the retained link preserves bytes.
- **Introduction:** Cleanup is new in `35be8778c3aacea179224676d42c45a6f50a33b7` (blame verified). Base `dd3b293d` has detection but no cleanup deletion; the alias was usable.
- **Evidence:** A standalone Rust harness directly includes pinned `assets.rs`, creates private temporary files, calls detection and cleanup, and exits successfully with:

  ```text
  before: jpg=true png=true binding=Some("./threat-risk-funnel.png")
  after: jpg=false png=false dangling_symlink=true
  ```

  Harness: `/tmp/tachi-sec-review.VKfdxm/repro.rs`; fixture: `/tmp/tachi-sec-review.VKfdxm/fixture`. The fixture contains a PNG signature plus sentinel bytes because production only checks the signature; deletion is independent of image decoding and applies equally to a complete valid PNG.
- **Confidence:** High; actual pinned helper reproduced.
- **Fix/test:** Skip cleanup unless candidate and retained counterpart are ordinary non-symlink files, or otherwise prove the retained path survives unlink. Add Unix regression cases for PNG/JPG and JPEG/PNG symlink aliases, asserting retained bytes and selected bindings stay readable. Preserve ordinary regular-file duplicate cleanup.
- **Read-only `verification_command`:**

  ```sh
  rtk proxy sh -c 'test ! -e /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.jpg && test -L /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.png && test ! -e /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.png && readlink /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.png'
  ```

  This checks the retained reproduction fixture without deleting anything. The harness itself mutates only private fixtures and is not the read-only verification command.

## SEC-2 [P2] [EXISTING_DEFECT] Attack-tree IDs select images outside the report directory

- **Location:** `crates/tachi-core/src/report_data.rs:232-244`. Input acceptance: `crates/tachi-core/src/attack_trees.rs:111` and `:124-130`; consumer: `templates/tachi/security-report/attack-path.typ:69-74`.
- **Trigger:** A tree artifact contains `# Attack Tree: ../../private/private -- Unrelated image`, high risk metadata, and a Mermaid block. A neighboring directory contains `private/private-attack-tree.svg`. No matching raw finding is necessary: supplied high severity and this ID are accepted. Run normal report-data on `report/`.
- **Consequence:** The new lookup concatenates unvalidated ID and image suffix; `is_file` resolves parent components. The generator emits `has-image=true` and an image path outside the selected report. The template then passes it to `image()` instead of displaying Mermaid fallback. Lower-trust Markdown can thus select a known neighboring image, contaminating a report or disclosing that image when the PDF is shared.
- **Scope limits:** Local generation, not an exposed HTTP service. Attacker must influence report Markdown, know an existing image with the expected suffix/extension, and have that image within the compiler's broader project root. No arbitrary text-file read, network exfiltration, shell execution, or RCE established. [Typst path documentation](https://www.typst.app/docs/reference/foundations/path/) confirms the project-root restriction. The escape here is from the selected report into an adjacent directory inside that root. The Rust generator itself probes existence before compilation.
- **Guard search:** Read tree parser, image closure, relative-path helper, consumer, and tests. The alternative `ID: title` heading branch has an alphanumeric/hyphen check, but `Attack Tree:` and metadata `Finding ID` branches do not. No ID grammar, canonical containment check, or matched-finding requirement guards lookup. `relative_path` preserves parent components. No Typst `eval` is required.
- **Introduction:** Lookup and wiring added in `c933cd4bb6b7a99a1e07ce56f940dbccab0d6cc4` (blame verified). Base `dd3b293d` did not project tree IDs into file lookup or emit tree-image data in report-data. Permissive parsing predates the window; its new filesystem composition introduces this regression.
- **Evidence:** Built actual pinned CLI offline with `CARGO_TARGET_DIR=/private/tmp/tachi-overseer-target cargo build --offline --locked -p tachi-cli --bin report-data`, exit 0. Private fixture `/tmp/tachi-sec-path.7Qm3Cu` generated:

  ```text
  #let has-attack-trees = true
  #let attack-trees = (("id": "../../private/private", ... "has-image": true,"image-path": "../../../report/attack-trees/../../private/private-attack-tree.svg", ...),)
  report-data.typ generated
  ```

  Selected SVG is outside `report/`. PDF compilation was not executed; the consumer is directly traced to `image(img-path, ...)`.
- **Confidence:** High for actual path-selection escape; medium for disclosure because compilation and sharing depend on caller workflow/root.
- **Fix/test:** Validate IDs against intended finding-ID grammar before filename use. Canonicalize candidate and image directory and require containment, including symlink handling. Invalid IDs should use safe text fallback or return a contextual error. Cover parent-relative and absolute IDs, symlinked images, valid case variations, and a neighboring report image that must never be selected.
- **Read-only `verification_command`:**

  ```sh
  rtk proxy sh -c '/private/tmp/tachi-overseer-target/debug/report-data --target-dir /tmp/tachi-sec-path.7Qm3Cu/report --template-dir /tmp/tachi-sec-path.7Qm3Cu/templates/tachi/security-report | rg "^#let (has-attack-trees|attack-trees)"'
  ```

  Fixture has no top-level image stems, so detection cannot invoke its legacy correction-copy behavior; no output path or cleanup flag is supplied.

## Positive findings and rejected escalation candidates

- Typst serialization escapes backslashes and quotes, dictionary keys are quoted, binding names are static, and inspected consumers render strings/raw Mermaid rather than evaluating Markdown as code. No demonstrated source injection.
- Cleanup uses fixed stems and byte equality; SEC-1 is retained-copy dependence, not arbitrary outside-root deletion.
- Catalog regeneration rejects input symlinks, stages renders, verifies PDF envelopes/provenance, rechecks inputs, and attempts rollback. No concrete external-attacker exploit established. Direct publication writes are not crash-atomic; that is a reliability limitation, not a demonstrated security escalation.
- Next.js proxy preserves old matcher, `getUser()` verification, and cookie logic. RLS migration has own-ID SELECT/INSERT/UPDATE policies. Tests cover anonymous read/insert denial, own read/insert, cross-user update, and forged insertion. Policy shape matches [Supabase guidance](https://supabase.com/docs/guides/database/postgres/row-level-security). No auth-bypass regression established.
- New `lib/prisma.ts` does not propagate user claims, but no scaffold application callers were found. Future callers need authorization; an unused privileged client is not an existing cross-tenant endpoint.
- Checked report-data now propagates primary attribution failures through CLI/MCP. Phase 2 optional-artifact/tier candidates were not independently elevated to hostile-service vulnerabilities here.

## Coverage and limits

Focused on file/input/output compositions, cleanup/catalog publication, Typst sinks, and auth/RLS. Also inspected permission checker and MCP diff. Not an exhaustive audit of all 48-hour files. No Python or product edits. No live database, browser session, hostile SVG decoder, production deployment, full workspace suite, or PDF compile exercised. Offline pinned CLI build and private-fixture probes completed; other evidence is static. Phase 3 ends here.


---

# devils_advocate — Phase 3

<!-- Source artifact: state/reviewer_devils_advocate_phase_3.md -->

> **Persona Profile — devils_advocate**
> Devil’s Advocate; challenge assumptions and tests; analogy; 20% agreement; phases 3/4/5/7. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 3 — Devil’s Advocate

Persona: Devil’s Advocate, 20% agreement; reasoning strategy: analogical failure-pattern reasoning. Score: **6/10**. Recommendation: **request changes** for DA-1; close DA-2 before treating the catalog gate as protection for published examples.

Scope: pinned `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112`. Read context, Phase 2 trace, codemap, pertinent diff, core report/control code, catalog checker/publication, workflow and tests. Graph was consulted first but its older symbols were not authoritative. No other reviewer artifacts read. Product source untouched; temporary reproduction fixtures only. Fresh offline build of report-data and catalog-drift passed using this pinned worktree and `/private/tmp/tachi-overseer-target`.

## Findings

## DA-1 — [P1] / BUG / false-negative report from a partial controls artifact

**Location:** `crates/tachi-core/src/report_data.rs:289` (premature acceptance); `crates/tachi-core/src/report_data.rs:177` (replacement); `crates/tachi-core/src/report_data.rs:276` (zero total). Supporting parser: `crates/tachi-core/src/compensating_controls.rs:257` and `:294`.

**Trigger:** a valid High scored finding `S-1`, accompanied by a partially generated or manually assembled compensating-controls.md containing only:

```markdown
## 3. Control Details

### Authentication

**Status**: Missing | **Effectiveness**: None
```

There is no residual assessment table, numeric coverage summary, or assessment completion evidence. The old parser still produces one control descriptor. The newly introduced guard accepts any nonempty `controls`; Tier 1 then replaces the valid risk finding with the empty residual vector. The report emits High=0, total=0, findings=(), controls=true, tier=1. Missing authentication becomes evidence for an apparently empty risk assessment.

**Observed verification:** freshly built report-data reproduced exactly `#let high-count = 0`, `#let total-findings = 0`, `#let data-source-tier = 1`, `#let has-compensating-controls = true`, `#let findings = ()`; exit success. Fixture retained at `/private/tmp/tachi-da-partial.0QxF1C`.

**Analogy:** accepting a partially filled reconciliation ledger as a completed balance statement. Evidence that a control was described does not establish that all residual findings were assessed.

**Guard search:** read the complete `has_control_assessment` function and parser, plus `completed_empty_controls_preserve_the_assessment_and_control_metadata` and `control_stubs_do_not_erase_valid_risk_findings_but_empty_assessments_are_retained`. The explicit residual-header and complete-numeric-summary guards are real but bypassed at line 289. The legitimate completed-empty test contains those additional signals; this reproduction contains neither. Existing stub tests never include a parseable control descriptor. No claim that every empty residual assessment is erroneous.

**Introduction:** base-to-head diff adds `render_document_data` and this acceptance/replacement path; the compensating-controls parser itself predates the window. The defect is the new composition, not the old parser.

**Confidence:** High; runtime reproduced, exact branch traced. P1 because a success-path security report silently removes known High findings for a normal incomplete-artifact case.

**verification_command (read-only replay):**

```sh
rtk proxy /private/tmp/tachi-overseer-target/debug/report-data --target-dir /private/tmp/tachi-da-partial.0QxF1C --template-dir templates/tachi/security-report
```

**Fix/regression:** distinguish control inventory from completed residual assessment. Without residual findings or explicit completed-empty signals, retain Tier 2/3 or reject incomplete authoritative input. Add a regression using valid S-1 High risk data and the controls-only fragment above; require the finding and High=1 to survive, or checked failure. Retain completed-empty assessment tests.

## DA-2 — [P2] / BUG / published PDF corruption passes catalog drift

**Location:** `crates/tachi-core/src/catalog_drift.rs:81` excludes published PDFs; `:193` checks only baseline hashes. Publication at `:313`–`:317` updates both copies.

**Trigger/consequence:** corrupt `examples/agentic-app/sample-report/security-report.pdf` while preserving its `.baseline` sibling. The new gate still passes because the reader-facing companion is neither an input nor a registered checked output.

**Observed verification:** copied required inputs into `/private/tmp/tachi-da-catalog.5jEfA2`. Initial catalog-drift check passed; after replacing only the sample-report published PDF with plain text `corrupt published PDF`, the second check also passed. The private probe completed successfully. Current committed maestro-reference and sample-report companions match their baselines; this finding concerns a reproduced gate false-negative, not currently corrupt committed PDFs.

**Guard search:** read rendering_inputs, check, regenerate, workflow and catalog tests. Regeneration updates companions, but check only validates baseline paths. The render-failure preservation test covers a different invariant. Its fixture has different companion and baseline bytes while expecting check success.

**Introduction:** catalog-drift module and workflow are new since base dd3b293d. Published companion maintenance is part of the new contract but only baseline copies receive hash verification.

**Confidence:** High; runtime reproduced. P2 because independent companion modification is required and committed copies currently agree. Analogy: checking a backup checksum while serving an unchecked live copy.

**verification_command (read-only replay):**

```sh
rtk proxy /private/tmp/tachi-overseer-target/debug/catalog-drift --check --root /private/tmp/tachi-da-catalog.5jEfA2
```

**Fix/regression:** register intentionally published companion paths and verify their bytes/hash against baselines, including missing companions. Keep generated-output exclusion from input hashing. Add corrupt-companion and missing-companion negative tests.

## Recommendation and coverage limits

Score **6/10**; **request changes**, prioritizing DA-1. No P0 evidence. Private staging, removal of copied baselines before rendering, version and PDF signature checks, completion of all renders before publication, and final input comparisons are real safeguards. Attribution errors also fail closed.

Further catalog crash/concurrency/provenance investigation is **unverified**: sequential in-place writes and best-effort rollback at catalog_drift.rs:351–:361 warrant fault injection before describing publication as crash-atomic. Source hashes describe --root while execution uses compiled code; stale-binary provenance was not reproduced and is not an additional finding. The companion-corruption probe above did complete; remaining investigation did not.

No full workspace test suite, Typst compilation, fault injection, concurrency stress, external advisory research or live product evaluation. Golden equality establishes consistency with regenerated baselines, not independently correct semantics: a faulty builder can regenerate consistent false-negative reports. DA-1 provides independent behavioral evidence. Stop at Phase 3.


---

# code_quality — Phase 3

<!-- Source artifact: state/reviewer_code_quality_phase_3.md -->

> **Persona Profile — code_quality**
> Code Quality Auditor; source and schema contracts; systematic enumeration; 40% agreement; phases 3/4/5/7. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 3 — Code Quality Auditor

Persona: Code Quality Auditor; agreement intensity 40%; systematic enumeration. Mixed review, precise for executable contracts. Independent: no other reviewer outputs read. Source pinned at `8df554e884b1e5dd24146111a965597eff5f4779`; base `dd3b293d81d358d1ae27424be83b720693539112`.

**Score: 7/10. Recommendation: request changes for the two bounded compatibility defects below.** No P0/P1 claim is made. These are source-traced findings, not executed reproductions.

## Findings

## CQ-1 — [P2] [EXISTING_DEFECT] Historic attribution IDs silently acquire different category meanings

**Current locations:** `schemas/taxonomy/owasp.yaml:471-473` replaces LLM05 with Data and Model Poisoning; `crates/tachi-core/src/threats_sarif.rs:170-179` constructs a 2026 reference from every primary LLM ID without inspecting input provenance. Related paths: `crates/tachi-core/src/parsers/findings.rs:293-339`, `crates/tachi-core/src/report_data.rs:36-51`.

**Trigger:** Reprocess an existing 2025-taxonomy threats document with an explicit `{taxonomy: owasp, id: LLM05, relationship: primary}` output-handling citation. A concrete previously supported artifact is `git show dd3b293d:tests/scripts/fixtures/source_attribution/valid_multi_record.md`: schema 1.5, dated 2026-04-20, finding LLM-5 explicitly says unsanitized LLM output enters an HTML sink, and cites LLM05 primary.

**Consequence:** The same historic evidence still parses and passes catalog membership (LLM05 remains a valid ID), but it is counted against the new poisoning category and exported with the current LLM-05 interpretation. There is no diagnostic or migration decision. This changes the compliance meaning of old evidence while retaining finding identity. Editing the repository's current examples does not migrate users' saved reports.

**Guard search:** Read source-attribution parsing, `validate_source_attribution`, coverage projection/aggregation, checked report builder, SARIF attachment, schema, and the semantic-contract/backward-compatibility tests. Searches for `version|2025|legacy|migration` in these runtime paths found no taxonomy-version dispatch or historic-ID migration. Validation only checks ID membership, so the colliding ID is accepted. Current backward-compatibility tests compare regenerated current fixtures; the old fixture was itself updated. The schema-version bump to 1.9 adds assets but the parser does not consult input schema_version when resolving attribution. The roadmap explicitly says to preserve historical references; there is no exemption documented for saved report inputs.

**Base comparison:** At dd3b293d the exact catalog key LLM05 meant Improper Output Handling and the saved fixture was consistent. This window reuses the key for a different category and adds forced 2026 SARIF attachment. The concern is not the external accuracy of the 2026 list: it is a collision between two meanings demonstrably present in base/current source.

**Confidence:** High in the source trace; medium in priority because prevalence of saved pre-migration reports is not measured. Static verification only; no claim of runtime test execution.

**verification_command (read only):**

```sh
rtk git diff dd3b293d -- schemas/taxonomy/owasp.yaml crates/tachi-core/src/threats_sarif.rs
```

Additional concrete input retrieval: `rtk git show dd3b293d:tests/scripts/fixtures/source_attribution/valid_multi_record.md`.

**Fix/test:** Preserve an explicit taxonomy edition in inputs, or use a documented legacy-input migration/rejection policy. Do not infer category changes from arbitrary threat IDs or blindly remap new reports. Freeze the base fixture unchanged and assert regeneration either preserves 2025 category semantics, performs an explicit edition-aware migration to LLM10, or fails with an actionable ambiguity message. Test multiple colliding categories and ensure genuinely current LLM05 poisoning remains LLM05.

## CQ-2 — [P2] [EXISTING_DEFECT] New nested attribution parser rejects the documented default relationship

**Current locations:** `crates/tachi-core/src/parsers/findings.rs:493-498`; underlying record at `crates/tachi-core/src/parsers/findings.rs:63-67`. Contract: `schemas/finding.yaml:277-290`. Existing flat parser defaults at `crates/tachi-core/src/parsers/findings.rs:598-600`.

**Trigger:** A supported nested attribution block contains a valid record with its optional relationship omitted:

```yaml
OI-1:
  source_attribution:
    - {taxonomy: owasp, id: LLM10}
```

Place this in a numbered Source Attribution YAML section or the newly supported `**Source Attribution**:` block, alongside a normal recommendation row.

**Consequence:** `serde_yaml::from_str::<BTreeMap<String, Entry>>` deserializes directly into `SourceAttributionRecord`, whose `relationship: String` has no serde default. Deserialization therefore returns a missing-field error labeled malformed YAML. The checked report, infographic, and SARIF paths propagate this error and reject an otherwise schema-valid input. The same record in the flat representation succeeds and acquires `primary`, making acceptance depend on representation rather than finding semantics.

**Guard search:** Read both nested and flat extractors, record derives, schema default, checked wrappers, and new attribution tests. `Default` on the Rust struct does not instruct serde to default absent fields; there is no `#[serde(default = ...)]` or postparse normalization before the deserialize error. Relationship enum checks run only after deserialization, so they cannot repair it. The schema expressly says the parser injects primary when absent.

**Base comparison:** Base had the flat parser's primary default and did not support this nested form. This window introduces a nested counterpart that violates the established record contract. In numbered sections the previously ignored nested input now causes a hard error, so the change is externally observable; this is a new-path defect, not an allegation that base already supported nested attribution correctly.

**Confidence:** High, by direct Rust/serde source trace. No runtime reproduction executed.

**verification_command (read only):**

```sh
rtk proxy awk '(NR>=63 && NR<=68)||(NR>=490 && NR<=509)||(NR>=590 && NR<=601) {printf "%d:%s\n",NR,$0}' crates/tachi-core/src/parsers/findings.rs
```

**Fix/test:** Give the deserialized relationship the same explicit primary default as the flat parser, while preserving rejection of unsupported explicit values. Test flat and nested representations with relationship omitted, primary, related, derived, and an invalid value through both parsing and a checked output boundary.

## Enumeration, positives, and limits

Read the shared context, Phase 2 trace, root atlas, relevant changes.diff sections, current/base OWASP catalog contracts, changed active agent/adapter semantic examples, source-attribution parsers, report coverage entry point, threat/risk SARIF builders, shell integration, schemas, and focused semantic/roundtrip/backward-compatibility tests. Graph-first search was performed; its older source positions were used only for orientation, and pinned files supplied all citations.

The new threat SARIF attachment correctly matches finding fingerprints rather than the shared OI/LLM rule ID. Explicit empty attribution remains distinguishable from absent evidence. Checked output paths preserve malformed-input errors. Context-specific active-example tests and negative token mutations are useful coverage.

Investigated but not promoted: risk SARIF derives `owasp-reference` from OI/MI prefixes, so an OI vector-filter finding with primary LLM09 also has LLM10 in that field. The field's intended relationship semantics were insufficiently explicit to claim a new defect; LLM10 is a legitimate related citation in the new fixture, and the mechanism predates the window. Similarly, model-theft example category choices require external domain validation and were not asserted as errors.

This focused pass is not an exhaustive audit of the whole 48-hour diff. It does not independently verify catalog publication rollback, filesystem boundary security, frontend/Prisma behavior, compiled PDF output, external taxonomy publication accuracy, binary artifacts, complete CI, or live-agent behavior. No product edits or Python execution occurred. No other persona state was read. Phase 3 ends here.


---

# pipeline — Phase 3

<!-- Source artifact: state/reviewer_pipeline_phase_3.md -->

> **Persona Profile — pipeline**
> Pipeline Reviewer; CI/build portability and provenance; checklist verification; 30% agreement; phases 3/4/5/7. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Pipeline Reviewer — Phase 3 independent review

Agreement intensity: 30%. Strategy: checklist specialist. Score: **7.5/10**. Recommendation: **targeted corrections requested**; no P0/P1 finding established by this review.

Pinned result: `8df554e884b1e5dd24146111a965597eff5f4779`. Comparison base: `dd3b293d81d358d1ae27424be83b720693539112`. Review mode: Precise. No other reviewer outputs were read.

## Findings

### PIPE-1 — [P2] [DEFECT] [STATICALLY VERIFIED]: new binary tests import Unix APIs on every platform

- Current location: `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:203` (module enabled by `#[cfg(test)]` at line 200; fake executable permission calls at 215–219).
- Trigger: compile this new binary test target on Windows, for example `cargo check -p tachi-cli --tests --target x86_64-pc-windows-msvc` on a provisioned Windows toolchain.
- Consequence: `std::os::unix::fs::PermissionsExt` does not exist for that target, so CLI test compilation fails before portable extraction/classification tests run. The shell executable fixture and `set_mode` call are also Unix-only. Hosted Linux jobs will not catch this.
- Base comparison: `git ls-tree` at dd3b293d confirms the entire new binary is absent. Existing CLI `control_plane_cli.rs:2` guards the analogous Unix import with `#[cfg(unix)]`; its Unix-specific tests are separately gated. This newly added target introduces an unguarded platform dependency.
- Guard search: examined the complete link-monitor module, Cargo manifest, CLI tests and workspace workflow. No `cfg(unix)` surrounds this import/helper/test; Cargo does not disable autobins or this test target. The workspace job uses Ubuntu. This finding concerns test compilation, not an assertion that all Windows runtime paths were previously supported.
- Confidence: **0.98** for the target-specific compile defect. No Windows cross target is installed in the review environment (only x86_64-apple-darwin), so no cross-compile success/failure is claimed.
- Read-only verification_command: `rtk proxy sed -n '198,222p' crates/tachi-cli/src/bin/taxonomy-link-monitor.rs`
- Fix/test: guard the Unix import, fake shell helper, and fake executable test with `cfg(unix)` while retaining portable pure-function tests on all targets; add a Windows test-compilation check for the CLI.

### PIPE-2 — [P2] [DEFECT] [CODE INFERENCE]: manifest can attest source that did not generate the PDFs

- Current location: `crates/tachi-core/src/catalog_drift.rs:242`, `:278`, `:319`, `:329`; CLI accepts arbitrary `--root` at `crates/tachi-cli/src/bin/catalog-drift.rs:10`.
- Trigger: run an already-built catalog-drift binary from revision A against a checkout at revision B whose report builder changed, or use a stale `target/debug/catalog-drift` executable after editing report code. The exposed `--root` interface permits this independently of the build directory.
- Consequence: `rendering_inputs(root)` hashes B source, but `crate::try_build_report_data_typst` runs the A implementation compiled into the executable. The after-render guard only compares the on-disk source before/after rendering, so stable B source passes. Publication records B source hashes beside PDFs assembled with A logic, and subsequent `--check` passes because it only rehashes the source and stored PDF bytes. The manifest therefore does not establish its stated source-to-PDF association in this supported invocation shape.
- Base comparison: catalog-drift and its manifest do not exist at dd3b293d. This is a defect in the newly introduced provenance contract, not a claim that older PDF comparison tests were reproducible in all environments.
- Guard search: examined the full CLI, `regenerate`, `check`, manifest serialization and catalog tests for compiled source identity, embedded build hashes, root/build matching, and executable identity checks. None exists. Typst version pinning, input stability recheck, staging, and rollback are real guards, but do not compare the Rust implementation used for rendering with the source recorded in the manifest. The documented `cargo run` from the same checkout avoids the ordinary stale-binary trigger; it does not enforce the library/CLI contract.
- Confidence: **0.92** for the traced mismatch; **runtime reproduction not performed** because it would require a second executable/source revision and renderer fixture. This is not evidence that the committed PDFs currently have wrong provenance.
- Read-only verification_command: `rtk proxy sed -n '240,335p' crates/tachi-core/src/catalog_drift.rs`
- Fix/test: bind the executable to a build-time digest of the relevant Rust source/build inputs and reject incompatible roots before regeneration, or arrange regeneration through a verified build of the selected root. Add a regression using a binary compiled before a deliberate report-builder change; regeneration must reject it rather than write an apparently current manifest.

## Positive checks and dismissed candidates

- Catalog and permissions workflows restrict token permissions to contents:read, use ordinary pull_request rather than pull_request_target, and include their own workflow plus relevant dependency/setup paths. No newly introduced privileged trigger or secret exposure established.
- Permissions checks are explicitly AC-2 rule-set/document membership checks. Combining allow/ask/deny is consistent with the existing AC-2 shell contract; lack of category-policy validation was not reported as a regression.
- Catalog staging rejects symlinks, validates all render outputs before publishing, verifies pinned Typst identity, detects input drift during rendering, and attempts rollback on ordinary write failure. A failed third render is covered. No claim of missing staging/rollback is made.
- Gitleaks adopter test invokes the scanner with redaction, tests inherited and custom detection and malformed configuration, and is explicitly enabled in CI. Existing scan/final failure checks remain fail-closed.
- Rust sha2 migration keeps fixed-width byte hex; package pins and toolchain versions are updated coherently in the inspected manifests. PostgreSQL workflow uses a fresh service and verifies several positive/negative RLS cases.
- Potential inherited Typst font environment influence and interruption-time publication durability were considered but are not findings: no environment reproduction was available, and ordinary-error rollback is narrower than crash-atomic guarantees.

## Coverage and limitations

Read shared context, Standard report-data trace, root codemap, changed-file inventory and relevant diff/current source. Focused on all changed workflow diffs, Rust setup, catalog check/regeneration, native permissions, adopter scanning, link monitoring, baseline tests, Rust tool/runtime pins, frontend package migrations and PostgreSQL RLS validation. This is not exhaustive review of every changed prompt, taxonomy record, report rendering branch, binary PDF or generated lockfile.

Graph search was used first for orientation; the shared context establishes its stale-source limitation. Direct pinned files and base comparison are authoritative. No product edits, Python, network checks, dependency installations or broad test execution occurred. Typst is absent from PATH; only the local macOS Rust target is installed. Commands above are inspection commands, not claimed behavioral reproductions. Prior memory was used only for graph/workspace orientation, not as current CI evidence.


---

# correctness — Phase 4

<!-- Source artifact: state/reviewer_correctness_phase_4.md -->

# Correctness Hawk — Phase 4 private reflection

Re-read only my Phase 3 review and pinned product source. No other reviewer material was read. The inherited available model remains in use; no claim of independent model diversity.

## Confidence Ratings

| Finding | Confidence | Severity | Reflection |
|---|---|---|---|
| COR-1 malformed risk table | High | P2 retained | The observed successful CLI output is decisive: a real High S-1 becomes an anonymous all-empty Tier 2 row. Re-read report_data.rs:164-168 and the parser defaults; no intervening validation changes the interpretation. This is newly introduced composition even though the permissive parser predates the window. |
| COR-2 unparsed populated controls | High mechanism; Medium exposure | P2 retained | The runtime zero-finding output is confirmed. Re-reading templates/tachi/output-schemas/compensating-controls.md:75-101 confirms that the canonical format requires severity grouping. Therefore the finding must be framed as unsafe acceptance of a malformed/noncanonical populated artifact, not as lack of support for an officially supported flat-table format. The new has_control_assessment scans headers throughout the section at report_data.rs:306-340, but cannot establish that an unparsed table has no rows. |
| COR-3 empty image defeats fallback | High branch; Medium end-to-end verification | P2 retained | Re-read attack-path.typ:69-81: has-image=true enters image(...) unconditionally, and the raw Mermaid fallback is only the else branch. CLI reproduction proves the zero-byte path is selected. PDF compilation remains unexecuted; the report should preserve that distinction rather than claiming an observed Typst exit. |

## Most/Least Defensible

**Most defensible: COR-1.** Its exact output corruption was reproduced with the pinned compiled CLI. It requires only a nonempty unexpected table under the recognized scoring heading, and it changes both identity and risk count. The fix is bounded and does not challenge the tier design.

**Least defensible as a standalone priority: COR-2.** It also has a concrete reproduction, but relies on grouping outside the canonical output schema and overlaps COR-1's higher-tier trust theme. A judge may merge COR-1 and COR-2 into one finding with two distinct ingress paths. Do not reject its observed mechanism merely because the input is malformed: the new code explicitly recognizes this header as completed-assessment evidence and reports a misleading success. Do not elevate it to P1 without evidence of ordinary generator output hitting the path.

COR-3 remains a distinct optional-asset degradation bug. Validation should skip unusable candidates, not fail the whole report on an absent image. A decoder failure test is still desirable before attaching an observed PDF-failure label.

## Changes

- No finding withdrawn; score remains 6.5/10 and recommendation remains request changes for bounded P2 fixes.
- Narrow COR-2 wording to **noncanonical or malformed populated residual input is silently accepted as an empty completed assessment**. The existing canonical schema has not changed to endorse flat tables.
- Distinguish confidence in deterministic source/runtime binding behavior from confidence in real-world trigger frequency and from verification tier. High confidence does not imply PDF execution was performed.
- COR-1/COR-2 can be consolidated for final issue counting provided the remediation still covers both risk-row validation and controls empty-assessment recognition.
- No new findings added. No additional executions or product edits were needed for this reflection.

## Remaining Uncertainty

The fixtures prove local output behavior, not frequency in generated production reports. The isolated CLI build and saved output remain available under /private/tmp/overseer-correctness-build and /private/tmp/overseer-correctness-DpxDFG. Full PDF compilation, complete test-suite execution, cross-platform asset lookup and report generation from live agents were not performed. Broad frontend, workflow and transactional catalog surfaces remain outside this focused review. These limitations do not change the reproduced boundary failures but constrain severity and completion claims.


---

# architecture — Phase 4

<!-- Source artifact: state/reviewer_architecture_phase_4.md -->

# Phase 4 — Architecture Critic Private Reflection

Only my own Phase 3 artifact and pinned source were consulted. No other reviewer findings were read. Score remains **7/10**; recommendation remains **request changes**.

## Confidence Ratings

- **ARC-1: High mechanism confidence; Medium P1 severity confidence.** Re-reading `report_data.rs:162-168` confirms that nonempty rows are the only Tier-2 validity criterion. `parsers/findings.rs:164-178` confirms missing columns yield empty strings rather than rejection. The controls guard at `report_data.rs:172-181` applies only if a qualifying controls file is present, so it does not protect the stated fixture with absent controls. The checked API never receives a risk-parse error because that parser returns a plain vector. I retain P1 because a successful security assessment substitutes invalid rows and suppresses its high-severity count; a judge could reasonably choose P2 if malformed intermediate artifacts are explicitly outside the supported input contract. The source contains no such enforced contract.
- **ARC-2: High binding/branch confidence; Medium end-to-end verification confidence.** `report_data.rs:234` still uses only `is_file`, so an empty PNG wins over later formats. The receiving template at `attack-path.typ:72-80` selects `image()` instead of its Mermaid fallback. I retain P2. An actual Typst compile should be the next verification step before labeling the final PDF failure runtime-verified.

## Most/Least Defensible

**Most defensible:** ARC-1's replacement of valid findings by a nonempty vector of empty strings. Every link is visible in a short producer-to-consumer trace, and the unchanged permissive parser becomes harmful through the new report-tier selection.

**Least defensible component:** ARC-1's P1 rating, because trigger frequency and downstream reliance are unmeasured. Its consequence must remain narrowly stated: the selected findings/detail/count projection loses `S-1`; the input file is not deleted, and raw attribution can still preserve the finding elsewhere. ARC-2 is technically straightforward but has not yet been compiled against the pinned renderer.

## Changes

No finding withdrawn or added. No severity change. Tightened ARC-1 severity confidence to Medium while retaining High confidence in the defect itself. Clarify ARC-2's fix scope: `assets.rs:128-168` provides nonempty/signature checks, not proof that arbitrary image bytes fully decode. Reusing that policy fixes the zero-byte case but does not automatically solve every corrupt PNG/JPEG/SVG. Final recommendation should request sufficient validation for the supported image formats, with a zero-byte regression as the minimal proof.

Reconfirmed that controls may intentionally represent a completed zero-residual assessment; I do not propose rejecting all empty selected assessments or forcing raw/residual counts to match. Reconfirmed catalog rollback is an ordinary-I/O-error safeguard, so claims of guaranteed crash-safe atomicity would exceed what this implementation promises.

## Remaining Uncertainty

Neither minimal fixture has been executed by me. A pinned report-data executable is available for later verification, but Phase 4 remained source-only. Renderer error text and final PDF behavior are not measured. No broad new investigation of frontend or catalog code was undertaken during reflection. Exact runtime regressions should be verified without modifying product files, and provenance should be kept pinned to the reviewed commit.


---

# security — Phase 4

<!-- Source artifact: state/reviewer_security_phase_4.md -->

# Phase 4 private reflection — Security Auditor

Reviewed only my Phase 3 artifact, relevant source, and the Phase 4 protocol. No other reviewer files read. Pinned source remains `8df554e884b1e5dd24146111a965597eff5f4779`.

## Confidence Ratings

- **SEC-1: High; retain P2 EXISTING_DEFECT.** The direct pinned-source reproduction demonstrates deletion of the only backing file, with the selected image path then dangling. Re-reading assets.rs:43-51 confirms byte equality is insufficient to prove the retained pathname survives unlink. Existing input symlink rejection is exclusive to catalog regeneration, which is not the report-data cleanup path. The flag is opt-in and local; those conditions limit exposure but do not authorize loss of the only image. A malformed signature-only fixture does not undermine the unlink mechanism, though a full-image fixture would make the regression test stronger.
- **SEC-2: High for wrong filesystem selection, Medium for confidentiality impact; retain P2 EXISTING_DEFECT with bounded wording.** Pinned CLI output proves parent traversal selects a neighboring SVG and emits has-image=true. report_data.rs:232-244 has only is_file; the parser has no applicable ID validation for the demonstrated heading branch. However, disclosure requires a later compile and sharing step, and the neighboring image must be inside the configured Typst root. The application is a trusted local generator; no remote request boundary has been established. The core actionable defect is accepting a path as a finding identifier and binding an unrelated image. Do not promote this into arbitrary-file read or P1 without stronger evidence.

## Most/Least Defensible

**Most defensible: SEC-1.** Concrete data loss is already demonstrated by actual pinned code. There is no separate compiler, service deployment, race, or malicious concurrent writer assumption. Symlinked aliases are unusual but valid filesystem inputs, and preservation of a retained copy is the purpose of the equality guard.

**Least defensible: SEC-2's downstream disclosure characterization.** Wrong path selection is solid. Whether the target report directory is an intended confidentiality boundary is less explicit than the cleanup preservation contract. Typst root confinement protects paths outside its root; if an adopter deliberately regards all report Markdown and every image under that root as equally trusted, the impact narrows to malformed-input report contamination. Keep both that condition and the suffix restriction prominent.

## Changes

No severity or score changes: 7.5/10, two P2 findings, no demonstrated P0/P1. Added literal `## Findings` above SEC-1 in Phase 3 solely to repair the requested schema. No other Phase 3 content changed. No new issue added and no repeated runtime checks needed: source reinspection supports the previous terminal reproductions.

For SEC-1, the simplest robust fix is to reject symlink counterparts during cleanup. For SEC-2, ID grammar validation and canonical image-directory containment should be separate protections; validating only slashes leaves symlink aliases, while containment alone still accepts nonsensical finding IDs. These are targeted fixes rather than grounds for claiming a complete sandbox.

## Remaining Uncertainty

No full Typst compile of the private-image fixture, database runtime/RLS replay, browser session-refresh test, or production consumer was available. The public primary-source Typst documentation establishes the outer project-root boundary, but not a guarantee that every adopter chooses the same root. Exact metadata and stale/malformed finding handling deserve correctness review; I have not generalized them into new security vulnerabilities. Existing Prisma server client is unused by inspected scaffold application code, so RLS bypass through hypothetical future callers remains excluded. Await debate before revising conclusions from other reviewers' evidence.


---

# devils_advocate — Phase 4

<!-- Source artifact: state/reviewer_devils_advocate_phase_4.md -->

# Phase 4 — Devil's Advocate private reflection

Re-read only my Phase 3 and current report_data/catalog_drift source. No other reviewer output inspected and no new probes performed.

## Confidence Ratings

- **DA-1 [P1]: High behavioral confidence; High defect confidence; Medium-High severity confidence.** The reproduction directly observes known High findings becoming zero through an unconditional higher-tier replacement. Source reconfirms that the nonempty controls vector bypasses every completed-empty assessment signal. The trigger requires incomplete or manually edited controls input, but incomplete-artifact safeguards already exist and explicitly preserve earlier findings for simpler stubs. A parseable descriptor of a missing control is not evidence that residual risk was assessed. P1 remains appropriate for silent success with lost security findings, without claiming production harm or exploitability.
- **DA-2 [P2]: High behavioral confidence; Medium contract/defect confidence; Medium severity confidence.** The private corrupted-companion check conclusively passes and the reader-facing sample PDF is unchecked. However, the narrow stated checker contract is ordered catalogs plus baseline hashes; companion publication is documented but an explicit companion-equality check may not be promised. Calling the checker false-negative assumes intended protection includes published companions. The finding should be presented as a bounded integrity gap in the newly expanded publication workflow, not failure of the baseline-only checksum algorithm. Current companion hashes match. Downgrade to advisory if the panel establishes intentional baseline-only gate scope.

## Most/Least Defensible

**Most defensible: DA-1.** It has a complete minimal fixture, freshly built pinned executable, deterministic output, exact causal branch, base introduction and counterexample to the existing stub-protection intent. Legitimate completed-empty assessments remain explicitly outside the allegation.

**Least defensible: DA-2's mandatory-fix classification.** Its observed mechanics are certain, but whether published companions fall within required check scope is less certain. A passing baseline-only test does not assert companion integrity by itself. The report should avoid implying corrupted PDFs are presently committed or that an existing safety check was removed.

## Changes

No finding withdrawn, no new finding added. Phase 3 formatting alone received a literal Findings heading and bracketed priority tags. Refined DA-2 overall confidence from undifferentiated High to High behavior / Medium contract, retaining proposed P2 subject to scope adjudication. Score remains 6/10 and request-changes rests on DA-1 rather than DA-2.

## Remaining Uncertainty

No PDF compilation for the DA-1 fixture; observed Typst bindings nevertheless establish data loss before rendering. No production frequency measurement. No crash or concurrent-publisher simulation, disk-exhaustion probe, stale-binary provenance reproduction, or workspace-wide suite. These remain limits, not severity findings. The companion corruption probe completed successfully before the prior turn interruption; only report writing was truncated and then repaired. Further verification should discriminate required publication guarantees from optional hardening rather than inflate successful checksum validation into broader safety claims.


---

# code_quality — Phase 4

<!-- Source artifact: state/reviewer_code_quality_phase_4.md -->

# Phase 4 — Code Quality Auditor private reflection

Only own Phase 3 and current source were read. No cross-reviewer evidence was consulted. Phase 3 received the requested format-only `## Findings` insertion; original findings remain unchanged.

## Confidence Ratings

- **CQ-1: Medium overall; P2 retained provisionally.** High confidence that versionless LLM05 evidence is accepted against the replacement catalog and that no edition-aware dispatch occurs. The concrete saved base fixture establishes a genuine old-input trigger. However, applicability of historical-input preservation as a release requirement needs adjudication: the roadmap's instruction to preserve historical references may have targeted authored provenance text rather than guaranteeing old report regeneration. The source schema still resolves IDs against the current catalog, which can be read as a current-edition-only contract. Do not present a runtime reproduction as having occurred.
- **CQ-2: High; P2 retained.** Re-read `schemas/finding.yaml:277-290`: `default: primary` is explicit, and prose specifically says the parser injects primary when absent. The new nested path deserializes `SourceAttributionRecord` before validation, while the struct has an unannotated required String and the older flat parser sets primary explicitly. The incompatible default is a concrete introduced path defect.

## Most/Least Defensible

**Most defensible: CQ-2.** A narrow omitted-field input contradicts a literal schema requirement, with a directly reachable deserialize failure and no competing guard.

**Least defensible: CQ-1.** The semantic collision is real, but severity and whether to treat unversioned old documents as supported inputs depend on product compatibility policy. Edition-aware migration cannot safely infer a category from finding prefixes or dates alone. A defensible minimum remedy could be an explicit compatibility limitation plus a version field for future documents, rather than an automatic remap.

## Changes

Re-read aggregation at `coverage_attestation.rs:204-240`: it matches only taxonomy and bare ID, so it cannot distinguish the two LLM05 meanings. It emits category IDs and classification, not category names. Narrow CQ-1 accordingly: this is **evidence counted under a reused current-catalog ID**, not a claim that the aggregation function itself prints the text “Data and Model Poisoning.”

Also re-read SARIF attachment: it constructs a 2026 intermediate reference, but `normalize_owasp_id` then strips the edition and outputs LLM-05. Thus the serialized SARIF does **not** necessarily contain a literal 2026 string from this path. The stronger CQ-1 evidence remains the catalog collision and editionless matching, not a promised visible SARIF year change. No severity elevation, no new findings. Preliminary score remains 7/10 pending debate.

## Remaining Uncertainty

No compiled reproductions, external taxonomy verification, or old-input compatibility policy clarification were performed. Runtime failures for CQ-2 follow directly from serde field requirements, but an isolated executable regression would strengthen the evidence. CQ-1 should be downgraded to compatibility-risk/advisory if the panel establishes that only explicitly migrated current-edition documents are supported. Prefix-derived risk-SARIF reference concerns remain excluded: a related citation is not by itself a proven contradiction.


---

# pipeline — Phase 4

<!-- Source artifact: state/reviewer_pipeline_phase_4.md -->

# Pipeline Reviewer — Phase 4 Private Reflection

Only my own Phase 3 and the cited current source/documentation were reread. No other reviewer output was accessed.

## Confidence Ratings

- **PIPE-1: High** for the new target-specific test compilation defect; **Medium** for its project-level importance. The test module has only `cfg(test)`, imports `std::os::unix::fs::PermissionsExt`, and unconditionally compiles the shell fixture helper. This is deterministic on a non-Unix target, but I have not established a required Windows CLI test lane or that the entire base workspace was Windows-test-clean. Keep [P2], with impact confined to the new CLI test target.
- **PIPE-2: Medium** overall, reduced from the Phase 3 numerical confidence. The implementation mismatch is a strong source-level inference: root inputs are hashed, a compiled-in builder executes, and only root stability is checked. However, the documented regeneration procedure explicitly uses `cargo run --locked` in the checkout and then runs the PDF compatibility test. That procedure prevents the ordinary stale executable trigger and should reveal a mismatch. The finding concerns the arbitrary-root/direct-binary interface and strength of the provenance claim, not the prescribed maintainer path or existing committed artifacts.

## Most/Least Defensible

Most defensible: PIPE-1 is a localized code defect with a conventional fix matching existing CLI platform guards. Its existence does not depend on interpreting the manifest contract.

Least defensible: PIPE-2 as a blocking acceptance issue. All native tools can be invoked from stale binaries; a requirement to authenticate the executable against a mutable source checkout is stronger than ordinary build reproducibility. The special feature here is that this command writes a source fingerprint that appears to certify association with generated PDFs. The docs at `docs/feature-roadmap-2026-10-04.md:95` support that interpretation, but do not explicitly promise refusal of binaries built elsewhere. I retain it as a conditional [P2] provenance weakness suitable for debate, not a release blocker.

## Changes

- No factual retraction for PIPE-1; clarify no baseline-wide Windows readiness claim.
- PIPE-2 confidence lowered to Medium and recommendation narrowed: first make the precondition explicit and decide whether arbitrary-root regeneration is intended across source revisions. If arbitrary roots are supported as the CLI suggests, enforce source/executable matching. Otherwise document and reject mismatched roots rather than immediately mandate a complex build-provenance subsystem.
- Phase 3 received formatting-only changes: added `## Findings` and bracketed `[P2]` labels. Its original assessment remains available as the independent record.
- No new findings or execution claims. No product files changed.

## Remaining Uncertainty

No Windows target or Typst binary is available in this review environment. PIPE-1 has not been cross-compiled. PIPE-2 has not been demonstrated with two builds and a rendered PDF; a targeted reproduction would distinguish an observable stale-output defect from a contractual improvement request. Existing input drift, renderer pinning, staged publication and ordinary-error rollback remain meaningful protections and must not be described as absent.


---

# correctness — Phase 5_round1

<!-- Source artifact: state/reviewer_correctness_phase_5_round1.md -->

# Correctness Hawk — Phase 5 debate, round 1

Read all six Phase 3 and Phase 4 artifacts, and rechecked the challenged source boundaries. No other round-1 output was read. Positions below rest on code/contracts and reproductions, not reviewer counts.

## Responses

### Malformed risk and incomplete controls: retain P2, keep consequences explicit

**ARC-1 / COR-1:** Agree on the defect; I independently runtime-reproduced the blank Tier-2 finding and high-count=0 with the pinned CLI. I challenge P1. The trigger is a malformed local optional artifact, not an ordinary schema-conforming assessment. There is no production-frequency evidence, no destructive modification of threats.md, and no proof that a released report was consumed under these conditions. The successful misleading projection is serious enough to require correction, but P2 captures the evidence without assuming a high-priority operational incident. Count severity and certainty separately: the defect is definite, its P1 priority is conditional. Source recheck at report_data.rs:164-168 confirms there is no extra validity guard.

**DA-1:** Accept the new independent reproduction and causal mechanism. Rechecked compensating_controls.rs:257-294: a status/effectiveness descriptor really produces a controls row even with no residual table. report_data.rs:289 returns true for that row, and :177 then replaces known findings with an empty vector. This is stronger than COR-2's flat-table case because it uses a legitimate control-detail fragment and bypasses the explicit completed-empty signals. Still retain P2: an incomplete local artifact is required and no release-path concurrency or routine completed-output case is shown. I do not infer that security-reporting context alone raises every false-negative edge case to P1.

### Merge COR-2 and DA-1 as one controls-assessment acceptance issue

**Merge supported.** Both hit has_control_assessment, cause the same unconditional Tier-1 replacement, and need one coherent completion/row-consumption contract. Use DA-1 as the primary trigger, then retain COR-2 as an additional regression illustrating that recognizable headers do not prove zero data rows. They are separate branches (:289 versus :306-340), so a fix merely removing controls from the first conditional is incomplete. Do not count two independent controls defects or discard either test. Keep risk-table COR-1/ARC-1 separate in the final actionable list because its parser and selection branch require independent validation.

### Catalog companion and binary provenance claims: advisory without stronger contract

**DA-2:** Rechecked catalog_drift.rs:193-201: it checks manifest.baselines. Lines :313-317 additionally refresh an existing companion during regeneration. Roadmap line 91 expressly describes seven registered hashes and same-publication companion updates; it does not explicitly promise that --check monitors every companion. The corruption probe proves a coverage limitation, but not a violation of the stated baseline-check algorithm. Current companions match. I recommend advisory follow-up/documented scope rather than a mandatory P2 defect unless a stronger companion-equality gate requirement is found.

**PIPE-2:** Rechecked :278 (compiled-in builder), :319 (root input stability), :321-331 (manifest association). A stale binary can indeed publish source hashes for different compiled logic. However, the documented procedure at docs/feature-roadmap-2026-10-04.md:91 uses cargo run from the checkout and follows with the PDF compatibility test. Most build tools assume their executable corresponds to the source when producing local provenance. The word “binds” at :95 supports concern, but does not clearly promise executable authentication or stale-binary rejection. Treat this as a medium-confidence provenance-design advisory, not a blocking defect, absent a reproduced violation of the prescribed workflow. Do not imply staged rendering, source drift checks, or rollback are missing.

### Attribution edition versus explicit default relationship

**CQ-2:** Accept P2 with High confidence. Re-read schemas/finding.yaml:277-290: relationship has default primary and explicit prose says the parser injects it when absent. The flat parser at findings.rs:598-600 supplies primary, while the new nested Entry path at :493-498 deserializes a required String without serde default. This is a concrete contract contradiction, unlike a preference for an unspecified compatibility policy. No requirement for a full PDF compile to establish the deserialize rejection.

**CQ-1:** The old/new semantic collision is real, but mandatory backward compatibility is unresolved. Schema finding.yaml:272-275 explicitly resolves bare IDs against the current catalog and carries no edition field. The Phase 4 correction that SARIF normalization removes the constructed year further narrows the visible effect. I recommend a compatibility advisory and explicit input-edition policy, not a required migration/remapping defect. A literal requirement to preserve historical authored references does not automatically promise regeneration of arbitrary old versionless inputs. Never infer migration merely from OI/LLM finding prefixes.

### Other independently supplied findings

**COR-3 / ARC-2:** Merge duplicate reports, retain P2. Both direct source and my runtime binding reproduction show empty PNG wins over Mermaid fallback. Keep downstream PDF failure labeled inferred until Typst execution occurs. “Reuse assets.rs” only establishes signature/nonempty policy, not complete decoder validation.

**SEC-1:** Accept P2. The concrete symlink dependence defeats the intended keep-an-identical-copy invariant and actually removes the sole backing bytes. Hardlinks are different and should not be rejected as proof of the same defect. No race or remote adversary is needed.

**SEC-2:** Accept bounded P2 for selecting an unrelated neighboring image, with the review's suffix/extension, Typst-root and later-compilation limits. Avoid arbitrary-file-read or demonstrated disclosure wording. ID validation and filesystem containment are distinct protections.

**PIPE-1:** Accept P2 restricted to the new CLI test target: unconditional std::os::unix in cfg(test) is not portable. Do not infer the entire base workspace was Windows-clean or a required Windows release lane was broken without checking that independently.

## Position Changes (new evidence required)

- COR-2 should merge into DA-1, based on the additional runtime fixture and confirmed early return at report_data.rs:289. This broadens the concrete controls regression set while reducing duplicated issue counting. The primary example becomes controls-only partial input; the flat populated table remains a required second test.
- Accept CQ-2 as an additional contract defect based on the explicit schema default and source reinspection, not the number of reviewers agreeing.
- Recommend DA-2, PIPE-2 and CQ-1 as advisories pending stronger contractual evidence. This does not dispute their underlying mechanics; it distinguishes a measurable scope limitation from an established promised behavior.
- No elevation of my findings to P1. Confidence in reproduced behavior remains High. Preliminary score stays 6.5/10; targeted corrections remain recommended.

## Unresolved Points

P1 vs P2 for silent report-data loss remains a priority judgment. Evidence that incomplete controls artifacts arise routinely in the supported completed-generation workflow would support elevation; no such evidence has been shown. Likewise, a direct requirement to check reader-facing PDF equality, reject stale executable provenance, or regenerate old taxonomy editions would convert the corresponding advisories into firmer contract defects. Typst execution and Windows cross-compilation remain unperformed verification opportunities.

## New Discoveries

No additional original issue added. The substantive new evidence for this reviewer is DA-1's controls-only path and CQ-2's literal schema-default contradiction. Both are source-rechecked above. Product files remained unchanged; this phase performed focused read-only source inspection and wrote only this assigned review artifact.


---

# architecture — Phase 5_round1

<!-- Source artifact: state/reviewer_architecture_phase_5_round1.md -->

# Phase 5 Round 1 — Architecture Critic

Read all six Phase 3/4 reviewer artifacts and re-read the disputed source and documented contracts. No other Round 1 artifact was read. Score remains **7/10** and targeted changes remain warranted.

## Responses

**COR-1 / ARC-1, COR-2, DA-1 — valid findings lost during tier promotion.** Correctness has now runtime-reproduced the risk replacement and noncanonical residual-table cases; Devil's Advocate has runtime-reproduced the control-inventory-only case. Those executions strengthen behavioral verification but do not establish production frequency. I accept all three mechanisms. Re-reading `report_data.rs:289` supports DA-1 specifically: any control inventory bypasses the stricter evidence checks, so its failure is distinct from COR-2's broad header-recognition path. A remediation that fixes only risk-row validity or only empty-table detection leaves another demonstrated defect. These may share a parent action, but retain three regression inputs.

I challenge DA-1's statement that this is a demonstrated “normal incomplete-artifact case” as grounds for P1. A fragment is plausible, but we have not shown a normal completed producer emits it or that a production user acted on the resulting report. The source contract at `templates/tachi/output-schemas/compensating-controls.md:75-101` explicitly groups residual rows by severity, and existing tests intentionally allow completed empty assessments. This supports accepting the defects while setting **P2 as the minimum justified priority**, including my ARC-1. A P1 policy could be defended for all silent security-report corruption, but that is a project prioritization rule rather than additional runtime evidence. No finding implies physical deletion of threat inputs.

**ARC-2 / COR-3 — image fallback.** Agree on deduplication into one P2. Correctness reproduced the binding; the compiler failure remains inferred until Typst is run. The final report should separately label those facts. SEC-2 is a different bug class at the same closure: canonical containment/ID validation and image usability require separate protections, and fixing one does not fix the other. SEC-1's retained-symlink data loss is also independent and has stronger runtime evidence than my image claim.

**DA-2 — companion integrity.** The corruption probe is conclusive evidence that `check` ignores published companions, but the required contract is narrower than the review's strongest wording. `docs/feature-roadmap-2026-10-04.md:91` promises checking seven registered hashes and updating companions in the same publication; `catalog_drift.rs:184-200` implements precisely the registered baseline list. Line 95's “PDF hashes remain checked separately” is ambiguous because it does not explicitly enumerate companion hashes. Recommend **advisory integrity gap**, not a mandatory P2 acceptance failure, unless a canonical requirement explicitly says the offline gate protects the reader-facing companions. Expanding it is useful and inexpensive, but usefulness alone is not proof of a broken guarantee. Current companions are reported to match; do not imply current corrupted output.

**PIPE-2 — stale executable provenance.** Agree with Pipeline's own Phase 4 narrowing. The source/hash mismatch is logically possible, but the documented command builds through `cargo run --locked` in the checkout, followed by a PDF compatibility test (`docs/feature-roadmap-2026-10-04.md:91`). That supported procedure addresses the ordinary stale-build case. Arbitrary `--root` permits testing fixtures and does not necessarily promise attestation across executable/source revisions. Recommend **advisory contract clarification**, not a release blocker or an immediate requirement for embedded source digests. Either document the executable/source identity precondition or explicitly support and enforce cross-root provenance. No two-revision reproduction presently demonstrates a failure in the documented maintainer workflow.

**CQ-1 — historical taxonomy editions.** The ID-meaning collision is real and architecturally important. However, current docs say to reconcile active artifacts against the current catalog and preserve *explicit* historical references; `docs/feature-roadmap-2026-10-04.md:73` specifically describes explicit 2025 attack-chain provenance. A saved unversioned attribution tuple does not contain an explicit edition. This leaves old-input interpretation underspecified rather than proving an edition-aware backward-compatibility promise was violated. I recommend **compatibility advisory pending contract clarification**, with a frozen old fixture and an explicit future edition field. Do not automatically map every LLM05 to LLM10: doing so breaks legitimate current poisoning citations. Agree with CQ Phase 4 that aggregation does not print the new category name and normalized SARIF does not necessarily serialize a literal 2026 year.

**CQ-2 and PIPE-1.** Both are narrower contract failures deserving P2. CQ-2 directly contradicts `schemas/finding.yaml:277-290`, which explicitly says omitted relationship defaults to primary; new nested serde parsing lacks that default. PIPE-1 is an unconditional Unix import under a test-only guard, a target-specific compile defect, without needing an unsupported broader claim that the whole baseline workspace was Windows-clean.

## Position Changes (evidence)

- **ARC-1 lowered P1 → P2.** Phase 4 already separated High mechanism confidence from Medium severity confidence. The concrete canonical-schema constraints and the absence of completed-producer/production evidence now support a bounded malformed-input priority. Runtime evidence from COR-1 increases verification confidence, not severity. This is not a retraction of data loss or a concession that malformed input may silently succeed.
- **ARC-2 remains P2.** Its binding behavior now has independent runtime evidence, while PDF failure remains unexecuted by these reviewers.
- My Phase 3 decision not to elevate stale-build provenance remains; DA-2 and CQ-1 should similarly remain explicit advisory limitations until required scope is established. Their mechanics can be true while mandatory-defect classification is unproven.

## Unresolved Points

1. Whether project severity policy promotes any silent security-report count suppression to P1 regardless of malformed-input trigger. My proposed minimum is P2; no P0 is supported.
2. Whether a canonical requirement extends the catalog gate to companions or executable/source identity beyond the documented cargo-based workflow. Source-level behavior alone cannot settle this scope question.
3. Whether historic versionless report regeneration is promised. An explicit edition policy is needed; automatic migration without provenance is unsafe.
4. Actual Typst outcome for empty-image and neighboring-image fixtures remains to be verified. Binding selection has been reproduced, PDF rendering/sharing has not.

## New Discoveries

No new independent finding added. New evidence consumed from the other reviewers establishes distinct control-tier failure triggers and runtime confirmation of my two producer-side mechanisms. Re-reading the documentation narrows catalog and historical-edition claims rather than manufacturing additional requirements. No product edits or new runtime probes occurred in this debate turn.


---

# security — Phase 5_round1

<!-- Source artifact: state/reviewer_security_phase_5_round1.md -->

# Security Auditor — Phase 5 Debate Round 1

Read all six Phase 3 and Phase 4 artifacts and relevant pinned source. Did not read other round-1 artifacts. Positions below follow mechanisms/contracts, not agreement counts.

## Responses

**ARC-1/COR-1 malformed risk selection: accept mechanism, challenge P1; recommend P2.** The pinned reproduction establishes loss of selected findings and severity counts. Yet the trigger is a malformed optional table, original threat files survive, and raw attribution may still expose the finding. No ordinary canonical generator output, operational deployment, or downstream authorization/release decision was demonstrated. The report's security purpose makes the fix important; it does not itself turn every silent output bug into a P1. ARC-1's Phase 4 appropriately narrows what disappears. Keep the concrete output loss and do not describe this as deletion of all security evidence.

**DA-1 controls-only fragment and COR-2 unparsed populated table: accept both paths, recommend P2.** Re-read report_data.rs:289: any nonempty controls or coverage_matrix bypasses the explicit completed-empty signals. DA-1 is stronger than a wholly arbitrary malformed table because a parseable control descriptor still proves nothing about residual completion. COR-2 separately shows recognition and extraction disagree. Fixing only the early return does not fix the header-only branch; fixing only risk validation fixes neither. These can form one selected-tier validation issue with separately testable ingress cases. P1 remains unsupported by observed exposure frequency or ordinary complete generator output; no claim that the input itself disappears is warranted. Intentional completed-empty assessment behavior must remain intact.

**ARC-2/COR-3 optional image failure: retain P2, qualify verification.** Zero bytes are demonstrably selected and bypass fallback; final decode failure is source-inferred, not compiled in the cited experiments. That distinction affects verification tier, not whether an empty optional image is a bug. Signature checks alone do not prove decoding succeeds. A shared usable-image resolver could address this and SEC-2 containment, but they have distinct failure modes and regression tests.

**SEC-1/SEC-2 threat model:** retain P2. SEC-1's backing-file deletion is a local safety defect independent of a hostile service model. SEC-2's ID-to-path escape is observed, but disclosure is conditional on later compilation, compiler root, and sharing. A user intentionally trusting every Markdown/image in the whole workspace reduces SEC-2 to report contamination; the finding should explicitly say so. Do not elevate either to arbitrary-root deletion, arbitrary-file reading, SVG execution, or RCE. Typst's broader project-root limit remains an effective outer guard.

**CQ-2 optional relationship default: accept P2 and upgrade verification to runtime-reproduced.** `schemas/finding.yaml:283-289` expressly promises parser injection of primary. The nested record has no serde default at findings.rs:63-67, and deserialization precedes enum checking at :497-504. I independently ran the pinned CLI with the exact omitted-relationship nested input; exit 1 reports `missing field relationship`. This is a supported-input contract violation rather than speculative compatibility policy.

**CQ-1 historical taxonomy meanings: real semantic collision, but recommend advisory/PLAN_RISK until compatibility scope is established.** Code Quality's reflection correctly retracts any claim that emitted SARIF necessarily gains a literal 2026 year. The saved artifact has bare IDs, and the schema says they resolve against the current catalog. Editionless IDs cannot distinguish old/new meanings. No date/prefix-based automatic migration is safe. The roadmap's preservation of historical references might govern authored citations rather than a promise to regenerate every legacy report. Document the unresolved migration contract and add edition provenance prospectively; do not silently invent automatic remapping. This is not evidence that the external OWASP catalog is wrong.

**PIPE-1 Windows test import: accept localized P2.** An unconditional Unix-only import in a newly added binary test module is directly actionable. Do not extrapolate to entire Windows production readiness or claim a cross-compile was performed.

**PIPE-2 stale binary and DA-2 unchecked PDF companion: keep as nonblocking provenance/integrity advisories.** Source mismatch and companion false-negative mechanisms are plausible/proven respectively, but compare the exact contract: roadmap line 91 prescribes `cargo run --locked` followed by PDF compatibility testing and says the offline check validates registered baseline hashes. That workflow avoids a stale direct binary and does not expressly promise checking every published companion. Line 95's phrase “binds PDFs” invites stronger interpretation, so documenting scope is justified. Neither proves currently committed artifacts have wrong provenance, nor that the manifest is a tamper-resistant attestation: a local actor able to edit root source/manifest already controls these inputs. Avoid mandatory executable-attestation infrastructure without an explicit cross-checkout binary support requirement.

## Position Changes with new evidence

Own SEC-1 and SEC-2 remain P2; security score remains 7.5/10. New independent CQ-2 runtime evidence:

- Built pinned report-data previously, at `/private/tmp/tachi-overseer-target/debug/report-data`.
- Fixture `/tmp/tachi-sec-default.po2jbe/threats.md` contains a valid High OI-1 recommendation and nested `{taxonomy: owasp, id: LLM10}` without relationship.
- Observed stderr: `Source Attribution: malformed YAML: OI-1.source_attribution[0]: missing field relationship at line 4 column 7`; exit 1.
- Read-only verification command: `rtk proxy /private/tmp/tachi-overseer-target/debug/report-data --target-dir /tmp/tachi-sec-default.po2jbe --template-dir templates/tachi/security-report` from pinned worktree.

This new run confirms the narrower default-contract finding; it adds no new issue and does not test historical taxonomy dispatch. No product edits, Python, database, or PDF execution.

## Unresolved Points

1. Whether policy assigns P1 to a reproducible incomplete-artifact false negative absent production frequency or a downstream decision boundary. My recommendation is P2 for all demonstrated report-tier cases.
2. Whether legacy taxonomy regeneration, companion integrity, and arbitrary-root binary provenance are promised supported contracts. Current evidence supports bounded advisories; a judge may require explicit documentation corrections.
3. Whether SEC-2 report-directory isolation is a formal confidentiality requirement. Wrong binding is definite; full PDF disclosure remains unexecuted and conditional.
4. Actual renderer result for corrupt/empty diagram and traversed SVG fixtures. Keep static/producer verification separate from compiled-render verification.

## New Discoveries

No new independent defect. CQ-2 now has an additional isolated terminal reproduction. The early controls acceptance and header-only recognition are distinct paths and must both survive any deduplication in the final report. Round 1 ends here.


---

# devils_advocate — Phase 5_round1

<!-- Source artifact: state/reviewer_devils_advocate_phase_5_round1.md -->

# Phase 5 — Devil's Advocate — Debate round 1

Read all six Phase 3 and Phase 4 reviews; read no other round-1 response. Rechecked current report tier guard, catalog roadmap guarantees and attribution schema. No new probe or product edit.

## Responses

### Report loss severity: ARC-1 / COR-1 / COR-2 / DA-1

Agree on the defects and on merging ARC-1 with COR-1. I retain **P1 for silent successful finding loss**, with Medium severity confidence and explicit malformed/incomplete-input preconditions. The important consequence is a report that presents zero High findings after known High evidence was available. This is more misleading than a compile error and cannot be detected by internal counts, since counts are recomputed from the wrong selected rows. Frequency is unmeasured; do not claim widespread production loss, deletion of source inputs, or disappearance from every attribution projection.

Challenge COR-2's proposal to use absence of ordinary generator evidence as decisive against P1: this guard exists specifically to handle incomplete/error artifacts. Its safety requirement cannot sensibly apply only to stubs that contain no parseable fields. DA-1 demonstrates a very short control inventory with a Missing control, not an exotic hostile payload. Nonetheless P2 is a defensible judge choice under a priority policy that weights malformed-input exposure above misleading success; High mechanism confidence alone does not settle priority.

Merge **COR-2 and DA-1** into one controls-assessment acceptance issue, but preserve two independent regression cases and code sites. COR-2 reaches header acceptance at report_data.rs:306–340 despite a populated unparsed table. DA-1 returns earlier at :289 solely because controls is nonempty, without any residual header. Restricting header acceptance to canonical severity sections fixes COR-2 but leaves DA-1. Removing controls-only acceptance fixes DA-1 but leaves COR-2. Do not lose either case when deduplicating.

Keep malformed risk selection as a distinct remediation item or explicit separate subcase: validation belongs to a different parser/branch and requires valid IDs/severities, whereas controls need reliable completed-empty detection. All fixes must retain explicitly completed empty assessments; equality with raw threat count is not a valid general invariant.

### DA-2 companion guarantee and PIPE-2 provenance

The companion-corruption behavior is reproduced, but re-reading roadmap lines 91–95 weakens mandatory P2 classification. It explicitly says the checker validates seven registered hashes and that companions are updated during publication. Those are different guarantees. The baseline registry contains .baseline paths only. I now propose **DA-2 as nonblocking advisory / P3**, or omit it from mandatory defect counts, while retaining the counterexample and recommendation. This avoids promoting an undocumented checksum guarantee into a blocking requirement. Current committed companions match.

Likewise, **PIPE-2 should remain advisory pending an explicit arbitrary-root provenance guarantee or two-build reproduction**. The source mismatch is logically plausible and matters for an attestation tool, but prescribed cargo run from the checkout plus backward compatibility validation defeats the normal stale-binary case. Every standalone compiler can be stale; this tool's stronger source-hash wording makes the limitation worth documenting, not automatically a release blocker. An inexpensive remedy is an explicit same-source-build precondition; do not mandate a complex authenticated-build system without a product requirement.

Do not merge DA-2 with PIPE-2: one leaves a consumer-facing output unchecked, the other risks associating generated output with source that the executable did not run. Different guarantees, triggers and remedies.

### Historical taxonomy and nested defaults

CQ-2 is a clear **P2 defect**: schema lines 277–290 expressly promise absent relationship becomes primary. Nested deserialization fails before any normalization, while flat input defaults correctly. This is stronger than a generic preference for tolerant parsing.

CQ-1 proves semantic ID collision, but the historical-preservation requirement remains ambiguous. Bare taxonomy IDs cannot identify their edition, and neither finding prefix nor document date safely disambiguates them. Preserve the frozen old fixture for compatibility adjudication; do not globally remap LLM05 and corrupt current poisoning citations. Treat as **P2 only if regeneration of prior edition reports is supported**, otherwise a documented compatibility limitation and versioning follow-up. CQ reflection correctly retracts any implication that SARIF necessarily prints a literal 2026 edition; the normalized emitted ID is editionless.

### Other reviewers

Support SEC-1 as bounded P2 data loss: its opt-in flag does not authorize deletion of the sole backing image. Support SEC-2 as bounded P2 wrong-image selection, not arbitrary-file disclosure or RCE. Support PIPE-1 as a new target-specific test compilation defect without claiming the baseline workspace was Windows clean. Merge ARC-2/COR-3; existing stronger image signature checks are not full decoder validation and SVG needs its own policy. End-to-end PDF failure still requires renderer verification.

## Position Changes (evidence)

- DA-2 changes from provisional P2 to advisory/P3 after re-reading the exact baseline-only checker and companion-publication guarantees in docs/feature-roadmap-2026-10-04.md:91–95. Reproduction remains valid; required contract coverage is narrower than my initial interpretation.
- DA-1 remains P1 proposed, High defect confidence / Medium priority confidence. Its earliest-return guard is distinct from COR-2; deduplication must preserve both regressions.
- Support conditional/advisory treatment of PIPE-2 and CQ-1 until their compatibility/provenance guarantees are established. This is a scope judgment, not denial of their code mechanics.

## Unresolved Points

1. Judge must choose P1 versus P2 for malformed/incomplete higher-tier data suppressing known findings, with consistent criteria across risk and controls paths.
2. Whether old-edition report regeneration is supported requires an explicit product/schema compatibility decision. Runtime collision alone does not settle that promise.
3. Whether published companions and arbitrary-root executable provenance belong to required integrity guarantees is not explicit enough to block on.
4. Empty-image PDF compilation remains unperformed in reviewers' evidence; preserve source/runtime-binding distinction.

## New Discoveries

No new defect or execution evidence. Cross-review synthesis establishes two separate controls guard bypasses and prevents an incomplete merged fix. Existing root confinement, checked attribution, symlink rejection during catalog staging, source-drift recheck, and ordinary-error rollback remain acknowledged safeguards.


---

# code_quality — Phase 5_round1

<!-- Source artifact: state/reviewer_code_quality_phase_5_round1.md -->

# Phase 5 — Code Quality Auditor — Debate Round 1

Read all six personas' Phase 3 and Phase 4 artifacts, then relevant pinned source. No other Round 1 response was read. Only this assigned review artifact and private fixture directories were written.

## Responses

**CQ-1 historical taxonomy:** I challenge my original mandatory-P2 framing. The base/current collision is demonstrable, but the source-attribution schema explicitly resolves the bare ID against the current catalog; no edition field was promised. The instruction to preserve historical references does not conclusively establish that old machine-readable findings must continue resolving against their original taxonomy edition. Keep this as a **P3 compatibility advisory**, not a blocker, unless an existing old-input compatibility contract is identified. Do not claim the export literally writes a 2026 version: the intermediate string is normalized to LLM-05.

**CQ-2 default relationship:** Retain **P2, now runtime confirmed**. The schema contract is explicit; a private fixture through the pinned CLI confirms nested/flat disagreement. Unlike CQ-1, this does not depend on an inferred compatibility promise. Detailed new evidence follows below.

**ARC-1 / COR-1 malformed risk table:** Agree this is one defect and must be fixed. The direct trace and Correctness runtime reproduction make the failure convincing. Prefer **P2**, rather than ARC-1's P1: it requires malformed optional assessment input and no frequency/ordinary-generation reproduction establishes urgent broad impact. Security-report context increases consequence, but does not by itself turn every malformed-input handling bug into P1. A request-changes recommendation remains warranted.

**COR-2 vs DA-1 controls:** Both expose incomplete assessment acceptance, with two independent guards that should remain represented in fixes/tests. DA-1's nonempty control inventory bypasses completion checks; COR-2's header evidence does not prove all populated rows were parsed. I support consolidating under a controls-tier completion defect with both fixtures, distinct from risk-tier validation. DA-1 is stronger because the fragment is individually canonical and plausibly partially generated, while COR-2 uses noncanonical grouping. Prefer **P2** absent production frequency or direct downstream safety/release gate evidence. A completed zero-residual assessment remains valid and must not be “fixed” by blanket rejection of empty residuals.

**ARC-2 / COR-3:** Agree and deduplicate at P2. File existence is not sufficient usable-image evidence. The demonstrated generated binding plus template branch proves fallback bypass; do not label PDF compilation as observed until performed.

**SEC-1 / SEC-2:** Support the bounded P2 conclusions. SEC-1 is especially strong actual destructive behavior: equal bytes through a symlink do not establish independently retained bytes. SEC-2 is an actual neighboring-image selection defect; preserve the Typst root, required suffix, local generation and sharing conditions rather than generalizing to arbitrary filesystem disclosure.

**DA-2 companion PDFs:** Agree the private corruption probe establishes that companions are unchecked. Challenge treating this as a failed existing guarantee: the checker explicitly enumerates baseline hashes and companion bytes are deliberately excluded from rendering inputs. Regeneration maintaining companions makes equality checking useful, but does not alone imply a full reader-facing artifact integrity gate. Prefer **P3/advisory**, unless the scope owner confirms the catalog check is intended to cover every published companion. Never describe current committed companions as corrupt.

**PIPE-2 stale builder provenance:** The mismatch is mechanically plausible, but the documented cargo-run-in-checkout procedure avoids it. Authenticating a native executable against mutable source is a stronger contract than ordinary source hashing. Prefer **P3/advisory** and document the same-checkout/rebuild precondition before requiring embedded build digests. A two-revision reproduction could prove mechanics, but still would not by itself resolve the intended supported workflow.

**PIPE-1 Windows compile:** Support **P2**, high source confidence. Re-read the unconditional test module import and set_mode calls: a Windows compile cannot resolve std::os::unix. Guard only the Unix helper/import/test so portable tests remain. No need to allege the entire baseline workspace was Windows-ready, or to pretend a Windows compiler was run.

## Position Changes (evidence)

- CQ-1 changes from provisional P2 to **P3 compatibility advisory**, because the historical-preservation requirement remains ambiguous and the declared lookup contract uses the current catalog. The collision itself is not withdrawn.
- CQ-2 remains **P2**, confidence strengthened from static High to runtime-confirmed High.
- Current score **7/10**, request changes based on CQ-2 and the independently corroborated report/asset/platform defects. Advisory provenance issues should not inflate defect counts or drive severity.

## Unresolved Points

1. Does the supported product contract include regeneration of pre-migration machine-readable attribution? If yes, CQ-1 returns to P2; if not, document current-edition-only input handling.
2. Is catalog-drift explicitly a baseline provenance gate or a gate for every published PDF? This decides mandatory treatment of DA-2.
3. Are arbitrary-root/stale-binary regeneration calls supported without rebuilding? This decides PIPE-2's contractual relevance.
4. P1 vs P2 for false-negative report generation remains a consequence/frequency judgment. My vote is P2 for the verified, bounded incomplete/malformed optional-input cases; all remain actionable.

## New Discoveries

No new finding. Executed CQ-2 minimal pair using `/private/tmp/tachi-overseer-target/debug/report-data`, whose pinned-source build provenance is recorded by Security/DA. Created only private fixtures at `/private/tmp/tachi-cq-default-DFqawh` with identical High OI-1 recommendation rows. Neither directory contains top-level image stems; no output-file or cleanup flags were used.

- `nested/threats.md` uses `OI-1: {source_attribution: [{taxonomy: owasp, id: LLM10}]}` in YAML block form. CLI **exit 1**: `Source Attribution: malformed YAML: OI-1.source_attribution[0]: missing field relationship at line 3 column 7`.
- `flat/threats.md` uses `OI-1: [{taxonomy: owasp, id: LLM10}]` in YAML block form. CLI **exit 0**, generated output includes `primary`.

Read-only replay:

```sh
rtk proxy /private/tmp/tachi-overseer-target/debug/report-data --target-dir /private/tmp/tachi-cq-default-DFqawh/nested --template-dir templates/tachi/security-report
```

The first command is expected to fail; this is observed rejection of schema-valid omitted relationship, not a claimed passing test. No full suite, PDF compile, or additional product mutation occurred.


---

# pipeline — Phase 5_round1

<!-- Source artifact: state/reviewer_pipeline_phase_5_round1.md -->

# Pipeline Reviewer — Phase 5 Debate Round 1

Read the six Phase 3 reviews and private reflections; no Round 1 response was read. Rechecked current report tier selection and the explicit relationship default in the schema. No new runtime probe or product edit.

## Responses

### PIPE-2: challenge the supported invocation and required provenance contract

I challenge my initial [P2] mandatory-defect framing. The mechanism is real: an arbitrary root can be hashed while a differently built Rust implementation renders it. But `docs/feature-roadmap-2026-10-04.md:91` explicitly prescribes `cargo run --locked` in the checkout followed by the PDF compatibility test. This is affirmative evidence of a procedure that avoids the ordinary stale executable condition, not merely an absent guard. Exposing `--root` supports choosing input roots; it does not unambiguously promise that a binary built from any revision will certify a different revision. I now recommend **advisory/P3**, unless a supported cross-checkout regeneration use case or reproduction establishes an in-scope contract violation. Do not count it as a confirmed blocking defect or imply the committed manifest is wrong. A clear same-revision precondition may be a proportionate first remedy.

### DA-2: companion check scope

The observed corrupted-companion pass is convincing, and protecting reader-facing PDFs would be useful. However, the checker expressly iterates `manifest.baselines`; generated companion exclusion is deliberate. The existing catalog fixture even assigns companions different bytes and expects check success, as DA notes. This is positive evidence of a baseline-only check contract. Publishing both files in one operation does not logically entail policing all later modifications to both. Recommend **advisory/P3** unless an explicit acceptance criterion demands companion integrity from `--check`. Preserve the useful proposal to add registered companion outputs, but describe it as extending gate scope. Current companions are confirmed to match and there is no evidence of current corrupt artifacts.

### ARC-1 / COR-1 / COR-2 / DA-1: defect agreement, priority challenge

I agree these are real introduced composition defects. The successful empty/blank report output is stronger than a mere parser strictness concern. DA-1 is particularly clear: a descriptor saying Authentication is Missing is not completion evidence for zero residual findings, and the broad nonempty-controls shortcut at report_data.rs:289 bypasses the later completion signals. Retain it separately from the malformed scored-table branch because fixing one does not fix the other.

I recommend **P2 for all three ingress cases**, with High behavioral confidence. The demonstrated triggers are incomplete or noncanonical files; no ordinary completed canonical artifact or production generation path has been shown to produce them. Security prioritization makes the consequence important but does not by itself establish P1 exposure. A P1 recommendation needs an ordinary interruption/resume path that subsequently auto-renders these partial inputs, or a canonical completed example that loses findings. Regardless of priority, fixes should reject or preserve an earlier tier with a visible diagnostic, while retaining intentionally completed-empty assessments. Do not treat any empty residual vector as invalid.

### PIPE-1: Windows compilation

Retain **P2, High confidence** for the narrow new test-target compile error. Guarding the Unix helper and test is a small, conventional correction; existing CLI tests demonstrate that pattern. This is not evidence of Windows runtime readiness or an already-required Windows CI matrix. A cross-target `cargo check --tests` on a provisioned target is useful verification; absent installed target support is an environment limit, not a reason to claim the check passed or to inflate severity. Portable extraction/classification tests should remain enabled outside Unix.

### CQ-2: relationship default

Agree with **P2, High confidence**. Current schema explicitly says the parser injects primary for omitted relationships (`schemas/finding.yaml:283-288`), while the nested deserializer requires the String before validation. Unlike PIPE-2 and DA-2, this is a direct contradiction of a literal input contract and has a bounded fix. Add explicit default behavior for the nested path and test parity with the flat representation while retaining invalid-value rejection. A runtime fixture would strengthen the label, but missing required serde fields already establish the causal mechanism.

### Other reviewers

ARC-2/COR-3 and SEC-1 have concrete guard/producer mismatches deserving bounded P2 fixes. SEC-2 should remain restricted to wrong neighboring image selection and conditional disclosure within the configured Typst root. CQ-1 is a real semantic migration risk, but I agree with its reflection that historic-output support requires policy adjudication; do not infer a serialized 2026 year that normalization removes.

## Position Changes (new evidence)

- PIPE-2: provisional P2 becomes advisory/P3. The decisive contract evidence is the documented same-checkout Cargo regeneration plus PDF comparison procedure, reinforced by the distinction between input-root selection and build-provenance guarantees.
- DA-2: recommend advisory/P3, based on the explicit baseline-only checker and intentional fixture acceptance of unequal companions. Its reproduction remains valid but proves an uncovered scope rather than a failed promised check.
- Support CQ-2, tier-validation and image/cleanup findings as bounded corrections. Prefer P2 over P1 for partial-input tier loss pending ordinary-path exposure evidence.
- PIPE-1 remains unchanged. These judgments do not change or erase the independent Phase 3 record.

## Unresolved Points

1. Whether the project requires Windows CLI test compilation, and whether the panel wants a narrow portability fix or a new ongoing CI lane.
2. Whether cross-revision `--root` regeneration is intended; a documented precondition would settle much of PIPE-2 without a build attestation subsystem.
3. Whether published companion integrity belongs in the mandatory catalog gate scope.
4. Whether completed canonical generator output or an automatic interruption/resume workflow reaches the tier-loss cases often enough to support P1.

## New Discoveries

No additional defect. New information came from other reviewers’ bounded reproductions and contract evidence, not broader probing. No full Typst, Windows, database or browser validation is claimed.


---

# Phase 6: Round 1 Summary

<!-- Source artifact: state/phase_6_round1_summary.md -->

# Phase 6 — Round 1 summary

## Resolved this round
- Risk-table silent replacement is reproduced (COR-1/ARC-1); Architecture lowers P1 to P2 based on malformed-input trigger and absent ordinary-workflow exposure evidence.
- Group COR-2 and DA-1 as incomplete controls assessment selection, preserving both distinct triggers: unconsumed populated flat residual table, and control metadata without residual completion evidence.
- Deduplicate COR-3/ARC-2 (invalid attack image bypasses text fallback). PDF compilation remains unperformed; generated bad binding is reproduced.
- SEC-1 cleanup symlink and SEC-2 image path traversal remain separate bounded P2 issues, with local-input/project-root limitations.
- CQ-2 documented relationship default is independently reproduced by Security and Code Quality: nested omitted relationship errors, flat form succeeds.
- PIPE-1 Unix-only import remains a P2 test-target compilation defect, no Windows execution claim.
- CQ-1 historical taxonomy, DA-2 companion integrity and PIPE-2 stale executable provenance are downgraded to advisories/P3 by their original reviewers after checking narrower contracts. No claim that committed PDFs are currently corrupt.

## Still in dispute
One material priority dispute remains: DA proposes P1 for successful zero-finding output from partial controls; the other five prefer P2 given malformed/incomplete input and unmeasured normal-workflow prevalence. Mechanism is agreed. The relevant gate (`crates/tachi-core/src/report_data.rs:289-291`) is:

```rust
    if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() {
        return true;
    }
```

The competing contract evidence is the canonical severity-grouped residual schema (`templates/tachi/output-schemas/compensating-controls.md:75-101`) and deliberate completed-empty tests; these do not legitimize silent truncation but constrain impact claims. Round 2 asks reviewers only to distinguish known impact from exposure assumptions and cite any project priority policy. Minimum justified severity is the eventual judge's responsibility.

## New discoveries
No new defect classes. CQ-2 gained independent runtime verification. All reviewers accepted that generated baseline equality does not independently establish semantic report correctness.

## Sycophancy audit
Four reviewers lowered a position (ARC-1, CQ-1, DA-2, PIPE-2) with explicit source-contract or trigger evidence. Unsupported majority-following changes: 0/4; no alert. Agreement is not verification, and correlated-model warning remains required.

## Next round
Round 2 required for the substantive P1/P2 disagreement. No need to reopen resolved technical mechanisms or invent new defects. If no new evidence emerges, preserve the disagreement and send it to severity verification/judge; omit Round 3 rather than repeat votes.


---

# correctness — Phase 5 Round 2

<!-- Source artifact: state/reviewer_correctness_phase_5_round2.md -->

# Correctness Hawk — Debate round 2

## Position

Retain **P2** for merged COR-2/DA-1. Request correction before relying on the affected output; do not equate P2 with acceptable behavior. Preserve DA's dissent for the judge.

## Evidence

The known consequence is serious and reproduced: report_data.rs:289 accepts control inventory as assessment evidence, :177 replaces findings, and :276 reports zero. No completed residual assessment is required. DA correctly notes that this violates the existing incomplete-artifact protection intent; a plausible fragment suffices, and no hostile payload is needed.

Priority still depends on scope: the trigger requires an incomplete local input, not a demonstrated complete producer output. No project priority rule was supplied that assigns P1 to every silent security-report corruption. Neither production harm nor ordinary workflow exposure is needed to establish a defect, but their absence limits the case for escalating this bounded input-validation bug above P2. Runtime certainty cannot resolve that priority judgment.

## Remaining Dispute

DA weights the misleading success consequence above the conditional exposure; I retain the minimum justified priority. Both controls triggers remain necessary regression cases. No further debate without a priority policy or new exposure evidence is useful.

## New Discoveries

None. Read the round summary and relevant DA/Architecture arguments; no probes or new general review performed.


---

# architecture — Phase 5 Round 2

<!-- Source artifact: state/reviewer_architecture_phase_5_round2.md -->

# Phase 5 Round 2 — Architecture Critic

## Position

Retain **P2** for incomplete-controls selection, including DA-1 and COR-2, and for malformed-risk selection. These are confirmed correctness defects deserving correction before accepting the reviewed behavior. P2 does not mean harmless or optional. The P1/P2 disagreement is about priority, not the truth of successful false-zero output. No change to my 7/10 score or request-changes recommendation.

## Evidence

DA-1 proves a parseable control descriptor is incorrectly promoted into a completed residual assessment through `report_data.rs:289`; replacement at `:177` then empties the selected findings. That is a stronger realistic input fragment than COR-2's ungrouped residual table, and the new early-return guard defeats its own stricter completion checks. Its observed zero totals are sufficient defect evidence without a PDF compile.

The counterweight remains scope: a partial controls artifact is required. `templates/tachi/output-schemas/compensating-controls.md:75-101` documents complete severity-grouped residual tables, and no inspected completed canonical input or normal interruption-to-auto-render workflow reaches the failure. Existing completed-empty tests require preserving legitimate zero-residual assessments. No project rule assigning P1 to every misleading security-report output has been cited in the shared evidence. I cannot treat plausible exposure as measured routine exposure.

I do not require evidence of an actual harmed production user to justify P1 in general. A documented ordinary path, a supported complete input, or an explicit consequence-based severity policy could suffice. None has been presented here. Conversely, original input survival does not undo false-zero risk communication; it only bounds claims of destructive loss.

## Remaining Dispute

DA's consequence-based P1 is defensible if the judge explicitly adopts a policy that successful omission of known High findings merits urgent priority despite the partial-artifact condition. My minimum justified priority remains P2. Preserve both views in the decision record rather than seeking another majority vote. A third debate round without new exposure or policy evidence would not resolve this distinction.

## New Discoveries

None. Read the Round 1 synthesis and reviewer arguments; no broad probing, new execution, product mutation, or blind-final work performed. Priorities for catalog and historical-edition advisories remain unchanged.


---

# security — Phase 5 Round 2

<!-- Source artifact: state/reviewer_security_phase_5_round2.md -->

# Security Auditor — Phase 5 Round 2

Read Phase 6 round-1 summary and the relevant Devil's Advocate/Architecture responses. No new probes and no blind final.

## Position

Retain **P2** for the combined incomplete-controls assessment issue, with High confidence in the defect and Medium confidence in priority. Do not equate five reviewers choosing P2 with proof; my basis is the demonstrated input/impact boundary and lack of an explicit project policy promoting all misleading security reports to P1.

## Evidence

The known impact is substantial and deterministic: `report_data.rs:289-291` treats a parseable control descriptor as completed-assessment evidence; `:174-177` replaces selected risk findings with the empty residual vector; `:268-276` produces zero counts. The controls-only fixture has no residual completion signal and is clearly distinct from intentionally completed empty assessments. The shorter metadata fragment is plausible as an interrupted intermediate artifact; it need not be a malicious payload. Therefore I reject any suggestion that malformed input makes the defect non-actionable or that the canonical schema is a sufficient runtime guard.

The evidence nevertheless establishes a local transformation of incomplete input, not physical destruction of primary evidence, a bypass of a deployed decision gate, an ordinary completed producer's default behavior, or broad exposure. The canonical residual format expects explicit severity-grouped findings, while source-input files and raw attribution can remain available. Those conditions bound the known failure. The report's subject is security, but its domain alone does not establish high incident priority. The existing negative stub tests show an intended safety property is broken; they establish defect status, not an automatic severity level.

Devil's Advocate correctly emphasizes that a successful false-zero report is more dangerous than a visible compile error. I agree and would prioritize this fix ahead of cosmetic/test portability issues within P2. P2 permits prompt correction of a material bug; it does not require accepting misleading success indefinitely. If a project rule explicitly ranks any silent loss of High findings as P1, apply that rule consistently to both risk and controls variants and state it as a policy choice. No such rule was identified in the reviewed evidence.

## Remaining Dispute

DA's P1 is a defensible impact-weighted judgment, not a different technical finding. My P2 is the minimum priority justified without adding an exposure/decision-boundary assumption. Preserve this disagreement for the judge rather than trying to settle it through another vote or repetitive round. The final issue should retain both the controls-only and populated-unparsed-table triggers and must preserve intentional completed-empty assessments. No additional execution is necessary to adjudicate priority.

## New Discoveries

None. No severity changes to my own security findings, no new claims, no product edits, and no execution during this round.


---

# devils_advocate — Phase 5 Round 2

<!-- Source artifact: state/reviewer_devils_advocate_phase_5_round2.md -->

# Phase 5 — Devil's Advocate — Round 2

## Position

Retain **proposed P1, Medium priority confidence**, for the merged controls false-zero issue; the mechanism has High confidence. A judge may choose P2 as the conservative project priority without changing the defect or verification status. This disagreement is about impact weighting, not disputed source behavior. No further debate round is needed without new policy or exposure evidence.

## Evidence

Known impact: a successful checked report generation replaces an existing High risk finding with zero findings solely because a controls inventory descriptor exists. report_data.rs:289 accepts a nonempty controls vector; :177 replaces findings with an empty residual vector; :268–276 recalculates zero severity/total counts. My pinned-source fixture demonstrates the output. No residual table or completed-empty metadata is present. This is a misleading final result, not merely an unsupported format rejected with an error.

Exposure limits: the controls document is incomplete, the canonical schema groups residual findings by severity, and no production frequency or normal completed-producer occurrence has been shown. Source input files are preserved and raw attribution can still show original evidence. I withdraw any reading of my phrase “normal incomplete-artifact case” as a measured ordinary-workflow claim; it meant a simple plausible partial document only.

Why retain P1: prioritizing a security report that silently declares zero known findings does not require proof that someone already relied on it. That would confuse defect priority with incident confirmation. Likewise, missing prevalence evidence is not evidence of rarity. I weigh misleading successful output highly because a user may trust the completed report without comparing input rows. Other reviewers reasonably weigh the required malformed/intermediate input more heavily. My judgment does not apply to every parser edge case; it applies to this demonstrated replacement of valid existing assessment content.

No inspected project or skill policy has been cited that mandates P1 for all silent security-report corruption or P2 for malformed local inputs. Therefore neither priority is mechanically required by policy. Do not invent such a rule. “Minimum justified severity” supports the judge choosing P2, but it should retain a short dissent rationale rather than suggest technical evidence was refuted.

## Remaining Dispute

Only P1 versus P2 remains. COR-2/DA-1 merge and both distinct regression inputs remain agreed. The header-recognition fix must not leave controls-only early acceptance intact, and the early-acceptance fix must not leave populated unparsed residual tables classified as completed empty assessments. No change to the catalog/provenance/historical-taxonomy advisory positions from Round 1.

## New Discoveries

None. Read phase_6_round1_summary and the relevant Correctness/Architecture Round 1 responses. No new probes, runtime evidence, product edits or broader review performed. The absence of a cited severity policy is an uncertainty, not a new finding.


---

# code_quality — Phase 5 Round 2

<!-- Source artifact: state/reviewer_code_quality_phase_5_round2.md -->

# Phase 5 — Code Quality Auditor — Debate Round 2

## Position

Retain **P2** for the merged incomplete-controls assessment defect, with High defect confidence and Medium priority confidence. Request changes remains appropriate. This is not acceptance of misleading zero-finding output, nor a claim that malformed input excuses silent success.

## Evidence

The known impact is precise: an incomplete controls artifact is accepted by report_data.rs:289-291, its empty residual vector replaces valid higher-count findings, and a successful generated report advertises zero selected High/total findings. DA's controls-only fragment needs no exotic syntax; COR's populated unparsed table reaches a separate completion guard. Both are demonstrated and require independent regression tests.

Exposure assumptions are distinct: no producer completion protocol, ordinary generator failure rate, downstream automated acceptance gate, or operational reliance has been demonstrated. Production harm need not occur before a bug merits P1; I do not adopt observed harm as a necessary condition. Nevertheless, a local report conversion with a specifically incomplete optional artifact, preserved original threat evidence, and no established automatic safety decision supports a bounded P2 priority. “Security report” alone is insufficient to infer an immediate system-wide decision impact.

DA's strongest counterargument is valid: the new guard explicitly intends to tolerate incomplete/error artifacts, and controls-only fragments are plausible. That establishes the defect and the importance of the regression; it does not provide new evidence of trigger prevalence or a project rule that all successful finding suppression is P1. No cited project priority policy resolves that distinction. Both P1 and P2 remain defensible judgments; my vote uses the minimum justified priority.

## Remaining Dispute

The only disagreement is urgency, not mechanism, scope, or need to fix. If the project declares these reports authoritative release/compliance gates, P1 becomes substantially easier to defend without waiting for actual harm. Until such a contract is established, retain P2 and prominently describe the false-zero consequence. Do not dilute remediation when merging: fix both inventory-only acceptance and populated-unparsed-table acceptance, while preserving completed empty assessments.

No further round is likely to add evidence by repeating votes. Preserve the minority P1 recommendation for the judge rather than manufacturing consensus.

## New Discoveries

None. Read Phase 6 Round 1 summary and relevant DA/Architecture Round 1 arguments only. No new probes or product edits. Score remains 7/10; do not start blind final from this response.


---

# pipeline — Phase 5 Round 2

<!-- Source artifact: state/reviewer_pipeline_phase_5_round2.md -->

# Pipeline Reviewer — Phase 5 Debate Round 2

## Position

Retain **P2 for the controls false-zero issue**, with High confidence in the defect and Medium confidence in the priority judgment. The finding merits correction; this position does not excuse malformed-input handling or dispute the reproduced consequence. No new evidence justifies a priority change since Round 1.

## Evidence

The verified impact is successful report-data generation that selects Tier 1, removes known High findings from the selected findings vector, and emits zero counts. `report_data.rs:289-291` accepts a control descriptor without residual assessment completion evidence. The independent populated-flat-table trigger reaches another overly broad acceptance branch. Both regression cases must remain in the merged finding.

DA correctly observes that incomplete-artifact protection is the purpose of the guard and that Missing authentication is a simple, plausible fragment. However, a short fixture is not itself evidence that the normal completed canonical generator output or an automatic retry/resume pipeline produces and renders that fragment. Severity should separate the high consequence of the reproduced output from the likelihood and supported workflow conditions that reach it. Source artifacts remain intact; no production harm or irreversible deletion is shown for this finding.

No project priority policy has been identified in the supplied summary/relevant debate that categorically assigns P1 to any misleading security report. I therefore use the narrower justified priority: bounded malformed/incomplete optional-input defect requiring a timely fix, P2. I would support P1 if a normal automatic workflow renders incomplete artifacts without an intervening completion boundary, or if a canonical completed controls document yields the same omission.

## Remaining Dispute

DA weighs silent false reassurance and the explicit incomplete-input guard purpose more heavily and proposes P1 with Medium priority confidence. I weigh the demonstrated noncanonical/incomplete-input condition and unestablished ordinary-workflow exposure as limiting factors and prefer P2. This is a prioritization dispute, not disputed code behavior. Preserve both rationales for the judge; further voting without new evidence will not resolve it.

## New Discoveries

None. Read the Round 1 summary and DA Round 1 response only for this follow-up. No probes, product changes, new findings, or blind final assessment performed.


---

# Phase 6: Round 2 Summary

<!-- Source artifact: state/phase_6_round2_summary.md -->

# Phase 6 — Round 2 summary

## Resolved
No new defect classes or runtime evidence. Five reviewers retain P2 for incomplete-controls false-zero output. Devil's Advocate retains an impact-based P1 proposal with Medium priority confidence, explicitly accepts P2 as defensible, and withdraws any implication that the trigger was observed in routine production. No inspected project severity policy settles urgency. Technical mechanism and both required controls regressions remain agreed.

## Remaining dispute and source
Priority only: successful suppression of known High findings versus the incomplete/noncanonical artifact trigger and unmeasured exposure. The decisive source remains `crates/tachi-core/src/report_data.rs:289-291`:
```rust
    if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() {
        return true;
    }
```
That proves acceptance of control inventory as completion, not frequency or downstream security decisions. Severity verification and the neutral judge must adjudicate the minimum justified priority; do not convert a vote into proof.

## Convergence and sycophancy
No unsupported movement toward a majority. DA qualified its own exposure claim from the evidence rather than changing its priority to match others. Round 3 is omitted because the only difference is an explicit priority judgment, with no new technical evidence emerging; further repetition would not resolve it. Two mandatory-quality debate rounds are preserved verbatim. Proceed to blind finals and independent completeness/verification.


---

# correctness — Phase 7 Blind Final

<!-- Source artifact: state/reviewer_correctness_phase_7.md -->

# Correctness Hawk — Phase 7 blind final

## Final Score

**6.5/10.** Independent final, based on my prior artifacts and shared round summaries; no other Phase 7 output read. No new finding introduced. The available inherited model was used throughout, so persona independence is not model diversity.

## Top 3 Points

1. **P2 — COR-1/ARC-1: malformed optional risk data displaces valid findings. [RUNTIME-VERIFIED OUTPUT] [HIGH CONFIDENCE]** The pinned CLI changed a real High S-1 into an anonymous empty Tier-2 row and high-count=0. report_data.rs:164-168 accepts any nonempty parser result. Fix row/header validation before tier selection. The input is malformed; no production prevalence or source-file deletion is claimed.
2. **P2 — DA-1/COR-2: incomplete controls are accepted as a completed zero-finding assessment. [RUNTIME-VERIFIED OUTPUT] [HIGH MECHANISM CONFIDENCE]** Keep this as one finding with two regressions: inventory-only early acceptance at report_data.rs:289 and an unconsumed populated residual table recognized at :306-340. Both cause replacement at :177. Preserve legitimate completed-empty assessments. DA's P1 dissent is an impact-based priority judgment; I retain P2 because exposure is conditional and no project rule resolves urgency.
3. **P2 — COR-3/ARC-2: an empty diagram file disables useful fallback. [RUNTIME-VERIFIED BINDING] [SOURCE-TRACED CONSUMER]** report_data.rs:234 selects a zero-byte PNG by is_file; attack-path.typ:69-81 chooses image() instead of Mermaid text. Reproduced binding is definite; final PDF failure has not been compiled. Validate usable candidates and continue searching alternative formats.

## Recommendation

Request targeted corrections and regression evidence before treating the new report path as ready. Also accept CQ-2's literal relationship-default violation as P2, supported by source contract and independently reported runtime reproduction; SEC-1's destructive symlink cleanup, SEC-2's bounded wrong-image selection, and PIPE-1's nonportable new test import warrant P2 fixes within their stated limits. These additional findings were not original independent discoveries by this reviewer.

Treat historical unversioned taxonomy interpretation, published-companion checks and stale-executable provenance as documented advisories pending stronger compatibility/attestation requirements. Do not silently remap taxonomy IDs or claim current PDF corruption.

## Verdict

**REQUEST CHANGES.** The architectural improvements are useful: checked attribution failures, shared MAESTRO states and coherent selected-tier summaries. The remaining defects arise where permissive parsers or filesystem existence are mistaken for valid evidence. My verdict does not depend on a P1 classification or agreement counts. No P0 established.

## Remaining Uncertainty

No full suite, PDF compilation, Windows cross-compilation, production-generation run or live database/browser validation was performed by this reviewer. The pinned CLI build and three fixtures completed successfully; evidence lives under /private/tmp/overseer-correctness-build and /private/tmp/overseer-correctness-DpxDFG. Some findings from other reviewers have runtime proof I did not independently rerun. Main source is pinned to 8df554e884b1e5dd24146111a965597eff5f4779, base dd3b293d81d358d1ae27424be83b720693539112. This is a focused correctness verdict, not an exhaustive assessment of every changed surface.


---

# architecture — Phase 7 Blind Final

<!-- Source artifact: state/reviewer_architecture_phase_7.md -->

# Phase 7 — Architecture Critic Blind Final

Consulted my own prior positions and Phase 6 summaries; no other Phase 7 output read. No new probes performed.

## Final Score

**7/10.** The shared Rust report path, checked adapter propagation, MAESTRO state model, and staged baseline publication are sound architectural improvements. Remaining defects concentrate at data-validity and filesystem boundaries rather than requiring a redesign.

## Top 3 Points

1. **P2 — Higher-tier promotion needs a validated assessment contract.** Retain ARC-1/COR-1 and the merged controls issue COR-2/DA-1. Runtime evidence establishes malformed risk rows replacing valid findings and incomplete control evidence producing false-zero residual output. Controls inventory, recognized headers, and completed residual assessment are different states. Fix both controls acceptance branches and the separate risk parser/selection boundary; preserve legitimate completed-empty assessments. My original ARC-1 P1 is superseded by P2 because completed-input or ordinary-workflow exposure remains unestablished. The judge should retain DA's explicit consequence-based P1 disagreement rather than erase it through vote counting.
2. **P2 — Consolidate asset-selection safety while retaining distinct defects.** ARC-2/COR-3's zero-byte selection, SEC-2's path-valued ID escape, and SEC-1's symlink-dependent cleanup need separate regression tests. An image resolver should establish containment and usability; cleanup must establish that retained bytes survive unlink. Runtime tests establish wrong bindings and cleanup deletion. Empty-image PDF failure and neighboring-image disclosure remain downstream inferences until compilation is verified.
3. **Separate explicit contracts from desirable extensions.** CQ-2's relationship default and PIPE-1's Unix test import are bounded P2 defects. Historic taxonomy edition handling, companion PDF policing, and stale-executable provenance deserve documented advisories, but current source and prescribed workflows do not establish the broader mandatory guarantees assumed by their strongest initial wording. Do not claim committed PDFs are corrupt or require a build-attestation subsystem without an agreed contract.

## Recommendation

**Request targeted corrections.** Address the demonstrated report-tier, asset, default-field, and platform-test defects, then run focused regressions. Keep advisory scope decisions separate from defect acceptance. A complete worktree/test/CI readiness assessment remains outside this review result.

## Verdict

The architecture is worth retaining; the newly connected report pipeline currently trusts partial parser and filesystem results too early. Correct those boundaries before accepting its successful outputs as reliable assessments.

## Remaining Uncertainty

P1 versus P2 for partial-controls false-zero output remains a priority judgment; my minimum justified priority is P2. No production frequency or downstream automated security decision has been demonstrated. Runtime evidence was contributed by other panelists and is not claimed as personally executed by me. Full PDF rendering, Windows target compilation, browser/auth flows, PostgreSQL replay, and full workspace tests remain unexecuted in my review. Catalog crash/concurrency behavior and historic-edition compatibility are not promoted to established defects. These limits constrain verification claims without negating the precise reproduced or source-proven failures.


---

# security — Phase 7 Blind Final

<!-- Source artifact: state/reviewer_security_phase_7.md -->

# Security Auditor — Phase 7 Blind Final

Independent final using own prior assessments, debate synthesis already considered, and pinned source context. No other final reviews read. No new probes.

## Final Score

**7.0/10.** Reduced from my original 7.5 because debate supplied concrete additional report-tier failure paths and my independent omitted-relationship probe confirmed another supported-input failure. This is an overall bounded implementation assessment, not a CVSS rating or an estimate of production exploitability.

## Top 3 Points

1. **P2 — cleanup can delete the only image bytes.** `crates/tachi-core/src/assets.rs:43-51` accepts byte equality between the mislabeled regular file and a correctly named symlink to that same file, then unlinks the backing file. Direct execution of pinned source proved both image paths unreadable afterward. This is the strongest security-review finding because it has a concrete destructive result without remote exposure or compiler assumptions. Opt-in cleanup does not authorize loss of the sole retained image.
2. **P2 — report-tier promotion trusts incomplete or malformed evidence.** The risk branch at `report_data.rs:164-168`, controls early return at `:289`, and populated-but-unparsed table acceptance at `:306-340` permit valid selected findings to disappear from details and counts. Distinct fixtures were runtime-reproduced. Preserve all three regression cases when deduplicating. I recommend P2 with high behavioral confidence, while retaining the unresolved impact-weighted P1 view for the judge. No production frequency or downstream release/authorization decision was established, and source artifacts are not deleted.
3. **P2 — new input contracts need narrower validation.** SEC-2's tree-ID path construction at `report_data.rs:232-244` selects neighboring image paths outside the report directory, proved by the pinned CLI. Treat disclosure as conditional on broader Typst root, compilation and sharing; no RCE or arbitrary text-file read. Separately, nested attribution deserialization at `parsers/findings.rs:497` rejects the documented omitted-relationship default; I reproduced exit 1 with a valid nested record. These are distinct fixes, not one generic sanitization issue.

## Recommendation

Request targeted fixes and regressions for demonstrated defects: safe retained-image independence, tier validation/completion checks, image ID/containment and usability, nested relationship default, and the localized unguarded Unix test import. Keep historical taxonomy interpretation, PDF companion coverage, and stale direct-binary provenance as nonblocking advisories pending explicit contract decisions. Existing checked primary parsing, fixed cleanup stems, catalog staging/rollback, Next.js getUser checks and own-profile RLS policies are meaningful safeguards and should be preserved.

## Verdict

**REQUEST CHANGES.** No established P0/P1 security vulnerability. My proposed priorities are P2 for concrete current failures, with bounded P3/advisory follow-ups for ambiguous compatibility/provenance guarantees. The controls priority disagreement should remain visible rather than disappear through majority agreement. Agreement and regenerated golden equality are not independent verification of semantic correctness.

## Remaining Uncertainty

No full PDF compilation, live database/RLS replay, browser authentication scenario, Windows cross-compilation, production deployment or complete workspace suite was performed by me. Actual image-path selection and cleanup loss were independently executed; final image rendering/disclosure is conditional and source-traced. Nested default rejection was independently executed. Other report-tier runtime evidence came from panel reproductions and was reconciled with source. No claim that current committed PDFs are corrupt or that an unused Prisma client exposes an endpoint. Findings are tied to pinned main `8df554e884b1e5dd24146111a965597eff5f4779` relative to `dd3b293d81d358d1ae27424be83b720693539112`.


---

# devils_advocate — Phase 7 Blind Final

<!-- Source artifact: state/reviewer_devils_advocate_phase_7.md -->

# Phase 7 — Devil's Advocate blind final

Read own prior outputs and Phase 6 summaries only for this final; no other Phase 7 output read. No new probes.

## Final Score

**6.5/10.** Increased from my initial 6/10 because catalog companion and executable provenance concerns are bounded advisories under the documented contract, not additional demonstrated blocking defects. Significant report-tier validation gaps still require correction. A broad useful implementation can pass its own baselines while preserving a semantic false-negative; test volume alone cannot remove that concern.

## Top 3 Points

1. **Controls assessment acceptance can erase valid High findings.** DA-1 is runtime-reproduced: a parseable Missing control descriptor without residual assessment evidence selects Tier 1 and emits zero findings. Merge with COR-2 but retain both controls-only and populated-unparsed-table regressions. Proposed **P1 with Medium priority confidence**, because successful misleading security output has substantial consequence; **P2 is defensible** given incomplete input and unmeasured exposure. No policy or incident evidence resolves priority. Technical confidence is High.
2. **Validity checks must precede authoritative tier replacement.** COR-1/ARC-1 establishes the same trust error through a different risk-table ingress: a nonempty parser vector can contain an entirely blank finding. Treat this as its own bounded validation fix and preserve legitimately completed empty controls assessments. Report/golden agreement cannot prove that a selected assessment is valid.
3. **Preservation and compatibility guards need precise contracts.** Retained-symlink cleanup can destroy the only image bytes; unusable image selection defeats text fallback; nested attribution omits the documented default; Unix-only test imports are unguarded. These bounded issues have stronger correction grounds than extending the catalog check beyond its documented baseline scope. Companion integrity and stale executable provenance remain advisory, as does historical taxonomy compatibility until its edition policy is explicit.

## Recommendation

Request targeted changes before relying on the new report path for assessment output. Validate risk rows and controls completion with negative fixtures that retain known input findings or return checked errors. Preserve both controls guard-bypass cases when merging findings. Correct the independent asset/default/platform defects and execute the relevant regressions. Do not impose source authentication, companion hashing or automatic historical-ID remapping as unstated release gates.

## Verdict

**REQUEST CHANGES.** No P0 established. My only priority dissent is proposed P1 for incomplete-controls false-zero output; the final judge may choose P2 without disputing the reproduction. Do not describe source inputs as deleted by the tier bug, current committed PDFs as corrupt, old taxonomy migration as unambiguously promised, or malformed-artifact exposure as measured normal production behavior.

## Remaining Uncertainty

No full workspace suite, production report evaluation, fault injection, concurrent publisher stress, stale-binary reproduction, Windows cross-compile or full Typst compilation was performed by me. DA-1 and companion-integrity behavior were reproduced from freshly built pinned source with private fixtures. Other findings rely on the panel's individually labeled runtime/source evidence. The companion check gap is real but its expansion is advisory. Crash durability remains unverified and ordinary-error rollback remains acknowledged. Shared model and shared Phase 2 trace limit reviewer independence; consensus does not replace direct verification.


---

# code_quality — Phase 7 Blind Final

<!-- Source artifact: state/reviewer_code_quality_phase_7.md -->

# Phase 7 — Code Quality Auditor blind final

Independent assessment after own reviews/debate and shared Phase 6 summary. No other persona's Phase 7 output was read. No new probe or product edit.

## Final Score

**7/10.** The implementation makes substantial useful improvements in checked error propagation, finding identity preservation and explicit evaluation states. Narrow but material input-contract and filesystem-edge defects remain. Score reflects correctness of the reviewed changes, not a claim of complete release readiness or exhaustive coverage.

## Top 3 Points

1. **P2: Tier selection can silently suppress known findings.** Risk-row nonemptiness and controls completion evidence are insufficient before replacing the authoritative findings vector. Runtime evidence supports malformed risk rows, control inventory without residual assessment, and populated but unparsed controls tables. Keep risk and controls remediation distinct and preserve both controls triggers in tests. My final priority is P2; the DA minority P1 view remains reasonable but depends on a stronger urgency/operational-impact interpretation. Completed empty assessments are legitimate.
2. **P2: Nested source attribution violates its literal default contract.** CQ-2 is personally runtime-confirmed: omitted relationship in the nested representation yields exit 1 with missing-field error, while equivalent flat input succeeds and defaults to primary. `schemas/finding.yaml:277-290` supplies explicit contract evidence. A serde primary default plus representation-parity tests is a bounded correction.
3. **P2: Optional artifact handling has unsafe edge cases.** The panel's confirmed cleanup symlink case loses the sole backing bytes; attack image traversal selects neighboring images; zero-byte diagram selection prevents fallback. These are separate bug classes with separate guards. Cross-platform test compilation also needs the unguarded Unix import fixed. Do not present these as a single generalized remote exploit or imply PDF execution was verified.

## Recommendation

**Request targeted changes** before declaring the reviewed implementation complete. Add meaningful regressions for the demonstrated boundaries and preserve the existing positive contracts. No broad redesign is needed for the confirmed P2 defects. CQ-1 historical versionless taxonomy semantics, DA-2 published-companion checksum scope, and PIPE-2 stale executable provenance should remain **P3/nonblocking advisories** unless supported-product requirements establish stronger promises. Their mechanics do not automatically establish broken mandatory guarantees.

## Verdict

**REQUEST CHANGES — actionable P2 defects; no P0 and no P1 endorsed by this reviewer.** The highest-confidence original CQ finding is the nested relationship-default inconsistency. Historical taxonomy ID reuse is real, but I withdraw the original mandatory-P2 classification because old-edition regeneration support is underspecified. Record that narrowing transparently rather than counting it as a confirmed blocker.

## Remaining Uncertainty

This is a focused source/contract audit with one personally executed minimal-pair reproduction, supplemented during debate by other reviewers' pinned runtime evidence. It is not an exhaustive review of every changed file. No live-agent evaluation, complete suite, Windows cross-compile, full PDF compile, production data or end-to-end consumer decision was exercised by me. The score does not certify CI freshness, release status, frontend/database behavior, or external taxonomy publication accuracy. Correlated-model limitations remain despite independent review phases.


---

# pipeline — Phase 7 Blind Final

<!-- Source artifact: state/reviewer_pipeline_phase_7.md -->

# Pipeline Reviewer — Phase 7 Blind Final

Read my own prior assessments and the Phase 6 summaries. No other Phase 7 output was read. No new probes or product edits.

## Final Score

**7.0/10.** The implementation has useful checked boundaries, workflow permissions, pinned tools and staged publication, but the report-selection contract still permits misleading successful outputs. This score reflects the verified defects and bounded exposure, not an average of other reviewers’ scores. My original broad provenance concern is advisory rather than a mandatory defect.

## Top 3 Points

1. **[P2] Validate optional assessment completion before replacing findings.** Malformed risk rows and incomplete controls can suppress known High findings while returning success. Controls require two preserved regressions: a populated unparsed residual table and control inventory without completion evidence. Do not reject legitimate completed-empty assessments. Behavioral evidence is strong; ordinary-workflow frequency remains unmeasured, so I do not independently support P1.
2. **[P2] Preserve valid inputs and usable fallbacks at new boundaries.** Nested attribution must honor the explicit primary default. Optional attack images must not defeat fallback merely by existing, and cleanup must not unlink the only image backing a retained symlink. Those are concrete correctness contracts; decoder failure and downstream confidentiality claims retain their stated limits.
3. **[P2] Fix the unguarded Unix CLI test dependency; keep broader integrity proposals advisory.** PIPE-1 has deterministic non-Unix compilation consequences but no executed Windows check. PIPE-2 stale-binary provenance and DA-2 published-companion integrity extend narrower documented procedures/checker scope; neither currently establishes wrong committed PDFs or a mandatory release blocker.

## Recommendation

Request targeted corrections with regression coverage for the established P2 defects. Preserve the read-only workflow permissions, Typst pin, catalog input-drift detection, staging and ordinary-error rollback. Do not require an authenticated-build subsystem or widen checksum guarantees based solely on my initial provenance concern. Record same-source regeneration assumptions and consider companion checks as follow-up improvements.

## Verdict

**REQUEST CHANGES — bounded correctness fixes; no P0 and no independently established P1.** The unresolved controls priority dispute should remain explicit for the judge: DA emphasizes misleading zero-risk output, whereas I require more ordinary-path exposure evidence to elevate beyond P2. Priority disagreement does not weaken the demonstrated defect.

## Remaining Uncertainty

My pipeline review is source-oriented. Other reviewers supplied bounded report-data reproductions; I did not independently execute Windows compilation, full Typst rendering, production agent workflows, database/browser tests, or the complete workspace suite. No assertion of present published-PDF corruption, universal Windows support, crash-atomic publication, or production security harm is justified. Historical taxonomy compatibility remains a policy/advisory question. Shared-model review and graph staleness limit independent coverage; pinned source and explicit verification evidence remain authoritative.


---

# Phase 8: Completeness Audit

<!-- Source artifact: state/phase_8_audit.md -->

> **Persona Profile — phase_8_audit.md**
> Completeness Auditor; independent gap scan; Phase 8. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 8 — Independent Completeness Audit

## Coverage

Read context, personas, Phase 2, both Phase 6 summaries, all six Phase 7 finals, and relevant Correctness/Pipeline Phase 3 findings. Compared pinned main `8df554e884b1e5dd24146111a965597eff5f4779` with base `dd3b293d81d358d1ae27424be83b720693539112`. Reviewed the full seven-commit interval inventory before considering temporal claims; no new temporal/event-frequency claim is made. This audit searches for omissions, not panel score quality.

Additional focus: MAESTRO duplicate evidence/state reduction, actual curl process behavior, permissions malformed-input guards, workflow triggers, and scaffold runtime wiring. Graph search was first for MAESTRO discovery; pinned source overrides stale graph. Current runtime probes used the supplied pinned report-data binary and a freshly successful offline build of taxonomy-link-monitor into `/private/tmp/tachi-overseer-target`. No product edits or Python execution occurred.

## New Findings

### AUD-1 — P2 / DEFECT / High confidence: duplicate MAESTRO layer rows can discard positive evidence and report clean

**Source:** `crates/tachi-core/src/infographic.rs:273-289`, with state propagation at `:292-306` and attempted per-finding repair at `:443-474`.

**Trigger:** A summary-only threats.md contains the canonical `#### Risk by MAESTRO Layer` table with rows `L1 | 2 | High` and then `L1 | 0 | Clean`. Duplicate layer IDs are inconsistent evidence, but must not silently erase known positive evidence. A report containing layer summary evidence without detailed finding rows is supported by existing zero-finding/report-state tests.

**Observed:** The pinned report-data command succeeds and emits exactly `(layer-id: "L1", layer-name: "Foundation Model", finding-count: 0, coverage-state: "clean", coverage-label: "Evaluated — no findings")`, with `most-exposed-layer = ""`. Positive High evidence in the same input is lost. This is distinct from the panel optional risk/controls tier replacement findings.

**Base:** Base parser appended every parsed summary row to a Vec. Both rows survived and most-exposed selection retained the positive count. The new map insertion introduces last-row-wins evidence loss; no old clean-state contract is assumed.

**Guard check:** Inspected complete parser, classifier, extraction reconciliation and MAESTRO state tests. `classify_evaluation` correctly prioritizes a positive count within one row, but only runs before duplicate reduction. `extract_maestro_data` repairs from per-finding rows only when those rows exist; the reproducer has none. `verify_states` compares already-reduced output and cannot recover discarded source evidence. No duplicate-input rejection exists.

**Fix/regression:** Reject conflicting duplicate canonical layer IDs or merge conservatively so positive findings dominate clean/not-applicable states. Test both row orders, canonical label aliases, and summary-only inputs; counts must not be summed blindly across duplicates.

### AUD-2 — P2 / DEFECT / High confidence: exhausted HTTP redirects are reported as healthy citations

**Source:** `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:157-163`; curl error capture `:148-154`; HEAD-to-GET fallback `:113-119`.

**Trigger:** A citation endpoint responds with an endless HTTP 302 redirect to itself. Real curl follows redirects, terminates with exit 47, and still writes final HTTP code 302. Both HEAD and GET fail this way.

**Observed:** Using a private Node loopback server and real installed curl, freshly built pinned monitor returned success with summary `Checked 1 URLs: 1 healthy, 0 needs review, 0 broken, 0 transient` and JSON containing `http_status: 302`, `status: "healthy"`, and `error: "curl: (47) Maximum (50) redirects followed"`. The failed HEAD did invoke the fallback, but the final failure is ignored by the earlier 200..=399 classifier arm. This is an informational-monitor false negative, not CI bypass or evidence that any current external citation is broken.

**Base:** Entire binary is absent at base. Its new health classification introduces this defect.

**Guard check:** Reviewed complete curl invocation, fallback, classifier, summary and tests. Tests deliberately expect `classify(204, true) == "healthy"`, but no real redirect-loop probe challenges that policy. The error remains in JSON, which limits concealment, yet summary and status are wrong. Existing timeout, range request and retry guards do not make failed redirects healthy.

**Fix/regression:** Prioritize transport/process failures over a residual successful/redirect HTTP code, at least for redirect exhaustion; emit transient or needs-review rather than healthy. Add real local HTTP redirect-loop coverage and a classifier regression for 302 plus failure.

## Rejected Candidates

- Permissions allow/ask/deny category collisions: checker promises existing AC-2 membership consistency; combined set behavior matches that scope. JSON syntax, missing/non-array sections, nonstring and blank rules are rejected. No new category-policy guarantee established.
- Next.js redirect discards refreshed cookies: same redirect construction exists in base middleware; excluded as preexisting. Current getUser validation remains present.
- Prisma adapter is not tenant-context-bound: no newly exposed Prisma application endpoint established. Existing policy/verification checks and unused-client limitations prevent elevating a hypothetical cross-tenant issue.
- Curl invalid-link findings fail to gate CI: explicitly informational by workflow and command contract, so successful command exit itself is not a defect.
- MAESTRO component mapping as evidence: implementation explicitly avoids treating mappings alone as completed analysis. New finding concerns conflicting positive summary rows only.

## Limitations

Focused supplemental audit, not exhaustive scaffold/browser/SQL execution or whole-workspace validation. No external web request, live taxonomy health claim, PDF compilation, Windows build or production-frequency estimate. The first loopback attempt was blocked by sandbox socket policy; a narrow approved escalation then completed the actual-curl reproduction. The initial report-data invocation used an incorrect flag and was corrected before obtaining the quoted successful output. Binary build success is not a suite-pass claim. All agents share the same underlying model; panel independence is not model diversity. Memory registry was consulted only for graph/workspace orientation (MEMORY.md lines 58 and 67), not current behavioral evidence.


---

# Phase 9: Commands

<!-- Source artifact: state/phase_9_commands.md -->

# Phase 9 — Advisory verification commands

P0 count: zero. Remaining proposed P1: DA-1 controls false-zero. ARC-1 was downgraded to P2 during debate. Reviewers supplied runnable report-data probes and sed/awk source commands, outside this phase's narrow grep/cat/head/tail/wc allowlist. Those exact commands were not replayed here. Existing private-fixture reproductions remain evidence for later verification; their successful outcomes are not invented here.

One read-only source inspection adapted from DA-1's cited guard was run (below the five-command cap):

```sh
rtk proxy head -n 291 crates/tachi-core/src/report_data.rs | rtk proxy tail -n 7
```

Exit 0, raw output:
```rust
fn has_control_assessment(
    content: &str,
    data: &crate::compensating_controls::CompensatingControlsData,
) -> bool {
    if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() {
        return true;
    }
```

[CMD_CONFIRMED] The cited early-return guard exists and accepts nonempty control inventory independently of residual findings. This inspection does not establish input prevalence or P1 severity. No command contradicted a finding. Mutation/executable probes are outside Phase 9 and must retain their own evidence labels.


---

# Phase 10: Claim Verification

<!-- Source artifact: state/phase_10_claim_verification.md -->

> **Persona Profile — phase_10_claim_verification.md**
> Claim Verifier; exact source/base and evidence checks; Phase 10. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 10 — Claim Verification

## Scope and method

Verified the 12 unique finding mechanisms (13 panel IDs plus AUD-1/AUD-2, with duplicate IDs mapped below) against pinned HEAD `8df554e884b1e5dd24146111a965597eff5f4779` and base `dd3b293d81d358d1ae27424be83b720693539112`. Read context, all six Phase 3/7 artifacts, relevant Phase 4/5 corrections and Phase 8. Root codemap was read first. Graph search returned the older Vec-based MAESTRO parser, confirming the documented stale-index limitation; current numbered files and base objects govern citations. No product edits, Python, broad new review or full test-suite execution. Existing outputs were preferred to probes. Runtime evidence described as retained is inspected evidence from prior reviewers, not a claim that Phase 10 originally performed it.

## Verification Table

All file references below are relative to the pinned worktree. VERIFIED means the bounded mechanism is supported; it does not certify every original priority/impact extrapolation. INACCURATE identifies wording requiring correction; UNVERIFIABLE identifies an unexecuted or unsupported extension. No finding required MISATTRIBUTED or HALLUCINATED classification.

| Unique item / ID mapping | Classification | Checked current citations and source mechanism | Observed versus inferred; correction |
|---|---|---|---|
| 1. COR-1 = ARC-1, malformed risk | VERIFIED | `crates/tachi-core/src/report_data.rs:164-168`: `if !rows.is_empty()` replaces findings. `crates/tachi-core/src/parsers/findings.rs:158-178` maps arbitrary table rows using `unwrap_or_default()` for ID/severity; `report_data.rs:268-276` recomputes counts. | Retained baseline/actual outputs in `/private/tmp/overseer-correctness-DpxDFG` show High=1/Tier3 becoming High=0/Tier2 with one anonymous row. Source artifacts are not deleted. ARC-1 original P1 was withdrawn; priority is not an independently proven mechanism. |
| 2. COR-2 = DA-1, incomplete controls | VERIFIED | `report_data.rs:289-291` accepts any controls inventory; separate `:306-340` accepts a residual header without checking its rows were consumed. Replacement is `:174-177`. `crates/tachi-core/src/compensating_controls.rs:125-130` only parses exact severity subsections. | Replayed DA partial fixture: High=0, total=0, Tier1, findings=(). Inspected retained COR flat-table output with same result. Two distinct triggers require two tests. Calling the flat grouping canonical would be INACCURATE: it is malformed/noncanonical input unsafely accepted. Legitimate completed-empty assessments remain supported. P1/P2 urgency dispute is not settled by source verification. |
| 3. COR-3 = ARC-2, empty image | VERIFIED binding; UNVERIFIABLE executed PDF failure | `report_data.rs:227-247`, especially `:234` `.find(|path| path.is_file())`; `templates/tachi/security-report/attack-path.typ:69-81` calls `image()` before the Mermaid fallback. | Retained actual.typ sets has-image=true for zero-byte PNG. Source proves fallback bypass and masking later extensions. ARC-2 original unconditional “breaks PDF generation” must be qualified as inferred decoder consequence: no PDF compile is supplied. |
| 4. SEC-1, retained symlink cleanup | VERIFIED | `crates/tachi-core/src/assets.rs:41-51` selects mislabeled candidate by equality then `fs::remove_file`; `:61-64` equality reads through links. | Phase 10 read-only check confirms JPG absent and PNG a dangling symlink to it in `/tmp/tachi-sec-review.VKfdxm/fixture`. Prior harness output records before/after. This is loss of sole backing bytes, not arbitrary-root deletion. Current citations correct. |
| 5. SEC-2, path-valued tree ID | VERIFIED selection; UNVERIFIABLE disclosure execution | `crates/tachi-core/src/attack_trees.rs:111,124-130` accepts metadata/Attack Tree heading IDs; `report_data.rs:232-246` joins them into filesystem candidates; `attack-path.typ:74` is the image sink. | Prior pinned CLI output selects neighboring SVG via ../../ ID. Disclosure requires suitable compiler root, actual compile and sharing; none was executed. No RCE or arbitrary text read is established. The alternate colon-heading grammar check does not guard the two cited input branches. |
| 6. CQ-1, historical taxonomy meanings | VERIFIED source-edition collision; UNVERIFIABLE mandatory compatibility requirement | `schemas/taxonomy/owasp.yaml:471-473` now assigns LLM05 to Data and Model Poisoning. `crates/tachi-core/src/threats_sarif.rs:170-179` forces primary LLM references to 2026. Base catalog assigns LLM05 to Improper Output Handling; base fixture `tests/scripts/fixtures/source_attribution/valid_multi_record.md:27,33` describes HTML output and cites LLM05. | Source/base semantics are verified; external publication accuracy is explicitly not the claim. No edition dispatch found in attribution/SARIF paths. Old-report support obligation is underspecified: final P3 advisory is justified; initial mandatory P2 is not independently established. No automatic remap prescription is verified. |
| 7. CQ-2, nested relationship default | VERIFIED | `crates/tachi-core/src/parsers/findings.rs:63-67` requires relationship String without serde default; `:493-498` directly deserializes nested entries; flat default is precisely `:600`. `schemas/finding.yaml:277-289` explicitly promises primary default. | Phase 10 replay of nested fixture reports missing field relationship; earlier flat/nested minimal pair records flat success. Checked failure occurs before enum normalization. Original base supported flat default; new nested path creates inconsistency. Citation precision correction: use :600 rather than treating :598-600 as all default code. |
| 8. PIPE-1, Unix test import | VERIFIED source-level compile defect; UNVERIFIABLE executed Windows check | `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:200-203` uses only cfg(test) around unguarded Unix import; `:205-218` helper uses set_mode. | No cfg(unix) and no Cargo autobin exemption found. Deterministic target-specific dependency is established; no installed Windows target/run is claimed. This is test-target portability, not a statement that all runtime Windows behavior is supported. |
| 9. PIPE-2, stale executable provenance | VERIFIED source mechanism; UNVERIFIABLE reproduced mismatch / mandatory attestation guarantee | `crates/tachi-core/src/catalog_drift.rs:242` hashes supplied root; `:278` calls compiled Rust builder; `:319` rechecks disk inputs; `:329` records those hashes. | No embedded executable/source identity guard found. A binary A/root B mismatch follows from composition but was not executed. Documented same-checkout cargo run avoids the ordinary trigger; final nonblocking advisory corrects original overbroad defect framing. No current PDF corruption established. |
| 10. DA-2, companion PDF gap | VERIFIED checker scope; INACCURATE as proof current published PDFs are corrupt | `catalog_drift.rs:78-86` excludes PDF companions from inputs; `:183-202` validates registered baseline hashes only; `:313-317` publishes companions when present. | Phase 10 inspected the private companion containing `corrupt published PDF` and replayed catalog-drift --check successfully. Current committed PDFs are not claimed corrupt. Final advisory recognizes baseline scope; original title is misleading without hypothetical/fixture qualification. |
| 11. AUD-1, duplicate MAESTRO rows | VERIFIED | `crates/tachi-core/src/infographic.rs:273-289` uses `by_layer.insert`, replacing earlier same-ID evidence; `:292-306` emits reduced map; `:443-474` repairs only observed detailed finding rows. | Inspected `/private/tmp/overseer-audit-fNFVfo/threats.md` (L1 2 High then L1 0 Clean) and out.typ (L1 0 clean, most-exposed empty). `maestro_coverage.rs:35-36` correctly prioritizes positive counts per row but cannot recover discarded row. Base Vec push retained both rows; new map reduction introduces loss. |
| 12. AUD-2, redirect exhaustion healthy | VERIFIED | `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:113-119` retries failed HEAD with GET; `:148-154` captures curl failure alongside residual HTTP code; `:157-163` classifies 200..399 healthy before testing failure. | Inspected retained actual links.json: HTTP302, healthy, curl exit47 maximum redirects error. Prior Phase 8 real-curl local server probe is corroborated by source and retained artifact. Phase 10 did not restart network server. Error remains visible in JSON; informational success exit is intentional and not a CI bypass. |

## Missing-guard and introduction checks

Guard search covered the entire immediate selection/acceptance branches plus record schema, attack parser, asset helper, classifier and catalog publication boundaries. No risk header/identity validation precedes replacement; checked primary attribution parsing validates another input. Controls have useful numeric/empty-assessment guards, but inventory returns early and header recognition does not establish row consumption. Image selection never invokes assets.rs nonempty/signature checks or containment; cleanup has byte equality, fixed stems and opt-in behavior but no retained-path independence or symlink_metadata guard. Nested serde deserialization fails before validation and has no default. MAESTRO detailed-finding reconciliation is real but unavailable to summary-only evidence. Catalog input-symlink rejection, staging, source drift recheck and rollback are real; they do not expand checked outputs or attest executable identity. The monitor retries HEAD correctly; its final classifier still overrides failure with residual 302.

Base diff confirms new render_document_data tier/image wiring, new cleanup, nested attribution path, and Vec-to-map MAESTRO reduction. Entire catalog_drift module and taxonomy-link-monitor binary are additions. Preexisting permissive parsers are not independently blamed as new regressions; new composition is the introduction. Base report_data is 208 lines and lacks the new authoritative selection block. Source-edition comparison uses base/current repository catalog values only, not claims about OWASP publication correctness.

## Limits and disposition

Retain nine bounded defect groups (risk, controls, empty image, cleanup, path selection, relationship default, Unix tests, duplicate MAESTRO, redirect exhaustion), plus three explicit advisories (historical edition policy, executable provenance, companion coverage). That is 12 unique items, with COR/ARC and COR/DA duplicate mappings shown above. Original six reviewers contributed 13 IDs mapping to 10 unique mechanisms; AUD adds two. No claim was hallucinated or assigned to an unrelated symbol. Severity disagreement remains a judgment for Phase 11; vote counts do not verify claims. No PDF execution, Windows compilation, production prevalence, external taxonomy accuracy, live database/browser behavior, crash durability or full release readiness was verified. Stop after Phase 10.


---

# Phase 11: Severity Verification

<!-- Source artifact: state/phase_11_severity_verification.md -->

> **Persona Profile — phase_11_severity_verification.md**
> Severity Verifier; bounded impact and safeguard analysis; Phase 11. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 11 — Severity Verification

Reviewed pinned main `8df554e884b1e5dd24146111a965597eff5f4779`. Read context, Phase 10 claim verification, both Phase 6 summaries, original/reflection/debate arguments for DA-1/COR-2 and ARC-1, and AUD-1/AUD-2. Re-read the relevant current source and controls contract tests. This phase adjudicates priority; prior runtime reproductions remain attributed to their originating phases. No product changes, Python, new runtime probes, or suite-pass claims.

## Decision table

“Actual priority” is the minimum evidence-justified priority selected here, not a measurement or panel vote. EXISTING_DEFECT means current executable behavior violates the bounded input/output requirement. PLAN_RISK means an unpromised future guarantee or broader contract would need a decision before a mandatory defect can be established.

| Item | Panel priority | Verified | Actual priority | Reason | Source | Domain type / classification |
|---|---|---|---|---|---|---|
| DA-1 = COR-2, incomplete controls | P1 dissent; five P2; mechanism unanimous | Yes, both independent triggers | **P2**, retain P1 dissent | Successful report can hide an existing High finding; credible impact is wrong remediation prioritization if a reader trusts the report. Trigger requires promoting an incomplete local inventory or noncanonical residual table into an authoritative assessment. This bounded report-input defect warrants correction; an urgent generalized exposure or deployment blocker is not established. | `report_data.rs:172-177,268-291,306-340`; `compensating_controls.rs:125-130,257-294`; Phase 10 item 2 | Internal code/contract; **EXISTING_DEFECT** |
| ARC-1 = COR-1, malformed risk | Original P1; Architecture withdrew to P2; DA discussed P1 impact rationale | Yes | **P2** | Arbitrary nonempty table replaces valid findings with blank fields and zero severity counts. An incorrect successful assessment is material, but this requires a malformed optional local table; no additional privilege, remote execution or input destruction follows. Same report-input prioritization criterion as controls. | `report_data.rs:164-168,268-276`; `parsers/findings.rs:158-178`; Phase 10 item 1 | Internal code/contract; **EXISTING_DEFECT** |
| AUD-1, duplicate MAESTRO evidence | P2 | Yes | **P2** | Contradictory duplicate local summary rows can erase positive High evidence and mark a layer clean. Summary-only input is supported, so detailed-finding repair is not a universal defense. Bounded inconsistent-input handling merits a fix without escalating beyond the report integrity cases above. | `infographic.rs:273-306,443-474`; Phase 8 and Phase 10 item 11 | Internal code/contract; **EXISTING_DEFECT** |
| AUD-2, redirect exhaustion | P2 | Yes | **P2** | Real failed redirect traversal receives a healthy status; both HEAD and GET retries are exhausted. False citation-health reporting is bounded to an informational monitor; the error remains visible and its success exit is intentional. No CI security-gate bypass is claimed. | `taxonomy-link-monitor.rs:113-119,148-163`; Phase 8 and Phase 10 item 12 | Internal classifier plus retained local runtime evidence; **EXISTING_DEFECT** |
| P0 inventory | None proposed | No P0 mechanism established | **None** | No unconditional severe outage, broad compromise or unrecoverable system-wide loss is shown by these findings. This is a bounded evidence conclusion, not a general security certification. | Phase 3/6/10 review record | Internal review scope |

All Rust paths in the table are beneath `crates/tachi-core/src/` except `taxonomy-link-monitor.rs`, which is beneath `crates/tachi-cli/src/bin/`.

## Exact guards and why they do not resolve the defects

Controls acceptance begins with `if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() { return true; }` at `report_data.rs:289-291`. A descriptor parsed from `**Status**: Missing | **Effectiveness**: None` populates controls, so it bypasses residual-table and numeric-completion checks. The selected vector at line 177 can consequently be empty. Separately, lines 306-340 recognize the six-column residual header and separator but never establish that its nonempty rows were consumed. Extraction at `compensating_controls.rs:125-130` visits only exact severity subsections. These require **two regression cases and two guard corrections**, not a single fix to the header path.

The existing tests at `crates/tachi-core/tests/report_document_contract.rs:249-272,311-347` support both rejecting incomplete stubs and preserving explicitly completed zero-residual assessments. The incomplete-stub set does not contain the parseable inventory descriptor. The documented schema at `templates/tachi/output-schemas/compensating-controls.md:75-101` groups rows by severity. That constrains the flat-table trigger; it does not justify silently accepting it as an empty completed assessment. Counting every raw finding as residual would also be wrong.

Risk selection checks only `!rows.is_empty()`. Missing fields default to empty strings before that check. Primary attribution validation protects another input; controls completion guards do not run for this risk-only fixture. Recomputed counts faithfully reflect the wrong selected vector and cannot detect the loss.

MAESTRO per-row positive-count classification runs before `by_layer.insert` overwrites an earlier duplicate. Reconciliation at lines 443-474 needs detailed finding rows, absent in the summary-only fixture. Curl's HEAD retry runs correctly, but `200..=399 => "healthy"` precedes the failed-process branch at lines 157-163, overriding the final error with residual 302.

## Impact, trust context and unresolved judgment

The controls P1 argument is credible: an interrupted or manually assembled local artifact needs only a short parseable fragment, and a user may trust successful zero-finding output without checking the preserved inputs. No actual harmed user is necessary to establish that potential consequence. Missing prevalence measurements are not evidence of rarity. Likewise, source-file survival does not restore the accuracy of the emitted report.

The P2 decision instead weighs the positively identified preconditions and boundary: local report artifacts must be incomplete or inconsistent, promotion occurs inside a report-generation workflow, and the demonstrated effect is the selected assessment projection. This record does not establish an automatic security-enforcement decision, remote untrusted service input, or completed canonical producer path hitting the defect. Those are limits of the proven failure, not prerequisites that every P1 must universally satisfy. The project has no inspected explicit priority rule resolving this tradeoff. Retain DA's **P1 / Medium priority confidence** dissent for the judge; the five-to-one vote is not evidence. A project policy prioritizing every successful suppression of known security findings urgently could reasonably select P1 without changing the verified mechanism.

No other severity is inflated. COR-3/ARC-2, SEC-1, SEC-2, CQ-2 and PIPE-1 retain bounded P2 EXISTING_DEFECT treatment from Phase 10, including its limits on unexecuted PDF disclosure/failure and Windows compilation. CQ-1, PIPE-2 and DA-2 retain P3 advisory treatment: classify their stronger historical-compatibility, arbitrary-root executable-attestation and companion-checking obligations as **PLAN_RISK / contract clarification**, not newly verified mandatory defects. Their existing source mechanics are real; unpromised guarantees and current corrupted committed PDFs are not established. Overall disposition remains nine unique bounded P2 defect groups and three P3 advisories, with the controls P1 dissent explicitly preserved.

## External-domain check and verification limits

Every core severity claim above is internal code/contract behavior. AUD-2 relies on retained real-curl loopback evidence and the inspected classifier, not on current internet endpoint status or a disputed external standard. Taxonomy publication accuracy, legal/compliance obligations, CVE severity, and current vendor guarantees are not premises. **External-domain dependency: none; web verification deliberately skipped.** No stale graph snippet was used as source truth. Memory `MEMORY.md:58` was orientation only for graph coverage; the pinned files and review artifacts supply current evidence. The preferred lean-ctx wrapper was unavailable, so subsequent shell commands used `rtk proxy`. No production frequency, final PDF behavior, full release readiness or external taxonomy truth was independently verified in this phase.


---

# Phase 12a: Draft Tiers

<!-- Source artifact: state/phase_12a_tiers.md -->

# Phase 12a — Confidence-based draft tiers

Phases 8–11 complete. Nine bounded defects plus three advisories; all citations checked. This stage targets unresolved priority, unexecuted image consequence and conditional advisory contracts, not a second general review.

| Point | Claim / IDs | Draft tier | Signal | Suggested lens |
|---|---|---|---|---|
| 1 | Controls false-zero P1 vs P2 (DA-1/COR-2) | Deep | Medium priority confidence; remained disputed across two rounds. Mechanism already reproduced. | Correctness / risk |
| 2 | Empty image prevents compilation (COR-3/ARC-2) | Deep | Runtime decoder consequence unexecuted; binary and pinned Typst are locally available. | Reporting / reliability |
| 3 | Legacy OWASP input compatibility obligation (CQ-1) | Standard | Medium confidence, cross-file contract and historical edition interpretation. Source collision verified; no automatic migration promise established. | Architecture / schema compatibility |
| 4 | Executable-to-source provenance guarantee (PIPE-2) | Standard | Medium confidence, supported invocation vs documented cargo procedure; cross-file contract. Mechanism inferred, current artifacts not shown wrong. | Build / release reliability |

DA-2 companion gap is reproduced and its scope dispute resolved to P3 advisory; no further verification is required. SEC-2 path selection is verified; disclosure stays conditional on compile/root/sharing and is not asserted as an executed consequence. Windows compilation is a deterministic source-target defect with execution limitation, not an unresolved semantic dispute. No new web claim needs external research.

Capabilities: Standard may read/trace multiple files and run static analysis; Deep may run focused private-fixture probes. Neither may edit product files or use Python. Priority advice must not require observed real-world harm as a prerequisite for P1, nor assume unseen production exposure. All points must retain explicit limits.


---

# Phase 12b: Tier Advisor

<!-- Source artifact: state/phase_12b_tiers.md -->

> **Persona Profile — phase_12b_tiers.md**
> Tier Advisor; domain-neutral confidence and capability refinement; Phase 12b. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 12b — Neutral tier advice

Scope: refine the four Phase 12a points only. Read the context, both Phase 6 summaries, relevant Phase 7 finals (Architecture, Code Quality, Pipeline and Devil's Advocate), Phase 10 claim verification and Phase 11 severity verification. No new general review, source verification, runtime reproduction or product edits were performed. Tier measures verification depth, not defect priority.

| Point | Draft | FinalTier | OverrideReason | Persona |
|---|---|---|---|---|
| 1. Controls false-zero priority, DA-1/COR-2 | Deep | **Deep** | No override. The substantive P1/P2 dispute survived two debate rounds, so the two-round priority rule retains Deep even though the mechanism is already reproduced and Phase 11 selected P2. A third vote or repeat mechanism probe will not resolve urgency. Use a focused independent assessment of impact, positively established trigger boundaries and any applicable project priority contract; preserve P1 dissent and both controls triggers. | Independent correctness and risk adjudicator; challenge both impact inflation and unsupported assumptions of rarity. |
| 2. Empty image compilation consequence, COR-3/ARC-2 | Deep | **Deep** | No override. Static verification establishes image selection and fallback bypass, but cannot claim an executed decoder failure. A private fixture compiled with the pinned Typst and report template is the minimum sufficient way to resolve this specific runtime uncertainty. Compare zero-byte selected PNG with the valid fallback path and record command, exit and diagnostic. Do not expand into full report or release testing. | Reporting reliability specialist with Rust-to-Typst integration experience. |
| 3. Historical OWASP input compatibility obligation, CQ-1 | Standard | **Standard** | No override. Source/base edition collision is already verified. The uncertainty is whether supported historical input regeneration promises edition preservation, not whether another fixture can reproduce ID reuse. Trace explicit input/schema, version and migration contracts; absent a promise, retain the bounded P3 advisory. Deep execution cannot create a missing support obligation. | Architecture and schema compatibility specialist; distinguish repository edition semantics from external taxonomy publication claims. |
| 4. Executable-to-source provenance guarantee, PIPE-2 | Standard | **Standard** | No override. Inspect CLI invocation and documented regeneration contracts against the compiled-builder/supplied-root composition. Same-checkout cargo invocation constrains the ordinary trigger; arbitrary binary/root attestation is not established as a promised guarantee. A stale-binary probe could prove a conditional mechanism but would not establish that obligation, so it is unnecessary for the current advisory. | Build and release reliability specialist focused on supported invocation, provenance boundaries and documented guarantees. |

## Bounded verification instructions

Point 1 remains Deep because of the two-round dispute, without requiring unnecessary new execution. Phase 11's P2 is a reasoned input, not a binding outcome or majority proof. Successful suppression of known High findings is established; real-world harm is not a prerequisite for considering P1. Conversely, do not assume an automated security gate, routine production exposure, or a completed canonical producer trigger. Preserve legitimately completed zero-residual assessments and the separate controls-inventory and populated-unparsed-table cases. If no additional policy or contract resolves urgency, present the reasoned minimum priority and explicit dissent to the neutral judge.

For point 2, use isolated temporary inputs and outputs; retain enough evidence to distinguish image decode failure from unrelated missing assets, packages, fonts or setup. If the valid control cannot compile, record the probe as inconclusive rather than converting an environment failure into proof. A successful focused probe establishes only that exercised rendering path, not neighboring-file disclosure or all PDF generation behavior.

For points 3 and 4, stop once the narrow contract question is resolved or remains explicitly unspecified. Escalate only if new concrete contract evidence makes runtime behavior material to a mandatory guarantee; any escalation must name that evidence. Do not prescribe automatic historic-ID remapping or an authenticated build subsystem without a supported requirement.

## Capability limits

Standard permits bounded multi-file reading, contract tracing and static analysis. Deep additionally permits focused private-fixture execution where it can answer the selected question; Deep does not require runtime execution for a priority-only disagreement. All shell commands must start with rtk; no Python, product edits, network publication, tracker changes or broad test campaign. Only assigned review artifacts may be written. The preferred lean-ctx path is unavailable in this environment, so shell reads used rtk proxy directly.

Do not reopen resolved DA-2 companion scope, conditional SEC-2 disclosure, or Windows execution merely to broaden this stage. No external publication claim is needed; consult primary external sources only if one becomes a premise. Shared-model correlation and inherited evidence remain limitations: this advisor did not independently rerun the underlying reproductions, verify current tool availability, or certify release readiness. Final tiers recommend the minimum sufficient depth while preserving the mandatory two-round priority treatment.


---

# Phase 13: phase_13_point_1_controls.md

<!-- Source artifact: state/phase_13_point_1_controls.md -->

> **Persona Profile — phase_13_point_1_controls.md**
> Correctness/Risk Verifier; matched to controls priority dispute; Deep; Phase 13 point 1. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 13 point 1 — Controls false-zero priority

Verdict: **VR_PARTIAL** for the proposition that the demonstrated controls defect requires P1. **Minimum recommended severity: P2; preserve P1 dissent with Medium priority confidence.** The defect and security-report integrity consequence are confirmed; the mandatory urgency classification remains a judgment. This does not refute P1 or require an actual incident before P1 is permissible.

## Scope and method

Reviewed the pinned worktree `/Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004` at the context-recorded main `8df554e884b1e5dd24146111a965597eff5f4779`. Read root codemap, context, Phase 12a/12b, Phase 11, Phase 10 controls evidence, DA/COR Phase 3 and Phase 7, and the exact current source/test/schema/template ranges below. A graph-first `search_graph` query for controls in report_data returned no matching nodes; direct pinned-file reads were therefore used. Existing runtime reproductions are attributed to their originating reviewers; this phase did not repeat them. No product edits, Python, broad review, external research, tracker changes, or new suite-pass claims.

The preferred `/usr/local/bin/lean-ctx` is absent (exit 127). After reading RTK instructions, focused commands used `rtk read` or `rtk proxy`. A narrow memory registry search produced no relevant match; no memory fact supplies this verdict.

## Confirmed evidence and boundaries

1. **Inventory-only trigger, DA-1.** Phase 3 retains a fixture at `/private/tmp/tachi-da-partial.0QxF1C`: a valid High S-1 plus `## 3. Control Details`, `### Authentication`, and `**Status**: Missing | **Effectiveness**: None`. The report succeeded with `#let high-count = 0`, `#let total-findings = 0`, `#let data-source-tier = 1`, and `#let findings = ()`. Phase 10 replayed this evidence. Current `crates/tachi-core/src/compensating_controls.rs:257-259,294-300` parses that descriptor and pushes a control. The accepting guard at `crates/tachi-core/src/report_data.rs:289-291` says exactly:

   `if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() { return true; }`

   That is evidence of inventory presence, not assessment completion. At `report_data.rs:174-177`, successful acceptance selects tier 1 and replaces the findings with `data.findings`; lines 268-276 derive zero totals from the replacement. An absent or ineffective control can therefore accompany an apparently finding-free output without any evidence of mitigation.

2. **Populated but unparsed table trigger, COR-2.** Phase 3 retains `/private/tmp/overseer-correctness-DpxDFG/unparsed-controls`, with a populated High S-1 residual row directly beneath Coverage Matrix. Phase 10 inspected its successful false-zero output. `report_data.rs:306-340` accepts the six-column residual header and separator without checking consumption of its data rows. By contrast, `compensating_controls.rs:125-130` iterates only `### {severity_label} Residual Severity`. This is a distinct acceptance defect. The schema explicitly states at `templates/tachi/output-schemas/compensating-controls.md:77`: “Threats grouped by residual severity (Critical first, then High, Medium, Low).” The flat grouping is noncanonical; it must not be represented as canonical producer output. Its successful promotion still violates the bounded input-validation requirement.

3. **Report-facing consequence.** `templates/tachi/security-report/findings-detail.typ:192-202` checks `if findings.len() == 0` and renders “No findings to display.” This consumer is source-traced here, not newly PDF-compiled. The demonstrated report bindings already suppress known High evidence. Source-file survival does not correct that artifact or prevent a reader from making the wrong remediation decision.

4. **Real contract, including legitimate zero.** `crates/tachi-core/tests/report_document_contract.rs:311-340`, named `control_stubs_do_not_erase_valid_risk_findings_but_empty_assessments_are_retained`, requires incomplete stubs to preserve Tier 2 and one finding. Its invalid cases omit the parseable inventory descriptor. Lines 342-346 explicitly preserve completed-empty Tier 1 output, as does `completed_empty_controls_preserve_the_assessment_and_control_metadata` at lines 249-267. Neither indiscriminately rejecting empty assessments nor retaining every inherent threat as residual is a valid fix.

## Priority reasoning and policy check

P1 has a concrete, substantial impact rationale: a short incomplete artifact is enough to make a successful security report omit an existing High finding, defeating a central reporting purpose. The trigger needs no exploitation technique beyond those file contents. A reader may consequently omit remediation. Actual observed harm, exploitability, a remote entry point, or an automated gate are **not universal prerequisites for P1**. Missing prevalence measurements do not establish rarity. Calling the defect merely cosmetic or harmless because its inputs survive would be wrong.

The positively established boundary is nevertheless an incomplete inventory or noncanonical residual table supplied to local report generation. The completed canonical producer path has not been shown to encounter either trigger, and the evidence proves a report projection failure rather than a demonstrated operational enforcement decision. Those facts bound the demonstrated case; they do not cancel its consequence. On this bounded record P2 is a defensible minimum: a material correctness defect requiring correction before reliance on the affected report path, without an established priority contract making this class automatically urgent.

A focused policy search covered AGENTS/CONTRIBUTING/SECURITY when present, the active roadmap, publish-readiness checklist and issue templates; it found no general P1/P2 assignment rule that settles this item. There is relevant **but nonbinding** project precedent: `docs/feature-roadmap-2026-10-04.md:27-31` assigns Truthful MAESTRO evaluation states P1 and requires “missing evidence never becomes clean.” That strengthens the P1 dissent by showing the project values avoiding unsupported clean states. It is scoped to MAESTRO delivery, not a blanket severity policy for all report-input defects. The same roadmap at lines 39-43 assigns Output-integrity preservation P2 and requires preservation of finding identity/attribution. Neither neighboring label mechanically determines DA-1/COR-2. These local contractual statements require no external publication claim or web check.

## Remaining judgment and disposition

No further mechanism probe would decide urgency. A product owner or neutral judge can reasonably choose P1 based on report trust and the simple partial-artifact trigger alone; no incident evidence is required. Choosing P2 must rest on the established input boundary and assessment of urgency, not unsupported rarity, panel majority, or a rule requiring remote compromise. This independent assessment recommends **P2 minimum, High mechanism confidence, Medium priority confidence**, retaining the explicit P1 position for the judge.

Keep DA-1/COR-2 as one controls finding with two separately tested acceptance paths. Require incomplete inventory and populated-unparsed table cases to preserve the known finding or produce a checked error; preserve completed-empty assessments. Review outcome remains request changes regardless of P1/P2 selection. This verification is not release certification, production-frequency evidence, canonical generator evaluation, or proof of user harm.


---

# Phase 13: phase_13_point_2_image.md

<!-- Source artifact: state/phase_13_point_2_image.md -->

> **Persona Profile — phase_13_point_2_image.md**
> Reporting/Reliability Verifier; matched to image decoder consequence; Deep; Phase 13 point 2. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 13 point 2 — Empty diagram image PDF consequence

**Tier: Deep. VR verdict: [VR_CONFIRMED] for COR-3/ARC-2's attack-tree binding, fallback bypass, and actual PDF compilation failure. Priority: retain P2. Confidence: high for the exercised path.** The formerly inferred decoder consequence is now executed: report-data succeeds with a zero-byte PNG, but the unchanged canonical main template fails to compile. Removing only that PNG from the private copied fixture restores successful PDF generation through the available Mermaid-source fallback.

## Scope and provenance

Read Phase 12a/12b, Phase 10, and COR-3/ARC-2 prior evidence. Repository atlas was read before source inspection. Graph discovery was attempted first; searches returned `project not found or not indexed`, including the available-project name, so exact existing source citations were inspected directly. This probe did not rely on graph contents. The preferred lean-ctx executable is absent; execution used `rtk proxy`. No Python, original-fixture mutation, product edits, network publication, or broad test campaign.

- Pinned worktree HEAD: `8df554e884b1e5dd24146111a965597eff5f4779`.
- Typst: `/private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst`; observed `typst 0.15.1 (9dfd3a08)`; SHA-256 `a6fb5786e71f6f95d8323b326353770734c1e4624e7c108783ea578723916b46`.
- Report CLI: `/private/tmp/tachi-overseer-target/debug/report-data`; SHA-256 `b6a7b534d0cede8ce9d01a4bdbbfe3b695ae706fcc547ff114fb01998f806318`. Used supplied pinned executable; this phase did not independently rebuild or attest executable/source correspondence.
- Original retained fixture: `/private/tmp/overseer-correctness-DpxDFG/empty-tree-image`.
- Private evidence tree: `/private/tmp/overseer-phase13-image-Sp9rCi`. Original zero-byte PNG remains in place; its SHA-256 is `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

## Executed commands

Executed from `/Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004`; the script below is the exact probe command with formatting expanded. `--root` includes both copied templates and copied input, so the failure is not a root-access restriction. All templates were copied unchanged; `cmp` also confirmed the copied main and attack-path templates equal their source files.

```sh
rtk proxy /private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst --version
rtk proxy sh -c '
probe_dir=$(mktemp -d /private/tmp/overseer-phase13-image-XXXXXX)
printf "%s\n" "$probe_dir" > /private/tmp/overseer-phase13-image-path
cp -R /private/tmp/overseer-correctness-DpxDFG/empty-tree-image "$probe_dir/fixture"
cp -R templates/tachi/security-report "$probe_dir/template"
/private/tmp/tachi-overseer-target/debug/report-data --target-dir "$probe_dir/fixture" --template-dir "$probe_dir/template" --output "$probe_dir/template/report-data.typ" > "$probe_dir/bad-generate.stdout" 2> "$probe_dir/bad-generate.stderr"
printf "bad_generate_exit=%s\n" "$?"
cp "$probe_dir/template/report-data.typ" "$probe_dir/bad-report-data.typ"
/private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst compile --root "$probe_dir" "$probe_dir/template/main.typ" "$probe_dir/bad.pdf" > "$probe_dir/bad-compile.stdout" 2> "$probe_dir/bad-compile.stderr"
printf "bad_compile_exit=%s\n" "$?"
mv "$probe_dir/fixture/attack-trees/S-1-attack-tree.png" "$probe_dir/zero-byte.png"
/private/tmp/tachi-overseer-target/debug/report-data --target-dir "$probe_dir/fixture" --template-dir "$probe_dir/template" --output "$probe_dir/template/report-data.typ" > "$probe_dir/control-generate.stdout" 2> "$probe_dir/control-generate.stderr"
printf "control_generate_exit=%s\n" "$?"
cp "$probe_dir/template/report-data.typ" "$probe_dir/control-report-data.typ"
/private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst compile --root "$probe_dir" "$probe_dir/template/main.typ" "$probe_dir/control.pdf" > "$probe_dir/control-compile.stdout" 2> "$probe_dir/control-compile.stderr"
printf "control_compile_exit=%s\n" "$?"
printf "probe_dir=%s\n" "$probe_dir"
cat "$probe_dir/bad-compile.stderr" "$probe_dir/control-compile.stderr"
ls -l "$probe_dir"/*.pdf
'
```

## Raw result and diagnostic

```text
bad_generate_exit=0
bad_compile_exit=1
control_generate_exit=0
control_compile_exit=0
probe_dir=/private/tmp/overseer-phase13-image-Sp9rCi
error: failed to decode image (unexpected end of file)
   ┌─ ../../../../../../private/tmp/overseer-phase13-image-Sp9rCi/template/attack-path.typ:74:6
   │
74 │       image(img-path, width: 100%, fit: "contain"),
   │       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

`control-compile.stderr` is zero bytes. `control.pdf` is 95,852 bytes; `file` identifies `PDF document, version 1.7, 12 pages`. No `bad.pdf` was created in the fresh private tree. The fixture's source is `graph TD` followed by `A --> B`. Both generated files retain `has-attack-trees = true` and the same S-1/Gateway/High entry and Mermaid source; line 58 of `bad-report-data.typ` has `"has-image": true,"image-path": "../fixture/attack-trees/S-1-attack-tree.png"`, whereas line 58 of `control-report-data.typ` has `"has-image": false,"image-path": ""`. The selected bad PNG is verified zero bytes.

## Causal citations and priority

- `crates/tachi-core/src/report_data.rs:227-239`: PNG/JPG/SVG candidate selection stops at the first `is_file()` result, without validating nonempty or decodable content.
- `crates/tachi-core/src/report_data.rs:243-247`: that presence becomes `has-image` and the relative image path, while Mermaid source remains available.
- `templates/tachi/security-report/attack-path.typ:69-81`: the selected image reaches `image()` at line 74; the alternative branch renders raw Mermaid source at line 79. This is source-text fallback, not Mermaid-to-vector diagram rendering.
- `crates/tachi-core/src/report_data.rs:255-259` shares image selection with attack chains. `templates/tachi/security-report/attack-chain.typ:83-92` likewise invokes `image()` but has **no Mermaid fallback**. Do not describe chains as bypassing a Mermaid fallback.

P2 remains appropriate: an optional incomplete diagram artifact deterministically blocks the entire exercised report PDF even though usable source fallback exists and report-data returns success. The narrow malformed-artifact trigger is established; routine exposure or broader outage urgency is not. Actual compilation strengthens the consequence evidence but does not itself imply P1. A targeted fix should reject unusable candidates, continue to valid alternate formats, and retain fallback; focused regression coverage should include this exact empty-PNG case.

## Limits

Only the attack-tree path was executed. The chain's analogous invalid-image risk is source-traced, not separately runtime-verified. No later-valid JPG/SVG masking probe, arbitrary corrupt nonempty image probe, or neighboring-file disclosure execution was performed. Successful control compilation isolates this failure from missing fonts, packages, assets, and compiler-root setup for this fixture; it does not certify all reports or visual quality. Existing templates and the original fixture were not modified. This is focused integration evidence, not release readiness or a full report test campaign. Persona independence is not model diversity.


---

# Phase 13: phase_13_point_3_history.md

<!-- Source artifact: state/phase_13_point_3_history.md -->

> **Persona Profile — phase_13_point_3_history.md**
> Architecture/Schema Verifier; matched to historical-input contract; Standard; Phase 13 point 3. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 13 point 3 — Historical OWASP compatibility contract

Verdict: **VR_PARTIAL**. Retain **CQ-1 as a bounded P3 advisory**. The repository promises preservation of explicit historical references during its contextual source-content cutover. The inspected contracts do not establish preservation or automatic migration of the original meaning of arbitrary historical **versionless** OWASP attribution when regenerated through the current runtime. The already verified source-edition collision remains supported; the claimed mandatory backward-compatibility obligation remains unestablished. This is not a finding that historical regeneration is forbidden or that semantic reinterpretation is harmless.

## Exact contract trail

1. Inputs: `state/phase_12a_tiers.md`, point 3; `state/phase_12b_tiers.md`, point 3 and bounded verification instructions; `state/phase_10_claim_verification.md`, CQ-1 row. These establish the narrow question, prior source/base collision verification, and Standard/static-only scope. All `state/` references here are relative to `docs/reviews/2026-10-04-main-48h/`. Root `codemap.md` was read before deeper exploration.
2. `docs/feature-roadmap-2026-10-04.md:15-19` says to reconcile active instructions, adapters, fixtures and generated outputs, preserve explicit historical references and the finding schema, and avoid global token replacement. The contextual explanation at `:71-75` specifically retains attack-chain provenance citing the 2025 source while describing the six baseline inputs as **current rendering fixtures** and correcting them by present category meaning. This is a meaningful source-editing constraint, but supplies no inference rule for a versionless imported record. The delivery plan repeats historical references remaining historical and schema unchanged at `docs/roadmap/2026-10-04-upstream-milestone-delivery.md:20`, and no global historical-ID replacement at `:26`.
3. `schemas/finding.yaml:118-127` permits free-form framework reference strings, illustrated with an explicit 2026 citation. In contrast, machine-readable attribution at `:254-289` contains taxonomy, id and relationship, with id required to resolve in `schemas/taxonomy/{taxonomy}.yaml` (`:270-276`). It declares no attribution edition field or historical-catalog dispatch. The runtime record agrees at `crates/tachi-core/src/parsers/findings.rs:63-68`; validation resolves IDs against the supplied catalog at `:342-370`, with loading from one taxonomy filename at `:390-398`. The narrower schema promise at `schemas/finding.yaml:250-253` preserves absent versus explicitly empty attribution; it is not a promise of historical taxonomy meaning.
4. Current SARIF reuse is concrete: `crates/tachi-core/src/threats_sarif.rs:166-179` copies attribution records, then formats primary LLM attribution as OWASP `<id>:2026`. Thus accepted historical versionless input can be presented as current-edition attribution; copying record bytes alone does not preserve historical semantics. This confirms the reason to document policy, without independently re-verifying the prior source/base collision or running a probe.
5. Generic compatibility language has a bounded scope. `docs/architecture/02_ADRs/ADR-028-source-attribution-schema-extension.md:87` explicitly argues byte preservation for five non-agentic baselines that omit attribution. `docs/architecture/02_ADRs/ADR-020-maestro-layer-classification.md:129-133` separately documents MAESTRO legacy parse aliases and schema-enum migration accountability. That precedent concerns MAESTRO/schema enum changes; it does not establish an OWASP catalog-edition migration promise. A taxonomy ID is a nonempty string in this schema, not an enum of OWASP category meanings.
6. `docs/feature-roadmap-2026-10-04.md:91-95` describes committed native PDF baselines and rendering fingerprints. `crates/tachi-core/tests/backward_compatibility.rs:8-9,94-118,139-146` exercises the registered current example inputs against their committed PDF bytes. This verifies a baseline rendering contract, not preservation of every previous user report or the historical semantics of a versionless citation. No test was executed in this verification.

## Actionable policy choice

Choose and document the input-edition rule before advertising historical regeneration support. The smallest current-scope option is to state that versionless OWASP attribution resolves against the current catalog, and require callers regenerating legacy material to review/re-attribute it with its source edition known. Alternatively, explicitly support edition preservation by carrying an edition at the input boundary and resolving the matching catalog, with a historical fixture regression. For ambiguous versionless material, request/require edition context or surface a diagnostic rather than silently guessing an old-to-new ID mapping. These are product policy alternatives, not a mandate to add automatic remapping, a new schema field, or a major-version migration in this review.

Priority stays P3 because the compatibility obligation is unspecified in the inspected authoritative surfaces; it is not lowered because observed production harm is absent. Elevate only upon a concrete supported historical-input preservation promise or an applicable migration requirement, then test that promised behavior. Current-source correctness and explicit historical prose preservation are separate obligations and are outside this narrowed point.

## Method and limits

Static numbered reads only, no product edits, Python, web, runtime probes or broad review. Initial preferred `/usr/local/bin/lean-ctx` invocation failed because the wrapper is absent; RTK instructions were then read and subsequent shell reads used `rtk proxy`. Graph discovery was attempted first: unqualified search reported no selected project; qualified search of `Volumes-dev-Git-SCM-tachi-rust` returned broad historical indexed sections, insufficient for this pinned-worktree contract question. Current numbered files therefore govern citations, consistent with Phase 10 documenting a stale index. Documentation/config literal searches are permitted fallback. One exploratory source path (`src/source_attribution.rs`) did not exist; the actual implementation was read in `src/parsers/findings.rs`. Large CHANGELOG search output was truncated and is not used as proof of absence. A quick memory keyword search supplied no applicable historical-edition contract and is not relied on. Absence is bounded to the cited schema, roadmap, ADR and runtime paths; no exhaustive repository-wide promise audit or external OWASP publication claim is made.


---

# Phase 13: phase_13_point_4_provenance.md

<!-- Source artifact: state/phase_13_point_4_provenance.md -->

> **Persona Profile — phase_13_point_4_provenance.md**
> Build/Release Verifier; matched to source/executable attestation contract; Standard; Phase 13 point 4. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 13 point 4 — Executable/source provenance contract

Verdict: **VR_PARTIAL**. Retain **PIPE-2 as a conditional P3 advisory (PLAN_RISK / contract clarification)**. The supplied-root hashes and compiled Rust builder can come from different revisions. That source-level mechanism is verified; a mandatory promise to detect or reject a mismatched executable is not established by the inspected contract. No mandatory defect or release blocker is warranted on this evidence. This does not certify that arbitrary cross-revision regeneration is correct.

## Evidence and contract trail

- Scope inputs: `state/phase_12a_tiers.md`, point 4; `state/phase_12b_tiers.md`, point 4 and bounded verification instructions; `state/phase_10_claim_verification.md:21,28`; and `state/phase_11_severity_verification.md:35`. All `state/` references are relative to `docs/reviews/2026-10-04-main-48h/`. Read the root codemap before source exploration. Pinned worktree HEAD was verified as `8df554e884b1e5dd24146111a965597eff5f4779`.
- Panel evolution: `state/reviewer_pipeline_phase_3.md:20-29` originally treated source association as mandatory P2. Phase 4 narrows confidence because the documented build procedure avoids the ordinary trigger. `state/reviewer_pipeline_phase_5_round1.md`, section PIPE-2, explicitly retracts mandatory framing in favor of P3; the Phase 7 final retains that qualification. Phase 5 round 2 concerns controls priority and supplies no additional provenance evidence. This verifier agrees based on the source and documentation below, not panel voting.
- Supported CLI shape is literal: `crates/tachi-cli/src/bin/catalog-drift.rs:39` prints `usage: catalog-drift (--check | --regenerate-baselines) [--root PATH] [--typst PATH]`. Root defaults to `.` at line 4; lines 10–12 accept a caller-selected path, and lines 21–24 dispatch directly into `tachi_core::catalog_drift`. Therefore a direct binary can select another source tree. Root selection alone does not explicitly promise compatibility or executable attestation across revisions.
- The prescribed build-and-run procedure is quoted in `docs/feature-roadmap-2026-10-04.md:91`: `cargo run --locked -p tachi-cli --bin catalog-drift -- --regenerate-baselines --typst /absolute/path/to/typst`. It then requires the same Typst executable on PATH and `cargo test -p tachi-core --test backward_compatibility`. There is no separate documented `cargo build` step in that procedure: `cargo run` performs the build/run through the selected checkout. With the default root and ordinary same-checkout invocation, the running builder is built from that checkout; this constrains the stale-direct-binary trigger. `--locked` constrains lockfile changes, not executable authentication. The offline CI procedure likewise uses `cargo run --locked ... --check` followed by the catalog test (`.github/workflows/catalog-drift.yml:36-37`). These procedures were inspected, not executed here.
- Strongest promise: `docs/feature-roadmap-2026-10-04.md:95` says, “Manifest version 2 binds PDFs to ordered content hashes of core source/build inputs, templates, brand assets and registered example inputs.” The same paragraph defines the operational checks: missing/changed/added/removed rendering inputs fail offline and regeneration rejects inputs changing during rendering. The roadmap at line 35 also promises hashes and renderer provenance. The inspected wording does not explicitly guarantee authenticated Rust executable identity, safe arbitrary cross-revision generation, or rejection of old binaries. “Binds” can overstate what this mechanism establishes if read as causal build attestation; documenting the same-source precondition is proportionate.

## Input hashes versus executing implementation

`crates/tachi-core/src/catalog_drift.rs:51-54,95-107` defines a conservative on-disk inventory including Cargo manifests/lockfile, toolchain, CLI source and the entire core source directory. Lines 88–90 hash file bytes under the supplied root. `regenerate` records those root hashes at line 242, stages templates/catalogs/examples from that root at lines 258–273, but calls `crate::try_build_report_data_typst` at line 278. That function is part of the already compiled executable; the source files being hashed are not compiled or loaded as Rust code by this call.

Lines 319–320 compare the root inputs and catalogs before/after rendering. Lines 322–329 put those input hashes, output hashes, Typst version, epoch and font policy into the manifest. `check` compares renderer constants and current root input hashes at lines 160–181 and registered baseline bytes at lines 183–202. The manifest shape at lines 41–49 contains no Rust executable identity. The complete inspected CLI and regeneration/check paths contain no build/root identity comparison.

Consequently, if binary A is run against a stable compatible root B with changed report-builder logic, and all rendering prerequisites succeed, it can record B source hashes alongside PDFs produced using A builder logic. A subsequent check with compatible catalog/manifest semantics can pass because it checks stored bytes against disk bytes, not the causal relation between Rust source and the executable. This is a conditional composition inference, not a reproduced wrong-PDF result. Existing Typst pinning, staged publication, source stability checks and rollback remain meaningful protections; they do not supply executable attestation.

## Disposition and remaining limits

Recommend a small documentation clarification: regeneration assumes the executable is built from the same source revision as the chosen root; use the documented Cargo procedure in that checkout. If cross-revision/root-independent regeneration is intentionally guaranteed, first state that supported requirement and then add an appropriate mismatch guard and targeted regression. This review does not mandate an authenticated-build subsystem or a new release gate.

Severity stays P3 because the stronger attestation obligation is unspecified in the inspected authoritative surfaces, not because observed production harm is absent. A stale-binary probe could establish the conditional mismatch but cannot establish the missing contractual obligation; it was deliberately not run under Standard/static-only scope. No current committed PDF corruption, failed documented same-checkout run, executable tampering, or release-readiness conclusion is claimed.

Method: numbered source/document reads and narrow contract searches only; no product edits, Python, runtime probes, network research or broad test campaign. Preferred lean-ctx path was unavailable; after reading RTK instructions, shell operations used `rtk proxy`. Graph discovery was attempted first; the bounded catalog_drift file-pattern query returned zero results, so pinned source reads were used under the insufficient-results fallback. The initial broad graph response was unrelated and not used as evidence. A quick memory keyword search found no applicable provenance contract and contributed no substantive facts. Review artifacts from earlier phases are inherited evidence; the narrow source trail above was independently reread. Absence of a stronger guarantee is bounded to the cited CLI, implementation, roadmap and CI contract, not an exhaustive audit of all repository or external promises.


---

# Phase 13: Verification Summary

<!-- Source artifact: state/phase_13_summary.md -->

# Phase 13 — Targeted verification summary

All four independent points completed in resource-bounded batches (three slots). Full trails remain in phase_13_point_*.md. No product edits.

| Point | Tier | Persona | Verdict | Key evidence / disposition |
|---|---|---|---|---|
| 1 Controls priority | Deep | Correctness / risk | VR_PARTIAL | Both silent-loss triggers confirmed; P1 urgency is not compelled by an explicit policy. Minimum P2 recommended without requiring observed harm. Preserve DA dissent. |
| 2 Empty image | Deep | Reporting / reliability | VR_CONFIRMED | Pinned Typst 0.15.1: bad fixture generation exit 0, compilation exit 1 (decode unexpected EOF); remove only copied empty PNG: generation/compile exit 0, no diagnostics, 12-page 95,852-byte PDF. P2. Attack chains share image selection but do not have the same Mermaid fallback and were not separately compiled. |
| 3 Historical input | Standard | Architecture / schema | VR_PARTIAL | Source-edit preservation of historical references exists; versionless runtime input migration/preservation is not promised. Current schema resolves current catalog. P3 policy advisory. |
| 4 Executable provenance | Standard | Build / release | VR_PARTIAL | Source hashing does not authenticate compiled builder; documented same-checkout Cargo command limits normal stale-binary trigger. No current artifact mismatch shown. P3 advisory. |

Counts: 1 confirmed, 3 partial, 0 refuted, 0 inconclusive, 0 new findings. No independent verification added a new defect. Nine supported defect groups and three advisories remain for judge arbitration. Citation verification and severity verification remain distinct from these four targeted verdicts.

The required state gate has passed all 33 mandatory artifacts (six reviewers × five outputs, plus audit/claim/severity), each at least 500 bytes with required schema markers. Two debate rounds were completed. Remote main rechecked before adjudication still points to `8df554e884b1e5dd24146111a965597eff5f4779`; tracked product diff is empty. Opus unavailable/model and capacity adaptations remain explicitly disclosed.


---

# Phase 13.5: Artifact Gate

<!-- Source artifact: state/phase_13_5_gate.json -->

[
  {
    "name": "reviewer_correctness_phase_3.md",
    "bytes": 11964,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_correctness_phase_4.md",
    "bytes": 4231,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_correctness_phase_5_round1.md",
    "bytes": 8231,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_correctness_phase_5_round2.md",
    "bytes": 1483,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_correctness_phase_7.md",
    "bytes": 3444,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_architecture_phase_3.md",
    "bytes": 10229,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_architecture_phase_4.md",
    "bytes": 3528,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_architecture_phase_5_round1.md",
    "bytes": 7398,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_architecture_phase_5_round2.md",
    "bytes": 2503,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_architecture_phase_7.md",
    "bytes": 3498,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_security_phase_3.md",
    "bytes": 9788,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_security_phase_4.md",
    "bytes": 3949,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_security_phase_5_round1.md",
    "bytes": 7283,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_security_phase_5_round2.md",
    "bytes": 3060,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_security_phase_7.md",
    "bytes": 4039,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_devils_advocate_phase_3.md",
    "bytes": 7284,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_devils_advocate_phase_4.md",
    "bytes": 3424,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_devils_advocate_phase_5_round1.md",
    "bytes": 6978,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_devils_advocate_phase_5_round2.md",
    "bytes": 3137,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_devils_advocate_phase_7.md",
    "bytes": 3722,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_code_quality_phase_3.md",
    "bytes": 8423,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_code_quality_phase_4.md",
    "bytes": 3428,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_code_quality_phase_5_round1.md",
    "bytes": 6962,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_code_quality_phase_5_round2.md",
    "bytes": 2750,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_code_quality_phase_7.md",
    "bytes": 3668,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_pipeline_phase_3.md",
    "bytes": 7482,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_pipeline_phase_4.md",
    "bytes": 3329,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_pipeline_phase_5_round1.md",
    "bytes": 6597,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_pipeline_phase_5_round2.md",
    "bytes": 2506,
    "pass": true,
    "missing": []
  },
  {
    "name": "reviewer_pipeline_phase_7.md",
    "bytes": 3331,
    "pass": true,
    "missing": []
  },
  {
    "name": "phase_8_audit.md",
    "bytes": 6594,
    "pass": true,
    "missing": []
  },
  {
    "name": "phase_10_claim_verification.md",
    "bytes": 10921,
    "pass": true,
    "missing": []
  },
  {
    "name": "phase_11_severity_verification.md",
    "bytes": 8677,
    "pass": true,
    "missing": []
  }
]


---

# Phase 14: Supreme Judge

<!-- Source artifact: state/phase_14_judge_ruling.md -->

> **Persona Profile — phase_14_judge_ruling.md**
> Supreme Judge; domain-neutral arbitration, evidence over rhetoric; Phase 14. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

# Phase 14 — Supreme Judge ruling

**Verdict: REQUEST CHANGES. Score: 6.5/10. Confidence: High in the bounded defect mechanisms; Medium in priority and overall coverage.**

Reviewed pinned main `8df554e884b1e5dd24146111a965597eff5f4779` against base `dd3b293d81d358d1ae27424be83b720693539112`, covering the seven merged commits in the context's 2026-10-02 21:37:02 UTC–2026-10-04 21:37:02 UTC window. This is a focused change review, not a release certification. The original dirty checkout is outside scope. No product edits or new runtime probes were performed by the judge.

## Machine-usable disposition

| Metric | Final count / value |
|---|---:|
| P0 defects | 0 |
| P1 defects | 0 |
| P2 defects | 9 |
| P3 mandatory defects | 0 |
| P3 advisories | 3 |
| Total stable action IDs | 12 |
| Consensus points below | 7 |
| Disagreement/arbitration rulings below | 5 |
| Accepted new completeness-audit findings | 2 |
| Targeted verification VR_CONFIRMED | 1 |
| Targeted verification VR_PARTIAL | 3 |
| Targeted verification VR_REFUTED | 0 |
| Targeted verification VR_INCONCLUSIVE | 0 |
| Judge-introduced findings | 0 |
| Judge-introduced P0/P1 | 0 |
| Mandatory pre-judge artifacts passing Phase 13.5 | 33/33 |

Stable priority order: **A1 → A2 → A3 → A4 → A5 → A6 → A7 → A8 → A9**, followed by advisories **A10 → A11 → A12**. This orders work within P2; it does not imply different formal severities. A1 prevents concrete data loss; A2/A3/A4 address successful suppression of security evidence. A5/A6 share image resolution code but remain separate bugs. Small independent fixes A7/A8 may proceed alongside them.

| ID | Original IDs / source | Title | Priority | Classification | Epistemic disposition | Confidence | Targeted tier/result |
|---|---|---|---|---|---|---|---|
| A1 | SEC-1 / Security | Cleanup deletes backing bytes of retained symlink | P2 | EXISTING_DEFECT | VERIFIED runtime and source | High | Not dispatched |
| A2 | COR-2, DA-1 / Correctness, DA | Incomplete controls become a zero-finding assessment | P2 | EXISTING_DEFECT | VERIFIED mechanism; priority judgment | High mechanism, Medium priority | Deep / VR_PARTIAL on mandatory P1 |
| A3 | COR-1, ARC-1 / Correctness, Architecture | Malformed risk table replaces valid findings | P2 | EXISTING_DEFECT | VERIFIED runtime and source | High | Not dispatched |
| A4 | AUD-1 / Completeness Auditor | Duplicate MAESTRO rows erase positive evidence | P2 | EXISTING_DEFECT | VERIFIED runtime and source | High | Not dispatched |
| A5 | COR-3, ARC-2 / Correctness, Architecture | Empty optional attack image blocks PDF | P2 | EXISTING_DEFECT | VERIFIED attack-tree PDF failure | High for exercised case | Deep / VR_CONFIRMED |
| A6 | SEC-2 / Security | Path-valued tree ID selects neighboring image | P2 | EXISTING_DEFECT | VERIFIED selection; disclosure conditional | High selection, Medium downstream impact | Not dispatched |
| A7 | CQ-2 / Code Quality; Security independently reproduced | Nested attribution rejects documented default | P2 | EXISTING_DEFECT | VERIFIED runtime and explicit contract | High | Not dispatched |
| A8 | PIPE-1 / Pipeline | Unix-only helper breaks Windows test compilation | P2 | EXISTING_DEFECT | VERIFIED source-target dependency; execution unverified | High source conclusion | Not dispatched |
| A9 | AUD-2 / Completeness Auditor | Failed redirect traversal classified healthy | P2 | EXISTING_DEFECT | VERIFIED real-curl local probe | High | Not dispatched |
| A10 | CQ-1 / Code Quality | Clarify historical versionless taxonomy inputs | P3 advisory | PLAN_RISK / contract clarification | VERIFIED edition collision; stronger obligation unestablished | Medium | Standard / VR_PARTIAL |
| A11 | PIPE-2 / Pipeline | Clarify builder/source revision precondition | P3 advisory | PLAN_RISK / contract clarification | VERIFIED composition; mismatch consequence conditional | Medium | Standard / VR_PARTIAL |
| A12 | DA-2 / DA | Clarify companion PDF integrity coverage | P3 advisory | PLAN_RISK / contract clarification | VERIFIED checker scope; broader promise unestablished | High scope, Medium obligation | Not dispatched |

`VERIFIED` describes the bounded claim, not every proposed impact or severity. No item relies on consensus as its verification method. Every high-priority post-judge classification inventory is empty: there are no P0/P1 findings to label PANEL-RAISED or JUDGE-INTRODUCED. Phase 14.5 must still write its required empty-case artifact.

## Score assessment

| Reviewer | Initial Phase 3 | Blind final Phase 7 | Interpretation |
|---|---:|---:|---|
| Correctness Hawk | 6.5 | 6.5 | Three concrete report regressions retained |
| Architecture Critic | 7.0 | 7.0 | Architecture retained; risk priority narrowed |
| Security Auditor | 7.5 | 7.0 | Additional reproduced default/tier failures |
| Devil's Advocate | 6.0 | 6.5 | Companion/provenance concerns narrowed to advisories |
| Code Quality Auditor | 7.0 | 7.0 | Default defect retained; historical contract narrowed |
| Pipeline Reviewer | 7.5 | 7.0 | Additional report-integrity evidence; provenance advisory |
| Arithmetic mean, descriptive only | 6.92 | 6.83 | Not the judge's decision rule |

The final 6.5 reflects material but localized defects at report-authority, asset, schema and monitor boundaries, including two independent audit additions and an executed PDF failure after blind finals. Checked attribution errors, shared MAESTRO states, coherent selected-tier summaries and staged publication are valuable improvements. Neither a broad rewrite nor a rejection of the architecture is justified. The score is slightly below the panel mean because the verified audit additions broaden the remaining correction work. P2 does not imply acceptance: repeated wrong successful outputs require targeted fixes before reliance on the affected path.

## Consensus points validated independently

1. **Validity must precede authoritative selection.** Nonempty parsed vectors and control inventory do not establish a complete assessment. Source replacement and runtime outputs support this, independently of six reviewers agreeing.
2. **A completed empty assessment is legitimate.** Existing contract tests explicitly preserve it; rejecting all empty controls output would break intended behavior.
3. **Three asset defects remain distinct.** Retained-file survival, candidate usability, and candidate containment require different guarantees and regressions.
4. **Nested attribution has an explicit default contract.** The YAML schema promises `primary`, while direct nested serde deserialization requires the field. This is stronger evidence than an inferred compatibility promise.
5. **Existing safeguards are meaningful but bounded.** Checked primary attribution, MAESTRO detailed-row repair, cleanup byte equality, catalog staging/source stability/rollback and monitor retries exist; none resolves the specific bypass cases below.
6. **Catalog checks and baseline equality are not semantic correctness proofs.** They verify their declared bytes and inputs; they do not prove assessment completeness, historical semantics or arbitrary executable provenance.
7. **The appropriate result is targeted corrections with explicit limits.** No broad compromise, current committed PDF corruption or complete release readiness is established.

## Disagreements and rulings

### R1 — Controls false-zero priority: P2, preserve P1 dissent

DA's P1 / Medium priority-confidence position has a legitimate consequence argument: successful security output omits a known High finding, and the incomplete inventory trigger is small. Actual user harm is not a prerequisite for urgency, and missing prevalence data is not proof of rarity. Source-input survival does not repair the misleading output.

The minimum justified priority on this record remains P2. The established triggers are an incomplete local inventory and a noncanonical residual table; a completed canonical producer path or automatic security decision hitting them is not established. This positively bounds the demonstrated failure. The roadmap's P1 MAESTRO truthfulness goal supports concern but does not impose a universal bug-priority policy; its P2 output-integrity goal likewise does not decide the matter mechanically. No inspected priority policy resolves the tradeoff. The ruling therefore weighs the demonstrated boundary and urgency, not five-to-one voting, imagined rarity or a universal remote-exploit requirement. Keep both controls regression cases, the impact rationale and the dissent in downstream reports.

### R2 — Malformed risk original P1: retain P2

ARC-1's initial P1 was explicitly withdrawn after trigger review. A malformed optional table becomes an anonymous row and suppresses known severity counts. That is a material new composition defect; the permissive parser itself predates the window. No source artifact deletion or ordinary completed-input exposure is shown. P2 is proportionate without calling the issue cosmetic.

### R3 — Empty-image downstream consequence: now executed, still P2

Phase 10 correctly limited its claim to binding/fallback evidence. Phase 13 subsequently compiled the attack-tree fixture with pinned Typst 0.15.1: generation exit 0, compile exit 1, `failed to decode image (unexpected end of file)`. Removing only the copied zero-byte PNG restored generation and compilation exit 0 and a 12-page, 95,852-byte PDF with empty diagnostics. This supersedes earlier unexecuted-PDF caveats for this exact attack-tree case. It does not verify neighboring-image disclosure, every corrupt image, later-format fallback or attack-chain compilation. Chains share selection but have no Mermaid text fallback. The narrow malformed-artifact trigger remains P2.

### R4 — Historical taxonomy compatibility: advisory

Current/base catalog meaning collision and current SARIF edition labeling are supported. Explicit historical-reference preservation in the cutover documentation concerns source editing; the versionless attribution schema resolves IDs in the current catalog without an edition field. Those facts do not establish promised preservation or migration of arbitrary historical versionless runtime input. Keep A10 as a policy clarification, not a mandatory automatic remapping change. This narrowing rests on contract scope, not absence of reported harm.

### R5 — Provenance and companion checking: two separate advisories

Source hashing does not authenticate the compiled Rust builder; baseline checking does not cover every published companion. Both mechanisms are real. However, the prescribed same-checkout Cargo invocation constrains stale-binary use, and the stated checker contract targets registered baselines. No current committed artifact corruption or broken same-checkout procedure is demonstrated. Preserve A11 and A12 separately because executable identity and output coverage are different concerns. Do not invent an attestation subsystem or release gate as a required correction.

Debate quality was adequate: the four narrowed positions cite input or contract evidence, and DA retained a qualified minority position. There is no evidence here of unsupported score following. Final-score spread is only 0.5, so **correlated-bias warning applies**. Shared models and the shared trace further limit diversity. Two debate rounds were completed; stopping before a third repetition did not skip the required debate work.

## Defect action items

Effort estimates are qualitative implementation-plus-focused-regression estimates, not delivery commitments: Small is a localized guard/helper or test change; Medium involves a parser/consumer boundary or multiple legitimate states. All source paths below are relative to the pinned worktree. Citations refer to that immutable reviewed revision.

### A1 — P2: preserve backing bytes during mislabeled-image cleanup

- **Source/snippet:** `crates/tachi-core/src/assets.rs:41-51` selects a mislabeled sibling then calls `fs::remove_file(&path)`; `:61-64` compares via `(fs::read(left), fs::read(right))`.
- **Trigger/consequence:** A correctly named PNG symlink points at the mislabeled regular JPG containing PNG bytes. Reads compare equal; cleanup unlinks the backing JPG, leaving both logical paths unreadable. Runtime source harness and retained dangling-link fixture support actual loss of sole backing bytes.
- **Safeguard/intro:** Fixed stems, byte equality and opt-in cleanup constrain the action but do not prove retained-file independence. This cleanup is new in the interval. No arbitrary-root deletion is claimed.
- **Evidence/source reviewers:** SEC-1, Security Phase 3/7; Phase 10 item 4. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Establish that the retained sibling survives deletion, or conservatively skip symlink-dependent cleanup; use no-follow metadata where appropriate. Test retained symlink to candidate, ordinary independent duplicate and missing/different sibling. Preserve the ordinary safe cleanup behavior. Medium.

### A2 — P2: require completion before controls replace findings

- **Source/snippet:** `crates/tachi-core/src/report_data.rs:289-291`: `!data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty()` returns true. Independently `:306-340` recognizes a residual header and returns true without proving row consumption; `:174-177` replaces findings. `crates/tachi-core/src/compensating_controls.rs:125-130` only parses exact severity subsections.
- **Trigger/consequence:** (1) A Missing/None control descriptor without residual assessment; (2) a populated residual table outside required severity grouping. Both can replace existing High S-1 with Tier 1, zero findings and zero High count while generation succeeds. The second input is noncanonical. This is output loss, not source-file deletion.
- **Safeguard/intro:** Numeric and completed-empty checks exist; inventory bypasses them, and header recognition does not detect unconsumed rows. New authoritative promotion exposes permissive parsing. Existing tests at `crates/tachi-core/tests/report_document_contract.rs:249-267,311-346` require both incomplete-input protection and legitimate completed-empty selection.
- **Evidence/source reviewers:** COR-2/DA-1, Correctness and DA; Phase 10 item 2 replay/inspection; Phase 11 and Phase 13 point 1. VERIFIED / EXISTING_DEFECT, High mechanism and Medium priority confidence. Preserve DA P1 dissent.
- **Fix/regression/effort:** Model assessment completion separately from inventory; reject or diagnose populated unparsed assessment rows. For each independent trigger, retain the known finding or return a checked error. Retain completed-empty output, including its control metadata. At least two negative regressions plus the existing positive empty contract. Medium.

### A3 — P2: validate optional risk rows before selecting Tier 2

- **Source/snippet:** `crates/tachi-core/src/report_data.rs:164-168`: `if !rows.is_empty()` selects Tier 2. `crates/tachi-core/src/parsers/findings.rs:158-178` defaults missing fields via `unwrap_or_default()`; `report_data.rs:268-276` recomputes counts from the replacement.
- **Trigger/consequence:** A nonempty unrelated/malformed scored table alongside valid High S-1 produces one anonymous all-empty finding and High=0, with success. Retained CLI minimal-pair evidence proves the projection change.
- **Safeguard/intro:** Primary attribution checking validates another input; no scored header/identity/severity validation intervenes. The parser preexists; new report-data tier selection introduces the regression.
- **Evidence/source reviewers:** COR-1/ARC-1, Correctness and Architecture; Phase 10 item 1. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Validate the scored column/row contract before promotion and reject invalid input through the checked API or retain the lower tier with diagnostic. Test unrelated headers and canonical headers with missing identity/severity cells; assert known S-1 never becomes an anonymous row. Medium.

### A4 — P2: prevent duplicate MAESTRO rows from erasing positive evidence

- **Source/snippet:** `crates/tachi-core/src/infographic.rs:273-289`: `by_layer.insert(layer_id.clone(), MaestroLayerDistribution { ... })`; `:292-306` emits the reduced map. Repair at `:443-474` requires observed detailed findings.
- **Trigger/consequence:** Summary-only canonical rows `L1 | 2 | High` then `L1 | 0 | Clean` yield count 0, clean state and empty most-exposed layer. Runtime output proves silent loss of positive evidence.
- **Safeguard/intro:** Positive-count precedence applies per row before overwrite. Detailed-row reconciliation cannot repair summary-only input. Base appended both rows to a Vec; new map reduction introduces last-row-wins loss.
- **Evidence/source reviewers:** AUD-1, independent Completeness Auditor; Phase 10 item 11 and Phase 11. VERIFIED / EXISTING_DEFECT, High confidence. Audit source and guard were independently re-read by the judge.
- **Fix/regression/effort:** Reject conflicting duplicate IDs or merge conservatively so positive evidence dominates clean/not-applicable states. Cover both row orders, normalized aliases and summary-only input. Avoid blindly summing duplicate counts. Medium.

### A5 — P2: skip unusable optional attack images and preserve fallback

- **Source/snippet:** `crates/tachi-core/src/report_data.rs:227-247`, especially `:234`: `.find(|path| path.is_file())`. `templates/tachi/security-report/attack-path.typ:69-81` calls `image(img-path, ...)` at :74 before raw Mermaid text fallback.
- **Trigger/consequence:** A zero-byte preferred PNG for a valid attack tree sets `has-image=true`; report-data succeeds but actual Typst compilation fails with unexpected EOF. Removing the copied empty image enables the existing text fallback and successful PDF compilation.
- **Safeguard/intro:** Filesystem existence is the only candidate acceptance test; the general asset helper's usability checks are not invoked. New image binding supplies the invalid candidate. Later formats are skipped by first-match selection; that extension is source-traced, not separately executed.
- **Evidence/source reviewers:** COR-3/ARC-2, Correctness and Architecture; Phase 10 item 3; Phase 13 point 2 integration minimal pair. VERIFIED / EXISTING_DEFECT, High confidence for the exercised tree path.
- **Fix/regression/effort:** Require a usable candidate, continue to valid alternatives and preserve no-image fallback. Test zero-byte preferred PNG plus available source text, and invalid first format plus valid later format. Evaluate nonempty invalid data deliberately rather than equating size with decodability. Chain selection should receive the same resolver fix, with its distinct template behavior tested. Medium.

### A6 — P2: contain image resolution for path-valued attack-tree IDs

- **Source/snippet:** `crates/tachi-core/src/attack_trees.rs:111,124-130` accepts metadata/heading IDs; `crates/tachi-core/src/report_data.rs:232-246` joins `format!("{id}-{suffix}.{ext}")` without containment. Image sink: `templates/tachi/security-report/attack-path.typ:74`.
- **Trigger/consequence:** An ID containing `../../` selects a neighboring SVG outside the intended report asset directory. The generated binding was executed. Actual disclosure additionally requires compiler-root access, successful compilation and sharing; those steps were not executed. No RCE or arbitrary text read is established.
- **Safeguard/intro:** A separate heading grammar check does not cover metadata or the cited Attack Tree heading branch. Relative-path conversion is not containment. New image-resolution composition introduces the unsafe selection.
- **Evidence/source reviewers:** SEC-2, Security; Phase 10 item 5. VERIFIED selection / EXISTING_DEFECT; High mechanism confidence, Medium downstream impact confidence.
- **Fix/regression/effort:** Validate IDs against the intended identifier grammar and enforce resolved-path containment, accounting for symlinks. Cover metadata and heading ingress, traversal/absolute values, symlink escape and a valid ID. Return a checked error or omit unsafe images with a clear diagnostic. Medium.

### A7 — P2: honor the primary relationship default in nested attribution

- **Source/snippet:** `crates/tachi-core/src/parsers/findings.rs:63-67`: `pub relationship: String` has no serde default; nested `serde_yaml::from_str(yaml)` at `:493-498` requires it. Flat parsing defaults at :600. `schemas/finding.yaml:277-289` specifies `default: primary` and injection when absent.
- **Trigger/consequence:** A valid nested record omitting relationship fails with missing-field error and command exit 1, while the equivalent flat record succeeds with primary. Two reviewers independently reproduced the minimal pair.
- **Safeguard/intro:** Validation/normalization happens after nested deserialization and cannot apply the default to a record never created. The nested path is new; the flat default predates it.
- **Evidence/source reviewers:** CQ-2, Code Quality; Security independent probe; Phase 10 item 7. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Add an explicit serde default returning primary or normalize through an equivalent boundary. Test absent relationship parity between nested and flat forms, explicit valid values and invalid values; preserve absent-versus-empty attribution semantics. Small.

### A8 — P2: isolate Unix-only test helpers from Windows targets

- **Source/snippet:** `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:200-203`: only `#[cfg(test)]` guards `use std::os::unix::fs::PermissionsExt`; :205-218 invokes `set_mode`.
- **Trigger/consequence:** Compiling this binary's test target for Windows includes unavailable Unix APIs. This is a deterministic source-target dependency finding, not an executed Windows failure or claim about every runtime feature's portability.
- **Safeguard/intro:** No `cfg(unix)` guard or applicable Cargo autobin exemption is identified. The binary is new.
- **Evidence/source reviewers:** PIPE-1, Pipeline; Phase 10 item 8. VERIFIED source / EXISTING_DEFECT, High confidence; Windows execution UNVERIFIED.
- **Fix/regression/effort:** Gate Unix-specific imports/helpers/tests appropriately or supply a portable fake executable. Keep platform-independent classifier tests enabled everywhere. Validate test-target compilation on a Windows target/runner after correction. Small.

### A9 — P2: treat exhausted redirects as failed health checks

- **Source/snippet:** `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:157-163`: `200..=399 => "healthy"` precedes failed-process handling. Error capture :148-154 retains curl failure; :113-119 retries HEAD with GET.
- **Trigger/consequence:** A loopback endpoint endlessly redirects. Real curl exits 47 after maximum redirects for both HEAD and GET but outputs HTTP302. The monitor reports one healthy URL while JSON also retains the curl error.
- **Safeguard/intro:** Retry, timeout and error capture exist. The final classifier overrides failed traversal with the residual status. The new monitor introduces the behavior. Its informational success exit is intentional and is not a CI-gate bypass.
- **Evidence/source reviewers:** AUD-2, independent Completeness Auditor; Phase 10 item 12 and Phase 11. VERIFIED / EXISTING_DEFECT, High confidence. Judge re-read actual classifier and failure capture.
- **Fix/regression/effort:** Prioritize relevant process/transport failure over residual successful/redirect codes, at least redirect exhaustion; report needs-review/transient rather than healthy. Add failed-302 classifier coverage and a real local redirect-loop regression while retaining informational command semantics. Small.

## Separate advisories

### A10 — P3 advisory: define historical versionless taxonomy policy

- **Source/snippet:** `schemas/taxonomy/owasp.yaml:471-473` now associates LLM05 with Data and Model Poisoning; base associated it with Improper Output Handling. `crates/tachi-core/src/threats_sarif.rs:170-179` formats primary LLM references with `:2026`. `schemas/finding.yaml:254-289` provides no edition field and requires current-catalog resolution.
- **Trigger/consequence:** Regenerating versionless historical attribution can reinterpret an ID under the current catalog. A mandatory historical-regeneration compatibility contract is unestablished; external OWASP publication accuracy is not adjudicated.
- **Source/confidence/class:** CQ-1, Code Quality; Phase 13 point 3, VR_PARTIAL; Medium obligation confidence; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** Document current-catalog resolution and the caller's legacy review obligation, or explicitly support edition-aware preservation and add historical fixtures. Do not silently guess ID remaps. Small for documentation; Medium or greater for chosen edition support.

### A11 — P3 advisory: state the same-source executable precondition

- **Source/snippet:** `crates/tachi-core/src/catalog_drift.rs:242` captures supplied-root hashes, :278 calls compiled `crate::try_build_report_data_typst`, and :319-329 rechecks/publishes root hashes. CLI root override: `crates/tachi-cli/src/bin/catalog-drift.rs:10-12,21-24`. Documented same-checkout Cargo command: `docs/feature-roadmap-2026-10-04.md:91`.
- **Trigger/consequence:** A compatible binary A used against source root B can theoretically attest B's disk inputs while building report data with A's compiled logic. No stale-binary mismatch was executed or current PDF corruption shown. Same-checkout Cargo invocation limits the ordinary trigger; input stability and rollback do not authenticate executable identity.
- **Source/confidence/class:** PIPE-2, Pipeline; Phase 13 point 4, VR_PARTIAL; Medium confidence in broader consequence/obligation; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** Document that the builder must come from the selected source revision and use the prescribed invocation. Only if arbitrary cross-revision generation is supported, define a mismatch check and targeted regression. Small clarification; larger attestation work needs an explicit requirement.

### A12 — P3 advisory: distinguish baseline checks from companion checks

- **Source/snippet:** `crates/tachi-core/src/catalog_drift.rs:78-86` excludes companion PDFs from rendering inputs, :183-202 checks registered baseline hashes, and :313-317 publishes companions when present.
- **Trigger/consequence:** Altering a published companion in a private fixture does not fail `--check` when registered baselines remain valid. The fixture behavior was reproduced; current committed PDFs are not claimed corrupt. A promise to verify every published copy is unestablished.
- **Source/confidence/class:** DA-2, DA; Phase 10 item 10; High scope confidence, Medium obligation confidence; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** State checker scope clearly. If companion integrity becomes a supported guarantee, register/hash those outputs and test companion-only corruption. Small clarification; Medium coverage extension.

## Audit, safeguard and coverage assessment

The audit's two findings survive source reinspection: duplicate map insertion precedes any consumer and can erase positive summary-only evidence, while monitor classification accepts 302 before considering curl failure. Neither is a restatement of panel tier selection, and both include base/introduction evidence. Keep both as full defects, not lower-confidence appendices merely because fewer reviewers raised them.

The judge re-read the report selection/completion and image resolver, cleanup equality/unlink, nested attribution type/deserializer/schema, attack-tree ID branches, MAESTRO reduction/reconciliation, monitor classifier and test guard, plus the attack-path consumer. This confirms that claimed missing guards are missing at the relevant boundaries. It does not erase real safeguards elsewhere. The focused gap scan examined the adjacent selection/reconciliation/retry/consumer paths and produced no additional defensible finding. It is not an exhaustive new independent review.

Coverage remains uneven. The panel and supplemental audit touched CI permissions, workflow triggers, frontend authentication and Prisma/RLS contracts, but did not execute browser/session flows, live database isolation, Windows compilation, crash durability or concurrent catalog publication. Generated/binary assets and large registry/lockfile surfaces were excluded from the shared text bundle and only selectively inspected. No production agent generation, real-user prevalence, operational decision impact or external taxonomy publication correctness was verified. Root/shell containment elsewhere does not automatically guard the direct report path; conversely an unused Prisma client does not establish a new tenant-exposure endpoint. Preexisting middleware cookie behavior is excluded from this interval. These remain limitations, not manufactured defects.

## Evidence and process limits

Inputs consumed: context/personas; both Phase 6 summaries; all six Phase 7 finals; Phase 8/9/10/11; Phase 12a/12b; all four Phase 13 point artifacts and summary; Phase 13.5 gate JSON; selected Phase 3 score/source material and pinned implementation. Earlier claims are superseded only where later evidence actually resolves them. In particular, Phase 13 resolves the empty-image attack-tree compile consequence; it does not resolve path disclosure or Windows execution.

Phase 9 performed one allowed read-only source check, CMD_CONFIRMED, and did not replay out-of-allowlist runtime commands. Runtime claims here are attributed to their documented reviewer/auditor/verifier probes, not to fresh judge execution. The available graph was documented stale for this pinned worktree; exact known source paths and current numbered citations govern. No external-domain assertion drives severity, so no web-domain validation is needed. A wrapper invocation failed because lean-ctx was absent; RTK was used after the local RTK instructions were read. No memory-derived fact supplies a current behavior or priority claim in this ruling.

This is a **full single-run protocol through Phase 14**, with all 33 mandatory pre-judge artifacts passing; **no compressed-run banner is warranted**. Phase 14.5 and sequential Phase 15 outputs remain for the orchestrator. Opus was unavailable, so reviewers used the inherited available model consistently; only three child slots permitted two isolated batches for six independent reviewers. These disclosed runtime adaptations do not constitute skipped phases, but shared-model/trace bias remains. No product fix, tracker mutation, release authorization or deployment is implied by this review.


---

# Phase 14.5: Judge Verification

<!-- Source artifact: state/phase_14_5_judge_verification.md -->

> **Persona Profile — phase_14_5_judge_verification.md**
> Judge-Output Verifier; independent ground-truth gate for new high-priority findings; Phase 14.5. Generic persona-prompted agent using the inherited available model; no Opus or Agent-plugin specialist was available.

No judge-introduced findings to verify