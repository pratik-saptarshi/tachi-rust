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
