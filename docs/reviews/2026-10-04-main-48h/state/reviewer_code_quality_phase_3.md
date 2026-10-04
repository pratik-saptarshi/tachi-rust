# Phase 3 — Code Quality Auditor

Persona: Code Quality Auditor; agreement intensity 40%; systematic enumeration. Mixed review, precise for executable contracts. Independent: no other reviewer outputs read. Source pinned at `8df554e884b1e5dd24146111a965597eff5f4779`; base `dd3b293d81d358d1ae27424be83b720693539112`.

**Score: 7/10. Recommendation: request changes for the two bounded compatibility defects below.** No P0/P1 claim is made. These are source-traced findings, not executed reproductions.

## Findings

## CQ-1 — [P2] [EXISTING_DEFECT] Historic attribution IDs silently acquire different category meanings

**Current locations:** `schemas/taxonomy/owasp.yaml:471-473` replaces LLM05 with Data and Model Poisoning; `crates/tachi-core/src/threats_sarif.rs:170-179` constructs a 2026 reference from every primary LLM ID without inspecting input provenance. Related paths: `crates/tachi-core/src/parsers/findings.rs:293-339`, `crates/tachi-core/src/report_data.rs:36-51`.

**Trigger:** Reprocess an existing 2025-taxonomy threats document with an explicit `{taxonomy: owasp, id: LLM05, relationship: primary}` output-handling citation. A concrete previously supported artifact is `git show dd3b293d:tests/scripts/fixtures/source_attribution/valid_multi_record.md`: schema 1.5, dated 2026-04-20, finding LLM-5 explicitly says unsanitized LLM output enters an HTML sink, and cites LLM05 primary.

**Consequence:** The same historic evidence still parses and passes catalog membership (LLM05 remains a valid ID), but it is counted against the new poisoning category and exported with the current LLM-05 interpretation. There is no diagnostic or migration decision. This changes the compliance meaning of old evidence while retaining finding identity. Editing the repository's current examples does not migrate users' saved reports.

**Guard search:** Read source-attribution parsing, `validate_source_attribution`, coverage projection/aggregation, checked report builder, SARIF attachment, schema, and the semantic-contract/backward-compatibility tests. Searches for `version|2025|legacy|migration` in these runtime paths found no taxonomy-version dispatch or historic-ID migration. Validation only checks ID membership, so the colliding ID is accepted. Current backward-compatibility tests compare regenerated current fixtures; the old fixture was itself updated. The schema-version bump to 1.9 adds assets but the parser does not consult input schema_version when resolving attribution. The roadmap explicitly says to preserve historical references; there is no exemption documented for saved report inputs.

**Base comparison:** At dd3b293d the exact catalog key LLM05 meant Improper Output Handling and the saved fixture was consistent. This window reuses the key for a different category and adds forced 2026 SARIF attachment. The concern is not the external accuracy of the 2026 list: it is a collision between two meanings demonstrably present in base/current source.

**Confidence:** High in the source trace; medium in priority because prevalence of saved pre-migration reports is not measured. Static verification only; no claim of runtime test execution.

**verification_command (read only):**

```sh
rtk git diff dd3b293d -- schemas/taxonomy/owasp.yaml crates/tachi-core/src/threats_sarif.rs
```

Additional concrete input retrieval: `rtk git show dd3b293d:tests/scripts/fixtures/source_attribution/valid_multi_record.md`.

**Fix/test:** Preserve an explicit taxonomy edition in inputs, or use a documented legacy-input migration/rejection policy. Do not infer category changes from arbitrary threat IDs or blindly remap new reports. Freeze the base fixture unchanged and assert regeneration either preserves 2025 category semantics, performs an explicit edition-aware migration to LLM10, or fails with an actionable ambiguity message. Test multiple colliding categories and ensure genuinely current LLM05 poisoning remains LLM05.

## CQ-2 — [P2] [EXISTING_DEFECT] New nested attribution parser rejects the documented default relationship

**Current locations:** `crates/tachi-core/src/parsers/findings.rs:493-498`; underlying record at `crates/tachi-core/src/parsers/findings.rs:63-67`. Contract: `schemas/finding.yaml:277-290`. Existing flat parser defaults at `crates/tachi-core/src/parsers/findings.rs:598-600`.

**Trigger:** A supported nested attribution block contains a valid record with its optional relationship omitted:

```yaml
OI-1:
  source_attribution:
    - {taxonomy: owasp, id: LLM10}
```

Place this in a numbered Source Attribution YAML section or the newly supported `**Source Attribution**:` block, alongside a normal recommendation row.

**Consequence:** `serde_yaml::from_str::<BTreeMap<String, Entry>>` deserializes directly into `SourceAttributionRecord`, whose `relationship: String` has no serde default. Deserialization therefore returns a missing-field error labeled malformed YAML. The checked report, infographic, and SARIF paths propagate this error and reject an otherwise schema-valid input. The same record in the flat representation succeeds and acquires `primary`, making acceptance depend on representation rather than finding semantics.

**Guard search:** Read both nested and flat extractors, record derives, schema default, checked wrappers, and new attribution tests. `Default` on the Rust struct does not instruct serde to default absent fields; there is no `#[serde(default = ...)]` or postparse normalization before the deserialize error. Relationship enum checks run only after deserialization, so they cannot repair it. The schema expressly says the parser injects primary when absent.

**Base comparison:** Base had the flat parser's primary default and did not support this nested form. This window introduces a nested counterpart that violates the established record contract. In numbered sections the previously ignored nested input now causes a hard error, so the change is externally observable; this is a new-path defect, not an allegation that base already supported nested attribution correctly.

**Confidence:** High, by direct Rust/serde source trace. No runtime reproduction executed.

**verification_command (read only):**

```sh
rtk proxy awk '(NR>=63 && NR<=68)||(NR>=490 && NR<=509)||(NR>=590 && NR<=601) {printf "%d:%s\n",NR,$0}' crates/tachi-core/src/parsers/findings.rs
```

**Fix/test:** Give the deserialized relationship the same explicit primary default as the flat parser, while preserving rejection of unsupported explicit values. Test flat and nested representations with relationship omitted, primary, related, derived, and an invalid value through both parsing and a checked output boundary.

## Enumeration, positives, and limits

Read the shared context, Phase 2 trace, root atlas, relevant changes.diff sections, current/base OWASP catalog contracts, changed active agent/adapter semantic examples, source-attribution parsers, report coverage entry point, threat/risk SARIF builders, shell integration, schemas, and focused semantic/roundtrip/backward-compatibility tests. Graph-first search was performed; its older source positions were used only for orientation, and pinned files supplied all citations.

The new threat SARIF attachment correctly matches finding fingerprints rather than the shared OI/LLM rule ID. Explicit empty attribution remains distinguishable from absent evidence. Checked output paths preserve malformed-input errors. Context-specific active-example tests and negative token mutations are useful coverage.

Investigated but not promoted: risk SARIF derives `owasp-reference` from OI/MI prefixes, so an OI vector-filter finding with primary LLM09 also has LLM10 in that field. The field's intended relationship semantics were insufficiently explicit to claim a new defect; LLM10 is a legitimate related citation in the new fixture, and the mechanism predates the window. Similarly, model-theft example category choices require external domain validation and were not asserted as errors.

This focused pass is not an exhaustive audit of the whole 48-hour diff. It does not independently verify catalog publication rollback, filesystem boundary security, frontend/Prisma behavior, compiled PDF output, external taxonomy publication accuracy, binary artifacts, complete CI, or live-agent behavior. No product edits or Python execution occurred. No other persona state was read. Phase 3 ends here.
