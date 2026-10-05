# Pipeline Reviewer — Phase 5 Debate Round 1

Read the six Phase 3 reviews and private reflections; no Round 1 response was read. Rechecked current report tier selection and the explicit relationship default in the schema. No new runtime probe or product edit.

## Responses

### PIPE-2: challenge the supported invocation and required provenance contract

I challenge my initial [P2] mandatory-defect framing. The mechanism is real: an arbitrary root can be hashed while a differently built Rust implementation renders it. But `docs/feature-roadmap-2026-10-04.md:91` explicitly prescribes `cargo run --locked` in the checkout followed by the PDF compatibility test. This is affirmative evidence of a procedure that avoids the ordinary stale executable condition, not merely an absent guard. Exposing `--root` supports choosing input roots; it does not unambiguously promise that a binary built from any revision will certify a different revision. I now recommend **advisory/P3**, unless a supported cross-checkout regeneration use case or reproduction establishes an in-scope contract violation. Do not count it as a confirmed blocking defect or imply the committed manifest is wrong. A clear same-revision precondition may be a proportionate first remedy.

### DA-2: companion check scope

The observed corrupted-companion pass is convincing, and protecting reader-facing PDFs would be useful. However, the checker expressly iterates `manifest.baselines`; generated companion exclusion is deliberate. The existing catalog fixture even assigns companions different bytes and expects check success, as DA notes. This is positive evidence of a baseline-only check contract. Publishing both files in one operation does not logically entail policing all later modifications to both. Recommend **advisory/P3** unless an explicit acceptance criterion demands companion integrity from `--check`. Preserve the useful proposal to add registered companion outputs, but describe it as extending gate scope. Current companions are confirmed to match and there is no evidence of current corrupt artifacts.

### ARC-1 / COR-1 / COR-2 / DA-1: defect agreement, priority challenge

I agree these are real introduced composition defects. The successful empty/blank report output is stronger than a mere parser strictness concern. DA-1 is particularly clear: a descriptor saying Authentication is Missing is not completion evidence for zero residual findings, and the broad nonempty-controls shortcut at report_data.rs:289 bypasses the later completion signals. Retain it separately from the malformed scored-table branch because fixing one does not fix the other.

I recommend **P2 for all three ingress cases**, with High behavioral confidence. The demonstrated triggers are incomplete or noncanonical files; no ordinary completed canonical artifact or production generation path has been shown to produce them. Security prioritization makes the consequence important but does not by itself establish P1 exposure. A P1 recommendation needs an ordinary interruption/resume path that subsequently auto-renders these partial inputs, or a canonical completed example that loses findings. Regardless of priority, fixes should reject or preserve an earlier tier with a visible diagnostic, while retaining intentionally completed-empty assessments. Do not treat any empty residual vector as invalid.

### PIPE-1: Windows compilation

Retain **P2, High confidence** for the narrow new test-target compile error. Guarding the Unix helper and test is a small, conventional correction; existing CLI tests demonstrate that pattern. This is not evidence of Windows runtime readiness or an already-required Windows CI matrix. A cross-target `cargo check --tests` on a provisioned target is useful verification; absent installed target support is an environment limit, not a reason to claim the check passed or to inflate severity. Portable extraction/classification tests should remain enabled outside Unix.

### CQ-2: relationship default

Agree with **P2, High confidence**. Current schema explicitly says the parser injects primary for omitted relationships (`schemas/finding.yaml:283-288`), while the nested deserializer requires the String before validation. Unlike PIPE-2 and DA-2, this is a direct contradiction of a literal input contract and has a bounded fix. Add explicit default behavior for the nested path and test parity with the flat representation while retaining invalid-value rejection. A runtime fixture would strengthen the label, but missing required serde fields already establish the causal mechanism.

### Other reviewers

ARC-2/COR-3 and SEC-1 have concrete guard/producer mismatches deserving bounded P2 fixes. SEC-2 should remain restricted to wrong neighboring image selection and conditional disclosure within the configured Typst root. CQ-1 is a real semantic migration risk, but I agree with its reflection that historic-output support requires policy adjudication; do not infer a serialized 2026 year that normalization removes.

## Position Changes (new evidence)

- PIPE-2: provisional P2 becomes advisory/P3. The decisive contract evidence is the documented same-checkout Cargo regeneration plus PDF comparison procedure, reinforced by the distinction between input-root selection and build-provenance guarantees.
- DA-2: recommend advisory/P3, based on the explicit baseline-only checker and intentional fixture acceptance of unequal companions. Its reproduction remains valid but proves an uncovered scope rather than a failed promised check.
- Support CQ-2, tier-validation and image/cleanup findings as bounded corrections. Prefer P2 over P1 for partial-input tier loss pending ordinary-path exposure evidence.
- PIPE-1 remains unchanged. These judgments do not change or erase the independent Phase 3 record.

## Unresolved Points

1. Whether the project requires Windows CLI test compilation, and whether the panel wants a narrow portability fix or a new ongoing CI lane.
2. Whether cross-revision `--root` regeneration is intended; a documented precondition would settle much of PIPE-2 without a build attestation subsystem.
3. Whether published companion integrity belongs in the mandatory catalog gate scope.
4. Whether completed canonical generator output or an automatic interruption/resume workflow reaches the tier-loss cases often enough to support P1.

## New Discoveries

No additional defect. New information came from other reviewers’ bounded reproductions and contract evidence, not broader probing. No full Typst, Windows, database or browser validation is claimed.
