# crates/tachi-shell/src/

## Responsibility

Implements reusable command use cases and adapter-neutral dispatch for shell,
desktop, CLI, and MCP entrypoints.

## Design Patterns

- `command_use_cases.rs` adapts `tachi_core::facade` domain outputs into strings
  and typed SARIF/report result structures.
- `commands.rs` defines the canonical registry, normalized `CommandOutput`,
  control-plane script resolution, and install/init/update/bootstrap wrappers.
- `progress.rs` provides observer-style `ProgressReporter` callbacks and a shared,
  atomically backed `CancellationToken` around command execution.
- `tauri_bridge.rs` is a transport boundary that parses command arguments,
  dispatches by registry kind, and enforces root-contained input/output paths.
- `commands/` isolates the process-execution strategy and output-finalization
  helpers from command metadata.

## Data & Control Flow

`dispatch_command[_with_progress]` looks up `CommandDispatchKind`, then routes to
either a repository script or a Rust-native handler. Native handlers parse flags,
canonicalize inputs, reject traversal/escape/symlink output paths, call a use-case
function, and optionally persist its payload. Script handlers wrap spawning with
progress, cancellation, timeout, process-group termination, and bounded capture.

## Integration Points

- Imports stable domain APIs from `tachi_core::facade`.
- Exports command registry and output helpers used by MCP contract/tool dispatch.
- Exports bridge dispatch and progress primitives used by the desktop host.
- Detailed execution map: [`commands/codemap.md`](commands/codemap.md).
