use std::{fs, process::Command};

#[test]
fn malformed_attribution_fails_report_cli_without_overwriting_output() {
    let root = std::env::temp_dir().join(format!("tachi-bad-report-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("threats.md"), "## 7. Recommended Actions\n\n| Finding ID | Component | Threat | Risk Level | Mitigation |\n|---|---|---|---|---|\n| OI-1 | Agent | Injection | High | Validate |\n\n## 9. Source Attribution\n\n```yaml\nOI-1: [\n```\n").unwrap();
    let output_path = root.join("report-data.typ");
    fs::write(&output_path, "existing report").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_report-data"))
        .env("PATH", "")
        .arg("--target-dir")
        .arg(&root)
        .arg("--template-dir")
        .arg(&root)
        .arg("--output")
        .arg(&output_path)
        .arg("--cleanup-mislabeled-images")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("threats.md") && error.contains("malformed YAML"),
        "{error}"
    );
    assert_eq!(fs::read_to_string(output_path).unwrap(), "existing report");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn permission_command_runs_without_interpreters_and_rejects_bad_files() {
    let root = std::env::temp_dir().join(format!("tachi-permissions-cli-{}", std::process::id()));
    fs::create_dir_all(root.join(".claude")).unwrap();
    fs::create_dir_all(root.join("docs/standards")).unwrap();
    fs::write(
        root.join("docs/standards/CLAUDE_PERMISSIONS.md"),
        "## 4. Rules\n| `Read` | reason |\n## 5. Built in\n",
    )
    .unwrap();
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_permissions-check"))
            .env_clear()
            .env("PATH", "")
            .arg(&root)
            .output()
            .unwrap()
    };
    for (settings, expected) in [
        ("{", false),
        (
            r#"{"permissions":{"allow":["Write"],"ask":[],"deny":[]}}"#,
            false,
        ),
        (
            r#"{"permissions":{"allow":["Read"],"ask":[],"deny":[]}}"#,
            true,
        ),
    ] {
        fs::write(root.join(".claude/settings.json"), settings).unwrap();
        let output = invoke();
        assert_eq!(
            output.status.success(),
            expected,
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn catalog_check_needs_no_python_typst_or_external_command() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_catalog-drift"))
        .env_clear()
        .env("PATH", "")
        .arg("--check")
        .arg("--root")
        .arg(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
