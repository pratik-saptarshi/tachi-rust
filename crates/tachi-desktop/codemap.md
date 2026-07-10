# crates/tachi-desktop/

## Responsibility

Provides the non-publishable native desktop host for Tachi commands. It supplies a minimal
macOS application shell and a reusable Rust host library with validated dispatch, app state,
offline recovery, registry parity, and release-package verification.

## Design Patterns

- **Host Adapter:** the crate wraps `tachi-shell` rather than reimplementing command logic.
- **Facade:** `DesktopHost` and free dispatch functions present a stable desktop-facing API
  over the shell command and progress infrastructure.
- **Boundary Validation:** typed input/output schemas and a desktop error taxonomy isolate UI
  callers from raw command-line and string-error contracts.
- **Platform Shell:** the binary uses macOS AppKit FFI for a small native window, while the
  library remains independently testable.

## Data & Control Flow

1. The desktop binary chooses a repository root and launches the native host window.
2. UI/application callers select from the shared registered command catalog.
3. Desktop schema validation converts invocation arguments into accepted command shapes.
4. Dispatch flows through `tachi_shell::tauri_bridge`, optionally carrying cancellation and
   progress callbacks, then validates the command-specific output contract.
5. App state records the latest command and a bounded history for rendering.

## Integration Points

- Depends on [`../tachi-shell`](../tachi-shell/codemap.md) for command registry, execution,
  cancellation, and progress reporting.
- Uses `serde_json` to validate structured command output and `sha2` for release manifests.
- `publish = false`: this is a workspace application/host, not a crates.io API package.
- Detailed implementation map: [`src/codemap.md`](src/codemap.md).
