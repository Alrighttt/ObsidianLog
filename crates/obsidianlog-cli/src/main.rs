//! Thin binary entry point for the ObsidianLog CLI.
//!
//! All logic lives in the `obsidianlog-cli` library crate. `main` only parses
//! arguments, hands off to [`obsidianlog_cli::run`], and renders top-level errors.

use std::process::ExitCode;

use clap::Parser;

use obsidianlog_cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match obsidianlog_cli::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // `{:#}` renders the full anyhow context chain on one line.
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}
