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
