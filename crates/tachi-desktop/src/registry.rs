use std::collections::BTreeSet;

use tachi_shell::commands::control_plane_commands;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryDiff {
    pub shared_commands: Vec<String>,
    pub cli_only_commands: Vec<String>,
    pub desktop_only_commands: Vec<String>,
}

pub fn collect_cli_commands() -> &'static [&'static str] {
    control_plane_commands()
}

pub fn collect_desktop_commands() -> &'static [&'static str] {
    super::registered_commands()
}

pub fn diff_registry(cli_commands: &[&str], desktop_commands: &[&str]) -> RegistryDiff {
    let cli_set = cli_commands.iter().copied().collect::<BTreeSet<_>>();
    let desktop_set = desktop_commands.iter().copied().collect::<BTreeSet<_>>();

    let shared_commands = cli_set
        .intersection(&desktop_set)
        .map(|command| (*command).to_string())
        .collect::<Vec<_>>();
    let cli_only_commands = cli_set
        .difference(&desktop_set)
        .map(|command| (*command).to_string())
        .collect::<Vec<_>>();
    let desktop_only_commands = desktop_set
        .difference(&cli_set)
        .map(|command| (*command).to_string())
        .collect::<Vec<_>>();

    RegistryDiff {
        shared_commands,
        cli_only_commands,
        desktop_only_commands,
    }
}
