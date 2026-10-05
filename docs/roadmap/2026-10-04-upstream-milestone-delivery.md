# Rust-native upstream roadmap: milestone delivery

Date: 2026-10-04. Tracking: completed epic `RT-3zm`. Original implementation: epic `RT-5vk` and [PR #42](https://github.com/pratik-saptarshi/tachi-rust/pull/42). Comparison anchors remain upstream `63438d78` and fork `cb567d32`.

Current delivery is complete: semantic PR #43, reporting PR #44, and final evidence PR #42 are protected auto-merges. All three phase issues and epic `RT-3zm` are closed. PR #42 merged at `8df554e884b1e5dd24146111a965597eff5f4779` after final required checks, review resolution, and evidence reconciliation. The committed tracker export was synchronized after merge and records the completed issue state.

The six-feature implementation was initially submitted together, then delivered through independently validated milestones. Three actionable review findings on PR #42 were corrected before merge; every review thread was resolved. Live Beads closure records final delivery evidence, and the checked-in export was synchronized afterward. References below to an earlier pre-merge snapshot describe historical planning evidence only.

## Invariants

- Preserve all six original feature contracts and the Rust architecture. Introduce no direct or indirect Python execution requirements in runtime, build, tests, documentation, generation or newly adopted workflows.
- Preserve the original dirty checkout and the separate PR #26 closeout scope. Work in isolated worktrees; never force-push or weaken branch protection to deliver a phase.
- At each phase start, inspect every open PR's live head, CI and review state. Fix in-scope failures before enabling that phase's auto-merge. An existing green check does not substitute for unresolved review or quality gates.
- Each phase has its own PR and final-head validation. Enable protected squash auto-merge only after its acceptance and review findings are addressed. GitHub must enforce the required checks and quality rules; no administrator bypass.

## Milestones and validation

| Phase | Deliverable and acceptance | Validation | Tracking / PR |
|---|---|---|---|
| 1 | Contextual OWASP 2026 semantic cutover. Active instructions, fixtures and current baseline example citations agree; historical references remain historical; finding schema unchanged. | Rust semantic mutation tests, catalog resolution, historical exceptions, parser/taxonomy/SARIF regressions, formatting, documentation gates and all required hosted checks. | `RT-3zm.1`; [PR #43](https://github.com/pratik-saptarshi/tachi-rust/pull/43) |
| 2 | Native reporting and automation: shared truthful MAESTRO states, permissions check, catalog drift/regeneration, OI identity/citation preservation and adopter scanning. Correct PR #42 review findings: populate remediation actions, detect/stage brand assets, and derive component distribution from the active tier. | Focused review regressions; all seven MAESTRO states agree across outputs; malformed/valid permission cases; drift and failed regeneration rollback; mixed OI/LLM identities/CWE/citations/assets; native Gitleaks inheritance; six pinned PDF comparisons; no-interpreter CLI/regeneration; full workspace, Clippy, formatting, docs, actionlint and hosted checks. | `RT-3zm.2`; [PR #44](https://github.com/pratik-saptarshi/tachi-rust/pull/44) |
| 3 | Final requirement-by-requirement delivery audit, canonical roadmap/codemap/evidence and Beads export reconciliation. Record actual phase PR merge states and remaining repository gates. | Refresh all open PRs, verify final main/branch freshness, native catalog/permissions checks, exact baseline hashes and rendering provenance, dependency execution-chain audit, final documentation and hosted checks. | `RT-3zm.3`; depends on phase 2; [PR #42](https://github.com/pratik-saptarshi/tachi-rust/pull/42) |

## Original feature acceptance retained

1. OWASP: output handling LLM10, misinformation LLM07, and every changed category maps by meaning; do not globally replace historical IDs. Source: [#363](https://github.com/davidmatousek/tachi/pull/363).
2. Permissions: malformed JSON, undocumented rules and missing sections fail actionably; valid settings pass. Read-only, path-triggered hosted check; no branch protection changes. Source: [#347](https://github.com/davidmatousek/tachi/pull/347).
3. MAESTRO: `findings`, `clean`, `not_applicable`, `not_evaluated` are shared typed states across Markdown evidence, Typst data and infographic JSON. Missing evidence defaults to not evaluated; keep all seven canonical layers. Source: [#318](https://github.com/davidmatousek/tachi/pull/318).
4. Catalog: ordered raw and in-scope `(id, out_of_scope)` fingerprints use the renderer registry/loader; citation-only edits pass. Stage all six PDFs and publish them with hashes/provenance only after all native renders succeed. Offline checking needs no Typst, Python or network. Source: [#344](https://github.com/davidmatousek/tachi/pull/344).
5. OI preservation: mixed OI/LLM identities use `findingId/v1`, retaining CWE-943, current OWASP references, source paths and assets. Deterministic transformation tests do not establish live-agent evaluation success; [#356](https://github.com/davidmatousek/tachi/issues/356) remains failed verification. Source: [#353](https://github.com/davidmatousek/tachi/pull/353).
6. Scanning: adopter rules extend current default rules and pinned native Gitleaks; synthetic secrets outside excluded fixture paths fail, permitted placeholders pass, and malformed config fails. Source: [#347](https://github.com/davidmatousek/tachi/pull/347).

Already adopted crosswalk parity, safe image cleanup, taxonomy expansion, asset-tag output and citation monitoring are not reopened.

## Phase-start audit

- 2026-10-04, phase 1: PR #42 head `0fe113b8` has all 17 required checks successful and terminal jobs, but three report review findings remain. PR #26 head `ce7535c3` also has terminal successful CI, is mergeable, remains blocked by repository gates and already has auto-merge enabled. It remains separate work.
- The repository requires conversation resolution and an active Code Quality rule. A blocked merge must be investigated and remediated or explicitly reported; green CI alone is insufficient. No policy changes or bypasses are authorized by this plan.

## Progress history

- Phase 1 completed: PR #43 auto-merged on 2026-10-04 at 13:32:02 UTC after required checks and review resolution. Its final source head was `28ad38ca`; full workspace and Clippy passed locally, and an empty-PATH native export plus companion refresh preserved the canonical controls SARIF byte-for-byte. `RT-3zm.1` is closed.
- Phase 2 delivery: [PR #44](https://github.com/pratik-saptarshi/tachi-rust/pull/44) is synchronized with phase 1 on main. Its previous head passed all 17 required hosted checks plus permissions/catalog checks. Fresh final-head checks and review remain required before protected auto-merge. The following preparation notes are historical validation evidence.

- Phase 1 review prerequisite: native SARIF export now carries explicit Markdown attribution by finding identity, including the canonical nested YAML format and explicit empty records. The companion refresh command therefore retains current citations. This moves the necessary parsing/export portion of UF-05 forward; the full reporting acceptance remains in phase 2. Existing `serde_yaml_ng` is promoted from development to runtime dependency without adding a package or Python execution chain.

- Phase 1: [PR #43](https://github.com/pratik-saptarshi/tachi-rust/pull/43), extracted from freshly fetched main. Review found valid-but-wrong category IDs in examples. Corrected retrieval poisoning to LLM09, training poisoning to LLM05, model extraction/theft to LLM06 per the loaded pattern catalog, and configuration leakage to LLM08 across active agents/adapters. Added parsed-YAML example checks and stale mutations; focused Rust, formatting and documentation validation passed. Hosted checks remain the phase merge gate.
- Phase 2: independent preparation while phase 1 finishes its hosted gate. Three report review fixes are implemented with focused Rust regressions. Canonical sample review also reproduced missing nested citations and suppressed attack sections; the Rust path now preserves both. All original six PDFs plus the canonical agentic sample regenerate with pinned Typst and `PATH=/nonexistent`, including existing PDF companions; all seven byte-identical comparisons pass. Full workspace tests, Clippy, native permissions/catalog commands, Gitleaks adopter tests, actionlint and documentation gates pass. Publication to main follows phase 1 merge.
- Phase 3 preparation checkpoint (historical): PR #42 was preserved until its changes were delivered through these milestones; it did not merge the combined implementation ahead of phase completion.

## Final contract audit

Phase 2 completed: PR #44 auto-merged on 2026-10-04 at 13:57:02 UTC from `d202416a`; all required checks were satisfied and all three review threads resolved. [Hosted workspace](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37207334418), [permissions](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37207334374), [catalog](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37207334425), and [Gitleaks/adopter](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37207334440) succeeded. `RT-3zm.2` is closed. Earlier preparation notes above are historical.

| Contract | Implementation and verification evidence |
|---|---|
| OWASP semantic cutover | PR #43 merged; semantic mutation/catalog/historical tests and canonical SARIF identities/citations pass. Native export and taxonomy refresh preserve all 86 canonical identities; empty-PATH refresh leaves the controls companion byte-identical. |
| Permissions CI | Rust command validates JSON, required sections and documented rules; malformed fixtures and workflow contracts pass. Hosted permissions check is separate and read-only. |
| MAESTRO states | Shared typed classification retains all seven layers and defaults ambiguous evidence to not evaluated. Both infographic formats and Typst agree; legacy status strings are preserved. Zero-finding evidence accepts level-three and level-four headings. |
| Catalog and baselines | Manifest v2 checks ordered catalog membership/scope, rendering-input fingerprints and all seven PDF hashes offline. Input mutations, citation-only exceptions and failed-render rollback pass. Pinned Typst 0.15.1 regeneration and byte-identical comparisons pass without Python. |
| Output integrity | Mixed OI/LLM fixtures preserve findingId/v1, CWE-943, OWASP citations, assets and source paths through Rust parsing/report/SARIF. Nested canonical citations and explicit empty records survive. This is deterministic transformation evidence; upstream #356 remains failed live-agent verification. |
| Adopter scanning | Inherited Gitleaks 8.30.1 defaults, synthetic credential failures, permitted placeholders and malformed configuration are exercised by native Rust tests and hosted scanning. |
| Report review remediation | Recommendations populate remediation actions; branding is detected/staged; component distribution follows the selected tier. Attack evidence remains available, and timeline-only reports enable their remediation roadmap. Dedicated regressions cover these cases. |

Dependency audit: no external package was added or upgraded. The existing Rust `serde_yaml_ng` dependency moved into core runtime; its normal/build tree has no interpreter requirement. New workflows reuse existing checkout/cache pins and shell-based Rust setup; Gitleaks remains a native pinned executable. No Python application, build, test, generation, documentation or new action dependency was introduced.

Historical checkpoint: full workspace and Clippy passed after the MAESTRO compatibility and rendering-input fixes. The subsequent timeline-only correction passed report-document, extraction and remediation tests plus Clippy; hosted workspace checks passed on its final head. Formatting, actionlint, documentation gates, no-interpreter CLI checks and seven pinned PDF comparisons passed. At the time, only the integration PR's final validation and merge remained publication gates; PR #42 later satisfied those gates and merged at `8df554e884b1e5dd24146111a965597eff5f4779`.

Late reviews on the merged phase PRs are remediated in #42: numbered nested attribution retains OWASP/CWE records; companion ML references derive their year from the catalog; empty control assessments retain their metadata; attack-chain output includes only surfaced entries; and compact attack-tree metadata splits fields at separators. Each correction has a dedicated regression, and affected PDF baselines are regenerated with the same pinned native tool.

Follow-up error-path checks require recognizable assessment evidence before selecting the control tier and propagate malformed attribution errors through CLI, desktop, MCP and baseline generation. Invalid input cannot publish a zero-finding report or overwrite an existing output. The legacy string API emits an explicit Typst panic document for invalid input; checked APIs return the original diagnostic.

## Final delivery status

As of 2026-10-04, all three milestones are complete: PRs #43, #44, and #42
merged through protected auto-merge; all required final-head checks and review
threads passed; and live Beads epic `RT-3zm` with children `.1`–`.3` is
closed. The final integrated SHA is `8df554e884b1e5dd24146111a965597eff5f4779`.
Earlier progress bullets above are dated checkpoints, not current open work.
