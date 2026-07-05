use std::path::PathBuf;
use std::process::ExitCode;

use tachi_shell::commands::risk_scores_sarif_output;

struct RiskScoresSarifArgs {
    risk_scores: PathBuf,
    threats: PathBuf,
    output: PathBuf,
    _baseline_run_id: Option<String>,
    _source_threats_uri: Option<String>,
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(values) => values,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };

    let payload = match risk_scores_sarif_output(&args.risk_scores, &args.threats) {
        Ok(payload) => payload,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if let Some(parent) = args.output.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            eprintln!("failed to create output directory: {err}");
            return ExitCode::from(1);
        }
    }
    if let Err(err) = std::fs::write(&args.output, payload.sarif.as_bytes()) {
        eprintln!("failed to write output file: {err}");
        return ExitCode::from(1);
    }

    eprintln!(
        "OK: wrote {} results to {}",
        payload.results_count,
        args.output.display()
    );
    ExitCode::SUCCESS
}

fn parse_args() -> Result<RiskScoresSarifArgs, String> {
    let mut args = std::env::args().skip(1);
    let mut risk_scores = None;
    let mut threats = None;
    let mut output = None;
    let mut baseline_run_id = None;
    let mut source_threats_uri = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--risk-scores" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("--risk-scores requires a path argument"))?;
                risk_scores = Some(PathBuf::from(value));
            }
            "--threats" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("--threats requires a path argument"))?;
                threats = Some(PathBuf::from(value));
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
                    "usage: risk-scores-sarif --risk-scores PATH --threats PATH --output PATH [--baseline-run-id ID] [--source-threats-uri URI]",
                ));
            }
            other => {
                return Err(format!("unrecognized argument: {other}"));
            }
        }
    }

    let risk_scores = risk_scores.ok_or_else(|| String::from("--risk-scores is required"))?;
    let threats = threats.ok_or_else(|| String::from("--threats is required"))?;
    let output = output.ok_or_else(|| String::from("--output is required"))?;
    Ok(RiskScoresSarifArgs {
        risk_scores,
        threats,
        output,
        _baseline_run_id: baseline_run_id,
        _source_threats_uri: source_threats_uri,
    })
}
