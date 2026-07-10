# crates/tachi-core/src/parsers/

## Responsibility

Provides the shared parsing and validation layer for Tachi's Markdown and
Mermaid report formats. It converts loosely structured report sections into
typed findings, severity totals, project scope, data-flow topology, source
attribution, and asset metadata.

## Design Patterns

- **Layered parser:** `table.rs` supplies a generic section-scoped Markdown
  table reader; domain parsers build typed records on its normalized maps.
- **Parser combinators by convention:** small functions isolate headings,
  rows, tags, and sentinel normalization before aggregate construction.
- **Validate after parse:** `findings.rs` parses source-attribution records and
  separately validates taxonomy IDs against repository catalogs.
- **Canonicalization at boundary:** casing, bold markers, empty sentinels, and
  project-name fallbacks are normalized before values reach domain builders.

## Data & Control Flow

1. `table::parse_markdown_table` finds a named section, identifies headers, and
   returns ordered row maps.
2. `findings.rs` uses those rows to produce `ThreatFinding`, risk, resolved,
   severity, delta, pattern, and source-attribution records.
3. `scope.rs` extracts components, flows, trust boundaries, and crossings into
   `ScopeData` for architecture and SARIF builders.
4. `mermaid.rs` scans selected diagram blocks, merges component asset tags, and
   cleans display labels.
5. `mod.rs` re-exports the parser surface and provides Typst escaping and
   project-title resolution helpers.

## Integration Points

- Consumed by infographic, attack-tree/chain, coverage, metadata, report
  extraction, risk-score, and SARIF modules.
- Reads taxonomy catalogs only when `validate_source_attribution` is requested;
  ordinary parsing remains deterministic and filesystem-free.
- Public parser types/functions are re-exported through `parsers::mod.rs`, with
  `parse_threats_findings` promoted through the crate facade.
