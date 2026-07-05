use std::fs;
use std::path::PathBuf;

use serde_json::json;

use tachi_mcp::server::McpServer;
use tachi_mcp::tools::{McpOutputMode, McpRequestContext};

fn temp_root(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "tachi-mcp-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn context(request_id: &str) -> McpRequestContext {
    McpRequestContext::new(request_id)
}

#[test]
fn registered_tools_match_the_analysis_surface_only() {
    let server = McpServer::default();
    let expected = [
        ("tachi.coverage-audit", "coverage-audit", "coverage-summary"),
        ("tachi.infographic-data", "infographic-data", "json"),
        ("tachi.report-data", "report-data", "typst"),
        ("tachi.risk-scores-sarif", "risk-scores-sarif", "sarif"),
        ("tachi.threats-sarif", "threats-sarif", "sarif"),
    ];

    assert_eq!(
        server.registered_tool_names(),
        expected
            .iter()
            .map(|(tool, _, _)| *tool)
            .collect::<Vec<_>>()
    );

    for (spec, (tool_name, command_name, output_kind)) in
        server.registered_tools().iter().zip(expected)
    {
        assert_eq!(spec.tool_name, tool_name);
        assert_eq!(spec.command_name, command_name);
        assert_eq!(spec.output_kind, output_kind);
    }
}

#[test]
fn coverage_audit_writes_canonical_artifact_and_returns_metadata() {
    let server = McpServer::default();
    let root = temp_root("coverage-audit");
    fs::create_dir_all(&root).expect("create temp root");

    let result = server
        .invoke_json(
            &context("req-coverage-audit"),
            "tachi.coverage-audit",
            &json!({
                "repo_root": root.to_string_lossy().to_string(),
                "output_mode": "artifact",
            }),
        )
        .expect("tool invocation");

    assert_eq!(result.tool_name, "tachi.coverage-audit");
    assert_eq!(result.request_id, "req-coverage-audit");
    assert_eq!(result.command_name, "coverage-audit");
    assert_eq!(result.output_mode, McpOutputMode::Artifact);
    assert!(!result.cancelled);

    let artifact_path = result.artifact_path.expect("artifact path");
    assert_eq!(
        artifact_path,
        root.join("target").join("mcp").join("coverage-audit.txt")
    );
    let written = fs::read_to_string(&artifact_path).expect("artifact content");
    assert_eq!(written, result.payload);
    assert!(!written.is_empty());
}

#[test]
fn coverage_audit_can_return_in_band_without_artifact_side_effects() {
    let server = McpServer::default();
    let root = temp_root("coverage-audit-in-band");
    fs::create_dir_all(&root).expect("create temp root");

    let result = server
        .invoke_json(
            &context("req-coverage-in-band"),
            "tachi.coverage-audit",
            &json!({
                "repo_root": root.to_string_lossy().to_string(),
                "output_mode": "in-band",
            }),
        )
        .expect("tool invocation");

    assert_eq!(result.tool_name, "tachi.coverage-audit");
    assert_eq!(result.output_mode, McpOutputMode::InBand);
    assert_eq!(result.artifact_path, None);
    assert_eq!(result.artifact_bytes, None);
    assert!(!root.join("target").join("mcp").exists());
    assert!(result.payload.contains("Coverage audit for"));
}

#[test]
fn malformed_tool_payloads_fail_before_dispatch() {
    let server = McpServer::default();
    let err = server
        .invoke_json(
            &context("req-bad-payload"),
            "tachi.coverage-audit",
            &json!({
                "repo_root": 42,
                "output_mode": "artifact",
            }),
        )
        .expect_err("malformed payload should fail");

    assert!(err.contains("invalid payload for tachi.coverage-audit"));
}

#[test]
fn analysis_tools_return_in_band_payloads_for_all_registered_outputs() {
    let server = McpServer::default();
    let root = temp_root("analysis-in-band");
    let target_dir = root.join("examples/agentic-app/sample-report");
    let template_dir = root.join("templates/tachi/security-report");
    fs::create_dir_all(&target_dir).expect("create target dir");
    fs::create_dir_all(&template_dir).expect("create template dir");
    fs::write(
        target_dir.join("threats.md"),
        "# Agentic AI Application\n\n## 7. Recommended Actions\n\n| Finding ID | Component | Threat | Risk Level | Mitigation | Status |\n| --- | --- | --- | --- | --- | --- |\n| AG-8 | Agent | Prompt injection | High | Harden prompts | [NEW] |\n",
    )
    .expect("write threats");
    fs::write(
        target_dir.join("risk-scores.md"),
        "## 2. Scored Threat Table\n\n| ID | Component | Threat | CVSS | Exploitability | Scalability | Reachability | Composite | Severity | SLA | Disposition |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n| AG-8 | Agent | Prompt injection | 9.1 | 9.0 | 8.5 | 8.0 | 8.8 | High | 7 | Monitor |\n",
    )
    .expect("write risk scores");

    let infographic = server
        .invoke_json(
            &context("req-infographic"),
            "tachi.infographic-data",
            &json!({
                "repo_root": target_dir.to_string_lossy().to_string(),
                "template": "maestro-stack",
                "output_mode": "in-band",
            }),
        )
        .expect("infographic invocation");
    assert_eq!(infographic.output_mode, McpOutputMode::InBand);
    let infographic_payload: serde_json::Value =
        serde_json::from_str(&infographic.payload).expect("valid infographic payload");
    assert_eq!(infographic_payload["template"], "maestro-stack");

    let report = server
        .invoke_json(
            &context("req-report"),
            "tachi.report-data",
            &json!({
                "target_dir": target_dir.to_string_lossy().to_string(),
                "template_dir": template_dir.to_string_lossy().to_string(),
                "output_mode": "in-band",
            }),
        )
        .expect("report invocation");
    assert_eq!(report.output_kind, "typst");
    assert!(report.payload.contains("#let project-name ="));

    let threats = server
        .invoke_json(
            &context("req-threats"),
            "tachi.threats-sarif",
            &json!({
                "input": target_dir.join("threats.md").to_string_lossy().to_string(),
                "output_mode": "in-band",
            }),
        )
        .expect("threats invocation");
    assert_eq!(threats.output_kind, "sarif");
    let threats_payload: serde_json::Value =
        serde_json::from_str(&threats.payload).expect("valid threats SARIF");
    assert_eq!(
        threats_payload["runs"][0]["results"][0]["partialFingerprints"]["findingId/v1"],
        "AG-8"
    );

    let risk = server
        .invoke_json(
            &context("req-risk"),
            "tachi.risk-scores-sarif",
            &json!({
                "risk_scores": target_dir.join("risk-scores.md").to_string_lossy().to_string(),
                "threats": target_dir.join("threats.md").to_string_lossy().to_string(),
                "output_mode": "in-band",
            }),
        )
        .expect("risk invocation");
    assert_eq!(risk.command_name, "risk-scores-sarif");
    let risk_payload: serde_json::Value =
        serde_json::from_str(&risk.payload).expect("valid risk SARIF");
    assert_eq!(
        risk_payload["runs"][0]["results"][0]["partialFingerprints"]["findingId/v1"],
        "AG-8"
    );
}

#[test]
fn unknown_tool_calls_fail_closed() {
    let server = McpServer::default();
    let err = server
        .invoke_json(&context("req-unknown"), "tachi.install", &json!({}))
        .expect_err("unknown tool should fail closed");

    assert!(err.contains("authorization error"));
}
