# crates/tachi-desktop/src/

## Responsibility

Implements the desktop host boundary: command dispatch and validation, stateful text UI
model, native macOS launch surface, typed failures, offline bootstrap support, command
registry comparison, and release artifact integrity checks.

## Design Patterns

- **Facade (`lib.rs`):** `DesktopHost` and dispatch helpers wrap
  `tachi_shell::tauri_bridge`, enforcing registered-command and schema checks around calls.
- **State Model (`app.rs`):** `DesktopAppState` owns repository selection and a bounded
  command-history ring; `DesktopApp` renders a deterministic textual view and delegates runs.
- **Typed Error Taxonomy (`error.rs`):** `DesktopError` categorizes validation, policy, I/O,
  timeout, cancellation, and internal failures and can convert them to `CommandOutput`.
- **Discriminated Input Schema (`schema.rs`):** `DesktopInvokeInput` models every accepted
  command shape; allowlisted flags, conflict groups, and shell-control rejection protect the
  desktop invocation boundary. Output validation dispatches by `CommandOutputKind`.
- **Offline Cache Service (`offline.rs`):** contained-path checks guard restoration of a fixed
  cache manifest, version comparison, and cached bootstrap operations.
- **Manifest/Set Reconciliation (`release_artifacts.rs`, `registry.rs`):** SHA-256 manifests
  and set differences make package integrity and CLI/desktop command parity deterministic.
- **Platform Adapter (`main.rs`):** macOS-only AppKit messaging constructs the native window;
  other platforms retain a simple host entry path.

## Data & Control Flow

1. `main` parses root-selection flags, optionally opens a native directory chooser, and
   launches the platform window for the chosen repository.
2. `DesktopAppState::run_command` calls no-op-progress dispatch, captures status/streams in a
   `DesktopCommandSnapshot`, and caps history growth.
3. Dispatch validates command membership and input schema, invokes the shell bridge with a
   cancellation token/reporter, then checks the output shape before returning it.
4. Offline functions validate that cache inputs and repository outputs cannot escape their
   roots, copy the fixed cache set, compare version pins, and bootstrap from restored scripts.
5. Release functions hash requested files into a sorted manifest, re-hash for verification,
   or recursively compare actual package files with an expected set.

## Integration Points

- `tachi_shell::commands`: shared registry, `CommandOutput`, and output-kind metadata.
- `tachi_shell::tauri_bridge`: actual command dispatch with optional progress/cancellation.
- `tachi_shell::progress`: `CancellationToken`, `ProgressReporter`, and no-op reporter.
- `serde_json`: semantic validation of infographic JSON payloads.
- `sha2`: release artifact SHA-256 computation.
- AppKit Objective-C runtime (macOS): application activation, open panel, text view, and
  window lifecycle.
