use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

const JPEG_MAGIC: &[u8] = b"\xff\xd8\xff\xe0\x00\x10JFIF";
const THREATS_MD: &str = r#"
# Agentic AI Application

## 7. Recommended Actions

| Finding ID | Component | Threat | Risk Level | Mitigation | Status |
| --- | --- | --- | --- | --- | --- |
| AG-8 | Agent | Prompt injection | High | Harden prompts | [NEW] |

## 6. Risk Summary

| Risk Level | Count |
| --- | --- |
| High | 1 |
| Total | 1 |
"#;
const RISK_SCORES_MD: &str = r#"
## 2. Scored Threat Table

| ID | Component | Threat | CVSS | Exploitability | Scalability | Reachability | Composite | Severity | SLA | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| AG-8 | Agent | Prompt injection | 9.1 | 9.0 | 8.5 | 8.0 | 8.8 | High | 7 | Monitor |

## 3. Dimensional Breakdown

### AG-8: Prompt injection

**Component**: Agent
**Category**: Agentic Threats
**MAESTRO Layer**: L3 Triage
**CVSS Vector**: `AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:L`
**Correlation Group**: Scores inherited from primary finding AG-3
*Score source: correlation primary*

## 4. Governance Fields

| ID | Owner | SLA | Disposition | Review Date |
| --- | --- | --- | --- | --- |
| AG-8 | Alice | 7 | Monitor | 2026-06-06 |
"#;

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{}-{counter}", std::process::id()))
}

fn binary_path(binary_name: &str) -> PathBuf {
    std::env::var(format!("CARGO_BIN_EXE_{binary_name}"))
        .unwrap_or_else(|_| panic!("CARGO_BIN_EXE_{binary_name} should be provided by cargo"))
        .into()
}

fn write_fixture() -> PathBuf {
    let root = unique_temp_dir("tachi-cli-artifacts-e2e");
    let target_dir = root.join("target");
    fs::create_dir_all(target_dir.join("templates")).expect("create fixture directories");
    fs::write(target_dir.join("threats.md"), THREATS_MD).expect("write threats fixture");
    fs::write(target_dir.join("risk-scores.md"), RISK_SCORES_MD)
        .expect("write risk scores fixture");
    fs::write(
        target_dir.join("threat-executive-architecture.jpg"),
        [JPEG_MAGIC, b"fixture"].concat(),
    )
    .expect("write report image fixture");
    root
}

fn assert_sarif(path: &Path) -> Value {
    assert!(path.exists(), "expected artifact at {}", path.display());
    let sarif: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("read generated SARIF artifact"))
            .expect("generated artifact should be valid JSON");
    assert_eq!(sarif["version"], "2.1.0");
    assert!(sarif["runs"][0]["results"].as_array().is_some());
    sarif
}

#[test]
fn cli_analysis_generates_report_and_sarif_artifacts_end_to_end() {
    let root = write_fixture();
    let target_dir = root.join("target");
    let report_dir = root.join("generated");
    let report_path = report_dir.join("report-data.typ");
    let threats_sarif_path = report_dir.join("threats.sarif");
    let risk_scores_sarif_path = report_dir.join("risk-scores.sarif");

    let report = Command::new(binary_path("report-data"))
        .args([
            "--target-dir",
            target_dir.to_string_lossy().as_ref(),
            "--template-dir",
            target_dir.join("templates").to_string_lossy().as_ref(),
            "--output",
            report_path.to_string_lossy().as_ref(),
        ])
        .output()
        .expect("run report-data artifact stage");
    assert!(report.status.success(), "report-data failed: {report:?}");
    assert!(fs::read_to_string(&report_path)
        .expect("read report-data artifact")
        .contains("has-executive-architecture"));

    let threats = Command::new(binary_path("threats-sarif"))
        .args([
            "--input",
            target_dir.join("threats.md").to_string_lossy().as_ref(),
            "--output",
            threats_sarif_path.to_string_lossy().as_ref(),
        ])
        .output()
        .expect("run threats-sarif artifact stage");
    assert!(
        threats.status.success(),
        "threats-sarif failed: {threats:?}"
    );
    let threats_sarif = assert_sarif(&threats_sarif_path);
    assert_eq!(
        threats_sarif["runs"][0]["results"][0]["partialFingerprints"]["findingId/v1"],
        "AG-8"
    );

    let risk_scores = Command::new(binary_path("risk-scores-sarif"))
        .args([
            "--risk-scores",
            target_dir.join("risk-scores.md").to_string_lossy().as_ref(),
            "--threats",
            target_dir.join("threats.md").to_string_lossy().as_ref(),
            "--output",
            risk_scores_sarif_path.to_string_lossy().as_ref(),
        ])
        .output()
        .expect("run risk-scores-sarif artifact stage");
    assert!(
        risk_scores.status.success(),
        "risk-scores-sarif failed: {risk_scores:?}"
    );
    let risk_scores_sarif = assert_sarif(&risk_scores_sarif_path);
    assert_eq!(
        risk_scores_sarif["runs"][0]["results"][0]["partialFingerprints"]["findingId/v1"],
        "AG-8"
    );
}

#[test]
fn cli_artifact_failure_does_not_leave_a_partial_sarif_file() {
    let root = write_fixture();
    let output_path = root.join("generated/invalid.sarif");

    let output = Command::new(binary_path("threats-sarif"))
        .args([
            "--input",
            root.join("target/missing-threats.md")
                .to_string_lossy()
                .as_ref(),
            "--output",
            output_path.to_string_lossy().as_ref(),
        ])
        .output()
        .expect("run threats-sarif with invalid input");

    assert_eq!(output.status.code(), Some(1));
    assert!(!output_path.exists());
    assert!(String::from_utf8_lossy(&output.stderr).contains("failed to read"));
}
