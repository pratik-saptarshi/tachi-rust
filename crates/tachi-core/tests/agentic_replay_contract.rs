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
    let path = env::temp_dir().join(format!("tachi-agentic-replay-{suffix}"));
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
    assert!(audit_lines.len() >= 5 * 3);
    assert!(audit_lines
        .iter()
        .all(|entry| entry["audit_id"].is_string()));
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
