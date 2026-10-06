use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

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
fn local_ci_manifest_is_the_canonical_projection_of_workspace_workflows() {
    let manifest_text = fs::read_to_string(repo_root().join(".github/ci-test-units.json"))
        .expect("read canonical local CI manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_text).expect("manifest must be valid JSON");
    let units = manifest
        .get("units")
        .and_then(serde_json::Value::as_array)
        .expect("manifest must contain a units array");

    let ids: BTreeSet<&str> = units
        .iter()
        .map(|unit| {
            unit.get("id")
                .and_then(serde_json::Value::as_str)
                .expect("each CI unit needs an id")
        })
        .collect();
    assert_eq!(ids.len(), units.len(), "CI unit ids must be unique");
    assert!(
        units.iter().all(|unit| matches!(
            unit.get("stage").and_then(serde_json::Value::as_str),
            Some("compile-and-test") | Some("test-slice")
        )),
        "every CI unit must declare a measurable execution stage"
    );

    let packages: BTreeSet<&str> = units
        .iter()
        .filter(|unit| unit.get("kind").and_then(serde_json::Value::as_str) == Some("package"))
        .map(|unit| {
            unit.get("package")
                .and_then(serde_json::Value::as_str)
                .expect("package units need a package")
        })
        .collect();
    assert_eq!(
        packages,
        [
            "tachi-core",
            "tachi-cli",
            "tachi-desktop",
            "tachi-mcp",
            "tachi-shell"
        ]
        .into_iter()
        .collect(),
        "manifest package units must match the hosted workspace matrix"
    );

    let shell_commands: BTreeSet<String> = units
        .iter()
        .filter(|unit| unit.get("kind").and_then(serde_json::Value::as_str) == Some("shell"))
        .map(|unit| {
            unit.get("argv")
                .and_then(serde_json::Value::as_array)
                .expect("shell units need argv")
                .iter()
                .map(|arg| arg.as_str().expect("argv entries must be strings"))
                .collect::<Vec<_>>()
                .join(" ")
                .to_owned()
        })
        .collect();
    for command in [
        "cargo test -p tachi-shell --test command_registry --test coverage_audit --test infographic_data --test output_integrity_preservation --test report_data_result --test tauri_shell_scaffold --test control_plane",
        "cargo test -p tachi-shell --test init_adversarial --test init_constitution --test init_defaults_env --test init_manifest_paths --test init_precommit_matrix --test init_substitution --test init_timing_trace --test init_trace_summary",
        "cargo test -p tachi-shell --test tauri_bridge --test template_config_load --test template_git_clone_timeout",
    ] {
        assert!(
            shell_commands.contains(command),
            "manifest must preserve hosted shell slice: {command}"
        );
    }

    for schema in [
        ".github/ci-test-units.json",
        "schemas/ci-test-units.schema.json",
        "schemas/ci-run-result.schema.json",
    ] {
        let path = repo_root().join(schema);
        assert!(
            path.is_file(),
            "planned CI contract artifact is missing: {schema}"
        );
        let text = fs::read_to_string(path).expect("read CI contract artifact");
        serde_json::from_str::<serde_json::Value>(&text)
            .unwrap_or_else(|err| panic!("{schema} must be valid JSON: {err}"));
    }
}

#[test]
fn local_runner_contract_is_argv_only_and_has_bounded_execution_controls() {
    let runner_path = repo_root().join("scripts/ci-local-runner.sh");
    let runner = fs::read_to_string(&runner_path).expect("read local CI runner");

    for forbidden in ["eval ", "sh -c", "bash -c", "sudo ", "docker.sock"] {
        assert!(
            !runner.contains(forbidden),
            "local runner must not contain unsafe execution primitive: {forbidden}"
        );
    }
    for required in [
        "local-full",
        "local-route-equivalent",
        "timeout",
        "0700",
        "rustup",
        "redact",
        "SIGTERM",
        "SIGINT",
        "cleanup",
        "ci-run-result.schema.json",
    ] {
        assert!(
            runner.contains(required),
            "local runner must document or implement {required}"
        );
    }
    assert!(
        runner_path
            .metadata()
            .expect("runner metadata")
            .permissions()
            .mode()
            & 0o111
            != 0,
        "local runner must be executable"
    );
}

#[test]
fn make_targets_use_the_canonical_runner_and_keep_publish_gate_hosted_only() {
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    assert!(
        makefile.contains("test: ## Run the canonical local-full CI unit runner")
            && makefile.contains("./scripts/ci-local-runner.sh --mode local-full"),
        "make test must invoke the canonical local-full runner"
    );
    assert!(
        makefile.contains("test-route: ## Run the route-equivalent CI unit runner")
            && makefile.contains("./scripts/ci-local-runner.sh --mode local-route-equivalent"),
        "make test-route must invoke the route-equivalent runner"
    );
    let publish_gate = makefile
        .split_once("publish-gate:")
        .expect("publish gate target")
        .1;
    assert!(
        publish_gate.contains("@$(MAKE) test"),
        "publish gate must retain the canonical local runner"
    );
    assert!(
        !publish_gate.contains("act") && !publish_gate.contains("podman"),
        "publish gate must not depend on advisory workflow emulation"
    );
}

#[test]
fn workspace_cargo_test_pr_gate_runs_full_workspace_suite() {
    let text = workflow_text("rust-workspace.yml");
    let workflow = parse_workflow("rust-workspace.yml", &text);
    let classifier = fs::read_to_string(repo_root().join("scripts/ci-route-classifier.sh"))
        .expect("read shared CI route classifier");

    assert_workflow_uses_pinned_repo_toolchain("rust-workspace.yml", &text);
    assert!(
        workflow_job_name(&workflow, "route")
            == Some("route decision and stable orchestrator check"),
        "rust-workspace workflow must expose the stable route classifier"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("scripts/ci-route-classifier.sh")),
        "rust-workspace must invoke the shared route classifier"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("scripts/ci-route-changed-paths.sh")),
        "rust-workspace must use the shared rename-aware changed-path producer"
    );
    assert!(
        !text.contains("git diff --name-only"),
        "rust-workspace must not discard rename source paths"
    );
    assert!(
        classifier.contains("docs/testing/tdd-evidence\\.json"),
        "test-owned TDD evidence must widen to the active contract route"
    );
    for required in [
        "active_contract_pattern=",
        "passive_docs_only",
        "docs-only passive paths observed",
        "dependency_closure",
        "crate-local changes routed through dependency closure",
        "active docs or shared surface touched",
        "protected ref stays full mode",
        "refs/heads/main",
        "unknown non-docs paths stay full mode",
        "emergency full-ci override",
        "repository_contract_pattern=",
    ] {
        assert!(
            classifier.contains(required),
            "shared route classifier must contain {required}"
        );
    }
    assert!(
        text.contains("force_full_ci"),
        "rust-workspace workflow must expose an emergency full-CI input"
    );
    assert!(
        classifier.contains("emergency full-ci override"),
        "route classifier must record the emergency full-CI override"
    );
    assert!(
        classifier.contains("repository_contracts_required"),
        "route classifier must expose repository-wide contract selection independently"
    );
    assert!(
        text.contains("fromJson(needs.route.outputs.packages_json)"),
        "cargo-test matrix must consume route-selected packages"
    );
    assert!(
        classifier.contains(
            "[\"tachi-core\",\"tachi-mcp\",\"tachi-cli\",\"tachi-shell\",\"tachi-desktop\"]"
        ),
        "route classifier must preserve the full-package baseline"
    );
    assert!(
        workflow_job_field(&workflow, "cargo-test", "if")
            == Some("needs.route.outputs.mode != 'passive_docs_only'"),
        "cargo-test matrix must skip passive docs-only changes"
    );
    assert!(
        workflow_job_field(&workflow, "shell-tests", "if")
            == Some("needs.route.outputs.mode != 'passive_docs_only'"),
        "shell-tests matrix must skip passive docs-only changes"
    );
    assert!(
        workflow_declares_unfiltered_event(&workflow, "pull_request"),
        "rust-workspace workflow must run on unfiltered pull_request events"
    );
    assert_eq!(
        workflow_job_name(&workflow, "cargo-test"),
        Some("cargo test -p ${{ matrix.package }} --all-targets"),
        "cargo-test job must use a package matrix"
    );
    assert_job_has_run_command(
        &workflow,
        "cargo-test",
        "cargo test -p ${{ matrix.package }} --all-targets",
    );
    assert!(
        workflow_run_bodies(&workflow).any(|command| command
            .contains("cargo test -p ${{ matrix.package }} --all-targets -- --test-threads=1")),
        "core package process-tree tests must run with serialized test threads"
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
            String::from("cargo test -p tachi-shell --test command_registry --test coverage_audit --test infographic_data --test output_integrity_preservation --test report_data_result --test tauri_shell_scaffold --test control_plane"),
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
    for required in [
        "Capture package timing artifact",
        "Upload package timing artifact",
        "ci-timing-package-${{ matrix.package }}",
        "Capture shell timing artifact",
        "Upload shell timing artifact",
        "ci-timing-shell-${{ matrix.suite }}",
        "duration_ms",
    ] {
        assert!(
            text.contains(required),
            "rust-workspace workflow must emit stage timing evidence: {required}"
        );
    }
}

#[test]
fn repository_contract_job_runs_independently_for_contract_inputs() {
    let text = workflow_text("rust-workspace.yml");
    let workflow = parse_workflow("rust-workspace.yml", &text);
    let route_outputs = workflow_job(&workflow, "route")
        .get("outputs")
        .expect("route outputs");

    assert_eq!(
        route_outputs
            .get("repository_contracts_required")
            .and_then(serde_yaml::Value::as_str),
        Some("${{ steps.route.outputs.repository_contracts_required }}"),
        "route job must expose repository contract selection"
    );
    assert_eq!(
        workflow_job_field(&workflow, "repository-contracts", "if"),
        Some("needs.route.outputs.repository_contracts_required == 'true'"),
        "repository contracts must run based on their inputs independently of package routing"
    );
    assert_eq!(
        workflow_job_name(&workflow, "repository-contracts"),
        Some("repository-wide contract tests"),
        "compact repository contract job must have a stable check name"
    );
    assert_job_has_run_command(
        &workflow,
        "repository-contracts",
        "cargo test -p tachi-core --test workflow_ci_gates -- --test-threads=1",
    );
    let job = workflow_job(&workflow, "repository-contracts");
    assert_eq!(
        job.get("needs")
            .and_then(serde_yaml::Value::as_sequence)
            .map(|needs| needs
                .iter()
                .filter_map(serde_yaml::Value::as_str)
                .collect::<Vec<_>>()),
        Some(vec!["route"]),
        "repository contract job must consume route classification"
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
fn codeql_v4_maintenance_contract_is_explicit_and_fail_closed() {
    let gitleaks = workflow_text("gitleaks.yml");
    let clippy = workflow_text("rust-clippy.yml");
    let policy_path = repo_root().join("docs/security/codeql-maintenance.md");
    let policy = fs::read_to_string(&policy_path).expect("read CodeQL maintenance policy");
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let maintenance_script = repo_root().join("scripts/codeql-maintenance-check.sh");

    for (name, text) in [("gitleaks.yml", gitleaks), ("rust-clippy.yml", clippy)] {
        assert!(
            text.contains("github/codeql-action/upload-sarif@v4"),
            "{name} must use the supported CodeQL action v4 line"
        );
        assert!(
            !text.contains("github/codeql-action/upload-sarif@v3"),
            "{name} must not regress to CodeQL action v3"
        );
        assert!(
            text.contains("sarif_file:") && text.contains("category:"),
            "{name} must identify the SARIF input and upload category"
        );
        assert!(
            text.contains("if: always()"),
            "{name} must preserve SARIF upload evidence after scanner failure"
        );
    }

    for required in [
        "v4.37.0",
        "2.26.0",
        "Node 24",
        "floating @v4 risk acceptance",
        "repository-contained",
        "redaction",
        "trusted-event",
        "rollback",
        "quarterly",
        "historical",
    ] {
        assert!(
            policy.contains(required),
            "CodeQL maintenance policy must document {required}"
        );
    }
    assert!(
        makefile.contains("codeql-maintenance-gate:")
            && makefile.contains("@$(MAKE) codeql-maintenance-gate"),
        "publish gate must include the CodeQL maintenance gate"
    );
    assert!(
        maintenance_script.is_file(),
        "CodeQL maintenance inventory script must be checked in"
    );

    let upstream_script = repo_root().join("scripts/codeql-upstream-release-check.sh");
    let upstream_workflow = workflow_text("codeql-maintenance.yml");
    assert!(
        upstream_script.is_file(),
        "CodeQL upstream release check script must be checked in"
    );
    for required in [
        "api.github.com/repos/github/codeql-action/releases",
        "jq",
        "latest non-prerelease v4 tag",
    ] {
        let script = fs::read_to_string(&upstream_script).expect("read upstream CodeQL check");
        assert!(
            script.contains(required),
            "upstream CodeQL check must contain {required}"
        );
    }
    assert!(
        upstream_workflow.contains("schedule:")
            && upstream_workflow.contains("workflow_dispatch:")
            && upstream_workflow.contains("codeql-upstream-release-check.sh"),
        "CodeQL upstream release check must be manual and scheduled"
    );

    let timing_script = repo_root().join("scripts/verify-ci-timing-artifacts.sh");
    let timing_script_text = fs::read_to_string(&timing_script).expect("read timing verifier");
    for required in [
        "gh run download",
        "ci-timing-package-tachi-core",
        "ci-timing-shell-shell-integration",
        "expected_artifacts:8",
        ".commit == $commit",
        "[ \"$COMMIT\" = \"auto\" ]",
        "gh run view",
        "workflowName",
        "conclusion",
        "completed",
        "attempt",
        "workflow_name",
        "source_head_sha",
        "control_pr_head_sha",
        "expected_run_commit_sha",
        "is_matched_control",
        "GITHUB_REF",
    ] {
        assert!(
            timing_script_text.contains(required),
            "timing verifier must contain {required}"
        );
    }
    assert!(
        makefile.contains("verify-ci-timing-artifacts:"),
        "Makefile must expose hosted timing artifact verification"
    );
    let runner_text = fs::read_to_string(repo_root().join("scripts/ci-local-runner.sh"))
        .expect("read local runner");
    for required in [
        "CI_LOCAL_RETENTION",
        "ephemeral",
        "retain",
        "CI_LOCAL_MAX_LOG_BYTES",
        "ci-cleanup-receipt.schema.json",
        "ci-run-aggregate.schema.json",
        "rm -rf -- \"$RUN_DIR\"",
    ] {
        assert!(
            runner_text.contains(required),
            "local runner must define retention contract: {required}"
        );
    }
}

#[test]
fn privileged_workflows_keep_their_permission_contracts() {
    let clippy = parse_workflow("rust-clippy.yml", &workflow_text("rust-clippy.yml"));
    let gitleaks = parse_workflow("gitleaks.yml", &workflow_text("gitleaks.yml"));
    let release = parse_workflow("release-please.yml", &workflow_text("release-please.yml"));

    assert_eq!(
        workflow_job_permissions(&clippy, "rust-clippy-analyze"),
        vec![
            ("actions", "read"),
            ("contents", "read"),
            ("security-events", "write")
        ],
        "clippy workflow must keep its code-scanning permission surface"
    );
    assert_eq!(
        workflow_top_level_permissions(&gitleaks),
        vec![("contents", "read"), ("security-events", "write")],
        "gitleaks workflow must keep its code-scanning permission surface"
    );
    assert_eq!(
        workflow_top_level_permissions(&release),
        vec![
            ("contents", "write"),
            ("issues", "write"),
            ("pull-requests", "write")
        ],
        "release-please workflow must keep its release-automation permission surface"
    );
}

#[test]
fn route_policy_manifest_records_full_mode_escalations() {
    let text = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-route-policy.md"))
        .expect("read route policy manifest");

    for required in [
        "Direct runs on `main`, release refs, and tags force full mode.",
        "base branch alone does not force full mode.",
        "Lockfiles, workflow files, and unknown routes force full mode.",
        "Scheduled release/security/canary lanes",
        "Active docs and shared surfaces force full mode.",
        "docs-only passive paths may narrow only when the active contract surface is not touched",
        "The observe-only artifact publishes the same classification used by the enforced Rust workflow.",
        "unknown, incomplete, or parse-failed route inputs must widen to full mode",
        "Passive docs: docs-only changes that do not touch active contract surfaces.",
        "Active docs: roadmap, standards, guide, BOM",
        "Shared surfaces: `README.md`, `CHANGELOG.md`, `SECURITY.md`",
        "Dependency closure: crate-local changes run the impacted crate and its",
        "Release/mainline: direct execution on `main`, release refs, and tag contexts always stays on full mode.",
    ] {
        assert!(
            text.contains(required),
            "route policy manifest must document {required}"
        );
    }
}

#[test]
fn route_fixture_manifest_covers_common_change_shapes() {
    let text = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-route-fixtures.md"))
        .expect("read route fixture manifest");

    for required in [
        "docs-only",
        "active-docs",
        "Rust crate",
        "dependency-closure",
        "UI",
        "shared-surface",
        "workflow",
        "lockfile",
        "release-mainline",
        "aod",
        "mixed",
        "unknown-file",
        "| active-docs | full_pr_matrix |",
        "| docs-only | passive_docs_only |",
        "\"reason\": \"docs-only passive paths observed\"",
    ] {
        assert!(
            text.contains(required),
            "route fixture manifest must cover {required}"
        );
    }
}

#[test]
fn protected_workflow_contract_table_is_explicit() {
    let text = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-execution-plan.md"))
        .expect("read execution plan");

    for required in [
        "| `rust-clippy.yml` | `security-events: write` on analysis; `if: always()` SARIF upload; fail closed after capturing clippy, converter, formatter, and SARIF statuses. |",
        "| `gitleaks.yml` | `security-events: write`; `if: always()` SARIF upload; fail closed after scanner and SARIF validation. |",
        "| `rust-supply-chain.yml` | Pinned `cargo audit` / `cargo deny` versions; advisories, bans, licenses, and sources fail closed. |",
        "| `tachi-pytest.yml` | Specialist trigger contract for docs-sensitive and compatibility coverage surfaces remains explicit. |",
        "| `tachi-mmdc-preflight.yml` | Missing-renderer contract remains explicit and path-sensitive so template/render checks cannot drift. |",
        "| `rust-feature-coverage-canary.yml` | Manual/scheduled only; not a required PR gate until signal/noise review promotes it. |",
        "| `release-please.yml` | Release automation keeps its write permissions and does not gain PR-writeable shortcut paths. |",
        "| `fuzz-mutation-audit.yml` | Non-blocking manual/scheduled audit lane remains offline-safe and baseline-report driven. |",
    ] {
        assert!(
            text.contains(required),
            "execution plan must document {required}"
        );
    }
}

#[test]
fn required_check_migration_note_is_explicit() {
    let execution_plan =
        fs::read_to_string(repo_root().join("docs/tachi-rust-ci-execution-plan.md"))
            .expect("read execution plan");
    let checklist =
        fs::read_to_string(repo_root().join("docs/publish-readiness-checklist.html.md"))
            .expect("read publish checklist");

    for required in [
        "Old broad-signal checks: `cargo test -p ${{ matrix.package }} --all-targets`",
        "and `cargo test -p tachi-shell (${{ matrix.suite }})`",
        "New stable route checks: `route decision and stable orchestrator check`",
        "`cargo fmt --all -- --check`",
        "`actionlint` parse gate",
        "`rust-clippy analyze`",
        "`cargo audit and cargo deny`",
        "Rollback rule: if route selection misclassifies a protected branch, tag, or",
    ] {
        assert!(
            execution_plan.contains(required),
            "execution plan must document {required}"
        );
    }

    for required in [
        "required-check migration note",
        "old matrix checks",
        "new stable route checks",
        "rollback rule for protected refs",
    ] {
        assert!(
            checklist.contains(required),
            "publish checklist must document {required}"
        );
    }
}

#[test]
fn heavy_rust_workflows_emit_elapsed_runtime_summaries() {
    let workflows = [
        "rust-workspace.yml",
        "rust-clippy.yml",
        "rustfmt.yml",
        "rust-supply-chain.yml",
    ];

    for workflow_name in workflows {
        let text = workflow_text(workflow_name);
        for required in ["GITHUB_STEP_SUMMARY", "elapsed_ms", "date +%s%N"] {
            assert!(
                text.contains(required),
                "{workflow_name} must emit elapsed runtime summaries containing {required}"
            );
        }
    }
}

#[test]
fn baseline_snapshot_records_the_phase_zero_contract() {
    let text = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-baseline.md"))
        .expect("read baseline snapshot");

    for required in [
        "Workflow Inventory",
        "ci-workflow-parse.yml",
        "rust-workspace.yml",
        "Required-Check Map",
        "route decision and stable orchestrator check",
        "Matrix Inventory",
        "tachi-core",
        "shell-init",
        "Local Validation Snapshot",
        "23 tests passed",
        "elapsed runtime summaries",
        "Local Timing Snapshot",
        "real 1.62s",
        "Warm Timing Comparison",
        "`origin/main` warm run: `real 0.58s`",
        "Current branch warm run: `real 1.39s`",
        "Timing Notes",
        "sample_size=22",
        "sample_size=23",
        "timing evidence required by the original baseline plan is",
        "operational monitoring, not a prerequisite",
    ] {
        assert!(
            text.contains(required),
            "baseline snapshot must document {required}"
        );
    }
}

#[test]
fn closeout_notes_separate_proof_from_operational_monitoring() {
    let text = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-closeout.md"))
        .expect("read closeout notes");

    for required in [
        "Proven Locally",
        "Route policy manifest and route artifact contracts exist",
        "Passive-docs narrowing",
        "dependency-closure routing",
        "emergency full-CI",
        "override",
        "Protected refs (`main`, `release/*`, and tags) are forced to full mode.",
        "Shared Rust setup is centralized",
        "Heavy Rust-facing workflows emit elapsed runtime summaries.",
        "Operational Monitoring (post-closeout)",
        "operational monitoring",
        "sample_size=22",
        "sample_size=23",
        "Branch-protection verification",
        "Post-push monitoring of `main` after a publish step",
        "Warm local timing comparison exists",
    ] {
        assert!(
            text.contains(required),
            "closeout notes must document {required}"
        );
    }
}

#[test]
fn rt_ci_latency_evidence_target_is_documented_and_invocable() {
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let baseline = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-baseline.md"))
        .expect("read baseline snapshot");
    let closeout = fs::read_to_string(repo_root().join("docs/tachi-rust-ci-closeout.md"))
        .expect("read closeout notes");
    let script = repo_root()
        .join("scripts")
        .join("rt-ci-latency-evidence.sh");
    let helper = fs::read_to_string(script).expect("read rt-ci-latency-evidence script");

    assert!(
        makefile.contains("rt-ci-latency-evidence"),
        "makefile must include the rt-ci-latency-evidence target"
    );
    assert!(
        makefile.contains("./scripts/rt-ci-latency-evidence.sh"),
        "makefile should call the rt-ci-latency-evidence script"
    );
    assert!(
        baseline.contains("make rt-ci-latency-evidence"),
        "baseline notes must reference the rt-ci-latency-evidence helper"
    );
    assert!(
        closeout.contains("make rt-ci-latency-evidence"),
        "closeout notes must reference the rt-ci-latency-evidence helper"
    );
    assert!(
        helper.contains("run list"),
        "latency evidence helper must document its GitHub run-list query"
    );
    assert!(
        helper.contains("createdAt"),
        "latency evidence helper must request createdAt for queue timing"
    );
    assert!(
        helper.contains("updatedAt"),
        "latency evidence helper must request updatedAt-equivalent timing completion"
    );
    assert!(
        helper.contains("databaseId,conclusion,createdAt"),
        "latency evidence helper must request each run's conclusion"
    );
    assert!(
        helper.contains("select(.conclusion == \"success\")"),
        "latency evidence helper must exclude unsuccessful completed runs"
    );
    assert!(
        makefile.contains("rt-ci-matched-controls"),
        "makefile must expose the matched-control collector"
    );
    assert!(
        makefile.contains("./scripts/rt-ci-matched-controls.sh"),
        "makefile should invoke the matched-control collector"
    );
}

#[test]
fn rt_ci_latency_evidence_excludes_failed_completed_runs() {
    let root = std::env::temp_dir().join(format!(
        "tachi-rt-ci-latency-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    ));
    let bin = root.join("bin");
    fs::create_dir_all(&bin).expect("create fake gh directory");
    let gh = bin.join("gh");
    fs::write(
        &gh,
        r##"#!/bin/sh
set -eu
case "$1" in
  run)
    if [ "${FAKE_GH_ALL_FAILED:-0}" = "1" ]; then
      cat <<'JSON'
[{"databaseId":9,"conclusion":"failure","event":"pull_request","createdAt":"2026-07-10T00:00:00Z","startedAt":"2026-07-10T00:00:10Z","updatedAt":"2026-07-10T00:10:10Z","displayTitle":"failed run","headBranch":"main"}]
JSON
      exit 0
    fi
    cat <<'JSON'
[
  {"databaseId":1,"conclusion":"success","event":"pull_request","createdAt":"2026-07-10T00:00:00Z","startedAt":"2026-07-10T00:00:10Z","updatedAt":"2026-07-10T00:01:10Z","displayTitle":"success one","headBranch":"main"},
  {"databaseId":2,"conclusion":"failure","event":"pull_request","createdAt":"2026-07-10T00:00:00Z","startedAt":"2026-07-10T00:00:10Z","updatedAt":"2026-07-10T00:10:10Z","displayTitle":"failed slow run","headBranch":"main"},
  {"databaseId":3,"conclusion":"success","event":"pull_request","createdAt":"2026-07-10T00:00:00Z","startedAt":"2026-07-10T00:00:20Z","updatedAt":"2026-07-10T00:02:20Z","displayTitle":"success two","headBranch":"main"}
]
JSON
    ;;
  repo)
    printf 'pratik-saptarshi/tachi-rust\n'
    ;;
  api)
    printf '{"required_status_checks":{"contexts":[]}}\n'
    ;;
  *)
    echo "unexpected gh command: $*" >&2
    exit 2
    ;;
esac
"##,
    )
    .expect("write fake gh executable");
    let mut permissions = fs::metadata(&gh)
        .expect("read fake gh metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&gh, permissions).expect("make fake gh executable");

    let original_path = std::env::var_os("PATH").unwrap_or_default();
    let path = format!("{}:{}", bin.display(), original_path.to_string_lossy());
    let script = repo_root().join("scripts/rt-ci-latency-evidence.sh");
    let run_collector = |all_failed: bool| {
        Command::new("bash")
            .arg(&script)
            .args(["rust-workspace.yml", "main", "40", ""])
            .env("PATH", &path)
            .env("GH_REPO", "pratik-saptarshi/tachi-rust")
            .env("FAKE_GH_ALL_FAILED", if all_failed { "1" } else { "0" })
            .output()
            .expect("run the latency collector with fake GitHub responses")
    };
    let output = run_collector(false);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "collector should succeed; stdout={stdout}; stderr={stderr}"
    );
    assert!(stdout.contains("sample_size=2"), "stdout={stdout}");
    assert!(stdout.contains("run_med_ms=90000"), "stdout={stdout}");
    assert!(stdout.contains("queue_med_ms=15000"), "stdout={stdout}");

    let empty = run_collector(true);
    let empty_stderr = String::from_utf8_lossy(&empty.stderr);
    assert!(empty.status.success(), "stderr={empty_stderr}");
    assert!(
        empty_stderr.contains("No successful completed runs found"),
        "stderr={empty_stderr}"
    );
    fs::remove_dir_all(&root).expect("remove fake gh directory");
}

#[test]
fn matched_control_collector_contract_captures_provenance_and_stays_read_only() {
    let workflow = workflow_text("rust-workspace.yml");
    let collector = fs::read_to_string(repo_root().join("scripts/rt-ci-matched-controls.sh"))
        .expect("read matched-control collector");
    let matcher = fs::read_to_string(repo_root().join("scripts/rt-ci-matched-controls.jq"))
        .expect("read matched-control matcher");

    for required in [
        "workflow_dispatch:",
        "force_full_ci:",
        "type: boolean",
        "permissions:",
        "contents: read",
        "name: route-decision",
        "if-no-files-found: error",
        "github.workflow_sha",
        "inputs.control_tree_sha",
        "inputs.control_pr_number",
        "execution_tree_sha",
        "needs.route.outputs.execution_sha",
        "control_pr_head_sha",
        "pull-requests: read",
        "DEFAULT_BRANCH",
        "refs/heads/${DEFAULT_BRANCH}",
        "matched controls must dispatch the trusted default-branch workflow",
        "persist-credentials: false",
        "--format=%P",
        "workflow_file_revision",
        "runner-provenance-",
        "ImageVersion",
        "GITHUB_RUN_ATTEMPT",
        "runner_definition",
    ] {
        assert!(
            workflow.contains(required),
            "workflow must include {required}"
        );
    }
    assert_eq!(
        workflow.matches("refs/pull/{0}/merge").count(),
        1,
        "only the route job may resolve the mutable PR merge ref"
    );
    assert_eq!(
        workflow
            .matches("ref: ${{ needs.route.outputs.execution_sha }}")
            .count(),
        3,
        "all three measured downstream jobs must use the route job's immutable SHA"
    );
    for required in [
        "gh run list",
        "gh run download",
        "force_full_requested",
        "workflow_file_revision",
        "execution_sha",
        "control_tree_sha",
        "control_pr_number",
        "control_pr_head_sha",
        "classifier_revision",
        "path_producer_revision",
        "runner_definition",
        "tree_sha",
        "fromdateiso8601",
        "rejection_reason",
        "rt-ci-matched-controls.jq",
    ] {
        assert!(
            collector.contains(required),
            "collector must include {required}"
        );
    }
    for required in [
        "run_attempt == 1",
        "conclusion == \"success\"",
        "$pr.execution_sha == $control.execution_sha",
        "$pr.tree_sha == $control.tree_sha",
        "$pr.workflow_file_revision == $control.workflow_file_revision",
        "$pr.classifier_revision == $control.classifier_revision",
        "$pr.path_producer_revision == $control.path_producer_revision",
        "$pr.runner_inventory",
        "$control.runner_inventory",
        "passive-docs",
        "dependency-closure",
        "optimized_execution_median_max_ratio: 0.65",
    ] {
        assert!(
            matcher.contains(required),
            "matcher must include {required}"
        );
    }
}

#[test]
fn matched_control_fixtures_exclude_failures_reruns_missing_routes_and_mismatches() {
    let root = std::env::temp_dir().join(format!(
        "tachi-rt-ci-matched-fixtures-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&root).expect("create fixture directory");

    let candidate = |run_id: &str,
                     event: &str,
                     conclusion: &str,
                     attempt: u64,
                     route_mode: &str,
                     tree_sha: &str,
                     workflow_file_revision: &str,
                     runner_definition: &str,
                     force_full: bool,
                     eligible: bool,
                     queue_ms: u64,
                     execution_ms: u64| {
        serde_json::json!({
            "run_id": run_id,
            "run_attempt": attempt,
            "event": event,
            "conclusion": conclusion,
            "route_mode": route_mode,
            "tree_sha": tree_sha,
            "execution_sha": if run_id == "14" {
                "different-commit".to_owned()
            } else {
                format!("commit-{tree_sha}")
            },
            "control_tree_sha": if event == "workflow_dispatch" {
                if run_id == "14" {
                    "different-commit".to_owned()
                } else {
                    format!("commit-{tree_sha}")
                }
            } else {
                String::new()
            },
            "control_pr_number": if event == "workflow_dispatch" { 42 } else { 0 },
            "control_pr_head_sha": if event == "workflow_dispatch" { format!("control-head-{run_id}") } else { String::new() },
            "workflow_file_revision": workflow_file_revision,
            "classifier_revision": "classifier-1",
            "path_producer_revision": "path-producer-1",
            "runner_definition": runner_definition,
            "runner_inventory": if run_id == "3" {
                serde_json::json!({"route": runner_definition, "cargo-test-tachi-core": "ubuntu-latest/image-a"})
            } else if run_id == "4" {
                serde_json::json!({"route": runner_definition, "cargo-test-tachi-core": "ubuntu-latest/image-b"})
            } else {
                serde_json::json!({"route": runner_definition})
            },
            "force_full_requested": force_full,
            "eligible": eligible,
            "pr_number": 42,
            "head_sha": format!("head-{run_id}"),
            "created_at": "2026-10-06T00:00:00Z",
            "started_at": "2026-10-06T00:00:10Z",
            "completed_at": "2026-10-06T00:01:10Z",
            "queue_duration_ms": queue_ms,
            "execution_duration_ms": execution_ms
        })
    };
    let candidates = serde_json::json!([
        candidate(
            "1",
            "pull_request",
            "success",
            1,
            "passive_docs_only",
            "tree-a",
            "blob-a",
            "ubuntu-latest",
            false,
            true,
            10,
            500
        ),
        candidate(
            "2",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-a",
            "blob-a",
            "ubuntu-latest",
            true,
            true,
            20,
            1000
        ),
        candidate(
            "3",
            "pull_request",
            "success",
            1,
            "dependency_closure",
            "tree-b",
            "blob-a",
            "ubuntu-latest",
            false,
            true,
            15,
            700
        ),
        candidate(
            "4",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-b",
            "blob-a",
            "ubuntu-latest",
            true,
            true,
            25,
            1000
        ),
        candidate(
            "5",
            "pull_request",
            "failure",
            1,
            "passive_docs_only",
            "tree-c",
            "blob-a",
            "ubuntu-latest",
            false,
            true,
            1,
            9999
        ),
        candidate(
            "6",
            "pull_request",
            "cancelled",
            1,
            "dependency_closure",
            "tree-d",
            "blob-a",
            "ubuntu-latest",
            false,
            true,
            1,
            9999
        ),
        candidate(
            "7",
            "pull_request",
            "success",
            2,
            "passive_docs_only",
            "tree-e",
            "blob-a",
            "ubuntu-latest",
            false,
            true,
            1,
            9999
        ),
        candidate(
            "8",
            "pull_request",
            "success",
            1,
            "passive_docs_only",
            "tree-f",
            "blob-a",
            "ubuntu-latest",
            false,
            false,
            1,
            9999
        ),
        candidate(
            "9",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-a",
            "wf-other",
            "ubuntu-latest",
            true,
            true,
            1,
            9999
        ),
        candidate(
            "10",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-a",
            "blob-a",
            "macos-latest",
            true,
            true,
            1,
            9999
        ),
        candidate(
            "11",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-g",
            "wf-a",
            "ubuntu-latest",
            false,
            true,
            1,
            9999
        ),
        candidate(
            "12",
            "pull_request",
            "success",
            1,
            "dependency_closure",
            "tree-z",
            "blob-a",
            "ubuntu-latest",
            false,
            true,
            15,
            700
        ),
        candidate(
            "13",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-z",
            "blob-a",
            "ubuntu-latest",
            true,
            true,
            25,
            1000
        ),
        candidate(
            "14",
            "workflow_dispatch",
            "success",
            1,
            "full_pr_matrix",
            "tree-a",
            "blob-a",
            "ubuntu-latest",
            true,
            true,
            20,
            1000
        )
    ]);
    let input = root.join("candidates.json");
    fs::write(
        &input,
        serde_json::to_vec(&serde_json::json!({ "candidates": candidates }))
            .expect("serialize candidates"),
    )
    .expect("write candidates");

    let output = Command::new("jq")
        .arg("-f")
        .arg(repo_root().join("scripts/rt-ci-matched-controls.jq"))
        .arg(&input)
        .output()
        .expect("run matched-control fixture matcher");
    assert!(
        output.status.success(),
        "jq matcher failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
            panic!(
                "report JSON: {error}; stdout={}; stderr={}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        });
    assert_eq!(report["pairs"].as_array().unwrap().len(), 2);
    assert_eq!(report["summary"][0]["valid_pairs"], 1);
    assert_eq!(report["summary"][0]["routed_execution_median_ms"], 500);
    assert_eq!(
        report["summary"][0]["full_control_execution_median_ms"],
        1000
    );
    assert_eq!(report["summary"][0]["status"], "insufficient_pairs");
    assert_eq!(report["summary"][1]["valid_pairs"], 1);
    assert!(report["pairs"]
        .as_array()
        .unwrap()
        .iter()
        .any(|pair| pair["tree_sha"] == "tree-z"));
    assert!(!report["pairs"]
        .as_array()
        .unwrap()
        .iter()
        .any(|pair| pair["tree_sha"] == "tree-b"));
    let passive_pair = report["pairs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|pair| pair["tree_sha"] == "tree-a")
        .expect("matched passive-docs pair");
    assert_eq!(passive_pair["execution_sha"], "commit-tree-a");
    assert_eq!(passive_pair["control_pr_number"], 42);
    assert_eq!(passive_pair["control_tree_sha"], "commit-tree-a");
    assert_eq!(passive_pair["control_pr_head_sha"], "control-head-2");
    assert!(report["candidate_dispositions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|candidate| candidate["run_id"] == "14" && candidate["disposition"] == "unmatched"));
    fs::remove_dir_all(&root).expect("remove fixture directory");
}

#[test]
fn matched_control_collector_handles_run_api_and_artifact_fixtures() {
    let root = std::env::temp_dir().join(format!(
        "tachi-rt-ci-matched-collector-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    ));
    let bin = root.join("bin");
    let fixtures = root.join("artifacts");
    fs::create_dir_all(&bin).expect("create fake command directory");
    fs::create_dir_all(&fixtures).expect("create artifact fixture directory");
    let runs = serde_json::json!([
        {"databaseId":1,"attempt":1,"conclusion":"success","event":"pull_request","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:10Z","updatedAt":"2026-10-06T00:01:10Z","headBranch":"route-pr","url":"https://example.test/run/1"},
        {"databaseId":2,"attempt":1,"conclusion":"success","event":"workflow_dispatch","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:20Z","updatedAt":"2026-10-06T00:02:20Z","headBranch":"route-pr","url":"https://example.test/run/2"},
        {"databaseId":3,"attempt":1,"conclusion":"failure","event":"pull_request","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:10Z","updatedAt":"2026-10-06T00:09:10Z","headBranch":"failed-pr","url":"https://example.test/run/3"},
        {"databaseId":4,"attempt":1,"conclusion":"cancelled","event":"pull_request","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:10Z","updatedAt":"2026-10-06T00:09:10Z","headBranch":"cancelled-pr","url":"https://example.test/run/4"},
        {"databaseId":5,"attempt":2,"conclusion":"success","event":"pull_request","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:10Z","updatedAt":"2026-10-06T00:09:10Z","headBranch":"rerun-pr","url":"https://example.test/run/5"},
        {"databaseId":6,"attempt":1,"conclusion":"success","event":"pull_request","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:10Z","updatedAt":"2026-10-06T00:01:10Z","headBranch":"no-artifact-pr","url":"https://example.test/run/6"},
        {"databaseId":7,"attempt":1,"conclusion":"success","event":"pull_request","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:10Z","updatedAt":"2026-10-06T00:01:10Z","headBranch":"mismatch-pr","url":"https://example.test/run/7"},
        {"databaseId":8,"attempt":1,"conclusion":"success","event":"workflow_dispatch","createdAt":"2026-10-06T00:00:00Z","startedAt":"2026-10-06T00:00:20Z","updatedAt":"2026-10-06T00:02:20Z","headBranch":"mismatch-pr","url":"https://example.test/run/8"}
    ]);
    let runs_file = root.join("runs.json");
    fs::write(&runs_file, serde_json::to_vec(&runs).unwrap()).expect("write fake run list");

    let gh = bin.join("gh");
    fs::write(
        &gh,
        r##"#!/bin/sh
set -eu
case "$1 $2" in
  "repo view")
    printf 'pratik-saptarshi/tachi-rust\n'
    ;;
  "run list")
    cat "$FAKE_RUNS_JSON"
    ;;
  "run download")
    run_id="$3"
    dir="$7"
    mkdir -p "$dir"
    if [ "$4" = "--name" ]; then
      source="$FAKE_ARTIFACTS/$run_id/route.json"
      [ -s "$source" ] || exit 1
      cp "$source" "$dir/route.json"
    elif [ "$4" = "--pattern" ]; then
      source="$FAKE_ARTIFACTS/$run_id/provenance/runner.json"
      [ -s "$source" ] || exit 1
      mkdir -p "$dir/runner-provenance"
      cp "$source" "$dir/runner-provenance/runner.json"
    else
      exit 2
    fi
    ;;
  api\ *)
    case "$2" in
      */head-pr) printf 'tree-a\n' ;;
      */head-control) printf 'tree-a\n' ;;
      */head-mismatch-pr) printf 'tree-b\n' ;;
      */head-mismatch-control) printf 'tree-c\n' ;;
      *) exit 2 ;;
    esac
    ;;
  *)
    echo "unexpected gh invocation: $*" >&2
    exit 2
    ;;
esac
"##,
    )
    .expect("write fake gh");
    let mut permissions = fs::metadata(&gh)
        .expect("read fake gh metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&gh, permissions).expect("make fake gh executable");

    let write_route = |run_id: &str,
                       event: &str,
                       mode: &str,
                       head_sha: &str,
                       tree_sha: &str,
                       execution_sha: &str,
                       control_tree_sha: &str,
                       force_full: bool,
                       pr_number: u64| {
        let dir = fixtures.join(run_id);
        fs::create_dir_all(&dir).expect("create fake run artifact directory");
        fs::write(
            dir.join("route.json"),
            serde_json::to_vec(&serde_json::json!({
                "run_id": run_id,
                "run_attempt": 1,
                "event": event,
                "mode": mode,
                "reason": "fixture route",
                "head_sha": head_sha,
                "workflow_file_revision": "workflow-blob-1",
                "execution_workflow_file_revision": "workflow-blob-1",
                "execution_sha": execution_sha,
                "tree_sha": tree_sha,
                "control_tree_sha": control_tree_sha,
                "control_pr_number": if event == "workflow_dispatch" { 42 } else { 0 },
                "control_pr_head_sha": if event == "workflow_dispatch" { "head-control" } else { "" },
                "classifier_revision": "classifier-1",
                "path_producer_revision": "path-producer-1",
                "runner_definition": "ubuntu-latest",
                "force_full_requested": force_full,
                "pr_number": pr_number
            }))
            .expect("serialize route artifact"),
        )
        .expect("write route artifact");
        if run_id != "1" {
            let provenance = dir.join("provenance");
            fs::create_dir_all(&provenance).expect("create runner provenance directory");
            fs::write(
                provenance.join("runner.json"),
                serde_json::to_vec(&serde_json::json!({
                    "job_key": "cargo-test-tachi-core",
                    "runner_definition": if run_id == "8" { "ubuntu-latest/ubuntu24/image-b" } else { "ubuntu-latest/ubuntu24/image-a" }
                }))
                .expect("serialize runner provenance"),
            )
            .expect("write runner provenance");
        }
    };
    write_route(
        "1",
        "pull_request",
        "passive_docs_only",
        "head-pr",
        "tree-a",
        "merge-control-a",
        "",
        false,
        101,
    );
    write_route(
        "2",
        "workflow_dispatch",
        "full_pr_matrix",
        "head-control",
        "tree-a",
        "merge-control-a",
        "merge-control-a",
        true,
        0,
    );
    write_route(
        "7",
        "pull_request",
        "dependency_closure",
        "head-mismatch-pr",
        "tree-b",
        "merge-pr-b",
        "",
        false,
        202,
    );
    write_route(
        "8",
        "workflow_dispatch",
        "full_pr_matrix",
        "head-mismatch-control",
        "tree-c",
        "merge-control-c",
        "merge-control-c",
        true,
        0,
    );

    let original_path = std::env::var_os("PATH").unwrap_or_default();
    let path = format!("{}:{}", bin.display(), original_path.to_string_lossy());
    let output = Command::new("bash")
        .arg(repo_root().join("scripts/rt-ci-matched-controls.sh"))
        .args(["rust-workspace.yml", "100"])
        .env("PATH", path)
        .env("FAKE_RUNS_JSON", runs_file)
        .env("FAKE_ARTIFACTS", &fixtures)
        .output()
        .expect("run matched-control collector with fake GitHub responses");
    assert!(
        output.status.success(),
        "collector failed: stdout={}; stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
            panic!(
                "collector report JSON: {error}; stdout={}; stderr={}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        });
    assert_eq!(report["pairs"].as_array().unwrap().len(), 1);
    assert_eq!(report["pairs"][0]["tree_sha"], "tree-a");
    assert_eq!(report["pairs"][0]["execution_sha"], "merge-control-a");
    assert_eq!(report["pairs"][0]["control_pr_number"], 42);
    assert_eq!(report["pairs"][0]["control_tree_sha"], "merge-control-a");
    assert_eq!(report["pairs"][0]["control_pr_head_sha"], "head-control");
    assert_eq!(report["summary"][0]["valid_pairs"], 1);
    assert_eq!(report["summary"][0]["status"], "insufficient_pairs");
    let candidates = report["candidate_runs"].as_array().unwrap();
    for (run_id, expected) in [
        ("3", "run conclusion is not success"),
        ("4", "run conclusion is not success"),
        ("5", "rerun attempt is ambiguous"),
        ("6", "route decision artifact is missing or unavailable"),
    ] {
        assert!(
            candidates.iter().any(|candidate| {
                candidate["run_id"] == run_id && candidate["rejection_reason"] == expected
            }),
            "candidate {run_id} should be rejected as {expected}"
        );
    }
    assert!(report["candidate_dispositions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|candidate| candidate["run_id"] == "7"
            && candidate["disposition"] == "unmatched"
            && candidate["reason"]
                == "no compatible opposite-event run or this tree was already counted"));
    fs::remove_dir_all(&root).expect("remove fake collector directory");
}

#[test]
fn shared_rust_setup_action_is_reused_across_rust_workflows() {
    let action = fs::read_to_string(repo_root().join(".github/actions/rust-setup/action.yml"))
        .expect("read shared rust setup action");

    for required in [
        "Install pinned Rust toolchain",
        "Cache Rust dependencies",
        "Print Rust toolchain proof",
        "include-rustfmt-proof",
        "use-cache",
        "if: inputs.use-cache == 'true'",
        "rustup which rustfmt",
    ] {
        assert!(
            action.contains(required),
            "shared rust setup action must document {required}"
        );
    }

    for workflow_name in [
        "rust-workspace.yml",
        "rust-clippy.yml",
        "rustfmt.yml",
        "rust-supply-chain.yml",
    ] {
        let text = workflow_text(workflow_name);
        let expected_path = if workflow_name == "rust-workspace.yml" {
            "./.ci-trusted/.github/actions/rust-setup"
        } else {
            "./.github/actions/rust-setup"
        };
        assert!(
            text.contains(expected_path),
            "{workflow_name} must reuse the shared rust setup action from {expected_path}"
        );
    }
    let workflow = workflow_text("rust-workspace.yml");
    assert_eq!(workflow.matches("persist-credentials: false").count(), 7);
    assert_eq!(
        workflow
            .matches("./.ci-trusted/.github/actions/rust-setup")
            .count(),
        3
    );
    assert_eq!(workflow.matches("use-cache: ${{ github.event_name != 'workflow_dispatch' || inputs.control_pr_number == 0 }}").count(), 3);
}

#[test]
fn route_observe_workflow_emits_route_artifact_and_stable_check() {
    let text = workflow_text("ci-route-observe.yml");
    let workflow = parse_workflow("ci-route-observe.yml", &text);
    let classifier = fs::read_to_string(repo_root().join("scripts/ci-route-classifier.sh"))
        .expect("read shared CI route classifier");
    let artifact_schema =
        fs::read_to_string(repo_root().join("docs/tachi-rust-ci-route-artifact.md"))
            .expect("read route artifact manifest");

    assert!(
        workflow_declares_event(&workflow, "pull_request"),
        "route observe workflow must run on pull_request"
    );
    assert!(
        workflow_declares_only_events(&workflow, ["pull_request"].as_slice()),
        "route observe workflow must keep a single pull_request trigger"
    );
    assert!(
        workflow_job_name(&workflow, "route-observe")
            == Some("route decision artifact and stable orchestrator check"),
        "route observe job must stay the required orchestrator check"
    );
    assert_eq!(
        workflow_step_field(&workflow, "Upload route decision artifact", "if"),
        Some("env.ACT_SMOKE != 'true'"),
        "local act runs must not call the hosted artifact service"
    );
    assert_eq!(
        workflow_step_field(&workflow, "Validate local act route artifact", "if"),
        Some("env.ACT_SMOKE == 'true'"),
        "local act runs must validate the generated route artifact in-container"
    );
    assert!(
        workflow_run_bodies(&workflow)
            .any(|run| run.contains("local act route artifact validated")),
        "local act validation step must emit a deterministic success marker"
    );
    assert!(
        workflow_step_field(&workflow, "Upload route decision artifact", "uses")
            == Some("actions/upload-artifact@v7"),
        "route observe workflow must upload the route artifact"
    );
    assert_workflow_has_run_line(&workflow, "cat route.json");
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("scripts/ci-route-classifier.sh")),
        "route observe workflow must consume the shared route classifier"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("scripts/ci-route-changed-paths.sh")),
        "route observe workflow must use the shared rename-aware changed-path producer"
    );
    assert!(
        !text.contains("git diff --name-only"),
        "route observe workflow must not discard rename source paths"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("selected_lanes_json")),
        "route observe workflow must capture selected lanes"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("repository_contracts_required")),
        "route artifact must expose whether repository-wide contracts are selected"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("escalation_reasons_json")),
        "route observe workflow must capture escalation reasons"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("policy_version")),
        "route observe workflow must capture a policy version"
    );
    assert!(
        classifier.contains("docs-only passive paths observed"),
        "shared route classifier must keep passive docs separate"
    );
    assert!(
        workflow_run_bodies(&workflow).any(|run| run.contains("route_mode")),
        "route artifact must expose the classifier's selected mode"
    );
    assert!(
        classifier.contains("unknown non-docs paths stay full mode"),
        "shared route classifier must widen unknown non-doc routes"
    );
    assert!(
        classifier.contains("active docs or shared surface touched"),
        "shared route classifier must widen active docs and shared surfaces"
    );
    assert!(
        classifier.contains("protected ref stays full mode"),
        "shared route classifier must keep direct protected refs in full mode"
    );

    for required in [
        "\"mode\":",
        "\"route_mode\":",
        "\"selected_packages\":",
        "\"repository_contracts_required\":",
        "\"changed_paths\":",
        "\"selected_lanes\":",
        "\"escalation_reasons\":",
        "\"policy_version\":",
        "stable orchestrator check",
    ] {
        assert!(
            artifact_schema.contains(required),
            "route artifact manifest must document {required}"
        );
    }
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
        "@$(MAKE) gitleaks-gate",
        "@$(MAKE) llvm-cov-nightly-branch",
    ] {
        assert!(
            text.contains(command),
            "publish gate must include {command}"
        );
    }
    assert_workflow_uses_pinned_repo_toolchain("rust-supply-chain.yml", &workflow_text);
    for command in [
        "cargo install --locked --version 0.22.2 cargo-audit",
        "cargo install --locked --version 0.20.2 cargo-deny",
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
        "cargo install --locked --version 0.9.1 cargo-llvm-cov",
        "cargo hack --version",
        "cargo llvm-cov --version",
        "cargo hack --version | grep -qx 'cargo-hack 0.6.45'",
        "cargo llvm-cov --version | grep -qx 'cargo-llvm-cov 0.9.1'",
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
        "cargo llvm-cov --version | grep -qx 'cargo-llvm-cov 0.9.1'",
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

#[test]
fn nightly_branch_coverage_gate_is_checked_in_and_fail_closed() {
    let makefile = fs::read_to_string(repo_root().join("Makefile")).expect("read Makefile");
    let script = fs::read_to_string(repo_root().join("scripts/llvm-cov-nightly-branch.sh"))
        .expect("read nightly branch coverage script");

    assert!(makefile.contains("llvm-cov-nightly-branch:"));
    assert!(makefile.contains("./scripts/llvm-cov-nightly-branch.sh"));
    assert!(script.contains("--branch"));
    assert!(script.contains("--summary-only"));
    assert!(script.contains("RUSTC"));
    assert!(script.contains("RUSTDOC"));
    assert!(script.contains("LLVM_COV"));
    assert!(script.contains("LLVM_PROFDATA"));
    assert!(script.contains("1.99.0"));
    assert!(script.contains("85"));
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
        (
            "rust-clippy.yml",
            ["push", "pull_request", "schedule"].as_slice(),
        ),
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
        "channel = \"1.99.0\"",
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
        workspace_manifest.contains("rust-version = \"1.99\""),
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
        if !steps
            .iter()
            .any(|step| workflow_step_run(step).is_some_and(|run| run.contains("cargo ")))
        {
            continue;
        }
        if job_uses_shared_rust_setup(steps) {
            assert_shared_rust_setup_action();
            continue;
        }
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

fn job_uses_shared_rust_setup(steps: &[serde_yaml::Value]) -> bool {
    steps.iter().any(|step| {
        step.get("uses")
            .and_then(serde_yaml::Value::as_str)
            .is_some_and(|uses| uses.ends_with(".github/actions/rust-setup"))
    })
}

fn assert_shared_rust_setup_action() {
    let action = fs::read_to_string(repo_root().join(".github/actions/rust-setup/action.yml"))
        .expect("read shared rust setup action");
    for required in [
        "Install pinned Rust toolchain",
        "Cache Rust dependencies",
        "Print Rust toolchain proof",
        "include-rustfmt-proof",
        "use-cache:",
        "if: inputs.use-cache == 'true'",
    ] {
        assert!(
            action.contains(required),
            "shared rust setup action must include {required}"
        );
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

fn workflow_job_permissions<'a>(
    workflow: &'a serde_yaml::Value,
    job_name: &'a str,
) -> Vec<(&'a str, &'a str)> {
    let Some(permissions) = workflow_job(workflow, job_name)
        .get("permissions")
        .and_then(serde_yaml::Value::as_mapping)
    else {
        return vec![];
    };

    let mut out = BTreeMap::new();
    for (key, value) in permissions {
        let Some(key) = key.as_str() else {
            continue;
        };
        let Some(value) = value.as_str() else {
            continue;
        };
        out.insert(key, value);
    }

    out.into_iter().collect()
}

fn workflow_top_level_permissions(workflow: &serde_yaml::Value) -> Vec<(&str, &str)> {
    let Some(permissions) = workflow
        .get("permissions")
        .and_then(serde_yaml::Value::as_mapping)
    else {
        return vec![];
    };

    let mut out = BTreeMap::new();
    for (key, value) in permissions {
        let Some(key) = key.as_str() else {
            continue;
        };
        let Some(value) = value.as_str() else {
            continue;
        };
        out.insert(key, value);
    }

    out.into_iter().collect()
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

fn workflow_job_field<'a>(
    workflow: &'a serde_yaml::Value,
    job_name: &str,
    field: &str,
) -> Option<&'a str> {
    workflow_job(workflow, job_name)
        .get(field)
        .and_then(serde_yaml::Value::as_str)
}

fn workflow_has_cancelling_concurrency(workflow: &serde_yaml::Value) -> bool {
    let Some(concurrency) = workflow
        .get("concurrency")
        .and_then(serde_yaml::Value::as_mapping)
    else {
        return false;
    };

    let group = workflow_mapping_value(concurrency, "group").and_then(serde_yaml::Value::as_str);
    let cancel = workflow_mapping_value(concurrency, "cancel-in-progress")
        .and_then(serde_yaml::Value::as_bool);

    group.is_some_and(|group| group.contains("github.workflow") && group.contains("github.ref"))
        && cancel == Some(true)
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
