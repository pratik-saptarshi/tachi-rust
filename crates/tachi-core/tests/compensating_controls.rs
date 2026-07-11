use tachi_core::facade::parse_compensating_controls_md;

const CC_MD_WITH_CROSS_LISTED: &str = r#"---
schema_version: "1.0"
---

## 1. Executive Summary

**Risk Reduction**: 100.0 inherent -> 60.0 residual (**40.0%** reduction)
**Coverage**: 30% Found | 30% Partial | 40% Missing

## 2. Coverage Matrix

### High Residual Severity

| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
|-----------|-----------|--------|----------------|-------------------|----------------|
| S-1 | API | Auth bypass | 8.0 | High | Partial |
| S-2 | DB | Data exfil | 5.5 | High | Missing |

### Medium Residual Severity

| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
|-----------|-----------|--------|----------------|-------------------|----------------|
| S-2 | DB | Data exfil | 5.5 | Medium | Missing |
| S-3 | Net | Flood | 4.5 | Medium | Found |

### Low Residual Severity

| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
|-----------|-----------|--------|----------------|-------------------|----------------|

### Critical Residual Severity

| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
|-----------|-----------|--------|----------------|-------------------|----------------|

## 3. Control Details
## 4. Recommendations
"#;

#[test]
fn parse_compensating_controls_dedupes_cross_listed_findings() {
    let data = parse_compensating_controls_md(CC_MD_WITH_CROSS_LISTED);

    let ids = data
        .findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(data.findings.len(), 3);
    assert_eq!(ids.iter().filter(|id| **id == "S-2").count(), 1);
    assert_eq!(data.severity.total, 3);
    assert_eq!(
        data.severity.critical + data.severity.high + data.severity.medium + data.severity.low,
        data.severity.total
    );
}

#[test]
fn parse_compensating_controls_covers_recommendations_metrics_controls_and_stride_statuses() {
    let markdown = r#"
## 1. Executive Summary

**Risk Reduction**: 100.0 inherent -> 55.0 residual (**45.0%** reduction)
**Coverage**: 50% Found | 25% Partial | 25% Missing

Coverage Distribution
| Status | Count |
| --- | --- |
| Found | 50 |
| Partial | 25 |
| Missing | 25 |

## 2. Coverage Matrix

### Critical Residual Severity
| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
| --- | --- | --- | --- | --- | --- |
| S-1 | API | Auth bypass | 9.5 | High | Found |

### High Residual Severity
| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
| --- | --- | --- | --- | --- | --- |
| T-1 | DB | Tamper | 7.5 | High | Partial |

### Medium Residual Severity
| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
| --- | --- | --- | --- | --- | --- |
| R-1 | Audit | Repudiation | 4.5 | Medium | No Control |
| I-1 | API | Disclosure | invalid |  | Found |
| D-1 | API | Flood | 3.0 | Low | found |

### Low Residual Severity
| Threat ID | Component | Threat | Residual Score | Residual Severity | Control Status |
| --- | --- | --- | --- | --- | --- |
| E-1 | API | Escalation | 2.0 | Low | partial |
| AG-1 | Agent | Prompt abuse | 1.0 | Low | unknown |
| LLM-1 | Model | Model theft | 1.0 | Low | Found - exception |
| X-1 | Other | Other issue | 1.0 | Low | Missing |

## 3. Control Details

### API Controls
**Status**: Implemented | **Effectiveness**: High | **Category**: Identity
**Detected in**: `controls/api.md`
Threats Mitigated
| Threat ID | Component |
| --- | --- |
| S-1 | **API Gateway** |

## 4. Recommendations

#### 1. S-1 Auth bypass
**What to Implement**: Rotate credentials
Add an automated rotation check.

## 5. End
"#;

    let data = parse_compensating_controls_md(markdown);

    assert_eq!(data.findings.len(), 9);
    assert_eq!(data.inherent_score, Some(100.0));
    assert_eq!(data.residual_score, Some(55.0));
    assert_eq!(data.risk_reduction, Some(45.0));
    assert_eq!(data.control_coverage_pct, Some(50.0));
    assert_eq!(data.coverage_summary.total_found, 50);
    assert_eq!(data.coverage_summary.total_partial, 25);
    assert_eq!(data.coverage_summary.total_missing, 25);
    assert_eq!(data.controls.len(), 1);
    assert_eq!(data.controls[0].component, "API Gateway");
    assert_eq!(
        data.findings[0].recommendation,
        "Rotate credentials Add an automated rotation check."
    );
    assert!(data
        .coverage_matrix
        .iter()
        .any(|row| row.category == "Spoofing"));
    assert!(data
        .coverage_matrix
        .iter()
        .any(|row| row.category == "Other"));
}

#[test]
fn parse_compensating_controls_handles_empty_and_unparseable_metrics() {
    assert_eq!(parse_compensating_controls_md(""), Default::default());

    let data = parse_compensating_controls_md(
        "**Risk Reduction**: unknown -> unknown (**unknown%**)
**Coverage**: unknown%
",
    );

    assert_eq!(data.inherent_score, None);
    assert_eq!(data.residual_score, None);
    assert_eq!(data.risk_reduction, None);
    assert_eq!(data.control_coverage_pct, None);
}
