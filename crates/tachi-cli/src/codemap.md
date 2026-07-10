# crates/tachi-cli/src/

## Responsibility

Holds the CLI crate's executable source tree. It is a container for binary entry points;
there is deliberately no `lib.rs` or shared application layer here.

## Design Patterns

- **Executable-per-Use-Case:** Cargo auto-discovers each file in `bin/` as a named binary.
- **Delegation:** cross-command behavior remains centralized in `tachi-shell`, preventing
  the CLI transport layer from becoming a second command implementation.
- **Process Boundary Adapter:** source translates between Rust command results and operating
  system concerns such as stdout, stderr, exit status, current directory, and file creation.

## Data & Control Flow

Cargo enters the selected `bin::<command>::main`; the entry point validates raw arguments,
delegates typed values to `tachi_shell::commands`, then renders the result to the process or
an explicitly requested output path.

## Integration Points

- Parent package contract: [`../codemap.md`](../codemap.md).
- Executable details: [`bin/codemap.md`](bin/codemap.md).
- Runtime API provider: `tachi-shell::commands`.
