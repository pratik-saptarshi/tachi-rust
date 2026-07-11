use tachi_core::facade::{
    build_remediation_actions, merge_delta_status, merge_source_attribution,
    parse_threat_report_md, RemediationFinding,
};
use tachi_core::parsers::ThreatFinding;

const THREAT_REPORT_PROSE_ONLY: &str = "# Threat Report\n\n## 1. Executive Summary\n\nThe system under review is a SaaS application with 42 active findings.\n\n**Risk profile by count**: 5 Critical, 12 High, 20 Medium, 5 Low.\n\n**Most critical unresolved exposure**: The auth service allows unauthenticated\nadmin access via a legacy flag that was never removed.\n\n## 2. Architecture Overview\n\nComponents and trust boundaries follow below.\n";

const THREAT_REPORT_WITH_TIMELINE: &str = "# Threat Report\n\n## 1. Executive Summary\n\n### Risk Posture\nHigh residual exposure remains in the authentication and reporting paths.\n\n### Remediation Timeline\n- **Short-term** (3 Critical findings)\n- **Mid-term** (12 High findings)\n\n## 2. Architecture Overview\n\nComponents and trust boundaries follow below.\n";

#[test]
fn parse_threat_report_falls_back_to_full_section1_prose() {
    let result = parse_threat_report_md(THREAT_REPORT_PROSE_ONLY);

    let narrative = result
        .executive_narrative
        .as_deref()
        .expect("narrative should be populated");

    assert!(narrative.contains("42 active findings"));
    assert!(narrative.contains("Risk profile by count"));
    assert!(!narrative.contains("## 2."));
}

#[test]
fn parse_threat_report_extracts_remediation_timeline_entries() {
    let result = parse_threat_report_md(THREAT_REPORT_WITH_TIMELINE);

    let narrative = result
        .executive_narrative
        .as_deref()
        .expect("narrative should be populated");

    assert!(narrative.contains("High residual exposure remains"));
    assert!(!narrative.contains("Architecture Overview"));

    assert_eq!(result.remediation_timeline.len(), 2);
    assert_eq!(result.remediation_timeline[0].timeline, "Short-term");
    assert_eq!(result.remediation_timeline[0].count, 3);
    assert_eq!(result.remediation_timeline[0].severity, "Critical");
    assert_eq!(result.remediation_timeline[1].timeline, "Mid-term");
    assert_eq!(result.remediation_timeline[1].count, 12);
    assert_eq!(result.remediation_timeline[1].severity, "High");
}

#[test]
fn parse_threat_report_truncates_safely_on_utf8_boundary() {
    let mut prose = String::from("# Threat Report\n\n## 1. Executive Summary\n\n");
    for _ in 0..1999 {
        prose.push('a');
    }
    prose.push('🦀');
    prose.push_str(" extra text\n\n## 2. Architecture Overview\n");

    let result = parse_threat_report_md(&prose);
    let narrative = result.executive_narrative.unwrap();
    assert!(narrative.len() <= 2000);
}

fn remediation_finding() -> RemediationFinding {
    RemediationFinding {
        id: String::from("F-001"),
        threat: String::from("Authentication bypass"),
        recommendation: String::from("Remove the legacy flag"),
        control_status: String::new(),
        residual_severity: String::from("High"),
        severity: String::from("Critical"),
        risk_level: String::from("Medium"),
        mitigation: String::from("Require MFA"),
    }
}

#[test]
fn remediation_actions_cover_control_and_timeline_modes() {
    let finding = remediation_finding();
    assert_eq!(build_remediation_actions(&[], 1, true, None), None);

    let controls = build_remediation_actions(&[finding.clone()], 1, true, None)
        .expect("controls should produce actions");
    assert_eq!(controls[0].sla, "14d");
    assert_eq!(controls[0].status, "pending");

    let mut finding_with_status = finding.clone();
    finding_with_status.control_status = String::from("accepted");
    let controls = build_remediation_actions(&[finding_with_status], 1, true, None).unwrap();
    assert_eq!(controls[0].status, "accepted");

    let timeline = parse_threat_report_md(THREAT_REPORT_WITH_TIMELINE);
    let tier_two =
        build_remediation_actions(&[finding.clone()], 2, false, Some(&timeline)).unwrap();
    assert_eq!(tier_two[0].severity, "Critical");
    assert_eq!(tier_two[0].recommendation, "Authentication bypass");
    assert_eq!(tier_two[0].sla, "7d");

    let tier_three = build_remediation_actions(&[finding], 3, false, Some(&timeline)).unwrap();
    assert_eq!(tier_three[0].severity, "Medium");
    assert_eq!(tier_three[0].recommendation, "Require MFA");
    assert_eq!(tier_three[0].sla, "30d");
}

#[test]
fn remediation_actions_require_a_timeline_for_non_control_modes() {
    let finding = remediation_finding();
    assert_eq!(build_remediation_actions(&[finding], 2, false, None), None);
    let empty_report = tachi_core::facade::ThreatReportData::default();
    assert_eq!(
        build_remediation_actions(&[remediation_finding()], 3, false, Some(&empty_report)),
        None
    );
}

#[test]
fn parse_threat_report_ignores_malformed_timeline_entries_and_empty_sections() {
    let report = "# Threat Report\n\n## 1. Executive Summary\n\n### Risk Posture\n\n### Remediation Timeline\n- malformed\n- **Short-term** (not-a-count Critical findings)\n- **Valid** (2 Low findings)\n\n## 2. Architecture Overview\n";
    let result = parse_threat_report_md(report);
    assert_eq!(result.executive_narrative, None);
    assert_eq!(result.remediation_timeline.len(), 1);
    assert_eq!(result.remediation_timeline[0].severity, "Low");
    assert_eq!(parse_threat_report_md("").remediation_timeline.len(), 0);
    assert_eq!(
        parse_threat_report_md("# unrelated").executive_narrative,
        None
    );
}

fn finding(id: &str) -> ThreatFinding {
    ThreatFinding {
        id: id.to_string(),
        ..ThreatFinding::default()
    }
}

#[test]
fn merge_delta_status_updates_matching_findings_only() {
    let mut findings = vec![finding("F-001"), finding("F-002")];
    merge_delta_status(&mut findings, "## 7. Recommended Actions\n\n| Finding ID | Status |\n| --- | --- |\n| F-001 | Resolved |\n| F-999 | Ignored |\n");
    assert_eq!(findings[0].delta_status.as_deref(), Some("Resolved"));
    assert_eq!(findings[1].delta_status, None);

    let mut findings = vec![finding("F-001")];
    merge_delta_status(
        &mut findings,
        "## 7. Recommended Actions\n\n| Finding ID | Status |\n| --- | --- |\n|  |  |\n",
    );
    assert_eq!(findings[0].delta_status, None);
}

#[test]
fn merge_source_attribution_handles_empty_and_valid_yaml_records() {
    let mut findings = vec![finding("F-001"), finding("F-002")];
    merge_source_attribution(&mut findings, "## 9. Source Attribution\n\n```yaml\nF-001: []\nF-002:\n  - { taxonomy: \"cwe\", id: \"CWE-287\", relationship: \"primary\" }\n```\n");
    assert_eq!(findings[0].source_attribution, Some(Vec::new()));
    let records = findings[1].source_attribution.as_ref().unwrap();
    assert_eq!(records[0].taxonomy, "cwe");
    assert_eq!(records[0].id, "CWE-287");

    let mut findings = vec![finding("F-001")];
    merge_source_attribution(&mut findings, "## 9. Source Attribution\nno yaml fence\n");
    assert_eq!(findings[0].source_attribution, None);
}
