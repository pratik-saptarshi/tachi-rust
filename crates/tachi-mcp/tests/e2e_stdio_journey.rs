use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{json, Value};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn fixture_root() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "tachi-mcp-stdio-e2e-{}-{}",
        std::process::id(),
        TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root).expect("create MCP fixture root");
    root
}

fn binary_path() -> PathBuf {
    std::env::var("CARGO_BIN_EXE_tachi-mcp")
        .expect("CARGO_BIN_EXE_tachi-mcp should be provided by cargo")
        .into()
}

fn response_lines(bytes: &[u8]) -> Vec<Value> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(|line| serde_json::from_str(line).expect("valid MCP response JSON"))
        .collect()
}

#[test]
fn mcp_stdio_process_round_trips_artifact_request_and_metadata() {
    let root = fixture_root();
    let request = json!({
        "request_id": "stdio-e2e-1",
        "tool": "tachi.coverage-audit",
        "input": {
            "repo_root": root,
            "output_mode": "artifact"
        }
    });

    let mut child = Command::new(binary_path())
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn MCP stdio process");
    child
        .stdin
        .take()
        .expect("MCP stdin")
        .write_all(format!("{request}\n").as_bytes())
        .expect("write MCP request");

    let output = child.wait_with_output().expect("wait for MCP process");
    assert!(output.status.success(), "MCP stderr: {:?}", output.stderr);
    let responses = response_lines(&output.stdout);
    assert_eq!(responses.len(), 1);
    let response = &responses[0];
    assert_eq!(response["request_id"], "stdio-e2e-1");
    assert_eq!(response["ok"], true);
    assert_eq!(response["result"]["tool_name"], "tachi.coverage-audit");
    assert_eq!(response["result"]["output_mode"], "artifact");
    let artifact_path = response["result"]["artifact_path"]
        .as_str()
        .expect("artifact path metadata");
    assert!(PathBuf::from(artifact_path).exists());
    assert!(fs::read_to_string(artifact_path)
        .expect("read MCP artifact")
        .contains("Coverage audit for"));
}

#[test]
fn mcp_process_requires_explicit_stdio_startup() {
    let output = Command::new(binary_path())
        .output()
        .expect("run MCP process without startup mode");
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing required --stdio flag"));
}

#[test]
fn mcp_stdio_process_returns_fail_closed_responses_for_unknown_and_cancelled_requests() {
    let root = fixture_root();
    let requests = [
        json!({
            "request_id": "stdio-unknown",
            "tool": "tachi.install",
            "input": {}
        }),
        json!({
            "request_id": "stdio-cancelled",
            "tool": "tachi.coverage-audit",
            "cancelled": true,
            "input": {
                "repo_root": root,
                "output_mode": "artifact"
            }
        }),
    ];

    let input = requests
        .iter()
        .map(|request| format!("{request}\n"))
        .collect::<String>();
    let mut child = Command::new(binary_path())
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn MCP stdio process for failures");
    child
        .stdin
        .take()
        .expect("MCP stdin")
        .write_all(input.as_bytes())
        .expect("write MCP failure requests");
    let mut stdout = Vec::new();
    child
        .stdout
        .take()
        .expect("MCP stdout")
        .read_to_end(&mut stdout)
        .expect("read MCP responses");
    let status = child.wait().expect("wait for MCP failure process");
    assert!(status.success());

    let responses = response_lines(&stdout);
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0]["request_id"], "stdio-unknown");
    assert_eq!(responses[0]["ok"], false);
    assert!(responses[0]["error"]
        .as_str()
        .unwrap()
        .contains("authorization error"));
    assert_eq!(responses[1]["request_id"], "stdio-cancelled");
    assert_eq!(responses[1]["ok"], false);
    assert!(responses[1]["error"]
        .as_str()
        .unwrap()
        .contains("cancelled"));
    assert!(responses[1]["result"].is_null());
}
