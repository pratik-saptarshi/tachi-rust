use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires pinned Gitleaks; explicitly run in the secret-scanning workflow"]
fn adopter_rules_retain_defaults_and_fail_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let temp = std::env::temp_dir().join(format!("tachi-adopter-scan-{}", std::process::id()));
    fs::create_dir_all(temp.join("application")).unwrap();
    let config = temp.join("adopter.toml");
    let base = fs::read_to_string(root.join(".gitleaks-adopter.toml.example")).unwrap();
    let parent = root.join(".gitleaks.toml").canonicalize().unwrap();
    fs::write(&config, base.replace("path = \".gitleaks.toml\"", &format!("path = {:?}", parent.display().to_string())) + "\n[[rules]]\nid = 'adopter-test'\ndescription = 'Synthetic organization token'\nregex = '''ORGTEST_[a-z0-9]{32}'''\n").unwrap();
    let scan = |config: &Path| {
        Command::new("gitleaks")
            .args([
                "dir",
                "--no-banner",
                "--redact",
                "--report-format",
                "json",
                "--report-path",
            ])
            .arg(temp.join("result.json"))
            .arg("--config")
            .arg(config)
            .arg(temp.join("application"))
            .output()
            .expect("gitleaks 8.30.1 is required for adopter validation")
    };
    let credential = format!("ghp_{}", "aB3dE7gH9jK2mN4pQ6sT8vW1xY5zC0rF2uL4");
    fs::write(
        temp.join("application/config.txt"),
        format!(
            "github_token = '{credential}'\norganization = 'ORGTEST_{}'",
            "a1".repeat(16)
        ),
    )
    .unwrap();
    assert_eq!(
        scan(&config).status.code(),
        Some(1),
        "synthetic secrets must fail"
    );
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(temp.join("result.json")).unwrap()).unwrap();
    for rule in ["github-pat", "adopter-test"] {
        assert!(
            report
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["RuleID"] == rule),
            "missing inherited/custom rule {rule}"
        );
    }
    fs::write(
        temp.join("application/config.txt"),
        "github_token = '<placeholder>'\nOPENAI_API_KEY=PLACEHOLDER\n",
    )
    .unwrap();
    assert!(
        scan(&config).status.success(),
        "documented placeholders must pass"
    );
    fs::write(&config, "[extend\nmalformed").unwrap();
    assert!(
        !scan(&config).status.success(),
        "malformed configuration must fail"
    );
    fs::remove_dir_all(temp).unwrap();
}
