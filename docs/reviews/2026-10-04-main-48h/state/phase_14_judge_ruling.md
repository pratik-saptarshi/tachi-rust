# Phase 14 — Supreme Judge ruling

**Verdict: REQUEST CHANGES. Score: 6.5/10. Confidence: High in the bounded defect mechanisms; Medium in priority and overall coverage.**

Reviewed pinned main `8df554e884b1e5dd24146111a965597eff5f4779` against base `dd3b293d81d358d1ae27424be83b720693539112`, covering the seven merged commits in the context's 2026-10-02 21:37:02 UTC–2026-10-04 21:37:02 UTC window. This is a focused change review, not a release certification. The original dirty checkout is outside scope. No product edits or new runtime probes were performed by the judge.

## Machine-usable disposition

| Metric | Final count / value |
|---|---:|
| P0 defects | 0 |
| P1 defects | 0 |
| P2 defects | 9 |
| P3 mandatory defects | 0 |
| P3 advisories | 3 |
| Total stable action IDs | 12 |
| Consensus points below | 7 |
| Disagreement/arbitration rulings below | 5 |
| Accepted new completeness-audit findings | 2 |
| Targeted verification VR_CONFIRMED | 1 |
| Targeted verification VR_PARTIAL | 3 |
| Targeted verification VR_REFUTED | 0 |
| Targeted verification VR_INCONCLUSIVE | 0 |
| Judge-introduced findings | 0 |
| Judge-introduced P0/P1 | 0 |
| Mandatory pre-judge artifacts passing Phase 13.5 | 33/33 |

Stable priority order: **A1 → A2 → A3 → A4 → A5 → A6 → A7 → A8 → A9**, followed by advisories **A10 → A11 → A12**. This orders work within P2; it does not imply different formal severities. A1 prevents concrete data loss; A2/A3/A4 address successful suppression of security evidence. A5/A6 share image resolution code but remain separate bugs. Small independent fixes A7/A8 may proceed alongside them.

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

`VERIFIED` describes the bounded claim, not every proposed impact or severity. No item relies on consensus as its verification method. Every high-priority post-judge classification inventory is empty: there are no P0/P1 findings to label PANEL-RAISED or JUDGE-INTRODUCED. Phase 14.5 must still write its required empty-case artifact.

## Score assessment

| Reviewer | Initial Phase 3 | Blind final Phase 7 | Interpretation |
|---|---:|---:|---|
| Correctness Hawk | 6.5 | 6.5 | Three concrete report regressions retained |
| Architecture Critic | 7.0 | 7.0 | Architecture retained; risk priority narrowed |
| Security Auditor | 7.5 | 7.0 | Additional reproduced default/tier failures |
| Devil's Advocate | 6.0 | 6.5 | Companion/provenance concerns narrowed to advisories |
| Code Quality Auditor | 7.0 | 7.0 | Default defect retained; historical contract narrowed |
| Pipeline Reviewer | 7.5 | 7.0 | Additional report-integrity evidence; provenance advisory |
| Arithmetic mean, descriptive only | 6.92 | 6.83 | Not the judge's decision rule |

The final 6.5 reflects material but localized defects at report-authority, asset, schema and monitor boundaries, including two independent audit additions and an executed PDF failure after blind finals. Checked attribution errors, shared MAESTRO states, coherent selected-tier summaries and staged publication are valuable improvements. Neither a broad rewrite nor a rejection of the architecture is justified. The score is slightly below the panel mean because the verified audit additions broaden the remaining correction work. P2 does not imply acceptance: repeated wrong successful outputs require targeted fixes before reliance on the affected path.

## Consensus points validated independently

1. **Validity must precede authoritative selection.** Nonempty parsed vectors and control inventory do not establish a complete assessment. Source replacement and runtime outputs support this, independently of six reviewers agreeing.
2. **A completed empty assessment is legitimate.** Existing contract tests explicitly preserve it; rejecting all empty controls output would break intended behavior.
3. **Three asset defects remain distinct.** Retained-file survival, candidate usability, and candidate containment require different guarantees and regressions.
4. **Nested attribution has an explicit default contract.** The YAML schema promises `primary`, while direct nested serde deserialization requires the field. This is stronger evidence than an inferred compatibility promise.
5. **Existing safeguards are meaningful but bounded.** Checked primary attribution, MAESTRO detailed-row repair, cleanup byte equality, catalog staging/source stability/rollback and monitor retries exist; none resolves the specific bypass cases below.
6. **Catalog checks and baseline equality are not semantic correctness proofs.** They verify their declared bytes and inputs; they do not prove assessment completeness, historical semantics or arbitrary executable provenance.
7. **The appropriate result is targeted corrections with explicit limits.** No broad compromise, current committed PDF corruption or complete release readiness is established.

## Disagreements and rulings

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

## Defect action items

Effort estimates are qualitative implementation-plus-focused-regression estimates, not delivery commitments: Small is a localized guard/helper or test change; Medium involves a parser/consumer boundary or multiple legitimate states. All source paths below are relative to the pinned worktree. Citations refer to that immutable reviewed revision.

### A1 — P2: preserve backing bytes during mislabeled-image cleanup

- **Source/snippet:** `crates/tachi-core/src/assets.rs:41-51` selects a mislabeled sibling then calls `fs::remove_file(&path)`; `:61-64` compares via `(fs::read(left), fs::read(right))`.
- **Trigger/consequence:** A correctly named PNG symlink points at the mislabeled regular JPG containing PNG bytes. Reads compare equal; cleanup unlinks the backing JPG, leaving both logical paths unreadable. Runtime source harness and retained dangling-link fixture support actual loss of sole backing bytes.
- **Safeguard/intro:** Fixed stems, byte equality and opt-in cleanup constrain the action but do not prove retained-file independence. This cleanup is new in the interval. No arbitrary-root deletion is claimed.
- **Evidence/source reviewers:** SEC-1, Security Phase 3/7; Phase 10 item 4. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Establish that the retained sibling survives deletion, or conservatively skip symlink-dependent cleanup; use no-follow metadata where appropriate. Test retained symlink to candidate, ordinary independent duplicate and missing/different sibling. Preserve the ordinary safe cleanup behavior. Medium.

### A2 — P2: require completion before controls replace findings

- **Source/snippet:** `crates/tachi-core/src/report_data.rs:289-291`: `!data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty()` returns true. Independently `:306-340` recognizes a residual header and returns true without proving row consumption; `:174-177` replaces findings. `crates/tachi-core/src/compensating_controls.rs:125-130` only parses exact severity subsections.
- **Trigger/consequence:** (1) A Missing/None control descriptor without residual assessment; (2) a populated residual table outside required severity grouping. Both can replace existing High S-1 with Tier 1, zero findings and zero High count while generation succeeds. The second input is noncanonical. This is output loss, not source-file deletion.
- **Safeguard/intro:** Numeric and completed-empty checks exist; inventory bypasses them, and header recognition does not detect unconsumed rows. New authoritative promotion exposes permissive parsing. Existing tests at `crates/tachi-core/tests/report_document_contract.rs:249-267,311-346` require both incomplete-input protection and legitimate completed-empty selection.
- **Evidence/source reviewers:** COR-2/DA-1, Correctness and DA; Phase 10 item 2 replay/inspection; Phase 11 and Phase 13 point 1. VERIFIED / EXISTING_DEFECT, High mechanism and Medium priority confidence. Preserve DA P1 dissent.
- **Fix/regression/effort:** Model assessment completion separately from inventory; reject or diagnose populated unparsed assessment rows. For each independent trigger, retain the known finding or return a checked error. Retain completed-empty output, including its control metadata. At least two negative regressions plus the existing positive empty contract. Medium.

### A3 — P2: validate optional risk rows before selecting Tier 2

- **Source/snippet:** `crates/tachi-core/src/report_data.rs:164-168`: `if !rows.is_empty()` selects Tier 2. `crates/tachi-core/src/parsers/findings.rs:158-178` defaults missing fields via `unwrap_or_default()`; `report_data.rs:268-276` recomputes counts from the replacement.
- **Trigger/consequence:** A nonempty unrelated/malformed scored table alongside valid High S-1 produces one anonymous all-empty finding and High=0, with success. Retained CLI minimal-pair evidence proves the projection change.
- **Safeguard/intro:** Primary attribution checking validates another input; no scored header/identity/severity validation intervenes. The parser preexists; new report-data tier selection introduces the regression.
- **Evidence/source reviewers:** COR-1/ARC-1, Correctness and Architecture; Phase 10 item 1. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Validate the scored column/row contract before promotion and reject invalid input through the checked API or retain the lower tier with diagnostic. Test unrelated headers and canonical headers with missing identity/severity cells; assert known S-1 never becomes an anonymous row. Medium.

### A4 — P2: prevent duplicate MAESTRO rows from erasing positive evidence

- **Source/snippet:** `crates/tachi-core/src/infographic.rs:273-289`: `by_layer.insert(layer_id.clone(), MaestroLayerDistribution { ... })`; `:292-306` emits the reduced map. Repair at `:443-474` requires observed detailed findings.
- **Trigger/consequence:** Summary-only canonical rows `L1 | 2 | High` then `L1 | 0 | Clean` yield count 0, clean state and empty most-exposed layer. Runtime output proves silent loss of positive evidence.
- **Safeguard/intro:** Positive-count precedence applies per row before overwrite. Detailed-row reconciliation cannot repair summary-only input. Base appended both rows to a Vec; new map reduction introduces last-row-wins loss.
- **Evidence/source reviewers:** AUD-1, independent Completeness Auditor; Phase 10 item 11 and Phase 11. VERIFIED / EXISTING_DEFECT, High confidence. Audit source and guard were independently re-read by the judge.
- **Fix/regression/effort:** Reject conflicting duplicate IDs or merge conservatively so positive evidence dominates clean/not-applicable states. Cover both row orders, normalized aliases and summary-only input. Avoid blindly summing duplicate counts. Medium.

### A5 — P2: skip unusable optional attack images and preserve fallback

- **Source/snippet:** `crates/tachi-core/src/report_data.rs:227-247`, especially `:234`: `.find(|path| path.is_file())`. `templates/tachi/security-report/attack-path.typ:69-81` calls `image(img-path, ...)` at :74 before raw Mermaid text fallback.
- **Trigger/consequence:** A zero-byte preferred PNG for a valid attack tree sets `has-image=true`; report-data succeeds but actual Typst compilation fails with unexpected EOF. Removing the copied empty image enables the existing text fallback and successful PDF compilation.
- **Safeguard/intro:** Filesystem existence is the only candidate acceptance test; the general asset helper's usability checks are not invoked. New image binding supplies the invalid candidate. Later formats are skipped by first-match selection; that extension is source-traced, not separately executed.
- **Evidence/source reviewers:** COR-3/ARC-2, Correctness and Architecture; Phase 10 item 3; Phase 13 point 2 integration minimal pair. VERIFIED / EXISTING_DEFECT, High confidence for the exercised tree path.
- **Fix/regression/effort:** Require a usable candidate, continue to valid alternatives and preserve no-image fallback. Test zero-byte preferred PNG plus available source text, and invalid first format plus valid later format. Evaluate nonempty invalid data deliberately rather than equating size with decodability. Chain selection should receive the same resolver fix, with its distinct template behavior tested. Medium.

### A6 — P2: contain image resolution for path-valued attack-tree IDs

- **Source/snippet:** `crates/tachi-core/src/attack_trees.rs:111,124-130` accepts metadata/heading IDs; `crates/tachi-core/src/report_data.rs:232-246` joins `format!("{id}-{suffix}.{ext}")` without containment. Image sink: `templates/tachi/security-report/attack-path.typ:74`.
- **Trigger/consequence:** An ID containing `../../` selects a neighboring SVG outside the intended report asset directory. The generated binding was executed. Actual disclosure additionally requires compiler-root access, successful compilation and sharing; those steps were not executed. No RCE or arbitrary text read is established.
- **Safeguard/intro:** A separate heading grammar check does not cover metadata or the cited Attack Tree heading branch. Relative-path conversion is not containment. New image-resolution composition introduces the unsafe selection.
- **Evidence/source reviewers:** SEC-2, Security; Phase 10 item 5. VERIFIED selection / EXISTING_DEFECT; High mechanism confidence, Medium downstream impact confidence.
- **Fix/regression/effort:** Validate IDs against the intended identifier grammar and enforce resolved-path containment, accounting for symlinks. Cover metadata and heading ingress, traversal/absolute values, symlink escape and a valid ID. Return a checked error or omit unsafe images with a clear diagnostic. Medium.

### A7 — P2: honor the primary relationship default in nested attribution

- **Source/snippet:** `crates/tachi-core/src/parsers/findings.rs:63-67`: `pub relationship: String` has no serde default; nested `serde_yaml::from_str(yaml)` at `:493-498` requires it. Flat parsing defaults at :600. `schemas/finding.yaml:277-289` specifies `default: primary` and injection when absent.
- **Trigger/consequence:** A valid nested record omitting relationship fails with missing-field error and command exit 1, while the equivalent flat record succeeds with primary. Two reviewers independently reproduced the minimal pair.
- **Safeguard/intro:** Validation/normalization happens after nested deserialization and cannot apply the default to a record never created. The nested path is new; the flat default predates it.
- **Evidence/source reviewers:** CQ-2, Code Quality; Security independent probe; Phase 10 item 7. VERIFIED / EXISTING_DEFECT, High confidence.
- **Fix/regression/effort:** Add an explicit serde default returning primary or normalize through an equivalent boundary. Test absent relationship parity between nested and flat forms, explicit valid values and invalid values; preserve absent-versus-empty attribution semantics. Small.

### A8 — P2: isolate Unix-only test helpers from Windows targets

- **Source/snippet:** `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:200-203`: only `#[cfg(test)]` guards `use std::os::unix::fs::PermissionsExt`; :205-218 invokes `set_mode`.
- **Trigger/consequence:** Compiling this binary's test target for Windows includes unavailable Unix APIs. This is a deterministic source-target dependency finding, not an executed Windows failure or claim about every runtime feature's portability.
- **Safeguard/intro:** No `cfg(unix)` guard or applicable Cargo autobin exemption is identified. The binary is new.
- **Evidence/source reviewers:** PIPE-1, Pipeline; Phase 10 item 8. VERIFIED source / EXISTING_DEFECT, High confidence; Windows execution UNVERIFIED.
- **Fix/regression/effort:** Gate Unix-specific imports/helpers/tests appropriately or supply a portable fake executable. Keep platform-independent classifier tests enabled everywhere. Validate test-target compilation on a Windows target/runner after correction. Small.

### A9 — P2: treat exhausted redirects as failed health checks

- **Source/snippet:** `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:157-163`: `200..=399 => "healthy"` precedes failed-process handling. Error capture :148-154 retains curl failure; :113-119 retries HEAD with GET.
- **Trigger/consequence:** A loopback endpoint endlessly redirects. Real curl exits 47 after maximum redirects for both HEAD and GET but outputs HTTP302. The monitor reports one healthy URL while JSON also retains the curl error.
- **Safeguard/intro:** Retry, timeout and error capture exist. The final classifier overrides failed traversal with the residual status. The new monitor introduces the behavior. Its informational success exit is intentional and is not a CI-gate bypass.
- **Evidence/source reviewers:** AUD-2, independent Completeness Auditor; Phase 10 item 12 and Phase 11. VERIFIED / EXISTING_DEFECT, High confidence. Judge re-read actual classifier and failure capture.
- **Fix/regression/effort:** Prioritize relevant process/transport failure over residual successful/redirect codes, at least redirect exhaustion; report needs-review/transient rather than healthy. Add failed-302 classifier coverage and a real local redirect-loop regression while retaining informational command semantics. Small.

## Separate advisories

### A10 — P3 advisory: define historical versionless taxonomy policy

- **Source/snippet:** `schemas/taxonomy/owasp.yaml:471-473` now associates LLM05 with Data and Model Poisoning; base associated it with Improper Output Handling. `crates/tachi-core/src/threats_sarif.rs:170-179` formats primary LLM references with `:2026`. `schemas/finding.yaml:254-289` provides no edition field and requires current-catalog resolution.
- **Trigger/consequence:** Regenerating versionless historical attribution can reinterpret an ID under the current catalog. A mandatory historical-regeneration compatibility contract is unestablished; external OWASP publication accuracy is not adjudicated.
- **Source/confidence/class:** CQ-1, Code Quality; Phase 13 point 3, VR_PARTIAL; Medium obligation confidence; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** Document current-catalog resolution and the caller's legacy review obligation, or explicitly support edition-aware preservation and add historical fixtures. Do not silently guess ID remaps. Small for documentation; Medium or greater for chosen edition support.

### A11 — P3 advisory: state the same-source executable precondition

- **Source/snippet:** `crates/tachi-core/src/catalog_drift.rs:242` captures supplied-root hashes, :278 calls compiled `crate::try_build_report_data_typst`, and :319-329 rechecks/publishes root hashes. CLI root override: `crates/tachi-cli/src/bin/catalog-drift.rs:10-12,21-24`. Documented same-checkout Cargo command: `docs/feature-roadmap-2026-10-04.md:91`.
- **Trigger/consequence:** A compatible binary A used against source root B can theoretically attest B's disk inputs while building report data with A's compiled logic. No stale-binary mismatch was executed or current PDF corruption shown. Same-checkout Cargo invocation limits the ordinary trigger; input stability and rollback do not authenticate executable identity.
- **Source/confidence/class:** PIPE-2, Pipeline; Phase 13 point 4, VR_PARTIAL; Medium confidence in broader consequence/obligation; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** Document that the builder must come from the selected source revision and use the prescribed invocation. Only if arbitrary cross-revision generation is supported, define a mismatch check and targeted regression. Small clarification; larger attestation work needs an explicit requirement.

### A12 — P3 advisory: distinguish baseline checks from companion checks

- **Source/snippet:** `crates/tachi-core/src/catalog_drift.rs:78-86` excludes companion PDFs from rendering inputs, :183-202 checks registered baseline hashes, and :313-317 publishes companions when present.
- **Trigger/consequence:** Altering a published companion in a private fixture does not fail `--check` when registered baselines remain valid. The fixture behavior was reproduced; current committed PDFs are not claimed corrupt. A promise to verify every published copy is unestablished.
- **Source/confidence/class:** DA-2, DA; Phase 10 item 10; High scope confidence, Medium obligation confidence; PLAN_RISK / contract clarification.
- **Recommendation/regression/effort:** State checker scope clearly. If companion integrity becomes a supported guarantee, register/hash those outputs and test companion-only corruption. Small clarification; Medium coverage extension.

## Audit, safeguard and coverage assessment

The audit's two findings survive source reinspection: duplicate map insertion precedes any consumer and can erase positive summary-only evidence, while monitor classification accepts 302 before considering curl failure. Neither is a restatement of panel tier selection, and both include base/introduction evidence. Keep both as full defects, not lower-confidence appendices merely because fewer reviewers raised them.

The judge re-read the report selection/completion and image resolver, cleanup equality/unlink, nested attribution type/deserializer/schema, attack-tree ID branches, MAESTRO reduction/reconciliation, monitor classifier and test guard, plus the attack-path consumer. This confirms that claimed missing guards are missing at the relevant boundaries. It does not erase real safeguards elsewhere. The focused gap scan examined the adjacent selection/reconciliation/retry/consumer paths and produced no additional defensible finding. It is not an exhaustive new independent review.

Coverage remains uneven. The panel and supplemental audit touched CI permissions, workflow triggers, frontend authentication and Prisma/RLS contracts, but did not execute browser/session flows, live database isolation, Windows compilation, crash durability or concurrent catalog publication. Generated/binary assets and large registry/lockfile surfaces were excluded from the shared text bundle and only selectively inspected. No production agent generation, real-user prevalence, operational decision impact or external taxonomy publication correctness was verified. Root/shell containment elsewhere does not automatically guard the direct report path; conversely an unused Prisma client does not establish a new tenant-exposure endpoint. Preexisting middleware cookie behavior is excluded from this interval. These remain limitations, not manufactured defects.

## Evidence and process limits

Inputs consumed: context/personas; both Phase 6 summaries; all six Phase 7 finals; Phase 8/9/10/11; Phase 12a/12b; all four Phase 13 point artifacts and summary; Phase 13.5 gate JSON; selected Phase 3 score/source material and pinned implementation. Earlier claims are superseded only where later evidence actually resolves them. In particular, Phase 13 resolves the empty-image attack-tree compile consequence; it does not resolve path disclosure or Windows execution.

Phase 9 performed one allowed read-only source check, CMD_CONFIRMED, and did not replay out-of-allowlist runtime commands. Runtime claims here are attributed to their documented reviewer/auditor/verifier probes, not to fresh judge execution. The available graph was documented stale for this pinned worktree; exact known source paths and current numbered citations govern. No external-domain assertion drives severity, so no web-domain validation is needed. A wrapper invocation failed because lean-ctx was absent; RTK was used after the local RTK instructions were read. No memory-derived fact supplies a current behavior or priority claim in this ruling.

This is a **full single-run protocol through Phase 14**, with all 33 mandatory pre-judge artifacts passing; **no compressed-run banner is warranted**. Phase 14.5 and sequential Phase 15 outputs remain for the orchestrator. Opus was unavailable, so reviewers used the inherited available model consistently; only three child slots permitted two isolated batches for six independent reviewers. These disclosed runtime adaptations do not constitute skipped phases, but shared-model/trace bias remains. No product fix, tracker mutation, release authorization or deployment is implied by this review.
