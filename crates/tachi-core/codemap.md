# crates/tachi-core/

## Responsibility

`tachi-core` is the host-independent domain library for Tachi's report-analysis
pipeline. It owns deterministic parsing, normalization, taxonomy, coverage,
infographic, remediation, and SARIF construction logic while leaving CLI,
desktop, MCP, and filesystem orchestration to adapter crates.

## Design Patterns

- **Facade:** `src/facade.rs` defines the stable cross-crate API; `src/lib.rs`
  re-exports that facade plus selected typed domain surfaces.
- **Functional core:** most transformations accept text or typed records and
  return deterministic structs, JSON, Typst, or Mermaid without host state.
- **Ports and adapters:** traits such as `TaxonomyStore` and
  `PromptScaffoldStore` isolate repository-backed data from pure builders.
- **Canonical data contracts:** Serde-compatible structs and ordered
  collections keep generated fixtures and reports reproducible.

## Data & Control Flow

1. A host reads workspace artifacts and invokes the public facade.
2. Parsers convert Markdown, Mermaid, and taxonomy content into typed findings,
   scope, severity, attribution, and risk records.
3. Domain modules normalize and enrich records with MAESTRO/OWASP coverage,
   attack paths, remediation actions, assets, and governance metadata.
4. Output builders serialize the enriched model as infographic JSON, SARIF,
   Typst bindings, Mermaid, or stable command fixtures.

## Integration Points

- Consumed by `tachi-shell`, `tachi-desktop`, and other host adapters through
  the facade rather than private implementation modules.
- Depends only on `serde`, `serde_json`, `sha2`, and `thiserror` at runtime,
  preserving the host-independent boundary.
- Detailed source map: [src/codemap.md](src/codemap.md).
