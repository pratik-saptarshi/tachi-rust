use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn temp_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = env::temp_dir().join(format!("tachi-act-run-contract-{suffix}"));
    fs::create_dir_all(&path).expect("create temp directory");
    path
}

#[test]
fn benchmark_skips_without_runtime_and_cannot_trigger_side_effects() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    executable(&bin.join("act"), "#!/bin/sh\nexit 127\n");
    executable(&bin.join("podman"), "#!/bin/sh\nexit 127\n");
    let output = root.join("benchmark.json");
    let caller_preflight = root.join("caller-owned-preflight.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke-run.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_RUN_OUTPUT", &output)
        .env("ACT_SMOKE_PREFLIGHT", &caller_preflight)
        .output()
        .expect("run benchmark wrapper");
    assert!(
        result.status.success(),
        "unavailable benchmark is advisory: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read benchmark output"))
            .expect("benchmark JSON");
    assert_eq!(json["status"], "SKIPPED_UNAVAILABLE");
    assert_eq!(json["job"], "route-observe");
    assert_eq!(json["side_effects"]["workflow_invoked"], false);
    assert_eq!(json["side_effects"]["sarif_upload"], false);
    assert_eq!(json["cleanup"]["verified"], true);
    assert!(
        caller_preflight.is_file(),
        "caller-owned preflight must not be deleted"
    );
    fs::remove_dir_all(root).expect("cleanup");
}

fn executable(path: &Path, body: &str) {
    fs::write(path, body).expect("write runtime shim");
    let mut permissions = fs::metadata(path).expect("shim metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions).expect("shim permissions");
}
