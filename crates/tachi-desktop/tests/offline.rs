use std::fs;
use std::os::unix::fs::symlink;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use tachi_desktop::{
    bootstrap_from_cache, bootstrap_from_cache_typed, check_for_update, check_for_update_typed,
    restore_offline_cache, restore_offline_cache_typed, BootstrapReport, DesktopErrorKind,
    OfflineRestoreReport, UpdateCheck,
};

fn fixture_root(prefix: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "tachi-rust-{prefix}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    fs::create_dir_all(&root).expect("create fixture root");
    root
}

fn write_executable_file(path: &PathBuf, content: &str) {
    fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
    fs::write(path, content).expect("write temporary file");
    let mut perms = fs::metadata(path).expect("read metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("set executable mode");
}

fn canonical(path: &PathBuf) -> PathBuf {
    fs::canonicalize(path).expect("canonicalize path")
}

#[test]
fn restore_offline_cache_restores_expected_files() {
    let repo_root = fixture_root("offline-repo");
    let cache_root = fixture_root("offline-cache");

    fs::create_dir_all(cache_root.join(".aod")).expect("create cache aod dir");
    write_executable_file(
        &cache_root.join("scripts/update.sh"),
        "#!/usr/bin/env bash\necho offline update\n",
    );
    fs::write(
        cache_root.join(".aod/aod-kit-version"),
        "version=v1.2.3\nsha=abc123\n",
    )
    .expect("write version pin");

    let report = restore_offline_cache(&repo_root, &cache_root).expect("restore cache");
    let canonical_repo_root = canonical(&repo_root);

    assert_eq!(
        report,
        OfflineRestoreReport {
            restored_files: vec![
                canonical_repo_root.join(".aod/aod-kit-version"),
                canonical_repo_root.join("scripts/update.sh"),
            ],
            missing_cache_files: vec![
                canonical_repo_root.join("scripts/install.sh"),
                canonical_repo_root.join("scripts/init.sh"),
            ],
        }
    );
    assert!(repo_root.join("scripts/update.sh").is_file());
    assert!(repo_root.join(".aod/aod-kit-version").is_file());
}

#[test]
fn check_for_update_reports_cached_version_difference() {
    let repo_root = fixture_root("offline-current");
    let cache_root = fixture_root("offline-updated");

    fs::create_dir_all(repo_root.join(".aod")).expect("create repo aod dir");
    fs::create_dir_all(cache_root.join(".aod")).expect("create cache aod dir");
    fs::write(repo_root.join(".aod/aod-kit-version"), "version=v1.0.0\n").expect("write current");
    fs::write(cache_root.join(".aod/aod-kit-version"), "version=v1.1.0\n").expect("write cached");

    let check = check_for_update(&repo_root, &cache_root).expect("check update");

    assert_eq!(
        check,
        UpdateCheck {
            current_version: Some(String::from("v1.0.0")),
            cached_version: Some(String::from("v1.1.0")),
            update_available: true,
        }
    );
}

#[test]
fn bootstrap_from_cache_restores_ready_offline_state() {
    let repo_root = fixture_root("offline-bootstrap-repo");
    let cache_root = fixture_root("offline-bootstrap-cache");

    fs::create_dir_all(cache_root.join(".aod")).expect("create cache aod dir");
    write_executable_file(
        &cache_root.join("scripts/update.sh"),
        "#!/usr/bin/env bash\necho bootstrap\n",
    );
    fs::write(
        cache_root.join(".aod/aod-kit-version"),
        "version=v2.0.0\nsha=abc123\n",
    )
    .expect("write cached version");

    let report = bootstrap_from_cache(&repo_root, &cache_root).expect("bootstrap from cache");
    let canonical_repo_root = canonical(&repo_root);

    assert_eq!(
        report,
        BootstrapReport {
            restore: OfflineRestoreReport {
                restored_files: vec![
                    canonical_repo_root.join(".aod/aod-kit-version"),
                    canonical_repo_root.join("scripts/update.sh"),
                ],
                missing_cache_files: vec![
                    canonical_repo_root.join("scripts/install.sh"),
                    canonical_repo_root.join("scripts/init.sh"),
                ],
            },
            update_check: UpdateCheck {
                current_version: Some(String::from("v2.0.0")),
                cached_version: Some(String::from("v2.0.0")),
                update_available: false,
            },
            offline_ready: true,
        }
    );
}

#[test]
fn restore_offline_cache_rejects_parent_traversal_roots() {
    let repo_root = fixture_root("offline-repo-traversal");
    let cache_root = fixture_root("offline-cache-traversal");

    let err = restore_offline_cache(&repo_root.join(".."), &cache_root)
        .expect_err("reject repo root traversal");
    assert!(err.contains("contains parent traversal"));
}

#[test]
fn restore_offline_cache_rejects_symlinked_cache_files() {
    let repo_root = fixture_root("offline-repo-symlink");
    let cache_root = fixture_root("offline-cache-symlink");
    let outside = fixture_root("offline-cache-outside");

    fs::create_dir_all(outside.join(".aod")).expect("create outside aod dir");
    fs::write(outside.join(".aod/aod-kit-version"), "version=v9.9.9\n").expect("write outside pin");
    fs::create_dir_all(cache_root.join(".aod")).expect("create cache aod dir");
    symlink(
        outside.join(".aod/aod-kit-version"),
        cache_root.join(".aod/aod-kit-version"),
    )
    .expect("create symlinked cache pin");

    let err = restore_offline_cache(&repo_root, &cache_root).expect_err("reject symlink escape");
    assert!(err.contains("path policy failed for offline cache file"));
}

#[test]
fn check_for_update_handles_missing_and_fallback_version_pins() {
    let repo_root = fixture_root("offline-version-fallback-repo");
    let cache_root = fixture_root("offline-version-fallback-cache");
    fs::create_dir_all(repo_root.join(".aod")).expect("create repo aod dir");
    fs::create_dir_all(cache_root.join(".aod")).expect("create cache aod dir");

    fs::write(
        repo_root.join(".aod/aod-kit-version"),
        "  local-fallback  \n",
    )
    .expect("write fallback current pin");
    let missing_cache = check_for_update(&repo_root, &cache_root).expect("missing cache pin");
    assert_eq!(
        missing_cache.current_version,
        Some(String::from("local-fallback"))
    );
    assert_eq!(missing_cache.cached_version, None);
    assert!(!missing_cache.update_available);

    fs::write(
        cache_root.join(".aod/aod-kit-version"),
        "comment-before-version\nversion=cached-v2\n",
    )
    .expect("write cached version pin");
    let cached = check_for_update(&repo_root, &cache_root).expect("cached version pin");
    assert_eq!(cached.cached_version, Some(String::from("cached-v2")));
    assert!(cached.update_available);
}

#[test]
fn bootstrap_from_cache_reports_not_ready_when_update_script_is_missing() {
    let repo_root = fixture_root("offline-not-ready-repo");
    let cache_root = fixture_root("offline-not-ready-cache");
    fs::create_dir_all(cache_root.join(".aod")).expect("create cache aod dir");
    fs::write(cache_root.join(".aod/aod-kit-version"), "version=v3.0.0\n")
        .expect("write cached version");

    let report = bootstrap_from_cache(&repo_root, &cache_root).expect("bootstrap report");
    assert!(!report.offline_ready);
    assert_eq!(report.restore.restored_files.len(), 1);
}

#[test]
fn restore_offline_cache_rejects_symlinked_restore_directory() {
    let repo_root = fixture_root("offline-restore-symlink-repo");
    let cache_root = fixture_root("offline-restore-symlink-cache");
    let outside = fixture_root("offline-restore-symlink-outside");
    fs::create_dir_all(cache_root.join("scripts")).expect("create cache scripts");
    fs::write(cache_root.join("scripts/update.sh"), "#!/bin/bash\n").expect("write cache script");
    symlink(&outside, repo_root.join("scripts")).expect("create destination symlink");

    let err = restore_offline_cache(&repo_root, &cache_root)
        .expect_err("reject symlinked restore destination");
    assert!(err.contains("traverses symlink"));
}

#[test]
fn typed_offline_errors_preserve_policy_io_and_internal_classification() {
    let repo_root = fixture_root("offline-typed-repo");
    let cache_root = fixture_root("offline-typed-cache");

    let policy = restore_offline_cache_typed(&repo_root.join(".."), &cache_root)
        .expect_err("parent traversal should be policy error");
    assert_eq!(policy.kind(), DesktopErrorKind::Policy);
    assert_eq!(policy.code(), 3);

    let missing_root = restore_offline_cache_typed(
        &repo_root.join("missing-repo"),
        &cache_root.join("missing-cache"),
    )
    .expect_err("missing roots should classify as path policy");
    assert_eq!(missing_root.kind(), DesktopErrorKind::Policy);
    assert_eq!(missing_root.code(), 3);

    fs::create_dir_all(repo_root.join(".aod/aod-kit-version"))
        .expect("create invalid version directory");
    let io = check_for_update_typed(&repo_root, &cache_root)
        .expect_err("directory version pin should classify as io");
    assert_eq!(io.kind(), DesktopErrorKind::Io);
    assert_eq!(io.code(), 4);

    let bootstrap = bootstrap_from_cache_typed(&repo_root, &cache_root)
        .expect_err("bootstrap should preserve io classification");
    assert_eq!(bootstrap.kind(), DesktopErrorKind::Io);
}
