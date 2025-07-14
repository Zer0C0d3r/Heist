//! Main entry point for Heist: Cross-platform shell history analyzer
//! Handles CLI argument parsing, shell detection, and mode switching (CLI/TUI)

mod cli;
mod parser;
mod ui;
mod analyzer;
mod models;

use clap::Parser;
use anyhow::Result;
use crate::cli::CliArgs;
use crate::parser::{detect_shell, parse_history};
use crate::ui::run_tui;
use crate::analyzer::analyze_history;

fn main() -> Result<()> {
    // Parse CLI arguments
    let args = CliArgs::parse();

    // Detect shell type (unless overridden)
    let shell = args.shell.clone().unwrap_or_else(detect_shell);

    // Parse shell history
    let history = parse_history(&shell, &args)?;

    if args.cli {
        // Non-interactive CLI mode
        analyze_history(&history, &args)?;
    } else {
        // Interactive TUI mode
        run_tui(&history, &args)?;
    }
    Ok(())
}
