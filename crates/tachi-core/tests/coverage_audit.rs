use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use tachi_core::{collect_audit, render};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}"))
}

fn write_file(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, "").expect("write test file");
}

#[test]
fn collect_audit_classifies_active_and_fixture_copies() {
    let root = unique_temp_dir("tachi-coverage-audit");

    write_file(&root.join("tests/scripts/test_smoke.py"));
    write_file(&root.join("tests/scripts/test_example_unit.py"));
    write_file(&root.join("tests/scripts/test_example_integration.py"));
    write_file(&root.join("tests/fixtures/init-baseline-tree/tests/scripts/test_fixture_unit.py"));

    let audit = collect_audit(&root);

    assert_eq!(audit.active.len(), 3);
    assert_eq!(audit.fixture_copies.len(), 1);
    assert_eq!(audit.smoke.len(), 1);
    assert_eq!(audit.unit.len(), 1);
    assert_eq!(audit.integration.len(), 1);
    assert_eq!(audit.e2e.len(), 0);
    assert_eq!(audit.support.len(), 0);

    let rendered = render(&audit, &root);
    assert!(rendered.contains("Active test modules: 3"));
    assert!(rendered.contains("Fixture-copy modules (excluded from active suite): 1"));
    assert!(rendered.contains("Smoke: 1"));
    assert!(rendered.contains("Unit: 1"));
    assert!(rendered.contains("Integration: 1"));
}

#[test]
fn collect_audit_includes_rust_test_modules_as_active_integration_coverage() {
    let root = unique_temp_dir("tachi-coverage-audit-rust");

    write_file(&root.join("tests/scripts/test_smoke.py"));
    write_file(&root.join("crates/tachi-core/tests/assets.rs"));
    write_file(&root.join("crates/tachi-shell/tests/infographic_data.rs"));
    write_file(&root.join("tests/fixtures/init-baseline-tree/tests/scripts/test_fixture_unit.py"));

    let audit = collect_audit(&root);

    assert_eq!(audit.active.len(), 3);
    assert_eq!(audit.fixture_copies.len(), 1);
    assert_eq!(audit.smoke.len(), 1);
    assert_eq!(audit.integration.len(), 2);
    assert_eq!(audit.unit.len(), 0);
    assert_eq!(audit.e2e.len(), 0);
    assert_eq!(audit.support.len(), 0);

    let rendered = render(&audit, &root);
    assert!(rendered.contains("Active test modules: 3"));
    assert!(rendered.contains("Integration: 2"));
    assert!(rendered.contains("Smoke: 1"));
}

#[test]
fn collect_audit_classifies_rust_smoke_canary_as_smoke_coverage() {
    let root = unique_temp_dir("tachi-coverage-audit-rust-smoke");

    write_file(&root.join("crates/tachi-core/tests/coverage_attestation_pagination.rs"));
    write_file(&root.join("crates/tachi-shell/tests/infographic_data.rs"));
    write_file(&root.join("tests/fixtures/init-baseline-tree/tests/scripts/test_fixture_unit.py"));

    let audit = collect_audit(&root);

    assert_eq!(audit.active.len(), 2);
    assert_eq!(audit.fixture_copies.len(), 1);
    assert_eq!(audit.smoke.len(), 1);
    assert_eq!(audit.integration.len(), 1);
    assert_eq!(audit.unit.len(), 0);
    assert_eq!(audit.e2e.len(), 0);
    assert_eq!(audit.support.len(), 0);

    let rendered = render(&audit, &root);
    assert!(rendered.contains("Smoke: 1"));
    assert!(rendered.contains("Integration: 1"));
}

#[test]
fn collect_audit_classifies_inline_source_tests_as_unit_coverage() {
    let root = unique_temp_dir("tachi-coverage-audit-unit");

    write_file(&root.join("crates/tachi-core/src/report_data.rs"));
    write_file(&root.join("crates/tachi-shell/tests/infographic_data.rs"));
    write_file(&root.join("tests/fixtures/init-baseline-tree/tests/scripts/test_fixture_unit.py"));

    fs::write(
        root.join("crates/tachi-core/src/report_data.rs"),
        "#[cfg(test)]\nmod tests {}\n",
    )
    .expect("write inline unit source");

    let audit = collect_audit(&root);

    assert_eq!(audit.active.len(), 2);
    assert_eq!(audit.fixture_copies.len(), 1);
    assert_eq!(audit.unit.len(), 1);
    assert_eq!(audit.integration.len(), 1);
    assert_eq!(audit.smoke.len(), 0);
    assert_eq!(audit.e2e.len(), 0);
    assert_eq!(audit.support.len(), 0);

    let rendered = render(&audit, &root);
    assert!(rendered.contains("Unit: 1"));
    assert!(rendered.contains("Integration: 1"));
}

#[test]
fn live_e2e_inventory_has_explicit_init_and_cli_artifact_boundaries() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root should be two levels above tachi-core manifest")
        .to_path_buf();

    let audit = collect_audit(&root);
    assert_eq!(
        audit.e2e,
        vec![
            PathBuf::from("crates/tachi-cli/tests/e2e_artifacts.rs"),
            PathBuf::from("crates/tachi-desktop/tests/e2e_command_journey.rs"),
            PathBuf::from("crates/tachi-mcp/tests/e2e_stdio_journey.rs"),
            PathBuf::from("crates/tachi-shell/tests/init_substitution.rs"),
        ]
    );
    assert!(
        audit.integration.iter().all(|path| {
            path != &PathBuf::from("crates/tachi-cli/tests/e2e_artifacts.rs")
                && path != &PathBuf::from("crates/tachi-desktop/tests/e2e_command_journey.rs")
                && path != &PathBuf::from("crates/tachi-mcp/tests/e2e_stdio_journey.rs")
                && path != &PathBuf::from("crates/tachi-shell/tests/init_substitution.rs")
        }),
        "the explicit E2E boundary must not be double-counted as integration"
    );

    let rendered = render(&audit, &root);
    assert!(rendered.contains("True end-to-end: 4"));
    assert!(rendered.contains("  - crates/tachi-cli/tests/e2e_artifacts.rs"));
    assert!(rendered.contains("  - crates/tachi-desktop/tests/e2e_command_journey.rs"));
    assert!(rendered.contains("  - crates/tachi-mcp/tests/e2e_stdio_journey.rs"));
    assert!(rendered.contains("  - crates/tachi-shell/tests/init_substitution.rs"));
}

#[test]
fn collect_audit_handles_missing_roots_and_all_python_categories() {
    let missing = unique_temp_dir("tachi-coverage-audit-missing");
    assert!(!missing.exists());
    let empty_audit = collect_audit(&missing);
    assert_eq!(empty_audit, Default::default());
    assert!(render(&empty_audit, &missing).contains("Active test modules: 0"));

    let root = unique_temp_dir("tachi-coverage-audit-categories");
    write_file(&root.join("tests/scripts/test_example_e2e.py"));
    write_file(&root.join("tests/scripts/test_example_smoke.py"));
    write_file(&root.join("tests/scripts/test_example_unit.py"));
    write_file(&root.join("tests/scripts/test_example_integration.py"));
    write_file(&root.join("tests/scripts/test_helper.py"));
    write_file(&root.join("crates/tachi-core/tests/support.rs"));
    let inline_source = root.join("crates/tachi-core/src/inline.rs");
    fs::create_dir_all(inline_source.parent().expect("inline source parent"))
        .expect("create inline source parent");
    fs::write(&inline_source, "#[cfg(test)]\nmod tests {}\n").expect("write inline test module");

    let audit = collect_audit(&root);
    assert_eq!(audit.e2e.len(), 1);
    assert_eq!(audit.smoke.len(), 1);
    assert_eq!(audit.unit.len(), 2);
    assert_eq!(audit.integration.len(), 2);
    assert_eq!(audit.support.len(), 1);
    let rendered = render(&audit, &root);
    assert!(rendered.contains("E2E: 1") || rendered.contains("True end-to-end: 1"));
    assert!(rendered.contains("Support / regression: 1"));
}
