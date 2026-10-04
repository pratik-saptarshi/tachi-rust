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
