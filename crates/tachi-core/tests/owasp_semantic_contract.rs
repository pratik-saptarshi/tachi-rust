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
                ("LLM-3", "LLM02"),
                ("LLM-5", "LLM05"),
                ("LLM-6", "LLM02"),
            ],
        ),
        (
            "mermaid-agentic-app",
            vec![("LLM-2", "LLM01"), ("LLM-3", "LLM05"), ("LLM-4", "LLM08")],
        ),
    ] {
        let text = fs::read_to_string(root.join(format!("examples/{example}/threats.md"))).unwrap();
        for (finding, category) in mappings {
            let row = text
                .lines()
                .find(|line| line.starts_with(&format!("| {finding} |")))
                .unwrap();
            assert!(
                row.contains(&format!("OWASP {category}:2026")),
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
