use std::{fs, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let root = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let result = (|| {
        let settings = fs::read_to_string(root.join(".claude/settings.json"))
            .map_err(|e| format!("settings.json: {e}"))?;
        let docs = fs::read_to_string(root.join("docs/standards/CLAUDE_PERMISSIONS.md"))
            .map_err(|e| format!("CLAUDE_PERMISSIONS.md: {e}"))?;
        tachi_core::permissions::validate_permissions(&settings, &docs)
    })();
    match result {
        Ok(()) => {
            println!("permissions consistency: passed");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
