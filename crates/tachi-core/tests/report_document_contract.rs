use std::{
    fs,
    path::{Path, PathBuf},
};
use tachi_core::build_report_data_typst;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("tachi-document-{}-{serial}", std::process::id()));
        fs::create_dir_all(root.join("templates/tachi/security-report")).unwrap();
        fs::create_dir_all(root.join("report")).unwrap();
        Self(root)
    }
    fn write(&self, name: &str, text: &str) {
        fs::write(self.0.join("report").join(name), text).unwrap();
    }
    fn render(&self) -> String {
        build_report_data_typst(
            &self.0.join("report"),
            &self.0.join("templates/tachi/security-report"),
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn binding<'a>(output: &'a str, name: &str) -> &'a str {
    let start = output
        .find(&format!("#let {name} = "))
        .unwrap_or_else(|| panic!("missing {name}"));
    output[start..].split("\n#let ").next().unwrap().trim_end()
}

const THREATS: &str = "# Threat Model: Review\n\n## 7. Recommended Actions\n\n| Finding ID | Component | Threat | Risk Level | Mitigation |\n|---|---|---|---|---|\n| S-1 | Raw Agent | Impersonation | High | Require signed requests |\n";
const CONTROLS: &str = "### High Residual Severity\n\n| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |\n|---|---|---|---|---|---|\n| S-1 | Controlled Agent | Impersonation | 7.2 | High | Partial |\n\n## 4. Recommendations\n\n#### 1. S-1 identity controls\n**What to Implement**: Enforce tenant-bound signatures\n";

#[test]
fn control_recommendations_and_component_distribution_survive_tier_selection() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    fixture.write("compensating-controls.md", CONTROLS);
    let output = fixture.render();
    assert_eq!(
        binding(&output, "data-source-tier"),
        "#let data-source-tier = 1"
    );
    let actions = binding(&output, "remediation-actions");
    for required in [
        "S-1",
        "Enforce tenant-bound signatures",
        "High",
        "14d",
        "Partial",
    ] {
        assert!(
            actions.contains(required),
            "missing remediation evidence {required}: {actions}"
        );
    }
    let distribution = binding(&output, "component-distribution");
    assert!(distribution.contains("Controlled Agent") && distribution.contains(",1,"));
    assert!(
        !distribution.contains("Raw Agent"),
        "must use selected tier"
    );
}

#[test]
fn risk_and_raw_tiers_preserve_their_own_component_breakdowns() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    let raw = fixture.render();
    assert!(binding(&raw, "component-distribution").contains("Raw Agent"));
    assert_eq!(
        binding(&raw, "remediation-actions"),
        "#let remediation-actions = ()"
    );
    fixture.write("risk-scores.md", "## 2. Scored Threat Table\n\n| ID | Component | Threat | Composite | Severity |\n|---|---|---|---|---|\n| S-1 | Risk Agent | Impersonation | 7.2 | High |\n");
    let risk = fixture.render();
    assert_eq!(
        binding(&risk, "data-source-tier"),
        "#let data-source-tier = 2"
    );
    assert!(binding(&risk, "component-distribution").contains("Risk Agent"));
    assert!(!binding(&risk, "component-distribution").contains("Raw Agent"));
}

#[test]
fn timeline_only_reports_enable_the_roadmap_without_executive_narrative() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    fixture.write(
        "threat-report.md",
        "# Threat Report\n\n## 1. Executive Summary\n\n### Remediation Timeline\n- **Short-term** (1 High findings)\n",
    );
    let output = fixture.render();
    assert_eq!(
        binding(&output, "has-threat-report"),
        "#let has-threat-report = true"
    );
    assert_eq!(
        binding(&output, "has-compensating-controls"),
        "#let has-compensating-controls = false"
    );
    let actions = binding(&output, "remediation-actions");
    assert!(actions.contains("S-1") && actions.contains("Require signed requests"));
    fixture.write("threat-report.md", "# Threat Report\n");
    let empty = fixture.render();
    assert_eq!(
        binding(&empty, "has-threat-report"),
        "#let has-threat-report = false"
    );
    assert_eq!(
        binding(&empty, "remediation-actions"),
        "#let remediation-actions = ()"
    );
}

#[test]
fn available_brand_assets_are_bound_relative_to_the_template_root() {
    let fixture = Fixture::new();
    let missing = fixture.render();
    assert_eq!(
        binding(&missing, "has-logo-primary"),
        "#let has-logo-primary = false"
    );
    let brand = fixture.0.join("brand/final");
    fs::create_dir_all(&brand).unwrap();
    for name in [
        "tachi-logo-primary.png",
        "tachi-logo-primary-dark.png",
        "tachi-logo-horizontal.png",
    ] {
        fs::write(brand.join(name), b"present asset").unwrap();
    }
    let output = fixture.render();
    assert_eq!(
        binding(&output, "has-logo-primary"),
        "#let has-logo-primary = true"
    );
    assert_eq!(
        binding(&output, "has-logo-horizontal"),
        "#let has-logo-horizontal = true"
    );
    assert!(binding(&output, "logo-primary-dark-path")
        .contains("../../../brand/final/tachi-logo-primary-dark.png"));
}

#[test]
fn maestro_reference_report_contains_its_actual_recommendations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = build_report_data_typst(
        &root.join("examples/maestro-reference"),
        &root.join("templates/tachi/security-report"),
    );
    let controls = tachi_core::parse_compensating_controls_md(
        &fs::read_to_string(root.join("examples/maestro-reference/compensating-controls.md"))
            .unwrap(),
    );
    let actions = binding(&output, "remediation-actions");
    for finding in controls
        .findings
        .iter()
        .filter(|finding| !finding.recommendation.is_empty())
    {
        let escaped = finding
            .recommendation
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        assert!(
            actions.contains(&escaped),
            "{} recommendation lost",
            finding.id
        );
    }
    assert_ne!(actions, "#let remediation-actions = ()");
}

#[test]
fn canonical_agentic_sample_preserves_nested_citations_and_attack_sections() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = root.join("examples/agentic-app/sample-report");
    let text = fs::read_to_string(target.join("threats.md")).unwrap();
    let findings = tachi_core::parsers::parse_threats_findings(&text).unwrap();
    for (id, taxonomy, category) in [
        ("LLM-4", "owasp", "LLM05"),
        ("LLM-14", "owasp", "LLM05"),
        ("OI-1", "owasp", "LLM10"),
        ("OI-1", "cwe", "CWE-79"),
    ] {
        let finding = findings.iter().find(|f| f.id == id).unwrap();
        assert!(
            finding
                .source_attribution
                .as_ref()
                .unwrap_or_else(|| panic!("{id}: missing attribution"))
                .iter()
                .any(|r| r.taxonomy == taxonomy && r.id == category),
            "{id}: lost {category}"
        );
    }
    let output = build_report_data_typst(&target, &root.join("templates/tachi/security-report"));
    assert_eq!(
        binding(&output, "has-attack-trees"),
        "#let has-attack-trees = true"
    );
    assert_eq!(
        binding(&output, "has-attack-chains"),
        "#let has-attack-chains = true"
    );
    assert!(binding(&output, "attack-trees").contains("LLM05:2026"));
    assert!(binding(&output, "attack-chains").contains("CHAIN-005"));
}

#[test]
fn malformed_nested_attribution_fails_instead_of_silently_losing_citations() {
    for block in [
        "OI-1: [",
        "OI-1:\n  source_attribution:\n    - {taxonomy: bogus, id: LLM10, relationship: primary}",
    ] {
        let text = format!("{THREATS}\n**Source Attribution**:\n```yaml\n{block}\n```\n");
        assert!(tachi_core::parsers::parse_threats_findings(&text).is_err());
        assert!(tachi_core::parsers::parse_threats_findings(
            &text.replace("**Source Attribution**:", "## 9. Source Attribution")
        )
        .is_err());
    }
}

#[test]
fn completed_empty_controls_preserve_the_assessment_and_control_metadata() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    fixture.write("compensating-controls.md", "# Compensating Controls\n\n## 1. Executive Summary\n\n| Status | Count |\n|---|---|\n| Found | 0 |\n| Partial | 0 |\n| Missing | 0 |\n\n## 2. Coverage Matrix\n\n### High Residual Severity\n\n| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |\n|---|---|---|---|---|---|\n\n## 3. Control Details\n\n### Authentication\n\n**Status**: Found | **Effectiveness**: High\n**Component**: Assessed Agent\n**Evidence**: Verified signatures\n\n## 4. Recommendations\n");
    let output = fixture.render();
    assert_eq!(
        binding(&output, "data-source-tier"),
        "#let data-source-tier = 1"
    );
    assert_eq!(
        binding(&output, "has-compensating-controls"),
        "#let has-compensating-controls = true"
    );
    assert_eq!(
        binding(&output, "total-findings"),
        "#let total-findings = 0"
    );
    assert!(binding(&output, "controls").contains("Authentication"));
    assert!(binding(&output, "coverage-summary").contains("\"total-found\": 0"));
    fixture.write("compensating-controls.md", "  \n");
    assert_eq!(
        binding(&fixture.render(), "has-compensating-controls"),
        "#let has-compensating-controls = false"
    );
}

#[test]
fn report_chains_exclude_unsurfaced_candidates_and_hide_an_empty_section() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    let chains = "# Cross-Layer Attack Chains\n\n## 2. Chain Details\n\n### CHAIN-001: Selected\n\n**Layers**: L1 -> L2\n**Max Severity**: High\n**Surfaced**: Yes\n\n### CHAIN-002: Excluded\n\n**Layers**: L2 -> L3\n**Max Severity**: Medium\n**Surfaced**: No\n";
    fixture.write("attack-chains.md", chains);
    let output = fixture.render();
    assert_eq!(
        binding(&output, "has-attack-chains"),
        "#let has-attack-chains = true"
    );
    assert!(binding(&output, "attack-chains").contains("CHAIN-001"));
    assert!(!binding(&output, "attack-chains").contains("CHAIN-002"));
    fixture.write(
        "attack-chains.md",
        &chains.replace("**Surfaced**: Yes", "**Surfaced**: No"),
    );
    assert_eq!(
        binding(&fixture.render(), "has-attack-chains"),
        "#let has-attack-chains = false"
    );
}

#[test]
fn compact_attack_tree_metadata_keeps_each_field_separate() {
    let fixture = Fixture::new();
    let target = fixture.0.join("report");
    fs::create_dir_all(target.join("attack-trees")).unwrap();
    fixture.write("attack-trees/S-5.md", "# Attack Tree: S-5 — Impersonation\n\n**Component**: Inter-Agent Channel | **Risk Level**: Critical | **Finding**: S-5\n\n```mermaid\ngraph TD\n A --> B\n```\n");
    let trees = tachi_core::attack_trees::parse_attack_trees(&target, &[], None);
    assert_eq!(trees.len(), 1);
    assert_eq!(trees[0].component, "Inter-Agent Channel");
    assert_eq!(trees[0].severity, "Critical");
}
