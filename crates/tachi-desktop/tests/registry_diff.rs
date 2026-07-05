use pretty_assertions::assert_eq;
use tachi_desktop::{collect_cli_commands, collect_desktop_commands, diff_registry, RegistryDiff};

#[test]
fn registry_diff_reports_matching_control_plane_surface() {
    assert_eq!(collect_cli_commands(), collect_desktop_commands());

    let diff = diff_registry(collect_cli_commands(), collect_desktop_commands());
    assert_eq!(
        diff,
        RegistryDiff {
            shared_commands: vec![
                String::from("bootstrap"),
                String::from("coverage-audit"),
                String::from("infographic-data"),
                String::from("init"),
                String::from("install"),
                String::from("report-data"),
                String::from("risk-scores-sarif"),
                String::from("threats-sarif"),
                String::from("update"),
            ],
            cli_only_commands: Vec::new(),
            desktop_only_commands: Vec::new(),
        }
    );
}

#[test]
fn registry_diff_reports_missing_and_extra_commands() {
    let diff = diff_registry(
        &[
            "install",
            "init",
            "update",
            "bootstrap",
            "infographic-data",
            "coverage-audit",
            "report-data",
            "risk-scores-sarif",
            "threats-sarif",
        ],
        &[
            "install",
            "init",
            "update",
            "bootstrap",
            "infographic-data",
            "coverage-audit",
            "report-data",
            "risk-scores-sarif",
            "threats-sarif",
            "sync-status",
        ],
    );

    assert_eq!(
        diff,
        RegistryDiff {
            shared_commands: vec![
                String::from("bootstrap"),
                String::from("coverage-audit"),
                String::from("infographic-data"),
                String::from("init"),
                String::from("install"),
                String::from("report-data"),
                String::from("risk-scores-sarif"),
                String::from("threats-sarif"),
                String::from("update"),
            ],
            cli_only_commands: Vec::new(),
            desktop_only_commands: vec![String::from("sync-status")],
        }
    );
}
