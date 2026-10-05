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
    // Compatibility API: invalid input produces an explicitly unrenderable
    // document. Commands use the checked API below to return the original error.
    try_build_report_data_typst(target_dir, template_dir)
        .unwrap_or_else(|error| format!("#panic({})\n", typst_string(&error)))
}

pub fn try_build_report_data_typst(
    target_dir: &Path,
    template_dir: &Path,
) -> Result<String, String> {
    let input = target_dir.join("threats.md");
    let threats_content = match fs::read_to_string(&input) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(format!("{}: {error}", input.display())),
    };
    let project_name = resolve_report_project_name(&threats_content, None, Some(target_dir));
    let findings = parse_threats_findings(&threats_content)
        .map_err(|error| format!("{}: {error}", input.display()))?;
    let risk_scores_path = target_dir.join("risk-scores.md");
    match fs::read_to_string(&risk_scores_path) {
        Ok(content) => validate_risk_scores(&risk_scores_path, &content)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(format!("{}: {error}", risk_scores_path.display())),
    }
    let images = detect_images(target_dir, template_dir);
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
        template_dir,
        &threats_content,
        &findings,
    ));
    Ok(output)
}

fn validate_risk_scores(path: &Path, content: &str) -> Result<(), String> {
    if risk_scores_are_valid(content) {
        return Ok(());
    }
    Err(format!(
        "{}: Scored Threat Table must contain ID, Component, Threat, Composite, and Severity columns, with nonempty finding IDs and severities",
        path.display()
    ))
}

fn risk_scores_are_valid(content: &str) -> bool {
    let source_rows = crate::parsers::parse_markdown_table(content, "## 2. Scored Threat Table");
    if source_rows.is_empty() {
        return true;
    }
    let required_columns = ["ID", "Component", "Threat", "Composite", "Severity"];
    let findings = crate::parsers::parse_risk_scores_findings(content);
    source_rows.iter().all(|row| {
        required_columns
            .iter()
            .all(|column| row.contains_key(*column))
    }) && !findings.is_empty()
        && findings
            .iter()
            .all(|row| !row.id.trim().is_empty() && !row.severity.trim().is_empty())
}

// The canonical Rust report path supplies the full Tier-3 template contract.
// Absent optional analysis is explicitly unavailable, never synthesized.
fn render_document_data(
    target: &Path,
    template_dir: &Path,
    content: &str,
    findings: &[crate::parsers::ThreatFinding],
) -> String {
    use serde_json::json;
    let scope = crate::parsers::parse_scope_data(content);
    let maestro = crate::infographic::extract_maestro_data(content);
    let brand_dir = template_dir
        .ancestors()
        .nth(3)
        .map(|root| root.join("brand/final"));
    let brand = crate::assets::detect_brand_assets(template_dir, brand_dir.as_deref());
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
        "has-logo-primary": brand.has_logo_primary, "has-logo-horizontal": brand.has_logo_horizontal,
        "logo-primary-path": brand.logo_primary_path, "logo-primary-dark-path": brand.logo_primary_dark_path,
        "logo-horizontal-path": brand.logo_horizontal_path,
        "has-risk-scores": false, "has-compensating-controls": false,
        "has-threat-report": report.executive_narrative.is_some() || !report.remediation_timeline.is_empty(),
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
            let valid_risk_assessment = !rows.is_empty() && risk_scores_are_valid(&risk);
            if valid_risk_assessment {
                values["has-risk-scores"] = json!(true);
                values["data-source-tier"] = json!(2);
                values["findings"] = json!(rows.iter().map(|f| json!({"id":f.id,"component":f.component,"threat":f.threat,"composite_score":f.composite_score,"severity":f.severity,"cvss":f.cvss,"exploitability":f.exploitability})).collect::<Vec<_>>());
            }
        }
    }
    if let Ok(text) = fs::read_to_string(target.join("compensating-controls.md")) {
        let data = crate::parse_compensating_controls_md(&text);
        if has_control_assessment(&text, &data) {
            values["has-compensating-controls"] = json!(true);
            values["data-source-tier"] = json!(1);
            values["findings"] = json!(data.findings.iter().map(|f| json!({"id":f.id,"component":f.component,"threat":f.threat,"residual_score":f.residual_score,"residual_severity":f.residual_severity,"control_status":f.control_status,"recommendation":f.recommendation})).collect::<Vec<_>>());
            values["coverage-matrix"] = json!(data.coverage_matrix.iter().map(|r| json!({"category":r.category,"found":r.found,"partial":r.partial,"missing":r.missing})).collect::<Vec<_>>());
            values["controls"] = json!(data.controls.iter().map(|r| json!({"component":r.component,"category":r.category,"status":r.status,"evidence":r.evidence,"effectiveness":r.effectiveness})).collect::<Vec<_>>());
            values["coverage-summary"] = json!({"total-found":data.coverage_summary.total_found,"total-partial":data.coverage_summary.total_partial,"total-missing":data.coverage_summary.total_missing});
        }
    }
    let tier = values["data-source-tier"].as_u64().unwrap();
    let active_findings = values["findings"].as_array().unwrap();
    let components = active_findings
        .iter()
        .map(|finding| {
            std::collections::BTreeMap::from([(
                "component".to_string(),
                finding["component"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            )])
        })
        .collect::<Vec<_>>();
    let remediation_findings = active_findings
        .iter()
        .map(|finding| {
            let field = |name: &str| finding[name].as_str().unwrap_or_default().to_string();
            crate::report_extraction::RemediationFinding {
                id: field("id"),
                threat: field("threat"),
                recommendation: field("recommendation"),
                control_status: field("control_status"),
                residual_severity: field("residual_severity"),
                severity: field("severity"),
                risk_level: field("risk_level"),
                mitigation: field("mitigation"),
            }
        })
        .collect::<Vec<_>>();
    let actions = crate::report_extraction::build_remediation_actions(
        &remediation_findings,
        tier as u8,
        values["has-compensating-controls"] == true,
        Some(&report),
    )
    .unwrap_or_default();
    values["component-distribution"] =
        json!(crate::parsers::parse_component_distribution(&components));
    values["remediation-actions"] = json!(actions.iter().map(|action| json!({
        "severity":action.severity, "finding-id":action.finding_id, "finding-name":action.finding_name,
        "recommendation":action.recommendation, "sla":action.sla, "status":action.status,
    })).collect::<Vec<_>>());
    let report_text = fs::read_to_string(target.join("threat-report.md")).ok();
    let trees = crate::attack_trees::parse_attack_trees(target, findings, report_text.as_deref());
    values["has-attack-trees"] = json!(!trees.is_empty());
    values["attack-trees"] = json!(trees.iter().map(|tree| {
        let image = resolve_report_image(target, template_dir, "attack-trees", &tree.id, "attack-tree");
        json!({"id":tree.id, "title":tree.title, "component":tree.component, "severity":tree.severity,
            "has-image":image.is_some(), "image-path":image.unwrap_or_default(),
            "narrative":tree.narrative, "remediation":tree.mitigation, "mermaid-code":tree.mermaid_code})
    }).collect::<Vec<_>>());
    let chain_text = fs::read_to_string(target.join("attack-chains.md")).ok();
    let chains = crate::attack_chains::parse_attack_chains(chain_text.as_deref())
        .into_iter()
        .filter(|chain| chain.surfaced)
        .collect::<Vec<_>>();
    values["has-attack-chains"] = json!(!chains.is_empty());
    values["attack-chains"] = json!(chains.iter().map(|chain| {
        let image = resolve_report_image(target, template_dir, "attack-chains", &chain.chain_id, "attack-chain");
        json!({"id":chain.chain_id, "title":chain.title, "layers":chain.layers.join(" → "), "max-severity":chain.max_severity,
            "has-image":image.is_some(), "image-path":image.unwrap_or_default(), "narrative":chain.narrative,
            "finding-ids":chain.findings.iter().map(|f| &f.finding_id).collect::<Vec<_>>()})
    }).collect::<Vec<_>>());
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

const MAX_REPORT_IMAGE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_REPORT_IMAGE_DIMENSION: u32 = 8192;
const MAX_REPORT_IMAGE_DECODE_BYTES: u64 = 64 * 1024 * 1024;

fn resolve_report_image(
    target: &Path,
    template_dir: &Path,
    directory: &str,
    id: &str,
    suffix: &str,
) -> Option<String> {
    if !is_report_image_id(directory, id) {
        return None;
    }
    let report_root = target.canonicalize().ok()?;
    let template_root = template_dir.canonicalize().ok()?;

    for id in [id.to_string(), id.to_ascii_lowercase()] {
        for extension in ["png", "jpg", "svg"] {
            let candidate = target
                .join(directory)
                .join(format!("{id}-{suffix}.{extension}"));
            let Ok(resolved) = candidate.canonicalize() else {
                continue;
            };
            if !resolved.starts_with(&report_root)
                || !resolved.is_file()
                || !is_usable_report_image(&resolved)
            {
                continue;
            }
            return Some(
                crate::assets::relative_path(&template_root, &resolved)
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
    None
}

fn is_report_image_id(directory: &str, id: &str) -> bool {
    let Some((prefix, numeric_suffix)) = id.split_once('-') else {
        return false;
    };
    if numeric_suffix.is_empty() || !numeric_suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }

    match directory {
        "attack-trees" => [
            "S", "T", "R", "I", "D", "E", "AG", "LLM", "AGP", "OI", "MI", "TE",
        ]
        .contains(&prefix),
        "attack-chains" => prefix == "CHAIN",
        _ => false,
    }
}

fn is_usable_report_image(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    if metadata.len() == 0 || metadata.len() > MAX_REPORT_IMAGE_BYTES {
        return false;
    }

    match path.extension().and_then(|extension| extension.to_str()) {
        Some(extension) if extension.eq_ignore_ascii_case("svg") => {
            let Ok(content) = fs::read_to_string(path) else {
                return false;
            };
            roxmltree::Document::parse(&content)
                .ok()
                .is_some_and(|document| document.root_element().tag_name().name() == "svg")
        }
        Some(extension)
            if extension.eq_ignore_ascii_case("png")
                || extension.eq_ignore_ascii_case("jpg")
                || extension.eq_ignore_ascii_case("jpeg") =>
        {
            let Ok(mut reader) =
                image::ImageReader::open(path).and_then(|reader| reader.with_guessed_format())
            else {
                return false;
            };
            let mut limits = image::Limits::default();
            limits.max_image_width = Some(MAX_REPORT_IMAGE_DIMENSION);
            limits.max_image_height = Some(MAX_REPORT_IMAGE_DIMENSION);
            limits.max_alloc = Some(MAX_REPORT_IMAGE_DECODE_BYTES);
            reader.limits(limits);
            reader.decode().is_ok()
        }
        _ => false,
    }
}

fn has_control_assessment(
    content: &str,
    data: &crate::compensating_controls::CompensatingControlsData,
) -> bool {
    if !data.findings.is_empty() {
        return true;
    }
    // A completed empty assessment may have a header-only residual table, but
    // only under a recognized severity subsection. Inventory-only controls
    // and populated tables the parser cannot consume are not completion proof.
    let lines: Vec<_> = content.lines().map(str::trim).collect();
    let coverage = lines
        .iter()
        .position(|line| *line == "## 2. Coverage Matrix")
        .map(|start| {
            let end = lines[start + 1..]
                .iter()
                .position(|line| line.starts_with("## "))
                .map_or(lines.len(), |offset| start + 1 + offset);
            &lines[start + 1..end]
        })
        .unwrap_or_default();
    let severity_sections = [
        "### Critical Residual Severity",
        "### High Residual Severity",
        "### Medium Residual Severity",
        "### Low Residual Severity",
    ];
    let has_empty_residual_table = coverage.iter().enumerate().any(|(section, heading)| {
        if !severity_sections.contains(heading) {
            return false;
        }
        let table = &coverage[section + 1..];
        let Some(header_index) = table.iter().position(|line| line.starts_with('|')) else {
            return false;
        };
        let Some(header) = table.get(header_index) else {
            return false;
        };
        let Some(separator) = table.get(header_index + 1) else {
            return false;
        };
        let cells: Vec<_> = header
            .split('|')
            .map(|cell| cell.trim().trim_matches('*'))
            .filter(|cell| !cell.is_empty())
            .collect();
        let separator_cells: Vec<_> = separator
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        let valid_separator = separator_cells.len() == cells.len()
            && separator_cells.iter().all(|cell| {
                cell.trim_matches(':').len() >= 3
                    && cell.trim_matches(':').chars().all(|c| c == '-')
            });
        let has_contract = [
            "Threat ID",
            "Component",
            "Threat",
            "Residual Score",
            "Residual Severity",
            "Control Status",
        ]
        .iter()
        .all(|column| cells.contains(column));
        let has_data = table[header_index + 2..]
            .iter()
            .take_while(|line| line.starts_with('|'))
            .any(|line| !line.trim().is_empty());
        valid_separator && has_contract && !has_data
    });
    if has_empty_residual_table {
        return true;
    }
    // Explicit numeric coverage metadata, including all-zero rows, is also
    // assessment evidence. Unrecognized tables and error prose remain absent.
    ["Coverage Distribution", "## 1. Executive Summary"]
        .iter()
        .any(|heading| {
            let mut seen = [false; 3];
            for row in crate::parsers::parse_markdown_table(content, heading) {
                let index = match row.get("Status").map(String::as_str) {
                    Some("Found" | "Control Found") => 0,
                    Some("Partial" | "Partial Control") => 1,
                    Some("Missing" | "No Control") => 2,
                    _ => continue,
                };
                if seen[index]
                    || !row
                        .get("Count")
                        .is_some_and(|count| count.parse::<usize>().is_ok())
                {
                    return false;
                }
                seen[index] = true;
            }
            seen.into_iter().all(|present| present)
        })
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
