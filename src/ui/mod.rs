//! TUI rendering module using ratatui + crossterm

use crate::cli::CliArgs;
use crate::models::HistoryEntry;
use anyhow::Result;
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets::*};
use std::io::{self};

enum Tab {
    Summary,
    PerCommand,
    Sessions,
    Search,
}

impl Copy for Tab {}
impl Clone for Tab {
    fn clone(&self) -> Self { *self }
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

    let mut scroll: usize = 0;
    let mut selected: usize = 0;
    let total = history.len();
    let mut running = true;
    let mut tab = Tab::Summary;
    let tab_titles: Vec<String> = TAB_ICONS.iter().map(|s| s.to_string()).collect();
    let help_text = "[←/→] Switch Tab  [↑/↓] Scroll  [Enter] Select  [q/Ctrl+C] Quit";

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
                    use std::collections::HashMap;
                    let mut freq: HashMap<&str, usize> = HashMap::new();
                    for entry in history {
                        let cmd = entry.command.split_whitespace().next().unwrap_or("");
                        *freq.entry(cmd).or_insert(0) += 1;
                    }
                    let mut freq_vec: Vec<_> = freq.into_iter().collect();
                    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
                    let items: Vec<ListItem> = freq_vec.iter().take(10)
                        .map(|(cmd, count)| ListItem::new(format!("󰄾 {:<15} | {:>4}", cmd, count)).style(Style::default().fg(Color::Green)))
                        .collect();
                    let list = List::new(items)
                        .block(Block::default().title("Top Commands ").borders(Borders::ALL).title_alignment(Alignment::Center))
                        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
                        .highlight_symbol("→ ");
                    f.render_widget(list, chunks[1]);
                },
                Tab::PerCommand => {
                    let visible = (chunks[1].height as usize).min(total - scroll);
                    let items: Vec<ListItem> = history
                        .iter()
                        .skip(scroll)
                        .take(visible)
                        .enumerate()
                        .map(|(i, entry)| {
                            let style = if i == selected { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::Gray) };
                            ListItem::new(format!(" {}", entry.command)).style(style)
                        })
                        .collect();
                    let list = List::new(items)
                        .block(Block::default().title("All Commands ").borders(Borders::ALL))
                        .highlight_symbol("→ ");
                    f.render_widget(list, chunks[1]);
                },
                Tab::Sessions => {
                    let para = Paragraph::new("Session stats coming soon... ")
                        .block(Block::default().title("Sessions ").borders(Borders::ALL))
                        .style(Style::default().fg(Color::Magenta));
                    f.render_widget(para, chunks[1]);
                },
                Tab::Search => {
                    let para = Paragraph::new("Search coming soon... ")
                        .block(Block::default().title("Search ").borders(Borders::ALL))
                        .style(Style::default().fg(Color::Yellow));
                    f.render_widget(para, chunks[1]);
                },
            }
            let help = Paragraph::new(help_text)
                .style(Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC));
            f.render_widget(help, chunks[2]);
        })?;
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(key) => match key.code {
                    event::KeyCode::Char('q') | event::KeyCode::Esc => running = false,
                    event::KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => running = false,
                    event::KeyCode::Down => {
                        if scroll + 1 < total { scroll += 1; }
                        if selected + 1 < total { selected += 1; }
                    }
                    event::KeyCode::Up => {
                        if scroll > 0 { scroll -= 1; }
                        if selected > 0 { selected -= 1; }
                    }
                    event::KeyCode::Right => {
                        tab = match tab {
                            Tab::Summary => Tab::PerCommand,
                            Tab::PerCommand => Tab::Sessions,
                            Tab::Sessions => Tab::Search,
                            Tab::Search => Tab::Summary,
                        };
                        scroll = 0; selected = 0;
                    }
                    event::KeyCode::Left => {
                        tab = match tab {
                            Tab::Summary => Tab::Search,
                            Tab::PerCommand => Tab::Summary,
                            Tab::Sessions => Tab::PerCommand,
                            Tab::Search => Tab::Sessions,
                        };
                        scroll = 0; selected = 0;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, event::DisableMouseCapture)?;
    Ok(())
}
