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

fn run_git(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .expect("run git command for route path fixture");
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("git output is UTF-8")
        .trim()
        .to_owned()
}

fn changed_paths_between(repo: &Path, base: &str, head: &str) -> Vec<String> {
    let output = Command::new("bash")
        .arg(repo_root().join("scripts/ci-route-changed-paths.sh"))
        .arg(base)
        .arg(head)
        .current_dir(repo)
        .output()
        .expect("run rename-aware changed-path producer");
    assert!(
        output.status.success(),
        "changed-path producer failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("changed paths are UTF-8")
        .lines()
        .map(str::to_owned)
        .collect()
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
fn tdd_evidence_json_routes_to_its_active_contract() {
    let temp = TestDir::new();
    let route = classify(
        &temp,
        "pull_request",
        "refs/pull/28/merge",
        &["docs/testing/tdd-evidence.json"],
        "false",
        "false",
    );

    assert_eq!(route["mode"], "full_pr_matrix");
    assert_eq!(route["reason"], "active docs or shared surface touched");
}

#[test]
fn renamed_crate_to_passive_docs_retains_both_endpoints_and_widens_route() {
    let temp = TestDir::new();
    let repo = temp.0.join("rename-repo");
    fs::create_dir_all(repo.join("crates/tachi-core/src")).expect("create source tree");
    run_git(&repo, &["init", "--quiet"]);
    run_git(&repo, &["config", "user.name", "Route Test"]);
    run_git(
        &repo,
        &["config", "user.email", "route-test@example.invalid"],
    );

    let old_path = "crates/tachi-core/src/rename_probe.rs";
    let new_path = "docs/reference/rename_probe.md";
    fs::write(repo.join(old_path), "pub fn route_probe() {}\n").expect("write source fixture");
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "--quiet", "-m", "base"]);
    let base = run_git(&repo, &["rev-parse", "HEAD"]);

    fs::create_dir_all(repo.join("docs/reference")).expect("create docs tree");
    fs::rename(repo.join(old_path), repo.join(new_path)).expect("rename source into docs");
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "--quiet", "-m", "rename"]);
    let head = run_git(&repo, &["rev-parse", "HEAD"]);

    let changed_paths = changed_paths_between(&repo, &base, &head);
    assert!(changed_paths.iter().any(|path| path == old_path));
    assert!(changed_paths.iter().any(|path| path == new_path));

    let changed_path_refs = changed_paths.iter().map(String::as_str).collect::<Vec<_>>();
    let route = classify(
        &temp,
        "pull_request",
        "refs/pull/29/merge",
        &changed_path_refs,
        "false",
        "false",
    );
    assert_eq!(route["mode"], "dependency_closure");
    assert!(route["packages"]
        .as_array()
        .unwrap()
        .contains(&"tachi-core".into()));

    let core_path = "crates/tachi-core/src/cross_crate.rs";
    let shell_path = "crates/tachi-shell/src/cross_crate.rs";
    fs::create_dir_all(repo.join("crates/tachi-shell/src")).expect("create shell source tree");
    fs::write(repo.join(core_path), "pub fn cross_crate_probe() {}\n")
        .expect("write cross-crate source fixture");
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "--quiet", "-m", "cross crate base"]);
    let cross_base = run_git(&repo, &["rev-parse", "HEAD"]);
    fs::rename(repo.join(core_path), repo.join(shell_path)).expect("rename source across crates");
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "--quiet", "-m", "cross crate rename"]);
    let cross_head = run_git(&repo, &["rev-parse", "HEAD"]);
    let cross_paths = changed_paths_between(&repo, &cross_base, &cross_head);
    assert!(cross_paths.iter().any(|path| path == core_path));
    assert!(cross_paths.iter().any(|path| path == shell_path));
    let cross_path_refs = cross_paths.iter().map(String::as_str).collect::<Vec<_>>();
    let cross_route = classify(
        &temp,
        "pull_request",
        "refs/pull/30/merge",
        &cross_path_refs,
        "false",
        "false",
    );
    assert_eq!(cross_route["mode"], "dependency_closure");
    assert!(cross_route["packages"]
        .as_array()
        .unwrap()
        .contains(&"tachi-core".into()));
    assert!(cross_route["packages"]
        .as_array()
        .unwrap()
        .contains(&"tachi-shell".into()));

    let docs_base = cross_head;
    fs::remove_file(repo.join("docs/reference/rename_probe.md")).expect("remove docs fixture");
    fs::write(
        repo.join("docs/reference/added_probe.md"),
        "passive prose\n",
    )
    .expect("write docs addition fixture");
    run_git(&repo, &["add", "-A"]);
    run_git(
        &repo,
        &["commit", "--quiet", "-m", "docs addition and deletion"],
    );
    let docs_head = run_git(&repo, &["rev-parse", "HEAD"]);
    let docs_paths = changed_paths_between(&repo, &docs_base, &docs_head);
    assert!(docs_paths
        .iter()
        .any(|path| path == "docs/reference/rename_probe.md"));
    assert!(docs_paths
        .iter()
        .any(|path| path == "docs/reference/added_probe.md"));
    let docs_path_refs = docs_paths.iter().map(String::as_str).collect::<Vec<_>>();
    let docs_route = classify(
        &temp,
        "pull_request",
        "refs/pull/31/merge",
        &docs_path_refs,
        "false",
        "false",
    );
    assert_eq!(docs_route["mode"], "passive_docs_only");
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
