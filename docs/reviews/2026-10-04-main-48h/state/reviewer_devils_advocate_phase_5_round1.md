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
