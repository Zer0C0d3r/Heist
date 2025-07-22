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
- Alias suggestions, dangerous command detection
- **Time-of-day analytics:** Show hourly command usage with `--time-of-day`
- **Weekly heatmap analytics:** Show hour-by-day usage heatmap with `--heatmap`

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

- [x] **Session detection improvements for all shells** (timestamp inference for plain-text shells, robust grouping for all supported shells)
- [x] **Alias suggestion engine** (Suggests aliases for long/frequent commands via CLI)
- [x] **Dangerous command flagging** (Flags risky commands in history via CLI)
- [x] **Per-directory and per-host stats** (CLI: --per-directory, --per-host)
- [x] **Time-of-day and heatmap analytics**
- [x] **Live tracking via PROMPT_COMMAND** (real-time command logging for Bash/Zsh, see README)
- [x] **Vim/Emacs keybindings in TUI**
- [x] **Accessibility improvements**
- [x] **Better error messages and logging**
- [x] **Performance benchmarks and tuning**
- [x] **Unit and integration tests for all modules**
- [x] **CI/CD improvements and release automation** (Advanced GitHub Actions: build, test, changelog, and auto-release on tag)
- [ ] **Better documentation and user guides**

---

## 🟢 Live Tracking via PROMPT_COMMAND

Heist supports real-time command tracking for Bash and Zsh using PROMPT_COMMAND (or Zsh's `precmd`).

### How it works

- Each time you run a command, it is appended instantly to `~/.heist_live_history`.
- This enables up-to-the-second analytics and session tracking, even before your shell history is flushed.
- Heist automatically merges this live-tracked file with your main history for analytics.

### Setup

- Run the installer (`./install.sh`) and choose to enable live tracking when prompted.
- This will add a snippet to your `~/.bashrc` and/or `~/.zshrc`:

```sh
source /path/to/Heist/contrib/heist_live_tracking.sh
```

- Restart your shell to activate live tracking.

### Manual setup

If you want to add it manually, append this to your `~/.bashrc` or `~/.zshrc`:

```sh
source /path/to/Heist/contrib/heist_live_tracking.sh
```

---

## 🚦 Performance Benchmarking & Tuning

Heist is designed for speed and efficiency, even with massive shell histories (100k+ entries). Here’s how to benchmark and tune performance:

### Benchmarking

- **Run built-in Rust benchmarks:**

  ```sh
  cargo bench
  ```

- **Profile CLI analytics:**

  ```sh
  time heist --cli --top 100
  time heist --cli --heatmap
  time heist --cli --session-summary
  ```

- **Profile TUI startup:**

  ```sh
  time heist
  ```

- **Memory usage:**

  ```sh
  /usr/bin/time -v heist --cli --top 100
  ```

- **Large history stress test:**

  ```sh
  head -c 1000000 /dev/urandom | base64 > big_history.txt
  # (or concatenate your real histories)
  heist --cli --shell bash --range "2020-01-01:2025-12-31"
  ```

### Tuning

- **Release build:** Always use the release binary for best performance:

  ```sh
  cargo build --release
  ./target/release/heist
  ```

- **Parallelism:** Heist’s analytics are optimized for single-threaded speed, but future versions may add parallel parsing/analysis for huge files.
- **Memory:** Handles 100k+ entries on modest hardware. For extreme cases, filter by date or command to reduce working set.
- **TUI:** For best TUI performance, use a modern terminal emulator and a Nerd Font.
- **Shell history size:** Periodically archive old history if you notice slowdowns.

---

## 📚 Comprehensive User Guide

### 1. Quick Start

- **Install:**

  ```sh
  git clone https://github.com/Zer0C0d3r/Heist.git
  cd Heist
  ./install.sh
  ```

- **Run TUI:**

  ```sh
  heist
  ```

- **Run CLI analytics:**

  ```sh
  heist --cli --top 10
  heist --cli --heatmap
  heist --cli --session-summary
  ```

### 2. TUI Mode: Interactive Dashboard

- **Tabs:** Summary, Per-Command, Sessions, Search
- **Navigation:**
  - `←/→`: Switch tabs
  - `↑/↓`: Scroll lists
  - `Enter`: Select
  - `q`, `Esc`, `Ctrl+C`: Quit
- **Keybindings:**
  - **Default:** Arrow keys, Enter, q/Esc
  - **Vim:** `h/j/k/l`, `:q`, `/` for search
  - **Emacs:** `Ctrl+A/E/N/P`, `Ctrl+X Ctrl+C` to quit
- **Accessibility:**
  - Theme switching: Press `t` to cycle Default, HighContrast, Colorblind
  - All UI elements have high-contrast and colorblind-friendly modes
- **Help Bar:** Always visible at the bottom
- **Live updates:** If live tracking is enabled, new commands appear instantly

### 3. CLI Mode: Analytics & Scripting

- **Top commands:**

  ```sh
  heist --cli --top 20
  ```

- **Regex search:**

  ```sh
  heist --cli --search 'git.*push'
  ```

- **Filter by command:**

  ```sh
  heist --cli --filter ls
  ```

- **Date range:**

  ```sh
  heist --cli --range "2025-01-01:2025-07-22"
  ```

- **Export:**

  ```sh
  heist --cli --export json
  heist --cli --export csv
  ```

- **Session summary:**

  ```sh
  heist --cli --session-summary
  ```

- **Alias suggestions:**

  ```sh
  heist --cli --suggest-aliases
  ```

- **Dangerous command flagging:**

  ```sh
  heist --cli --flag-dangerous
  ```

- **Per-directory/host stats:**

  ```sh
  heist --cli --per-directory
  heist --cli --per-host
  ```

- **Time-of-day/heatmap:**

  ```sh
  heist --cli --time-of-day
  heist --cli --heatmap
  ```

### 4. Advanced Analytics & Workflows

- **Combine filters:**

  ```sh
  heist --cli --filter git --range "2025-01-01:2025-07-22" --top 5
  ```

- **Script integration:**

  ```sh
  heist --cli --export json | jq '.[] | select(.command | test("cargo"))'
  ```

- **Live tracking:**
  - Enable via installer or manually source `contrib/heist_live_tracking.sh` in your shell config
  - View real-time analytics in TUI or CLI
- **Session analysis:**
  - See how your workflow changes over time, identify productivity patterns
- **Dangerous command audit:**
  - Quickly find risky commands in your history for security reviews

### 5. Extending Heist (For Power Users & Developers)

- **Add a new shell parser:**
  - Create a new module in `src/parser/`
  - Implement parsing logic for the shell’s history format
  - Register the parser in `cli.rs`
- **Add new analytics:**
  - Extend `src/analyzer.rs` with new stats functions
  - Add CLI flags in `cli.rs`
- **Add new TUI tabs:**
  - Edit `src/ui/mod.rs` to add new analytics or visualizations
- **Testing:**
  - Add tests in module files (see `src/analyzer.rs`)
  - Run `cargo test` for all modules
- **CI/CD:**
  - GitHub Actions automate build, test, changelog, and releases

### 6. Performance Tips

- Use release builds for all analytics
- Filter or export only the data you need for large histories
- Archive old history files if you notice slowdowns
- Use a modern terminal and Nerd Font for best TUI experience

### 7. FAQ & Troubleshooting

- **TUI looks weird?** Use a Nerd Font and a modern terminal
- **Installer fails?** Check for `sudo` and `cargo` in your PATH
- **Shell not detected?** Use `--shell` to force the shell type
- **Analytics slow?** Archive old history, use filters, or split history files
- **Live tracking not working?** Ensure `contrib/heist_live_tracking.sh` is sourced in your shell config
- **Tests fail?** Ensure all dependencies are installed, run `cargo clean && cargo test`

### 8. Example Workflows (Beginner to God-Tier)

- **Beginner:**
  - Install and run TUI, explore tabs
  - Use `heist --cli --top 10` to see most-used commands
- **Intermediate:**
  - Use filters, date ranges, and exports for custom analytics
  - Enable live tracking for real-time stats
- **Advanced:**
  - Script Heist output with `jq`, `awk`, or other tools
  - Audit for dangerous commands, analyze sessions
- **God-Tier:**
  - Extend Heist with custom analytics, parsers, or TUI tabs
  - Automate workflows with CI/CD, contribute to the project
  - Benchmark and tune for massive histories (1M+ entries)

---

## 📜 License

MIT
