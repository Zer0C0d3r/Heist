//! CLI argument parsing using clap
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
#[command(name = "heist", about = "Cross-platform shell history analyzer")]
pub struct CliArgs {
    /// Run in non-interactive CLI mode
    #[arg(long)]
    pub cli: bool,

    /// Show top N most used commands
    #[arg(long, value_name = "N")]
    pub top: Option<usize>,

    /// Search for commands using regex
    #[arg(long, value_name = "PATTERN")]
    pub search: Option<String>,

    /// Only analyze specific commands
    #[arg(long, value_name = "COMMAND")]
    pub filter: Option<String>,

    /// Filter by time range (YYYY-MM-DD:YYYY-MM-DD)
    #[arg(long, value_name = "RANGE")]
    pub range: Option<String>,

    /// Export data to CSV or JSON
    #[arg(long, value_name = "FORMAT")]
    pub export: Option<String>,

    /// Force shell type (bash, zsh, fish)
    #[arg(long, value_enum, value_name = "SHELL")]
    pub shell: Option<ShellType>,

    /// Print session-level stats
    #[arg(long)]
    pub session_summary: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
}
