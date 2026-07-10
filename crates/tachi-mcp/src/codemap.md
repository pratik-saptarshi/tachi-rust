# crates/tachi-mcp/src/

## Responsibility

Implements the MCP contract model, tool catalog, authorization boundary, server
dispatcher, and stdio wire adapter for the standalone MCP process.

## Design Patterns

- `tools.rs` defines the typed tool enum, immutable registry, allowlist policy,
  request context, input DTOs, and invocation result DTO.
- `server.rs` is an application-service dispatcher with an injectable cleanup
  callback; it centralizes typed deserialization and in-band/artifact handling.
- `stdio.rs` is a line-oriented protocol adapter generic over `BufRead`/`Write`,
  keeping transport behavior testable without process IO.
- `lib.rs` derives versioned command and schema snapshots from canonical registries
  rather than maintaining a second command inventory.
- `main.rs` is the composition root and converts library errors to process status.

## Data & Control Flow

`main` -> `stdio::run` -> `stdio::serve` -> `McpServer::invoke_json` -> policy and
tool resolution -> `McpServer::invoke_tool` -> a `tachi_shell::commands::*_output`
use case -> `write_or_return` -> `StdioWireResponse`. Cancelled requests run the
optional cleanup hook and stop before tool dispatch. Artifact mode creates parent
directories, writes deterministic output, and reports path and byte count.

## Integration Points

- `tachi_shell::commands::{command_registry, CommandDispatchKind,
  CommandOutputKind}` supplies contract metadata.
- Shell output helpers supply coverage audit, infographic, report-data, and both
  SARIF payloads.
- JSON aliases `id` to `request_id`, preserving compatibility at the wire edge.
- Tool/schema snapshots are public library APIs for drift checks and clients.
