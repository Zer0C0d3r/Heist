//! TUI rendering module using ratatui + crossterm

use crate::cli::CliArgs;
use crate::models::HistoryEntry;
use anyhow::Result;
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets::*};
use std::io::{self};
use std::time::Duration;
use chrono::{DateTime, Local};
use regex::Regex;

#[derive(Copy, Clone, PartialEq)]
enum Tab {
    Summary,
    PerCommand,
    Sessions,
    Search,
}

const TAB_ICONS: [&str; 4] = [
    " Summary",      // Nerd Font: nf-md-view_dashboard
    " Commands",     // nf-fa-terminal
    " Sessions",     // nf-md-calendar_clock
    " Search",       // nf-fa-search
];

pub fn run_tui(history: &Vec<HistoryEntry>, _args: &CliArgs) -> Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected: usize = 0;
    let total = history.len();
    let mut running = true;
    let mut tab = Tab::Summary;
    let tab_titles: Vec<String> = TAB_ICONS.iter().map(|s| s.to_string()).collect();
    let help_text = "[←/→] Switch Tab  [↑/↓] Scroll  [Enter] Select  [q/Ctrl+C] Quit | [/] Search | [Esc] Back";

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
    let mut search_results: Vec<&HistoryEntry> = vec![];
    let mut search_selected: usize = 0;

    // Cache summary data to avoid flicker
    let mut freq_vec: Vec<(String, usize)> = {
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
    let max_count = freq_vec.first().map(|x| x.1).unwrap_or(1);
    let total_cmds = history.len();

    while running {
        terminal.draw(|f| {
            let size = f.size();
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
                        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD));
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
                    let mut session_list = List::new(session_items)
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
                        let mut cmd_list = List::new(cmd_items)
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
            }
            let help = Paragraph::new(if search_mode {"Type to search, [Esc] to exit search, [Enter] to select"} else {help_text})
                .style(Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC));
            f.render_widget(help, chunks[2]);
        })?;
        // --- Input handling ---
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(key) => {
                    if search_mode {
                        match key.code {
                            event::KeyCode::Esc => { search_mode = false; search_query.clear(); search_results.clear(); },
                            event::KeyCode::Char(c) => { search_query.push(c); },
                            event::KeyCode::Backspace => { search_query.pop(); },
                            event::KeyCode::Down => { if search_selected + 1 < search_results.len() { search_selected += 1; } },
                            event::KeyCode::Up => { if search_selected > 0 { search_selected -= 1; } },
                            _ => {}
                        }
                        // Update search results
                        search_results = if !search_query.is_empty() {
                            if let Ok(re) = Regex::new(&search_query) {
                                history.iter().filter(|e| re.is_match(&e.command)).collect()
                            } else {
                                history.iter().filter(|e| e.command.contains(&search_query)).collect()
                            }
                        } else { vec![] };
                        if search_selected >= search_results.len() { search_selected = 0; }
                        continue;
                    }
                    match key.code {
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
                                Tab::Search => Tab::Summary,
                            };
                            selected = 0; session_selected = 0; session_cmd_selected = 0; search_selected = 0;
                        },
                        event::KeyCode::Left => {
                            tab = match tab {
                                Tab::Summary => Tab::Search,
                                Tab::PerCommand => Tab::Summary,
                                Tab::Sessions => Tab::PerCommand,
                                Tab::Search => Tab::Sessions,
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
                },
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, event::DisableMouseCapture)?;
    Ok(())
}
