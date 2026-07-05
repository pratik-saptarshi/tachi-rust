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
        text.contains("cargo install --locked --version 0.8.0 clippy-sarif")
            && text.contains("cargo install --locked --version 0.8.0 sarif-fmt"),
        "clippy SARIF helper tools must be version-pinned"
    );
    for status in [
        "PIPE_STATUSES=(\"${PIPESTATUS[@]}\")",
        "CLIPPY_STATUS=${PIPE_STATUSES[0]}",
        "CONVERTER_STATUS=${PIPE_STATUSES[1]}",
        "FORMATTER_STATUS=${PIPE_STATUSES[3]}",
        "SARIF_STATUS=$?",
    ] {
        assert!(
            text.contains(status),
            "clippy workflow must capture {status}"
        );
    }
    assert!(
        text.contains("jq -e '.version == \"2.1.0\" and (.runs | type == \"array\")' rust-clippy-results.sarif"),
        "clippy workflow must structurally validate SARIF before upload"
    );
    assert!(
        text.contains("for status in CLIPPY_STATUS CONVERTER_STATUS FORMATTER_STATUS SARIF_STATUS"),
        "clippy workflow must re-emit captured pipeline statuses after SARIF upload"
    );
}

#[test]
fn gitleaks_workflow_uploads_sarif_but_fails_closed() {
    let text = workflow_text("gitleaks.yml");

    assert!(
        !text.contains("continue-on-error: true"),
        "gitleaks workflow must not mask scanner failures with continue-on-error"
    );
    assert!(
        workflow_step_has_line(
            &text,
            "Upload SARIF to GitHub Code Scanning",
            "if: always()"
        ),
        "gitleaks SARIF upload step must still run after scanner failures"
    );
    for status in [
        "GITLEAKS_STATUS=$?",
        "GITLEAKS_SARIF_STATUS=$?",
        "exit \"$GITLEAKS_STATUS\"",
    ] {
        assert!(
            text.contains(status),
            "gitleaks workflow must capture and re-emit {status}"
        );
    }
    assert!(
        text.contains(
            "jq -e '.version == \"2.1.0\" and (.runs | type == \"array\")' gitleaks.sarif"
        ),
        "gitleaks workflow must structurally validate SARIF before upload"
    );
}

#[test]
fn publish_gate_runs_supply_chain_policy_checks() {
    let text = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let deny_config = fs::read_to_string(repo_root().join("deny.toml")).expect("read deny.toml");
    let workflow = workflow_text("rust-supply-chain.yml");

    assert!(
        text.contains("supply-chain-gate:"),
        "Makefile must expose a local supply-chain gate"
    );
    for command in [
        "cargo audit",
        "cargo deny check advisories bans licenses sources",
        "@$(MAKE) supply-chain-gate",
    ] {
        assert!(
            text.contains(command),
            "publish gate must include {command}"
        );
    }
    assert_workflow_uses_pinned_repo_toolchain("rust-supply-chain.yml", &workflow);
    for command in [
        "cargo install --locked --version 0.22.2 cargo-audit",
        "cargo install --locked --version 0.19.9 cargo-deny",
        "cargo audit",
        "cargo deny check advisories bans licenses sources",
    ] {
        assert!(
            workflow.contains(command),
            "supply-chain workflow must include {command}"
        );
    }
    assert!(deny_config.contains("multiple-versions = \"deny\""));
    assert!(deny_config.contains("wildcards = \"allow\""));
    assert_license_exceptions_require_metadata(&deny_config);
}

#[test]
fn transitional_tauri_adapter_is_explicitly_standalone() {
    let workspace_manifest =
        fs::read_to_string(repo_root().join("Cargo.toml")).expect("read workspace Cargo.toml");
    let adapter_manifest = fs::read_to_string(repo_root().join("src-tauri/Cargo.toml"))
        .expect("read src-tauri Cargo.toml");
    let adapter_lock = repo_root().join("src-tauri/Cargo.lock");
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let workflow = workflow_text("tauri-adapter-compatibility.yml");
    let required_workflows = [
        "rust-workspace.yml",
        "rust-clippy.yml",
        "rust-supply-chain.yml",
    ];

    assert!(
        workspace_manifest.contains("exclude = [\"src-tauri\"]"),
        "root workspace must explicitly exclude the transitional Tauri adapter"
    );
    assert!(
        !workspace_members_section(&workspace_manifest).contains("\"src-tauri\""),
        "src-tauri must not be an active workspace member"
    );
    assert!(
        adapter_lock.exists(),
        "standalone src-tauri adapter must commit its own Cargo.lock for locked validation"
    );
    for required in [
        "rust-version = \"1.96\"",
        "license = \"Apache-2.0\"",
        "publish = false",
    ] {
        assert!(
            adapter_manifest.contains(required),
            "standalone src-tauri manifest must include {required}"
        );
    }
    for command in [
        "tauri-adapter-check:",
        "cargo metadata --manifest-path src-tauri/Cargo.toml --locked --format-version 1",
        "cargo check --manifest-path src-tauri/Cargo.toml --locked",
    ] {
        assert!(
            makefile.contains(command),
            "Makefile must expose non-publish-blocking adapter validation command {command}"
        );
    }
    assert!(
        !publish_gate_commands(&makefile).any(|line| line.contains("tauri-adapter-check")),
        "transitional adapter validation must not be part of publish-gate until promoted"
    );
    assert_workflow_uses_pinned_repo_toolchain("tauri-adapter-compatibility.yml", &workflow);
    assert!(
        workflow.contains("workflow_dispatch:") && workflow.contains("schedule:"),
        "adapter workflow must be manual/scheduled compatibility evidence"
    );
    for dependency in [
        "libwebkit2gtk-4.1-dev",
        "libayatana-appindicator3-dev",
        "librsvg2-dev",
        "libssl-dev",
        "libxdo-dev",
        "pkg-config",
    ] {
        assert!(
            workflow.contains(dependency),
            "adapter workflow must install Tauri Linux dependency {dependency}"
        );
    }
    assert!(
        !workflow.contains("pull_request:") && !workflow.contains("push:"),
        "adapter workflow must not become a required PR/main gate accidentally"
    );
    for name in required_workflows {
        let text = workflow_text(name);
        assert!(
            !text.contains("src-tauri") && !text.contains("tachi-tauri"),
            "{name} must not accidentally gate the transitional adapter"
        );
    }
}

fn publish_gate_commands(makefile: &str) -> impl Iterator<Item = &str> {
    let mut in_publish_gate = false;
    makefile.lines().filter(move |line| {
        if line.starts_with("publish-gate:") {
            in_publish_gate = true;
            return false;
        }
        if in_publish_gate && !line.starts_with('\t') && !line.trim().is_empty() {
            in_publish_gate = false;
        }
        in_publish_gate && line.starts_with('\t')
    })
}

fn workspace_members_section(manifest: &str) -> &str {
    let start = manifest.find("members = [").expect("workspace members");
    let end = manifest[start..]
        .find(']')
        .map(|offset| start + offset + 1)
        .expect("workspace members close");
    &manifest[start..end]
}

fn assert_license_exceptions_require_metadata(deny_config: &str) {
    let uncommented = strip_toml_comments(deny_config);
    let exceptions = extract_inline_array(&uncommented, "exceptions")
        .expect("deny.toml must declare licenses.exceptions");
    let entries = inline_table_entries(exceptions);
    for entry in entries {
        let reason = inline_table_value(entry, "reason").unwrap_or("");
        for required in ["owner", "expires", "issue", "remediation"] {
            assert!(
                reason.contains(required),
                "deny.toml license exception `{entry}` must include {required} metadata in reason"
            );
        }
    }
}

fn strip_toml_comments(text: &str) -> String {
    text.lines()
        .map(|line| line.split_once('#').map_or(line, |(content, _)| content))
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_inline_array<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let start = text.find(&format!("{key} = ["))?;
    let after_open = text[start..].find('[')? + start + 1;
    let after_close = text[after_open..].find(']')? + after_open;
    Some(&text[after_open..after_close])
}

fn inline_table_entries(array: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut start = None;
    let mut depth = 0usize;
    for (index, character) in array.char_indices() {
        match character {
            '{' => {
                if depth == 0 {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = start.take() {
                        entries.push(&array[start..=index]);
                    }
                }
            }
            _ => {}
        }
    }
    entries
}

fn inline_table_value<'a>(entry: &'a str, key: &str) -> Option<&'a str> {
    let start = entry.find(&format!("{key} = \""))?;
    let value_start = start + key.len() + 4;
    let value_end = entry[value_start..].find('"')? + value_start;
    Some(&entry[value_start..value_end])
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
