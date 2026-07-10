# crates/tachi-mcp/

## Responsibility

Provides the standalone MCP-facing executable and library. It exposes a fixed set
of analysis tools over newline-delimited JSON on stdio while preserving the
canonical `tachi-shell` command contract, request identity, authorization, output
mode, and cancellation state.

## Design Patterns

- **Transport adapter:** `main.rs` is deliberately thin; protocol handling and
  dispatch live in the library.
- **Registry and policy objects:** static tool specifications are wrapped by a
  registry, while an allowlist policy gates invocation before dispatch.
- **Versioned contract snapshot:** command metadata is projected from the shell
  registry and SHA-256 hashed to make cross-adapter drift observable.
- **Command pattern:** typed tool IDs and input structures select shared shell use
  cases; results support in-band payloads or deterministic artifact paths.

## Data & Control Flow

1. `tachi-mcp --stdio` starts the stdio loop.
2. Each JSON line becomes a request context plus a tool name and typed payload.
3. `McpServer` checks cancellation and authorization, resolves the tool registry,
   deserializes its input, and calls the corresponding `tachi-shell` output helper.
4. The server returns content in-band or writes it beneath a tool-specific `mcp/`
   path, then the transport emits one correlated JSON response line.

## Integration Points

- Depends on `tachi-shell` for command metadata and all analysis/report outputs.
- Uses `serde`/`serde_json` for wire and snapshot models and `sha2` for contract
  fingerprints.
- Exposes the `tachi-mcp` binary for MCP clients and a library surface for
  contract, schema, policy, server, and transport consumers.
- Source details: [`src/codemap.md`](src/codemap.md).
