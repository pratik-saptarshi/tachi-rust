use std::fs;
use std::path::Path;

use crate::assets::detect_images;
use crate::coverage_attestation::{
    build_per_finding_rows, build_per_framework_aggregates, CoverageFindingRow,
    CoverageFrameworkAggregate, CoverageReference,
};
use crate::metadata::resolve_report_project_name;
use crate::parsers::{compute_has_source_attribution, parse_threats_findings};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportImageBinding {
    pub has_name: &'static str,
    pub path_name: &'static str,
    pub path: Option<String>,
}

pub fn build_report_data_typst(target_dir: &Path, template_dir: &Path) -> String {
    let images = detect_images(target_dir, template_dir);
    let threats_content = fs::read_to_string(target_dir.join("threats.md")).unwrap_or_default();
    let project_name = resolve_report_project_name(&threats_content, None, Some(target_dir));
    let findings = parse_threats_findings(&threats_content).unwrap_or_default();
    let has_source_attribution = compute_has_source_attribution(&findings);
    let per_finding_rows = build_per_finding_rows(&findings);
    let taxonomy_dir = template_dir
        .ancestors()
        .nth(3)
        .map(|root| root.join("schemas/taxonomy"));
    let per_framework_aggregates = match taxonomy_dir.filter(|dir| dir.is_dir()) {
        Some(dir) => {
            crate::coverage_attestation::build_per_framework_aggregates_in_dir(&dir, &findings)
        }
        None => build_per_framework_aggregates(&findings),
    };

    let mut output = render_report_data_typst(
        &project_name,
        &[
            ReportImageBinding {
                has_name: "has-funnel-image",
                path_name: "funnel-image-path",
                path: images.funnel_image_path,
            },
            ReportImageBinding {
                has_name: "has-baseball-image",
                path_name: "baseball-image-path",
                path: images.baseball_image_path,
            },
            ReportImageBinding {
                has_name: "has-architecture-image",
                path_name: "architecture-image-path",
                path: images.architecture_image_path,
            },
            ReportImageBinding {
                has_name: "has-maestro-stack-image",
                path_name: "maestro-stack-image-path",
                path: images.maestro_stack_image_path,
            },
            ReportImageBinding {
                has_name: "has-maestro-heatmap-image",
                path_name: "maestro-heatmap-image-path",
                path: images.maestro_heatmap_image_path,
            },
            ReportImageBinding {
                has_name: "has-executive-architecture",
                path_name: "executive-architecture-image-path",
                path: images.executive_architecture_image_path,
            },
        ],
    );
    output.push_str(&render_coverage_attestation_typst(
        has_source_attribution,
        &per_finding_rows,
        &per_framework_aggregates,
    ));
    output.push_str(&render_maestro_coverage_typst(&threats_content));
    output.push_str(&render_document_data(
        target_dir,
        &threats_content,
        &findings,
    ));
    output
}

// The canonical Rust report path supplies the full Tier-3 template contract.
// Absent optional analysis is explicitly unavailable, never synthesized.
fn render_document_data(
    target: &Path,
    content: &str,
    findings: &[crate::parsers::ThreatFinding],
) -> String {
    use serde_json::json;
    let scope = crate::parsers::parse_scope_data(content);
    let maestro = crate::infographic::extract_maestro_data(content);
    let report = crate::parse_threat_report_md(
        &fs::read_to_string(target.join("threat-report.md")).unwrap_or_default(),
    );
    let metadata = |key: &str| {
        content
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key}:")))
            .unwrap_or("unknown")
            .trim()
            .trim_matches('"')
            .to_string()
    };
    let mut values = json!({
        "assessment-date": metadata("date"), "classification": metadata("classification"),
        "critical-count": findings.iter().filter(|f| f.risk_level == "Critical").count(),
        "high-count": findings.iter().filter(|f| f.risk_level == "High").count(),
        "medium-count": findings.iter().filter(|f| f.risk_level == "Medium").count(),
        "low-count": findings.iter().filter(|f| f.risk_level == "Low").count(),
        "total-findings": findings.len(), "data-source-tier": 3,
        "has-logo-primary": false, "has-logo-horizontal": false,
        "logo-primary-path": "", "logo-horizontal-path": "",
        "has-risk-scores": false, "has-compensating-controls": false,
        "has-threat-report": report.executive_narrative.is_some(),
        "executive-narrative": report.executive_narrative, "component-distribution": [],
        "has-attack-trees": false, "attack-trees": [], "has-attack-chains": false, "attack-chains": [],
        "coverage-matrix": [], "controls": [], "coverage-summary": {}, "remediation-actions": []
    });
    let detail = json!({
        "scope-component-count": scope.components.len(), "scope-data-flow-count": scope.data_flows.len(), "scope-trust-boundary-count": scope.trust_boundaries.len(),
        "scope-components": scope.components.iter().map(|c| json!({"name":c.name,"type":c.kind,"description":c.description})).collect::<Vec<_>>(),
        "scope-data-flows": scope.data_flows.iter().map(|f| json!({"source":f.source,"destination":f.destination,"data":f.data,"protocol":f.protocol})).collect::<Vec<_>>(),
        "scope-trust-boundaries": scope.trust_boundaries.iter().map(|b| json!({"zone":b.zone,"trust-level":b.trust_level,"components":b.components})).collect::<Vec<_>>(),
        "scope-boundary-crossings": scope.boundary_crossings.iter().map(|b| json!({"crossing":b.crossing,"from-zone":b.from_zone,"to-zone":b.to_zone,"components":b.components,"controls":b.controls})).collect::<Vec<_>>(),
        "most-exposed-layer": maestro.most_exposed_layer,
        "maestro-findings-by-layer": crate::infographic::group_maestro_findings_by_layer(&maestro).iter().map(|g| json!({"layer-id":g.layer_id,"layer-name":g.layer_name,"findings":g.findings.iter().map(|f|json!({"id":f.id,"component":f.component,"severity":f.risk_level,"threat":f.threat})).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "findings": findings.iter().map(|f| json!({"id":f.id,"component":f.component,"threat":f.threat,"likelihood":f.likelihood,"impact":f.impact,"risk_level":f.risk_level,"mitigation":f.mitigation})).collect::<Vec<_>>()
    });
    values
        .as_object_mut()
        .unwrap()
        .extend(detail.as_object().unwrap().clone());
    // Preserve the available inherent-risk tier instead of inventing scores.
    if let Ok(risk) = fs::read_to_string(target.join("risk-scores.md")) {
        {
            let rows = crate::parsers::parse_risk_scores_findings(&risk);
            if !rows.is_empty() {
                values["has-risk-scores"] = json!(true);
                values["data-source-tier"] = json!(2);
                values["findings"] = json!(rows.iter().map(|f| json!({"id":f.id,"component":f.component,"threat":f.threat,"composite_score":f.composite_score,"severity":f.severity,"cvss":f.cvss,"exploitability":f.exploitability})).collect::<Vec<_>>());
            }
        }
    }
    if let Ok(text) = fs::read_to_string(target.join("compensating-controls.md")) {
        let data = crate::parse_compensating_controls_md(&text);
        if !data.findings.is_empty() {
            values["has-compensating-controls"] = json!(true);
            values["data-source-tier"] = json!(1);
            values["findings"] = json!(data.findings.iter().map(|f| json!({"id":f.id,"component":f.component,"threat":f.threat,"residual_score":f.residual_score,"residual_severity":f.residual_severity,"control_status":f.control_status,"recommendation":f.recommendation})).collect::<Vec<_>>());
            values["coverage-matrix"] = json!(data.coverage_matrix.iter().map(|r| json!({"category":r.category,"found":r.found,"partial":r.partial,"missing":r.missing})).collect::<Vec<_>>());
            values["controls"] = json!(data.controls.iter().map(|r| json!({"component":r.component,"category":r.category,"status":r.status,"evidence":r.evidence,"effectiveness":r.effectiveness})).collect::<Vec<_>>());
            values["coverage-summary"] = json!({"total-found":data.coverage_summary.total_found,"total-partial":data.coverage_summary.total_partial,"total-missing":data.coverage_summary.total_missing});
        }
    }
    let tier = values["data-source-tier"].as_u64().unwrap();
    let severity_key = if tier == 1 {
        "residual_severity"
    } else if tier == 2 {
        "severity"
    } else {
        "risk_level"
    };
    for severity in ["Critical", "High", "Medium", "Low"] {
        values[format!("{}-count", severity.to_lowercase())] = json!(values["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|f| f[severity_key] == severity)
            .count());
    }
    values["total-findings"] = json!(values["findings"].as_array().unwrap().len());
    values
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| format!("#let {key} = {}\n", typst_value(value)))
        .collect()
}

fn typst_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "none".into(),
        serde_json::Value::String(s) => typst_string(s),
        serde_json::Value::Array(items) => format!(
            "({})",
            items
                .iter()
                .map(|v| format!("{},", typst_value(v)))
                .collect::<String>()
        ),
        serde_json::Value::Object(items) => {
            if items.is_empty() {
                return "(:)".into();
            }
            format!(
                "({})",
                items
                    .iter()
                    .map(|(k, v)| format!("{}: {},", typst_string(k), typst_value(v)))
                    .collect::<String>()
            )
        }
        other => other.to_string(),
    }
}

pub fn render_maestro_coverage_typst(content: &str) -> String {
    let data = crate::infographic::extract_maestro_data(content);
    let mut output = format!(
        "#let has-maestro-data = {}\n#let maestro-layer-coverage = (\n",
        data.has_maestro_data
    );
    for layer in &data.maestro_layer_distribution {
        output.push_str(&format!("  (layer-id: {}, layer-name: {}, finding-count: {}, coverage-state: {}, coverage-label: {}),\n", typst_string(&layer.layer_id), typst_string(&layer.layer_name), layer.finding_count, typst_string(layer.coverage_state.as_str()), typst_string(layer.coverage_state.label())));
    }
    output.push_str(")\n#let maestro-layer-distribution = maestro-layer-coverage\n");
    output
}

fn render_report_data_typst(project_name: &str, bindings: &[ReportImageBinding]) -> String {
    let mut lines = Vec::with_capacity(bindings.len() * 2 + 1);
    lines.push(format!(
        "#let project-name = {}",
        typst_string(project_name)
    ));

    for binding in bindings {
        let has_image = binding.path.is_some();
        lines.push(format!("#let {} = {}", binding.has_name, has_image));
        lines.push(format!(
            "#let {} = {}",
            binding.path_name,
            typst_string(binding.path.as_deref().unwrap_or(""))
        ));
    }

    lines.join("\n") + "\n"
}

fn render_coverage_attestation_typst(
    has_source_attribution: bool,
    per_finding_rows: &[CoverageFindingRow],
    per_framework_aggregates: &[CoverageFrameworkAggregate],
) -> String {
    let mut lines = Vec::new();
    lines.push(String::from(
        "// --- Coverage Attestation Data ----------------------------------------------",
    ));
    lines.push(format!(
        "#let has-source-attribution = {}",
        has_source_attribution
    ));

    if per_finding_rows.is_empty() {
        lines.push(String::from("#let per-finding-rows = ()"));
    } else {
        lines.push(String::from("#let per-finding-rows = ("));
        for row in per_finding_rows {
            lines.push(format!(
                "  (id: {}, title: {}, severity: {}, owasp-refs: {}, mitre-refs: {}, nist-refs: {}, cwe-refs: {}),",
                typst_string(&row.id),
                typst_string(&row.title),
                typst_string(&row.severity),
                render_reference_group(&row.owasp_refs),
                render_reference_group(&row.mitre_refs),
                render_reference_group(&row.nist_refs),
                render_reference_group(&row.cwe_refs),
            ));
        }
        lines.push(String::from(")"));
    }

    if per_framework_aggregates.is_empty() {
        lines.push(String::from("#let per-framework-aggregates = ()"));
    } else {
        lines.push(String::from("#let per-framework-aggregates = ("));
        for aggregate in per_framework_aggregates {
            let items = render_framework_items(&aggregate.items);
            lines.push(format!(
                "  (framework: {}, yaml-record-count: {}, in-scope-record-count: {}, covered-count: {}, partial-count: {}, gap-count: {}, coverage-percentage: {}, items: {}),",
                typst_string(&aggregate.framework),
                aggregate.yaml_record_count,
                aggregate.in_scope_yaml_record_count,
                aggregate.covered_count,
                aggregate.partial_count,
                aggregate.gap_count,
                typst_string(&aggregate.coverage_percentage),
                items,
            ));
        }
        lines.push(String::from(")"));
    }

    lines.push(String::new());
    lines.join("\n") + "\n"
}

fn render_reference_group(items: &[CoverageReference]) -> String {
    if items.is_empty() {
        return String::from("()");
    }

    let inner = items
        .iter()
        .map(|item| {
            format!(
                "(id: {}, relationship: {})",
                typst_string(&item.id),
                typst_string(&item.relationship)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("({inner},)")
}

fn render_framework_items(items: &[crate::coverage_attestation::CoverageFrameworkItem]) -> String {
    if items.is_empty() {
        return String::from("()");
    }

    let inner = items
        .iter()
        .map(|item| {
            format!(
                "(id: {}, classification: {})",
                typst_string(&item.id),
                typst_string(&item.classification)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("({inner},)")
}

fn typst_string(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::resolve_report_project_name;
    use std::path::Path;

    #[test]
    fn parse_report_project_name_from_threats_content_prefers_existing_text() {
        let threats_content = "# Threat Model: Single Read Report\n";
        let project_name = resolve_report_project_name(
            threats_content,
            None,
            Some(Path::new("/tmp/single-read-report")),
        );

        assert_eq!(project_name, "Single Read Report");
    }
}
