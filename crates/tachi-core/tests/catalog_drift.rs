use std::{fs, path::PathBuf};
use tachi_core::{
    catalog_drift::{
        self, BaselineHash, Manifest, BASELINES, MANIFEST, SOURCE_DATE_EPOCH, TYPST_VERSION,
    },
    coverage_attestation::ORDERED_FRAMEWORKS,
};

struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn fixture() -> Fixture {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let root = Fixture(std::env::temp_dir().join(format!(
        "tachi-catalog-{}-{nonce}-{serial}",
        std::process::id()
    )));
    fs::create_dir_all(root.0.join("schemas/taxonomy")).unwrap();
    for framework in ORDERED_FRAMEWORKS {
        fs::write(root.0.join(format!("schemas/taxonomy/{framework}.yaml")), "- id: FIRST\n  out_of_scope: false\n  url: https://example.test/one\n- id: SECOND\n  out_of_scope: true\n").unwrap();
    }
    let mut hashes = Vec::new();
    for name in BASELINES {
        let dir = root.0.join(format!("examples/{name}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("threats.md"), "# Fixture\n").unwrap();
        let path = format!("examples/{name}/security-report.pdf.baseline");
        fs::write(root.0.join(&path), b"%PDF-1.7 original %%EOF").unwrap();
        hashes.push(BaselineHash {
            path,
            sha256: catalog_drift::sha256(b"%PDF-1.7 original %%EOF"),
        });
    }
    let manifest = Manifest {
        version: 1,
        catalogs: catalog_drift::fingerprints(&root.0).unwrap(),
        renderer: TYPST_VERSION.into(),
        source_date_epoch: SOURCE_DATE_EPOCH.into(),
        font_policy: "embedded-only".into(),
        baselines: hashes,
    };
    fs::write(
        root.0.join(MANIFEST),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    root
}

#[test]
fn detects_membership_order_scope_registry_and_manifest_drift_but_not_citations() {
    let root = fixture();
    catalog_drift::check(&root.0).unwrap();
    let path = root.0.join("schemas/taxonomy/owasp.yaml");
    let original = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        original.replace(
            "https://example.test/one",
            "https://example.test/new-citation",
        ),
    )
    .unwrap();
    catalog_drift::check(&root.0).unwrap();
    for mutation in [
        original.replace("FIRST", "RENAMED"),
        original.replace("out_of_scope: true", "out_of_scope: false"),
        format!("{original}- id: THIRD\n"),
        "- id: SECOND\n  out_of_scope: true\n- id: FIRST\n".into(),
        "- id: FIRST\n".into(),
    ] {
        fs::write(&path, mutation).unwrap();
        assert!(catalog_drift::check(&root.0).unwrap_err().contains("owasp"));
    }
    fs::write(&path, &original).unwrap();
    let manifest_path = root.0.join(MANIFEST);
    let original_manifest = fs::read(&manifest_path).unwrap();
    let mut manifest: Manifest = serde_json::from_slice(&original_manifest).unwrap();
    manifest.catalogs.swap(0, 1);
    fs::write(&manifest_path, serde_json::to_vec(&manifest).unwrap()).unwrap();
    assert!(catalog_drift::check(&root.0).is_err());
    fs::write(&manifest_path, b"{").unwrap();
    assert!(catalog_drift::check(&root.0)
        .unwrap_err()
        .contains("malformed manifest"));
    fs::remove_file(&manifest_path).unwrap();
    assert!(catalog_drift::check(&root.0).is_err());
}

#[test]
fn malformed_missing_empty_and_duplicate_catalogs_fail_explicitly() {
    let root = fixture();
    let path = root.0.join("schemas/taxonomy/owasp.yaml");
    for invalid in [
        "[",
        "{}",
        "[]",
        "- name: missing-id\n",
        "- id: X\n  out_of_scope: perhaps\n",
        "- id: X\n- id: X\n",
    ] {
        fs::write(&path, invalid).unwrap();
        assert!(
            catalog_drift::fingerprints(&root.0)
                .unwrap_err()
                .contains("owasp.yaml"),
            "{invalid}"
        );
    }
    fs::remove_file(path).unwrap();
    assert!(catalog_drift::fingerprints(&root.0)
        .unwrap_err()
        .contains("owasp.yaml"));
}

#[cfg(unix)]
#[test]
fn third_render_failure_preserves_all_six_baselines_and_manifest() {
    use std::os::unix::fs::PermissionsExt;
    let root = fixture();
    let templates = root.0.join("templates/tachi/security-report");
    fs::create_dir_all(&templates).unwrap();
    fs::write(templates.join("main.typ"), "fixture").unwrap();
    let renderer = root.0.join("fake-typst");
    fs::write(&renderer, format!("#!/bin/sh\nif [ \"$1\" = --version ]; then echo '{TYPST_VERSION}'; exit 0; fi\ncase \"$3\" in *ascii-web-api*) echo 'forced third render failure' >&2; exit 1;; esac\nprintf '%%PDF-1.7\\n%%%%EOF\\n' > \"$3\"\n")).unwrap();
    fs::set_permissions(&renderer, fs::Permissions::from_mode(0o700)).unwrap();
    let before = fs::read(root.0.join(MANIFEST)).unwrap();
    let error = catalog_drift::regenerate(&root.0, &renderer).unwrap_err();
    assert!(
        error.contains("ascii-web-api") && error.contains("forced third render failure"),
        "{error}"
    );
    assert_eq!(fs::read(root.0.join(MANIFEST)).unwrap(), before);
    catalog_drift::check(&root.0).unwrap();
}
