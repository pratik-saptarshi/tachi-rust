use tachi_core::permissions::validate_permissions;

const SETTINGS: &str =
    r#"{"permissions":{"allow":["Read"],"ask":[],"deny":["Bash(curl * | sh)"]}}"#;
const DOCS: &str = "## 4. Rules\n| `Read` | rationale |\n| `Bash(curl * \\| sh)` | denied |\n## 5. Built in\n| `Ignored` | example |\n";

#[test]
fn valid_and_negative_permission_contracts() {
    assert_eq!(validate_permissions(SETTINGS, DOCS), Ok(()));
    for (settings, docs, diagnostic) in [
        ("{", DOCS, "malformed JSON"),
        ("{}", DOCS, "permissions object"),
        (
            r#"{"permissions":{"allow":[],"deny":[]}}"#,
            DOCS,
            "permissions.ask",
        ),
        (SETTINGS, "## 5. Built in", "section ## 4."),
        (SETTINGS, "## 4. Rules", "section ## 5."),
        (
            SETTINGS,
            "## 4. Rules\n## 5. Built in",
            "no permission rules",
        ),
    ] {
        assert!(validate_permissions(settings, docs)
            .unwrap_err()
            .contains(diagnostic));
    }
    let changed = SETTINGS.replace("Read", "Write");
    let error = validate_permissions(&changed, DOCS).unwrap_err();
    assert!(error.contains("undocumented rules [\"Write\"]"));
    assert!(error.contains("orphaned section 4 rules [\"Read\"]"));
}

#[test]
fn repository_and_hosted_workflow_agree() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    validate_permissions(
        &std::fs::read_to_string(root.join(".claude/settings.json")).unwrap(),
        &std::fs::read_to_string(root.join("docs/standards/CLAUDE_PERMISSIONS.md")).unwrap(),
    )
    .unwrap();
    let workflow: serde_yaml::Value = serde_yaml::from_str(
        &std::fs::read_to_string(root.join(".github/workflows/permissions-consistency.yml"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(workflow["permissions"]["contents"].as_str(), Some("read"));
    assert_eq!(workflow["permissions"].as_mapping().unwrap().len(), 1);
    for event in ["push", "pull_request"] {
        assert_eq!(workflow["on"][event]["branches"][0].as_str(), Some("main"));
        let paths = workflow["on"][event]["paths"].as_sequence().unwrap();
        for required in [
            ".claude/settings.json",
            "docs/standards/CLAUDE_PERMISSIONS.md",
            "crates/tachi-core/src/permissions.rs",
            ".github/workflows/permissions-consistency.yml",
        ] {
            assert!(
                paths.iter().any(|p| p.as_str() == Some(required)),
                "{event}: missing {required}"
            );
        }
    }
}
