# Heist

**Heist** is a blazing-fast, cross-platform shell history analyzer with interactive TUI and CLI modes. It supports Linux, macOS, BSD, and Termux, and can parse history from Bash, Zsh, Fish, Csh, Tcsh, Ksh, Dash, Sh, Mksh, Yash, Osh, and more.

---

## 🌟 About & Philosophy

Heist is designed for:

- **Everyday users** who want to see and search their shell history in a beautiful, interactive way.
- **Power users** who need advanced analytics, regex search, and export features.
- **Developers** who want a modular, hackable Rust codebase for shell analytics.

**Why Heist?**

- Most shell history tools are either too simple or too complex. Heist aims for a sweet spot: fast, beautiful, and powerful, but easy to use and extend.
- TUI and CLI are first-class citizens: use Heist as a daily dashboard or a quick command-line tool.
- Designed for privacy: all analysis is local, no cloud sync by default.

---

## 🚀 Features

- Interactive TUI with rich ASCII/Unicode graphics (ratatui, crossterm, Nerd Fonts)
- CLI mode for quick stats, search, and export
- Supports history formats for Bash, Zsh, Fish, Csh, Tcsh, Ksh, Dash, Sh, Mksh, Yash, Osh
- Modular, extensible, and performant Rust codebase
- Export to CSV/JSON, session analytics, regex search, and more
- Global installer/uninstaller script
- Session grouping, time-based analytics, and more

---

## 🧩 Shell History Format Details

- **Bash**: `~/.bash_history` (plain text)
- **Zsh**: `~/.zsh_history` (timestamps)
- **Fish**: `~/.local/share/fish/fish_history` (YAML-like)
- **Csh/Tcsh**: `~/.history` (plain text, tcsh may have timestamps)
- **Ksh**: `~/.sh_history` (plain text)
- **Dash/Sh**: typically use Bash format
- **Mksh**: `~/.mksh_history` (plain text)
- **Yash**: `~/.yash_history` (plain text)
- **Osh**: `~/.osh_history` (plain text)
- **Session grouping**: Commands are grouped if there is a gap of 10+ minutes between them.

---

## 🏗️ Architecture & Design

- **src/cli.rs**: CLI argument parsing (clap)
- **src/parser/**: Shell history parsing logic (modular for each shell)
- **src/ui/**: TUI rendering (ratatui, crossterm, Nerd Fonts)
- **src/analyzer.rs**: Analytics, stats, and CLI output
- **src/models.rs**: Data models for commands, sessions, and history entries
- **install.sh**: Interactive installer/uninstaller

**Extensibility:**

- Add new shell support by creating a new parser module
- Add new analytics by extending `analyzer.rs`
- Add new TUI tabs by editing `ui/mod.rs`

---

## 🛠️ Installation

### 1. Prerequisites

- **Rust** (install from [rustup.rs](https://rustup.rs/))
- **Nerd Font** (for best TUI experience)
- A POSIX-compliant shell (bash, zsh, fish, csh, tcsh, ksh, dash, sh, mksh, yash, osh)

### 2. Quick Install (Recommended)

```sh
# Clone the repo
git clone https://github.com/Zer0C0d3r/Heist.git
cd Heist
# Run the interactive installer
./install.sh
```

- To **uninstall**, run `./install.sh` and choose the uninstall option.

### 3. Manual Build

```sh
cargo build --release
sudo cp target/release/heist /usr/local/bin/
```

---

## 🧑‍💻 Usage

### TUI Mode (default)

```sh
heist
```

- Use arrow keys to navigate tabs and lists
- Tabs: Summary, Per-Command, Sessions, Search
- Quit: `q`, `Esc`, or `Ctrl+C`

### CLI Mode

```sh
heist --cli [flags]
```

#### Common CLI Flags

- `--top N` Show top N most used commands
- `--search <pattern>` Regex search for commands
- `--filter <command>` Only analyze specific commands
- `--range "YYYY-MM-DD:YYYY-MM-DD"` Filter by date range
- `--export <format>` Export data to CSV or JSON
- `--shell <bash|zsh|fish|csh|tcsh|ksh|dash|sh|mksh|yash|osh>` Force shell type
- `--session-summary` Print session-level stats

#### Examples

```sh
# Top 10 commands
heist --cli --top 10

# Search for all 'cargo build' commands
heist --cli --search 'cargo.*build'

# Only show 'ls' commands
heist --cli --filter ls

# Filter by date range
heist --cli --range "2025-07-01:2025-07-14"

# Export to JSON
heist --cli --export json

# Print session summary
heist --cli --session-summary
```

---

## 🖥️ TUI Features

- **Tabs**: Summary (top commands), Per-Command, Sessions, Search
- **Navigation**: `←/→` switch tabs, `↑/↓` scroll, `Enter` select, `q`/`Esc`/`Ctrl+C` quit
- **Colors & Icons**: Uses Nerd Font icons and color highlights for a modern look
- **Help Bar**: Always visible at the bottom
- **Performance**: Handles large history files efficiently

---

## 🐚 Supported Shells & History Files

- **Bash**: `~/.bash_history`
- **Zsh**: `~/.zsh_history`
- **Fish**: `~/.local/share/fish/fish_history`
- **Csh/Tcsh**: `~/.history`
- **Ksh**: `~/.sh_history`
- **Dash/Sh**: Bash format
- **Mksh**: `~/.mksh_history`
- **Yash**: `~/.yash_history`
- **Osh**: `~/.osh_history`
- Auto-detects shell, or use `--shell` to override

---

## 🧠 Analytics & Power Features

- Command frequency, ranking, and histograms
- Regex and keyword search
- Session grouping (10 min gap)
- Export to CSV/JSON
- Session stats: total sessions, average session length
- Alias suggestions, dangerous command detection, plugin/AI support (planned)

---

## 🛠️ Troubleshooting

- **TUI looks weird?**
  - Use a Nerd Font in your terminal
  - Try a different terminal emulator if colors/icons are missing
- **Permission denied?**
  - Use `sudo` for global install, or install to a user-writable directory
- **Shell history not detected?**
  - Use `--shell` to force the shell type
- **Rust not installed?**
  - Install from [rustup.rs](https://rustup.rs/)
- **Installer fails?**
  - Check for `sudo` and `cargo` in your PATH
  - Try manual build steps

---

## 👩‍💻 For Developers

- Modular code: see `src/cli.rs`, `src/parser/`, `src/ui/`, `src/analyzer.rs`, `src/models.rs`
- Add new analytics or UI tabs easily
- Run tests: `cargo test`
- Lint/fix: `cargo clippy`, `cargo fmt`
- Contribute via PRs and issues!
- See the TODO for ideas to contribute

---

## 📦 Project Structure

```
heist/
├── src/
│   ├── main.rs         # Entry point
│   ├── cli.rs          # CLI argument parsing
│   ├── parser/         # Shell history parsing
│   ├── ui/             # TUI rendering
│   ├── analyzer.rs     # Analytics/statistics
│   └── models.rs       # Data models
├── install.sh          # Interactive installer/uninstaller
├── Cargo.toml          # Rust dependencies
├── README.md           # This file
└── ...
```

---

## 📝 TODO (Long Term & Future)

- [ ] **Session detection improvements for all shells**
- [ ] **Alias suggestion engine**
- [ ] **Dangerous command flagging**
- [ ] **Time-of-day and heatmap analytics**
- [ ] **Per-directory and per-host stats**
- [ ] **Plugin system for custom analytics**
- [ ] **Live tracking via PROMPT_COMMAND**
- [ ] **Cloud sync (opt-in, privacy-first)**
- [ ] **SQLite backend for large history**
- [ ] **HTML dashboard export**
- [ ] **Config file support (TOML/YAML/JSON)**
- [ ] **Customizable TUI themes**
- [ ] **Vim/Emacs keybindings in TUI**
- [ ] **Accessibility improvements**
- [ ] **Internationalization (i18n)**
- [ ] **Better error messages and logging**
- [ ] **Performance benchmarks and tuning**
- [ ] **More shell support (tcsh, ksh, etc.)**
- [ ] **Unit and integration tests for all modules**
- [ ] **CI/CD improvements and release automation**
- [ ] **Better documentation and user guides**

---

## 📜 License

MIT
