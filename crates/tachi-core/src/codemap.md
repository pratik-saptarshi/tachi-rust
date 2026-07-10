# crates/tachi-core/src/

## Responsibility

Implements Tachi's typed security-report domain model and deterministic
transformation pipeline. The directory converts project artifacts into
validated findings, coverage evidence, visual-report payloads, remediation
plans, and standards-compatible exports.

## Design Patterns

- **Public facade:** `facade.rs` curates adapter-safe operations; `lib.rs`
  controls public versus crate-private modules and re-exports.
- **Pipeline stages:** `parsers/` extracts records; normalization and taxonomy
  modules canonicalize them; report, infographic, and SARIF modules render them.
- **Catalog/registry:** `aisvs.rs` and `coverage_taxonomy.rs` expose validated,
  canonical control and taxonomy registries.
- **Strategy ports:** `TaxonomyStore` permits alternate taxonomy sources while
  keeping aggregation logic independent of the filesystem.
- **Shared envelope builder:** `sarif_common.rs` centralizes SARIF metadata,
  severity mapping, component lookup, and envelope construction.

## Data & Control Flow

1. `artifacts` and `assets` inspect a target tree and select report tier and
   image/brand bindings.
2. `parsers`, `metadata`, `report_extraction`, `attack_chains`,
   `compensating_controls`, and `risk_scores` turn Markdown into domain structs.
3. `normalization`, `coverage_taxonomy`, `coverage_attestation`, and `aisvs`
   validate/canonicalize evidence and compute framework coverage.
4. `infographic`, `report_data`, `threats_sarif`, `risk_scores`, and
   `attack_chains` emit JSON, Typst, SARIF, and Mermaid artifacts.
5. `fixtures` canonicalizes JSON and hashes payloads for parity contracts;
   `mmdc` reports external Mermaid-renderer preflight failures.

## Integration Points

- `facade.rs` is the primary boundary used by host crates.
- `parsers/` supplies common table, finding, scope, and Mermaid extraction to
  most report builders; see [parsers/codemap.md](parsers/codemap.md).
- `infographic.rs` coordinates specialized builders in `infographic/`; see
  [infographic/codemap.md](infographic/codemap.md).
- Taxonomy aggregation reads repository YAML through an injected store or a
  filesystem adapter; all final JSON uses ordered/canonicalized structures.
