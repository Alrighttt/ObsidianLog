//! CLI surface tests.
//!
//! Verifies the argument parser stays wired correctly. These run today (they
//! don't touch the stubbed command bodies) and guard against accidental CLI
//! regressions.

use clap::CommandFactory;

use obsidianlog_cli::Cli;

#[test]
fn cli_definition_is_valid() {
    // Panics if the derive tree is malformed (duplicate args, bad defaults, …).
    Cli::command().debug_assert();
}
