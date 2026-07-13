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
    for attempt in 0..100 {
        let path = env::temp_dir().join(format!("tachi-act-run-contract-{suffix}-{attempt}"));
        if fs::create_dir(&path).is_ok() {
            let mut permissions = fs::metadata(&path)
                .expect("temp directory metadata")
                .permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(&path, permissions).expect("temp directory permissions");
            return path;
        }
    }
    panic!("create unique temp directory")
}

#[test]
fn benchmark_skips_without_runtime_and_cannot_trigger_side_effects() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir_all(&bin).expect("create fake bin");
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

#[test]
fn benchmark_runs_explicit_docker_fallback_with_bounded_act_policy() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir_all(&bin).expect("create fake bin");
    executable(
        &bin.join("docker"),
        "#!/bin/sh
case \"$1 $2\" in
  version*) printf '%s\\n' '24.0.7' ;;
  info*) printf '%s\\n' '{\"ServerVersion\":\"24.0.7\",\"NCPU\":2,\"MemTotal\":4294967296,\"Architecture\":\"x86_64\",\"OSType\":\"linux\"}' ;;
  'image inspect'*)
    case \"$*\" in
      *Size*) printf '%s\\n' '123456' ;;
      *) printf '%s\\n' 'sha256:act-fixture-image' ;;
    esac
    ;;
  'ps -aq'*) ;;
  *) exit 0 ;;
esac
",
    );
    executable(
        &bin.join("act"),
        "#!/bin/sh
if [ \"$1\" = \"--version\" ]; then
  printf '%s\\n' 'act version 0.2.89'
  exit 0
fi
printf '%s\\n' \"$*\" > \"$HOME/act-args.txt\"
printf '%s\\n' 'token=supersecret path=/Users/neo/private'
if [ -n \"$GITHUB_TOKEN\" ]; then
  printf '%s\\n' leaked > \"$HOME/act-env.txt\"
else
  printf '%s\\n' clean > \"$HOME/act-env.txt\"
fi
exit 0
",
    );
    let output = root.join("benchmark.json");
    let args = root.join("act-args.txt");
    let act_env = root.join("act-env.txt");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke-run.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_RUNTIME", "docker")
        .env("ACT_SMOKE_ALLOW_DOCKER_FALLBACK", "true")
        .env("ACT_SMOKE_ALLOW_MUTABLE_IMAGE", "true")
        .env("ACT_SMOKE_IMAGE", "example/act-fixture:latest")
        .env("DOCKER_HOST", "unix:///tmp/fake-docker.sock")
        .env("ACT_SMOKE_NETWORK", "host")
        .env("HOME", &root)
        .env("TMPDIR", &root)
        .env("ACT_SMOKE_RETAIN", "true")
        .env("GITHUB_TOKEN", "should-not-reach-act")
        .env("ACT_SMOKE_RUN_OUTPUT", &output)
        .output()
        .expect("run available-runtime benchmark");
    assert!(
        result.status.success(),
        "explicit fallback should produce a benchmark result: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read benchmark output"))
            .expect("benchmark JSON");
    assert_eq!(json["status"], "PASSED");
    assert_eq!(json["runtime"]["kind"], "docker");
    assert_eq!(json["runtime"]["endpoint"], "local-unix-socket");
    assert_eq!(
        json["preflight"]["runtime"]["endpoint"],
        "local-unix-socket"
    );
    assert_eq!(json["runtime"]["image_digest"], "sha256:act-fixture-image");
    assert_eq!(json["benchmark"]["cpu_count"], 2u64);
    assert_eq!(json["benchmark"]["memory_total_bytes"], 4294967296u64);
    assert_eq!(json["benchmark"]["image_size_bytes"], 123456u64);
    assert_eq!(json["benchmark"]["image_present_before_pull"], true);
    assert_eq!(json["runtime"]["cache_mode"], "warm");
    assert_eq!(
        json["runtime"]["workflow_sha256"].as_str().unwrap().len(),
        64
    );
    assert_eq!(json["cleanup"]["logs_verified"], true);
    assert_eq!(json["cleanup"]["artifacts_removed"], true);
    assert_eq!(json["side_effects"]["workflow_invoked"], true);
    assert_eq!(json["side_effects"]["sarif_upload"], false);
    assert_eq!(json["cleanup"]["verified"], true);
    assert_eq!(json["cleanup"]["temp_dir_retained"], true);
    let retained_dir = fs::read_dir(&root)
        .expect("read retained run directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("tachi-act-run."))
        })
        .expect("retained run directory");
    let retained_log =
        fs::read_to_string(retained_dir.join("act-redacted.log")).expect("read retained log");
    assert!(!retained_log.contains("supersecret"));
    assert!(!retained_log.contains("/Users/"));
    assert!(retained_log.len() <= 65536);
    let act_args = fs::read_to_string(&args).expect("read act arguments");
    assert_eq!(
        fs::read_to_string(&act_env).expect("read act environment"),
        "clean\n"
    );
    for required in [
        "--container-daemon-socket=-",
        "--secret-file=/dev/null",
        "--network=host",
        "--env=ACT_SMOKE=true",
        "--rm",
    ] {
        assert!(
            act_args.contains(required),
            "act invocation must contain {required}"
        );
    }
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn benchmark_rejects_workflows_outside_the_route_observe_allowlist() {
    let root = temp_dir();
    let output = root.join("benchmark.json");
    let result = Command::new(repo_root().join("scripts/act-smoke-run.sh"))
        .env(
            "ACT_SMOKE_WORKFLOW",
            repo_root().join(".github/workflows/release-please.yml"),
        )
        .env("ACT_SMOKE_JOB", "release-please")
        .env("ACT_SMOKE_RUN_OUTPUT", &output)
        .output()
        .expect("run disallowed workflow benchmark");
    assert!(
        !result.status.success(),
        "privileged workflows must be rejected before preflight: {result:?}"
    );
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("allowlist"),
        "rejection should identify the workflow/job allowlist: {result:?}"
    );
    assert!(!output.exists(), "rejected workflow must not emit a result");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn benchmark_reports_cold_cache_when_image_is_absent_before_pull() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir_all(&bin).expect("create fake bin");
    executable(
        &bin.join("docker"),
        "#!/bin/sh
case \"$1 $2\" in
  version*) printf '%s\\n' '29.5.2' ;;
  info*) printf '%s\\n' '{\"ServerVersion\":\"29.5.2\",\"NCPU\":2,\"MemTotal\":4096}' ;;
  context*) printf '%s\\n' 'unix:///tmp/fake-docker.sock' ;;
  pull*) sleep 0.02 ;;
  'ps -aq'*) ;;
  'image inspect'*)
    case \"$*\" in
      *Size*) printf '%s\\n' '123456' ;;
      *format*) printf '%s\\n' 'sha256:act-fixture-image' ;;
      *) exit 1 ;;
    esac
    ;;
  *) exit 0 ;;
esac
",
    );
    executable(
        &bin.join("act"),
        "#!/bin/sh
printf '%s\\n' 'act version 0.2.89'
",
    );
    let preflight = root.join("preflight.json");
    fs::write(
        &preflight,
        serde_json::json!({
            "schema_version": 1,
            "status": "READY",
            "runtime": {
                "kind": "docker",
                "image": "example/act-fixture:latest",
                "endpoint": "unix:///tmp/fake-docker.sock",
                "rootless": null
            },
            "policy": {
                "secrets": "empty",
                "privileged": false,
                "host_mounts": false,
                "socket_mounts": false
            },
            "side_effects": {
                "workflow_invoked": false,
                "sarif_upload": false
            },
            "resource_profile": {
                "cpu_limit": "2",
                "memory_limit": "4096"
            }
        })
        .to_string(),
    )
    .expect("write preflight");
    let output = root.join("benchmark.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke-run.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_RUNTIME", "docker")
        .env("ACT_SMOKE_ALLOW_DOCKER_FALLBACK", "true")
        .env("ACT_SMOKE_ALLOW_MUTABLE_IMAGE", "true")
        .env("ACT_SMOKE_IMAGE", "example/act-fixture:latest")
        .env("ACT_SMOKE_PREFLIGHT", &preflight)
        .env("DOCKER_HOST", "unix:///tmp/fake-docker.sock")
        .env("ACT_SMOKE_RUN_OUTPUT", &output)
        .output()
        .expect("run cold benchmark");
    assert!(
        result.status.success(),
        "cold benchmark should pass: {result:?}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read benchmark output"))
            .expect("benchmark JSON");
    assert_eq!(json["status"], "PASSED");
    assert_eq!(json["runtime"]["cache_mode"], "cold");
    assert_eq!(json["benchmark"]["image_present_before_pull"], false);
    assert!(json["benchmark"]["image_pull_ms"].as_u64().unwrap() > 0);
    assert_eq!(json["benchmark"]["image_size_bytes"], 123456u64);
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn benchmark_times_out_act_and_reports_cleanup_failure_state() {
    let root = temp_dir();
    let bin = root.join("bin");
    fs::create_dir_all(&bin).expect("create fake bin");
    executable(
        &bin.join("docker"),
        "#!/bin/sh
case \"$1 $2\" in
  version*) printf '%s\\n' '24.0.7' ;;
  info*) printf '%s\\n' '{\"ServerVersion\":\"24.0.7\",\"NCPU\":2,\"MemTotal\":4294967296,\"Architecture\":\"x86_64\",\"OSType\":\"linux\"}' ;;
  'image inspect'*) printf '%s\\n' 'sha256:act-fixture-image' ;;
  'ps -aq'*) ;;
  *) exit 0 ;;
esac
",
    );
    executable(
        &bin.join("act"),
        "#!/bin/sh
if [ \"$1\" = \"--version\" ]; then
  printf '%s\\n' 'act version 0.2.89'
  exit 0
fi
sleep 5
",
    );
    let output = root.join("benchmark.json");
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/act-smoke-run.sh"))
        .env("PATH", path)
        .env("ACT_SMOKE_RUNTIME", "docker")
        .env("ACT_SMOKE_ALLOW_DOCKER_FALLBACK", "true")
        .env("ACT_SMOKE_ALLOW_MUTABLE_IMAGE", "true")
        .env("ACT_SMOKE_IMAGE", "example/act-fixture:latest")
        .env("DOCKER_HOST", "unix:///tmp/fake-docker.sock")
        .env("ACT_SMOKE_TIMEOUT_SECONDS", "1")
        .env("ACT_SMOKE_RUN_OUTPUT", &output)
        .output()
        .expect("run timeout benchmark");
    assert!(
        !result.status.success(),
        "timeout must fail the available benchmark"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read benchmark output"))
            .expect("benchmark JSON");
    assert_eq!(json["status"], "FAILED");
    assert_eq!(json["benchmark"]["timed_out"], true);
    assert_eq!(json["benchmark"]["timeout_seconds"], 1);
    assert_eq!(json["cleanup"]["verified"], true);
    fs::remove_dir_all(root).expect("cleanup");
}

fn executable(path: &Path, body: &str) {
    fs::write(path, body).expect("write runtime shim");
    let mut permissions = fs::metadata(path).expect("shim metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions).expect("shim permissions");
}
