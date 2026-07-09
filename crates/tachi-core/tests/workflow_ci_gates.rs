use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

// Contract inventory:
// - Breadth gate: rust-workspace.yml keeps the full PR matrix explicit.
// - Fast-fail guardrails: PR concurrency, parse, and rustfmt lanes fail early.
// - Security/release invariants: clippy, gitleaks, supply-chain, mmdc, and
//   transitional init checks stay fail-closed and separately asserted.

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

fn parse_workflow(name: &str, text: &str) -> serde_yaml::Value {
    serde_yaml::from_str(text).unwrap_or_else(|err| panic!("parse workflow {name}: {err}"))
}

#[test]
fn workspace_cargo_test_pr_gate_runs_full_workspace_suite() {
    let text = workflow_text("rust-workspace.yml");
    let workflow = parse_workflow("rust-workspace.yml", &text);
    let active_packages = workspace_member_packages();

    assert_workflow_uses_pinned_repo_toolchain("rust-workspace.yml", &text);
    assert!(
        workflow_declares_unfiltered_event(&workflow, "pull_request"),
        "rust-workspace workflow must run on unfiltered pull_request events"
    );
    assert_eq!(
        workflow_job_name(&workflow, "cargo-test"),
        Some("cargo test -p ${{ matrix.package }} --all-targets"),
        "cargo-test job must use a package matrix"
    );
    assert_eq!(
        workflow_matrix_values(&workflow, "cargo-test", "package"),
        active_packages,
        "cargo-test matrix must derive from active root workspace packages"
    );
    assert_job_has_run_command(
        &workflow,
        "cargo-test",
        "cargo test -p ${{ matrix.package }} --all-targets",
    );
    assert_eq!(
        workflow_job_name(&workflow, "shell-tests"),
        Some("cargo test -p tachi-shell (${{ matrix.suite }})"),
        "shell tests must run in a dedicated split matrix"
    );
    assert_eq!(
        workflow_matrix_values(&workflow, "shell-tests", "suite"),
        vec![
            String::from("shell-init"),
            String::from("shell-integration"),
            String::from("shell-smoke"),
        ],
        "shell test matrix must include the semantic suite set"
    );
    assert_eq!(
        workflow_matrix_values(&workflow, "shell-tests", "command"),
        vec![
            String::from("cargo test -p tachi-shell --test command_registry --test coverage_audit --test infographic_data --test report_data_result --test tauri_shell_scaffold --test control_plane"),
            String::from("cargo test -p tachi-shell --test init_adversarial --test init_constitution --test init_defaults_env --test init_manifest_paths --test init_precommit_matrix --test init_substitution --test init_timing_trace --test init_trace_summary"),
            String::from("cargo test -p tachi-shell --test tauri_bridge --test template_config_load --test template_git_clone_timeout"),
        ],
        "shell test matrix commands must match the required semantic slices"
    );
    assert_job_has_run_command(
        &workflow,
        "cargo-test",
        "sudo apt-get install -y ripgrep pkg-config",
    );
}

#[test]
fn clippy_sarif_workflow_fails_closed_without_losing_upload() {
    let text = workflow_text("rust-clippy.yml");
    let workflow = parse_workflow("rust-clippy.yml", &text);

    assert_workflow_uses_pinned_repo_toolchain("rust-clippy.yml", &text);
    assert!(
        !workflow_has_continue_on_error(&workflow),
        "clippy workflow must not mask lint failures with continue-on-error"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("-- -D warnings")),
        "clippy workflow must deny warnings"
    );
    assert!(
        workflow_step_field(&workflow, "Upload analysis results to GitHub", "if")
            == Some("always()"),
        "clippy SARIF upload step must still run after clippy failures"
    );
    assert_workflow_has_run_line(
        &workflow,
        "cargo install --locked --version 0.8.0 clippy-sarif",
    );
    assert_workflow_has_run_line(
        &workflow,
        "cargo install --locked --version 0.8.0 sarif-fmt",
    );
    for status in [
        "PIPE_STATUSES=(\"${PIPESTATUS[@]}\")",
        "CLIPPY_STATUS=${PIPE_STATUSES[0]}",
        "CONVERTER_STATUS=${PIPE_STATUSES[1]}",
        "FORMATTER_STATUS=${PIPE_STATUSES[3]}",
        "SARIF_STATUS=$?",
    ] {
        assert_workflow_has_run_line(&workflow, status);
    }
    assert!(
        workflow_run_bodies(&workflow).any(|run| {
            run.contains("jq -e '.version == \"2.1.0\" and (.runs | type == \"array\")' rust-clippy-results.sarif")
        }),
        "clippy workflow must structurally validate SARIF before upload"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| {
            run.contains(
                "for status in CLIPPY_STATUS CONVERTER_STATUS FORMATTER_STATUS SARIF_STATUS",
            )
        }),
        "clippy workflow must re-emit captured pipeline statuses after SARIF upload"
    );
}

#[test]
fn gitleaks_workflow_uploads_sarif_but_fails_closed() {
    let text = workflow_text("gitleaks.yml");
    let workflow = parse_workflow("gitleaks.yml", &text);

    assert!(
        !workflow_has_continue_on_error(&workflow),
        "gitleaks workflow must not mask scanner failures with continue-on-error"
    );
    assert!(
        workflow_step_field(&workflow, "Upload SARIF to GitHub Code Scanning", "if")
            == Some("always()"),
        "gitleaks SARIF upload step must still run after scanner failures"
    );
    for status in [
        "GITLEAKS_STATUS=$?",
        "GITLEAKS_SARIF_STATUS=$?",
        "exit \"$GITLEAKS_STATUS\"",
    ] {
        assert_workflow_has_run_line(&workflow, status);
    }
    assert!(
        workflow_run_bodies(&workflow).any(|run| {
            run.contains(
                "jq -e '.version == \"2.1.0\" and (.runs | type == \"array\")' gitleaks.sarif",
            )
        }),
        "gitleaks workflow must structurally validate SARIF before upload"
    );
}

#[test]
fn publish_gate_runs_supply_chain_policy_checks() {
    let text = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let deny_config = fs::read_to_string(repo_root().join("deny.toml")).expect("read deny.toml");
    let workflow_text = workflow_text("rust-supply-chain.yml");
    let workflow = parse_workflow("rust-supply-chain.yml", &workflow_text);

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
    assert_workflow_uses_pinned_repo_toolchain("rust-supply-chain.yml", &workflow_text);
    for command in [
        "cargo install --locked --version 0.22.2 cargo-audit",
        "cargo install --locked --version 0.19.9 cargo-deny",
        "cargo audit",
        "cargo deny check advisories bans licenses sources",
    ] {
        assert_workflow_has_run_line(&workflow, command);
    }
    assert!(deny_config.contains("multiple-versions = \"deny\""));
    assert!(deny_config.contains("wildcards = \"allow\""));
    assert_license_exceptions_require_metadata(&deny_config);
}

#[test]
fn retired_tauri_adapter_is_absent_from_release_dependency_surface() {
    let workspace_manifest =
        fs::read_to_string(repo_root().join("Cargo.toml")).expect("read workspace Cargo.toml");
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let required_workflows = [
        "rust-workspace.yml",
        "rust-clippy.yml",
        "rust-supply-chain.yml",
    ];

    assert!(
        !workspace_manifest.contains("src-tauri"),
        "root workspace must not retain retired src-tauri member or exclude entries"
    );
    for retired_path in [
        "src-tauri/Cargo.toml",
        "src-tauri/Cargo.lock",
        ".github/workflows/tauri-adapter-compatibility.yml",
    ] {
        assert!(
            !repo_root().join(retired_path).exists(),
            "{retired_path} must stay absent after retiring the vulnerable adapter surface"
        );
    }
    for retired_command in [
        "tauri-adapter-check",
        "cargo metadata --manifest-path src-tauri/Cargo.toml",
        "cargo check --manifest-path src-tauri/Cargo.toml",
    ] {
        assert!(
            !makefile.contains(retired_command),
            "Makefile must not expose retired adapter command {retired_command}"
        );
    }
    for name in required_workflows {
        let text = workflow_text(name);
        assert!(
            !text.contains("src-tauri") && !text.contains("tachi-tauri"),
            "{name} must not accidentally gate the retired adapter"
        );
    }
}

#[test]
fn feature_and_coverage_canary_tools_are_pinned_and_non_required() {
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let workflow = workflow_text("rust-feature-coverage-canary.yml");
    let workflow_yaml: serde_yaml::Value =
        serde_yaml::from_str(&workflow).expect("parse rust-feature-coverage-canary.yml");
    let llvm_cov_script =
        fs::read_to_string(repo_root().join("scripts/llvm-cov.sh")).expect("read llvm-cov.sh");

    assert_workflow_uses_pinned_repo_toolchain("rust-feature-coverage-canary.yml", &workflow);
    let steps = workflow_yaml
        .get("jobs")
        .and_then(|jobs| jobs.get("feature-coverage-canary"))
        .and_then(|job| job.get("steps"))
        .and_then(serde_yaml::Value::as_sequence)
        .expect("feature canary workflow steps");
    let install_index = workflow_step_index(steps, "Install pinned canary tools")
        .expect("install pinned canary tools step");
    let proof_index =
        workflow_step_index(steps, "Print canary tool proof").expect("print canary proof step");
    let feature_index =
        workflow_step_index(steps, "Run feature-combination canary").expect("feature canary step");
    let coverage_index =
        workflow_step_index(steps, "Run coverage tool proof canary").expect("coverage canary step");
    assert!(
        install_index < proof_index && proof_index < feature_index && feature_index < coverage_index,
        "feature/coverage canary workflow must install, prove, run cargo-hack, then run coverage serially"
    );
    assert!(
        workflow_declares_event(&workflow_yaml, "workflow_dispatch"),
        "feature/coverage canary workflow must include workflow_dispatch"
    );
    assert!(
        workflow_declares_event(&workflow_yaml, "schedule"),
        "feature/coverage canary workflow must include schedule"
    );
    assert!(
        !workflow_declares_event(&workflow_yaml, "pull_request")
            && !workflow_declares_event(&workflow_yaml, "push"),
        "feature/coverage canary must not be a required PR/main-push gate yet"
    );
    for command in [
        "cargo install --locked --version 0.6.45 cargo-hack",
        "cargo install --locked --version 0.8.7 cargo-llvm-cov",
        "cargo hack --version",
        "cargo llvm-cov --version",
        "cargo hack --version | grep -qx 'cargo-hack 0.6.45'",
        "cargo llvm-cov --version | grep -qx 'cargo-llvm-cov 0.8.7'",
        "cargo hack check --workspace --locked --each-feature --no-dev-deps",
        "git diff --exit-code -- Cargo.toml 'crates/*/Cargo.toml'",
        "./scripts/llvm-cov.sh --workspace --summary-only --fail-under-lines 85 --ignore-filename-regex 'target/|tests/'",
    ] {
        assert_workflow_has_run_line(&workflow_yaml, command);
    }
    for command in [
        "feature-combination-canary:",
        "cargo hack --version | grep -qx 'cargo-hack 0.6.45'",
        "cargo hack check --workspace --each-feature --no-dev-deps",
        "git diff --quiet -- Cargo.toml crates/*/Cargo.toml",
        "coverage-tool-proof:",
        "cargo llvm-cov --version | grep -qx 'cargo-llvm-cov 0.8.7'",
        "$(MAKE) llvm-cov",
    ] {
        assert!(
            makefile.contains(command),
            "Makefile must expose local canary command {command}"
        );
    }
    assert!(
        !publish_gate_commands(&makefile).any(|line| {
            line.contains("feature-combination-canary") || line.contains("coverage-tool-proof")
        }),
        "canary targets must not be part of publish-gate until signal/noise review promotes them"
    );
    for proof in [
        "missing llvm-cov tools in active toolchain",
        "llvm-cov",
        "llvm-profdata",
    ] {
        assert!(
            llvm_cov_script.contains(proof),
            "llvm-cov wrapper must keep llvm-tools-preview proof for {proof}"
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

fn workspace_member_packages() -> Vec<String> {
    let manifest =
        fs::read_to_string(repo_root().join("Cargo.toml")).expect("read workspace Cargo.toml");
    let mut packages = workspace_members_section(&manifest)
        .lines()
        .filter_map(|line| {
            let member = line.trim().trim_end_matches(',').trim_matches('"');
            member.strip_prefix("crates/").map(String::from)
        })
        .collect::<Vec<_>>();
    packages.sort();
    packages
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
fn pr_facing_workflows_cancel_superseded_runs() {
    for name in [
        "ci-workflow-parse.yml",
        "rust-workspace.yml",
        "rust-clippy.yml",
        "rust-supply-chain.yml",
        "gitleaks.yml",
        "tachi-mmdc-preflight.yml",
        "tachi-pytest.yml",
        "rustfmt.yml",
    ] {
        let workflow = parse_workflow(name, &workflow_text(name));

        assert!(
            workflow_has_cancelling_concurrency(&workflow),
            "{name} must cancel superseded runs on the same ref"
        );
    }
}

#[test]
fn ci_workflow_parse_gate_reports_actionlint() {
    let text = workflow_text("ci-workflow-parse.yml");
    let workflow = parse_workflow("ci-workflow-parse.yml", &text);

    assert!(
        workflow_declares_event(&workflow, "pull_request"),
        "workflow parse gate must run on pull_request"
    );
    assert_workflow_has_run_line(
        &workflow,
        "go install github.com/rhysd/actionlint/cmd/actionlint@v1.7.12",
    );
    assert_workflow_has_run_line(&workflow, "actionlint .github/workflows/*.yml");
}

#[test]
fn rustfmt_workflow_runs_as_a_dedicated_guardrail() {
    let text = workflow_text("rustfmt.yml");
    let workflow = parse_workflow("rustfmt.yml", &text);

    assert_workflow_uses_pinned_repo_toolchain("rustfmt.yml", &text);
    assert!(
        workflow_declares_event(&workflow, "pull_request"),
        "rustfmt workflow must run on pull_request"
    );
    assert_workflow_has_run_line(&workflow, "cargo fmt --all -- --check");
}

#[test]
fn specialist_workflows_keep_their_trigger_contracts() {
    for (name, allowed) in [
        ("gitleaks.yml", ["pull_request"].as_slice()),
        ("tachi-mmdc-preflight.yml", ["pull_request"].as_slice()),
        ("rust-clippy.yml", ["push", "pull_request", "schedule"].as_slice()),
        (
            "rust-supply-chain.yml",
            ["push", "pull_request", "schedule"].as_slice(),
        ),
        (
            "tachi-pytest.yml",
            ["push", "pull_request", "workflow_dispatch"].as_slice(),
        ),
    ] {
        let workflow = parse_workflow(name, &workflow_text(name));

        assert!(
            workflow_declares_only_events(&workflow, allowed),
            "{name} must keep its specialist trigger surface limited to {allowed:?}"
        );
    }
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
    let workflow = parse_workflow(name, text);
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

fn workflow_job<'a>(workflow: &'a serde_yaml::Value, job_name: &str) -> &'a serde_yaml::Value {
    workflow
        .get("jobs")
        .and_then(|jobs| jobs.get(job_name))
        .unwrap_or_else(|| panic!("workflow must define job {job_name}"))
}

fn workflow_job_name<'a>(workflow: &'a serde_yaml::Value, job_name: &str) -> Option<&'a str> {
    workflow_job(workflow, job_name)
        .get("name")
        .and_then(serde_yaml::Value::as_str)
}

fn workflow_matrix_values(
    workflow: &serde_yaml::Value,
    job_name: &str,
    matrix_key: &str,
) -> Vec<String> {
    let mut values = workflow_job(workflow, job_name)
        .get("strategy")
        .and_then(|strategy| strategy.get("matrix"))
        .and_then(|matrix| matrix.get("include"))
        .and_then(serde_yaml::Value::as_sequence)
        .unwrap_or_else(|| panic!("{job_name} must define matrix.include"))
        .iter()
        .filter_map(|entry| entry.get(matrix_key))
        .filter_map(serde_yaml::Value::as_str)
        .map(String::from)
        .collect::<Vec<_>>();
    values.sort();
    values
}

fn workflow_job_steps<'a>(
    workflow: &'a serde_yaml::Value,
    job_name: &str,
) -> &'a [serde_yaml::Value] {
    workflow_job(workflow, job_name)
        .get("steps")
        .and_then(serde_yaml::Value::as_sequence)
        .map(Vec::as_slice)
        .unwrap_or_else(|| panic!("{job_name} must define steps"))
}

fn assert_job_has_run_command(workflow: &serde_yaml::Value, job_name: &str, command: &str) {
    assert!(
        workflow_job_steps(workflow, job_name).iter().any(|step| {
            workflow_step_run(step)
                .is_some_and(|run| run.lines().any(|line| line.trim() == command))
        }),
        "{job_name} must run {command}"
    );
}

fn workflow_run_bodies(workflow: &serde_yaml::Value) -> impl Iterator<Item = &str> {
    workflow
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .into_iter()
        .flat_map(|jobs| jobs.values())
        .filter_map(|job| job.get("steps"))
        .filter_map(serde_yaml::Value::as_sequence)
        .flat_map(|steps| steps.iter())
        .filter_map(workflow_step_run)
}

fn assert_workflow_has_run_line(workflow: &serde_yaml::Value, command: &str) {
    assert!(
        workflow_run_bodies(workflow).any(|run| run.lines().any(|line| line.trim() == command)),
        "workflow must run {command}"
    );
}

fn workflow_has_continue_on_error(workflow: &serde_yaml::Value) -> bool {
    let jobs = workflow.get("jobs").and_then(serde_yaml::Value::as_mapping);
    let job_level = jobs.into_iter().flat_map(|jobs| jobs.values()).any(|job| {
        job.get("continue-on-error")
            .and_then(serde_yaml::Value::as_bool)
            == Some(true)
    });
    let step_level = jobs
        .into_iter()
        .flat_map(|jobs| jobs.values())
        .filter_map(|job| job.get("steps"))
        .filter_map(serde_yaml::Value::as_sequence)
        .flat_map(|steps| steps.iter())
        .any(|step| {
            step.get("continue-on-error")
                .and_then(serde_yaml::Value::as_bool)
                == Some(true)
        });
    job_level || step_level
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

fn workflow_declares_unfiltered_event(workflow: &serde_yaml::Value, event: &str) -> bool {
    match workflow.get("on").and_then(|events| events.get(event)) {
        Some(serde_yaml::Value::Null) => true,
        Some(serde_yaml::Value::Mapping(config)) => {
            !config.contains_key("paths") && !config.contains_key("paths-ignore")
        }
        Some(_) => true,
        None => false,
    }
}

fn workflow_declares_event(workflow: &serde_yaml::Value, event: &str) -> bool {
    workflow
        .get("on")
        .is_some_and(|events| events.get(event).is_some())
}

fn workflow_declares_only_events(workflow: &serde_yaml::Value, allowed: &[&str]) -> bool {
    let Some(events) = workflow.get("on").and_then(serde_yaml::Value::as_mapping) else {
        return false;
    };

    let declared = events
        .keys()
        .filter_map(serde_yaml::Value::as_str)
        .collect::<BTreeSet<_>>();
    let allowed = allowed.iter().copied().collect::<BTreeSet<_>>();

    declared == allowed
}

fn workflow_step_field<'a>(
    workflow: &'a serde_yaml::Value,
    step_name: &str,
    field: &str,
) -> Option<&'a str> {
    workflow
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)?
        .values()
        .filter_map(|job| job.get("steps"))
        .filter_map(serde_yaml::Value::as_sequence)
        .flat_map(|steps| steps.iter())
        .find(|step| {
            step.get("name")
                .and_then(serde_yaml::Value::as_str)
                .is_some_and(|name| name == step_name)
        })?
        .get(field)
        .and_then(serde_yaml::Value::as_str)
}

fn workflow_has_cancelling_concurrency(workflow: &serde_yaml::Value) -> bool {
    let Some(concurrency) = workflow.get("concurrency").and_then(serde_yaml::Value::as_mapping)
    else {
        return false;
    };

    let group = workflow_mapping_value(concurrency, "group").and_then(serde_yaml::Value::as_str);
    let cancel = workflow_mapping_value(concurrency, "cancel-in-progress")
        .and_then(serde_yaml::Value::as_bool);

    group.is_some_and(|group| {
        group.contains("github.workflow") && group.contains("github.ref")
    }) && cancel == Some(true)
}

fn workflow_mapping_value<'a>(
    mapping: &'a serde_yaml::Mapping,
    key: &str,
) -> Option<&'a serde_yaml::Value> {
    mapping
        .iter()
        .find(|(candidate, _)| candidate.as_str() == Some(key))
        .map(|(_, value)| value)
}
