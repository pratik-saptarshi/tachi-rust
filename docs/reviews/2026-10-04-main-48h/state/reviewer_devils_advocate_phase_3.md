# Phase 3 — Devil’s Advocate

Persona: Devil’s Advocate, 20% agreement; reasoning strategy: analogical failure-pattern reasoning. Score: **6/10**. Recommendation: **request changes** for DA-1; close DA-2 before treating the catalog gate as protection for published examples.

Scope: pinned `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112`. Read context, Phase 2 trace, codemap, pertinent diff, core report/control code, catalog checker/publication, workflow and tests. Graph was consulted first but its older symbols were not authoritative. No other reviewer artifacts read. Product source untouched; temporary reproduction fixtures only. Fresh offline build of report-data and catalog-drift passed using this pinned worktree and `/private/tmp/tachi-overseer-target`.

## Findings

## DA-1 — [P1] / BUG / false-negative report from a partial controls artifact

**Location:** `crates/tachi-core/src/report_data.rs:289` (premature acceptance); `crates/tachi-core/src/report_data.rs:177` (replacement); `crates/tachi-core/src/report_data.rs:276` (zero total). Supporting parser: `crates/tachi-core/src/compensating_controls.rs:257` and `:294`.

**Trigger:** a valid High scored finding `S-1`, accompanied by a partially generated or manually assembled compensating-controls.md containing only:

```markdown
## 3. Control Details

### Authentication

**Status**: Missing | **Effectiveness**: None
```

There is no residual assessment table, numeric coverage summary, or assessment completion evidence. The old parser still produces one control descriptor. The newly introduced guard accepts any nonempty `controls`; Tier 1 then replaces the valid risk finding with the empty residual vector. The report emits High=0, total=0, findings=(), controls=true, tier=1. Missing authentication becomes evidence for an apparently empty risk assessment.

**Observed verification:** freshly built report-data reproduced exactly `#let high-count = 0`, `#let total-findings = 0`, `#let data-source-tier = 1`, `#let has-compensating-controls = true`, `#let findings = ()`; exit success. Fixture retained at `/private/tmp/tachi-da-partial.0QxF1C`.

**Analogy:** accepting a partially filled reconciliation ledger as a completed balance statement. Evidence that a control was described does not establish that all residual findings were assessed.

**Guard search:** read the complete `has_control_assessment` function and parser, plus `completed_empty_controls_preserve_the_assessment_and_control_metadata` and `control_stubs_do_not_erase_valid_risk_findings_but_empty_assessments_are_retained`. The explicit residual-header and complete-numeric-summary guards are real but bypassed at line 289. The legitimate completed-empty test contains those additional signals; this reproduction contains neither. Existing stub tests never include a parseable control descriptor. No claim that every empty residual assessment is erroneous.

**Introduction:** base-to-head diff adds `render_document_data` and this acceptance/replacement path; the compensating-controls parser itself predates the window. The defect is the new composition, not the old parser.

**Confidence:** High; runtime reproduced, exact branch traced. P1 because a success-path security report silently removes known High findings for a normal incomplete-artifact case.

**verification_command (read-only replay):**

```sh
rtk proxy /private/tmp/tachi-overseer-target/debug/report-data --target-dir /private/tmp/tachi-da-partial.0QxF1C --template-dir templates/tachi/security-report
```

**Fix/regression:** distinguish control inventory from completed residual assessment. Without residual findings or explicit completed-empty signals, retain Tier 2/3 or reject incomplete authoritative input. Add a regression using valid S-1 High risk data and the controls-only fragment above; require the finding and High=1 to survive, or checked failure. Retain completed-empty assessment tests.

## DA-2 — [P2] / BUG / published PDF corruption passes catalog drift

**Location:** `crates/tachi-core/src/catalog_drift.rs:81` excludes published PDFs; `:193` checks only baseline hashes. Publication at `:313`–`:317` updates both copies.

**Trigger/consequence:** corrupt `examples/agentic-app/sample-report/security-report.pdf` while preserving its `.baseline` sibling. The new gate still passes because the reader-facing companion is neither an input nor a registered checked output.

**Observed verification:** copied required inputs into `/private/tmp/tachi-da-catalog.5jEfA2`. Initial catalog-drift check passed; after replacing only the sample-report published PDF with plain text `corrupt published PDF`, the second check also passed. The private probe completed successfully. Current committed maestro-reference and sample-report companions match their baselines; this finding concerns a reproduced gate false-negative, not currently corrupt committed PDFs.

**Guard search:** read rendering_inputs, check, regenerate, workflow and catalog tests. Regeneration updates companions, but check only validates baseline paths. The render-failure preservation test covers a different invariant. Its fixture has different companion and baseline bytes while expecting check success.

**Introduction:** catalog-drift module and workflow are new since base dd3b293d. Published companion maintenance is part of the new contract but only baseline copies receive hash verification.

**Confidence:** High; runtime reproduced. P2 because independent companion modification is required and committed copies currently agree. Analogy: checking a backup checksum while serving an unchecked live copy.

**verification_command (read-only replay):**

```sh
rtk proxy /private/tmp/tachi-overseer-target/debug/catalog-drift --check --root /private/tmp/tachi-da-catalog.5jEfA2
```

**Fix/regression:** register intentionally published companion paths and verify their bytes/hash against baselines, including missing companions. Keep generated-output exclusion from input hashing. Add corrupt-companion and missing-companion negative tests.

## Recommendation and coverage limits

Score **6/10**; **request changes**, prioritizing DA-1. No P0 evidence. Private staging, removal of copied baselines before rendering, version and PDF signature checks, completion of all renders before publication, and final input comparisons are real safeguards. Attribution errors also fail closed.

Further catalog crash/concurrency/provenance investigation is **unverified**: sequential in-place writes and best-effort rollback at catalog_drift.rs:351–:361 warrant fault injection before describing publication as crash-atomic. Source hashes describe --root while execution uses compiled code; stale-binary provenance was not reproduced and is not an additional finding. The companion-corruption probe above did complete; remaining investigation did not.

No full workspace test suite, Typst compilation, fault injection, concurrency stress, external advisory research or live product evaluation. Golden equality establishes consistency with regenerated baselines, not independently correct semantics: a faulty builder can regenerate consistent false-negative reports. DA-1 provides independent behavioral evidence. Stop at Phase 3.
