use std::{fs, path::Path};

// Context-specific ledger entries; never remap arbitrary historic tokens.
const CONTRACTS: &[(&str, &str, &str)] = &[
    (
        ".claude/agents/tachi/output-integrity.md",
        "id: LLM10, relationship: primary",
        "id: LLM05,",
    ),
    (
        ".claude/agents/tachi/misinformation.md",
        "id: LLM07, relationship: primary",
        "id: LLM09,",
    ),
    (
        ".claude/agents/tachi/prompt-injection.md",
        "OWASP LLM01/LLM08",
        "OWASP LLM01/LLM07",
    ),
    (
        ".claude/agents/tachi/data-poisoning.md",
        "OWASP LLM04/LLM05/LLM09",
        "OWASP LLM03/LLM04/LLM08",
    ),
    (
        ".claude/agents/tachi/model-theft.md",
        "OWASP LLM06/LLM08/LLM04",
        "OWASP LLM10/LLM07/LLM03",
    ),
    (
        ".claude/skills/tachi-output-integrity/references/detection-patterns.md",
        "id: LLM09, relationship: primary",
        "id: LLM08,",
    ),
    (
        ".claude/skills/tachi-misinformation/references/detection-patterns.md",
        "id: LLM07, relationship: primary",
        "id: LLM09,",
    ),
    (
        ".claude/skills/tachi-tool-abuse/references/detection-patterns.md",
        "Category 6 cites LLM04",
        "cites LLM03 as",
    ),
];

fn check_example(text: &str, heading: &str, category: &str) -> Result<(), String> {
    let section = text
        .split_once(&format!("**{heading}**:"))
        .ok_or("missing example")?
        .1;
    let yaml = section
        .split_once("```yaml\n")
        .ok_or("missing YAML")?
        .1
        .split("```")
        .next()
        .unwrap();
    let finding: serde_yaml::Value = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;
    let reference = format!("OWASP {category}:2026");
    if !finding["references"]
        .as_sequence()
        .ok_or("missing references")?
        .iter()
        .any(|r| r.as_str() == Some(&reference))
    {
        return Err(format!("{heading}: expected reference {reference}"));
    }
    if let Some(attribution) = finding["source_attribution"].as_sequence() {
        let primary = attribution.iter().find(|r| {
            r["taxonomy"].as_str() == Some("owasp") && r["relationship"].as_str() == Some("primary")
        });
        if primary.and_then(|r| r["id"].as_str()) != Some(category) {
            return Err(format!(
                "{heading}: expected primary attribution {category}"
            ));
        }
    }
    Ok(())
}

#[test]
fn every_active_agent_and_adapter_example_uses_contextual_categories() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (family, mappings) in [
        (
            "data-poisoning",
            vec![
                ("RAG Index Poisoning via User-Uploaded Documents", "LLM09"),
                ("Fine-Tuning Data Manipulation via Shared Storage", "LLM05"),
            ],
        ),
        (
            "model-theft",
            vec![
                ("Model Weights Exposed via Unprotected Storage", "LLM06"),
                ("API-Based Model Extraction via Logprob Exposure", "LLM06"),
                ("Model Architecture Leakage via Error Messages", "LLM08"),
            ],
        ),
    ] {
        let generic_number = if family == "data-poisoning" {
            "08"
        } else {
            "09"
        };
        for path in [
            format!(".claude/agents/tachi/{family}.md"),
            format!("agents/ai/{family}.md"),
            format!("adapters/claude-code/agents/{family}.md"),
            format!("adapters/copilot/agents/{family}.agent.md"),
            format!("adapters/cursor/rules/{family}.mdc"),
            format!("adapters/generic/prompts/{generic_number}-{family}.md"),
        ] {
            let text = fs::read_to_string(root.join(&path)).unwrap();
            if family == "model-theft" && !path.starts_with(".claude/") {
                assert!(
                    text.contains("**OWASP LLM08:2026 - Hidden Context Exposure**"),
                    "{path}: missing declared reference"
                );
                if !path.starts_with("adapters/generic/") {
                    let inventory = text
                        .lines()
                        .find(|line| line.starts_with("owasp_references:"))
                        .expect("declared OWASP inventory");
                    let metadata: serde_yaml::Value = serde_yaml::from_str(inventory).unwrap();
                    let declared = metadata["owasp_references"]
                        .as_sequence()
                        .expect("declared OWASP inventory");
                    for category in ["LLM06", "LLM08", "LLM04"] {
                        assert!(
                            declared
                                .iter()
                                .any(|value| value.as_str()
                                    == Some(&format!("OWASP {category}:2026"))),
                            "{path}: undeclared {category}"
                        );
                    }
                }
            }
            for (heading, category) in &mappings {
                check_example(&text, heading, category).unwrap_or_else(|e| panic!("{path}: {e}"));
                let stale = text.replace(category, "LLM04");
                assert!(
                    check_example(&stale, heading, category).is_err(),
                    "{path}/{heading}: stale mutation escaped"
                );
            }
            if text.contains("**Knowledge Base Corruption via Unaudited Edits**:") {
                check_example(
                    &text,
                    "Knowledge Base Corruption via Unaudited Edits",
                    "LLM09",
                )
                .unwrap_or_else(|e| panic!("{path}: {e}"));
            }
        }
    }
}

fn check(text: &str, required: &str, stale: &str) -> Result<(), String> {
    if !text.contains(required) || text.contains(stale) {
        return Err(format!("expected {required}; stale instruction {stale}"));
    }
    Ok(())
}

#[test]
fn active_emission_instructions_reject_stale_semantics() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (path, required, stale) in CONTRACTS {
        let text = fs::read_to_string(root.join(path)).unwrap();
        check(&text, required, stale).unwrap_or_else(|e| panic!("{path}: {e}"));
        assert!(
            check(&text.replace(required, stale), required, stale).is_err(),
            "{path}: stale mutation escaped"
        );
    }
}

#[test]
fn changed_categories_resolve_to_current_catalog_meanings() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output_agent =
        fs::read_to_string(root.join(".claude/agents/tachi/output-integrity.md")).unwrap();
    let declared = output_agent
        .lines()
        .find(|line| line.starts_with("owasp_references:"))
        .unwrap();
    for category in ["LLM09:2026", "LLM10:2026"] {
        assert!(
            declared.contains(category),
            "output-integrity: undeclared emission category {category}"
        );
    }
    let catalog: Vec<serde_yaml::Value> = serde_yaml::from_str(
        &fs::read_to_string(root.join("schemas/taxonomy/owasp.yaml")).unwrap(),
    )
    .unwrap();
    for (id, meaning) in [
        ("LLM01", "Prompt Injection"),
        ("LLM02", "Sensitive Information Disclosure"),
        ("LLM03", "Excessive Agency"),
        ("LLM04", "Supply Chain"),
        ("LLM05", "Data and Model Poisoning"),
        ("LLM06", "Unbounded Consumption"),
        ("LLM07", "Misinformation"),
        ("LLM08", "Hidden Context Exposure"),
        ("LLM09", "Vector and Embedding"),
        ("LLM10", "Improper Output Handling"),
    ] {
        let row = catalog
            .iter()
            .find(|r| r["id"].as_str() == Some(id))
            .unwrap();
        assert!(
            row["name"].as_str().unwrap().contains(meaning),
            "{id}: wrong category meaning"
        );
    }
    // Provenance is historical, not an emission instruction.
    let historical = fs::read_to_string(
        root.join(".claude/skills/tachi-shared/references/attack-chain-patterns-shared.md"),
    )
    .unwrap();
    assert!(historical.contains("OWASP LLM Top 10 v2025 chained attack scenarios"));
}

#[test]
fn current_baseline_findings_keep_contextual_citations_consistent() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (example, mappings) in [
        (
            "maestro-reference",
            vec![
                ("LLM-2", "LLM05"),
                ("LLM-3", "LLM06"),
                ("LLM-4", "ML01"),
                ("LLM-5", "ML02"),
                ("LLM-6", "ML04"),
            ],
        ),
        (
            "mermaid-agentic-app",
            vec![("LLM-2", "LLM01"), ("LLM-3", "LLM09"), ("LLM-4", "LLM08")],
        ),
    ] {
        let text = fs::read_to_string(root.join(format!("examples/{example}/threats.md"))).unwrap();
        for (finding, category) in mappings {
            let row = text
                .lines()
                .find(|line| line.starts_with(&format!("| {finding} |")))
                .unwrap();
            let year = if category.starts_with("ML") {
                "2023"
            } else {
                "2026"
            };
            assert!(
                row.contains(&format!("OWASP {category}:{year}")),
                "{example}/{finding}: incorrect current category"
            );
            assert!(
                text.contains(&format!(
                    "{finding}:\n  - {{taxonomy: owasp, id: {category}, relationship: primary}}"
                )),
                "{example}/{finding}: attribution disagrees with threat table"
            );
        }
    }
}

#[test]
fn canonical_agentic_example_and_companion_narratives_use_current_categories() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for source in ["agentic-app", "agentic-app/sample-report"] {
        let path = format!("examples/{source}/threats.md");
        let text = fs::read_to_string(root.join(&path)).unwrap();
        for finding in ["LLM-4", "LLM-9", "LLM-11", "LLM-14"] {
            let row = text
                .lines()
                .find(|line| line.starts_with(&format!("| {finding} |")))
                .unwrap();
            check(row, "OWASP LLM05:2026", "OWASP LLM04:2026")
                .unwrap_or_else(|e| panic!("{path}/{finding}: {e}"));
            assert!(check(
                &row.replace("LLM05", "LLM04"),
                "OWASP LLM05:2026",
                "OWASP LLM04:2026"
            )
            .is_err());
        }
    }
    for tree in ["llm-4", "llm-9", "llm-11", "LLM-14"] {
        let path = format!("examples/agentic-app/sample-report/attack-trees/{tree}-attack-tree.md");
        let text = fs::read_to_string(root.join(&path)).unwrap();
        check(&text, "OWASP LLM05:2026", "OWASP LLM04:2026")
            .unwrap_or_else(|e| panic!("{path}: {e}"));
    }
    let path = "examples/mermaid-agentic-app/threat-report.md";
    let text = fs::read_to_string(root.join(path)).unwrap();
    for (finding, current, stale) in [("LLM-3", "LLM09", "LLM04"), ("LLM-4", "LLM08", "LLM06")] {
        let row = text
            .lines()
            .find(|line| line.starts_with(&format!("**{finding}**")))
            .unwrap();
        check(
            row,
            &format!("OWASP {current}:2026"),
            &format!("OWASP {stale}:2026"),
        )
        .unwrap_or_else(|e| panic!("{path}/{finding}: {e}"));
    }
}

#[test]
fn current_sarif_guidance_uses_catalog_category_meanings() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = "examples/maestro-reference/threats.sarif";
    let sarif: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join(path)).unwrap()).unwrap();
    let rules = sarif["runs"][0]["tool"]["driver"]["rules"]
        .as_array()
        .unwrap();
    let guidance = rules
        .iter()
        .map(|r| r["help"]["markdown"].as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    for (current, stale) in [
        (
            "OWASP LLM05:2026 (Data and Model Poisoning)",
            "OWASP LLM04:2026 (Training Data Poisoning)",
        ),
        (
            "OWASP ML05:2023 (Model Theft)",
            "OWASP LLM06:2026 (Model Theft)",
        ),
    ] {
        check(&guidance, current, stale).unwrap();
        assert!(check(&guidance.replace(current, stale), current, stale).is_err());
    }
}
