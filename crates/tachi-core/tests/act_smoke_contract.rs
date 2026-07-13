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
    let path = env::temp_dir().join(format!("tachi-act-contract-{suffix}"));
    fs::create_dir_all(&path).expect("create temp directory");
    path
}

#[test]
fn act_preflight_is_unavailable_safe_without_runtime_or_side_effects() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    executable(&bin.join("act"), "#!/bin/sh\nexit 127\n");
    executable(&bin.join("podman"), "#!/bin/sh\nexit 127\n");
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

#[test]
fn act_preflight_rejects_remote_docker_endpoints() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    executable(
        &bin.join("act"),
        "#!/bin/sh\nprintf '%s\\n' 'act version 0.2.89'\n",
    );
    executable(
        &bin.join("docker"),
        "#!/bin/sh
case \"$1 $2\" in
  version*) printf '%s\\n' '29.5.2' ;;
  info*) printf '%s\\n' '{\"ServerVersion\":\"29.5.2\",\"NCPU\":2,\"MemTotal\":4096}' ;;
  context*) printf '%s\\n' 'tcp://remote.example:2376' ;;
  *) exit 0 ;;
esac
",
    );
    let output = root.join("result.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_RUNTIME", "docker")
        .env("ACT_SMOKE_ALLOW_DOCKER_FALLBACK", "true")
        .env("ACT_SMOKE_OUTPUT", &output)
        .output()
        .expect("run remote endpoint preflight");
    assert!(
        result.status.success(),
        "remote endpoint is unavailable-safe: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read preflight output"))
            .expect("preflight JSON");
    assert_eq!(json["status"], "SKIPPED_UNAVAILABLE");
    assert_eq!(json["runtime"]["api_compatible"], false);
    assert!(json["reason"]
        .as_str()
        .expect("reason")
        .contains("local unix socket"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn act_preflight_prefers_explicit_docker_host_over_context() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    executable(
        &bin.join("act"),
        "#!/bin/sh\nprintf '%s\\n' 'act version 0.2.89'\n",
    );
    executable(
        &bin.join("docker"),
        "#!/bin/sh
case \"$1 $2\" in
  version*) printf '%s\\n' '29.5.2' ;;
  info*) printf '%s\\n' '{\"ServerVersion\":\"29.5.2\",\"NCPU\":2,\"MemTotal\":4096}' ;;
  context*) printf '%s\\n' 'tcp://stale.example:2376' ;;
  *) exit 0 ;;
esac
",
    );
    let output = root.join("result.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_RUNTIME", "docker")
        .env("ACT_SMOKE_ALLOW_DOCKER_FALLBACK", "true")
        .env("DOCKER_HOST", "unix:///tmp/explicit-docker.sock")
        .env("ACT_SMOKE_OUTPUT", &output)
        .output()
        .expect("run explicit Docker host preflight");
    assert!(
        result.status.success(),
        "explicit Docker host is advisory-ready: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read preflight output"))
            .expect("preflight JSON");
    assert_eq!(json["status"], "READY");
    assert_eq!(
        json["runtime"]["endpoint"],
        "unix:///tmp/explicit-docker.sock"
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn act_preflight_rejects_unverified_rootful_podman() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    executable(
        &bin.join("act"),
        "#!/bin/sh\nprintf '%s\\n' 'act version 0.2.89'\n",
    );
    executable(
        &bin.join("podman"),
        "#!/bin/sh
case \"$1 $2\" in
  version*) printf '%s\\n' '5.8.5' ;;
  info*) printf '%s\\n' '{\"host\":{\"security\":{\"rootless\":false},\"remoteSocket\":{\"path\":\"/tmp/podman.sock\"}},\"version\":{\"Version\":\"5.8.5\"}}' ;;
  *) exit 0 ;;
esac
",
    );
    let output = root.join("result.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_OUTPUT", &output)
        .output()
        .expect("run rootful Podman preflight");
    assert!(
        result.status.success(),
        "rootful Podman is advisory-unavailable: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read preflight output"))
            .expect("preflight JSON");
    assert_eq!(json["status"], "SKIPPED_UNAVAILABLE");
    assert_eq!(json["runtime"]["rootless"], false);
    assert_eq!(json["runtime"]["endpoint"], "unix:///tmp/podman.sock");
    assert!(json["reason"]
        .as_str()
        .expect("reason")
        .contains("rootless"));
    fs::remove_dir_all(root).expect("cleanup");
}

fn executable(path: &Path, body: &str) {
    fs::write(path, body).expect("write runtime shim");
    let mut permissions = fs::metadata(path).expect("shim metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions).expect("shim permissions");
}
