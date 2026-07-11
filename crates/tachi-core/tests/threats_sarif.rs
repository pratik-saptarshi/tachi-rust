use std::collections::BTreeMap;

use pretty_assertions::assert_eq;
use tachi_core::sarif_common::{ComponentMetadata, SARIF_SCHEMA_URI};

#[test]
fn build_threats_sarif_marks_agentic_finding_with_asi07_metadata() {
    let mut component_meta = BTreeMap::new();
    component_meta.insert(
        String::from("Agent"),
        ComponentMetadata {
            zone: String::from("Core"),
            dfd_type: String::from("Data Store"),
        },
    );

    let finding = tachi_core::threats_sarif::ThreatSarifFinding {
        id: String::from("AG-8"),
        prefix: String::from("AG"),
        status: String::from("[NEW]"),
        component: String::from("Agent"),
        maestro: String::from("L3 Triage"),
        agentic_pattern: String::from("trust_exploitation"),
        threat: String::from("Prompt injection"),
        owasp_ref: String::new(),
        likelihood: String::from("High"),
        impact: String::from("High"),
        risk_level: String::from("High"),
        mitigation: String::from("Harden prompts"),
    };

    let source_threats_uri = "reports/custom/threats.md";
    let sarif = tachi_core::threats_sarif::build_threats_sarif(
        &[finding],
        &component_meta,
        source_threats_uri,
        Some(source_threats_uri),
    );
    let run = &sarif["runs"][0];
    let result = &run["results"][0];

    assert_eq!(sarif["$schema"], SARIF_SCHEMA_URI);
    assert_eq!(sarif["version"], "2.1.0");
    assert_eq!(result["ruleId"], "tachi/ai/agentic");
    assert_eq!(result["level"], "error");
    assert_eq!(result["message"]["text"], "Prompt injection");
    assert_eq!(result["message"]["markdown"], "Harden prompts");
    assert_eq!(
        result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
        source_threats_uri
    );
    assert_eq!(
        result["locations"][0]["logicalLocations"][0]["fullyQualifiedName"],
        "Core/Agent"
    );
    assert_eq!(
        result["locations"][0]["logicalLocations"][0]["kind"],
        serde_json::Value::Null
    );
    assert_eq!(result["partialFingerprints"]["findingId/v1"], "AG-8");
    assert_eq!(
        result["partialFingerprints"]["primaryLocationLineHash"]
            .as_str()
            .unwrap()
            .len(),
        16
    );
    assert_eq!(result["partialFingerprints"]["baselineRunId"], "");
    assert_eq!(result["properties"]["baselineState"], "new");
    assert_eq!(result["properties"]["maestro-layer"], "L3 Triage");
    assert_eq!(result["properties"]["asi07_emission"], true);
    assert_eq!(
        result["properties"]["feature"],
        "219-asi07-tool-abuse-enrichment"
    );
    assert_eq!(result["properties"]["pattern_category"], 9);
    assert_eq!(result["properties"]["tags"][0], "security");
    assert_eq!(result["properties"]["tags"][1], "ai");
}

#[test]
fn build_threats_sarif_uses_shared_baseline_run_id_for_existing_finding() {
    let mut component_meta = BTreeMap::new();
    component_meta.insert(
        String::from("Agent"),
        ComponentMetadata {
            zone: String::from("Core"),
            dfd_type: String::from("Data Store"),
        },
    );

    let finding = tachi_core::threats_sarif::ThreatSarifFinding {
        id: String::from("AG-8"),
        prefix: String::from("AG"),
        status: String::from("[UNCHANGED]"),
        component: String::from("Agent"),
        maestro: String::from("L3 Triage"),
        agentic_pattern: String::new(),
        threat: String::from("Prompt injection"),
        owasp_ref: String::new(),
        likelihood: String::from("High"),
        impact: String::from("High"),
        risk_level: String::from("High"),
        mitigation: String::from("Harden prompts"),
    };

    let sarif = tachi_core::threats_sarif::build_threats_sarif(
        &[finding],
        &component_meta,
        "reports/custom/threats.md",
        Some("reports/custom/run-id-2026-06-27"),
    );
    let result = &sarif["runs"][0]["results"][0];

    assert_eq!(
        result["partialFingerprints"]["baselineRunId"],
        "reports/custom/run-id-2026-06-27"
    );
    assert_eq!(result["properties"]["baselineState"], "unchanged");
}

#[test]
fn build_threats_sarif_covers_prefix_risk_and_reference_fallbacks() {
    let mut component_meta = BTreeMap::new();
    component_meta.insert(
        String::from("Known"),
        ComponentMetadata {
            zone: String::from("Zone"),
            dfd_type: String::from("Process"),
        },
    );

    let prefixes = [
        "S", "T", "R", "I", "D", "E", "AGP", "LLM", "OI", "MI", "OTHER",
    ];
    let findings = prefixes
        .iter()
        .enumerate()
        .map(
            |(index, prefix)| tachi_core::threats_sarif::ThreatSarifFinding {
                id: format!("{prefix}-{index}"),
                prefix: String::from(*prefix),
                status: if index == 0 {
                    String::from("[NEW]")
                } else {
                    String::from("[UNCHANGED]")
                },
                component: if index == 0 {
                    String::from("Known")
                } else {
                    String::from("Unknown")
                },
                maestro: if index == 1 {
                    String::from("—")
                } else if index == 2 {
                    String::from("Unclassified")
                } else {
                    String::from("L2 — Data")
                },
                agentic_pattern: if index % 2 == 0 {
                    String::from("pattern")
                } else {
                    String::new()
                },
                threat: String::from("Threat"),
                owasp_ref: match index {
                    0 => String::from("OWASP LLM01: Prompt Injection"),
                    1 => String::from("ASI-7"),
                    2 => String::from("MCP-2"),
                    _ => String::new(),
                },
                likelihood: String::from("Medium"),
                impact: String::from("Medium"),
                risk_level: match index % 5 {
                    0 => String::from("Critical"),
                    1 => String::from("High"),
                    2 => String::from("Medium"),
                    3 => String::from("Low"),
                    _ => String::from("Note"),
                },
                mitigation: String::from("Mitigate"),
            },
        )
        .collect::<Vec<_>>();

    let sarif = tachi_core::threats_sarif::build_threats_sarif(
        &findings,
        &component_meta,
        "threats.md",
        Some("baseline"),
    );
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results array");
    assert_eq!(results.len(), prefixes.len());
    assert_eq!(results[0]["level"], "error");
    assert_eq!(results[2]["level"], "warning");
    assert_eq!(results[3]["level"], "note");
    assert_eq!(results[0]["properties"]["owasp_id"], "LLM-01");
    assert_eq!(results[1]["properties"]["owasp_id"], "ASI-07");
    assert_eq!(results[2]["properties"]["owasp_id"], "MCP-02");
    assert_eq!(
        results[0]["locations"][0]["logicalLocations"][0]["kind"],
        "process"
    );
    assert_eq!(
        results[1]["locations"][0]["logicalLocations"][0]["kind"],
        "process"
    );
}
