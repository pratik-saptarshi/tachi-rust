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
        assert!(
            ["passed", "failed", "skipped", "inconclusive"]
                .contains(&entry["promotion_status"].as_str().unwrap()),
            "invalid promotion status for {level}"
        );
        assert!(!entry["tests"].as_array().unwrap().is_empty());
    }
}
