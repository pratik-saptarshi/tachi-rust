use std::env;
use std::fs;
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
    let path = env::temp_dir().join(format!("tachi-act-contract-{suffix}"));
    fs::create_dir_all(&path).expect("create temp directory");
    path
}

#[test]
fn act_preflight_is_unavailable_safe_without_runtime_or_side_effects() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    let output = root.join("result.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_OUTPUT", &output)
        .output()
        .expect("run act preflight");
    assert!(
        result.status.success(),
        "unavailable runtime is advisory: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read preflight output"))
            .expect("preflight JSON");
    assert_eq!(json["status"], "SKIPPED_UNAVAILABLE");
    assert_eq!(json["policy"]["secrets"], "empty");
    assert_eq!(json["policy"]["privileged"], false);
    assert_eq!(json["policy"]["host_mounts"], false);
    assert!(json["side_effects"]["release_or_security_steps"].as_bool() == Some(false));
    fs::remove_dir_all(root).expect("cleanup");
}
