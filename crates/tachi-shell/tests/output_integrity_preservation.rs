use serde_json::Value;
use std::{fs, path::PathBuf};
use tachi_shell::commands::{report_data_output, threats_sarif_output};

const FIXTURE: &str = include_str!("../../../tests/fixtures/output-integrity-vector/threats.md");

#[test]
fn numbered_canonical_attribution_preserves_nested_owasp_and_cwe_records() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = threats_sarif_output(&root.join("examples/agentic-app/threats.md")).unwrap();
    let sarif: Value = serde_json::from_str(&output.sarif).unwrap();
    for (id, expected) in [("OI-1", "LLM10"), ("OI-1", "CWE-79"), ("MI-1", "LLM07")] {
        let result = sarif["runs"][0]["results"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["partialFingerprints"]["findingId/v1"] == id)
            .unwrap();
        assert!(
            result["properties"]["source-attribution"]
                .as_array()
                .unwrap()
                .iter()
                .any(|record| record["id"] == expected),
            "{id}: missing {expected}"
        );
    }
}

#[test]
fn explicit_empty_attribution_is_distinct_from_missing_evidence() {
    let mut findings = tachi_core::parse_threats_findings(FIXTURE).unwrap();
    findings[0].source_attribution = Some(vec![]);
    findings[1].source_attribution = None;
    let mut sarif = serde_json::json!({"runs": [{"results": [
        {"partialFingerprints": {"findingId/v1": "OI-1"}, "properties": {}},
        {"partialFingerprints": {"findingId/v1": "LLM-1"}, "properties": {}}
    ]}]});
    tachi_core::threats_sarif::attach_source_attribution(&mut sarif, &findings);
    let results = sarif["runs"][0]["results"].as_array().unwrap();
    assert_eq!(
        results[0]["properties"]["source-attribution"],
        serde_json::json!([])
    );
    assert!(results[1]["properties"].get("source-attribution").is_none());
}

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

#[test]
fn canonical_sample_sarif_preserves_every_explicit_citation() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let input = root.join("examples/agentic-app/sample-report/threats.md");
    let regenerated: Value =
        serde_json::from_str(&threats_sarif_output(&input).unwrap().sarif).unwrap();
    let committed: Value =
        serde_json::from_str(&fs::read_to_string(input.with_extension("sarif")).unwrap()).unwrap();
    let expected = committed["runs"][0]["results"].as_array().unwrap();
    let actual = regenerated["runs"][0]["results"].as_array().unwrap();
    assert_eq!(actual.len(), expected.len());
    for finding in actual {
        let id = &finding["partialFingerprints"]["findingId/v1"];
        let saved = expected
            .iter()
            .find(|r| &r["partialFingerprints"]["findingId/v1"] == id)
            .unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!(
            finding["properties"]["source-attribution"], saved["properties"]["source-attribution"],
            "{id}: committed citations differ from Rust output"
        );
    }
    for (id, reference) in [("LLM-4", "LLM05"), ("OI-1", "LLM10"), ("MI-1", "LLM07")] {
        let finding = actual
            .iter()
            .find(|r| r["partialFingerprints"]["findingId/v1"] == id)
            .unwrap();
        assert!(
            finding["properties"]["source-attribution"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["id"] == reference),
            "{id}: lost {reference}"
        );
    }
}
