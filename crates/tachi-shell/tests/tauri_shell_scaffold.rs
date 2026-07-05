use std::path::PathBuf;

#[test]
fn desktop_shell_scaffold_is_active_and_tauri_adapter_is_retired() {
    let repo_root = repo_root();
    assert!(repo_root.join("crates/tachi-desktop/Cargo.toml").is_file());
    assert!(repo_root.join("crates/tachi-desktop/src/lib.rs").is_file());
    assert!(!repo_root.join("src-tauri/Cargo.toml").exists());
    assert!(!repo_root.join("src-tauri/Cargo.lock").exists());
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("repo root")
        .to_path_buf()
}
