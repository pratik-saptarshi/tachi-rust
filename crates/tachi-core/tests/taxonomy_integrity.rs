use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use tachi_core::parsers::{validate_source_attribution, SourceAttributionRecord, ThreatFinding};

const CATALOG_FILENAMES: &[&str] = &[
    "owasp.yaml",
    "mitre-attack.yaml",
    "mitre-atlas.yaml",
    "nist-ai-rmf.yaml",
    "nist-ai-600-1.yaml",
    "cwe.yaml",
    "tachi-control-category.yaml",
    "tachi-stride-ai-category.yaml",
    "aisvs.yaml",
];

const TAXONOMIES: &[&str] = &[
    "owasp",
    "mitre-attack",
    "mitre-atlas",
    "nist-ai-rmf",
    "nist-ai-600-1",
    "cwe",
    "tachi-control-category",
    "tachi-stride-ai-category",
    "aisvs",
];

const EDGE_TYPES: &[&str] = &["primary", "related", "superseded"];
const CONFIDENCE_VALUES: &[&str] = &["high", "medium", "low"];
const PRIMARY_EDGE_FLOOR: usize = 500;
const UPSTREAM_PARITY_DELTA_2026_08_13: &[(&str, &str, &str, &str, &str, &str)] = &[
    (
        "owasp",
        "LLM01",
        "cwe",
        "CWE-77",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "cwe",
        "CWE-200",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM04",
        "cwe",
        "CWE-1395",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM05",
        "cwe",
        "CWE-20",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM01",
        "cwe",
        "CWE-20",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM01",
        "cwe",
        "CWE-94",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM01",
        "cwe",
        "CWE-1427",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "cwe",
        "CWE-201",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "cwe",
        "CWE-538",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "cwe",
        "CWE-359",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM04",
        "cwe",
        "CWE-1357",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM04",
        "cwe",
        "CWE-494",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM04",
        "cwe",
        "CWE-829",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM05",
        "cwe",
        "CWE-502",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM05",
        "cwe",
        "CWE-915",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM05",
        "cwe",
        "CWE-1395",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM03",
        "cwe",
        "CWE-250",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM03",
        "cwe",
        "CWE-269",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM03",
        "cwe",
        "CWE-285",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM03",
        "cwe",
        "CWE-732",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "API8",
        "cwe",
        "CWE-16",
        "primary",
        "https://owasp.org/API-Security/editions/2023/en/0xa8-security-misconfiguration/",
    ),
    (
        "mitre-attack",
        "T1562",
        "cwe",
        "CWE-693",
        "primary",
        "https://attack.mitre.org/techniques/T1562/",
    ),
    (
        "owasp",
        "ASI01",
        "cwe",
        "CWE-1427",
        "primary",
        "https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/",
    ),
    (
        "owasp",
        "ASI04",
        "cwe",
        "CWE-1357",
        "primary",
        "https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/",
    ),
    (
        "owasp",
        "ASI07",
        "cwe",
        "CWE-319",
        "primary",
        "https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/",
    ),
    (
        "owasp",
        "ASI09",
        "cwe",
        "CWE-1426",
        "primary",
        "https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/",
    ),
    (
        "owasp",
        "ASI10",
        "cwe",
        "CWE-1427",
        "primary",
        "https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/",
    ),
    (
        "owasp",
        "LLM01",
        "mitre-atlas",
        "AML.T0051",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0051",
    ),
    (
        "owasp",
        "LLM05",
        "mitre-atlas",
        "AML.T0020",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0020",
    ),
    (
        "owasp",
        "LLM05",
        "mitre-atlas",
        "AML.T0018",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0018",
    ),
    (
        "owasp",
        "LLM04",
        "mitre-atlas",
        "AML.T0010",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0010",
    ),
    (
        "owasp",
        "LLM04",
        "mitre-attack",
        "T1195",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM03",
        "mitre-atlas",
        "AML.T0061",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0061",
    ),
    (
        "owasp",
        "LLM03",
        "mitre-attack",
        "T1548",
        "primary",
        "https://attack.mitre.org/techniques/T1548/",
    ),
    (
        "owasp",
        "LLM06",
        "mitre-attack",
        "T1499",
        "primary",
        "https://attack.mitre.org/techniques/T1499/",
    ),
    (
        "owasp",
        "LLM06",
        "mitre-attack",
        "T1498",
        "primary",
        "https://attack.mitre.org/techniques/T1498/",
    ),
    (
        "owasp",
        "LLM01",
        "cwe",
        "CWE-79",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM01",
        "cwe",
        "CWE-116",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "cwe",
        "CWE-532",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "cwe",
        "CWE-209",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM03",
        "cwe",
        "CWE-284",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "mitre-attack",
        "T1005",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "mitre-attack",
        "T1213",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM04",
        "mitre-attack",
        "T1195.001",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM04",
        "mitre-attack",
        "T1195.002",
        "primary",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "mitre-attack",
        "T1190",
        "cwe",
        "CWE-20",
        "primary",
        "https://attack.mitre.org/techniques/T1190/",
    ),
    (
        "mitre-atlas",
        "AML.T0060",
        "mitre-attack",
        "T1557",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0060",
    ),
    (
        "mitre-atlas",
        "AML.T0000",
        "mitre-attack",
        "T1213",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0000",
    ),
    (
        "mitre-atlas",
        "AML.T0003",
        "mitre-attack",
        "T1195.002",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0003",
    ),
    (
        "mitre-atlas",
        "AML.T0016",
        "mitre-attack",
        "T1195.002",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0016",
    ),
    (
        "mitre-atlas",
        "AML.T0029",
        "mitre-attack",
        "T1499",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0029",
    ),
    (
        "mitre-atlas",
        "AML.T0001",
        "mitre-attack",
        "T1213",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0001",
    ),
    (
        "mitre-atlas",
        "AML.T0005",
        "mitre-attack",
        "T1213",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0005",
    ),
    (
        "mitre-atlas",
        "AML.T0025",
        "mitre-attack",
        "T1005",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0025",
    ),
    (
        "mitre-atlas",
        "AML.T0037",
        "mitre-attack",
        "T1213",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0037",
    ),
    (
        "mitre-atlas",
        "AML.T0043",
        "mitre-attack",
        "T1190",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0043",
    ),
    (
        "mitre-atlas",
        "AML.T0048",
        "mitre-attack",
        "T1499",
        "primary",
        "https://atlas.mitre.org/techniques/AML.T0048",
    ),
    (
        "mitre-atlas",
        "AML.T0011",
        "mitre-attack",
        "T1204",
        "related",
        "https://atlas.mitre.org/techniques/AML.T0011 (ATT&CK-reference T1204)",
    ),
    (
        "mitre-atlas",
        "AML.T0037",
        "mitre-attack",
        "T1005",
        "related",
        "https://atlas.mitre.org/techniques/AML.T0037 (ATT&CK-reference T1005)",
    ),
    (
        "owasp",
        "LLM01",
        "mitre-atlas",
        "AML.T0054",
        "related",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM02",
        "mitre-atlas",
        "AML.T0024",
        "related",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM07",
        "mitre-atlas",
        "AML.T0048",
        "related",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
    (
        "owasp",
        "LLM06",
        "mitre-atlas",
        "AML.T0025",
        "related",
        "https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/",
    ),
];
const PRE_MISINFORMATION_ID_PREFIXES: &[&str] =
    &["S", "T", "R", "I", "D", "E", "AG", "LLM", "AGP", "OI"];
const PRE_OUTPUT_INTEGRITY_ID_PREFIXES: &[&str] =
    &["S", "T", "R", "I", "D", "E", "AG", "LLM", "AGP"];

#[derive(Debug)]
struct CatalogRecord {
    id: String,
    body: Vec<String>,
}

#[derive(Debug)]
struct CrosswalkEdge {
    source_taxonomy: String,
    source_id: String,
    target_taxonomy: String,
    target_id: String,
    edge_type: String,
    confidence: String,
    citation: String,
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn taxonomy_dir(root: &Path) -> PathBuf {
    root.join("schemas/taxonomy")
}

fn parse_catalog_records(text: &str) -> Vec<CatalogRecord> {
    let mut records = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_body = Vec::new();

    for line in text.lines() {
        if let Some(rest) = line.trim().strip_prefix("- id: ") {
            if let Some(id) = current_id.replace(rest.trim().to_string()) {
                records.push(CatalogRecord {
                    id: id.trim_matches(['"', '\'']).to_string(),
                    body: std::mem::take(&mut current_body),
                });
            }
        } else if current_id.is_some() {
            current_body.push(line.to_string());
        }
    }

    if let Some(id) = current_id {
        records.push(CatalogRecord {
            id: id.trim_matches(['"', '\'']).to_string(),
            body: current_body,
        });
    }

    records
}

fn scalar_value(line: &str, key: &str) -> Option<String> {
    line.trim()
        .strip_prefix(key)
        .map(|value| value.trim().trim_matches('"').to_string())
}

fn contains_key(record: &CatalogRecord, key: &str) -> bool {
    record
        .body
        .iter()
        .any(|line| line.trim_start().starts_with(key))
}

fn record_url(record: &CatalogRecord) -> Option<String> {
    record
        .body
        .iter()
        .find_map(|line| scalar_value(line, "url:"))
}

fn is_url_or_existing_file(root: &Path, value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://") || root.join(value).is_file()
}

fn finding_id_prefixes(schema_text: &str) -> BTreeSet<String> {
    schema_text
        .lines()
        .find_map(|line| scalar_value(line, "pattern:"))
        .and_then(|pattern| pattern.strip_prefix("^(").map(str::to_string))
        .and_then(|pattern| {
            pattern
                .split_once(")-\\\\d+$")
                .map(|(prefixes, _)| prefixes.to_string())
        })
        .expect("finding.id.pattern should use the expected prefix alternation shape")
        .split('|')
        .map(ToOwned::to_owned)
        .collect()
}

fn schema_version(schema_text: &str) -> String {
    schema_text
        .lines()
        .find_map(|line| scalar_value(line, "schema_version:"))
        .expect("schema_version should be present")
}

fn finding_id_matches(prefixes: &BTreeSet<String>, value: &str) -> bool {
    let Some((prefix, suffix)) = value.split_once('-') else {
        return false;
    };

    prefixes.contains(prefix) && !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
}

fn top_level_scalar(text: &str, key: &str) -> Option<String> {
    text.lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| scalar_value(line, key))
}

fn source_attribution_records(text: &str) -> Vec<SourceAttributionRecord> {
    let mut records = Vec::new();
    let mut current = SourceAttributionRecord::default();
    let mut in_source_attribution = false;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == "source_attribution:" {
            in_source_attribution = true;
            continue;
        }
        if !in_source_attribution {
            continue;
        }
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if !line.starts_with("  ") {
            break;
        }

        if let Some(value) = trimmed.strip_prefix("- taxonomy:") {
            if !current.taxonomy.is_empty() {
                records.push(std::mem::take(&mut current));
            }
            current.taxonomy = value.trim().to_string();
        } else if let Some(value) = trimmed.strip_prefix("id:") {
            current.id = value.trim().to_string();
        } else if let Some(value) = trimmed.strip_prefix("relationship:") {
            current.relationship = value.trim().to_string();
        }
    }

    if !current.taxonomy.is_empty() {
        records.push(current);
    }

    records
}

fn profile_sort_key(id: &str) -> (u32, u32) {
    let (major, minor) = id.split_once('.').unwrap_or((id, "0"));
    (major.parse().unwrap_or(0), minor.parse().unwrap_or(0))
}

fn nist_sort_key(id: &str) -> (String, u32, u32, String) {
    let (function, number) = id.split_once(' ').unwrap_or((id, ""));
    let (major, minor) = number.split_once('.').unwrap_or((number, "0"));
    (
        function.to_string(),
        major.parse().unwrap_or(0),
        minor.parse().unwrap_or(0),
        id.to_string(),
    )
}

fn parse_crosswalk_edges(text: &str) -> Vec<CrosswalkEdge> {
    let mut edges = Vec::new();
    let mut source_taxonomy = String::new();
    let mut source_id = String::new();
    let mut target_taxonomy = String::new();
    let mut target_id = String::new();
    let mut edge_type = String::new();
    let mut confidence = String::new();
    let mut citation = String::new();
    let mut endpoint: Option<&str> = None;

    for line in text.lines() {
        let trimmed = line.trim();
        match trimmed {
            "- source:" => {
                if !source_taxonomy.is_empty() {
                    edges.push(CrosswalkEdge {
                        source_taxonomy: std::mem::take(&mut source_taxonomy),
                        source_id: std::mem::take(&mut source_id),
                        target_taxonomy: std::mem::take(&mut target_taxonomy),
                        target_id: std::mem::take(&mut target_id),
                        edge_type: std::mem::take(&mut edge_type),
                        confidence: std::mem::take(&mut confidence),
                        citation: std::mem::take(&mut citation),
                    });
                }
                endpoint = Some("source");
            }
            "target:" => endpoint = Some("target"),
            _ => {
                if let Some(value) = scalar_value(trimmed, "taxonomy:") {
                    match endpoint {
                        Some("source") => source_taxonomy = value,
                        Some("target") => target_taxonomy = value,
                        _ => {}
                    }
                } else if let Some(value) = scalar_value(trimmed, "id:") {
                    match endpoint {
                        Some("source") => source_id = value,
                        Some("target") => target_id = value,
                        _ => {}
                    }
                } else if let Some(value) = scalar_value(trimmed, "edge_type:") {
                    edge_type = value;
                    endpoint = None;
                } else if let Some(value) = scalar_value(trimmed, "confidence:") {
                    confidence = value;
                } else if let Some(value) = scalar_value(trimmed, "citation:") {
                    citation = value;
                }
            }
        }
    }

    if !source_taxonomy.is_empty() {
        edges.push(CrosswalkEdge {
            source_taxonomy,
            source_id,
            target_taxonomy,
            target_id,
            edge_type,
            confidence,
            citation,
        });
    }

    edges
}

#[test]
fn taxonomy_integrity_contract_is_rust_native() {
    let root = workspace_root();
    assert!(
        !root
            .join("tests/schemas/test_taxonomy_integrity.py")
            .exists(),
        "taxonomy integrity coverage should live in Rust tests, not pytest"
    );

    let mut catalog_ids: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();

    for filename in CATALOG_FILENAMES {
        let path = taxonomy_dir(&root).join(filename);
        let text = fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!(
                "expected taxonomy catalog {} to load: {err}",
                path.display()
            )
        });
        let records = parse_catalog_records(&text);
        assert!(
            !records.is_empty(),
            "{filename}: expected non-empty records"
        );

        let mut seen_ids = BTreeSet::new();
        let mut ids = Vec::new();
        for record in &records {
            assert!(
                seen_ids.insert(record.id.clone()),
                "{filename}: duplicate id {:?}",
                record.id
            );
            ids.push(record.id.clone());

            for key in ["full_id:", "name:", "url:"] {
                assert!(
                    contains_key(record, key),
                    "{filename}: {} missing {key}",
                    record.id
                );
            }

            if *filename == "cwe.yaml" {
                assert!(
                    !contains_key(record, "cwe_refs:"),
                    "{filename}: {} must not carry cwe_refs",
                    record.id
                );
            } else {
                assert!(
                    contains_key(record, "cwe_refs:"),
                    "{filename}: {} missing cwe_refs",
                    record.id
                );
            }

            let url = record_url(record).expect("url checked above");
            assert!(
                is_url_or_existing_file(&root, &url),
                "{filename}: {} url {url:?} is not URL-shaped or an existing file",
                record.id
            );
        }

        let mut expected = ids.clone();
        if *filename == "nist-ai-rmf.yaml" {
            expected.sort_by_key(|id| nist_sort_key(id));
        } else if *filename == "nist-ai-600-1.yaml" {
            expected.sort_by_key(|id| profile_sort_key(id));
        } else {
            expected.sort();
        }
        assert_eq!(ids, expected, "{filename}: records should be sorted by id");

        catalog_ids.insert(filename.trim_end_matches(".yaml"), seen_ids);
    }

    let crosswalk_path = taxonomy_dir(&root).join("crosswalk.yaml");
    let crosswalk = fs::read_to_string(&crosswalk_path).unwrap_or_else(|err| {
        panic!(
            "expected taxonomy crosswalk {} to load: {err}",
            crosswalk_path.display()
        )
    });
    let edges = parse_crosswalk_edges(&crosswalk);
    assert!(
        !edges.is_empty(),
        "crosswalk.yaml: expected non-empty edges"
    );
    assert_eq!(
        edges.len(),
        645,
        "crosswalk edge total after upstream parity"
    );
    let actual_edge_citations: BTreeMap<_, _> = edges
        .iter()
        .map(|edge| {
            (
                (
                    edge.source_taxonomy.as_str(),
                    edge.source_id.as_str(),
                    edge.target_taxonomy.as_str(),
                    edge.target_id.as_str(),
                    edge.edge_type.as_str(),
                ),
                edge.citation.as_str(),
            )
        })
        .collect();
    for &(source_taxonomy, source_id, target_taxonomy, target_id, edge_type, citation) in
        UPSTREAM_PARITY_DELTA_2026_08_13
    {
        assert_eq!(
            actual_edge_citations.get(&(source_taxonomy, source_id, target_taxonomy, target_id, edge_type)),
            Some(&citation),
            "missing upstream edge or changed citation: {source_taxonomy}:{source_id} -> {target_taxonomy}:{target_id} ({edge_type})",
        );
    }

    let mut seen_edges = BTreeSet::new();
    let mut primary_count = 0;
    let mut related_count = 0;
    for edge in &edges {
        assert!(TAXONOMIES.contains(&edge.source_taxonomy.as_str()));
        assert!(TAXONOMIES.contains(&edge.target_taxonomy.as_str()));
        assert!(EDGE_TYPES.contains(&edge.edge_type.as_str()));
        assert!(CONFIDENCE_VALUES.contains(&edge.confidence.as_str()));
        assert!(
            !edge.citation.is_empty() && is_url_or_existing_file(&root, edge.citation.as_str()),
            "crosswalk.yaml: citation {:?} is not URL-shaped or an existing file",
            edge.citation
        );

        assert!(
            catalog_ids
                .get(edge.source_taxonomy.as_str())
                .is_some_and(|ids| ids.contains(&edge.source_id)),
            "crosswalk.yaml: source {:?}:{:?} not found",
            edge.source_taxonomy,
            edge.source_id
        );
        assert!(
            catalog_ids
                .get(edge.target_taxonomy.as_str())
                .is_some_and(|ids| ids.contains(&edge.target_id)),
            "crosswalk.yaml: target {:?}:{:?} not found",
            edge.target_taxonomy,
            edge.target_id
        );

        assert!(
            seen_edges.insert((
                edge.source_taxonomy.as_str(),
                edge.source_id.as_str(),
                edge.target_taxonomy.as_str(),
                edge.target_id.as_str(),
                edge.edge_type.as_str(),
            )),
            "crosswalk.yaml: duplicate edge {edge:?}"
        );

        if edge.edge_type == "primary" {
            primary_count += 1;
        } else if edge.edge_type == "related" {
            related_count += 1;
        }
    }

    assert_eq!(primary_count, 608, "crosswalk primary edge total");
    assert_eq!(related_count, 37, "crosswalk related edge total");
    assert!(
        primary_count >= PRIMARY_EDGE_FLOOR,
        "crosswalk.yaml: {primary_count} primary edges below floor of {PRIMARY_EDGE_FLOOR}"
    );
}

#[test]
fn misinformation_id_schema_contract_is_rust_native() {
    let root = workspace_root();
    assert!(
        !root.join("tests/scripts/test_misinformation.py").exists(),
        "misinformation schema coverage should live in Rust tests, not pytest"
    );

    let schema_path = root.join("schemas/finding.yaml");
    let schema = fs::read_to_string(&schema_path).unwrap_or_else(|err| {
        panic!(
            "expected finding schema {} to load: {err}",
            schema_path.display()
        )
    });
    let prefixes = finding_id_prefixes(&schema);

    for prefix in PRE_MISINFORMATION_ID_PREFIXES {
        let finding_id = format!("{prefix}-1");
        assert!(
            finding_id_matches(&prefixes, &finding_id),
            "pre-1.7 ID prefix {prefix:?} should remain valid"
        );
    }

    for finding_id in ["MI-1", "MI-10", "MI-99"] {
        assert!(
            finding_id_matches(&prefixes, finding_id),
            "MI finding ID {finding_id:?} should match the finding.id pattern"
        );
    }

    for finding_id in ["MI1", "MIA-1", "mi-1", "", "MI-abc", "MI-"] {
        assert!(
            !finding_id_matches(&prefixes, finding_id),
            "malformed finding ID {finding_id:?} should not match the finding.id pattern"
        );
    }
}

#[test]
fn output_integrity_schema_contract_is_rust_native() {
    let root = workspace_root();
    assert!(
        !root.join("tests/scripts/test_output_integrity.py").exists(),
        "output-integrity schema coverage should live in Rust tests, not pytest"
    );

    let schema_path = root.join("schemas/finding.yaml");
    let schema = fs::read_to_string(&schema_path).unwrap_or_else(|err| {
        panic!(
            "expected finding schema {} to load: {err}",
            schema_path.display()
        )
    });
    assert_eq!(schema_version(&schema), "1.9");
    let prefixes = finding_id_prefixes(&schema);

    for prefix in PRE_OUTPUT_INTEGRITY_ID_PREFIXES {
        let finding_id = format!("{prefix}-1");
        assert!(
            finding_id_matches(&prefixes, &finding_id),
            "pre-1.6 ID prefix {prefix:?} should remain valid"
        );
    }

    for finding_id in ["OI-1", "OI-10", "OI-99", "OI-100"] {
        assert!(
            finding_id_matches(&prefixes, finding_id),
            "OI finding ID {finding_id:?} should match the finding.id pattern"
        );
    }

    for finding_id in [
        "OI1", "OIA-1", "oi-1", "", "OI-", "OI-abc", "XX-1", "OI-1 ", " OI-1",
    ] {
        assert!(
            !finding_id_matches(&prefixes, finding_id),
            "malformed finding ID {finding_id:?} should not match the finding.id pattern"
        );
    }

    let fixture_dir = root.join("tests/scripts/fixtures/output_integrity");
    let valid_path = fixture_dir.join("valid_oi_finding.yaml");
    let invalid_path = fixture_dir.join("invalid_attribution_finding.yaml");
    let valid_text = fs::read_to_string(&valid_path).unwrap_or_else(|err| {
        panic!(
            "expected valid OI fixture {} to load: {err}",
            valid_path.display()
        )
    });
    let invalid_text = fs::read_to_string(&invalid_path).unwrap_or_else(|err| {
        panic!(
            "expected invalid OI fixture {} to load: {err}",
            invalid_path.display()
        )
    });

    let valid_records = source_attribution_records(&valid_text);
    assert_eq!(
        top_level_scalar(&valid_text, "id:").as_deref(),
        Some("OI-1")
    );
    assert_eq!(
        top_level_scalar(&valid_text, "category:").as_deref(),
        Some("llm")
    );
    assert!(
        valid_records.iter().any(|record| record.taxonomy == "owasp"
            && record.id == "LLM10"
            && record.relationship == "primary"),
        "valid OI fixture should cite OWASP LLM05 as primary"
    );
    assert!(
        validate_source_attribution(
            &[ThreatFinding {
                id: String::from("OI-1"),
                source_attribution: Some(valid_records),
                ..ThreatFinding::default()
            }],
            &taxonomy_dir(&root),
        )
        .is_empty(),
        "valid OI fixture source attribution should resolve against the catalog"
    );

    let invalid_records = source_attribution_records(&invalid_text);
    assert!(
        invalid_records
            .iter()
            .any(|record| record.taxonomy == "cwe" && record.id == "CWE-9999"),
        "invalid OI fixture should retain its absent CWE citation"
    );
    let errors = validate_source_attribution(
        &[ThreatFinding {
            id: String::from("OI-99"),
            source_attribution: Some(invalid_records),
            ..ThreatFinding::default()
        }],
        &taxonomy_dir(&root),
    );
    assert!(
        errors
            .iter()
            .any(|error| error.record.taxonomy == "cwe" && error.record.id == "CWE-9999"),
        "invalid OI fixture should fail on its absent CWE identifier"
    );
}

#[test]
fn aisvs_schema_and_catalog_contract_are_rust_native() {
    let root = workspace_root();
    assert!(
        !root.join("tests/scripts/test_aisvs.py").exists(),
        "AISVS schema coverage should live in Rust tests, not pytest"
    );

    let taxonomy_path = taxonomy_dir(&root).join("aisvs.yaml");
    let taxonomy = fs::read_to_string(&taxonomy_path).unwrap_or_else(|err| {
        panic!(
            "expected AISVS taxonomy {} to load: {err}",
            taxonomy_path.display()
        )
    });
    let records = parse_catalog_records(&taxonomy);
    assert_eq!(records.len(), 12, "aisvs.yaml: expected 12 control records");
    assert_eq!(
        records.first().map(|record| record.id.as_str()),
        Some("C01")
    );
    assert_eq!(records.last().map(|record| record.id.as_str()), Some("C12"));

    for record in &records {
        for key in [
            "full_id:",
            "name:",
            "url:",
            "cwe_refs:",
            "capability:",
            "feature:",
            "task:",
            "function:",
            "validation_command:",
            "acceptance_criteria:",
        ] {
            assert!(
                contains_key(record, key),
                "aisvs.yaml: {} missing {key}",
                record.id
            );
        }
    }

    let schema_path = root.join("schemas/aisvs.yaml");
    let schema = fs::read_to_string(&schema_path).unwrap_or_else(|err| {
        panic!(
            "expected AISVS schema {} to load: {err}",
            schema_path.display()
        )
    });
    assert_eq!(schema_version(&schema), "1.0");
    assert!(schema.contains("framework_name:"));
    assert!(schema.contains("value: \"AISVS 1.0\""));
    assert!(schema.contains("framework_version:"));
    assert!(schema.contains("value: \"1.0\""));
    assert!(schema.contains("count: 12"));
    assert!(schema.contains("crates/tachi-core/src/aisvs.rs"));
    assert!(schema.contains("crates/tachi-core/tests/aisvs_controls.rs"));
    for control_id in [
        "C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08", "C09", "C10", "C11", "C12",
    ] {
        assert!(
            schema.contains(control_id),
            "aisvs.yaml: missing control id {control_id}"
        );
    }
}
