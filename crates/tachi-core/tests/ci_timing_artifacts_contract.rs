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
    fs::write(path, body).expect("write fake gh");
    let mut permissions = fs::metadata(path).expect("fake gh metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions).expect("make fake gh executable");
}

#[test]
fn verifier_rejects_wrong_workflow_metadata_before_accepting_artifacts() {
    let root = temp_dir("tachi-ci-timing-metadata");
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    let log = root.join("gh.log");
    executable(
        &bin.join("gh"),
        r##"#!/bin/sh
printf '%s\n' "$*" >> "$GH_LOG"
if [ "$1" = "repo" ] && [ "$2" = "view" ]; then
  printf '%s\n' 'pratik-saptarshi/tachi-rust'
  exit 0
fi
if [ "$1" = "run" ] && [ "$2" = "view" ]; then
  printf '%s\n' "$FAKE_RUN_METADATA"
  exit 0
fi
if [ "$1" = "run" ] && [ "$2" = "download" ]; then
  name=""; dir=""
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --name) name="$2"; shift 2 ;;
      --dir) dir="$2"; shift 2 ;;
      *) shift ;;
    esac
  done
  mkdir -p "$dir"
  case "$name" in
    ci-timing-package-*) unit="cargo-test-${name#ci-timing-package-}"; stage="compile-and-test" ;;
    ci-timing-shell-*) unit="shell-tests-${name#ci-timing-shell-}"; stage="test-slice" ;;
    *) exit 2 ;;
  esac
  jq -n --arg unit "$unit" --arg stage "$stage" --arg run_id "$GITHUB_RUN_ID" \
    '{schema_version:1,stage:$stage,unit:$unit,commit:"merge-sha",duration_ms:1,runner:{run_id:$run_id,attempt:1,event:"pull_request"}}' \
    > "$dir/result.json"
  exit 0
fi
exit 2
"##,
    );

    let metadata = r#"{"workflowName":"untrusted workflow","event":"pull_request","status":"completed","conclusion":"success","headBranch":"feature/test","headSha":"source-sha","attempt":1,"databaseId":123}"#;
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let result = Command::new(repo_root().join("scripts/verify-ci-timing-artifacts.sh"))
        .args(["123", "auto"])
        .env("PATH", path)
        .env("GH_REPO", "pratik-saptarshi/tachi-rust")
        .env("GH_LOG", &log)
        .env("GITHUB_RUN_ID", "123")
        .env("FAKE_RUN_METADATA", metadata)
        .output()
        .expect("run timing verifier");

    assert!(
        !result.status.success(),
        "wrong workflow metadata must fail closed"
    );
    let calls = fs::read_to_string(log).expect("read fake gh call log");
    assert!(
        calls.contains("run view"),
        "verifier must inspect run metadata"
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn verifier_accepts_valid_pr_synthetic_merge_provenance() {
    let root = temp_dir("tachi-ci-timing-pr");
    let bin = root.join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    executable(
        &bin.join("gh"),
        r##"#!/bin/sh
if [ "$1" = "run" ] && [ "$2" = "view" ]; then
  printf '%s\n' "$FAKE_RUN_METADATA"
  exit 0
fi
if [ "$1" = "run" ] && [ "$2" = "download" ]; then
  name=""; dir=""
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --name) name="$2"; shift 2 ;;
      --dir) dir="$2"; shift 2 ;;
      *) shift ;;
    esac
  done
  mkdir -p "$dir"
  case "$name" in
    ci-timing-package-*) unit="cargo-test-${name#ci-timing-package-}"; stage="compile-and-test" ;;
    ci-timing-shell-*) unit="shell-tests-${name#ci-timing-shell-}"; stage="test-slice" ;;
    *) exit 2 ;;
  esac
  jq -n --arg unit "$unit" --arg stage "$stage" --arg event "$FAKE_EVENT" --arg workflow "$FAKE_WORKFLOW" --arg ref "$FAKE_REF" --arg head "$FAKE_ARTIFACT_HEAD" --arg source "$FAKE_SOURCE_HEAD" \
    '{schema_version:1,stage:$stage,unit:$unit,commit:"merge-sha",duration_ms:1,runner:{run_id:123,attempt:1,event:$event,workflow_name:$workflow,ref:$ref,head_sha:$head,source_head_sha:$source}}' > "$dir/result.json"
  exit 0
fi
exit 2
"##,
    );
    let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
    let run = |ref_name: &str, source_head: &str, attempt: u32| {
        let metadata = format!(
            r#"{{"workflowName":"rust workspace tests","event":"pull_request","status":"completed","conclusion":"success","headBranch":"feature/test","headSha":"source-sha","attempt":{attempt},"databaseId":123}}"#
        );
        Command::new(repo_root().join("scripts/verify-ci-timing-artifacts.sh"))
            .args(["123", "merge-sha"])
            .env("PATH", &path)
            .env("GH_REPO", "pratik-saptarshi/tachi-rust")
            .env("FAKE_RUN_METADATA", metadata)
            .env("FAKE_EVENT", "pull_request")
            .env("FAKE_WORKFLOW", "rust workspace tests")
            .env("FAKE_REF", ref_name)
            .env("FAKE_ARTIFACT_HEAD", "merge-sha")
            .env("FAKE_SOURCE_HEAD", source_head)
            .output()
            .expect("run timing verifier")
    };
    let result = run("refs/pull/7/merge", "source-sha", 1);
    assert!(
        result.status.success(),
        "valid PR provenance must pass: {result:?}"
    );
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("verified_artifacts") && stdout.contains("status"));
    assert!(!run("refs/pull/7/merge", "wrong-source", 1).status.success());
    assert!(!run("refs/heads/feature/test", "source-sha", 1)
        .status
        .success());
    let rerun = run("refs/pull/7/merge", "source-sha", 2);
    assert!(!rerun.status.success());
    assert!(String::from_utf8_lossy(&rerun.stderr).contains("rerun attempt"));
    fs::remove_dir_all(root).expect("cleanup");
}
