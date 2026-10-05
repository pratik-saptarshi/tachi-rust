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
