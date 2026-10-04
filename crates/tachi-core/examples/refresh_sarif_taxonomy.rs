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
    let taxa: Vec<_> = catalog
        .iter()
        .filter(|r| r["id"].as_str().is_some_and(|id| id.starts_with("LLM")))
        .map(|r| json!({"id":r["id"], "name":r["name"]}))
        .collect();
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
    let run = &mut target["runs"][0];
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
    }
}
