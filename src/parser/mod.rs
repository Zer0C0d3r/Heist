//! Shell history parser module
//! Supports bash, zsh, fish, and other Unix shells

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write as IoWrite};
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, TimeZone};
use dirs::home_dir;
use regex::Regex;

use crate::cli::{CliArgs, ShellType};
use crate::models::HistoryEntry;

// Logging macro for errors
macro_rules! log_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        eprintln!("[heist error] {}", msg);
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("heist_error.log") 
        {
            let _ = writeln!(f, "{}", msg);
        }
    }};
}

/// Detect the user's shell from the SHELL environment variable
pub fn detect_shell() -> ShellType {
    let shell = std::env::var("SHELL").unwrap_or_default();
    
    match shell.as_str() {
        s if s.contains("zsh") => ShellType::Zsh,
        s if s.contains("fish") => ShellType::Fish,
        s if s.contains("tcsh") => ShellType::Tcsh,
        s if s.contains("csh") => ShellType::Csh,
        s if s.contains("ksh") => ShellType::Ksh,
        s if s.contains("dash") => ShellType::Dash,
        s if s.contains("mksh") => ShellType::Mksh,
        s if s.contains("yash") => ShellType::Yash,
        s if s.contains("osh") => ShellType::Osh,
        s if s.contains("sh") => ShellType::Sh,
        s if s.contains("bash") => ShellType::Bash,
        _ => ShellType::Bash, // Default fallback
    }
}

/// Parse shell history based on shell type and CLI args
pub fn parse_history(shell: &ShellType, args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = match shell {
        ShellType::Bash => parse_bash_history(args)?,
        ShellType::Zsh => parse_zsh_history(args)?,
        ShellType::Fish => parse_fish_history(args)?,
        ShellType::Csh => parse_csh_history(args)?,
        ShellType::Tcsh => parse_tcsh_history(args)?,
        ShellType::Ksh => parse_ksh_history(args)?,
        ShellType::Dash => parse_dash_history(args)?,
        ShellType::Sh => parse_sh_history(args)?,
        ShellType::Mksh => parse_mksh_history(args)?,
        ShellType::Yash => parse_yash_history(args)?,
        ShellType::Osh => parse_osh_history(args)?,
    };

    // Merge live tracking history
    let mut live_entries = parse_heist_live_history();
    entries.append(&mut live_entries);

    // Sort and deduplicate
    entries.sort_by_key(|e| e.timestamp);
    entries.dedup_by(|a, b| a.timestamp == b.timestamp && a.command == b.command);

    if entries.is_empty() {
        log_error!("No entries parsed for shell {:?}", shell);
    }

    Ok(entries)
}

/// Get home directory with error handling
fn get_home_dir() -> Result<std::path::PathBuf> {
    home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
}

/// Read lines from a history file
fn read_history_file(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        eprintln!("Warning: History file not found at {:?}", path);
        return Ok(Vec::new());
    }

    let file = File::open(path)
        .context(format!("Failed to open history file: {:?}", path))?;
    
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<Vec<_>, _>>()
        .context("Failed to read history file lines")
}

/// Create a basic history entry
fn create_entry(command: String, timestamp: Option<DateTime<Local>>) -> HistoryEntry {
    HistoryEntry {
        timestamp,
        command,
        session_id: None,
    }
}

/// Parse bash history file (~/.bash_history)
fn parse_bash_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let hist_path = get_home_dir()?.join(".bash_history");
    let lines = read_history_file(&hist_path)?;
    
    Ok(lines
        .into_iter()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(create_entry(trimmed.to_string(), None))
            }
        })
        .collect())
}

/// Parse zsh history file (~/.zsh_history)
fn parse_zsh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let hist_path = get_home_dir()?.join(".zsh_history");
    let lines = read_history_file(&hist_path)?;
    let re = Regex::new(r"^: (\d+):\d+;(.*)").unwrap();
    
    Ok(lines
        .into_iter()
        .filter_map(|line| {
            if let Some(cap) = re.captures(&line) {
                let timestamp = cap[1]
                    .parse::<i64>()
                    .ok()
                    .and_then(|t| Local.timestamp_opt(t, 0).single());
                let command = cap[2].trim().to_string();
                Some(create_entry(command, timestamp))
            } else if !line.trim().is_empty() {
                Some(create_entry(line.trim().to_string(), None))
            } else {
                None
            }
        })
        .collect())
}

/// Parse fish history file (~/.local/share/fish/fish_history)
fn parse_fish_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let hist_path = get_home_dir()?.join(".local/share/fish/fish_history");
    let lines = read_history_file(&hist_path)?;
    
    let mut entries = Vec::new();
    let mut current_command = None;
    let mut current_timestamp = None;
    
    for line in lines {
        if line.trim_start().starts_with("- cmd: ") {
            // Save previous entry if exists
            if let Some(cmd) = current_command.take() {
                entries.push(create_entry(cmd, current_timestamp.take()));
            }
            current_command = Some(line.trim_start()[7..].to_string());
        } else if line.trim_start().starts_with("  when: ") {
            current_timestamp = line.trim_start()[8..]
                .parse::<i64>()
                .ok()
                .and_then(|t| Local.timestamp_opt(t, 0).single());
        }
    }
    
    // Add final entry if exists
    if let Some(cmd) = current_command {
        entries.push(create_entry(cmd, current_timestamp));
    }
    
    Ok(entries)
}

/// Infer timestamps for plain-text history files using file modification time
fn infer_timestamps_from_file(hist_path: &Path, line_count: usize) -> Vec<Option<DateTime<Local>>> {
    let mtime = std::fs::metadata(hist_path)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(DateTime::<Local>::from);
    
    match mtime {
        Some(last_ts) => {
            // Spread timestamps backwards by 1 minute per command
            (0..line_count)
                .rev()
                .map(|i| Some(last_ts - chrono::Duration::minutes((line_count - 1 - i) as i64)))
                .collect()
        }
        None => vec![None; line_count],
    }
}

/// Parse history files without native timestamps
fn parse_plain_history(file_name: &str) -> Result<Vec<HistoryEntry>> {
    let hist_path = get_home_dir()?.join(file_name);
    let lines: Vec<String> = read_history_file(&hist_path)?
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect();
    
    let timestamps = infer_timestamps_from_file(&hist_path, lines.len());
    
    Ok(lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let timestamp = timestamps.get(i).cloned().unwrap_or(None);
            create_entry(line.trim().to_string(), timestamp)
        })
        .collect())
}

/// Parse csh history file (~/.history)
fn parse_csh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_plain_history(".history")
}

/// Parse tcsh history file (~/.history)
fn parse_tcsh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let hist_path = get_home_dir()?.join(".history");
    let lines = read_history_file(&hist_path)?;
    
    Ok(lines
        .into_iter()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            
            // Check for tab-separated timestamp format: timestamp\tcommand
            if let Some(tab_idx) = trimmed.find('\t') {
                let timestamp = trimmed[..tab_idx]
                    .parse::<i64>()
                    .ok()
                    .and_then(|t| Local.timestamp_opt(t, 0).single());
                let command = trimmed[tab_idx + 1..].to_string();
                Some(create_entry(command, timestamp))
            } else {
                Some(create_entry(trimmed.to_string(), None))
            }
        })
        .collect())
}

/// Parse ksh history file (~/.sh_history)
fn parse_ksh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_plain_history(".sh_history")
}

/// Parse dash history file (uses bash format)
fn parse_dash_history(args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_bash_history(args)
}

/// Parse sh history file (uses bash format)
fn parse_sh_history(args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_bash_history(args)
}

/// Parse mksh history file (~/.mksh_history)
fn parse_mksh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_plain_history(".mksh_history")
}

/// Parse yash history file (~/.yash_history)
fn parse_yash_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_plain_history(".yash_history")
}

/// Parse osh history file (~/.osh_history)
fn parse_osh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    parse_plain_history(".osh_history")
}

/// Parse live-tracked history file (~/.heist_live_history)
pub fn parse_heist_live_history() -> Vec<HistoryEntry> {
    let path = match get_home_dir() {
        Ok(home) => home.join(".heist_live_history"),
        Err(_) => return Vec::new(),
    };
    
    if !path.exists() {
        return Vec::new();
    }
    
    let Ok(file) = File::open(&path) else {
        return Vec::new();
    };
    
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            // Format: 2024-06-09T12:34:56+0000|command
            let (ts_str, cmd) = line.split_once('|')?;
            let timestamp = chrono::DateTime::parse_from_str(ts_str, "%Y-%m-%dT%H:%M:%S%z")
                .ok()
                .map(|dt| dt.with_timezone(&Local));
            
            Some(create_entry(cmd.trim().to_string(), timestamp))
        })
        .collect()
}