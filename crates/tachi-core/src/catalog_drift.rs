//! Offline catalog contract and all-or-nothing baseline generation.
use crate::coverage_attestation::{
    load_framework_yaml_records_checked, FrameworkRecord, ORDERED_FRAMEWORKS,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub const MANIFEST: &str = "schemas/taxonomy/catalog-manifest.json";
pub const TYPST_VERSION: &str = "typst 0.15.1 (9dfd3a08)";
pub const SOURCE_DATE_EPOCH: &str = "1700000000";
pub const BASELINES: [&str; 6] = [
    "web-app",
    "microservices",
    "ascii-web-api",
    "mermaid-agentic-app",
    "free-text-microservice",
    "maestro-reference",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogFingerprint {
    pub framework: String,
    pub raw: Vec<FrameworkRecord>,
    pub in_scope: Vec<FrameworkRecord>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaselineHash {
    pub path: String,
    pub sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub catalogs: Vec<CatalogFingerprint>,
    pub renderer: String,
    pub source_date_epoch: String,
    pub font_policy: String,
    pub baselines: Vec<BaselineHash>,
}

pub fn fingerprints(root: &Path) -> Result<Vec<CatalogFingerprint>, String> {
    ORDERED_FRAMEWORKS
        .iter()
        .map(|framework| {
            let path = root.join(format!("schemas/taxonomy/{framework}.yaml"));
            Ok(CatalogFingerprint {
                framework: (*framework).into(),
                raw: load_framework_yaml_records_checked(&path, false)?,
                in_scope: load_framework_yaml_records_checked(&path, true)?,
            })
        })
        .collect()
}

pub fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn check(root: &Path) -> Result<(), String> {
    let path = root.join(MANIFEST);
    let manifest: Manifest = serde_json::from_slice(&fs::read(&path).map_err(|e| {
        format!(
            "{}: {e}; run catalog-drift --regenerate-baselines",
            path.display()
        )
    })?)
    .map_err(|e| format!("{}: malformed manifest: {e}", path.display()))?;
    let actual = fingerprints(root)?;
    if manifest.version != 1 || manifest.catalogs != actual {
        let changed = actual
            .iter()
            .enumerate()
            .find(|(i, f)| manifest.catalogs.get(*i) != Some(f))
            .map(|(_, f)| f.framework.as_str())
            .unwrap_or("framework registry");
        return Err(format!("catalog drift in {changed}: ordered raw/in-scope records differ; run catalog-drift --regenerate-baselines"));
    }
    if manifest.renderer != TYPST_VERSION
        || manifest.source_date_epoch != SOURCE_DATE_EPOCH
        || manifest.font_policy != "embedded-only"
    {
        return Err("stale renderer provenance; run catalog-drift --regenerate-baselines".into());
    }
    let expected_paths: Vec<_> = BASELINES.iter().map(|name| baseline_path(name)).collect();
    if manifest
        .baselines
        .iter()
        .map(|b| &b.path)
        .collect::<Vec<_>>()
        != expected_paths.iter().collect::<Vec<_>>()
    {
        return Err("manifest must contain all six baselines in canonical order".into());
    }
    for baseline in manifest.baselines {
        let bytes =
            fs::read(root.join(&baseline.path)).map_err(|e| format!("{}: {e}", baseline.path))?;
        if sha256(&bytes) != baseline.sha256 {
            return Err(format!(
                "{}: baseline hash mismatch; regenerate complete set",
                baseline.path
            ));
        }
    }
    Ok(())
}

fn baseline_path(name: &str) -> String {
    format!("examples/{name}/security-report.pdf.baseline")
}

struct Stage(PathBuf);
impl Drop for Stage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_tree(source: &Path, dest: &Path) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| format!("{}: {e}", source.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        let target = dest.join(entry.file_name());
        if kind.is_symlink() {
            return Err(format!(
                "baseline input symlink is unsupported: {}",
                entry.path().display()
            ));
        }
        if kind.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// All renders happen in a private tree. Originals are untouched on any render
/// failure. Publication rolls back completed writes on an ordinary I/O error.
pub fn regenerate(root: &Path, typst: &Path) -> Result<(), String> {
    let catalogs = fingerprints(root)?;
    let version = Command::new(typst)
        .arg("--version")
        .output()
        .map_err(|e| format!("Typst: {e}"))?;
    if !version.status.success() || String::from_utf8_lossy(&version.stdout).trim() != TYPST_VERSION
    {
        return Err(format!("Typst must be pinned to {TYPST_VERSION}"));
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let stage =
        Stage(std::env::temp_dir().join(format!("tachi-baselines-{}-{nonce}", std::process::id())));
    fs::create_dir(&stage.0).map_err(|e| e.to_string())?;
    let templates = stage.0.join("templates/tachi/security-report");
    copy_tree(&root.join("templates/tachi/security-report"), &templates)?;
    let brand = root.join("brand/final");
    if brand.exists() {
        copy_tree(&brand, &stage.0.join("brand/final"))?;
    }
    copy_tree(
        &root.join("schemas/taxonomy"),
        &stage.0.join("schemas/taxonomy"),
    )?;
    let mut writes = Vec::new();
    let mut hashes = Vec::new();
    for name in BASELINES {
        let relative = format!("examples/{name}");
        let target = stage.0.join(&relative);
        copy_tree(&root.join(&relative), &target)?;
        let threats = fs::read_to_string(target.join("threats.md"))
            .map_err(|e| format!("{name}/threats.md: {e}"))?;
        crate::parsers::parse_threats_findings(&threats)
            .map_err(|e| format!("{name}/threats.md: {e}"))?;
        let data = crate::build_report_data_typst(&target, &templates);
        fs::write(templates.join("report-data.typ"), data).map_err(|e| e.to_string())?;
        let pdf = target.join("security-report.pdf.baseline");
        // Remove the copied baseline so a successful no-op renderer cannot pass.
        if pdf.exists() {
            fs::remove_file(&pdf).map_err(|e| e.to_string())?;
        }
        let result = Command::new(typst)
            .arg("compile")
            .arg(templates.join("main.typ"))
            .arg(&pdf)
            .arg("--root")
            .arg(&stage.0)
            .arg("--format")
            .arg("pdf")
            .arg("--ignore-system-fonts")
            .env("SOURCE_DATE_EPOCH", SOURCE_DATE_EPOCH)
            .current_dir(&stage.0)
            .output()
            .map_err(|e| format!("{name}: {e}"))?;
        if !result.status.success() {
            return Err(format!(
                "{name}: Typst render failed: {}",
                String::from_utf8_lossy(&result.stderr)
            ));
        }
        let bytes = fs::read(&pdf).map_err(|e| format!("{name}: missing rendered PDF: {e}"))?;
        if !bytes.starts_with(b"%PDF-") || !bytes.windows(5).any(|w| w == b"%%EOF") {
            return Err(format!("{name}: renderer did not produce a complete PDF"));
        }
        let path = baseline_path(name);
        hashes.push(BaselineHash {
            path: path.clone(),
            sha256: sha256(&bytes),
        });
        writes.push((root.join(path), bytes));
    }
    let manifest = Manifest {
        version: 1,
        catalogs,
        renderer: TYPST_VERSION.into(),
        source_date_epoch: SOURCE_DATE_EPOCH.into(),
        font_policy: "embedded-only".into(),
        baselines: hashes,
    };
    writes.push((
        root.join(MANIFEST),
        serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?,
    ));
    publish(&writes)
}

fn publish(writes: &[(PathBuf, Vec<u8>)]) -> Result<(), String> {
    let backups: Vec<_> = writes
        .iter()
        .map(|(p, _)| {
            if p.exists() {
                fs::read(p)
                    .map(Some)
                    .map_err(|e| format!("{}: {e}", p.display()))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<_, _>>()?;
    for (index, (path, bytes)) in writes.iter().enumerate() {
        if let Err(error) = fs::write(path, bytes) {
            let mut rollback_errors = Vec::new();
            for ((path, _), backup) in writes[..=index].iter().zip(&backups) {
                let result = match backup {
                    Some(bytes) => fs::write(path, bytes),
                    None => fs::remove_file(path),
                };
                if let Err(e) = result {
                    rollback_errors.push(format!("{}: {e}", path.display()));
                }
            }
            return Err(format!(
                "baseline publication failed at {}: {error}; rollback errors: {rollback_errors:?}",
                path.display()
            ));
        }
    }
    Ok(())
}
