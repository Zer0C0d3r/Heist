# Changelog

## [1.0.0] - 2025-07-23

### Major Release

- Stable, feature-complete release: TUI/CLI, multi-shell, real-time analytics, accessibility, advanced error logging, installer, and docs.
- Real-time history updates in TUI (live tracking via PROMPT_COMMAND).
- Accessibility: theme switching (default, high-contrast, colorblind), Vim/Emacs keybindings, icons, animations.
- Advanced analytics: alias suggestions, dangerous command flagging, per-directory/host stats, time-of-day, heatmap, session grouping.
- Comprehensive tests, robust error handling, and logging.
- Professional README and installer script.
- CI/CD automation, release workflow, dependabot, coverage, lint.

### Added

- All analytics features: alias suggestion, dangerous command flagging, per-directory/host stats, time-of-day, heatmap, session detection, live tracking.
- TUI polish: tabs, icons, animations, accessibility, flicker-free rendering.
- Installer: animated, auto-update, live tracking setup.
- Documentation: README overhaul, contributor guide, architecture, FAQ.

### Changed

- Refactored codebase for modularity, performance, and extensibility.
- Improved error messages, logging, and accessibility.
- Updated workflows and automation for releases.

### Fixed

- All known issues from previous versions resolved.
- No breaking changes; all features stable and tested.

---

## [0.2.1] - 2024-06-09

### Added

- Time-of-day analytics: Show hourly command usage with `--time-of-day` flag.
- Weekly heatmap analytics: Show hour-by-day usage heatmap with `--heatmap` flag.

### Changed

- CLI: New flags `--time-of-day` and `--heatmap` for advanced analytics.
- Docs: Updated to reflect new analytics features.

### Fixed

- No breaking changes. All previous features remain stable.

---

## [0.2.0] - 2024-06-08

- Major refactor, TUI/CLI improvements, multi-shell support, session detection, alias suggestion, dangerous command flagging, per-directory/host stats, installer rewrite, CI/CD automation, release workflow, removal of cloud/i18n

---

## [0.1.5] - 2024-06-05

### Added

- Per-directory and per-host stats (CLI: --per-directory, --per-host)
- Improved session detection for all supported shells

### Changed

- More robust grouping for plain-text shells

---

## [0.1.4] - 2024-06-03

### Added

- Dangerous command flagging (CLI: --flag-dangerous)
- Alias suggestion engine (CLI: --suggest-aliases)

---

## [0.1.3] - 2024-06-01

### Added

- Interactive, animated installer script with auto-update function
- Uninstaller option in installer

---

## [0.1.2] - 2024-05-31

### Added

- CI/CD improvements: GitHub Actions for build, test, changelog, and auto-release on tag
- Release workflow automation

### Changed

- Removed cloud/i18n features for privacy and simplicity

---

## [0.1.1] - 2024-05-30

### Added

- Export to CSV/JSON
- Regex and keyword search in CLI
- Session stats: total sessions, average session length

---

## [0.1.0] - 2024-05-30

### Added

- Initial release: cross-platform shell history analyzer in Rust
- Interactive TUI (ratatui, crossterm)
- CLI mode for quick stats and search
- Bash, Zsh, Fish, Csh, Tcsh, Ksh, Dash, Sh, Mksh, Yash, Osh support
- Modular parser and analytics engine
- Session grouping, per-command stats, regex search
- Export to CSV/JSON
- Installer/uninstaller script

### Known Issues

- No per-directory/host stats
- No alias suggestion or dangerous command flagging
- No CI/CD automation
- No time-of-day/heatmap analytics
- No live tracking
- No Vim/Emacs keybindings, accessibility, or advanced error logging
