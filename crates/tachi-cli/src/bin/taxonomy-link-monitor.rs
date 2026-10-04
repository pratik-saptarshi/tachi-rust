use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
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
    run_with(&root, &report_path, check_url)
}

fn run_with(
    root: &Path,
    report_path: &Path,
    mut check: impl FnMut(&str) -> serde_json::Value,
) -> Result<(), String> {
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
        let result = check(&url);
        hosts.insert(host, Instant::now());
        results.push(result);
    }
    let report = json!({ "checked_at": now_utc(), "results": results });
    let serialized = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    fs::write(report_path, serialized)
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
    check_url_with(url, curl)
}

fn check_url_with(
    url: &str,
    mut request: impl FnMut(&str, bool) -> (u16, Option<String>),
) -> serde_json::Value {
    let head = request(url, true);
    let (code, error) = if head.0 == 405 || head.0 == 501 || head.1.is_some() {
        request(url, false)
    } else {
        head
    };
    json!({"url": url, "http_status": code, "status": classify(code, error.is_some()), "error": error})
}

fn curl(url: &str, head: bool) -> (u16, Option<String>) {
    curl_with_program(url, head, OsStr::new("curl"))
}

fn curl_with_program(url: &str, head: bool, program: &OsStr) -> (u16, Option<String>) {
    let mut command = Command::new(program);
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
    use std::os::unix::fs::PermissionsExt;

    fn fake_curl(contents: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "taxonomy-link-monitor-fake-curl-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::write(&path, contents).expect("write fake curl");
        let mut permissions = fs::metadata(&path).expect("stat fake curl").permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&path, permissions).expect("make fake curl executable");
        path
    }

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
    fn extracts_http_urls_and_trims_supported_trailing_punctuation() {
        for punctuation in [")", "]", "}", ",", ";", ".", "\"", "'"] {
            let text = format!("citation: http://example.org/ref{punctuation}");
            assert_eq!(extract_urls(&text), ["http://example.org/ref"]);
        }
        assert!(extract_urls("citation: www.example.org").is_empty());
    }

    #[test]
    fn classifies_http_results_without_gating_review_or_transient_states() {
        assert_eq!(classify(204, false), "healthy");
        assert_eq!(classify(399, false), "healthy");
        assert_eq!(classify(401, false), "needs_review");
        assert_eq!(classify(403, false), "needs_review");
        assert_eq!(classify(429, false), "needs_review");
        assert_eq!(classify(400, false), "broken");
        assert_eq!(classify(404, false), "broken");
        assert_eq!(classify(499, false), "broken");
        assert_eq!(classify(199, false), "transient");
        assert_eq!(classify(503, false), "transient");
        assert_eq!(classify(0, true), "transient");
        assert_eq!(classify(0, false), "transient");
        assert_eq!(classify(204, true), "healthy");
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

    #[test]
    fn collects_only_yaml_citation_fields_and_reports_read_errors() {
        let dir = std::env::temp_dir().join(format!(
            "taxonomy-link-monitor-filter-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("catalog.yaml"),
            "title: https://example.org/ignored\nurl: https://example.org/one\ncitation: See https://example.org/two.\n# citation: https://example.org/three\n",
        )
        .unwrap();
        fs::write(dir.join("notes.txt"), "url: https://example.org/not-yaml\n").unwrap();
        assert_eq!(
            collect_urls(&dir).unwrap(),
            [
                "https://example.org/one",
                "https://example.org/three",
                "https://example.org/two"
            ]
            .into_iter()
            .map(str::to_owned)
            .collect()
        );

        let broken = dir.join("broken.yaml");
        fs::create_dir(&broken).unwrap();
        assert!(collect_urls(&dir).unwrap_err().contains("broken.yaml"));
        assert!(collect_urls(&dir.join("missing"))
            .unwrap_err()
            .contains("read"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn summarizes_known_and_unexpected_status_values() {
        let values = serde_json::json!([
            {"status":"healthy"},
            {"status":"needs_review"},
            {"status":"broken"},
            {"status":"future"},
            {}
        ]);
        assert_eq!(report_summary(&values), (5, 1, 1, 1, 2));
        assert_eq!(report_summary(&serde_json::json!({})), (0, 0, 0, 0, 0));
    }

    #[test]
    fn extracts_lowercase_host_with_and_without_a_scheme() {
        assert_eq!(host_of("https://Example.ORG/path"), "example.org");
        assert_eq!(host_of("Example.ORG/path"), "example.org/path");
        assert_eq!(host_of("Example.ORG"), "example.org");
    }

    #[test]
    fn retries_get_when_head_is_unsupported_or_fails() {
        for head in [
            (405, None),
            (501, None),
            (0, Some("curl unavailable".to_owned())),
        ] {
            let mut calls = Vec::new();
            let result = check_url_with("https://example.org", |url, is_head| {
                calls.push((url.to_owned(), is_head));
                if is_head {
                    head.clone()
                } else {
                    (206, None)
                }
            });
            assert_eq!(result["status"], "healthy");
            assert_eq!(calls.len(), 2);
            assert!(calls[0].1);
            assert!(!calls[1].1);
        }
    }

    #[test]
    fn keeps_successful_head_responses_without_a_get_retry() {
        let mut calls = Vec::new();
        let result = check_url_with("https://example.org", |url, is_head| {
            calls.push((url.to_owned(), is_head));
            (403, None)
        });
        assert_eq!(result["status"], "needs_review");
        assert_eq!(calls, [("https://example.org".to_owned(), true)]);
    }

    #[test]
    fn curl_parses_http_status_and_reports_process_failures() {
        let success = fake_curl("#!/bin/sh\nprintf '204'\n");
        assert_eq!(
            curl_with_program("https://example.org", true, success.as_os_str()),
            (204, None)
        );
        let _ = fs::remove_file(success);

        let failure =
            fake_curl("#!/bin/sh\nprintf 'not-a-status'\nprintf 'fixture failure' >&2\nexit 7\n");
        let (code, error) = curl_with_program("https://example.org", false, failure.as_os_str());
        assert_eq!(code, 0);
        assert_eq!(error.as_deref(), Some("fixture failure"));
        let _ = fs::remove_file(failure);

        let missing = std::env::temp_dir().join(format!(
            "taxonomy-link-monitor-no-curl-{}",
            std::process::id()
        ));
        let (code, error) = curl_with_program("https://example.org", true, missing.as_os_str());
        assert_eq!(code, 0);
        assert!(error.is_some());
    }

    #[test]
    fn run_limits_request_frequency_per_host_and_writes_summary() {
        let root = std::env::temp_dir().join(format!(
            "taxonomy-link-monitor-rate-limit-{}",
            std::process::id()
        ));
        let taxonomy = root.join("schemas/taxonomy");
        fs::create_dir_all(&taxonomy).unwrap();
        fs::write(
            taxonomy.join("links.yaml"),
            "url: https://example.org/one\nurl: https://example.org/two\nurl: https://example.org/three\n",
        )
        .unwrap();
        let report = root.join("report.json");
        let mut calls = 0;
        run_with(&root, &report, |_| {
            calls += 1;
            if calls == 2 {
                thread::sleep(Duration::from_millis(260));
            }
            json!({"status":"healthy"})
        })
        .unwrap();
        assert_eq!(calls, 3);
        let result: serde_json::Value =
            serde_json::from_slice(&fs::read(&report).unwrap()).unwrap();
        assert_eq!(report_summary(&result["results"]), (3, 3, 0, 0, 0));
        let _ = fs::remove_dir_all(root);
    }
}
