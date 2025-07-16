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
    } else if shell.contains("csh") && !shell.contains("tcsh") {
        ShellType::Csh
    } else if shell.contains("tcsh") {
        ShellType::Tcsh
    } else if shell.contains("ksh") {
        ShellType::Ksh
    } else if shell.contains("dash") {
        ShellType::Dash
    } else if shell.contains("mksh") {
        ShellType::Mksh
    } else if shell.contains("yash") {
        ShellType::Yash
    } else if shell.contains("osh") {
        ShellType::Osh
    } else if shell.contains("sh") {
        ShellType::Sh
    } else if shell.contains("bash") {
        ShellType::Bash
    } else {
        ShellType::Bash // Default fallback
    }
}

/// Parse shell history based on shell type and CLI args
pub fn parse_history(shell: &ShellType, args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    match shell {
        ShellType::Bash => parse_bash_history(args),
        ShellType::Zsh => parse_zsh_history(args),
        ShellType::Fish => parse_fish_history(args),
        ShellType::Csh => parse_csh_history(args),
        ShellType::Tcsh => parse_tcsh_history(args),
        ShellType::Ksh => parse_ksh_history(args),
        ShellType::Dash => parse_dash_history(args),
        ShellType::Sh => parse_sh_history(args),
        ShellType::Mksh => parse_mksh_history(args),
        ShellType::Yash => parse_yash_history(args),
        ShellType::Osh => parse_osh_history(args),
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

/// Parse csh history file (~/.history)
fn parse_csh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: csh history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open csh history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        entries.push(HistoryEntry {
            timestamp: None, // csh history usually does not have timestamps
            command: trimmed.to_string(),
            session_id: None,
        });
    }
    Ok(entries)
}

/// Parse tcsh history file (~/.history)
fn parse_tcsh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    // tcsh history format is similar to csh, but may include timestamps if set
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: tcsh history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open tcsh history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        // Example: 1234567890	command
        let (timestamp, command) = if let Some(tab_idx) = trimmed.find('\t') {
            let ts = trimmed[..tab_idx].parse::<i64>().ok();
            let cmd = trimmed[tab_idx+1..].to_string();
            let timestamp = ts.and_then(|t| chrono::Local.timestamp_opt(t, 0).single());
            (timestamp, cmd)
        } else {
            (None, trimmed.to_string())
        };
        entries.push(HistoryEntry {
            timestamp,
            command,
            session_id: None,
        });
    }
    Ok(entries)
}

/// Parse ksh history file (~/.sh_history)
fn parse_ksh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".sh_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: ksh history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open ksh history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        entries.push(HistoryEntry {
            timestamp: None, // ksh history usually does not have timestamps
            command: trimmed.to_string(),
            session_id: None,
        });
    }
    Ok(entries)
}

/// Parse dash history file
fn parse_dash_history(args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    // Dash typically uses the same history file as bash
    parse_bash_history(args)
}

/// Parse sh history file
fn parse_sh_history(args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    // sh typically uses the same history file as bash
    parse_bash_history(args)
}

/// Parse mksh history file (~/.mksh_history)
fn parse_mksh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".mksh_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: mksh history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open mksh history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        // TODO: Add timestamp parsing if present
        entries.push(HistoryEntry {
            timestamp: None,
            command: trimmed.to_string(),
            session_id: None,
        });
    }
    Ok(entries)
}

/// Parse yash history file (~/.yash_history)
fn parse_yash_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".yash_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: yash history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open yash history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        // TODO: Add timestamp parsing if present
        entries.push(HistoryEntry {
            timestamp: None,
            command: trimmed.to_string(),
            session_id: None,
        });
    }
    Ok(entries)
}

/// Parse osh history file (~/.osh_history)
fn parse_osh_history(_args: &CliArgs) -> Result<Vec<HistoryEntry>> {
    let mut entries = Vec::new();
    let hist_path = home_dir()
        .map(|d| d.join(".osh_history"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    if !hist_path.exists() {
        eprintln!("Warning: osh history file not found at {:?}", hist_path);
        return Ok(entries);
    }
    let file = File::open(&hist_path).context(format!("Failed to open osh history file: {:?}", hist_path))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        // TODO: Add timestamp parsing if present
        entries.push(HistoryEntry {
            timestamp: None,
            command: trimmed.to_string(),
            session_id: None,
        });
    }
    Ok(entries)
}
