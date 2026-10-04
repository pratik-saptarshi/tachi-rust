use serde_json::Value;
use std::{fs, path::PathBuf};
use tachi_shell::commands::{report_data_output, threats_sarif_output};

const FIXTURE: &str = include_str!("../../../tests/fixtures/output-integrity-vector/threats.md");

#[test]
fn vector_filter_identity_attribution_assets_and_paths_survive() {
    let root = std::env::temp_dir().join(format!("tachi-oi-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    for name in ["threats.md", "tenant report.md"] {
        let input = root.join(name);
        fs::write(&input, FIXTURE).unwrap();
        let parsed = tachi_core::parse_threats_findings(FIXTURE).unwrap();
        assert_eq!(
            parsed.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(),
            ["OI-1", "LLM-1"]
        );
        let output = threats_sarif_output(&input).unwrap();
        assert_eq!(output.sarif, threats_sarif_output(&input).unwrap().sarif);
        let sarif: Value = serde_json::from_str(&output.sarif).unwrap();
        let results = sarif["runs"][0]["results"].as_array().unwrap();
        let oi = results
            .iter()
            .find(|r| r["partialFingerprints"]["findingId/v1"] == "OI-1")
            .unwrap();
        let llm = results
            .iter()
            .find(|r| r["partialFingerprints"]["findingId/v1"] == "LLM-1")
            .unwrap();
        assert_eq!(oi["ruleId"], llm["ruleId"]); // Shared family is not identity.
        assert_eq!(
            oi["properties"]["source-attribution"],
            serde_json::to_value(parsed[0].source_attribution.as_ref().unwrap()).unwrap()
        );
        assert!(oi["properties"]["source-attribution"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == "CWE-943"));
        assert_eq!(oi["properties"]["owasp_id"], "LLM-09");
        assert_eq!(
            oi["properties"]["affected_assets"],
            serde_json::json!(["auth", "pii"])
        );
        assert_eq!(
            oi["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
            input.display().to_string()
        );
    }
    let report = report_data_output(&root, &root);
    for token in ["OI-1", "LLM-1", "CWE-943", "LLM10", "LLM09"] {
        assert!(report.contains(token), "report lost {token}");
    }
    let absent = FIXTURE
        .lines()
        .filter(|line| !line.starts_with("| OI-1"))
        .collect::<Vec<_>>()
        .join("\n");
    let input: PathBuf = root.join("absent.md");
    fs::write(&input, absent).unwrap();
    let sarif: Value = serde_json::from_str(&threats_sarif_output(&input).unwrap().sarif).unwrap();
    assert!(sarif["runs"][0]["results"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["partialFingerprints"]["findingId/v1"] != "OI-1"));
    fs::remove_dir_all(root).unwrap();
}
