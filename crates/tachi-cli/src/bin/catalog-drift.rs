use std::{path::PathBuf, process::ExitCode};
fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut root = PathBuf::from(".");
    let mut typst = PathBuf::from("typst");
    let mut mode = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" | "--regenerate-baselines" if mode.is_none() => mode = Some(arg),
            "--root" => match args.next() {
                Some(v) => root = v.into(),
                None => return usage(),
            },
            "--typst" => match args.next() {
                Some(v) => typst = v.into(),
                None => return usage(),
            },
            _ => return usage(),
        }
    }
    let result = match mode.as_deref() {
        Some("--check") => tachi_core::catalog_drift::check(&root),
        Some("--regenerate-baselines") => tachi_core::catalog_drift::regenerate(&root, &typst),
        _ => return usage(),
    };
    match result {
        Ok(()) => {
            println!("catalog-drift: passed");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("catalog-drift: {e}");
            ExitCode::FAILURE
        }
    }
}
fn usage() -> ExitCode {
    eprintln!(
        "usage: catalog-drift (--check | --regenerate-baselines) [--root PATH] [--typst PATH]"
    );
    ExitCode::from(2)
}
