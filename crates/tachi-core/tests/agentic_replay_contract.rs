use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn temp_dir() -> PathBuf {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "tachi-agentic-replay-{suffix}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&path).expect("create temp directory");
    path
}

#[test]
fn deterministic_replay_covers_safety_outcomes_without_live_model_or_network() {
    let root = temp_dir();
    let output = root.join("replay.json");
    let audit = root.join("audit.jsonl");
    let result = Command::new(repo_root().join("scripts/agentic-replay.sh"))
        .env("AGENTIC_REPLAY_OUTPUT", &output)
        .env("AGENTIC_REPLAY_AUDIT_OUTPUT", &audit)
        .output()
        .expect("run deterministic replay");
    assert!(result.status.success(), "replay failed: {result:?}");
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(&output).expect("read replay output"))
            .expect("replay JSON");
    assert_eq!(value["status"], "passed");
    assert_eq!(value["network_used"], false);
    assert_eq!(value["network_policy"], "deny");
    assert_eq!(value["model"], "scripted-fake");
    assert_eq!(value["promotion_status"], "skipped");
    assert_eq!(value["audit_sink"], "audit.jsonl");
    assert!(value["seed"].is_number());
    let expected_transitions = [
        ("approval", vec!["approved", "executing", "completed"]),
        ("denial", vec!["denied"]),
        ("timeout", vec!["executing", "timed_out"]),
        ("cancel", vec!["executing", "cancelled"]),
        (
            "circuit_breaker",
            vec!["executing", "circuit_open", "blocked"],
        ),
    ];
    for (case, transitions) in expected_transitions {
        let entry = value["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == case)
            .unwrap_or_else(|| panic!("missing replay case: {case}"));
        assert_eq!(entry["status"], "passed");
        assert!(!entry["audit_id"].as_str().unwrap().is_empty());
        let events = entry["audit_events"].as_array().unwrap();
        assert!(events.len() >= 3);
        assert_eq!(events.last().unwrap()["correlated_to"], entry["audit_id"]);
        let actual_transitions: Vec<&str> = events
            .iter()
            .filter_map(|event| event["transition"].as_str())
            .collect();
        assert_eq!(actual_transitions, transitions);
        assert_eq!(entry["outcome"], entry["expected"]);
    }
    let audit_lines: Vec<serde_json::Value> = fs::read_to_string(&audit)
        .expect("read independent audit sink")
        .lines()
        .map(|line| serde_json::from_str(line).expect("audit JSONL entry"))
        .collect();
    let mut expected_audit_lines = Vec::new();
    for entry in value["cases"].as_array().unwrap() {
        for event in entry["audit_events"].as_array().unwrap() {
            expected_audit_lines.push((entry, event));
        }
    }
    assert_eq!(audit_lines.len(), expected_audit_lines.len());
    for (sink, (case, event)) in audit_lines.iter().zip(expected_audit_lines) {
        assert_eq!(sink["audit_id"], case["audit_id"]);
        assert_eq!(sink["case_id"], case["id"]);
        assert_eq!(sink["audit_sink"], "audit.jsonl");
        for key in ["event", "id", "outcome", "transition", "correlated_to"] {
            assert_eq!(sink.get(key), event.get(key), "audit field mismatch: {key}");
        }
    }
    #[cfg(unix)]
    assert_eq!(
        fs::metadata(&audit)
            .expect("audit metadata")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );

    let repeat_root = temp_dir();
    let repeat_output = repeat_root.join("replay.json");
    let repeat_audit = repeat_root.join("audit.jsonl");
    let repeat = Command::new(repo_root().join("scripts/agentic-replay.sh"))
        .env("AGENTIC_REPLAY_OUTPUT", &repeat_output)
        .env("AGENTIC_REPLAY_AUDIT_OUTPUT", &repeat_audit)
        .output()
        .expect("run deterministic replay twice");
    assert!(repeat.status.success(), "repeat replay failed: {repeat:?}");
    assert_eq!(
        fs::read(&output).expect("first replay bytes"),
        fs::read(&repeat_output).expect("repeat replay bytes")
    );
    assert_eq!(
        fs::read(&audit).expect("first audit bytes"),
        fs::read(&repeat_audit).expect("repeat audit bytes")
    );

    let collision_root = temp_dir();
    let collision = collision_root.join("same.json");
    let collision_result = Command::new(repo_root().join("scripts/agentic-replay.sh"))
        .env("AGENTIC_REPLAY_OUTPUT", &collision)
        .env("AGENTIC_REPLAY_AUDIT_OUTPUT", &collision)
        .output()
        .expect("run colliding replay");
    assert!(
        !collision_result.status.success(),
        "output/audit collision must fail closed"
    );
    #[cfg(unix)]
    {
        let symlink_root = temp_dir();
        let aliased_output = symlink_root.join("output.json");
        let audit_target = symlink_root.join("audit-target.jsonl");
        fs::write(&audit_target, b"existing").expect("write audit target");
        std::os::unix::fs::symlink(&audit_target, &aliased_output).expect("create audit alias");
        let symlink_result = Command::new(repo_root().join("scripts/agentic-replay.sh"))
            .env("AGENTIC_REPLAY_OUTPUT", &audit_target)
            .env("AGENTIC_REPLAY_AUDIT_OUTPUT", &aliased_output)
            .output()
            .expect("run symlink-alias replay");
        assert!(
            !symlink_result.status.success(),
            "symlink aliases must fail closed"
        );
        fs::remove_dir_all(symlink_root).expect("symlink cleanup");
    }
    fs::remove_dir_all(repeat_root).expect("repeat cleanup");
    fs::remove_dir_all(collision_root).expect("collision cleanup");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn replay_rejects_live_network_fixture() {
    let root = temp_dir();
    let fixture = root.join("unsafe.json");
    fs::write(
        &fixture,
        r#"{"schema_version":1,"model":"scripted-fake","seed":1,"max_iterations":1,"timeout_seconds":1,"network":true,"allowlisted_commands":["printf"],"cases":[]}"#,
    )
    .expect("write unsafe fixture");
    let result = Command::new(repo_root().join("scripts/agentic-replay.sh"))
        .env("AGENTIC_REPLAY_FIXTURE", &fixture)
        .output()
        .expect("run unsafe replay");
    assert!(
        !result.status.success(),
        "live network fixture must fail closed"
    );
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn replay_rejects_allowlisted_denial_fixture() {
    let root = temp_dir();
    let fixture = root.join("inconsistent.json");
    let source = fs::read_to_string(repo_root().join("tests/fixtures/agentic/replay.json"))
        .expect("read replay fixture");
    fs::write(
        &fixture,
        source.replacen(
            r#"{"id": "denial", "expected": "denied", "tool": "curl"}"#,
            r#"{"id": "denial", "expected": "denied", "tool": "printf"}"#,
            1,
        ),
    )
    .expect("write inconsistent fixture");
    let result = Command::new(repo_root().join("scripts/agentic-replay.sh"))
        .env("AGENTIC_REPLAY_FIXTURE", &fixture)
        .output()
        .expect("run inconsistent replay");
    assert!(
        !result.status.success(),
        "allowlisted denial fixture must fail closed"
    );
    fs::remove_dir_all(root).expect("cleanup");
}
