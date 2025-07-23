//! TUI rendering module using ratatui + crossterm

use crate::cli::CliArgs;
use crate::models::HistoryEntry;
use anyhow::Result;
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets::*};
use std::fs::OpenOptions;
use std::io::{self, Write as IoWrite};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use regex::Regex;

#[derive(Copy, Clone, PartialEq)]
enum Tab {
    Summary,
    PerCommand,
    Sessions,
    Search,
    Aliases,
    Dangerous,
    Directory,
    Host,
    TimeOfDay,
    Heatmap,
}

#[derive(Copy, Clone, PartialEq)]
enum KeyMode {
    Default,
    Vim,
    Emacs,
}

#[derive(Copy, Clone, PartialEq)]
enum Theme {
    Default,
    HighContrast,
    Colorblind,
}

const TAB_ICONS: [&str; 10] = [
    " Summary",      // Dashboard
    " Commands",     // Terminal
    " Sessions",     // Calendar
    " Search",       // Search
    " Aliases",      // Tag
    " Dangerous",    // Warning
    " Directory",    // Folder
    " Host",         // Server
    " TimeOfDay",    // Clock
    " Heatmap",      // Chart
];

macro_rules! log_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        eprintln!("[heist error] {}", msg);
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("heist_error.log") {
            let _ = writeln!(f, "{}", msg);
        }
    }};
}

pub fn run_tui(history: &Vec<HistoryEntry>, _args: &CliArgs) -> Result<()> {
    // Replace get_history_path and load_history_from_file with correct parser logic
    let shell = crate::parser::detect_shell();
    let args = _args.clone();
    let history_data = Arc::new(Mutex::new(history.clone()));
    let history_data_clone = Arc::clone(&history_data);
    thread::spawn(move || {
        loop {
            let mut new_history = match crate::parser::parse_history(&shell, &args) {
                Ok(h) => h,
                Err(_) => Vec::new(),
            };
            let live_entries = crate::parser::parse_heist_live_history();
            new_history.extend(live_entries);
            let mut data = history_data_clone.lock().unwrap();
            *data = new_history;
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    let mut stdout = io::stdout();
    if let Err(e) = terminal::enable_raw_mode() {
        log_error!("Failed to enable raw mode: {}", e);
        return Err(e.into());
    }
    execute!(stdout, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(e) => {
            log_error!("Failed to create terminal: {}", e);
            return Err(e.into());
        }
    };

    let mut selected: usize = 0;
    let total = history.len();
    let mut running = true;
    let mut tab = Tab::Summary;
    let mut key_mode = KeyMode::Default;
    let mut theme = Theme::Default;
    let tab_titles: Vec<String> = TAB_ICONS.iter().map(|s| s.to_string()).collect();
    let help_text = String::from("[←/→] Switch Tab  [↑/↓] Scroll  [Enter] Select  [q/Ctrl+C] Quit | [/] Search | [Esc] Back | [F2] KeyMode | [F3] Theme");

    // --- Sessions grouping ---
    let mut sessions: Vec<(DateTime<Local>, DateTime<Local>, Vec<&HistoryEntry>)> = vec![];
    if !history.is_empty() {
        let mut current: Vec<&HistoryEntry> = vec![];
        let mut last_ts: Option<DateTime<Local>> = None;
        for entry in history {
            if let Some(ts) = entry.timestamp {
                if let Some(last) = last_ts {
                    if ts.signed_duration_since(last).num_minutes() > 10 {
                        if !current.is_empty() {
                            let start = current.first().unwrap().timestamp.unwrap();
                            let end = current.last().unwrap().timestamp.unwrap();
                            sessions.push((start, end, current));
                            current = vec![];
                        }
                    }
                }
                last_ts = Some(ts);
            }
            current.push(entry);
        }
        if !current.is_empty() {
            let start = current.first().unwrap().timestamp.unwrap();
            let end = current.last().unwrap().timestamp.unwrap();
            sessions.push((start, end, current));
        }
    }
    let mut session_selected: usize = 0;
    let mut session_cmd_selected: usize = 0;

    // --- Search state ---
    let mut search_mode = false;
    let mut search_query = String::new();
    let mut search_results: Vec<HistoryEntry> = vec![];
    let mut search_selected: usize = 0;

    // Cache summary data to avoid flicker
    let freq_vec: Vec<(String, usize)> = {
        use std::collections::HashMap;
        let mut freq: HashMap<String, usize> = HashMap::new();
        for entry in history {
            let cmd = entry.command.split_whitespace().next().unwrap_or("").to_string();
            *freq.entry(cmd).or_insert(0) += 1;
        }
        let mut freq_vec: Vec<_> = freq.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        freq_vec
    };
    // Cache alias suggestions to avoid flicker
    let alias_vec: Vec<(String, usize)> = {
        use std::collections::HashMap;
        let mut freq: HashMap<String, usize> = HashMap::new();
        for entry in history {
            let cmd = entry.command.trim().to_string();
            if cmd.len() > 15 {
                *freq.entry(cmd).or_insert(0) += 1;
            }
        }
        let mut alias_vec: Vec<_> = freq.into_iter().collect();
        alias_vec.sort_by(|a, b| b.1.cmp(&a.1));
        alias_vec
    };
    let max_count = freq_vec.first().map(|x| x.1).unwrap_or(1);
    let total_cmds = history.len();

    while running {
        let history = history_data.lock().unwrap();
        if let Err(e) = terminal.draw(|f| {
            let size = f.area(); // .size() is deprecated
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(size);
            let tabs = Tabs::new(tab_titles.iter().map(|s| Line::from(s.as_str())).collect::<Vec<_>>())
                .select(tab as usize)
                .block(Block::default().borders(Borders::ALL).title(" Heist"))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(tabs, chunks[0]);
            match tab {
                Tab::Summary => {
                    // Modern summary: Table with bar visualization
                    let header = Row::new(vec!["#", "Command", "Count", "Usage"])
                        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    let rows: Vec<Row> = freq_vec.iter().take(10).enumerate().map(|(i, (cmd, count))| {
                        let bar_len = ((*count as f64 / max_count as f64) * 20.0).round() as usize;
                        let bar = "█".repeat(bar_len);
                        Row::new(vec![
                            format!("{:>2}", i+1),
                            format!("{:<15}", cmd),
                            format!("{:>4}", count),
                            bar,
                        ]).style(Style::default().fg(Color::Green))
                    }).collect();
                    let table = Table::new(
                        rows,
                        [
                            Constraint::Length(3),
                            Constraint::Length(16),
                            Constraint::Length(6),
                            Constraint::Min(10),
                        ]
                    )
                        .header(header)
                        .block(Block::default().title("Top Commands ").borders(Borders::ALL).title_alignment(Alignment::Center))
                        .column_spacing(1)
                        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)); // highlight_style -> row_highlight_style
                    f.render_widget(table, chunks[1]);
                    // Subtitle with total commands
                    let subtitle = Paragraph::new(format!("Total commands: {}", total_cmds))
                        .style(Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC));
                    f.render_widget(subtitle, Rect {
                        x: chunks[1].x,
                        y: chunks[1].y + chunks[1].height.saturating_sub(2),
                        width: chunks[1].width,
                        height: 1,
                    });
                },
                Tab::PerCommand => {
                    let area_height = chunks[1].height as usize;
                    let visible_count = area_height.min(total);
                    // Ensure selected is always in bounds
                    if selected >= total && total > 0 {
                        selected = total - 1;
                    }
                    // Calculate scroll offset so selected is always visible
                    let mut scroll = 0;
                    if selected + 1 > visible_count {
                        scroll = selected + 1 - visible_count;
                    }
                    let items: Vec<ListItem> = history
                        .iter()
                        .skip(scroll)
                        .take(visible_count)
                        .map(|entry| ListItem::new(format!(" {}", entry.command)).style(Style::default().fg(Color::Gray)))
                        .collect();
                    let mut list = List::new(items)
                        .block(Block::default().title("All Commands ").borders(Borders::ALL))
                        .highlight_symbol("→ ");
                    // Set highlight to the correct relative index
                    let highlight_idx = selected.saturating_sub(scroll);
                    list = list.highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD));
                    let mut state = ListState::default();
                    state.select(Some(highlight_idx));
                    f.render_stateful_widget(list, chunks[1], &mut state);
                },
                Tab::Sessions => {
                    let area = chunks[1];
                    let (left, right) = if area.width > 60 {
                        (Rect { x: area.x, y: area.y, width: area.width/2, height: area.height },
                         Rect { x: area.x+area.width/2, y: area.y, width: area.width-area.width/2, height: area.height })
                    } else {
                        (area, Rect { x: 0, y: 0, width: 0, height: 0 })
                    };
                    // Session list
                    let session_items: Vec<ListItem> = sessions.iter().enumerate().map(|(i, (start, end, cmds))| {
                        ListItem::new(format!("Session {:>2}: {} - {} ({} cmds)", i+1, start.format("%Y-%m-%d %H:%M"), end.format("%H:%M"), cmds.len()))
                    }).collect();
                    let session_list = List::new(session_items)
                        .block(Block::default().title("Sessions").borders(Borders::ALL))
                        .highlight_symbol("→ ")
                        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD));
                    let mut session_state = ListState::default();
                    session_state.select(Some(session_selected));
                    f.render_stateful_widget(session_list, left, &mut session_state);
                    // Session detail
                    if !sessions.is_empty() && right.width > 0 {
                        let (_, _, cmds) = &sessions[session_selected];
                        let cmd_items: Vec<ListItem> = cmds.iter().map(|e| ListItem::new(format!("{}", e.command))).collect();
                        let cmd_list = List::new(cmd_items)
                            .block(Block::default().title("Commands").borders(Borders::ALL))
                            .highlight_symbol("→ ")
                            .highlight_style(Style::default().bg(Color::Magenta).fg(Color::White).add_modifier(Modifier::BOLD));
                        let mut cmd_state = ListState::default();
                        cmd_state.select(Some(session_cmd_selected.min(cmds.len().saturating_sub(1))));
                        f.render_stateful_widget(cmd_list, right, &mut cmd_state);
                    }
                },
                Tab::Search => {
                    let area = chunks[1];
                    let input = Paragraph::new(format!("Search: {}", search_query))
                        .block(Block::default().title("Search").borders(Borders::ALL))
                        .style(Style::default().fg(Color::Yellow));
                    f.render_widget(input, Rect { x: area.x, y: area.y, width: area.width, height: 3 });
                    let results_area = Rect { x: area.x, y: area.y+3, width: area.width, height: area.height.saturating_sub(3) };
                    let items: Vec<ListItem> = search_results.iter().map(|e| {
                        let mut styled = e.command.clone();
                        if !search_query.is_empty() {
                            if let Ok(re) = Regex::new(&search_query) {
                                styled = re.replace_all(&styled, |caps: &regex::Captures| format!("{{{}}}", &caps[0])).to_string();
                            } else if let Some(idx) = styled.find(&search_query) {
                                styled.replace_range(idx..idx+search_query.len(), &format!("{{{}}}", &search_query));
                            }
                        }
                        ListItem::new(styled)
                    }).collect();
                    let mut list = List::new(items)
                        .block(Block::default().title("Results").borders(Borders::ALL))
                        .highlight_symbol("→ ")
                        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD));
                    let mut state = ListState::default();
                    state.select(Some(search_selected));
                    f.render_stateful_widget(list, results_area, &mut state);
                },
                Tab::Aliases => {
                    let rows: Vec<Row> = alias_vec.iter().take(10).enumerate().map(|(i, (cmd, count))| {
                        Row::new(vec![
                            format!(" a{}", i+1),
                            format!("{}", cmd),
                            format!("{}", count),
                        ]).style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                    }).collect();
                    let table = Table::new(rows, [Constraint::Length(6), Constraint::Min(30), Constraint::Length(6)])
                        .header(Row::new(vec!["Alias", "Command", "Count"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)))
                        .block(Block::default().title("Alias Suggestions ").borders(Borders::ALL).title_alignment(Alignment::Center))
                        .column_spacing(1)
                        .row_highlight_style(Style::default().bg(Color::Rgb(255,0,128)).fg(Color::White).add_modifier(Modifier::BOLD | Modifier::ITALIC)); // highlight_style -> row_highlight_style
                    f.render_widget(table, chunks[1]);
                },
                Tab::Dangerous => {
                    let patterns = ["rm -rf", "rm -r /", "dd if=", "mkfs", ":(){ :|:& };:", "shutdown", "reboot", "curl | sh", "wget | sh", "chmod 777 /", "chown root", "> /dev/sda", "/dev/sda", ":(){ :|: & };:", "rm -rf --no-preserve-root", "poweroff", "halt", "init 0", "mkfs.ext", "dd of=/dev/", "mv /", "cp /dev/null", "yes | rm", "yes | dd", "yes | mkfs"];
                    let mut items: Vec<ListItem> = vec![];
                    for entry in history.iter() {
                        for pat in &patterns {
                            if entry.command.contains(pat) {
                                items.push(ListItem::new(format!("⚠️  {} (pattern: '{}')", entry.command, pat)).style(Style::default().fg(Color::Red)));
                                break;
                            }
                        }
                    }
                    if items.is_empty() {
                        items.push(ListItem::new("No dangerous commands found.").style(Style::default().fg(Color::Green)));
                    }
                    let list = List::new(items).block(Block::default().title("Dangerous Commands ").borders(Borders::ALL));
                    f.render_widget(list, chunks[1]);
                },
                Tab::Directory => {
                    use std::collections::HashMap;
                    let mut dir_counts: HashMap<String, usize> = HashMap::new();
                    let mut last_dir = String::from("~");
                    for entry in history.iter() {
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
                    let rows: Vec<Row> = dir_vec.iter().take(15).map(|(dir, count)| Row::new(vec![dir.clone(), count.to_string()])).collect();
                    let table = Table::new(rows, [Constraint::Min(30), Constraint::Length(6)])
                        .header(Row::new(vec!["Directory", "Count"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
                        .block(Block::default().title("Per-Directory Stats ").borders(Borders::ALL).title_alignment(Alignment::Center));
                    f.render_widget(table, chunks[1]);
                },
                Tab::Host => {
                    use std::collections::HashMap;
                    use std::env;
                    let mut host_counts: HashMap<String, usize> = HashMap::new();
                    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
                    for _ in history.iter() {
                        *host_counts.entry(hostname.clone()).or_insert(0) += 1;
                    }
                    let rows: Vec<Row> = host_counts.iter().map(|(host, count)| Row::new(vec![host.clone(), count.to_string()])).collect();
                    let table = Table::new(rows, [Constraint::Min(20), Constraint::Length(6)])
                        .header(Row::new(vec!["Host", "Count"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
                        .block(Block::default().title("Per-Host Stats ").borders(Borders::ALL).title_alignment(Alignment::Center));
                    f.render_widget(table, chunks[1]);
                },
                Tab::TimeOfDay => {
                    use chrono::Timelike;
                    let mut hours = [0usize; 24];
                    for entry in history.iter() {
                        if let Some(ts) = entry.timestamp {
                            hours[ts.hour() as usize] += 1;
                        }
                    }
                    let rows: Vec<Row> = (0..24).map(|h| {
                        let count = hours[h];
                        let bar = "#".repeat(count / 2.max(1));
                        Row::new(vec![format!("{:02}:00", h), count.to_string(), bar])
                    }).collect();
                    let table = Table::new(rows, [Constraint::Length(7), Constraint::Length(6), Constraint::Min(10)])
                        .header(Row::new(vec!["Hour", "Count", "Bar"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
                        .block(Block::default().title("Time-of-Day Stats ").borders(Borders::ALL).title_alignment(Alignment::Center));
                    f.render_widget(table, chunks[1]);
                },
                Tab::Heatmap => {
                    use chrono::{Datelike, Timelike};
                    let mut heatmap = [[0usize; 24]; 7];
                    for entry in history.iter() {
                        if let Some(ts) = entry.timestamp {
                            let wd = ts.weekday().num_days_from_monday() as usize;
                            let hr = ts.hour() as usize;
                            heatmap[wd][hr] += 1;
                        }
                    }
                    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
                    let mut rows: Vec<Row> = vec![];
                    for (d, row) in heatmap.iter().enumerate() {
                        let mut cells = vec![days[d].to_string()];
                        for &count in row {
                            let c = match count {
                                0 => " ",
                                1..=2 => ".",
                                3..=5 => "*",
                                6..=10 => "o",
                                _ => "#",
                            };
                            cells.push(c.to_string());
                        }
                        rows.push(Row::new(cells));
                    }
                    let mut header_cells = vec!["Day".to_string()];
                    for h in 0..24 { header_cells.push(format!("{:02}", h)); }
                    let table = Table::new(rows, vec![Constraint::Length(4)].into_iter().chain(std::iter::repeat(Constraint::Length(2)).take(24)).collect::<Vec<_>>())
                        .header(Row::new(header_cells).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
                        .block(Block::default().title("Weekly Heatmap ").borders(Borders::ALL).title_alignment(Alignment::Center));
                    f.render_widget(table, chunks[1]);
                },
            }
            // Show key mode in help bar
            let mode_str = match key_mode {
                KeyMode::Default => "Default",
                KeyMode::Vim => "Vim",
                KeyMode::Emacs => "Emacs",
            };
            // Show theme in help bar
            let theme_str = match theme {
                Theme::Default => "Default",
                Theme::HighContrast => "HighContrast",
                Theme::Colorblind => "Colorblind",
            };
            let help_string = format!("{} | Mode: {} | Theme: {}", help_text, mode_str, theme_str);
            let help: &str = if search_mode { "Type to search, [Esc] to exit search, [Enter] to select" } else { &help_string };
            // Render help bar
            let help_bar = Paragraph::new(help);
            f.render_widget(help_bar, chunks[2]);
        }) {
            log_error!("TUI draw error: {}", e);
        }
        // --- Input handling ---
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(key) => {
                    if key.code == event::KeyCode::F(2) {
                        key_mode = match key_mode {
                            KeyMode::Default => KeyMode::Vim,
                            KeyMode::Vim => KeyMode::Emacs,
                            KeyMode::Emacs => KeyMode::Default,
                        };
                        continue;
                    }
                    if key.code == event::KeyCode::F(3) {
                        theme = match theme {
                            Theme::Default => Theme::HighContrast,
                            Theme::HighContrast => Theme::Colorblind,
                            Theme::Colorblind => Theme::Default,
                        };
                        continue;
                    }
                    if search_mode {
                        match key_mode {
                            KeyMode::Vim => match key.code {
                                event::KeyCode::Char('j') => { if search_selected + 1 < search_results.len() { search_selected += 1; } },
                                event::KeyCode::Char('k') => { if search_selected > 0 { search_selected -= 1; } },
                                event::KeyCode::Esc => { search_mode = false; search_query.clear(); search_results.clear(); },
                                event::KeyCode::Char(c) => { search_query.push(c); },
                                event::KeyCode::Backspace => { search_query.pop(); },
                                _ => {}
                            },
                            KeyMode::Emacs => match key.code {
                                event::KeyCode::Char('n') if key.modifiers.contains(event::KeyModifiers::CONTROL) => { if search_selected + 1 < search_results.len() { search_selected += 1; } },
                                event::KeyCode::Char('p') if key.modifiers.contains(event::KeyModifiers::CONTROL) => { if search_selected > 0 { search_selected -= 1; } },
                                event::KeyCode::Esc => { search_mode = false; search_query.clear(); search_results.clear(); },
                                event::KeyCode::Char(c) => { search_query.push(c); },
                                event::KeyCode::Backspace => { search_query.pop(); },
                                _ => {}
                            },
                            _ => match key.code {
                                event::KeyCode::Esc => { search_mode = false; search_query.clear(); search_results.clear(); },
                                event::KeyCode::Char(c) => { search_query.push(c); },
                                event::KeyCode::Backspace => { search_query.pop(); },
                                event::KeyCode::Down => { if search_selected + 1 < search_results.len() { search_selected += 1; } },
                                event::KeyCode::Up => { if search_selected > 0 { search_selected -= 1; } },
                                _ => {}
                            }
                        }
                        // Update search results
                        let search_vec: Vec<HistoryEntry> = if !search_query.is_empty() {
                            if let Ok(re) = Regex::new(&search_query) {
                                history.iter().filter(|e| re.is_match(&e.command)).cloned().collect()
                            } else {
                                history.iter().filter(|e| e.command.contains(&search_query)).cloned().collect()
                            }
                        } else { vec![] };
                        search_results = search_vec;
                        if search_selected >= search_results.len() { search_selected = 0; }
                        continue;
                    }
                    match key_mode {
                        KeyMode::Vim => match key.code {
                            event::KeyCode::Char('h') => {
                                tab = match tab {
                                    Tab::Summary => Tab::Search,
                                    Tab::PerCommand => Tab::Summary,
                                    Tab::Sessions => Tab::PerCommand,
                                    Tab::Search => Tab::Sessions,
                                    Tab::Aliases => Tab::Summary,
                                    Tab::Dangerous => Tab::Aliases,
                                    Tab::Directory => Tab::Dangerous,
                                    Tab::Host => Tab::Directory,
                                    Tab::TimeOfDay => Tab::Host,
                                    Tab::Heatmap => Tab::TimeOfDay,
                                };
                                selected = 0; session_selected = 0; session_cmd_selected = 0; search_selected = 0;
                            },
                            event::KeyCode::Char('l') => {
                                tab = match tab {
                                    Tab::Summary => Tab::PerCommand,
                                    Tab::PerCommand => Tab::Sessions,
                                    Tab::Sessions => Tab::Search,
                                    Tab::Search => Tab::Aliases,
                                    Tab::Aliases => Tab::Dangerous,
                                    Tab::Dangerous => Tab::Directory,
                                    Tab::Directory => Tab::Host,
                                    Tab::Host => Tab::TimeOfDay,
                                    Tab::TimeOfDay => Tab::Heatmap,
                                    Tab::Heatmap => Tab::Summary,
                                };
                                selected = 0; session_selected = 0; session_cmd_selected = 0; search_selected = 0;
                            },
                            event::KeyCode::Char('j') => {
                                match tab {
                                    Tab::PerCommand => if selected + 1 < total { selected += 1; },
                                    Tab::Sessions => if session_selected + 1 < sessions.len() { session_selected += 1; session_cmd_selected = 0; },
                                    Tab::Search => if search_selected + 1 < search_results.len() { search_selected += 1; },
                                    _ => {}
                                }
                            },
                            event::KeyCode::Char('k') => {
                                match tab {
                                    Tab::PerCommand => if selected > 0 { selected -= 1; },
                                    Tab::Sessions => if session_selected > 0 { session_selected -= 1; session_cmd_selected = 0; },
                                    Tab::Search => if search_selected > 0 { search_selected -= 1; },
                                    _ => {}
                                }
                            },
                            event::KeyCode::Char('q') | event::KeyCode::Esc => running = false,
                            event::KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => running = false,
                            event::KeyCode::Enter => {
                                if tab == Tab::Sessions && !sessions.is_empty() && !sessions[session_selected].2.is_empty() {
                                    session_cmd_selected = (session_cmd_selected + 1) % sessions[session_selected].2.len();
                                }
                            },
                            _ => {}
                        },
                        KeyMode::Emacs => match key.code {
                            event::KeyCode::Char('a') if key.modifiers.contains(event::KeyModifiers::CONTROL) => { selected = 0; },
                            event::KeyCode::Char('e') if key.modifiers.contains(event::KeyModifiers::CONTROL) => { selected = total.saturating_sub(1); },
                            event::KeyCode::Char('n') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                                match tab {
                                    Tab::PerCommand => if selected + 1 < total { selected += 1; },
                                    Tab::Sessions => if session_selected + 1 < sessions.len() { session_selected += 1; session_cmd_selected = 0; },
                                    Tab::Search => if search_selected + 1 < search_results.len() { search_selected += 1; },
                                    _ => {}
                                }
                            },
                            event::KeyCode::Char('p') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                                match tab {
                                    Tab::PerCommand => if selected > 0 { selected -= 1; },
                                    Tab::Sessions => if session_selected > 0 { session_selected -= 1; session_cmd_selected = 0; },
                                    Tab::Search => if search_selected > 0 { search_selected -= 1; },
                                    _ => {}
                                }
                            },
                            event::KeyCode::Char('q') | event::KeyCode::Esc => running = false,
                            event::KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => running = false,
                            event::KeyCode::Enter => {
                                if tab == Tab::Sessions && !sessions.is_empty() && !sessions[session_selected].2.is_empty() {
                                    session_cmd_selected = (session_cmd_selected + 1) % sessions[session_selected].2.len();
                                }
                            },
                            _ => {}
                        },
                        _ => match key.code {
                            event::KeyCode::Char('/') => { search_mode = true; search_query.clear(); search_results.clear(); search_selected = 0; },
                            event::KeyCode::Char('q') | event::KeyCode::Esc => running = false,
                            event::KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => running = false,
                            event::KeyCode::Down => {
                                match tab {
                                    Tab::PerCommand => if selected + 1 < total { selected += 1; },
                                    Tab::Sessions => if session_selected + 1 < sessions.len() { session_selected += 1; session_cmd_selected = 0; },
                                    Tab::Search => if search_selected + 1 < search_results.len() { search_selected += 1; },
                                    _ => {}
                                }
                            },
                            event::KeyCode::Up => {
                                match tab {
                                    Tab::PerCommand => if selected > 0 { selected -= 1; },
                                    Tab::Sessions => if session_selected > 0 { session_selected -= 1; session_cmd_selected = 0; },
                                    Tab::Search => if search_selected > 0 { search_selected -= 1; },
                                    _ => {}
                                }
                            },
                            event::KeyCode::Right => {
                                tab = match tab {
                                    Tab::Summary => Tab::PerCommand,
                                    Tab::PerCommand => Tab::Sessions,
                                    Tab::Sessions => Tab::Search,
                                    Tab::Search => Tab::Aliases,
                                    Tab::Aliases => Tab::Dangerous,
                                    Tab::Dangerous => Tab::Directory,
                                    Tab::Directory => Tab::Host,
                                    Tab::Host => Tab::TimeOfDay,
                                    Tab::TimeOfDay => Tab::Heatmap,
                                    Tab::Heatmap => Tab::Summary,
                                };
                                selected = 0; session_selected = 0; session_cmd_selected = 0; search_selected = 0;
                            },
                            event::KeyCode::Left => {
                                tab = match tab {
                                    Tab::Summary => Tab::Search,
                                    Tab::PerCommand => Tab::Summary,
                                    Tab::Sessions => Tab::PerCommand,
                                    Tab::Search => Tab::Sessions,
                                    Tab::Aliases => Tab::Summary,
                                    Tab::Dangerous => Tab::Aliases,
                                    Tab::Directory => Tab::Dangerous,
                                    Tab::Host => Tab::Directory,
                                    Tab::TimeOfDay => Tab::Host,
                                    Tab::Heatmap => Tab::TimeOfDay,
                                };
                                selected = 0; session_selected = 0; session_cmd_selected = 0; search_selected = 0;
                            },
                            event::KeyCode::Enter => {
                                if tab == Tab::Sessions && !sessions.is_empty() && !sessions[session_selected].2.is_empty() {
                                    session_cmd_selected = (session_cmd_selected + 1) % sessions[session_selected].2.len();
                                }
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
    }
    if let Err(e) = terminal::disable_raw_mode() {
        log_error!("Failed to disable raw mode: {}", e);
    }
    if let Err(e) = execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, event::DisableMouseCapture) {
        log_error!("Failed to leave alternate screen: {}", e);
    }
    Ok(())
}
