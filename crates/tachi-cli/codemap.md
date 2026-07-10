# crates/tachi-cli/

## Responsibility

Defines the workspace's native command-line application package. The crate exposes one
small executable per supported operation and keeps business logic in `tachi-shell`.

## Design Patterns

- **Ports and Adapters:** binaries adapt process arguments, standard streams, exit codes,
  and filesystem output to the reusable command functions in `tachi-shell`.
- **Thin Command Wrapper:** the package contains no shared library target; each file under
  `src/bin/` is an independently compiled entry point.
- **Workspace Boundary:** `Cargo.toml` intentionally depends only on `tachi-shell` at
  runtime, with `serde_json` reserved for integration-test assertions.

## Data & Control Flow

1. Cargo selects a binary from `src/bin/<command>.rs`.
2. The binary parses `std::env::args`, applying command-specific required flags and a
   conventional exit status of `2` for invalid invocation.
3. Parsed paths and pass-through arguments enter the corresponding `tachi_shell::commands`
   function.
4. The adapter prints or persists the returned payload, maps command failures to status
   `1`, and preserves control-plane status codes where supplied.

## Integration Points

- Depends on [`../tachi-shell`](../tachi-shell/codemap.md) for command implementations,
  output models, SARIF generation, report data, and lifecycle operations.
- Provides the CLI side of the command registry also consumed by `tachi-desktop`.
- Integration tests under `tests/` exercise executable contracts but are outside this map's
  source inventory.
- Detailed maps: [`src/`](src/codemap.md) and [`src/bin/`](src/bin/codemap.md).
