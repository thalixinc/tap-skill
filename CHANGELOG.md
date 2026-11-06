# Changelog

All notable changes to TAP will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-21

### Added

- Initial release of TAP (Traceable Acceptance Pack)
- Rust CLI (`tap`) with four commands:
  - `tap version` — print version + check for updates
  - `tap update` / `tap upgrade` — self-update via cargo install
  - `tap install` — install TAP skill into agent skill directories
  - `tap doctor` — show detected skill locations + installation status
- TAP skill (`skills/tap/SKILL.md`) for Cursor/Claude/Codex agents
- Six role briefs (`skills/tap/roles/`):
  - `audit.md` — Current-state inventory (CARRY/REWRITE/DROP)
  - `platform.md` — Surface/interface design (Claims + UCs)
  - `runtime.md` — Execution/state design (Claims + UCs)
  - `critique.md` — Must-fix judge (no rubber-stamp)
  - `chair.md` — Fold & synthesize (single spine)
  - `filer.md` — GitHub Issues from locked plan
- Three starter templates (`templates/`):
  - `CLAIM-TEMPLATE.md` — empty Claim table
  - `UC-TEMPLATE.md` — empty UC writeup
  - `BUILD-PLAN-TEMPLATE.md` — epic/story/task skeleton
- Comprehensive documentation:
  - `README.md` — installation, CLI usage, TAP overview
  - `docs/process.md` — TAP process deep dive
- Embedded skill files in binary (via `include_dir`)
- Self-update mechanism (fetch version from GitHub, cargo install --force)
- MIT license

### Technical Details

- Rust edition 2021
- Dependencies: clap 4.5, serde 1.0, serde_json 1.0, include_dir 0.7
- Binary name: `tap`
- Install via: `cargo install --git https://github.com/thalixinc/tap`

[0.1.0]: https://github.com/thalixinc/tap/releases/tag/v0.1.0
