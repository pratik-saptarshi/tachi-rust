pub mod app;
pub mod error;
pub mod offline;
pub mod registry;
pub mod release_artifacts;
pub mod schema;

use std::path::Path;
use std::path::PathBuf;

use tachi_shell::commands::control_plane_commands;
use tachi_shell::commands::CommandOutput;
use tachi_shell::progress::CancellationToken;
use tachi_shell::progress::NoopProgressReporter;
use tachi_shell::progress::ProgressReporter;
use tachi_shell::tauri_bridge::dispatch_command;
use tachi_shell::tauri_bridge::dispatch_command_with_progress;

#[derive(Debug, Default, Clone, Copy)]
pub struct DesktopHost;

impl DesktopHost {
    pub const fn new() -> Self {
        Self
    }

    pub fn registered_commands(self) -> &'static [&'static str] {
        registered_commands()
    }

    pub fn dispatch_command(self, command: &str, repo_root: &Path, args: &[&str]) -> CommandOutput {
        dispatch_desktop_command(command, repo_root, args)
    }

    pub fn dispatch_command_with_progress(
        self,
        command: &str,
        repo_root: &Path,
        args: &[&str],
        token: &CancellationToken,
        reporter: &mut dyn ProgressReporter,
    ) -> CommandOutput {
        dispatch_desktop_command_with_progress(command, repo_root, args, token, reporter)
    }
}

pub fn registered_commands() -> &'static [&'static str] {
    control_plane_commands()
}

pub fn dispatch_desktop_command(command: &str, repo_root: &Path, args: &[&str]) -> CommandOutput {
    if let Err(err) = schema::validate_invoke_input_typed(command, repo_root, args) {
        return err.into_command_output(command);
    }
    let output = dispatch_command(command, repo_root, args);
    match schema::validate_invoke_output_typed(command, &output) {
        Ok(()) => output,
        Err(err) => err.into_command_output(command),
    }
}

pub fn dispatch_desktop_command_with_progress(
    command: &str,
    repo_root: &Path,
    args: &[&str],
    token: &CancellationToken,
    reporter: &mut dyn ProgressReporter,
) -> CommandOutput {
    if let Err(err) = schema::validate_invoke_input_typed(command, repo_root, args) {
        return err.into_command_output(command);
    }
    let output = dispatch_command_with_progress(command, repo_root, args, token, reporter);
    match schema::validate_invoke_output_typed(command, &output) {
        Ok(()) => output,
        Err(err) => err.into_command_output(command),
    }
}

pub fn dispatch_desktop_command_owned(
    command: &str,
    repo_root: PathBuf,
    args: Vec<String>,
) -> CommandOutput {
    let borrowed_args = args.iter().map(String::as_str).collect::<Vec<_>>();
    dispatch_desktop_command(command, &repo_root, &borrowed_args)
}

pub fn dispatch_desktop_command_with_noop_progress(
    command: &str,
    repo_root: &Path,
    args: &[&str],
) -> CommandOutput {
    let token = CancellationToken::new();
    let mut reporter = NoopProgressReporter;
    dispatch_desktop_command_with_progress(command, repo_root, args, &token, &mut reporter)
}

pub use tachi_shell::commands::CommandOutput as DesktopCommandOutput;
pub use tachi_shell::progress::CancellationToken as DesktopCancellationToken;
pub use tachi_shell::progress::ProgressEvent;
pub use tachi_shell::progress::ProgressReporter as DesktopProgressReporter;

pub use error::DesktopError;
pub use error::DesktopErrorKind;
pub use offline::bootstrap_from_cache;
pub use offline::bootstrap_from_cache_typed;
pub use offline::check_for_update;
pub use offline::check_for_update_typed;
pub use offline::restore_offline_cache;
pub use offline::restore_offline_cache_typed;
pub use offline::BootstrapReport;
pub use offline::OfflineRestoreReport;
pub use offline::UpdateCheck;
pub use registry::collect_cli_commands;
pub use registry::collect_desktop_commands;
pub use registry::diff_registry;
pub use registry::RegistryDiff;
pub use release_artifacts::build_release_manifest;
pub use release_artifacts::build_release_manifest_typed;
pub use release_artifacts::validate_package_contents;
pub use release_artifacts::validate_package_contents_typed;
pub use release_artifacts::verify_checksum_matrix;
pub use release_artifacts::verify_checksum_matrix_typed;
pub use release_artifacts::PackageContentReport;
pub use release_artifacts::ReleaseArtifact;
pub use release_artifacts::ReleaseManifest;
pub use schema::render_schema_error;
pub use schema::validate_invoke_input;
pub use schema::validate_invoke_input_typed;
pub use schema::validate_invoke_output;
pub use schema::validate_invoke_output_typed;
pub use schema::DesktopInvokeInput;
