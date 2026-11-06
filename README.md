# TAP: Traceable Acceptance Pack

**TAP** = **Traceable Acceptance Pack**: a reusable documentation and agent skill method for product design that ensures **Claims → Use Cases → Acceptance Tests** traceability.

🔗 **Requirements traceability** (RTM / V-Model / Spec-by-Example)  
🔒 **Founder-lockable Claims** (architecture principles)  
✅ **Executable acceptance paths** (step-by-step UCs with test hooks)  
📋 **Work breakdown** (epics/stories/tasks from locked spine)

---

## What is TAP?

TAP is **not** a branded ISO framework. It borrows from proven practices:

- **Requirements Traceability Matrix (RTM)** — bidirectional Claim↔UC↔Test links
- **V-Model** (V&V) — prove-design-with-tests shape
- **Specification by Example / ATDD / BDD** — executable acceptance
- **ADR** (Architecture Decision Records) — locked architecture principles

The **Claim** is the top lock unit: a counsel-coined term for founder-lockable assertions (architecture principles, technical contracts).

**One-liner:** Requirements/architecture traceability where the top lock unit is **Claims**.

---

## Installation

### Install TAP CLI

```bash
cargo install --git https://github.com/thalixinc/tap-skill
```

Or from a tagged release:

```bash
cargo install --git https://github.com/thalixinc/tap-skill --tag v0.1.0
```

### Install TAP Skill for Agents

After installing the CLI, install the TAP skill into your agent skill directory.

**Interactive mode (recommended):**

```bash
tap-skill install
```

The CLI will prompt you to:
1. Select harness(es): Cursor, Claude, Codex, or custom path (multi-select)
2. Select scope: User (global `~/.cursor/skills`) or Project (local `.cursor/skills`)

**Non-interactive mode (for scripts/CI):**

```bash
# Single harness, user scope
tap-skill install --harness cursor --scope user

# Multiple harnesses, user scope
tap-skill install --harness cursor,claude,codex --scope user

# Project scope (installs to .cursor/skills in current directory)
tap-skill install --harness cursor --scope project

# Custom path
tap-skill install --harness custom --scope user --path ~/.my-agent/skills
```

**Check installation status:**

```bash
tap-skill doctor
```

---

## CLI Commands

### `tap-skill version`

Print version information and check for updates.

```bash
tap-skill version
```

Auto-update if newer version available:

```bash
tap-skill version --yes
```

### `tap-skill update` / `tap-skill upgrade`

Update TAP to the latest version.

```bash
tap-skill update
```

Check for updates without installing:

```bash
tap-skill update --check
```

### `tap-skill install`

Install TAP skill into an agent skill directory.

**Interactive mode (prompts for harness + scope):**

```bash
tap-skill install
```

Prompts:
1. **Select harness(es):** Cursor, Claude, Codex, or custom (comma-separated numbers)
2. **Select scope:** User (global) or Project (local)

**Non-interactive mode (flags):**

```bash
# Required flags: --harness and --scope
tap-skill install --harness cursor --scope user
tap-skill install --harness cursor,claude --scope user
tap-skill install --harness cursor --scope project
tap-skill install --harness custom --scope user --path ~/.my-skills
```

**Harness values:** `cursor`, `claude`, `codex`, `custom` (comma-separated for multiple)  
**Scope values:** `user` (global: `~/.cursor/skills/tap`) or `project` (local: `.cursor/skills/tap`)

### `tap-skill doctor`

Show detected skill install locations and installation status.

```bash
tap-skill doctor
```

Example output:

```
TAP skill installation diagnostics

Cursor       ✓ installed -> /home/user/.cursor/skills/tap
Claude       ✗ not found -> /home/user/.claude/skills/tap
Codex        ✗ not found -> /home/user/.codex/skills/tap

To install: tap-skill install [--cursor|--claude|--codex|--path <dir>]
```

---

## When to Use TAP

Use TAP when:

1. **Greenfield product design** — building a new product from scratch
2. **Before filing GitHub Issues/Projects** — need architecture spine + acceptance paths first
3. **Traceability required** — every use case must tag Claims; every UC needs test hooks
4. **Counsel-style design** — multiple specialists propose/critique before founder lock
5. **No MVP exists yet** — you need the "day-one basics" acceptance freeze before build

**Do NOT use TAP for:**

- Simple feature additions to existing products
- Urgent bug fixes
- Products with frozen architecture already documented elsewhere
- Marketing briefs without technical steers

---

## TAP Layers

### 1. Claims (Architecture)

**Claims** = locked architecture principles or technical contracts.

**Heuristic:** A Claim is locked only when:

1. It survives independent challenge (Platform + Runtime)
2. Critique has no open must-fix
3. Founder locks the pack (or a named Claim)

**Example:**

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 1 | Factory must not mint terminal runs | Runtime monopoly: only Appender terminals | UC-07, UC-13 |
| 2 | RuleSets are missions (no Mission object) | Founder lock: simplicity over abstraction | UC-05, UC-14 |

### 2. Use Cases (Acceptance)

**Use Cases** = step-by-step CLI/GUI paths with test hooks.

**Heuristic:**

- Every UC must tag Claims (fail if orphan UC or Claim)
- Every UC has a test hook ID (e.g., `uc01_bootstrap`)
- UCs are NOT slogans — they are executable acceptance paths

**Example:**

```markdown
## UC-01: Bootstrap CLI

**Actor:** Ops admin  
**Preconditions:** Fresh install  
**Claims:** 1, 4, 9b  
**Steps:**
1. Run `factory init --env prod`
2. Verify config written to `~/.factory/config.toml`
3. Check `factory status` shows "ready"

**Expected result:** CLI installed, no errors  
**Test hook:** `uc01_bootstrap`  
**Docs:** §3.1 Installation
```

### 3. Tests (Verification)

**Tests** = acceptance suite entries.

**Heuristic:**

- If a UC has no test hook, it is not done
- Split mega-suites so one agent/PR can own a batch
- Tests assert architecture rules (e.g., Factory cannot terminal)

### 4. BUILD-PLAN (Work Breakdown)

**BUILD-PLAN** = epics/stories/tasks with coverage matrix.

**Heuristic:**

- Epics E-A…E-N map to Claim clusters / surfaces
- Stories/tasks carry three directives (where to look, DoD, self-check)
- Coverage matrix (Claim×UC×Epic) produced **before** filing tasks
- File only what the locked plan contains — no invented tickets

---

## TAP Roles

TAP uses a **Design Counsel** pattern with these roles:

| Role | Responsibility | Deliverable |
|------|---------------|-------------|
| **Audit** | Current-state inventory: CARRY/REWRITE/DROP | Audit map |
| **Platform** | Independent design (surface/interface) | Platform proposal: Claims + UCs |
| **Runtime** | Independent design (execution/state) | Runtime proposal: Claims + UCs |
| **Critique** | Must-fix ranks (no rubber-stamp) | Must-fix list |
| **Chair** | Fold into single spine table | Authoritative pack file |
| **Filer** | File GitHub Issues (only after lock) | Filing report: plan-id → issue# |

**Three directives on every assignment:**

1. Where to look
2. Definition of done
3. How to self-check

See `skills/tap/roles/*.md` for detailed role briefs.

---

## TAP Recipe (Repeatable Playbook)

### Counsel Pattern (Sequential Gates)

```
Founder steer (via FCoA or equivalent)
    ↓
Audit inventory / reference map (if legacy exists)
    ↓
Platform + Runtime independent designs (no peek)
    ↓
Critique must-fix first (no rubber stamp)
    ↓
Chair fold into one pack file
    ↓
Founder lock (or reject)
    ↓
Only then: Filer creates Issues / Project fields
    ↓
Counsel idle unless assigned
```

### Inputs Needed from Founder

1. Product one-liner + hard non-goals
2. Whether legacy code is reference or SoT
3. Day-one cuts (CLI vs GUI, single-user vs org)
4. Who locks packs (founder via FCoA)
5. When filing is allowed (after lock, not before)

### Artifacts to Produce

1. `AUTHORITATIVE-SPINE.md` — locked Claims table
2. `DAYONE-USE-CASES.md` — UC ↔ Claims ↔ test hooks
3. `BUILD-PLAN.md` — epics/stories/tasks + coverage matrix
4. Filing report — plan id → issue#; Project URL; SHA
5. Optional second packs (same counsel→lock→file loop)

### Do-Nots

- Do not invent traction, SHAs, or "already built"
- Do not file Issues before founder lock
- Do not make forbidden objects (e.g., Mission if founder forbade)
- Do not pin vendor CLIs as product SoT — CARRY intent, REWRITE binding

---

## Templates

TAP includes starter templates:

- `templates/CLAIM-TEMPLATE.md` — empty Claim table
- `templates/UC-TEMPLATE.md` — empty UC writeup
- `templates/BUILD-PLAN-TEMPLATE.md` — epic/story/task skeleton

Access templates after `tap install`:

```bash
# Cursor
~/.cursor/skills/tap/templates/

# Custom path
<your-path>/tap/templates/
```

---

## Example: Codefactory (Reference)

**Product:** Chris CF / `thalixinc/codefactory-v2`  
**Project:** https://github.com/orgs/thalixinc/projects/40

**Chronology:**

| When | Step | Outcome |
|------|------|---------|
| Early Round 4 | Hard reset: greenfield, not salvage | Old clean-pack withdrawn |
| Claim cycles 1–9 | Challenge memos → Critique → Chair LOCKED | Spine grows claim-by-claim |
| 2026-09-19 | Whole spine founder-locked | Architecture freeze |
| 2026-09-19 | Day-one UC pack founder-locked | Acceptance freeze |
| 2026-09-19–20 | BUILD-PLAN → Critique → PASS → file | First board (#1–#73) |
| 2026-09-20–21 | Providers pack → founder lock → file | Second pack (#74–#81) |

**Artifacts:**

- `AUTHORITATIVE-SPINE.md` (Claims 1–9 + E1–E4)
- `DAYONE-USE-CASES.md` (UC-01…16)
- `BUILD-PLAN.md` (Epics E-A…E-N)
- `BUILD-PLAN-GITHUB-FILED.md` (report)
- `FACTORY-PROVIDERS-PACK.md` (second pack)

---

## Coverage Matrix (Pre-Filing)

**Write the coverage matrix BEFORE drafting tasks.**

| Claims | UCs | Epics |
|--------|-----|-------|
| 1, 4, 9b | UC-01, UC-02 | E-A |
| 2, 3, 5 | UC-14, UC-15 | E-G |

This ensures:

1. No orphan Claims (every Claim → at least one UC)
2. No orphan UCs (every UC → at least one test hook)
3. No orphan Epics (every Epic → Claims + UCs)

---

## Gates (Do Not Skip)

| Gate | Fail if… |
|------|----------|
| **Audit** | Invented features; vocab bleed; salvage treated as SoT |
| **Independent designs** | Specialists peeked / rubber-stamped each other |
| **Critique** | Must-fix open |
| **Chair pack** | Multiple competing "authoritative" docs |
| **Founder lock** | Filing Issues "to get ahead" |
| **Filer** | Scope beyond locked § / plan |

---

## What We Would Do Differently Next Time

1. Keep **one** authoritative spine path from day one
2. Write **coverage matrix** (Claim×UC×Epic) **before** drafting tasks
3. Pre-agree Project field schema (Phase / Work type / Epic id)
4. Split "human build SDLC board" vs "in-product providers" earlier
5. Capture this process doc **during** the first lock, not after filing

---

## Repository Structure

```
tap/
├── Cargo.toml              # Rust package manifest
├── src/
│   └── main.rs             # CLI implementation
├── skills/
│   └── tap/
│       ├── SKILL.md        # TAP skill for agents
│       ├── roles/          # Role briefs (audit, platform, runtime, critique, chair, filer)
│       └── templates/      # Starter templates (copied from top-level templates/)
├── templates/              # Source templates (embedded in binary)
│   ├── CLAIM-TEMPLATE.md
│   ├── UC-TEMPLATE.md
│   └── BUILD-PLAN-TEMPLATE.md
├── docs/
│   └── process.md          # TAP process deep dive
├── README.md               # This file
├── LICENSE                 # MIT license
└── CHANGELOG.md            # Version history
```

---

## Development

### Build from Source

```bash
git clone https://github.com/thalixinc/tap.git
cd tap
cargo build --release
```

Binary: `target/release/tap`

### Test

```bash
cargo test
```

### Install from Local

```bash
cargo install --path .
```

---

## Self-Update

TAP supports self-update via `cargo install --git`:

```bash
tap-skill update
```

Or auto-update on version check:

```bash
tap-skill version --yes
```

The update mechanism:

1. Fetches latest version from GitHub `Cargo.toml`
2. Compares with current version (semver)
3. Runs `cargo install --git https://github.com/thalixinc/tap --force` if newer

---

## License

MIT License. See [LICENSE](LICENSE) file.

---

## Repository

🔗 **https://github.com/thalixinc/tap-skill**

---

## Support

- **Issues:** https://github.com/thalixinc/tap-skill/issues
- **Docs:** See `docs/` directory and `skills/tap/SKILL.md`
- **Examples:** Codefactory reference (see SKILL.md)

---

## Summary

**TAP** = **Traceable Acceptance Pack**: Claims → Use Cases → Acceptance Tests.

- **Claims** = locked architecture principles (top lock unit)
- **Use Cases** = step-by-step CLI/GUI paths with test hooks
- **Tests** = acceptance suite entries (uc01_…, etc.)
- **BUILD-PLAN** = epics/stories/tasks with coverage matrix
- **Filer** = GitHub Issues (only after founder lock)

**Counsel pattern:** Audit → Platform + Runtime (independent) → Critique → Chair → Founder lock → File.

**Three directives on every assignment:** where to look, DoD, self-check.

**Do not file Issues before founder lock. Do not invent traction.**

---

**Install:** `cargo install --git https://github.com/thalixinc/tap-skill`  
**Use:** `tap-skill install` (interactive) or with flags  
**Update:** `tap-skill update`
