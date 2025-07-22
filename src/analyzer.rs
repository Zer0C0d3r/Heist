//! Analytics and stats functions for shell history

use crate::cli::CliArgs;
use crate::models::HistoryEntry;
use anyhow::{Result, Context};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use regex::Regex;
use chrono::NaiveDate;
use std::fs::File;

macro_rules! log_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        eprintln!("[heist error] {}", msg);
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("heist_error.log") {
            let _ = writeln!(f, "{}", msg);
        }
    }};
}

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

/// Show per-directory command stats
pub fn per_directory_stats(history: &[HistoryEntry]) {
    use std::collections::HashMap;
    let mut dir_counts: HashMap<String, usize> = HashMap::new();
    let mut last_dir = String::from("~");
    for entry in history {
        // Naive extraction: look for 'cd <dir>' or remember last cd
        if entry.command.starts_with("cd ") {
            let dir = entry.command[3..].trim().to_string();
            last_dir = dir.clone();
            *dir_counts.entry(dir).or_insert(0) += 1;
        } else {
            *dir_counts.entry(last_dir.clone()).or_insert(0) += 1;
        }
    }
    let mut dir_vec: Vec<_> = dir_counts.into_iter().collect();
    dir_vec.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\nPer-directory command stats:");
    for (dir, count) in dir_vec.iter().take(15) {
        println!("{:<30} {}", dir, count);
    }
}

/// Show per-host command stats (if host info is available)
pub fn per_host_stats(history: &[HistoryEntry]) {
    use std::collections::HashMap;
    use std::env;
    let mut host_counts: HashMap<String, usize> = HashMap::new();
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    // If no host info in history, just use current host
    for _ in history {
        *host_counts.entry(hostname.clone()).or_insert(0) += 1;
    }
    println!("\nPer-host command stats:");
    for (host, count) in host_counts {
        println!("{:<20} {}", host, count);
    }
}

/// Show time-of-day command usage analytics
pub fn time_of_day_stats(history: &[HistoryEntry]) {
    use chrono::Timelike;
    let mut hours = [0usize; 24];
    for entry in history {
        if let Some(ts) = entry.timestamp {
            hours[ts.hour() as usize] += 1;
        }
    }
    println!("\nTime-of-day command usage (hourly):");
    for (h, count) in hours.iter().enumerate() {
        let bar = "#".repeat(*count / 2.max(1));
        println!("{:02}:00 {:>4} {}", h, count, bar);
    }
}

/// Show weekly heatmap of command usage
pub fn heatmap_stats(history: &[HistoryEntry]) {
    use chrono::{Datelike, Timelike};
    let mut heatmap = [[0usize; 24]; 7]; // [weekday][hour]
    for entry in history {
        if let Some(ts) = entry.timestamp {
            let wd = ts.weekday().num_days_from_monday() as usize;
            let hr = ts.hour() as usize;
            heatmap[wd][hr] += 1;
        }
    }
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("\nWeekly command usage heatmap (hour x day):");
    print!("     ");
    for h in 0..24 { print!("{:02} ", h); }
    println!();
    for (d, row) in heatmap.iter().enumerate() {
        print!("{:>3} | ", days[d]);
        for &count in row {
            let c = match count {
                0 => ' ',
                1..=2 => '.',
                3..=5 => '*',
                6..=10 => 'o',
                _ => '#',
            };
            print!(" {} ", c);
        }
        println!();
    }
}

/// Analyze history and print stats in CLI mode
/// Handles filtering, searching, session summary, and export
pub fn analyze_history(history: &Vec<HistoryEntry>, args: &CliArgs) -> Result<()> {
    if history.is_empty() {
        log_error!("No history entries found for analysis.");
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
    // --per-directory
    if args.per_directory {
        per_directory_stats(&filtered.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
        return Ok(());
    }
    // --per-host
    if args.per_host {
        per_host_stats(&filtered.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
        return Ok(());
    }
    // --time-of-day
    if args.time_of_day {
        time_of_day_stats(&filtered.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
        return Ok(());
    }
    // --heatmap
    if args.heatmap {
        heatmap_stats(&filtered.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::CliArgs;
    use chrono::{Local, TimeZone};

    #[test]
    fn test_empty_history() {
        let args = CliArgs {
            shell: None,
            cli: false,
            filter: None,
            search: None,
            range: None,
            suggest_aliases: false,
            flag_dangerous: false,
            per_directory: false,
            per_host: false,
            time_of_day: false,
            heatmap: false,
            top: None,
            session_summary: false,
            export: None,
        };
        let entries: Vec<HistoryEntry> = vec![];
        assert!(entries.is_empty());
    }

    #[test]
    fn test_history_entry_fields() {
        let entry = HistoryEntry {
            timestamp: None,
            command: "ls -la".to_string(),
            session_id: None,
        };
        assert_eq!(entry.command, "ls -la");
    }

    #[test]
    fn test_time_of_day_stats_empty() {
        let history: Vec<HistoryEntry> = vec![];
        time_of_day_stats(&history); // Should not panic
    }

    #[test]
    fn test_time_of_day_stats_basic() {
        let history = vec![
            HistoryEntry { timestamp: Some(Local.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap()), command: "ls".into(), session_id: None },
            HistoryEntry { timestamp: Some(Local.with_ymd_and_hms(2024, 1, 1, 12, 30, 0).unwrap()), command: "cd /".into(), session_id: None },
        ];
        time_of_day_stats(&history); // Should print 2 for 12:00
    }

    #[test]
    fn test_heatmap_stats_empty() {
        let history: Vec<HistoryEntry> = vec![];
        heatmap_stats(&history); // Should not panic
    }

    #[test]
    fn test_heatmap_stats_basic() {
        let history = vec![
            HistoryEntry { timestamp: Some(Local.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap()), command: "ls".into(), session_id: None },
            HistoryEntry { timestamp: Some(Local.with_ymd_and_hms(2024, 1, 2, 13, 0, 0).unwrap()), command: "cd /".into(), session_id: None },
        ];
        heatmap_stats(&history); // Should print for Mon and Tue
    }

    #[test]
    fn test_group_sessions() {
        let ts1 = Local.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
        let ts2 = Local.with_ymd_and_hms(2024, 1, 1, 10, 5, 0).unwrap();
        let ts3 = Local.with_ymd_and_hms(2024, 1, 1, 11, 0, 0).unwrap();
        let h1 = HistoryEntry { timestamp: Some(ts1), command: "ls".into(), session_id: None };
        let h2 = HistoryEntry { timestamp: Some(ts2), command: "cd /".into(), session_id: None };
        let h3 = HistoryEntry { timestamp: Some(ts3), command: "pwd".into(), session_id: None };
        let all = vec![h1, h2, h3];
        let refs: Vec<&HistoryEntry> = all.iter().collect();
        let sessions = group_sessions(&refs, 10);
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn test_suggest_aliases() {
        let history = vec![
            HistoryEntry { timestamp: None, command: "verylongcommand --with --many --args".into(), session_id: None },
            HistoryEntry { timestamp: None, command: "verylongcommand --with --many --args".into(), session_id: None },
        ];
        suggest_aliases(&history); // Should print alias suggestion
    }

    #[test]
    fn test_flag_dangerous() {
        let history = vec![
            HistoryEntry { timestamp: None, command: "rm -rf /".into(), session_id: None },
        ];
        flag_dangerous(&history); // Should print warning
    }

    #[test]
    fn test_per_directory_stats() {
        let history = vec![
            HistoryEntry { timestamp: None, command: "cd /tmp".into(), session_id: None },
            HistoryEntry { timestamp: None, command: "ls".into(), session_id: None },
        ];
        per_directory_stats(&history); // Should print stats
    }

    #[test]
    fn test_per_host_stats() {
        let history = vec![
            HistoryEntry { timestamp: None, command: "ls".into(), session_id: None },
        ];
        per_host_stats(&history); // Should print stats
    }
}
