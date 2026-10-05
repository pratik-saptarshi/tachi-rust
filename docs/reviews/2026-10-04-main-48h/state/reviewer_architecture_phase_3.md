# Phase 3 — Architecture Critic

Agreement intensity: 50%. Reasoning: backward from trustworthy report output, consistent adapter results, and complete baseline publication. Review mode: Precise for implementation/configuration. Score: **7/10**. Recommendation: **Request changes for report input validation; retain the architectural direction.**

Reviewed resulting revision `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112`, covering the seven-commit window. No other independent reviewer output was read. No product files were modified. Findings below are source-traced defects, not claims of runtime reproduction or a passing test suite.

## Desired outcomes and backward trace

The final findings cards, counts, component distribution, and remediation must all describe a valid selected assessment. Tracing backward from `main.typ:405-409` reaches one selection point in `report_data.rs:162-181`. Deriving all downstream summaries from the selected rows is a good improvement, but that selection point needs a validity contract: a nonempty parser result is not evidence of a valid assessment.

A failed image render must preserve the useful attack-path analysis. Tracing backward from `attack-path.typ:72-80` reaches a second asset-selection policy, implemented independently from `assets.rs::choose_image`. It treats mere file existence as successful image availability and therefore bypasses the new Mermaid fallback.

Checked errors are propagated consistently through the CLI, shell bridge, and MCP path. Catalog regeneration stages inputs and outputs, rejects renderer failures, checks source drift before publication, snapshots destination bytes, and attempts rollback including the failing destination. These safeguards are real; I am not claiming render failure corrupts existing baselines or that publication promises crash-safe atomicity.

## Findings

### ARC-1 — P1 — A malformed optional risk table replaces valid threat findings with blank rows

**Labels:** [CODE-TRACE] [DEFINITE-DEFECT] [PRECISE]. Confidence: **High** for the data-loss mechanism; severity assumes reports are relied on for security prioritization. No runtime reproduction performed.

**Exact source:** `crates/tachi-core/src/report_data.rs:164-168` selects Tier 2 and replaces the full findings vector whenever parsed risk rows are nonempty. `crates/tachi-core/src/parsers/findings.rs:164-178` constructs a `RiskScoreFinding` from every table row, defaulting missing ID, severity, component, and score columns to empty strings. `crates/tachi-core/src/report_data.rs:268-276` subsequently recomputes counts from those replacement rows. The consumer at `templates/tachi/security-report/main.typ:405-409` receives those rows directly; `findings-detail.typ:182-213` does not reject invalid rows.

**Concrete trigger:** A valid `threats.md` containing a High `S-1` recommendation, alongside this syntactically readable `risk-scores.md`:

```markdown
## 2. Scored Threat Table

| Unexpected |
| --- |
| generator failure |
```

`parse_markdown_table` produces a nonempty map, the risk parser produces one all-empty finding, and `render_document_data` emits `data-source-tier = 2`, `has-risk-scores = true`, one blank finding, and zero Critical/High/Medium/Low counts. The valid `S-1` disappears from the detail table and component distribution. The checked command still succeeds. Attribution may still list the raw finding, so this can also create an internally inconsistent report rather than an obvious failure.

**Base comparison:** The permissive risk parser predates the window. The defect is its new composition into the canonical report builder: base `report_data.rs` never reads `risk-scores.md` or replaces the findings vector, whereas the reviewed revision adds `render_document_data` and this selection branch. This is not a claim that the baseline already emitted a complete, correct findings document.

**Safeguards considered:** Primary threat parsing is checked before generation; that does not validate the optional risk file. Controls have a dedicated `has_control_assessment` guard and control-stub regressions in `report_document_contract.rs`; neither applies to the risk branch. Final count recomputation is internally correct for the wrong rows and therefore cannot detect the loss. I searched the report contract tests and relevant parser/renderer paths for malformed risk-table validation and found no guard on this path.

**Read-only verification_command:** `rtk proxy sh -c 'sed -n "158,179p" crates/tachi-core/src/parsers/findings.rs; sed -n "161,181p" crates/tachi-core/src/report_data.rs; sed -n "261,276p" crates/tachi-core/src/report_data.rs'`

**Fix and regression:** Validate the required header/row contract before selecting a tier, ideally returning a typed validated assessment and distinguishing absent, incomplete, and invalid inputs. The checked API should reject malformed existing risk data with its filename, or deliberately retain Tier 3 with a visible diagnostic. Add a regression for the fixture above asserting that a successful result never drops `S-1` or reports zero High findings. Also test partially populated rows and invalid severities; checking only nonempty IDs is insufficient.

### ARC-2 — P2 — An empty attack-tree image disables the text fallback and breaks PDF generation

**Labels:** [CODE-TRACE] [DEFINITE-DEFECT] [PRECISE]. Confidence: **High** for the producer/consumer mismatch. The Typst compilation failure is inferred from feeding an empty image file to the explicit `image` call; no compilation was run in this phase.

**Exact source:** `crates/tachi-core/src/report_data.rs:227-239` selects the first `png`, `jpg`, or `svg` path satisfying only `is_file()`. Lines `243-247` set `has-image` from that selection. `templates/tachi/security-report/attack-path.typ:69-80` calls `image()` whenever that flag is true and only renders the Mermaid source in the other branch. The same lookup is reused for chains at `report_data.rs:255-259`.

**Concrete trigger:** A parsed High/Critical attack tree for `S-1`, with valid Mermaid source, plus an empty `attack-trees/S-1-attack-tree.png` left by a failed diagram command. That file wins selection even if a valid JPG exists. The generated binding sets `has-image=true`, making the PDF consumer attempt to decode empty bytes instead of using available text. Report-data generation itself returns success, but PDF rendering fails.

**Base comparison:** Base `report_data.rs` does not bind attack trees or chains. This window adds both the image-selection closure and canonical attack-tree output, activating this failure path. The template's image call existed previously, but the new producer does not establish its usable-image precondition.

**Safeguards considered:** `assets.rs:128-168` rejects empty candidates and uses image signatures for the top-level report images. That function is not used by this closure. The checked report-data API does not compile Typst or validate these bytes. The newly added Mermaid branch is a useful fallback but is unreachable for an existing unusable file.

**Read-only verification_command:** `rtk proxy sh -c 'sed -n "227,259p" crates/tachi-core/src/report_data.rs; sed -n "69,81p" templates/tachi/security-report/attack-path.typ; sed -n "128,168p" crates/tachi-core/src/assets.rs'`

**Fix and regression:** Centralize image usability/selection rather than maintain separate top-level and attack-diagram policies. Skip empty/invalid candidates and continue to other formats; preserve SVG support explicitly. Add an integration fixture with valid Mermaid source and empty PNG, asserting the generated report selects the fallback and compiles. Add empty-PNG-plus-valid-JPG coverage to prove selection continues instead of stopping at the first file.

## Architectural strengths and non-findings

- The checked report API is threaded through the affected CLI, shell bridge, and MCP paths. The legacy API deliberately emits an unrenderable panic document on error; I found no error swallowing in the newly updated adapter handoffs themselves.
- MAESTRO report coverage and grouped findings use the same extractor, and explicit evaluation state preserves the distinction between clean and not evaluated. I did not treat inherent-attribution counts versus residual-findings counts as automatically erroneous; those describe different projections by design.
- Catalog publication includes staging, renderer-version checking, complete-PDF checks, source fingerprint rechecks, and ordinary-error rollback. Crash recovery, interprocess serialization, and stale executable provenance remain broader limitations, not promoted here without a demonstrated in-scope failure.
- The Next.js middleware-to-proxy migration preserves the existing authentication and cookie logic; the Prisma migration adds own-profile RLS and explicit PostgreSQL checks. The existing privileged Prisma connection is not, on its own, proof of a new cross-tenant endpoint exposure. I found no concrete frontend migration regression in the inspected files.

## Coverage and limits

Read the shared context/data-flow trace and atlas; inspected relevant changed implementation and tests for report building, parser contracts, attack-tree rendering, source attribution/SARIF propagation, MAESTRO integration, catalog staging/publication, shell/MCP adapters, and Next.js/Supabase/Prisma migration. Graph discovery was attempted first; the shared context warns it describes an older source state, so pinned source is authoritative. The memory registry was used only for discovery/exclusion guidance (`MEMORY.md:58-67`), not as proof of current behavior.

This was not exhaustive review of every changed line, generated lockfile, taxonomy record, binary PDF, prompt, or CI workflow. I did not execute the full workspace tests, a browser workflow, PostgreSQL migration, or renderer. Read-only commands support the cited control flow; proposed reproductions and regressions remain to be executed by a later verification phase. No temporal range filtering was found in the inspected runtime paths; the review's 48-hour window is a commit-selection criterion, not application date filtering.
