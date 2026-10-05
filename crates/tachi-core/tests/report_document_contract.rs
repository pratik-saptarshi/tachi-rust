use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
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

fn valid_png() -> Vec<u8> {
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(1, 1, image::Rgba([12, 34, 56, 255]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    bytes.into_inner()
}

fn valid_jpeg() -> Vec<u8> {
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbImage::from_pixel(1, 1, image::Rgb([12, 34, 56]))
        .write_to(&mut bytes, image::ImageFormat::Jpeg)
        .unwrap();
    bytes.into_inner()
}

fn valid_svg() -> Vec<u8> {
    b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><rect width=\"1\" height=\"1\"/></svg>".to_vec()
}

fn attack_tree_markdown(id: &str) -> String {
    format!(
        "# Attack Tree: {id} — Example\n\n**Component**: Agent | **Risk Level**: Critical | **Finding**: {id}\n\n```mermaid\ngraph TD\n A --> B\n```\n"
    )
}

#[test]
fn attack_tree_images_skip_empty_and_corrupt_preferred_candidates() {
    for (case, preferred) in [
        ("empty", Vec::new()),
        ("corrupt", b"not a PNG image".to_vec()),
    ] {
        let fixture = Fixture::new();
        let trees = fixture.0.join("report/attack-trees");
        fs::create_dir_all(&trees).unwrap();
        fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
        fs::write(trees.join("S-1-attack-tree.png"), preferred).unwrap();
        fs::write(trees.join("S-1-attack-tree.jpg"), valid_jpeg()).unwrap();

        let output = fixture.render();
        let binding = binding(&output, "attack-trees");
        assert!(binding.contains("\"has-image\": true"), "{case}: {binding}");
        assert!(binding.contains("S-1-attack-tree.jpg"), "{case}: {binding}");
        assert!(
            !binding.contains("S-1-attack-tree.png"),
            "{case}: {binding}"
        );
    }
}

#[test]
fn attack_tree_with_only_unusable_images_keeps_mermaid_fallback() {
    let fixture = Fixture::new();
    let trees = fixture.0.join("report/attack-trees");
    fs::create_dir_all(&trees).unwrap();
    fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
    fs::write(trees.join("S-1-attack-tree.png"), b"broken PNG payload").unwrap();
    fs::write(trees.join("S-1-attack-tree.jpg"), b"broken JPEG payload").unwrap();
    fs::write(trees.join("S-1-attack-tree.svg"), b"<svg").unwrap();

    let output = fixture.render();
    let binding = binding(&output, "attack-trees");
    assert!(binding.contains("\"has-image\": false"), "{binding}");
    assert!(
        binding.contains("graph TD"),
        "Mermaid fallback must remain: {binding}"
    );
}

#[test]
fn attack_chain_image_resolver_skips_empty_preferred_image() {
    let fixture = Fixture::new();
    let chains = fixture.0.join("report/attack-chains");
    fs::create_dir_all(&chains).unwrap();
    fs::write(chains.join("CHAIN-001-attack-chain.png"), []).unwrap();
    fs::write(chains.join("CHAIN-001-attack-chain.jpg"), valid_jpeg()).unwrap();
    fixture.write(
        "attack-chains.md",
        "# Cross-Layer Attack Chains\n\n## 2. Chain Details\n\n### CHAIN-001: Selected\n\n**Layers**: L1 -> L2\n**Max Severity**: High\n**Surfaced**: Yes\n",
    );

    let output = fixture.render();
    let binding = binding(&output, "attack-chains");
    assert!(binding.contains("\"has-image\": true"), "{binding}");
    assert!(binding.contains("CHAIN-001-attack-chain.jpg"), "{binding}");
}

#[test]
fn attack_tree_image_resolver_accepts_valid_svg_after_corrupt_png() {
    let fixture = Fixture::new();
    let trees = fixture.0.join("report/attack-trees");
    fs::create_dir_all(&trees).unwrap();
    fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
    fs::write(trees.join("S-1-attack-tree.png"), b"corrupt PNG").unwrap();
    fs::write(
        trees.join("S-1-attack-tree.svg"),
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><rect width=\"1\" height=\"1\"/></svg>",
    )
    .unwrap();

    let output = fixture.render();
    let binding = binding(&output, "attack-trees");
    assert!(binding.contains("\"has-image\": true"), "{binding}");
    assert!(binding.contains("S-1-attack-tree.svg"), "{binding}");
}

#[test]
fn attack_tree_image_resolver_rejects_unrenderable_svg_candidates() {
    for (case, svg) in [
        (
            "zero-dimensions",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"0\" height=\"1\"><rect width=\"1\" height=\"1\"/></svg>",
        ),
        (
            "wrong-namespace",
            "<svg xmlns=\"urn:example:not-svg\" width=\"1\" height=\"1\"><rect width=\"1\" height=\"1\"/></svg>",
        ),
        (
            "invalid-path-semantics",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><path d=\"M nonsense\"/></svg>",
        ),
        (
            "zero-sized-shape",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><rect width=\"0\" height=\"1\"/></svg>",
        ),
        (
            "hidden-shape",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><path style=\"display:none\" d=\"M0 0L1 1\"/></svg>",
        ),
    ] {
        let fixture = Fixture::new();
        let trees = fixture.0.join("report/attack-trees");
        fs::create_dir_all(&trees).unwrap();
        fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
        fs::write(trees.join("S-1-attack-tree.svg"), svg).unwrap();

        let output = fixture.render();
        let binding = binding(&output, "attack-trees");
        assert!(
            binding.contains("\"has-image\": false"),
            "{case}: unrenderable SVG must not suppress the Mermaid fallback: {binding}"
        );
        assert!(binding.contains("graph TD"), "{case}: {binding}");
        assert!(
            !binding.contains("S-1-attack-tree.svg"),
            "{case}: invalid SVG must not be selected: {binding}"
        );
    }
}

#[test]
fn attack_tree_image_resolution_rejects_path_ids_and_outside_symlinks() {
    let fixture = Fixture::new();
    let report = fixture.0.join("report");
    let trees = report.join("attack-trees");
    fs::create_dir_all(&trees).unwrap();
    fs::write(
        trees.join("metadata.md"),
        "# Attack Tree: Metadata ID\n\n| Field | Value |\n|---|---|\n| Finding ID | ../../outside-meta |\n| Risk Level | Critical |\n\n```mermaid\ngraph TD\n A --> B\n```\n",
    )
    .unwrap();
    fs::write(
        trees.join("heading.md"),
        attack_tree_markdown("../../outside-heading"),
    )
    .unwrap();
    fs::write(
        fixture.0.join("outside-meta-attack-tree.svg"),
        "<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
    )
    .unwrap();
    fs::write(
        fixture.0.join("outside-heading-attack-tree.svg"),
        "<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
    )
    .unwrap();
    fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
    fs::write(trees.join("S-1-attack-tree.png"), valid_png()).unwrap();

    let output = fixture.render();
    let binding = binding(&output, "attack-trees");
    assert!(
        binding.contains("S-1-attack-tree.png"),
        "valid in-root ID should resolve: {binding}"
    );
    assert_eq!(
        binding.matches("\"has-image\": true").count(),
        1,
        "unsafe IDs must not resolve: {binding}"
    );
    assert_eq!(
        binding.matches("\"has-image\": false").count(),
        2,
        "both unsafe IDs must be omitted: {binding}"
    );
    assert!(
        !binding.contains("outside-meta-attack-tree.svg"),
        "outside metadata asset must not escape: {binding}"
    );
    assert!(
        !binding.contains("outside-heading-attack-tree.svg"),
        "outside heading asset must not escape: {binding}"
    );
}

#[cfg(unix)]
#[test]
fn attack_tree_image_resolution_rejects_symlink_escape() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new();
    let trees = fixture.0.join("report/attack-trees");
    fs::create_dir_all(&trees).unwrap();
    fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
    let outside = fixture.0.join("outside.svg");
    fs::write(&outside, "<svg xmlns=\"http://www.w3.org/2000/svg\"/>").unwrap();
    symlink(outside, trees.join("S-1-attack-tree.svg")).unwrap();

    let output = fixture.render();
    let binding = binding(&output, "attack-trees");
    assert!(binding.contains("\"has-image\": false"), "{binding}");
}

#[test]
fn attack_tree_image_fallbacks_compile_with_pinned_typst() {
    let typst = std::env::var_os("TACHI_TYPST")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            Command::new("typst")
                .arg("--version")
                .output()
                .ok()
                .filter(|output| output.status.success())
                .map(|_| std::path::PathBuf::from("typst"))
        });
    let Some(typst) = typst else {
        eprintln!("skipping Typst compilation; set TACHI_TYPST or install typst");
        return;
    };

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    for (case, preferred, later_candidates) in [
        ("valid-preferred-png", valid_png(), Vec::new()),
        (
            "empty-preferred-jpeg-fallback",
            Vec::new(),
            vec![("jpg", valid_jpeg())],
        ),
        (
            "corrupt-preferred-svg-fallback",
            b"corrupt PNG".to_vec(),
            vec![("svg", valid_svg())],
        ),
        (
            "no-usable-image-invalid-svg-fallback",
            b"corrupt PNG".to_vec(),
            vec![
                ("jpg", b"corrupt JPEG".to_vec()),
                (
                    "svg",
                    b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><path d=\"M nonsense\"/></svg>".to_vec(),
                ),
            ],
        ),
    ] {
        let fixture = Fixture::new();
        let template_dir = fixture.0.join("templates/tachi/security-report");
        copy_dir_all(
            &workspace.join("templates/tachi/security-report"),
            &template_dir,
        );
        let trees = fixture.0.join("report/attack-trees");
        fs::create_dir_all(&trees).unwrap();
        fs::write(trees.join("S-1.md"), attack_tree_markdown("S-1")).unwrap();
        fs::write(trees.join("S-1-attack-tree.png"), preferred).unwrap();
        for (extension, bytes) in later_candidates {
            fs::write(trees.join(format!("S-1-attack-tree.{extension}")), bytes).unwrap();
        }
        fixture.write(
            "threats.md",
            "# Threat Model: Image fallback\n\n## 7. Recommended Actions\n\n| Finding ID | Component | Threat | Risk Level | Mitigation |\n|---|---|---|---|---|\n| S-1 | Agent | Example | High | Validate input |\n",
        );
        fs::write(template_dir.join("report-data.typ"), fixture.render()).unwrap();
        let output_pdf = fixture.0.join(format!("{case}.pdf"));
        let result = Command::new(&typst)
            .arg("compile")
            .arg(template_dir.join("main.typ"))
            .arg(&output_pdf)
            .arg("--root")
            .arg(&fixture.0)
            .current_dir(&fixture.0)
            .output()
            .expect("run Typst");
        assert!(
            result.status.success(),
            "{case}: stdout={} stderr={}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(fs::metadata(output_pdf).unwrap().len() > 0);
    }
}

fn copy_dir_all(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let from = entry.path();
        let to = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir_all(&from, &to);
        } else {
            fs::copy(from, to).unwrap();
        }
    }
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
fn malformed_risk_rows_do_not_replace_findings() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    for risk_scores in [
        "## 2. Scored Threat Table\n\n| Source | Details |\n|---|---|\n| unrelated | not a finding |\n",
        "## 2. Scored Threat Table\n\n| ID | Component | Threat | Composite | Severity |\n|---|---|---|---|---|\n| | Risk Agent | Impersonation | 7.2 | High |\n",
        "## 2. Scored Threat Table\n\n| ID | Component | Threat | Composite | Severity |\n|---|---|---|---|---|\n| S-1 | Risk Agent | Impersonation | 7.2 | |\n",
    ] {
        fixture.write("risk-scores.md", risk_scores);
        let error = tachi_core::try_build_report_data_typst(
            &fixture.0.join("report"),
            &fixture.0,
        )
        .expect_err("invalid optional risk evidence must not produce a misleading report");
        assert!(error.contains("risk-scores.md") && error.contains("Scored Threat Table"));
    }
}

#[test]
fn incomplete_controls_do_not_replace_findings() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    fixture.write(
        "compensating-controls.md",
        "# Compensating Controls\n\n## 3. Control Details\n\n### Authentication\n\n**Status**: Missing | **Effectiveness**: None\n",
    );

    let output = fixture.render();
    assert_eq!(
        binding(&output, "data-source-tier"),
        "#let data-source-tier = 3"
    );
    assert_eq!(
        binding(&output, "total-findings"),
        "#let total-findings = 1"
    );
    assert!(binding(&output, "findings").contains("Raw Agent"));
    assert_eq!(
        binding(&output, "has-compensating-controls"),
        "#let has-compensating-controls = false"
    );
}

#[test]
fn ungrouped_residual_rows_do_not_replace_findings() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    fixture.write(
        "compensating-controls.md",
        "# Compensating Controls\n\n## 2. Coverage Matrix\n\n| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |\n|---|---|---|---|---|---|\n| S-1 | Controlled Agent | Impersonation | 8 | High | Missing |\n\n## 4. Recommendations\n",
    );

    let output = fixture.render();
    assert_eq!(
        binding(&output, "data-source-tier"),
        "#let data-source-tier = 3"
    );
    assert_eq!(
        binding(&output, "total-findings"),
        "#let total-findings = 1"
    );
    assert!(binding(&output, "findings").contains("Raw Agent"));
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
    let fixture = Fixture::new();
    for block in [
        "OI-1: [",
        "OI-1:\n  source_attribution:\n    - {taxonomy: bogus, id: LLM10, relationship: primary}",
    ] {
        let text = format!("{THREATS}\n**Source Attribution**:\n```yaml\n{block}\n```\n");
        assert!(tachi_core::parsers::parse_threats_findings(&text).is_err());
        fixture.write("threats.md", &text);
        let error = tachi_core::try_build_report_data_typst(&fixture.0.join("report"), &fixture.0)
            .unwrap_err();
        assert!(
            error.contains("threats.md") && error.to_ascii_lowercase().contains("attribution"),
            "{error}"
        );
        let legacy = fixture.render();
        assert!(legacy.starts_with("#panic("));
        assert!(!legacy.contains("#let total-findings = 0"));
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

#[test]
fn control_stubs_do_not_erase_valid_risk_findings_but_empty_assessments_are_retained() {
    let fixture = Fixture::new();
    fixture.write("threats.md", THREATS);
    fixture.write("risk-scores.md", "## 2. Scored Threat Table\n\n| ID | Component | Threat | Composite | Severity |\n|---|---|---|---|---|\n| S-1 | Risk Agent | Impersonation | 7.2 | High |\n");
    for invalid in [
        "---\nstatus: error\n---",
        "Generator failed",
        "# Compensating Controls\n\n## 2. Coverage Matrix\n\n## 3. Control Details\n",
        "## 1. Executive Summary\n\n| Status | Count |\n|---|---|\n| Found | failed |\n",
        "## 1. Executive Summary\n\n| Status | Count |\n|---|---|\n| Found | 0 |\n",
        "## 1. Executive Summary\n\n| Status | Count |\n|---|---|\n| Found | 0 |\n| Partial | 0 |\n",
        "## 1. Executive Summary\n\n| Status | Count |\n|---|---|\n| Found | 0 |\n| Partial | 0 |\n| Missing | 0 |\n| Found | 1 |\n",
        "## 2. Coverage Matrix\n\n| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |\n",
    ] {
        fixture.write("compensating-controls.md", invalid);
        let output = fixture.render();
        assert_eq!(
            binding(&output, "data-source-tier"),
            "#let data-source-tier = 2",
            "{invalid}"
        );
        assert_eq!(
            binding(&output, "has-compensating-controls"),
            "#let has-compensating-controls = false"
        );
        assert_eq!(
            binding(&output, "total-findings"),
            "#let total-findings = 1"
        );
        assert!(binding(&output, "findings").contains("Risk Agent"));
    }
    for valid in ["## 2. Coverage Matrix\n\n### High Residual Severity\n\n| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |\n|---|---|---|---|---|---|\n", "## 1. Executive Summary\n\n| Status | Count |\n|---|---|\n| Found | 0 |\n| Partial | 0 |\n| Missing | 0 |\n"] {
        fixture.write("compensating-controls.md", valid);
        let output = fixture.render();
        assert_eq!(binding(&output, "data-source-tier"), "#let data-source-tier = 1");
        assert_eq!(binding(&output, "total-findings"), "#let total-findings = 0");
    }
}
