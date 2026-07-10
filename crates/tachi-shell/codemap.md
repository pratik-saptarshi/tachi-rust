# crates/tachi-shell/

## Responsibility

Owns the shared application-command layer between the domain core and external
adapters. It unifies control-plane script commands, Rust-native analysis/report
use cases, desktop dispatch, cancellation/progress reporting, filesystem policy,
and bounded child-process execution.

## Design Patterns

- **Facade:** public modules present one stable command surface to CLI, desktop,
  and MCP consumers.
- **Command registry:** immutable specifications bind command names to dispatch and
  output kinds, providing adapter parity and uniqueness validation.
- **Strategy/dependency injection:** progress reporters, script executors, and
  output sinks are traits so orchestration can be exercised without real process IO.
- **Use-case services:** Rust-native report, infographic, coverage, and SARIF
  functions wrap the stable `tachi-core::facade` exports.

## Data & Control Flow

External adapters select a canonical command and call the shell facade. Native
analysis commands validate arguments and contained paths, invoke a core use case,
and return or write the generated payload. Control-plane commands resolve scripts
inside the repository, spawn them with timeout/output caps and progress events,
then normalize exit, cancellation, timeout, stdout, and stderr into `CommandOutput`.

## Integration Points

- Depends on `tachi-core` for all domain transformations.
- Consumed by `tachi-cli`, `tachi-mcp`, and `tachi-desktop`.
- Executes repository-owned control-plane scripts for install/init/update/bootstrap.
- Source details: [`src/codemap.md`](src/codemap.md).
