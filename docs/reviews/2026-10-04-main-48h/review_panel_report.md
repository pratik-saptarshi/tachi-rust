# Review Panel Report — current main, last 48 hours

**Work reviewed:** tachi-rust main at `8df554e884b1e5dd24146111a965597eff5f4779`\
**Date:** 2026-10-04\
**Window:** 2026-10-02 21:37:02 UTC–2026-10-04 21:37:02 UTC (committer timestamps on main)\
**Comparison base:** `dd3b293d81d358d1ae27424be83b720693539112`\
**Panel:** 6 reviewers + independent Auditor + Judge + verification specialists\
**Verdict:** REQUEST CHANGES — targeted corrections  |  **Score:** 6.5/10  |  **Confidence:** Medium overall; High for bounded defect mechanisms\
**Review mode:** Mixed — Precise code/config, Exhaustive supporting prose\
**Signals:** Rust, security, data/schema transformations, CI/infra, reliability, frontend/auth\
**Data flow trace:** Standard | 1 critical report path | 3 initial candidates, not automatic high-priority findings\
**Codebase state:** detached snapshot of fresh origin/main | 0 commits behind at capture and final freshness check | isolated worktree\
**Protocol:** Overseer 3.2.0, one run, two debate rounds, 33/33 mandatory phase artifacts passed; post-judge gate found no new P0/P1.

> **Runtime adaptation:** Opus and the skill's Agent-plugin specialists were unavailable. All agents used the same inherited available model. Three concurrent child slots required two isolated batches per six-reviewer phase. No mandatory phase was omitted; no compressed-run warning applies.

## Executive Summary

The panel recommends correcting **nine P2 defects** before relying on the affected paths. The strongest cases are cleanup deleting the only backing image, incomplete or malformed assessment artifacts suppressing known findings, and duplicate MAESTRO rows erasing positive evidence. Other defects affect image containment/rendering, attribution defaults, Windows test compilation, and redirect health classification. **Three P3 advisories** concern compatibility and attestation policy; no P0 or P1 is adopted. This is a focused change review, not a release certification.

**Correlation notice:** final reviewer scores span only 0.5 points (6.5–7.0). Shared model and shared-trace bias remain; agreement is not evidence. The judge's 6.5/10 is below the descriptive panel mean of 6.83 because the independent audit added two verified cases.

## Scope & Limitations

Seven merges were reviewed: PRs [#35](https://github.com/pratik-saptarshi/tachi-rust/pull/35), [#37](https://github.com/pratik-saptarshi/tachi-rust/pull/37), [#38](https://github.com/pratik-saptarshi/tachi-rust/pull/38), [#41](https://github.com/pratik-saptarshi/tachi-rust/pull/41), [#43](https://github.com/pratik-saptarshi/tachi-rust/pull/43), [#44](https://github.com/pratik-saptarshi/tachi-rust/pull/44), and [#42](https://github.com/pratik-saptarshi/tachi-rust/pull/42). The range has 286 changed paths; the shared text bundle has 15,270 lines. Binary PDFs and large generated lockfiles, SARIF companions, crosswalk data, manifest and tracker exports were excluded from the bundle and inspected selectively. Findings concern current resulting behavior introduced or worsened in this interval, not corrected intermediate bugs or general debt.

No production deployment, live-agent evaluation, full workspace rerun, Windows execution, browser/session flow, live database isolation, or publication crash/concurrency stress was performed. Focused Rust builds, private fixture reproductions, real curl against a private loopback server, and one pinned Typst minimal-pair compile supply runtime evidence. Product source, the original dirty checkout, branch protection and Beads were not changed.

Labels: **[VERIFIED]** means the bounded mechanism is supported; **[CONSENSUS]** records agreement rather than proof; **[DISPUTED]** preserves a priority difference; **[UNVERIFIED]** marks an unexecuted consequence. **[EXISTING_DEFECT]** is current behavior; **[PLAN_RISK]** here marks a conditional policy/contract advisory rather than a demonstrated broken promise. External taxonomy publication accuracy is not adjudicated.

## Score Summary

| Reviewer | Persona | Agreement intensity | Initial | Final | Recommendation |
|---|---|---|---:|---:|---|
| correctness | Correctness Hawk | 30% | 6.5 | 6.5 | Targeted corrections |
| architecture | Architecture Critic | 50% | 7 | 7 | Targeted corrections |
| security | Security Auditor | 30% | 7.5 | 7 | Targeted corrections |
| devils_advocate | Devil's Advocate | 20% | 6 | 6.5 | Targeted corrections |
| code_quality | Code Quality Auditor | 40% | 7 | 7 | Targeted corrections |
| pipeline | Pipeline Reviewer | 30% | 7.5 | 7 | Targeted corrections |

## Action Items

**9 P2 defects; 3 separate P3 advisories; 0 P0/P1.** IDs remain stable across this report, process history and HTML dashboard. A1–A9 are recommended corrections, in judge priority order; A10–A12 are optional policy decisions.

| ID | Original IDs / source | Title | Priority | Classification | Epistemic disposition | Confidence | Targeted tier/result |
|---|---|---|---|---|---|---|---|
| A1 | SEC-1 / Security | Cleanup deletes backing bytes of retained symlink | P2 | EXISTING_DEFECT | VERIFIED runtime and source | High | Not dispatched |
| A2 | COR-2, DA-1 / Correctness, DA | Incomplete controls become a zero-finding assessment | P2 | EXISTING_DEFECT | VERIFIED mechanism; priority judgment | High mechanism, Medium priority | Deep / VR_PARTIAL on mandatory P1 |
| A3 | COR-1, ARC-1 / Correctness, Architecture | Malformed risk table replaces valid findings | P2 | EXISTING_DEFECT | VERIFIED runtime and source | High | Not dispatched |
| A4 | AUD-1 / Completeness Auditor | Duplicate MAESTRO rows erase positive evidence | P2 | EXISTING_DEFECT | VERIFIED runtime and source | High | Not dispatched |
| A5 | COR-3, ARC-2 / Correctness, Architecture | Empty optional attack image blocks PDF | P2 | EXISTING_DEFECT | VERIFIED attack-tree PDF failure | High for exercised case | Deep / VR_CONFIRMED |
| A6 | SEC-2 / Security | Path-valued tree ID selects neighboring image | P2 | EXISTING_DEFECT | VERIFIED selection; disclosure conditional | High selection, Medium downstream impact | Not dispatched |
| A7 | CQ-2 / Code Quality; Security independently reproduced | Nested attribution rejects documented default | P2 | EXISTING_DEFECT | VERIFIED runtime and explicit contract | High | Not dispatched |
| A8 | PIPE-1 / Pipeline | Unix-only helper breaks Windows test compilation | P2 | EXISTING_DEFECT | VERIFIED source-target dependency; execution unverified | High source conclusion | Not dispatched |
| A9 | AUD-2 / Completeness Auditor | Failed redirect traversal classified healthy | P2 | EXISTING_DEFECT | VERIFIED real-curl local probe | High | Not dispatched |
| A10 | CQ-1 / Code Quality | Clarify historical versionless taxonomy inputs | P3 advisory | PLAN_RISK / contract clarification | VERIFIED edition collision; stronger obligation unestablished | Medium | Standard / VR_PARTIAL |
| A11 | PIPE-2 / Pipeline | Clarify builder/source revision precondition | P3 advisory | PLAN_RISK / contract clarification | VERIFIED composition; mismatch consequence conditional | Medium | Standard / VR_PARTIAL |
| A12 | DA-2 / DA | Clarify companion PDF integrity coverage | P3 advisory | PLAN_RISK / contract clarification | VERIFIED checker scope; broader promise unestablished | High scope, Medium obligation | Not dispatched |



## Defect action items

Effort estimates are qualitative implementation-plus-focused-regression estimates, not delivery commitments: Small is a localized guard/helper or test change; Medium involves a parser/consumer boundary or multiple legitimate states. All source paths below are relative to the pinned worktree. Citations refer to that immutable reviewed revision.

### A1 — P2: preserve backing bytes during mislabeled-image cleanup

- **Source/snippet:** [crates/tachi-core/src/assets.rs:41](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/assets.rs#L41) selects a mislabeled sibling then calls `fs::remove_file(&path)`; `:61-64` compares via `(fs::read(left), fs::read(right))`.
- **Trigger/consequence:** A correctly named PNG symlink points at the mislabeled regular JPG containing PNG bytes. Reads compare equal; cleanup unlinks the backing JPG, leaving both logical paths unreadable. Runtime source harness and retained dangling-link fixture support actual loss of sole backing bytes.
- **Safeguard/intro:** Fixed stems, byte equality and opt-in cleanup constrain the action but do not prove retained-file independence. This cleanup is new in the interval. No arbitrary-root deletion is claimed.
- **Evidence/source reviewers:** SEC-1, Security Phase 3/7; Phase 10 item 4. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Establish that the retained sibling survives deletion, or conservatively skip symlink-dependent cleanup; use no-follow metadata where appropriate. Test retained symlink to candidate, ordinary independent duplicate and missing/different sibling. Preserve the ordinary safe cleanup behavior. Medium.

### A2 — P2: require completion before controls replace findings

- **Source/snippet:** [crates/tachi-core/src/report_data.rs:289](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/report_data.rs#L289): `!data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty()` returns true. Independently `:306-340` recognizes a residual header and returns true without proving row consumption; `:174-177` replaces findings. [crates/tachi-core/src/compensating_controls.rs:125](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/compensating_controls.rs#L125) only parses exact severity subsections.
- **Trigger/consequence:** (1) A Missing/None control descriptor without residual assessment; (2) a populated residual table outside required severity grouping. Both can replace existing High S-1 with Tier 1, zero findings and zero High count while generation succeeds. The second input is noncanonical. This is output loss, not source-file deletion.
- **Safeguard/intro:** Numeric and completed-empty checks exist; inventory bypasses them, and header recognition does not detect unconsumed rows. New authoritative promotion exposes permissive parsing. Existing tests at `crates/tachi-core/tests/report_document_contract.rs:249-267,311-346` require both incomplete-input protection and legitimate completed-empty selection.
- **Evidence/source reviewers:** COR-2/DA-1, Correctness and DA; Phase 10 item 2 replay/inspection; Phase 11 and Phase 13 point 1. VERIFIED / EXISTING_DEFECT, High mechanism and Medium priority confidence. Preserve DA P1 dissent.
- **Fix/regression/effort:** Model assessment completion separately from inventory; reject or diagnose populated unparsed assessment rows. For each independent trigger, retain the known finding or return a checked error. Retain completed-empty output, including its control metadata. At least two negative regressions plus the existing positive empty contract. Medium.

### A3 — P2: validate optional risk rows before selecting Tier 2

- **Source/snippet:** [crates/tachi-core/src/report_data.rs:164](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/report_data.rs#L164): `if !rows.is_empty()` selects Tier 2. [crates/tachi-core/src/parsers/findings.rs:158](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/parsers/findings.rs#L158) defaults missing fields via `unwrap_or_default()`; `report_data.rs:268-276` recomputes counts from the replacement.
- **Trigger/consequence:** A nonempty unrelated/malformed scored table alongside valid High S-1 produces one anonymous all-empty finding and High=0, with success. Retained CLI minimal-pair evidence proves the projection change.
- **Safeguard/intro:** Primary attribution checking validates another input; no scored header/identity/severity validation intervenes. The parser preexists; new report-data tier selection introduces the regression.
- **Evidence/source reviewers:** COR-1/ARC-1, Correctness and Architecture; Phase 10 item 1. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Validate the scored column/row contract before promotion and reject invalid input through the checked API or retain the lower tier with diagnostic. Test unrelated headers and canonical headers with missing identity/severity cells; assert known S-1 never becomes an anonymous row. Medium.

### A4 — P2: prevent duplicate MAESTRO rows from erasing positive evidence

- **Source/snippet:** [crates/tachi-core/src/infographic.rs:273](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/infographic.rs#L273): `by_layer.insert(layer_id.clone(), MaestroLayerDistribution { ... })`; `:292-306` emits the reduced map. Repair at `:443-474` requires observed detailed findings.
- **Trigger/consequence:** Summary-only canonical rows `L1 | 2 | High` then `L1 | 0 | Clean` yield count 0, clean state and empty most-exposed layer. Runtime output proves silent loss of positive evidence.
- **Safeguard/intro:** Positive-count precedence applies per row before overwrite. Detailed-row reconciliation cannot repair summary-only input. Base appended both rows to a Vec; new map reduction introduces last-row-wins loss.
- **Evidence/source reviewers:** AUD-1, independent Completeness Auditor; Phase 10 item 11 and Phase 11. VERIFIED / EXISTING_DEFECT, High confidence. Audit source and guard were independently re-read by the judge.
- **Fix/regression/effort:** Reject conflicting duplicate IDs or merge conservatively so positive evidence dominates clean/not-applicable states. Cover both row orders, normalized aliases and summary-only input. Avoid blindly summing duplicate counts. Medium.

### A5 — P2: skip unusable optional attack images and preserve fallback

- **Source/snippet:** [crates/tachi-core/src/report_data.rs:227](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/report_data.rs#L227), especially `:234`: `.find(|path| path.is_file())`. [templates/tachi/security-report/attack-path.typ:69](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/templates/tachi/security-report/attack-path.typ#L69) calls `image(img-path, ...)` at :74 before raw Mermaid text fallback.
- **Trigger/consequence:** A zero-byte preferred PNG for a valid attack tree sets `has-image=true`; report-data succeeds but actual Typst compilation fails with unexpected EOF. Removing the copied empty image enables the existing text fallback and successful PDF compilation.
- **Safeguard/intro:** Filesystem existence is the only candidate acceptance test; the general asset helper's usability checks are not invoked. New image binding supplies the invalid candidate. Later formats are skipped by first-match selection; that extension is source-traced, not separately executed.
- **Evidence/source reviewers:** COR-3/ARC-2, Correctness and Architecture; Phase 10 item 3; Phase 13 point 2 integration minimal pair. VERIFIED / EXISTING_DEFECT, High confidence for the exercised tree path.
- **Fix/regression/effort:** Require a usable candidate, continue to valid alternatives and preserve no-image fallback. Test zero-byte preferred PNG plus available source text, and invalid first format plus valid later format. Evaluate nonempty invalid data deliberately rather than equating size with decodability. Chain selection should receive the same resolver fix, with its distinct template behavior tested. Medium.

### A6 — P2: contain image resolution for path-valued attack-tree IDs

- **Source/snippet:** `crates/tachi-core/src/attack_trees.rs:111,124-130` accepts metadata/heading IDs; [crates/tachi-core/src/report_data.rs:232](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/report_data.rs#L232) joins `format!("{id}-{suffix}.{ext}")` without containment. Image sink: [templates/tachi/security-report/attack-path.typ:74](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/templates/tachi/security-report/attack-path.typ#L74).
- **Trigger/consequence:** An ID containing `../../` selects a neighboring SVG outside the intended report asset directory. The generated binding was executed. Actual disclosure additionally requires compiler-root access, successful compilation and sharing; those steps were not executed. No RCE or arbitrary text read is established.
- **Safeguard/intro:** A separate heading grammar check does not cover metadata or the cited Attack Tree heading branch. Relative-path conversion is not containment. New image-resolution composition introduces the unsafe selection.
- **Evidence/source reviewers:** SEC-2, Security; Phase 10 item 5. VERIFIED selection / EXISTING_DEFECT; High mechanism confidence, Medium downstream impact confidence.
- **Fix/regression/effort:** Validate IDs against the intended identifier grammar and enforce resolved-path containment, accounting for symlinks. Cover metadata and heading ingress, traversal/absolute values, symlink escape and a valid ID. Return a checked error or omit unsafe images with a clear diagnostic. Medium.

### A7 — P2: honor the primary relationship default in nested attribution

- **Source/snippet:** [crates/tachi-core/src/parsers/findings.rs:63](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/parsers/findings.rs#L63): `pub relationship: String` has no serde default; nested `serde_yaml::from_str(yaml)` at `:493-498` requires it. Flat parsing defaults at :600. [schemas/finding.yaml:277](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/schemas/finding.yaml#L277) specifies `default: primary` and injection when absent.
- **Trigger/consequence:** A valid nested record omitting relationship fails with missing-field error and command exit 1, while the equivalent flat record succeeds with primary. Two reviewers independently reproduced the minimal pair.
- **Safeguard/intro:** Validation/normalization happens after nested deserialization and cannot apply the default to a record never created. The nested path is new; the flat default predates it.
- **Evidence/source reviewers:** CQ-2, Code Quality; Security independent probe; Phase 10 item 7. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Add an explicit serde default returning primary or normalize through an equivalent boundary. Test absent relationship parity between nested and flat forms, explicit valid values and invalid values; preserve absent-versus-empty attribution semantics. Small.

### A8 — P2: isolate Unix-only test helpers from Windows targets

- **Source/snippet:** [crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:200](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-cli/src/bin/taxonomy-link-monitor.rs#L200): only `#[cfg(test)]` guards `use std::os::unix::fs::PermissionsExt`; :205-218 invokes `set_mode`.
- **Trigger/consequence:** Compiling this binary's test target for Windows includes unavailable Unix APIs. This is a deterministic source-target dependency finding, not an executed Windows failure or claim about every runtime feature's portability.
- **Safeguard/intro:** No `cfg(unix)` guard or applicable Cargo autobin exemption is identified. The binary is new.
- **Evidence/source reviewers:** PIPE-1, Pipeline; Phase 10 item 8. VERIFIED source / EXISTING_DEFECT, High confidence; Windows execution UNVERIFIED.
- **Fix/regression/effort:** Gate Unix-specific imports/helpers/tests appropriately or supply a portable fake executable. Keep platform-independent classifier tests enabled everywhere. Validate test-target compilation on a Windows target/runner after correction. Small.

### A9 — P2: treat exhausted redirects as failed health checks

- **Source/snippet:** [crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:157](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-cli/src/bin/taxonomy-link-monitor.rs#L157): `200..=399 => "healthy"` precedes failed-process handling. Error capture :148-154 retains curl failure; :113-119 retries HEAD with GET.
- **Trigger/consequence:** A loopback endpoint endlessly redirects. Real curl exits 47 after maximum redirects for both HEAD and GET but outputs HTTP302. The monitor reports one healthy URL while JSON also retains the curl error.
- **Safeguard/intro:** Retry, timeout and error capture exist. The final classifier overrides failed traversal with the residual status. The new monitor introduces the behavior. Its informational success exit is intentional and is not a CI-gate bypass.
- **Evidence/source reviewers:** AUD-2, independent Completeness Auditor; Phase 10 item 12 and Phase 11. VERIFIED / EXISTING_DEFECT, High confidence. Judge re-read actual classifier and failure capture.
- **Fix/regression/effort:** Prioritize relevant process/transport failure over residual successful/redirect codes, at least redirect exhaustion; report needs-review/transient rather than healthy. Add failed-302 classifier coverage and a real local redirect-loop regression while retaining informational command semantics. Small.

## Separate advisories

### A10 — P3 advisory: define historical versionless taxonomy policy

- **Source/snippet:** [schemas/taxonomy/owasp.yaml:471](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/schemas/taxonomy/owasp.yaml#L471) now associates LLM05 with Data and Model Poisoning; base associated it with Improper Output Handling. [crates/tachi-core/src/threats_sarif.rs:170](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/threats_sarif.rs#L170) formats primary LLM references with `:2026`. [schemas/finding.yaml:254](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/schemas/finding.yaml#L254) provides no edition field and requires current-catalog resolution.
- **Trigger/consequence:** Regenerating versionless historical attribution can reinterpret an ID under the current catalog. A mandatory historical-regeneration compatibility contract is unestablished; external OWASP publication accuracy is not adjudicated.
- **Source/confidence/class:** CQ-1, Code Quality; Phase 13 point 3, VR_PARTIAL; Medium obligation confidence; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** Document current-catalog resolution and the caller's legacy review obligation, or explicitly support edition-aware preservation and add historical fixtures. Do not silently guess ID remaps. Small for documentation; Medium or greater for chosen edition support.

### A11 — P3 advisory: state the same-source executable precondition

- **Source/snippet:** [crates/tachi-core/src/catalog_drift.rs:242](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/catalog_drift.rs#L242) captures supplied-root hashes, :278 calls compiled `crate::try_build_report_data_typst`, and :319-329 rechecks/publishes root hashes. CLI root override: `crates/tachi-cli/src/bin/catalog-drift.rs:10-12,21-24`. Documented same-checkout Cargo command: [docs/feature-roadmap-2026-10-04.md:91](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/docs/feature-roadmap-2026-10-04.md#L91).
- **Trigger/consequence:** A compatible binary A used against source root B can theoretically attest B's disk inputs while building report data with A's compiled logic. No stale-binary mismatch was executed or current PDF corruption shown. Same-checkout Cargo invocation limits the ordinary trigger; input stability and rollback do not authenticate executable identity.
- **Source/confidence/class:** PIPE-2, Pipeline; Phase 13 point 4, VR_PARTIAL; Medium confidence in broader consequence/obligation; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** Document that the builder must come from the selected source revision and use the prescribed invocation. Only if arbitrary cross-revision generation is supported, define a mismatch check and targeted regression. Small clarification; larger attestation work needs an explicit requirement.

### A12 — P3 advisory: distinguish baseline checks from companion checks

- **Source/snippet:** [crates/tachi-core/src/catalog_drift.rs:78](https://github.com/pratik-saptarshi/tachi-rust/blob/8df554e884b1e5dd24146111a965597eff5f4779/crates/tachi-core/src/catalog_drift.rs#L78) excludes companion PDFs from rendering inputs, :183-202 checks registered baseline hashes, and :313-317 publishes companions when present.
- **Trigger/consequence:** Altering a published companion in a private fixture does not fail `--check` when registered baselines remain valid. The fixture behavior was reproduced; current committed PDFs are not claimed corrupt. A promise to verify every published copy is unestablished.
- **Source/confidence/class:** DA-2, DA; Phase 10 item 10; High scope confidence, Medium obligation confidence; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** State checker scope clearly. If companion integrity becomes a supported guarantee, register/hash those outputs and test companion-only corruption. Small clarification; Medium coverage extension.

## Consensus Points (7)

1. **Validity must precede authoritative selection.** Nonempty parsed vectors and control inventory do not establish a complete assessment. Source replacement and runtime outputs support this, independently of six reviewers agreeing.
2. **A completed empty assessment is legitimate.** Existing contract tests explicitly preserve it; rejecting all empty controls output would break intended behavior.
3. **Three asset defects remain distinct.** Retained-file survival, candidate usability, and candidate containment require different guarantees and regressions.
4. **Nested attribution has an explicit default contract.** The YAML schema promises `primary`, while direct nested serde deserialization requires the field. This is stronger evidence than an inferred compatibility promise.
5. **Existing safeguards are meaningful but bounded.** Checked primary attribution, MAESTRO detailed-row repair, cleanup byte equality, catalog staging/source stability/rollback and monitor retries exist; none resolves the specific bypass cases below.
6. **Catalog checks and baseline equality are not semantic correctness proofs.** They verify their declared bytes and inputs; they do not prove assessment completeness, historical semantics or arbitrary executable provenance.
7. **The appropriate result is targeted corrections with explicit limits.** No broad compromise, current committed PDF corruption or complete release readiness is established.

## Disagreement Points and Judge Rulings (5)

### R1 — Controls false-zero priority: P2, preserve P1 dissent

DA's P1 / Medium priority-confidence position has a legitimate consequence argument: successful security output omits a known High finding, and the incomplete inventory trigger is small. Actual user harm is not a prerequisite for urgency, and missing prevalence data is not proof of rarity. Source-input survival does not repair the misleading output.

The minimum justified priority on this record remains P2. The established triggers are an incomplete local inventory and a noncanonical residual table; a completed canonical producer path or automatic security decision hitting them is not established. This positively bounds the demonstrated failure. The roadmap's P1 MAESTRO truthfulness goal supports concern but does not impose a universal bug-priority policy; its P2 output-integrity goal likewise does not decide the matter mechanically. No inspected priority policy resolves the tradeoff. The ruling therefore weighs the demonstrated boundary and urgency, not five-to-one voting, imagined rarity or a universal remote-exploit requirement. Keep both controls regression cases, the impact rationale and the dissent in downstream reports.

### R2 — Malformed risk original P1: retain P2

ARC-1's initial P1 was explicitly withdrawn after trigger review. A malformed optional table becomes an anonymous row and suppresses known severity counts. That is a material new composition defect; the permissive parser itself predates the window. No source artifact deletion or ordinary completed-input exposure is shown. P2 is proportionate without calling the issue cosmetic.

### R3 — Empty-image downstream consequence: now executed, still P2

Phase 10 correctly limited its claim to binding/fallback evidence. Phase 13 subsequently compiled the attack-tree fixture with pinned Typst 0.15.1: generation exit 0, compile exit 1, `failed to decode image (unexpected end of file)`. Removing only the copied zero-byte PNG restored generation and compilation exit 0 and a 12-page, 95,852-byte PDF with empty diagnostics. This supersedes earlier unexecuted-PDF caveats for this exact attack-tree case. It does not verify neighboring-image disclosure, every corrupt image, later-format fallback or attack-chain compilation. Chains share selection but have no Mermaid text fallback. The narrow malformed-artifact trigger remains P2.

### R4 — Historical taxonomy compatibility: advisory

Current/base catalog meaning collision and current SARIF edition labeling are supported. Explicit historical-reference preservation in the cutover documentation concerns source editing; the versionless attribution schema resolves IDs in the current catalog without an edition field. Those facts do not establish promised preservation or migration of arbitrary historical versionless runtime input. Keep A10 as a policy clarification, not a mandatory automatic remapping change. This narrowing rests on contract scope, not absence of reported harm.

### R5 — Provenance and companion checking: two separate advisories

Source hashing does not authenticate the compiled Rust builder; baseline checking does not cover every published companion. Both mechanisms are real. However, the prescribed same-checkout Cargo invocation constrains stale-binary use, and the stated checker contract targets registered baselines. No current committed artifact corruption or broken same-checkout procedure is demonstrated. Preserve A11 and A12 separately because executable identity and output coverage are different concerns. Do not invent an attestation subsystem or release gate as a required correction.

Debate quality was adequate: the four narrowed positions cite input or contract evidence, and DA retained a qualified minority position. There is no evidence here of unsupported score following. Final-score spread is only 0.5, so **correlated-bias warning applies**. Shared models and the shared trace further limit diversity. Two debate rounds were completed; stopping before a third repetition did not skip the required debate work.

## Completeness Audit Findings

Both new audit findings survived source and runtime checks: **A4**, duplicate MAESTRO summary evidence lost by map insertion; **A9**, curl redirect exhaustion classified healthy. They are separate from the panel's report-tier findings. No judge-introduced finding was added.

## Verification Summary

| Stage | Result |
|---|---|
| Independent review / reflection / debate / blind final | All six reviewers completed every mandatory phase; two debate rounds |
| Citation verification | 12 unique mechanisms checked; no hallucinated or misattributed finding |
| Severity verification | Minimum justified P2; controls P1 dissent preserved |
| Targeted verification | 1 VR_CONFIRMED, 3 VR_PARTIAL, 0 refuted, 0 inconclusive |
| Image minimal pair | Bad image: generation exit 0, Typst exit 1; remove only copied image: successful 12-page PDF |
| Pre-judge output gate | 33/33 existence, minimum size and schema-marker checks pass |
| Post-judge gate | No judge-introduced P0/P1 findings |
| Main freshness / product diff | Remote main still 8df554e8; tracked product diff empty |

## Coverage Gaps

Coverage remains uneven. The panel and supplemental audit touched CI permissions, workflow triggers, frontend authentication and Prisma/RLS contracts, but did not execute browser/session flows, live database isolation, Windows compilation, crash durability or concurrent catalog publication. Generated/binary assets and large registry/lockfile surfaces were excluded from the shared text bundle and only selectively inspected. No production agent generation, real-user prevalence, operational decision impact or external taxonomy publication correctness was verified. Root/shell containment elsewhere does not automatically guard the direct report path; conversely an unused Prisma client does not establish a new tenant-exposure endpoint. Preexisting middleware cookie behavior is excluded from this interval. These remain limitations, not manufactured defects.

## Detailed Reviews and Evidence

[Full chronological process history](review_panel_process.md) preserves the complete written agent outputs, profiles, debate, verification trails and judge ruling. [Interactive HTML dashboard](review_panel_report.html) provides expandable finding cards and filters.

<details>
<summary>Setup and data-flow map</summary>

- [context.md](state/context.md)
- [personas.md](state/personas.md)
- [commits.txt](state/commits.txt)
- [phase_2_data_flow.md](state/phase_2_data_flow.md)

</details>

<details>
<summary>Round 0 independent reviews</summary>

- [reviewer_correctness_phase_3.md](state/reviewer_correctness_phase_3.md)
- [reviewer_architecture_phase_3.md](state/reviewer_architecture_phase_3.md)
- [reviewer_security_phase_3.md](state/reviewer_security_phase_3.md)
- [reviewer_devils_advocate_phase_3.md](state/reviewer_devils_advocate_phase_3.md)
- [reviewer_code_quality_phase_3.md](state/reviewer_code_quality_phase_3.md)
- [reviewer_pipeline_phase_3.md](state/reviewer_pipeline_phase_3.md)

</details>

<details>
<summary>Private reflections</summary>

- [reviewer_correctness_phase_4.md](state/reviewer_correctness_phase_4.md)
- [reviewer_architecture_phase_4.md](state/reviewer_architecture_phase_4.md)
- [reviewer_security_phase_4.md](state/reviewer_security_phase_4.md)
- [reviewer_devils_advocate_phase_4.md](state/reviewer_devils_advocate_phase_4.md)
- [reviewer_code_quality_phase_4.md](state/reviewer_code_quality_phase_4.md)
- [reviewer_pipeline_phase_4.md](state/reviewer_pipeline_phase_4.md)

</details>

<details>
<summary>Debate rounds and summaries</summary>

- [reviewer_correctness_phase_5_round1.md](state/reviewer_correctness_phase_5_round1.md)
- [reviewer_architecture_phase_5_round1.md](state/reviewer_architecture_phase_5_round1.md)
- [reviewer_security_phase_5_round1.md](state/reviewer_security_phase_5_round1.md)
- [reviewer_devils_advocate_phase_5_round1.md](state/reviewer_devils_advocate_phase_5_round1.md)
- [reviewer_code_quality_phase_5_round1.md](state/reviewer_code_quality_phase_5_round1.md)
- [reviewer_pipeline_phase_5_round1.md](state/reviewer_pipeline_phase_5_round1.md)
- [phase_6_round1_summary.md](state/phase_6_round1_summary.md)
- [reviewer_correctness_phase_5_round2.md](state/reviewer_correctness_phase_5_round2.md)
- [reviewer_architecture_phase_5_round2.md](state/reviewer_architecture_phase_5_round2.md)
- [reviewer_security_phase_5_round2.md](state/reviewer_security_phase_5_round2.md)
- [reviewer_devils_advocate_phase_5_round2.md](state/reviewer_devils_advocate_phase_5_round2.md)
- [reviewer_code_quality_phase_5_round2.md](state/reviewer_code_quality_phase_5_round2.md)
- [reviewer_pipeline_phase_5_round2.md](state/reviewer_pipeline_phase_5_round2.md)
- [phase_6_round2_summary.md](state/phase_6_round2_summary.md)

</details>

<details>
<summary>Blind final assessments</summary>

- [reviewer_correctness_phase_7.md](state/reviewer_correctness_phase_7.md)
- [reviewer_architecture_phase_7.md](state/reviewer_architecture_phase_7.md)
- [reviewer_security_phase_7.md](state/reviewer_security_phase_7.md)
- [reviewer_devils_advocate_phase_7.md](state/reviewer_devils_advocate_phase_7.md)
- [reviewer_code_quality_phase_7.md](state/reviewer_code_quality_phase_7.md)
- [reviewer_pipeline_phase_7.md](state/reviewer_pipeline_phase_7.md)

</details>

<details>
<summary>Audit and verification</summary>

- [phase_8_audit.md](state/phase_8_audit.md)
- [phase_9_commands.md](state/phase_9_commands.md)
- [phase_10_claim_verification.md](state/phase_10_claim_verification.md)
- [phase_11_severity_verification.md](state/phase_11_severity_verification.md)
- [phase_12a_tiers.md](state/phase_12a_tiers.md)
- [phase_12b_tiers.md](state/phase_12b_tiers.md)
- [phase_13_point_1_controls.md](state/phase_13_point_1_controls.md)
- [phase_13_point_2_image.md](state/phase_13_point_2_image.md)
- [phase_13_point_3_history.md](state/phase_13_point_3_history.md)
- [phase_13_point_4_provenance.md](state/phase_13_point_4_provenance.md)
- [phase_13_summary.md](state/phase_13_summary.md)
- [phase_13_5_gate.json](state/phase_13_5_gate.json)

</details>

<details>
<summary>Judge and post-judge verification</summary>

- [phase_14_judge_ruling.md](state/phase_14_judge_ruling.md)
- [phase_14_5_judge_verification.md](state/phase_14_5_judge_verification.md)

</details>

---
Generated using [Overseer 3.2.0](/Users/neo/.agents/skills/overseer/SKILL.md). HTML enhancements use Tailwind CSS, Chart.js and Prism.js CDNs; text remains readable without them.
