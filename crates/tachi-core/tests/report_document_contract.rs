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
    output
        .lines()
        .find(|line| line.starts_with(&format!("#let {name} = ")))
        .unwrap_or_else(|| panic!("missing {name}"))
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
