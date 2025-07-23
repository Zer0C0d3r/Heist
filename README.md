<p align="center">
  <img src="https://raw.githubusercontent.com/Zer0C0d3r/Heist/main/assets/logo.png" alt="Heist Logo" width="120"/>
</p>

<h1 align="center">Heist <img src="https://img.shields.io/badge/shell%20history%20analytics-%F0%9F%94%A5-blue" alt="analytics"/></h1>

<p align="center">
  <b>Blazing-fast, cross-platform shell history analytics with TUI & CLI</b><br>
  <a href="https://github.com/Zer0C0d3r/Heist/actions/workflows/ci.yml"><img src="https://github.com/Zer0C0d3r/Heist/actions/workflows/ci.yml/badge.svg" alt="CI"/></a>
  <a href="https://github.com/Zer0C0d3r/Heist/actions/workflows/lint.yml"><img src="https://github.com/Zer0C0d3r/Heist/actions/workflows/lint.yml/badge.svg" alt="Lint"/></a>
  <a href="https://github.com/Zer0C0d3r/Heist/actions/workflows/coverage.yml"><img src="https://github.com/Zer0C0d3r/Heist/actions/workflows/coverage.yml/badge.svg" alt="Coverage"/></a>
  <a href="https://crates.io/crates/heist"><img src="https://img.shields.io/crates/v/heist.svg" alt="crates.io"/></a>
  <a href="https://github.com/Zer0C0d3r/Heist/blob/master/LICENSE"><img src="https://img.shields.io/github/license/Zer0C0d3r/Heist.svg" alt="License"/></a>
</p>

---

<p align="center">
  <img src="https://raw.githubusercontent.com/Zer0C0d3r/Heist/main/assets/screenshot.png" alt="Heist TUI Screenshot" width="600"/>
</p>

---

# 🚀 Features

- ⚡ **Super-fast analytics** for Bash, Zsh, Fish, Csh, Ksh, Dash, and more
- 🖥️ **Interactive TUI** with tabs, Vim/Emacs keybindings, and accessibility themes
- 🧠 **Advanced CLI**: search, filter, export, session stats, alias suggestions, dangerous command flagging
- 📊 **Time-of-day & heatmap analytics**
- 🏷️ **Per-directory & per-host stats**
- 🔴 **Live tracking** via PROMPT_COMMAND (real-time history)
- 📦 **Export** to CSV/JSON
- 🛠️ **Easy install/uninstall** script
- 🔒 **Privacy-first**: all analysis is local

---

# ⚡ Quickstart

## 1. Install

```sh
# With installer (recommended)
git clone https://github.com/Zer0C0d3r/Heist.git
cd Heist
./install.sh
```

## 2. Run

```sh
# TUI mode (default)
heist

# CLI analytics
heist --cli --top 10
heist --cli --heatmap
```

---

# 🖥️ TUI Highlights

- Tabs: Summary, Per-Command, Sessions, Search
- Navigation: ←/→ tabs, ↑/↓ scroll, Enter select, q/Esc/Ctrl+C quit
- Keybindings: Default, Vim (h/j/k/l, :q, /), Emacs (Ctrl+A/E/N/P)
- Accessibility: Theme switching (t), high-contrast, colorblind
- Live updates: See new commands instantly with live tracking

---

# 🧑‍💻 CLI Power

```sh
# Top commands
heist --cli --top 20

# Regex search
heist --cli --search 'git.*push'

# Filter by command
heist --cli --filter ls

# Date range
heist --cli --range "2025-01-01:2025-07-22"

# Export
heist --cli --export json
heist --cli --export csv

# Session summary
heist --cli --session-summary

# Alias suggestions
heist --cli --suggest-aliases

# Dangerous command flagging
heist --cli --flag-dangerous

# Per-directory/host stats
heist --cli --per-directory
heist --cli --per-host

# Time-of-day/heatmap
heist --cli --time-of-day
heist --cli --heatmap
```

---

# 📊 Analytics & Workflows

- Combine filters: `heist --cli --filter git --range "2025-01-01:2025-07-22" --top 5`
- Script integration: `heist --cli --export json | jq '.[] | select(.command | test("cargo"))'`
- Live tracking: Enable via installer or source `contrib/heist_live_tracking.sh`
- Session analysis: Productivity patterns, session grouping
- Dangerous command audit: Security reviews

---

# 🏗️ Architecture

- `src/cli.rs`: CLI argument parsing
- `src/parser/`: Shell history parsing (modular)
- `src/ui/`: TUI rendering
- `src/analyzer.rs`: Analytics/statistics
- `src/models.rs`: Data models
- `install.sh`: Installer/uninstaller

---

# 🛠️ Developer & Contributor Guide

- Modular, hackable Rust codebase
- Add new analytics, parsers, or TUI tabs easily
- Run tests: `cargo test`
- Lint/fix: `cargo clippy`, `cargo fmt`
- CI: GitHub Actions (CI, lint, coverage, release)
- PRs and issues welcome!

---

# 🧩 Supported Shells

- Bash, Zsh, Fish, Csh, Tcsh, Ksh, Dash, Sh, Mksh, Yash, Osh
- Auto-detects shell, or use `--shell` to override

---

# 🛡️ Security & Privacy

- All analytics are local by default
- No cloud sync, no telemetry
- Open source (MIT License)

---

# 📝 FAQ & Troubleshooting

- **TUI looks weird?** Use a Nerd Font and a modern terminal
- **Installer fails?** Check for `sudo` and `cargo` in your PATH
- **Shell not detected?** Use `--shell` to force the shell type
- **Analytics slow?** Archive old history, use filters, or split history files
- **Live tracking not working?** Ensure `contrib/heist_live_tracking.sh` is sourced in your shell config
- **Tests fail?** Run `cargo clean && cargo test`

---

# 📜 License

MIT
