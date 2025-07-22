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

    /// Suggest aliases for long or frequently used commands
    #[arg(long)]
    pub suggest_aliases: bool,

    /// Flag potentially dangerous commands in history
    #[arg(long)]
    pub flag_dangerous: bool,

    /// Show per-directory command stats
    #[arg(long)]
    pub per_directory: bool,
    /// Show per-host command stats
    #[arg(long)]
    pub per_host: bool,

    /// Show time-of-day command usage analytics
    #[arg(long)]
    pub time_of_day: bool,
    /// Show weekly heatmap of command usage
    #[arg(long)]
    pub heatmap: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Csh,
    Tcsh,
    Ksh,
    Dash,
    Sh,
    Mksh,
    Yash,
    Osh,
    // Add more as needed
}
