use pretty_assertions::assert_eq;

use tachi_core::infographic::{parse_per_finding_maestro, MaestroFinding};

#[test]
fn parse_per_finding_maestro_reads_named_columns_and_skips_empty_layers() {
    let markdown = r#"
### 3.1 Threat Findings

| ID | Agent | Component | Threat | Risk Level | MAESTRO Layer | Notes | Pattern | Owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A-1 | Chat | API | Prompt injection | High | L6 — Agent Ecosystem | first | agent_collusion | Team A |
| A-2 | Chat | API | Prompt injection | Medium |  | second | none | Team A |
"#;

    let actual = parse_per_finding_maestro(markdown);
    let expected = vec![MaestroFinding {
        id: String::from("A-1"),
        component: String::from("API"),
        maestro_layer: String::from("L6 — Security and Compliance"),
        risk_level: String::from("High"),
        threat: String::from("Prompt injection"),
    }];

    assert_eq!(actual, expected);
}

#[test]
fn parse_per_finding_maestro_skips_malformed_sections_rows_and_headers() {
    let markdown = r#"
### 2. Not Findings
| ID | Component |
| --- | --- |
| S-0 | ignored |

### 3. AI Findings
| Wrong | Header |
| --- | --- |
| S-1 | ignored |

### 4. AI Findings
| ID | Component | Threat | Risk Level | MAESTRO Layer |
| --- | --- | --- | --- | --- |
| bad | API | Bad id | High | L1 |
| S-2 | API | Missing layer | Medium |  |
| S-3 | API | Valid | Low | L1 |
"#;

    let findings = parse_per_finding_maestro(markdown);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].id, "S-3");
}
