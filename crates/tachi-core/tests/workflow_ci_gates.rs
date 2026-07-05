use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn workflow_text(name: &str) -> String {
    fs::read_to_string(repo_root().join(".github/workflows").join(name))
        .unwrap_or_else(|err| panic!("read workflow {name}: {err}"))
}

#[test]
fn workspace_cargo_test_pr_gate_runs_full_workspace_suite() {
    let text = workflow_text("rust-workspace.yml");

    assert_workflow_uses_pinned_repo_toolchain("rust-workspace.yml", &text);
    assert!(
        workflow_declares_unfiltered_event(&text, "pull_request"),
        "rust-workspace workflow must run on unfiltered pull_request events"
    );
    assert!(
        text.contains("name: cargo test -p ${{ matrix.package }} --all-targets"),
        "cargo-test job must use a package matrix"
    );
    for package in [
        "tachi-core",
        "tachi-mcp",
        "tachi-shell",
        "tachi-cli",
        "tachi-desktop",
    ] {
        assert!(
            text.contains(&format!("- package: {package}")),
            "cargo-test job must include {package} in the matrix"
        );
    }
    assert!(
        text.contains("cargo test -p ${{ matrix.package }} --all-targets"),
        "cargo-test job must run package-scoped cargo test --all-targets"
    );
    assert!(
        text.contains("name: cargo test -p tachi-shell (${{ matrix.suite }})"),
        "shell tests must run in a dedicated split matrix"
    );
    for suite in ["shell-smoke", "shell-init", "shell-integration"] {
        assert!(
            text.contains(&format!("suite: {suite}")),
            "shell test matrix must include {suite}"
        );
    }
    for command in [
        "cargo test -p tachi-shell --test command_registry --test coverage_audit --test infographic_data --test report_data_result --test tauri_shell_scaffold --test control_plane",
        "cargo test -p tachi-shell --test init_adversarial --test init_constitution --test init_defaults_env --test init_manifest_paths --test init_precommit_matrix --test init_substitution --test init_timing_trace --test init_trace_summary",
        "cargo test -p tachi-shell --test tauri_bridge --test template_config_load --test template_git_clone_timeout",
    ] {
        assert!(
            text.contains(command),
            "shell test matrix must run {command}"
        );
    }
    assert!(
        text.contains("sudo apt-get install -y ripgrep pkg-config"),
        "cargo-test job must install the lightweight tools required by the GTK-free workspace"
    );
}

#[test]
fn clippy_sarif_workflow_fails_closed_without_losing_upload() {
    let text = workflow_text("rust-clippy.yml");

    assert_workflow_uses_pinned_repo_toolchain("rust-clippy.yml", &text);
    assert!(
        !text.contains("continue-on-error: true"),
        "clippy workflow must not mask lint failures with continue-on-error"
    );
    assert!(
        text.contains("-- -D warnings"),
        "clippy workflow must deny warnings"
    );
    assert!(
        workflow_step_has_line(&text, "Upload analysis results to GitHub", "if: always()"),
        "clippy SARIF upload step must still run after clippy failures"
    );
    assert!(
        text.contains("exit \"$CLIPPY_STATUS\""),
        "clippy workflow must re-emit the captured clippy exit status"
    );
}

#[test]
fn transitional_init_workflow_uses_pinned_repo_toolchain() {
    let text = workflow_text("tachi-pytest.yml");

    assert_workflow_uses_pinned_repo_toolchain("tachi-pytest.yml", &text);
}

#[test]
fn repo_pins_required_rust_toolchain_components() {
    let text = fs::read_to_string(repo_root().join("rust-toolchain.toml"))
        .expect("read rust-toolchain.toml");
    let workspace_manifest =
        fs::read_to_string(repo_root().join("Cargo.toml")).expect("read workspace Cargo.toml");

    for required in [
        "channel = \"1.96.1\"",
        "profile = \"minimal\"",
        "\"clippy\"",
        "\"rustfmt\"",
        "\"llvm-tools-preview\"",
    ] {
        assert!(
            text.contains(required),
            "rust-toolchain.toml must include {required}"
        );
    }
    assert!(
        workspace_manifest.contains("rust-version = \"1.96\""),
        "workspace must declare the public Rust compiler floor"
    );
    for manifest in [
        "crates/tachi-core/Cargo.toml",
        "crates/tachi-cli/Cargo.toml",
        "crates/tachi-mcp/Cargo.toml",
        "crates/tachi-shell/Cargo.toml",
        "crates/tachi-desktop/Cargo.toml",
    ] {
        let text = fs::read_to_string(repo_root().join(manifest))
            .unwrap_or_else(|err| panic!("read {manifest}: {err}"));
        assert!(
            text.contains("rust-version.workspace = true"),
            "{manifest} must inherit the workspace Rust compiler floor"
        );
    }
}

fn assert_workflow_uses_pinned_repo_toolchain(name: &str, text: &str) {
    assert!(
        !text.contains("dtolnay/rust-toolchain@stable"),
        "{name} must not float on dtolnay/rust-toolchain@stable"
    );
    assert!(
        !text.contains("toolchain: stable"),
        "{name} must not override the repo pin with floating stable"
    );
    let workflow: serde_yaml::Value =
        serde_yaml::from_str(text).unwrap_or_else(|err| panic!("parse workflow {name}: {err}"));
    let jobs = workflow
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .unwrap_or_else(|| panic!("{name} must define jobs"));
    for (job_name, job) in jobs {
        let job_name = job_name.as_str().unwrap_or("<unnamed>");
        let steps = job
            .get("steps")
            .and_then(serde_yaml::Value::as_sequence)
            .unwrap_or_else(|| panic!("{name} job {job_name} must define steps"));
        let install_index = workflow_step_index(steps, "Install pinned Rust toolchain")
            .unwrap_or_else(|| panic!("{name} job {job_name} must install pinned Rust"));
        let proof_index = workflow_step_index(steps, "Print Rust toolchain proof")
            .unwrap_or_else(|| panic!("{name} job {job_name} must print toolchain proof"));
        assert!(
            install_index < proof_index,
            "{name} job {job_name} must install pinned Rust before printing proof"
        );
        let proof_run = workflow_step_run(&steps[proof_index])
            .unwrap_or_else(|| panic!("{name} job {job_name} proof step must be a run step"));
        for command in [
            "rustc -Vv",
            "cargo -Vv",
            "which rustc",
            "which cargo",
            "rustup which rustc",
        ] {
            assert!(
                proof_run.lines().any(|line| line.trim() == command),
                "{name} job {job_name} proof step must run {command}"
            );
        }
        if let Some(first_cargo_offset) = steps
            .iter()
            .enumerate()
            .skip(proof_index + 1)
            .position(|(_, step)| workflow_step_run(step).is_some_and(|run| run.contains("cargo ")))
        {
            let first_cargo_index = proof_index + 1 + first_cargo_offset;
            assert!(
                proof_index < first_cargo_index,
                "{name} job {job_name} must print proof before cargo commands"
            );
        }
    }
}

fn workflow_step_index(steps: &[serde_yaml::Value], step_name: &str) -> Option<usize> {
    steps.iter().position(|step| {
        step.get("name")
            .and_then(serde_yaml::Value::as_str)
            .is_some_and(|name| name == step_name)
    })
}

fn workflow_step_run(step: &serde_yaml::Value) -> Option<&str> {
    step.get("run").and_then(serde_yaml::Value::as_str)
}

fn workflow_declares_unfiltered_event(text: &str, event: &str) -> bool {
    let lines = text.lines().collect::<Vec<_>>();
    let Some(event_index) = lines
        .iter()
        .position(|line| line.trim() == format!("{event}:"))
    else {
        return false;
    };

    let event_indent = indentation(lines[event_index]);

    for line in lines.iter().skip(event_index + 1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = indentation(line);
        if indent <= event_indent && trimmed.ends_with(':') {
            break;
        }

        if matches!(trimmed, "paths:" | "paths-ignore:") {
            return false;
        }
    }

    true
}

fn indentation(line: &str) -> usize {
    line.chars()
        .take_while(|character| *character == ' ')
        .count()
}

fn workflow_step_has_line(text: &str, step_name: &str, required_line: &str) -> bool {
    let mut in_step = false;

    for line in text.lines().map(str::trim) {
        if line.starts_with("- name: ") {
            in_step = line == format!("- name: {step_name}");
            continue;
        }

        if in_step && line == required_line {
            return true;
        }
    }

    false
}
