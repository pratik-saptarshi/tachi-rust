use std::path::PathBuf;
use std::process::ExitCode;

use tachi_shell::commands::threats_sarif_output;

fn main() -> ExitCode {
    let (input, output, baseline_run_id, source_threats_uri) = match parse_args() {
        Ok(values) => values,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };

    let payload = match threats_sarif_output(
        &input,
        source_threats_uri.as_deref(),
        baseline_run_id.as_deref(),
    ) {
        Ok(payload) => payload,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if let Some(parent) = output.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            eprintln!("failed to create output directory: {err}");
            return ExitCode::from(1);
        }
    }
    if let Err(err) = std::fs::write(&output, payload.sarif.as_bytes()) {
        eprintln!("failed to write output file: {err}");
        return ExitCode::from(1);
    }

    eprintln!(
        "OK: wrote {} findings to {}",
        payload.findings_count,
        output.display()
    );
    eprintln!(
        "AG-8 present: {} ({})",
        payload.ag8_status.is_some(),
        payload.ag8_status.unwrap_or_else(|| String::from("absent"))
    );

    ExitCode::SUCCESS
}

fn parse_args() -> Result<(PathBuf, PathBuf, Option<String>, Option<String>), String> {
    let mut args = std::env::args().skip(1);
    let mut input = None;
    let mut output = None;
    let mut baseline_run_id = None;
    let mut source_threats_uri = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("--input requires a path argument"))?;
                input = Some(PathBuf::from(value));
            }
            "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("--output requires a path argument"))?;
                output = Some(PathBuf::from(value));
            }
            "--baseline-run-id" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("--baseline-run-id requires a value"))?;
                baseline_run_id = Some(value);
            }
            "--source-threats-uri" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("--source-threats-uri requires a value"))?;
                source_threats_uri = Some(value);
            }
            "--help" | "-h" => {
                return Err(String::from(
                    "usage: threats-sarif --input PATH --output PATH [--baseline-run-id ID] [--source-threats-uri URI]",
                ));
            }
            other => {
                return Err(format!("unrecognized argument: {other}"));
            }
        }
    }

    let input = input.ok_or_else(|| String::from("--input is required"))?;
    let output = output.ok_or_else(|| String::from("--output is required"))?;
    Ok((input, output, baseline_run_id, source_threats_uri))
}
