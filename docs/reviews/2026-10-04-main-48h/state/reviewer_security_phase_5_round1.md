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
