//! Evaluation evidence shared by every MAESTRO output. Component maps are not evidence.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationState {
    Findings,
    Clean,
    NotApplicable,
    #[default]
    NotEvaluated,
}

impl EvaluationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Findings => "findings",
            Self::Clean => "clean",
            Self::NotApplicable => "not_applicable",
            Self::NotEvaluated => "not_evaluated",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Findings => "Findings",
            Self::Clean => "Evaluated — no findings",
            Self::NotApplicable => "Not applicable",
            Self::NotEvaluated => "Not evaluated",
        }
    }
}

/// Positive counts are evidence even if a stale annotation says otherwise.
/// Zero or absent counts alone cannot establish evaluation or applicability.
pub fn classify_evaluation(count: Option<usize>, evidence: &str) -> EvaluationState {
    if count.is_some_and(|n| n > 0) {
        return EvaluationState::Findings;
    }
    let normalized = evidence
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>();
    let normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
    match normalized.as_str() {
        "not applicable" | "n a" => EvaluationState::NotApplicable,
        "clean"
        | "evaluated no findings"
        | "analyzed no findings"
        | "analysed no findings"
        | "analyzed no findings this scan"
        | "evaluated no findings this scan"
            if count == Some(0) =>
        {
            EvaluationState::Clean
        }
        _ => EvaluationState::NotEvaluated,
    }
}

/// Compare output state arrays, reporting the exact layer and output on drift.
pub fn verify_states(
    expected: &[crate::infographic::MaestroLayerDistribution],
    output: &str,
    actual: &serde_json::Value,
) -> Result<(), String> {
    let rows = actual
        .as_array()
        .ok_or_else(|| format!("{output}: missing MAESTRO layer array"))?;
    for layer in expected {
        let matches: Vec<_> = rows
            .iter()
            .filter(|r| r["layer_id"].as_str() == Some(&layer.layer_id))
            .collect();
        if matches.len() != 1
            || matches[0]["coverage_state"].as_str() != Some(layer.coverage_state.as_str())
        {
            return Err(format!(
                "{}: {output} coverage_state disagrees with evidence ({})",
                layer.layer_id,
                layer.coverage_state.as_str()
            ));
        }
    }
    if rows.len() != expected.len() {
        return Err(format!("{output}: unexpected MAESTRO layers"));
    }
    Ok(())
}
