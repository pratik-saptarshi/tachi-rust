use pretty_assertions::assert_eq;

use tachi_core::infographic::{
    compute_most_exposed_layer, parse_maestro_layer_distribution, MaestroLayerDistribution,
};

#[test]
fn parse_maestro_layer_distribution_reads_table_rows() {
    let markdown = r#"
#### Risk by MAESTRO Layer

| MAESTRO Layer | Finding Count | Highest Severity |
| --- | --- | --- |
| L5 — Infrastructure Controls | 4 | High |
| L7 — User Interface | 1 | Critical |
"#;

    let actual = parse_maestro_layer_distribution(markdown);
    let expected = vec![
        MaestroLayerDistribution {
            layer_id: String::from("L5"),
            layer_name: String::from("Evaluation and Observability"),
            finding_count: 4,
            highest_severity: String::from("High"),
        },
        MaestroLayerDistribution {
            layer_id: String::from("L7"),
            layer_name: String::from("Agent Ecosystem"),
            finding_count: 1,
            highest_severity: String::from("Critical"),
        },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn compute_most_exposed_layer_prefers_count_severity_then_layer_id() {
    let layer_distribution = vec![
        MaestroLayerDistribution {
            layer_id: String::from("L2"),
            layer_name: String::from("Data Safety"),
            finding_count: 5,
            highest_severity: String::from("Medium"),
        },
        MaestroLayerDistribution {
            layer_id: String::from("L1"),
            layer_name: String::from("Foundation Model"),
            finding_count: 5,
            highest_severity: String::from("High"),
        },
        MaestroLayerDistribution {
            layer_id: String::from("L7"),
            layer_name: String::from("Agent Ecosystem"),
            finding_count: 7,
            highest_severity: String::from("Low"),
        },
    ];

    let actual = compute_most_exposed_layer(&layer_distribution);

    assert_eq!(actual, "L7 — Agent Ecosystem");
}

#[test]
fn maestro_distribution_and_exposure_handle_empty_and_malformed_rows() {
    assert!(parse_maestro_layer_distribution("").is_empty());
    let rows = parse_maestro_layer_distribution(
        r#"#### Risk by MAESTRO Layer

| MAESTRO Layer | Finding Count | Highest Severity |
| --- | --- | --- |
|  | invalid |  |
| L3 | invalid | Medium |
"#,
    );
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].layer_id, "L3");
    assert_eq!(rows[0].finding_count, 0);
    assert_eq!(rows[0].highest_severity, "Medium");

    assert_eq!(compute_most_exposed_layer(&[]), "");
    assert_eq!(
        compute_most_exposed_layer(&[MaestroLayerDistribution {
            layer_id: String::from("L1"),
            layer_name: String::new(),
            finding_count: 1,
            highest_severity: String::from("Low"),
        }]),
        "L1"
    );
}
