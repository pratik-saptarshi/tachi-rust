use tachi_core::{
    infographic::{build_infographic_payload_from_content, parse_maestro_layer_distribution},
    maestro_coverage::{classify_evaluation, verify_states, EvaluationState as State},
};

const EVIDENCE: &str = "# Threat Model: Coverage\n\n#### Risk by MAESTRO Layer\n\n| MAESTRO Layer | Finding Count | Highest Severity |\n|---|---|---|\n| L1 | 2 | High |\n| L2 | 0 | Analyzed — no findings this scan |\n| L3 | 0 | Not applicable |\n| L4 | 0 | — |\n| L5 | 0 | High |\n| L6 | invalid | Clean |\n";

#[test]
fn classification_requires_unambiguous_evidence() {
    let legacy: tachi_core::infographic::MaestroLayerDistribution = serde_json::from_str(r#"{"layer_id":"L1","layer_name":"Foundation Model","finding_count":0,"highest_severity":""}"#).unwrap();
    assert_eq!(legacy.coverage_state, State::NotEvaluated);
    for evidence in [
        "Analyzed — no findings this scan",
        "  ANALYZED:  no findings this scan. ",
        "Evaluated - no findings",
        "clean",
    ] {
        assert_eq!(classify_evaluation(Some(0), evidence), State::Clean);
    }
    for evidence in ["Not applicable", "Not-applicable.", " N/A "] {
        assert_eq!(classify_evaluation(Some(0), evidence), State::NotApplicable);
    }
    for evidence in [
        "",
        "—",
        "High",
        "None",
        "0",
        "possibly clean",
        "not evaluated",
        "not applicable or clean",
    ] {
        assert_eq!(classify_evaluation(Some(0), evidence), State::NotEvaluated);
    }
    assert_eq!(classify_evaluation(None, "clean"), State::NotEvaluated);
    assert_eq!(
        classify_evaluation(Some(2), "Not applicable"),
        State::Findings
    );
    assert_eq!(
        serde_json::from_str::<State>("\"not_evaluated\"").unwrap(),
        State::default()
    );
}

#[test]
fn evaluated_zero_finding_reports_can_render_maestro() {
    for heading in [
        "### Risk by MAESTRO Layer",
        "#### Risk by MAESTRO Layer",
        "  ##  Risk by MAESTRO Layer ##  ",
    ] {
        let evidence = format!("{heading}\n\n| MAESTRO Layer | Finding Count | Highest Severity |\n|---|---|---|\n| L1 | 0 | Evaluated — no findings |\n| L2 | 0 | Not applicable |\n");
        for template in ["maestro-stack", "maestro-heatmap"] {
            let value = build_infographic_payload_from_content(
                &evidence,
                3,
                "Clean".into(),
                None,
                None,
                template,
            )
            .unwrap();
            assert_eq!(
                value["template_data"]["maestro_layer_distribution"][0]["coverage_state"],
                "clean"
            );
            assert_eq!(
                value["template_data"]["maestro_layer_distribution"][1]["coverage_state"],
                "not_applicable"
            );
            assert_eq!(
                value["template_data"]["maestro_layer_distribution"][2]["coverage_state"],
                "not_evaluated"
            );
            assert_eq!(value["template_data"]["has_maestro_data"], true);
        }
    }
    for evidence in [
        "Risk by MAESTRO Layer",
        "```md\n### Risk by MAESTRO Layer\n```",
    ] {
        assert!(build_infographic_payload_from_content(
            evidence,
            3,
            "Missing".into(),
            None,
            None,
            "maestro-stack"
        )
        .is_err());
    }
}

#[test]
fn actual_findings_override_a_missing_or_stale_summary() {
    let evidence = "### 3.1 Spoofing\n\n| ID | Component | MAESTRO Layer | Risk Level | Threat |\n|---|---|---|---|---|\n| S-1 | Agent | L1 | High | Impersonation |\n";
    let data = tachi_core::infographic::extract_maestro_data(evidence);
    assert_eq!(data.maestro_layer_distribution[0].finding_count, 1);
    assert_eq!(
        data.maestro_layer_distribution[0].coverage_state,
        State::Findings
    );
    assert_eq!(data.maestro_layer_distribution[0].highest_severity, "High");
    assert_eq!(
        data.maestro_layer_distribution[1].coverage_state,
        State::NotEvaluated
    );
}

#[test]
fn markdown_and_infographic_states_agree_and_drift_identifies_layer() {
    let rows = parse_maestro_layer_distribution(EVIDENCE);
    assert_eq!(rows.len(), 7);
    let expected = [
        State::Findings,
        State::Clean,
        State::NotApplicable,
        State::NotEvaluated,
        State::NotEvaluated,
        State::NotEvaluated,
        State::NotEvaluated,
    ];
    assert_eq!(
        rows.iter().map(|r| r.coverage_state).collect::<Vec<_>>(),
        expected
    );
    for template in ["maestro-stack", "maestro-heatmap"] {
        let payload = build_infographic_payload_from_content(&format!("{EVIDENCE}\n## 7. Recommended Actions\n\n| Finding ID | Component | Threat | Risk Level | Mitigation |\n|---|---|---|---|---|\n| LLM-1 | Model | Injection | High | Filter |\n"), 3, "Coverage".into(), None, None, template).unwrap();
        let mut actual = payload["template_data"]["maestro_layer_distribution"].clone();
        verify_states(&rows, template, &actual).unwrap();
        actual[3]["coverage_state"] = "clean".into();
        let error = verify_states(&rows, template, &actual).unwrap_err();
        assert!(error.contains("L4") && error.contains(template), "{error}");
    }
}

#[test]
fn typst_consumes_the_same_seven_states() {
    let target = std::env::temp_dir().join(format!("tachi-maestro-state-{}", std::process::id()));
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("threats.md"), EVIDENCE).unwrap();
    let output = tachi_core::build_report_data_typst(&target, &target);
    for row in parse_maestro_layer_distribution(EVIDENCE) {
        let line = output
            .lines()
            .find(|l| l.contains(&format!("layer-id: \"{}\"", row.layer_id)))
            .unwrap();
        assert!(
            line.contains(&format!(
                "coverage-state: \"{}\"",
                row.coverage_state.as_str()
            )),
            "{}: Typst disagrees",
            row.layer_id
        );
    }
    std::fs::remove_dir_all(target).unwrap();
}
