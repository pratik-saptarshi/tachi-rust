use serde_json::json;
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

fn temp_dir(prefix: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = env::temp_dir().join(format!("{prefix}-{suffix}"));
    fs::create_dir_all(&path).expect("create temp directory");
    path
}

fn executable(path: &Path, body: &str) {
    fs::write(path, body).expect("write fake cargo");
    let mut permissions = fs::metadata(path)
        .expect("fake cargo metadata")
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions).expect("make fake cargo executable");
}

fn manifest(path: &Path, argv: &[&str], timeout_seconds: u64) {
    let value = json!({
        "$schema": "../schemas/ci-test-units.schema.json",
        "version": 1,
        "source_workflow": ".github/workflows/rust-workspace.yml",
        "toolchain": "rust-toolchain.toml",
        "modes": {
            "local-full": "all units",
            "local-route-equivalent": "route-equivalent units"
        },
        "units": [{
            "id": "fake-cargo-unit",
            "kind": "package",
            "stage": "compile-and-test",
            "package": "tachi-core",
            "argv": argv,
            "modes": ["local-full", "local-route-equivalent"],
            "timeout_seconds": timeout_seconds
        }]
    });
    fs::write(
        path,
        serde_json::to_vec_pretty(&value).expect("serialize manifest"),
    )
    .expect("write manifest");
}

fn run_runner(
    manifest_path: &Path,
    fake_bin: &Path,
    output: &Path,
    secret: Option<&str>,
    retention: Option<&str>,
    max_log_bytes: Option<&str>,
) -> std::process::Output {
    let path = format!(
        "{}{}{}",
        fake_bin.display(),
        if cfg!(windows) { ";" } else { ":" },
        env::var("PATH").unwrap_or_default()
    );
    let mut command = Command::new(repo_root().join("scripts/ci-local-runner.sh"));
    command
        .arg("--mode")
        .arg("local-full")
        .arg("--output-dir")
        .arg(output)
        .env("CI_LOCAL_MANIFEST", manifest_path)
        .env(
            "CI_LOCAL_RESULT_SCHEMA",
            repo_root().join("schemas/ci-run-result.schema.json"),
        )
        .env_remove("CI_LOCAL_CACHE_STATE")
        .env("PATH", path);
    if let Some(secret) = secret {
        command.env("CI_LOCAL_SECRET", secret);
    }
    if let Some(retention) = retention {
        command.env("CI_LOCAL_RETENTION", retention);
    }
    if let Some(max_log_bytes) = max_log_bytes {
        command.env("CI_LOCAL_MAX_LOG_BYTES", max_log_bytes);
    }
    command.output().expect("run local CI runner")
}

fn result_json(output: &Path) -> serde_json::Value {
    let run_dir = fs::read_dir(output)
        .expect("read runner output")
        .map(|entry| entry.expect("runner entry").path())
        .find(|path| path.is_dir())
        .expect("run directory");
    serde_json::from_slice(&fs::read(run_dir.join("results.json")).expect("results JSON"))
        .expect("parse results JSON")
}

#[test]
fn runner_executes_fake_cargo_as_direct_argv_and_redacts_logs() {
    let root = temp_dir("tachi-ci-runner-success");
    let fake_bin = root.join("bin");
    fs::create_dir(&fake_bin).expect("fake bin");
    let args_file = root.join("args");
    let pem_begin = format!("{}{}", "-----BEGIN ", "PRIVATE KEY-----");
    executable(
        &fake_bin.join("cargo"),
        &format!("#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nprintf '%s\\n' 'local-secret ghp_example-token'\nprintf '%s\\n' '{} secret-without-end-marker'\n", args_file.display(), pem_begin),
    );
    let manifest_path = root.join("manifest.json");
    manifest(&manifest_path, &["cargo", "--version"], 5);
    let output = root.join("output");
    let run = run_runner(
        &manifest_path,
        &fake_bin,
        &output,
        Some("local-secret"),
        Some("retain"),
        None,
    );
    assert!(run.status.success(), "runner failed: {:?}", run);
    assert_eq!(
        fs::read_to_string(args_file).expect("argv capture"),
        "--version\n"
    );
    let result = result_json(&output);
    assert_eq!(result["passed"], 1);
    assert_eq!(result["results"][0]["status"], "passed");
    assert_eq!(result["results"][0]["cache_context"], "unknown");
    let run_dir = fs::read_dir(output)
        .expect("read output")
        .map(|entry| entry.expect("entry").path())
        .find(|path| path.is_dir())
        .expect("run dir");
    let log = fs::read_to_string(run_dir.join("fake-cargo-unit.log")).expect("log");
    assert!(!log.contains("local-secret"));
    assert!(!log.contains("ghp_example-token"));
    assert!(!log.contains("BEGIN PRIVATE KEY"));
    assert!(log.contains("[REDACTED]"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn runner_records_timeout_and_kills_descendant_processes() {
    let root = temp_dir("tachi-ci-runner-timeout");
    let fake_bin = root.join("bin");
    fs::create_dir(&fake_bin).expect("fake bin");
    let pid_file = root.join("child.pid");
    let term_file = root.join("term.marker");
    executable(
        &fake_bin.join("cargo"),
        &format!(
            "#!/bin/sh\nterm() {{ kill \"$child\" 2>/dev/null || true; printf '%s' term > '{}'; exit 143; }}\ntrap term TERM INT\nsleep 30 & child=$!\nprintf '%s' \"$child\" > '{}'\nwait \"$child\"\n",
            term_file.display(),
            pid_file.display(),
        ),
    );
    let manifest_path = root.join("manifest.json");
    manifest(&manifest_path, &["cargo", "hang"], 1);
    let output = root.join("output");
    let run = run_runner(
        &manifest_path,
        &fake_bin,
        &output,
        None,
        Some("retain"),
        None,
    );
    assert!(!run.status.success(), "timeout must fail aggregate run");
    let result = result_json(&output);
    assert_eq!(result["timed_out"], 1);
    assert_eq!(result["results"][0]["status"], "timed_out");
    assert!(
        fs::metadata(term_file).is_ok(),
        "fake cargo did not receive TERM"
    );
    assert!(
        fs::metadata(pid_file).is_ok(),
        "fake cargo did not start a child"
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn runner_default_retention_removes_run_directory_after_success() {
    let root = temp_dir("tachi-ci-runner-ephemeral");
    let fake_bin = root.join("bin");
    fs::create_dir(&fake_bin).expect("fake bin");
    executable(&fake_bin.join("cargo"), "#!/bin/sh\nexit 0\n");
    let manifest_path = root.join("manifest.json");
    manifest(&manifest_path, &["cargo", "--version"], 5);
    let output = root.join("output");
    let run = run_runner(&manifest_path, &fake_bin, &output, None, None, None);
    assert!(run.status.success(), "runner failed: {:?}", run);
    let run_directories = fs::read_dir(&output)
        .expect("read output")
        .map(|entry| entry.expect("output entry").path())
        .filter(|path| path.is_dir())
        .count();
    assert_eq!(
        run_directories, 0,
        "default retention must remove the run directory"
    );
    let receipts = fs::read_dir(&output)
        .expect("read cleanup receipts")
        .map(|entry| entry.expect("receipt entry").path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    assert_eq!(
        receipts.len(),
        1,
        "ephemeral cleanup must leave one receipt; stderr={:?}",
        run.stderr
    );
    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipts[0]).expect("read receipt"))
            .expect("receipt JSON");
    assert_eq!(receipt["verified"], true);
    assert_eq!(receipt["retention"], "ephemeral");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn runner_bounds_redacts_and_sanitizes_retained_diagnostics() {
    let root = temp_dir("tachi-ci-runner-privacy");
    let fake_bin = root.join("bin");
    fs::create_dir(&fake_bin).expect("fake bin");
    let pem_begin = format!("{}{}", "-----BEGIN ", "PRIVATE KEY-----");
    executable(
        &fake_bin.join("cargo"),
        &format!("#!/bin/sh\nprintf '%s' 'AWS_SECRET_ACCESS_KEY=secret Authorization: Basic YWJj {} hidden -----END PRIVATE KEY-----'\nhead -c 1100000 /dev/zero | tr '\\0' A\n", pem_begin),
    );
    let manifest_path = root.join("manifest.json");
    manifest(
        &manifest_path,
        &[
            "cargo",
            root.to_str().expect("root path"),
            "Bearer secret-token",
        ],
        5,
    );
    let output = root.join("output");
    let run = run_runner(
        &manifest_path,
        &fake_bin,
        &output,
        None,
        Some("retain"),
        Some("64"),
    );
    assert!(run.status.success(), "runner failed: {:?}", run);
    let result = result_json(&output);
    assert_eq!(result["results"][0]["cleanup"]["verified"], false);
    assert_eq!(result["cleanup"]["retention"], "retain");
    assert_eq!(result["results"][0]["log_path"], "fake-cargo-unit.log");
    assert!(result["results"][0]["argv"][1]
        .as_str()
        .expect("sanitized argv")
        .starts_with("<path>/tachi-ci-runner-privacy-"));
    assert_eq!(result["results"][0]["argv"][2], "[REDACTED]");
    let run_dir = fs::read_dir(&output)
        .expect("read output")
        .map(|entry| entry.expect("entry").path())
        .find(|path| path.is_dir())
        .expect("run dir");
    let log = fs::read_to_string(run_dir.join("fake-cargo-unit.log")).expect("log");
    assert!(log.contains("[REDACTED]"));
    assert!(!log.contains("secret"));
    assert!(!log.contains("YWJj"));
    assert!(!log.contains("hidden"));
    assert!(log.len() <= 64, "log must be bounded");
    fs::remove_dir_all(root).expect("cleanup");
}
