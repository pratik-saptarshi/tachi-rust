# crates/

## Responsibility

Contains the Rust workspace's deployable and reusable packages. The crates divide
domain transformations, command-line adapters, shared command execution, MCP
transport, and the active native desktop host into explicit dependency layers.

## Design Patterns

- **Layered architecture:** `tachi-core` owns domain behavior; `tachi-shell`
  coordinates use cases and process-backed control-plane commands; `tachi-cli`,
  `tachi-mcp`, and `tachi-desktop` adapt that behavior to external transports.
- **Ports and adapters:** binaries and desktop/MCP boundaries depend on stable
  Rust facades rather than reimplementing parsing, reporting, or SARIF logic.
- **Shared registry contracts:** shell command metadata is the canonical source
  used to keep CLI, desktop, and MCP command/output semantics aligned.

## Data & Control Flow

1. A CLI invocation, MCP request, or desktop action enters an adapter crate.
2. The adapter validates transport-specific arguments and delegates to
   `tachi-shell` or directly to the stable `tachi-core::facade` surface.
3. `tachi-core` reads threat-model artifacts and produces report, infographic,
   coverage, or SARIF data; shell-owned control-plane commands may execute
   repository scripts with bounded runtime and output.
4. The adapter serializes the result to stdout, an artifact, or native host state.

## Integration Points

| Crate | Role | Detailed map |
|---|---|---|
| `tachi-core` | Domain parsers and report/coverage/SARIF transformations. | [`tachi-core/codemap.md`](tachi-core/codemap.md) |
| `tachi-cli` | Thin command-line binaries. | [`tachi-cli/codemap.md`](tachi-cli/codemap.md) |
| `tachi-shell` | Shared command facade, dispatch, progress, and execution policy. | [`tachi-shell/codemap.md`](tachi-shell/codemap.md) |
| `tachi-mcp` | Standalone stdio MCP transport and tool contract. | [`tachi-mcp/codemap.md`](tachi-mcp/codemap.md) |
| `tachi-desktop` | Active GTK-free native desktop host. | [`tachi-desktop/codemap.md`](tachi-desktop/codemap.md) |
