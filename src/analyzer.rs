//! Analytics and stats functions for shell history

use crate::cli::CliArgs;
use crate::models::HistoryEntry;
use anyhow::Result;
use std::collections::HashMap;

/// Analyze history and print stats in CLI mode
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
        let re = Regex::new(pat)?;
        filtered.retain(|e| re.is_match(&e.command));
    }
    // --range "YYYY-MM-DD:YYYY-MM-DD"
    if let Some(ref range) = args.range {
        let parts: Vec<_> = range.split(':').collect();
        if parts.len() == 2 {
            let start = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d")?;
            let end = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d")?;
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
    // --top N
    if let Some(top_n) = args.top {
        use std::collections::HashMap;
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
        // Simple session grouping: 10 min gap
        let mut sessions = vec![];
        let mut current: Vec<&HistoryEntry> = vec![];
        let mut last_ts = None;
        for entry in &filtered {
            if let Some(ts) = entry.timestamp {
                if let Some(last) = last_ts {
                    if ts.signed_duration_since(last).num_minutes() > 10 {
                        if !current.is_empty() {
                            sessions.push(current);
                            current = vec![];
                        }
                    }
                }
                last_ts = Some(ts);
            }
            current.push(entry);
        }
        if !current.is_empty() {
            sessions.push(current);
        }
        println!("Total sessions: {}", sessions.len());
        let avg_len = sessions.iter().map(|s| s.len()).sum::<usize>() as f64 / sessions.len() as f64;
        println!("Average session length: {:.2} commands", avg_len);
        return Ok(());
    }
    // --export <format>
    if let Some(ref fmt) = args.export {
        match fmt.as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&filtered)?;
                let mut f = File::create("heist_export.json")?;
                f.write_all(json.as_bytes())?;
                println!("Exported to heist_export.json");
            },
            "csv" => {
                let mut f = File::create("heist_export.csv")?;
                writeln!(f, "timestamp,command")?;
                for e in &filtered {
                    let ts = e.timestamp.map(|t| t.to_rfc3339()).unwrap_or_default();
                    writeln!(f, "{}\t{}", ts, e.command)?;
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
