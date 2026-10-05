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
