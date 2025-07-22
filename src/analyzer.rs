//! Analytics and stats functions for shell history

use crate::cli::CliArgs;
use crate::models::HistoryEntry;
use anyhow::{Result, Context};
use std::collections::HashMap;

/// Group history entries into sessions based on a time gap (in minutes)
pub fn group_sessions<'a>(entries: &'a[&'a HistoryEntry], gap_minutes: i64) -> Vec<Vec<&'a HistoryEntry>> {
    let mut sessions = vec![];
    let mut current: Vec<&HistoryEntry> = vec![];
    let mut last_ts: Option<chrono::DateTime<chrono::Local>> = None;
    for entry in entries {
        if let Some(ts) = entry.timestamp {
            if let Some(last) = last_ts {
                if ts.signed_duration_since(last).num_minutes() > gap_minutes {
                    if !current.is_empty() {
                        sessions.push(current);
                        current = vec![];
                    }
                }
            }
            last_ts = Some(ts);
        }
        current.push(*entry);
    }
    if !current.is_empty() {
        sessions.push(current);
    }
    sessions
}

/// Suggest aliases for long or frequently used commands
pub fn suggest_aliases(history: &[HistoryEntry]) {
    use std::collections::HashMap;
    // Count full command lines (not just the first word)
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for entry in history {
        let cmd = entry.command.trim();
        if cmd.len() > 15 { // Only consider long commands
            *freq.entry(cmd).or_insert(0) += 1;
        }
    }
    let mut freq_vec: Vec<_> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\nAlias Suggestions (for long/frequent commands):");
    let mut alias_num = 1;
    for (cmd, count) in freq_vec.into_iter().take(10) {
        // Suggest a short alias (a1, a2, ...)
        let alias = format!("a{}", alias_num);
        println!("alias {}='{}'  # used {} times", alias, cmd, count);
        alias_num += 1;
    }
    if alias_num == 1 {
        println!("No long or frequent commands found for alias suggestion.");
    }
}

/// Flag potentially dangerous commands in history
pub fn flag_dangerous(history: &[HistoryEntry]) {
    // List of dangerous command patterns (simple, can be extended)
    let patterns = [
        "rm -rf", "rm -r /", "dd if=", "mkfs", ":(){ :|:& };:", "shutdown", "reboot", "curl | sh", "wget | sh", "chmod 777 /", "chown root", "> /dev/sda", "/dev/sda", ":(){ :|: & };:", "rm -rf --no-preserve-root", "poweroff", "halt", "init 0", "mkfs.ext", "dd of=/dev/", "mv /", "cp /dev/null", "yes | rm", "yes | dd", "yes | mkfs"
    ];
    println!("\nDangerous Command Flagging:");
    let mut found = false;
    for entry in history {
        for pat in &patterns {
            if entry.command.contains(pat) {
                println!("⚠️  {}\n    ↳ Matched pattern: '{}'", entry.command, pat);
                found = true;
                break;
            }
        }
    }
    if !found {
        println!("No dangerous commands found in history.");
    }
}

/// Analyze history and print stats in CLI mode
/// Handles filtering, searching, session summary, and export
pub fn analyze_history(history: &Vec<HistoryEntry>, args: &CliArgs) -> Result<()> {
    use chrono::NaiveDate;
    use regex::Regex;
    use std::fs::File;
    use std::io::Write;
    if history.is_empty() {
        println!("No history entries found.");
        return Ok(());
    }
    let mut filtered: Vec<&HistoryEntry> = history.iter().collect();
    // --filter <command>
    if let Some(ref filter) = args.filter {
        filtered.retain(|e| e.command.starts_with(filter));
    }
    // --search <pattern>
    if let Some(ref pat) = args.search {
        let re = Regex::new(pat).context("Invalid regex pattern")?;
        filtered.retain(|e| re.is_match(&e.command));
    }
    // --range "YYYY-MM-DD:YYYY-MM-DD"
    if let Some(ref range) = args.range {
        let parts: Vec<_> = range.split(':').collect();
        if parts.len() == 2 {
            let start = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").context("Invalid start date")?;
            let end = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d").context("Invalid end date")?;
            filtered.retain(|e| {
                if let Some(ts) = e.timestamp {
                    let date = ts.date_naive();
                    date >= start && date <= end
                } else {
                    false
                }
            });
        }
    }
    // --suggest-aliases
    if args.suggest_aliases {
        suggest_aliases(&filtered.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
        return Ok(());
    }
    // --flag-dangerous
    if args.flag_dangerous {
        flag_dangerous(&filtered.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
        return Ok(());
    }
    // --top N
    if let Some(top_n) = args.top {
        let mut freq: HashMap<&str, usize> = HashMap::new();
        for entry in &filtered {
            let cmd = entry.command.split_whitespace().next().unwrap_or("");
            *freq.entry(cmd).or_insert(0) += 1;
        }
        let mut freq_vec: Vec<_> = freq.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        println!("Top {} commands:", top_n);
        for (i, (cmd, count)) in freq_vec.into_iter().take(top_n).enumerate() {
            println!("{:>2}. {:<20} {}", i+1, cmd, count);
        }
        return Ok(());
    }
    // --session-summary
    if args.session_summary {
        let sessions = group_sessions(&filtered, 10);
        println!("Total sessions: {}", sessions.len());
        let avg_len = if !sessions.is_empty() {
            sessions.iter().map(|s| s.len()).sum::<usize>() as f64 / sessions.len() as f64
        } else { 0.0 };
        println!("Average session length: {:.2} commands", avg_len);
        return Ok(());
    }
    // --export <format>
    if let Some(ref fmt) = args.export {
        match fmt.as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&filtered).context("Failed to serialize JSON")?;
                let mut f = File::create("heist_export.json").context("Failed to create JSON export file")?;
                f.write_all(json.as_bytes()).context("Failed to write JSON export")?;
                println!("Exported to heist_export.json");
            },
            "csv" => {
                let mut f = File::create("heist_export.csv").context("Failed to create CSV export file")?;
                writeln!(f, "timestamp,command").context("Failed to write CSV header")?;
                for e in &filtered {
                    let ts = e.timestamp.map(|t| t.to_rfc3339()).unwrap_or_default();
                    writeln!(f, "{}{},{}", ts, if ts.is_empty() {""} else {""}, e.command.replace(',', " ")).context("Failed to write CSV row")?;
                }
                println!("Exported to heist_export.csv");
            },
            _ => println!("Unknown export format: {}", fmt),
        }
        return Ok(());
    }
    // Default: print all filtered commands
    println!("{} entries:", filtered.len());
    for entry in filtered {
        println!("{}", entry.command);
    }
    Ok(())
}
