# crates/tachi-core/src/infographic/

## Responsibility

Builds template-specific infographic data beneath the coordinator in
`../infographic.rs`. It packages parsed threat-report data, architecture scope,
MAESTRO aggregation, and prompt guidance into stable JSON payloads for visual
renderers.

## Design Patterns

- **Builder pipeline:** `payload.rs` selects a template and composes shared and
  template-specific data into `InfographicPayload`.
- **Strategy by template:** executive architecture and MAESTRO stack/heatmap
  builders produce distinct JSON schemas behind one payload entry point.
- **Repository port:** `PromptScaffoldStore` abstracts template loading;
  `FilesystemPromptScaffoldStore` is the production adapter.
- **Bounded allocation:** executive callouts are severity-ranked and capped to
  keep generated diagrams readable and deterministic.

## Data & Control Flow

1. `build_infographic_payload` detects report artifacts, reads `threats.md`,
   derives the report tier/name, and optionally loads prompt scaffolding.
2. `build_infographic_payload_from_content` parses findings and severity,
   computes top findings, heat maps, and MAESTRO summaries, then dispatches on
   the requested template.
3. `executive_architecture.rs` parses scope components, trust boundaries, data
   flows, and clusters and associates prioritized finding callouts with layers.
4. `maestro_templates.rs` converts layer distributions and heat-map cells into
   normalized JSON values; `prompt_scaffold.rs` extracts reusable prompt
   segments from template Markdown.

## Integration Points

- Parent coordinator/types: `../infographic.rs`.
- Inputs: `crate::parsers`, `crate::metadata`, `crate::artifacts`, and MAESTRO
  canonicalization from `crate::coverage_taxonomy`.
- Output: `build_infographic_payload` is re-exported through `crate::facade` for
  shell/desktop hosts and downstream renderers.
