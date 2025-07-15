//! Shell history parser module
//! Supports bash, zsh, and fish history formats

use crate::cli::{CliArgs, ShellType};
use anyhow::{Result, Context};
use crate::models::HistoryEntry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use dirs::home_dir;
use chrono::{DateTime, Local, TimeZone};
use regex::Regex;

/// Detect the user's shell from the SHELL environment variable
pub fn detect_shell() -> ShellType {
    use std::env;
    let shell = env::var("SHELL").unwrap_or_default();
    if shell.contains("zsh") {
        ShellType::Zsh
    } else if shell.contains("fish") {
        ShellType::Fish
    } else {
        ShellType::Bash
    }
}

/// Parse shell history based on shell type and CLI args
pub fn parse_history(shell: &ShellType, args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    match shell {
        ShellType::Bash => parse_bash_history(args),
        ShellType::Zsh => parse_zsh_history(args),
        ShellType::Fish => parse_fish_history(args),
    }
}

/// Parse bash history file (~/.bash_history)
fn parse_bash_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".bash_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: Bash history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open Bash history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        entries.push(HistoryEntry {
            timestamp: None,
            command: trimmed.to_string(),
            session_id: None,
        });
    }
    Ok(entries)
}

/// Parse zsh history file (~/.zsh_history)
fn parse_zsh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    use chrono::TimeZone;
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".zsh_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: Zsh history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open Zsh history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    let re = regex::Regex::new(r"^: (\d+):\d+;(.*)").unwrap();
    for line in reader.lines() {
        let line = line?;
        if let Some(cap) = re.captures(&line) {
            let ts = cap[1].parse::<i64>().ok();
            let command = cap[2].trim().to_string();
            let timestamp = ts.and_then(|t| {
                chrono::Local.timestamp_opt(t, 0).single()
            });
            entries.push(HistoryEntry {
                timestamp,
                command,
                session_id: None,
            });
        } else if !line.trim().is_empty() {
            entries.push(HistoryEntry {
                timestamp: None,
                command: line.trim().to_string(),
                session_id: None,
            });
        }
    }
    Ok(entries)
}

/// Parse fish history file (~/.local/share/fish/fish_history)
fn parse_fish_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".local/share/fish/fish_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: Fish history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open Fish history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    let mut command = None;
    let mut timestamp = None;
    for line in reader.lines() {
        let line = line?;
        if line.trim_start().starts_with("- cmd: ") {
            if let Some(cmd) = command.take() {
                entries.push(HistoryEntry {
                    timestamp,
                    command: cmd,
                    session_id: None,
                });
                timestamp = None;
            }
            command = Some(line.trim_start()[7..].to_string());
        } else if line.trim_start().starts_with("  when: ") {
            let ts = line.trim_start()[8..].parse::<i64>().ok();
            timestamp = ts.map(|t| {
                chrono::Local.timestamp_opt(t, 0).single()
            }).flatten();
        }
    }
    if let Some(cmd) = command.take() {
        entries.push(HistoryEntry {
            timestamp,
            command: cmd,
            session_id: None,
        });
    }
    Ok(entries)
}
