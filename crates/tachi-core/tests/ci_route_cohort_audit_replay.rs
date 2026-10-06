use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn retained_route_cohort_replays_with_the_recorded_classifier_revision() {
    let root = repo_root();
    let output = Command::new("bash")
        .arg(root.join("scripts/replay-rt-ci-route-cohort-audit.sh"))
        .current_dir(&root)
        .output()
        .expect("run route cohort replay");

    assert!(
        output.status.success(),
        "route cohort replay failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let replay: Value = serde_json::from_slice(&output.stdout).expect("replay emits JSON summary");
    assert_eq!(replay["status"], "passed");
    assert_eq!(replay["candidates_replayed"], 37);
    assert_eq!(
        replay["classifier_commit"],
        "10339cc8f586fdf0050c01bd2d4889f7709906e6"
    );
    assert_eq!(
        replay["classifier_blob_sha"],
        "247aa0721637272ff9b6df3088b058e2301ac4de"
    );
    assert_eq!(replay["classifier_policy_version"], "2026-10-05");
    assert_eq!(replay["route_counts"]["full_pr_matrix"], 37);
    assert_eq!(replay["route_counts"]["passive_docs_only"], 0);
    assert_eq!(replay["route_counts"]["dependency_closure"], 0);
    assert_eq!(
        replay["scope"],
        "retained historical candidates only; not timing evidence"
    );
}

#[test]
fn replay_rejects_tampered_candidate_inputs_and_provenance() {
    let root = repo_root();
    let manifest_path =
        root.join("docs/reports/rt-ci-route-cohort-audit-replay-manifest-2026-10-05.json");
    let mut manifest: Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("read route audit manifest"))
            .expect("parse route audit manifest");
    let temp_dir = std::env::temp_dir().join(format!(
        "tachi-ci-route-audit-tamper-{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("create replay tamper fixture directory");
    let tampered_inputs: [(&str, fn(&mut Value)); 4] = [
        ("route reason", |value: &mut Value| {
            value["candidates"][0]["route_reason"] = Value::String("forged audit reason".into());
        }),
        ("changed paths", |value: &mut Value| {
            value["candidates"][0]["changed_paths"] = serde_json::json!(["README.md"]);
        }),
        ("head identity", |value: &mut Value| {
            value["candidates"][0]["head_sha"] = Value::String("0".repeat(40));
        }),
        ("classifier provenance", |value: &mut Value| {
            value["source_commit"] = Value::String("0".repeat(40));
        }),
    ];

    for (label, tamper) in tampered_inputs {
        let mut candidate = manifest.clone();
        tamper(&mut candidate);
        let tampered_manifest = temp_dir.join(format!("{label}.json"));
        fs::write(
            &tampered_manifest,
            serde_json::to_vec(&candidate).expect("serialize tampered manifest"),
        )
        .expect("write tampered manifest");

        let output = Command::new("bash")
            .arg(root.join("scripts/replay-rt-ci-route-cohort-audit.sh"))
            .arg(&tampered_manifest)
            .current_dir(&root)
            .output()
            .expect("run replay against tampered historical evidence");

        assert!(
            !output.status.success(),
            "replay must reject tampered {label}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("manifest does not match the reviewed snapshot digest"),
            "failure should identify the reviewed-manifest digest mismatch for {label}: {output:?}"
        );
    }
    let _ = fs::remove_dir_all(&temp_dir);
}
