use std::process::Command;

#[test]
fn desktop_binary_headless_renders_selected_repo_root() {
    let binary = std::env::var("CARGO_BIN_EXE_tachi-desktop")
        .expect("CARGO_BIN_EXE_tachi-desktop should be provided by cargo");
    let output = Command::new(binary)
        .args(["--headless", "--repo-root", "/tmp/tachi-desktop-e2e"])
        .output()
        .expect("run desktop binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Tachi Desktop"));
    assert!(stdout.contains("Repository root: /tmp/tachi-desktop-e2e"));
    assert!(stdout.contains("- bootstrap"));
}

#[test]
fn desktop_binary_headless_handles_default_and_missing_repo_root_arguments() {
    let binary = std::env::var("CARGO_BIN_EXE_tachi-desktop")
        .expect("CARGO_BIN_EXE_tachi-desktop should be provided by cargo");

    let default_root = Command::new(&binary)
        .arg("--headless")
        .output()
        .expect("run desktop binary with default root");
    assert!(default_root.status.success());
    assert!(String::from_utf8_lossy(&default_root.stdout).contains("Repository root:"));

    let missing_root = Command::new(binary)
        .args(["--headless", "--repo-root"])
        .output()
        .expect("run desktop binary with missing repo root");
    assert!(missing_root.status.success());
    assert!(String::from_utf8_lossy(&missing_root.stdout).contains("Repository root:"));
}

#[cfg(not(target_os = "macos"))]
#[test]
fn desktop_binary_uses_current_root_without_headless_flag() {
    let binary = std::env::var("CARGO_BIN_EXE_tachi-desktop")
        .expect("CARGO_BIN_EXE_tachi-desktop should be provided by cargo");
    let output = Command::new(binary).output().expect("run desktop binary");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Tachi Desktop"));
}
