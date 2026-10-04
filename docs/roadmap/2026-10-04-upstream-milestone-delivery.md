# Rust-native upstream roadmap: milestone delivery

Date: 2026-10-04. Tracking: epic `RT-3zm`. Original implementation: epic `RT-5vk` and [PR #42](https://github.com/pratik-saptarshi/tachi-rust/pull/42). Comparison anchors remain upstream `63438d78` and fork `cb567d32`.

The six-feature implementation was initially submitted together. This delivery plan separates it into independently validated milestones. Implementation issue closure is historical evidence of code and tests; it does not prove merge readiness. Three actionable review findings on PR #42 require remediation before reporting delivery complete.

## Invariants

- Preserve all six original feature contracts and the Rust architecture. Introduce no direct or indirect Python execution requirements in runtime, build, tests, documentation, generation or newly adopted workflows.
- Preserve the original dirty checkout and the separate PR #26 closeout scope. Work in isolated worktrees; never force-push or weaken branch protection to deliver a phase.
- At each phase start, inspect every open PR's live head, CI and review state. Fix in-scope failures before enabling that phase's auto-merge. An existing green check does not substitute for unresolved review or quality gates.
- Each phase has its own PR and final-head validation. Enable protected squash auto-merge only after its acceptance and review findings are addressed. GitHub must enforce the required checks and quality rules; no administrator bypass.

## Milestones and validation

| Phase | Deliverable and acceptance | Validation | Tracking / PR |
|---|---|---|---|
| 1 | Contextual OWASP 2026 semantic cutover. Active instructions, fixtures and current baseline example citations agree; historical references remain historical; finding schema unchanged. | Rust semantic mutation tests, catalog resolution, historical exceptions, parser/taxonomy/SARIF regressions, formatting, documentation gates and all required hosted checks. | `RT-3zm.1`; standalone PR to be recorded after validation |
| 2 | Native reporting and automation: shared truthful MAESTRO states, permissions check, catalog drift/regeneration, OI identity/citation preservation and adopter scanning. Correct PR #42 review findings: populate remediation actions, detect/stage brand assets, and derive component distribution from the active tier. | Focused review regressions; all seven MAESTRO states agree across outputs; malformed/valid permission cases; drift and failed regeneration rollback; mixed OI/LLM identities/CWE/citations/assets; native Gitleaks inheritance; six pinned PDF comparisons; no-interpreter CLI/regeneration; full workspace, Clippy, formatting, docs, actionlint and hosted checks. | `RT-3zm.2`; depends on phase 1; standalone PR after implementation review |
| 3 | Final requirement-by-requirement delivery audit, canonical roadmap/codemap/evidence and Beads export reconciliation. Record actual phase PR merge states and remaining repository gates. | Refresh all open PRs, verify final main/branch freshness, native catalog/permissions checks, exact baseline hashes and rendering provenance, dependency execution-chain audit, final documentation and hosted checks. | `RT-3zm.3`; depends on phase 2; PR #42 retained as integration/evidence record |

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

## Progress

- Phase 1 review prerequisite: native SARIF export now carries explicit Markdown attribution by finding identity, including the canonical nested YAML format and explicit empty records. The companion refresh command therefore retains current citations. This moves the necessary parsing/export portion of UF-05 forward; the full reporting acceptance remains in phase 2. Existing `serde_yaml_ng` is promoted from development to runtime dependency without adding a package or Python execution chain.

- Phase 1: [PR #43](https://github.com/pratik-saptarshi/tachi-rust/pull/43), extracted from freshly fetched main. Review found valid-but-wrong category IDs in examples. Corrected retrieval poisoning to LLM09, training poisoning to LLM05, model extraction/theft to LLM06 per the loaded pattern catalog, and configuration leakage to LLM08 across active agents/adapters. Added parsed-YAML example checks and stale mutations; focused Rust, formatting and documentation validation passed. Hosted checks remain the phase merge gate.
- Phase 2: independent preparation while phase 1 finishes its hosted gate. Three report review fixes are implemented with focused Rust regressions. Canonical sample review also reproduced missing nested citations and suppressed attack sections; the Rust path now preserves both. All original six PDFs plus the canonical agentic sample regenerate with pinned Typst and `PATH=/nonexistent`, including existing PDF companions; all seven byte-identical comparisons pass. Full workspace tests, Clippy, native permissions/catalog commands, Gitleaks adopter tests, actionlint and documentation gates pass. Publication to main follows phase 1 merge.
- Phase 3: pending phase gates. PR #42 is preserved until its changes are delivered through these milestones; it must not merge the combined implementation ahead of phase completion.
