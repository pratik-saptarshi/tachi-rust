use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::parsers::{SourceAttributionRecord, ThreatFinding};

pub const ORDERED_FRAMEWORKS: [&str; 6] = [
    "owasp",
    "mitre-attack",
    "mitre-atlas",
    "nist-ai-rmf",
    "nist-ai-600-1",
    "cwe",
];

const MITRE_PREFIXES: [(&str, &str); 2] = [("mitre-attack", "ATT&CK:"), ("mitre-atlas", "ATLAS:")];

const TAXONOMY_REF_GROUPS: [(&str, &[&str]); 4] = [
    ("owasp_refs", &["owasp"]),
    ("mitre_refs", &["mitre-attack", "mitre-atlas"]),
    ("nist_refs", &["nist-ai-rmf", "nist-ai-600-1"]),
    ("cwe_refs", &["cwe"]),
];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageReference {
    pub id: String,
    pub relationship: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageFindingRow {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub owasp_refs: Vec<CoverageReference>,
    pub mitre_refs: Vec<CoverageReference>,
    pub nist_refs: Vec<CoverageReference>,
    pub cwe_refs: Vec<CoverageReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageFrameworkItem {
    pub id: String,
    pub classification: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageFrameworkAggregate {
    pub framework: String,
    pub yaml_record_count: usize,
    pub in_scope_yaml_record_count: usize,
    pub covered_count: usize,
    pub partial_count: usize,
    pub gap_count: usize,
    pub coverage_percentage: String,
    pub items: Vec<CoverageFrameworkItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FrameworkRecord {
    pub id: String,
    #[serde(default)]
    pub out_of_scope: bool,
}

impl FrameworkRecord {
    pub fn new(id: impl Into<String>, out_of_scope: bool) -> Self {
        Self {
            id: id.into(),
            out_of_scope,
        }
    }
}

pub trait TaxonomyStore {
    fn load_framework_records(
        &self,
        framework_name: &str,
        in_scope_only: bool,
    ) -> Vec<FrameworkRecord>;
}

struct FilesystemTaxonomyStore {
    taxonomy_dir: PathBuf,
}

impl TaxonomyStore for FilesystemTaxonomyStore {
    fn load_framework_records(
        &self,
        framework_name: &str,
        in_scope_only: bool,
    ) -> Vec<FrameworkRecord> {
        load_framework_yaml_records_from_dir(&self.taxonomy_dir, framework_name, in_scope_only)
    }
}

pub fn build_per_finding_rows(findings: &[ThreatFinding]) -> Vec<CoverageFindingRow> {
    let mut rows = Vec::with_capacity(findings.len());

    for finding in findings {
        let mut ref_buckets = BTreeMap::from([
            ("owasp_refs", Vec::new()),
            ("mitre_refs", Vec::new()),
            ("nist_refs", Vec::new()),
            ("cwe_refs", Vec::new()),
        ]);

        for reference in finding.source_attribution.as_deref().unwrap_or_default() {
            let Some((bucket_key, display_id)) = map_reference(reference) else {
                continue;
            };

            ref_buckets
                .entry(bucket_key)
                .or_default()
                .push(CoverageReference {
                    id: display_id,
                    relationship: reference.relationship.clone(),
                });
        }

        rows.push(CoverageFindingRow {
            id: finding.id.clone(),
            title: if finding.threat.is_empty() {
                finding.component.clone()
            } else {
                finding.threat.clone()
            },
            severity: finding.risk_level.to_ascii_lowercase(),
            owasp_refs: ref_buckets.remove("owasp_refs").unwrap_or_default(),
            mitre_refs: ref_buckets.remove("mitre_refs").unwrap_or_default(),
            nist_refs: ref_buckets.remove("nist_refs").unwrap_or_default(),
            cwe_refs: ref_buckets.remove("cwe_refs").unwrap_or_default(),
        });
    }

    rows
}

pub fn build_per_framework_aggregates(
    findings: &[ThreatFinding],
) -> Vec<CoverageFrameworkAggregate> {
    let store = FilesystemTaxonomyStore {
        taxonomy_dir: workspace_taxonomy_dir(),
    };
    build_per_framework_aggregates_from_store(&store, findings)
}

pub fn build_per_framework_aggregates_in_dir(
    taxonomy_dir: &Path,
    findings: &[ThreatFinding],
) -> Vec<CoverageFrameworkAggregate> {
    let store = FilesystemTaxonomyStore {
        taxonomy_dir: taxonomy_dir.to_path_buf(),
    };
    build_per_framework_aggregates_from_store(&store, findings)
}

pub fn build_per_framework_aggregates_from_store(
    store: &dyn TaxonomyStore,
    findings: &[ThreatFinding],
) -> Vec<CoverageFrameworkAggregate> {
    let mut aggregates = Vec::with_capacity(ORDERED_FRAMEWORKS.len());
    let raw_counts = load_framework_yaml_record_counts_from_store(store);
    let in_scope_counts = load_framework_yaml_in_scope_record_counts_from_store(store);

    for framework in ORDERED_FRAMEWORKS {
        let yaml_record_count = raw_counts.get(framework).copied().unwrap_or(0);
        let in_scope_yaml_record_count = in_scope_counts.get(framework).copied().unwrap_or(0);
        let records = store.load_framework_records(framework, true);
        let items = classify_framework_items(findings, framework, &records);

        aggregates.push(build_per_framework_aggregate(
            framework,
            items,
            yaml_record_count,
            in_scope_yaml_record_count,
        ));
    }

    aggregates
}

fn map_reference(reference: &SourceAttributionRecord) -> Option<(&'static str, String)> {
    for (bucket_key, taxonomies) in TAXONOMY_REF_GROUPS {
        if taxonomies.contains(&reference.taxonomy.as_str()) {
            let display_id = if let Some((_, prefix)) = MITRE_PREFIXES
                .iter()
                .find(|(name, _)| *name == reference.taxonomy)
            {
                format!("{prefix}{}", reference.id)
            } else {
                reference.id.clone()
            };

            return Some((bucket_key, display_id));
        }
    }

    None
}

fn classify_framework_items(
    findings: &[ThreatFinding],
    framework_name: &str,
    framework_records: &[FrameworkRecord],
) -> Vec<CoverageFrameworkItem> {
    let mut items = Vec::with_capacity(framework_records.len());

    for record in framework_records {
        let mut relationships = Vec::new();

        for finding in findings {
            for ref_record in finding.source_attribution.as_deref().unwrap_or_default() {
                if ref_record.taxonomy == framework_name && ref_record.id == record.id {
                    relationships.push(ref_record.relationship.clone());
                }
            }
        }

        let classification = if relationships
            .iter()
            .any(|relationship| relationship == "primary")
        {
            "covered"
        } else if relationships
            .iter()
            .any(|relationship| relationship == "related" || relationship == "derived")
        {
            "partial"
        } else {
            "gap"
        };

        items.push(CoverageFrameworkItem {
            id: record.id.clone(),
            classification: classification.to_string(),
        });
    }

    items
}

fn build_per_framework_aggregate(
    framework_name: &str,
    items: Vec<CoverageFrameworkItem>,
    yaml_record_count: usize,
    in_scope_yaml_record_count: usize,
) -> CoverageFrameworkAggregate {
    let covered_count = items
        .iter()
        .filter(|item| item.classification == "covered")
        .count();
    let partial_count = items
        .iter()
        .filter(|item| item.classification == "partial")
        .count();
    let gap_count = items
        .iter()
        .filter(|item| item.classification == "gap")
        .count();
    let coverage_percentage = if in_scope_yaml_record_count == 0 {
        String::from("N/A")
    } else {
        format!(
            "{:.2}%",
            (covered_count as f64 / in_scope_yaml_record_count as f64) * 100.0
        )
    };

    CoverageFrameworkAggregate {
        framework: framework_name.to_string(),
        yaml_record_count,
        in_scope_yaml_record_count,
        covered_count,
        partial_count,
        gap_count,
        coverage_percentage,
        items,
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn workspace_taxonomy_dir() -> PathBuf {
    workspace_root().join("schemas/taxonomy")
}

pub fn load_framework_yaml_records_from_dir(
    taxonomy_dir: &Path,
    framework_name: &str,
    in_scope_only: bool,
) -> Vec<FrameworkRecord> {
    let path = taxonomy_dir.join(format!("{framework_name}.yaml"));
    load_framework_yaml_records_checked(&path, in_scope_only).unwrap_or_default()
}

pub fn load_framework_yaml_record_counts() -> BTreeMap<String, usize> {
    let store = FilesystemTaxonomyStore {
        taxonomy_dir: workspace_taxonomy_dir(),
    };
    load_framework_yaml_record_counts_from_store(&store)
}

pub fn load_framework_yaml_record_counts_from_dir(taxonomy_dir: &Path) -> BTreeMap<String, usize> {
    let store = FilesystemTaxonomyStore {
        taxonomy_dir: taxonomy_dir.to_path_buf(),
    };
    load_framework_yaml_record_counts_from_store(&store)
}

pub fn load_framework_yaml_in_scope_record_counts() -> BTreeMap<String, usize> {
    let store = FilesystemTaxonomyStore {
        taxonomy_dir: workspace_taxonomy_dir(),
    };
    load_framework_yaml_in_scope_record_counts_from_store(&store)
}

pub fn load_framework_yaml_in_scope_record_counts_from_dir(
    taxonomy_dir: &Path,
) -> BTreeMap<String, usize> {
    let store = FilesystemTaxonomyStore {
        taxonomy_dir: taxonomy_dir.to_path_buf(),
    };
    load_framework_yaml_in_scope_record_counts_from_store(&store)
}

pub fn load_framework_yaml_record_counts_from_store(
    store: &dyn TaxonomyStore,
) -> BTreeMap<String, usize> {
    ORDERED_FRAMEWORKS
        .into_iter()
        .map(|framework| {
            (
                framework.to_string(),
                store.load_framework_records(framework, false).len(),
            )
        })
        .collect()
}

pub fn load_framework_yaml_in_scope_record_counts_from_store(
    store: &dyn TaxonomyStore,
) -> BTreeMap<String, usize> {
    ORDERED_FRAMEWORKS
        .into_iter()
        .map(|framework| {
            (
                framework.to_string(),
                store.load_framework_records(framework, true).len(),
            )
        })
        .collect()
}

/// Shared renderer/drift loading semantics. Strict callers retain actionable errors.
pub fn load_framework_yaml_records_checked(
    path: &Path,
    in_scope_only: bool,
) -> Result<Vec<FrameworkRecord>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let records: Vec<FrameworkRecord> = serde_yaml::from_str(&text)
        .map_err(|e| format!("{}: malformed catalog: {e}", path.display()))?;
    if records.is_empty() {
        return Err(format!("{}: empty catalog", path.display()));
    }
    let mut seen = std::collections::BTreeSet::new();
    for record in &records {
        if record.id.trim().is_empty() || record.id != record.id.trim() || !seen.insert(&record.id)
        {
            return Err(format!(
                "{}: empty, padded or duplicate id {:?}",
                path.display(),
                record.id
            ));
        }
    }
    Ok(records
        .into_iter()
        .filter(|r| !in_scope_only || !r.out_of_scope)
        .collect())
}
