# Heist: The Ultimate Shell History Analyzer

---

## What is Heist?

Heist is a cross-platform, blazing-fast shell history analyzer and visualizer. It helps you understand, optimize, and secure your command-line workflow with beautiful analytics, smart suggestions, and advanced features. Supports Bash, Zsh, Fish, and most POSIX shells.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [CLI Usage](#cli-usage)
- [TUI Usage](#tui-usage)
- [Analytics Explained](#analytics-explained)
- [Live Tracking & Shell Integration](#live-tracking--shell-integration)
- [Customization](#customization)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)
- [Developer Guide](#developer-guide)
- [FAQ](#faq)
- [Changelog](#changelog)
- [Credits & License](#credits--license)
- [Security & Privacy](#security--privacy)
- [Shell History File Formats](#shell-history-file-formats)
- [How Heist Works (Architecture)](#how-heist-works-architecture)
- [Performance & Benchmarks](#performance--benchmarks)
- [Accessibility Features](#accessibility-features)
- [Customizing Analytics](#customizing-analytics)
- [Integrations & Scripting](#integrations--scripting)
- [Example Workflows](#example-workflows)
- [Community & Support](#community--support)
- [Glossary](#glossary)
- [Bug Reporting & Feature Requests](#bug-reporting--feature-requests)


---

## Features

- Modern TUI: tabs, icons, themes, accessibility, animations
- Powerful CLI: top commands, search, filter, export
- Alias suggestion engine: recommends shortcuts for long/frequent commands
- Dangerous command flagging: highlights risky patterns (e.g. `rm -rf`)
- Per-directory and per-host stats: see where and on which machine you run commands
- Time-of-day and weekly heatmap analytics: visualize your habits
- Session detection: group commands by shell session
- Multi-shell support: Bash, Zsh, Fish, and more
- Installer: auto-update, live tracking setup
- Docker: multi-arch, healthcheck, easy mounting
- Accessibility: colorblind themes, keyboard navigation

---

## Installation

### (Linux/macOS)

**One-liner:**

```sh
curl -fsSL https://raw.githubusercontent.com/Zer0C0d3R/Heist/master/install.sh | bash
```

**Manual:**

```sh
git clone https://github.com/Zer0C0d3R/Heist.git
cd Heist
./install.sh
```

<<<<<<< HEAD
f145e0 (Change : Docker Instructions Deleted)
---

## Quick Start

- Run `heist` for the TUI dashboard
- Run `heist --cli` for command-line analytics
- Use `heist --help` to see all options

---

## CLI Usage

Analyze your history with flexible options:

```sh
heist --cli --top 10 --search "rm -rf" --export json
heist --cli --per-directory --per-host --time-of-day --heatmap
heist --cli --suggest-aliases --flag-dangerous
```

**Export formats:** CSV, JSON

**Filter by time:** `--range 2025-01-01:2025-07-23`

**Filter by command:** `--filter git`

---

## TUI Usage

- Launch with `heist`
- Navigate tabs: ←/→
- Scroll: ↑/↓
- Search: `/` (type, then Enter)
- Switch key mode: F2 (Default/Vim/Emacs)
- Switch theme: F3 (Default/HighContrast/Colorblind)
- Quit: `q` or `Ctrl+C`

**Tabs:**

- Summary: Top commands, usage bar
- PerCommand: All commands, scrollable
- Sessions: Grouped by shell session
- Search: Regex or substring
- Aliases: Suggestions for long/frequent commands
- Dangerous: Flagged risky commands
- Directory/Host: Stats by location/machine
- TimeOfDay/Heatmap: Visualize habits

---

## Analytics Explained

- **Alias Suggestions:** Finds long or frequent commands and recommends short aliases
- **Dangerous Flagging:** Highlights commands matching risky patterns (customizable)
- **Per-Directory/Host:** Shows where and on which host you run commands most
- **Time-of-Day/Heatmap:** Visualizes when you use your shell most (hourly, weekly)
- **Session Detection:** Groups commands by shell session (10+ min gap = new session)

---

## Live Tracking & Shell Integration

Enable real-time history updates:

- Installer can append a snippet to your `.bashrc`/`.zshrc` for live tracking
- Uses `PROMPT_COMMAND` to log each command instantly
- To enable manually:
  - Source `contrib/heist_live_tracking.sh` in your shell config

---

## Customization

- **Themes:** Default, HighContrast, Colorblind (F3)
- **Keybindings:** Default, Vim, Emacs (F2)
- **Accessibility:** Keyboard navigation, colorblind support
- **Config:** Edit `config.toml` (coming soon)

---

## Advanced Usage

- **Export:** `heist --cli --export csv > history.csv`
- **Scripting:** Use CLI output in scripts for automation
- **Custom Patterns:** Edit dangerous patterns in source
- **Session Analysis:** Filter by time, host, directory
- **Performance:** Handles large history files efficiently

---

## Troubleshooting

- **TUI not rendering:** Run in a true terminal, not redirected or in Docker without `-it`
- **Corrupted output:** Use supported terminal emulator
- **Installer issues:** Ensure dependencies (`cargo`, `sudo`) are installed
- **Docker issues:** Use `-it`, mount history files, check healthcheck
- **Live tracking not working:** Restart shell, check `.bashrc`/`.zshrc` for snippet

---

## Developer Guide

- **Codebase:** Rust, ratatui, crossterm, modular (CLI, TUI, parser, analyzer, models)
- **Tests:** Unit tests in module files
- **Logging:** Errors logged to `heist_error.log`
- **Contributing:** Fork, branch, PR, follow [CONTRIBUTING.md](CONTRIBUTING.md)
- **Release:** Tag, update version in installer/Dockerfile/README
- **CI/CD:** GitHub Actions, Dependabot

---

## FAQ

**Q: Which shells are supported?**

A: Bash, Zsh, Fish, Csh, Tcsh, Ksh, Dash, Sh, Mksh, Yash, Osh

**Q: How do I customize dangerous patterns?**

A: Edit the patterns in `src/ui/mod.rs` and rebuild

**Q: How do I update Heist?**

A: Run the installer and choose Update, or pull latest and rerun

**Q: How do I contribute?**

A: Fork, branch, PR, see [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and release notes.

---

## Credits & License

Created by [Zer0C0d3R](https://github.com/Zer0C0d3R). Powered by Rust, ratatui, crossterm, and the open source community.

MIT License. See [LICENSE](LICENSE).

---

## Security & Privacy

- Heist only reads your local shell history files; no data is sent externally.
- Sensitive commands (passwords, tokens) are never stored or exported by Heist.
- You can exclude or redact history entries by editing your history file before analysis.
- All analytics are performed locally and securely.

---

## Shell History File Formats

- **Bash:** `~/.bash_history` (plain text)
- **Zsh:** `~/.zsh_history` (may be extended format)
- **Fish:** `~/.local/share/fish/fish_history` (YAML)
- **Others:** See [Supported Shells](#faq)
- Heist auto-detects and parses most formats; for custom formats, see developer guide.

---

## How Heist Works (Architecture)

- Modular Rust codebase: CLI, TUI, parser, analyzer, models
- History is parsed, analyzed, and cached for fast rendering
- TUI uses ratatui and crossterm for rich UI and keyboard navigation
- CLI provides flexible analytics and export options
- Live tracking uses shell hooks to append commands in real time

---

## Performance & Benchmarks

- Handles history files with 100,000+ entries efficiently
- Caching and optimized data structures minimize flicker and lag
- Benchmarks: <100ms for analytics on typical history files
- For huge files, use CLI mode for fastest results

---

## Accessibility Features

- Colorblind-friendly themes (F3)
- Keyboard navigation (Tab, arrows, Vim/Emacs modes)
- High-contrast mode for low-vision users
- All analytics available in CLI for screen readers

---

## Customizing Analytics

- Edit dangerous patterns in `src/ui/mod.rs` to flag custom commands
- Change alias suggestion thresholds in source or config (coming soon)
- Filter by time, command, directory, host using CLI flags
- Export analytics for further processing

---

## Integrations & Scripting

- Use CLI output in shell scripts for automation
- Integrate with shell plugins (e.g., Oh My Zsh, Prezto)
- Export to CSV/JSON for use in CI/CD or reporting tools
- Example: `heist --cli --export csv | grep rm`

---

## Example Workflows

- **Security Audit:** `heist --cli --flag-dangerous --export json`
- **Productivity Review:** `heist --cli --top 20 --per-directory --time-of-day`
- **Alias Optimization:** `heist --cli --suggest-aliases`
- **Team Analytics:** Mount multiple history files and aggregate results

---

## Community & Support

- GitHub Issues: [Report bugs or request features](https://github.com/Zer0C0d3r/Heist/issues)
- Discussions: [Join the community](https://github.com/Zer0C0d3r/Heist/discussions)
- Roadmap: See [Projects](https://github.com/Zer0C0d3r/Heist/projects)
- Contributing: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Glossary

- **Session:** Group of commands run in a shell instance
- **Heatmap:** Visual representation of command frequency by time
- **Alias:** Short command mapped to a longer one
- **Dangerous Command:** Command pattern flagged as risky
- **TUI:** Text User Interface
- **CLI:** Command Line Interface

---

## Bug Reporting & Feature Requests

- Use GitHub Issues for bugs, feature requests, and questions
- Include OS, shell, Heist version, and steps to reproduce
- For security issues, use private contact in repository

