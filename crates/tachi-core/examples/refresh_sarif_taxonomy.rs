//! Refresh a companion SARIF's taxonomy metadata/citations from a native threat
//! export and the current catalog, preserving scores, identities and evidence.
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: refresh_sarif_taxonomy THREATS_SARIF COMPANION_SARIF".into());
    }
    let source: Value = serde_json::from_slice(&fs::read(&args[0])?)?;
    let mut target: Value = serde_json::from_slice(&fs::read(&args[1])?)?;
    let catalog: Vec<Value> = serde_yaml::from_str(&fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/taxonomy/owasp.yaml"),
    )?)?;
    refresh(&source, &mut target, &catalog)?;
    fs::write(
        &args[1],
        format!("{}\n", serde_json::to_string_pretty(&target)?),
    )?;
    Ok(())
}

fn refresh(source: &Value, target: &mut Value, catalog: &[Value]) -> Result<(), Box<dyn Error>> {
    if source["runs"].as_array().is_none_or(|runs| runs.len() != 1) {
        return Err("expected one source SARIF run".into());
    }
    let taxa: Vec<_> = catalog
        .iter()
        .filter(|r| r["id"].as_str().is_some_and(|id| id.starts_with("LLM")))
        .map(|r| json!({"id":r["id"], "name":r["name"]}))
        .collect();
    if taxa.is_empty() || taxa.iter().any(|row| !row["name"].is_string()) {
        return Err("OWASP catalog must contain named LLM categories".into());
    }
    let references: BTreeMap<_, _> = source["runs"][0]["results"]
        .as_array()
        .ok_or("missing source results")?
        .iter()
        .map(|r| {
            (
                r["partialFingerprints"]["findingId/v1"]
                    .as_str()
                    .unwrap_or_default(),
                &r["properties"]["source-attribution"],
            )
        })
        .collect();
    let runs = target
        .get_mut("runs")
        .and_then(Value::as_array_mut)
        .ok_or("missing companion runs array")?;
    if runs.len() != 1 {
        return Err("expected one companion SARIF run".into());
    }
    let run = &mut runs[0];
    let guidance = taxa
        .iter()
        .map(|r| {
            format!(
                "OWASP {}:2026 ({})",
                r["id"].as_str().unwrap(),
                r["name"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    if let Some(rules) = run["tool"]["driver"]["rules"].as_array_mut() {
        for rule in rules.iter_mut().filter(|r| r["id"] == "tachi/ai/llm") {
            rule["fullDescription"]["text"] = json!(format!(
                "LLM integration threats classified by the current catalog: {guidance}."
            ));
            rule["help"]["markdown"] = json!(format!("Review prompt boundaries, training data provenance, model access controls and output validation.\n\n**References**: {guidance}; CWE-74 (Improper Neutralization of Special Elements)."));
        }
    }
    for taxonomy in run["taxonomies"]
        .as_array_mut()
        .ok_or("missing companion taxonomies")?
    {
        if taxonomy["name"] == "OWASP-LLM" {
            taxonomy["version"] = json!("2026");
            taxonomy["taxa"] = json!(taxa);
        }
    }
    for taxonomy in run["tool"]["driver"]["supportedTaxonomies"]
        .as_array_mut()
        .ok_or("missing supported taxonomies")?
    {
        if taxonomy["name"] == "OWASP-LLM" {
            taxonomy["version"] = json!("2026");
        }
    }
    for result in run["results"]
        .as_array_mut()
        .ok_or("missing companion results")?
    {
        let id = result["partialFingerprints"]["findingId/v1"]
            .as_str()
            .ok_or("missing companion finding identity")?;
        let records = references
            .get(id)
            .ok_or_else(|| format!("{id}: absent from native threat export"))?;
        if records.is_array() {
            result["properties"]["source-attribution"] = (*records).clone();
            if let Some(primary) = records
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["taxonomy"] == "owasp" && r["relationship"] == "primary")
            {
                let category = primary["id"].as_str().ok_or("invalid primary category")?;
                if !result["properties"]["owasp_id"].is_null() {
                    result["properties"]["owasp_id"] = json!(category
                        .strip_prefix("LLM")
                        .map(|suffix| format!("LLM-{suffix}"))
                        .unwrap_or_else(|| category.into()));
                }
                if !result["properties"]["owasp-reference"].is_null() && category.starts_with("LLM")
                {
                    result["properties"]["owasp-reference"] =
                        json!(format!("OWASP {category}:2026"));
                }
            }
        } else if records.is_null() {
            if let Some(properties) = result["properties"].as_object_mut() {
                properties.remove("source-attribution");
            }
        } else {
            return Err(format!("{id}: source attribution must be an array or absent").into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_preserves_identity_scores_evidence_and_rejects_unknown_findings() {
        let source = json!({"runs":[{"results":[{"partialFingerprints":{"findingId/v1":"OI-1"},"properties":{"source-attribution":[{"taxonomy":"owasp","id":"LLM10","relationship":"primary"}]}}]}]});
        let mut target = json!({"runs":[{"tool":{"driver":{"supportedTaxonomies":[{"name":"OWASP-LLM","version":"2025"}]}},"taxonomies":[{"name":"OWASP-LLM","version":"2025","taxa":[]}],"results":[{"partialFingerprints":{"findingId/v1":"OI-1","baselineRunId":"original-run"},"message":{"text":"Original evidence"},"properties":{"residual_score":5.6,"composite":7.5,"source-attribution":[{"taxonomy":"owasp","id":"LLM05","relationship":"primary"}]}}]}]});
        let original = target["runs"][0]["results"][0].clone();
        let catalog = vec![json!({"id":"LLM10","name":"Improper Output Handling"})];
        refresh(&source, &mut target, &catalog).unwrap();
        let result = &target["runs"][0]["results"][0];
        assert_eq!(
            result["partialFingerprints"],
            original["partialFingerprints"]
        );
        assert_eq!(result["message"], original["message"]);
        for score in ["residual_score", "composite"] {
            assert_eq!(result["properties"][score], original["properties"][score]);
        }
        assert_eq!(result["properties"]["source-attribution"][0]["id"], "LLM10");
        assert_eq!(
            target["runs"][0]["taxonomies"][0]["taxa"][0]["name"],
            "Improper Output Handling"
        );
        let once = target.clone();
        refresh(&source, &mut target, &catalog).unwrap();
        assert_eq!(once, target);
        target["runs"][0]["results"][0]["partialFingerprints"]["findingId/v1"] = json!("UNKNOWN-1");
        assert!(refresh(&source, &mut target, &catalog)
            .unwrap_err()
            .to_string()
            .contains("UNKNOWN-1"));
        assert!(refresh(&source, &mut json!({}), &catalog).is_err());
        assert!(refresh(&source, &mut target, &[]).is_err());
        let mut absent = source.clone();
        absent["runs"][0]["results"][0]["properties"]
            .as_object_mut()
            .unwrap()
            .remove("source-attribution");
        target = once;
        refresh(&absent, &mut target, &catalog).unwrap();
        assert!(target["runs"][0]["results"][0]["properties"]
            .get("source-attribution")
            .is_none());
        absent["runs"][0]["results"][0]["properties"]["source-attribution"] = json!([]);
        refresh(&absent, &mut target, &catalog).unwrap();
        assert_eq!(
            target["runs"][0]["results"][0]["properties"]["source-attribution"],
            json!([])
        );
    }
}
