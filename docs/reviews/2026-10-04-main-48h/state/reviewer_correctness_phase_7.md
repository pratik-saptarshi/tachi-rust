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
