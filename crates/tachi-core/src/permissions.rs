//! Fail-closed JSON and documentation checks for the existing AC-2 contract.
use std::collections::BTreeSet;

pub fn validate_permissions(settings: &str, docs: &str) -> Result<(), String> {
    let value: serde_json::Value = serde_json::from_str(settings)
        .map_err(|e| format!("settings.json: malformed JSON: {e}"))?;
    let permissions = value
        .get("permissions")
        .and_then(|v| v.as_object())
        .ok_or("settings.json: missing permissions object")?;
    let mut rules = BTreeSet::new();
    for section in ["allow", "ask", "deny"] {
        let entries = permissions
            .get(section)
            .and_then(|v| v.as_array())
            .ok_or_else(|| format!("settings.json: missing permissions.{section} array"))?;
        for entry in entries {
            let rule = entry
                .as_str()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| {
                    format!("settings.json: permissions.{section} must contain nonempty strings")
                })?;
            rules.insert(rule.to_owned());
        }
    }
    let start = docs
        .find("\n## 4.")
        .or_else(|| docs.starts_with("## 4.").then_some(0))
        .ok_or("CLAUDE_PERMISSIONS.md: missing required section ## 4.")?;
    let section = &docs[start..];
    let end = section
        .find("\n## 5.")
        .ok_or("CLAUDE_PERMISSIONS.md: missing required section ## 5.")?;
    let documented: BTreeSet<String> = section[..end]
        .lines()
        .filter_map(|line| {
            let row = line.trim().strip_prefix("| `")?;
            Some(row.split_once('`')?.0.replace("\\|", "|"))
        })
        .collect();
    if documented.is_empty() {
        return Err("CLAUDE_PERMISSIONS.md: section 4 has no permission rules".into());
    }
    let missing: Vec<_> = rules.difference(&documented).collect();
    let orphaned: Vec<_> = documented.difference(&rules).collect();
    if !missing.is_empty() || !orphaned.is_empty() {
        return Err(format!("permissions drift: undocumented rules {missing:?}; orphaned section 4 rules {orphaned:?}; update settings.json or CLAUDE_PERMISSIONS.md section 4"));
    }
    Ok(())
}
