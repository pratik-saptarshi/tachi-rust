use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("taxonomy-link-monitor: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let (root, report_path) = parse_args()?;
    let urls = collect_urls(&root.join("schemas/taxonomy"))?;
    let mut results = Vec::new();
    let mut hosts = BTreeMap::<String, Instant>::new();
    for url in urls {
        let host = host_of(&url);
        if let Some(previous) = hosts.get(&host) {
            let elapsed = previous.elapsed();
            if elapsed < Duration::from_millis(250) {
                thread::sleep(Duration::from_millis(250) - elapsed);
            }
        }
        let result = check_url(&url);
        hosts.insert(host, Instant::now());
        results.push(result);
    }
    let report = json!({ "checked_at": now_utc(), "results": results });
    let serialized = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    fs::write(&report_path, serialized)
        .map_err(|e| format!("write {}: {e}", report_path.display()))?;
    let summary = report_summary(&report["results"]);
    println!("## Taxonomy citation link monitor\n\nChecked {} URLs: {} healthy, {} needs review, {} broken, {} transient.\n\nHTTP link results are informational and do not gate CI.", summary.0, summary.1, summary.2, summary.3, summary.4);
    Ok(())
}

fn parse_args() -> Result<(PathBuf, PathBuf), String> {
    let mut root = std::env::current_dir().map_err(|e| e.to_string())?;
    let mut report = PathBuf::from("taxonomy-link-report.json");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = PathBuf::from(args.next().ok_or("--root requires a path")?),
            "--report" => report = PathBuf::from(args.next().ok_or("--report requires a path")?),
            "--help" | "-h" => {
                return Err("usage: taxonomy-link-monitor [--root PATH] [--report PATH]".into())
            }
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    Ok((root, report))
}

fn collect_urls(dir: &Path) -> Result<BTreeSet<String>, String> {
    let mut urls = BTreeSet::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("read {}: {e}", dir.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }
        let content =
            fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        for line in content.lines() {
            let value = line.trim();
            if value.starts_with("url:")
                || value.starts_with("citation:")
                || value.starts_with("# citation:")
            {
                urls.extend(extract_urls(value));
            }
        }
    }
    Ok(urls)
}

fn extract_urls(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter_map(|word| {
            let start = word.find("https://").or_else(|| word.find("http://"))?;
            let url = word[start..].trim_end_matches(|c: char| {
                matches!(c, ')' | ']' | '}' | ',' | ';' | '.' | '"' | '\'')
            });
            (!url.is_empty()).then(|| url.to_string())
        })
        .collect()
}

fn check_url(url: &str) -> serde_json::Value {
    let head = curl(url, true);
    let (code, error) = if head.0 == 405 || head.0 == 501 || head.1.is_some() {
        curl(url, false)
    } else {
        head
    };
    json!({"url": url, "http_status": code, "status": classify(code, error.is_some()), "error": error})
}

fn curl(url: &str, head: bool) -> (u16, Option<String>) {
    let mut command = Command::new("curl");
    command.args([
        "--location",
        "--silent",
        "--show-error",
        "--max-time",
        "20",
        "--output",
        "/dev/null",
        "--write-out",
        "%{http_code}",
    ]);
    if head {
        command.arg("--head");
    } else {
        command.args(["--range", "0-0"]);
    }
    let output = match command.arg(url).output() {
        Ok(output) => output,
        Err(error) => return (0, Some(error.to_string())),
    };
    let code = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .unwrap_or(0);
    let error = (!output.status.success())
        .then(|| String::from_utf8_lossy(&output.stderr).trim().to_string());
    (code, error)
}

fn classify(code: u16, failed: bool) -> &'static str {
    match code {
        200..=399 => "healthy",
        401 | 403 | 429 => "needs_review",
        400..=499 => "broken",
        _ if failed || code == 0 || code >= 500 => "transient",
        _ => "transient",
    }
}

fn host_of(url: &str) -> String {
    url.split_once("://")
        .map(|(_, rest)| rest.split('/').next().unwrap_or(rest))
        .unwrap_or(url)
        .to_ascii_lowercase()
}

fn report_summary(results: &serde_json::Value) -> (usize, usize, usize, usize, usize) {
    let values = results.as_array().map(Vec::as_slice).unwrap_or(&[]);
    let mut counts = [0; 5];
    counts[0] = values.len();
    for value in values {
        let index = match value["status"].as_str().unwrap_or("") {
            "healthy" => 1,
            "needs_review" => 2,
            "broken" => 3,
            _ => 4,
        };
        counts[index] += 1;
    }
    (counts[0], counts[1], counts[2], counts[3], counts[4])
}

fn now_utc() -> String {
    Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_and_trims_url_candidates() {
        let got = extract_urls(
            "# citation: source [https://example.org/ref). url: https://example.org/taxonomy",
        );
        assert_eq!(
            got,
            ["https://example.org/ref", "https://example.org/taxonomy"]
        );
    }
    #[test]
    fn classifies_http_results_without_gating_review_or_transient_states() {
        assert_eq!(classify(204, false), "healthy");
        assert_eq!(classify(403, false), "needs_review");
        assert_eq!(classify(404, false), "broken");
        assert_eq!(classify(503, false), "transient");
        assert_eq!(classify(0, true), "transient");
    }
    #[test]
    fn deduplicates_urls_across_taxonomy_sources() {
        let dir =
            std::env::temp_dir().join(format!("taxonomy-link-monitor-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("catalog.yaml"),
            "url: https://example.org\n# citation: docs https://example.org/page.\n",
        )
        .unwrap();
        let urls = collect_urls(&dir).unwrap();
        assert_eq!(urls.len(), 2);
        let _ = fs::remove_dir_all(dir);
    }
}
