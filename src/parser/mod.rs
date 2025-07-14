//! Shell history parser module
//! Supports bash, zsh, and fish history formats

use crate::cli::{CliArgs, ShellType};
use anyhow::Result;
use crate::models::HistoryEntry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use dirs::home_dir;
use chrono::{DateTime, Local, TimeZone};
use regex::Regex;


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

pub fn parse_history(shell: &ShellType, args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    match shell {
        ShellType::Bash => parse_bash_history(args),
        ShellType::Zsh => parse_zsh_history(args),
        ShellType::Fish => parse_fish_history(args),
    }
}

fn parse_bash_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    // Locate ~/.bash_history
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".bash_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        return Ok(entries);
    }
    let file = File::open(hist_path)?;
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

fn parse_zsh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    use chrono::TimeZone;
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".zsh_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        return Ok(entries);
    }
    let file = File::open(hist_path)?;
    let reader = BufReader::new(file);
    let re = regex::Regex::new(r"^: (\d+):\d+;(.*)").unwrap();
    for line in reader.lines() {
        let line = line?;
        if let Some(cap) = re.captures(&line) {
            let ts = cap[1].parse::<i64>().ok();
            let command = cap[2].trim().to_string();
            let timestamp = ts.and_then(|t| {
                let ndt = chrono::NaiveDateTime::from_timestamp(t, 0);
                chrono::Local.from_local_datetime(&ndt).single()
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

fn parse_fish_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    use serde_yaml::Value;
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".local/share/fish/fish_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        return Ok(entries);
    }
    let file = File::open(hist_path)?;
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
                let ndt = chrono::NaiveDateTime::from_timestamp(t, 0);
                chrono::Local.from_local_datetime(&ndt).single()
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
