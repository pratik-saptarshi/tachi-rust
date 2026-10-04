use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn fixture_root(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "taxonomy-link-monitor-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    fs::create_dir_all(&path).expect("create fixture root");
    path
}

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_taxonomy-link-monitor"))
}

#[test]
fn writes_an_empty_report_without_network_access_when_there_are_no_links() {
    let root = fixture_root("empty");
    fs::create_dir_all(root.join("schemas/taxonomy")).expect("create taxonomy directory");
    fs::write(
        root.join("schemas/taxonomy/index.yaml"),
        "title: No URL here\n",
    )
    .expect("write taxonomy fixture");
    let report = root.join("taxonomy-link-report.json");

    let output = binary()
        .current_dir(&root)
        .output()
        .expect("run taxonomy link monitor");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Checked 0 URLs"));
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(&report).expect("read report")).expect("valid JSON");
    assert_eq!(value["results"], serde_json::json!([]));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn reports_usage_and_io_errors_with_a_nonzero_exit_code() {
    let unknown = binary()
        .arg("--unknown")
        .output()
        .expect("run invalid command");
    assert_eq!(unknown.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("unknown argument"));

    let root = fixture_root("errors");
    let missing_taxonomy = binary()
        .args(["--root", root.to_str().unwrap()])
        .output()
        .expect("run with missing taxonomy directory");
    assert_eq!(missing_taxonomy.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&missing_taxonomy.stderr).contains("schemas/taxonomy"));

    fs::create_dir_all(root.join("schemas/taxonomy")).expect("create taxonomy directory");
    let report = root.join("missing-parent/report.json");
    let report_write_error = binary()
        .args([
            "--root",
            root.to_str().unwrap(),
            "--report",
            report.to_str().unwrap(),
        ])
        .output()
        .expect("run with an invalid report path");
    assert_eq!(report_write_error.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&report_write_error.stderr).contains("write"));
    let _ = fs::remove_dir_all(root);
}
