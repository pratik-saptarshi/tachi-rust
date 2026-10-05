use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_TEMP_DIR: AtomicUsize = AtomicUsize::new(0);

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "tachi-ci-route-classifier-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create route classifier test directory");
        Self(path)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn classify(
    temp: &TestDir,
    event: &str,
    ref_name: &str,
    changed_paths: &[&str],
    force_input: &str,
    force_var: &str,
) -> Value {
    let changed_paths_file = temp.0.join("changed-paths.txt");
    fs::write(
        &changed_paths_file,
        changed_paths
            .iter()
            .map(|path| format!("{path}\n"))
            .collect::<String>(),
    )
    .expect("write changed paths");

    let classifier = repo_root().join("scripts/ci-route-classifier.sh");
    let output = Command::new("bash")
        .arg(classifier)
        .arg(event)
        .arg(ref_name)
        .arg(changed_paths_file)
        .arg(force_input)
        .arg(force_var)
        .output()
        .expect("run CI route classifier");
    assert!(
        output.status.success(),
        "classifier failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("classifier emits stable JSON")
}

#[test]
fn main_target_passive_docs_use_the_narrow_route() {
    let temp = TestDir::new();
    let route = classify(
        &temp,
        "pull_request",
        "refs/pull/17/merge",
        &["docs/reference/cli-usage.md"],
        "false",
        "false",
    );

    assert_eq!(route["mode"], "passive_docs_only");
    assert_eq!(route["reason"], "docs-only passive paths observed");
    assert_eq!(route["changed_paths"][0], "docs/reference/cli-usage.md");
}

#[test]
fn main_target_crate_changes_select_the_complete_dependency_closure() {
    let temp = TestDir::new();
    let route = classify(
        &temp,
        "pull_request",
        "refs/pull/18/merge",
        &["crates/tachi-shell/src/lib.rs"],
        "false",
        "false",
    );

    assert_eq!(route["mode"], "dependency_closure");
    assert_eq!(
        route["packages"],
        serde_json::json!(["tachi-cli", "tachi-desktop", "tachi-shell"])
    );

    let core_route = classify(
        &temp,
        "pull_request",
        "refs/pull/24/merge",
        &[
            "crates/tachi-core/Cargo.toml",
            "crates/tachi-core/src/lib.rs",
        ],
        "false",
        "false",
    );
    assert_eq!(core_route["mode"], "dependency_closure");
    assert_eq!(
        core_route["packages"],
        serde_json::json!([
            "tachi-cli",
            "tachi-core",
            "tachi-desktop",
            "tachi-mcp",
            "tachi-shell"
        ])
    );

    let mixed_docs_route = classify(
        &temp,
        "pull_request",
        "refs/pull/25/merge",
        &["docs/reference/cli-usage.md", "crates/tachi-mcp/src/lib.rs"],
        "false",
        "false",
    );
    assert_eq!(mixed_docs_route["mode"], "dependency_closure");
    assert_eq!(
        mixed_docs_route["packages"],
        serde_json::json!(["tachi-mcp"])
    );
}

#[test]
fn active_unknown_protected_and_override_routes_stay_full() {
    let temp = TestDir::new();
    for (event, ref_name, changed_paths, force_input, force_var, reason) in [
        (
            "pull_request",
            "refs/pull/19/merge",
            vec!["docs/roadmap/plan.md"],
            "false",
            "false",
            "active docs or shared surface touched",
        ),
        (
            "pull_request",
            "refs/pull/20/merge",
            vec!["scripts/unclassified-tool.sh"],
            "false",
            "false",
            "unknown non-docs paths stay full mode",
        ),
        (
            "pull_request",
            "refs/pull/26/merge",
            vec![
                "crates/tachi-mcp/src/lib.rs",
                "scripts/unclassified-tool.sh",
            ],
            "false",
            "false",
            "unknown non-docs paths stay full mode",
        ),
        (
            "push",
            "refs/heads/main",
            vec!["docs/reference/cli-usage.md"],
            "false",
            "false",
            "protected ref stays full mode",
        ),
        (
            "push",
            "refs/tags/v1.0.0",
            vec!["docs/reference/cli-usage.md"],
            "false",
            "false",
            "protected ref stays full mode",
        ),
        (
            "pull_request",
            "refs/pull/21/merge",
            vec!["docs/reference/cli-usage.md"],
            "true",
            "false",
            "emergency full-ci override (workflow_dispatch input)",
        ),
        (
            "pull_request",
            "refs/pull/22/merge",
            vec!["docs/reference/cli-usage.md"],
            "false",
            "true",
            "emergency full-ci override (repo variable)",
        ),
        (
            "pull_request",
            "refs/pull/23/merge",
            vec![],
            "false",
            "false",
            "empty or unavailable diff stays full mode",
        ),
    ] {
        let route = classify(
            &temp,
            event,
            ref_name,
            &changed_paths,
            force_input,
            force_var,
        );
        assert_eq!(route["mode"], "full_pr_matrix", "{reason}");
        assert_eq!(route["reason"], reason);
    }
}
