# crates/tachi-cli/src/bin/

## Responsibility

Implements the user-facing CLI entry points for repository lifecycle operations, coverage
inspection, report/infographic data generation, and SARIF export.

## Design Patterns

- **Command Adapter:** every `main` owns only parsing, presentation, file emission, and exit
  codes; domain behavior is delegated to a same-named `tachi_shell::commands` function.
- **Pass-through Facade:** `install`, `init`, `update`, and `bootstrap` strip the host-level
  `--root`, forward remaining flags, and replay the returned `CommandOutput` streams/status.
- **Parse-Validate-Execute:** artifact commands parse required paths into `PathBuf` values,
  reject unknown/missing flags, execute a shell-layer function, then create parent
  directories and write the artifact atomically at the adapter level.
- **Explicit Error Protocol:** invalid CLI shape returns status `2`; execution or filesystem
  failure returns `1`; successful help/control-plane execution preserves status `0`.

## Data & Control Flow

- `install.rs`, `init.rs`, `update.rs`, `bootstrap.rs`: raw argv -> root plus pass-through
  flags -> `*_output` -> stdout/stderr replay -> returned status.
- `coverage-audit.rs`: optional root -> `coverage_audit_output` -> textual stdout summary.
- `infographic-data.rs`: root/template/output -> `infographic_data_output` -> JSON on stdout
  or disk.
- `report-data.rs`: target/template directories -> `report_data_result` ->
  `render_report_data_result` -> Typst stdout/file plus completion marker.
- `threats-sarif.rs`: threats input -> `threats_sarif_output` -> SARIF file plus finding and
  AG-8 diagnostics.
- `risk-scores-sarif.rs`: score and threat inputs -> `risk_scores_sarif_output` -> SARIF file
  plus result count. Compatibility metadata flags are accepted but not yet forwarded.

## Integration Points

- Calls `tachi_shell::commands::{bootstrap_output, coverage_audit_output,
  infographic_data_output, init_output, install_output, report_data_result,
  render_report_data_result, risk_scores_sarif_output, threats_sarif_output, update_output}`.
- Writes operator-consumed JSON, Typst, and SARIF artifacts; otherwise emits to standard
  streams for shell/CI composition.
- Command names and output forms must remain aligned with `tachi-desktop` schema and registry
  validation.
