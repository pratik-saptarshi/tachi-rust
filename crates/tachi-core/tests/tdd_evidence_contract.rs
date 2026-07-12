use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn tdd_evidence_maps_acceptance_criteria_to_all_test_levels() {
    let path = repo_root().join("docs/testing/tdd-evidence.json");
    let value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&path).expect("read durable TDD evidence contract"),
    )
    .expect("TDD evidence must be valid JSON");
    assert_eq!(value["schema_version"], 1);
    let levels = value["levels"].as_array().expect("levels array");
    assert_eq!(levels.len(), 5, "exactly five test levels are required");
    let names = levels
        .iter()
        .map(|entry| entry["level"].as_str().expect("level name"))
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), 5, "test levels must be unique");
    for level in ["unit", "integration", "functional", "e2e", "agentic"] {
        let entry = levels
            .iter()
            .find(|entry| entry["level"] == level)
            .unwrap_or_else(|| panic!("missing TDD level: {level}"));
        assert!(!entry["acceptance_criteria"].as_str().unwrap().is_empty());
        for phase in ["red", "green", "refactor"] {
            assert!(
                !entry[phase].as_str().unwrap().is_empty(),
                "missing {phase} for {level}"
            );
        }
        let status = entry["promotion_status"].as_str().unwrap();
        assert!(
            ["passed", "failed", "skipped", "inconclusive"].contains(&status),
            "invalid promotion status for {level}"
        );
        if level == "agentic" {
            assert_eq!(
                status, "skipped",
                "agentic promotion requires E2E-COV-010.2"
            );
        } else {
            assert_eq!(status, "passed", "established level must be promoted");
        }
        assert!(!entry["promotion_note"].as_str().unwrap().is_empty());
        let tests = entry["tests"].as_array().unwrap();
        assert!(!tests.is_empty());
        assert!(tests.iter().all(|test| {
            let value = test.as_str().unwrap();
            !value.is_empty() && !value.contains('*')
        }));
    }
}
