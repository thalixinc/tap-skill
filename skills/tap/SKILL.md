---
name: Traceable Acceptance Pack (TAP)
description: Requirements-to-tests traceability method using Claims → Use Cases → Acceptance Tests. Use when designing a new product from scratch and need to establish architecture spine, acceptance paths, and work breakdown before filing GitHub Issues.
---

# Traceable Acceptance Pack (TAP)

## What is TAP?

**TAP** = **Traceable Acceptance Pack**: a reusable documentation and agent skill method for product design that ensures **Claims → Use Cases → Acceptance Tests** traceability. This is **not** a branded ISO framework — it borrows from:

- **Requirements Traceability Matrix (RTM)** / test traceability matrix (ISO/IEC/IEEE 29148)
- **V-Model** (V&V) for prove-design-with-tests shape
- **Specification by Example** / ATDD / BDD for executable acceptance
- **ADR** (Architecture Decision Records) for locked architecture principles

The **Claim** is the top lock unit: a counsel-coined term for founder-lockable assertions (architecture principles, technical contracts).

**One-liner:** Requirements/architecture traceability (RTM/V-Model/Spec-by-Example flavors) where the top lock unit is called **Claims**.

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

## TAP Roles

TAP uses a **Design Counsel** pattern with these roles. Each role can be mapped to **agents or subagents**. Every assignment to a role carries **three directives**:

1. Where to look (context/constraints)
2. Definition of done (deliverable shape)
3. How to self-check (quality gate)

| Role | Responsibility | Deliverable |
|------|---------------|-------------|
| **Audit** | Current-state inventory: CARRY/REWRITE/DROP on legacy code (if exists). Stops salvage-as-SoT; cites paths/issues or marks UNKNOWN. | Audit map: what exists, what to keep as reference only, gaps |
| **Platform** | Independent design: templates, CLI-first, packs/share, day-one catalog. Proposes Claims and UCs without peeking at Runtime. | Platform proposal: Claims + UCs from surface/interface view |
| **Runtime** | Independent design: run events, appender monopoly, learning gate, observability. Proposes Claims and UCs without peeking at Platform. | Runtime proposal: Claims + UCs from execution/state view |
| **Critique** | Must-fix ranks (no rubber-stamp). Blocks progression until open must-fixes resolved. Improves wording; catches orphan Claims/UCs; enforces coverage. | Must-fix list: MF-1, MF-2, etc. with severity and gate |
| **Chair** | Fold independent proposals into single spine table. Withdraws superseded docs. Owns coverage matrix (Claim×UC×Epic). | Single authoritative pack file per layer (spine, UC, BUILD-PLAN) |
| **Filer** | **Only after founder lock**: files GitHub Issues from BUILD-PLAN. Reports plan-id → issue# map. No invented tickets. | Filing report: plan id → issue#, Project URL, SHA of locked pack |

**Three directives on every assignment:**

- **Where to look:** Founder steers, locked packs (if any), reference code (labeled as such)
- **Definition of done:** Pack file with Claims/UCs/tests; coverage matrix; no open must-fixes
- **How to self-check:** Every UC tags Claims; every UC has test hook; no orphan Claims

---

## TAP Recipe (Repeatable Playbook)

### Inputs Needed from Founder (Before Inventing)

1. **Product one-liner + hard non-goals**
2. **Whether legacy code is reference or SoT** (e.g., "reference only")
3. **Day-one cuts** (CLI vs GUI, single-user vs org, learning on/off)
4. **Who locks packs** (founder via FCoA or similar authority)
5. **When filing is allowed** (after lock, not before)

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

**Three directives on every assignment and every Ready card:**

1. Where to look
2. Definition of done
3. How to self-check

### Gates (Do Not Skip)

| Gate | Fail if… |
|------|----------|
| **Audit** | Invented features; vocab bleed from other products; salvage treated as SoT without label |
| **Independent designs** | Specialists peeked / rubber-stamped each other |
| **Critique** | Must-fix open |
| **Chair pack** | Multiple competing "authoritative" docs |
| **Founder lock** | Filing Issues "to get ahead" |
| **Filer** | Scope beyond locked § / plan |

### Artifacts to Produce (Minimum)

1. **`AUTHORITATIVE-SPINE.md`** (or product spine) — locked Claims table
2. **`DAYONE-USE-CASES.md`** (or acceptance pack) — UC ↔ Claims ↔ test hooks
3. **`BUILD-PLAN.md`** — epics/stories/tasks + coverage matrix + three directives
4. **Filing report** — plan id → issue#; Project URL; SHA of pack on main
5. **Optional second packs** (e.g., Providers) using the **same** counsel→lock→file loop

### Do-Nots

- **Do not invent traction, SHAs, or "already built"** — only cite locked packs/reports
- **Do not file Issues before founder lock**
- **Do not make forbidden objects** (e.g., Mission-like objects if founder forbade them)
- **Do not blur roles** (e.g., Admin = Chair = promote authority)
- **Do not pin vendor CLIs as product SoT** — CARRY intent, REWRITE binding
- **Do not stand a sealed build squad unless founder asks**

---

## Claims (Architecture Layer)

### What is a Claim?

A **Claim** is a locked architecture principle or technical contract. Claims are the **top lock unit** in TAP.

**Heuristic:** A Claim is locked only when:

1. It survives independent challenge (Platform + Runtime)
2. Critique has no open must-fix
3. Founder locks the pack (or a named Claim)

**Prefer small durable contracts over frameworks.**

### Claim Sources

| Source | Role in Claim Creation |
|--------|----------------------|
| Founder steers | Product metaphors + hard locks (e.g., "RuleSets are missions; no Mission object") |
| Audit | Stops salvage-as-SoT; cites paths/issues or UNKNOWN |
| Platform | Templates, factory thinness, CLI-first, packs/share |
| Runtime | Run events, appender monopoly, learning gate, observability |
| Critique | Must-fixes; improves wording; blocks rubber-stamp |
| Chair | Single spine table; withdraws superseded docs |

### Example Claim Shape

```markdown
| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 1 | Factory must not mint terminal runs | Runtime monopoly: only Appender terminals | UC-07, UC-13 |
| 2 | RuleSets are missions (no Mission object) | Founder lock: simplicity over abstraction | UC-05, UC-14 |
```

---

## Use Cases (Acceptance Layer)

### What is a Use Case?

A **Use Case** is a **step-by-step CLI path** (or GUI path) with:

- Actor
- Preconditions
- Expected result
- **Test hook ID** (e.g., `uc01_bootstrap`, `uc07_run_fail_closed`)
- Docs section reference

**UCs are NOT slogans.** They are executable acceptance paths.

### UC Heuristic

- **Every UC must tag Claims** (fail if orphan UC or Claim)
- **Day-one cuts explicit** (install → register harness → templates → crew → run → observe → train stub → fail-closed)
- **Out-of-scope listed** (e.g., "UC-16 promote path: later, when learning enabled")

### UC Sources

| Source | Role in UC Creation |
|--------|---------------------|
| Locked Claims | Every UC must tag Claims |
| Founder "day-one basics" ask | Install → templates → run → observe → train stub |
| Platform | CLI-shaped catalog |
| Runtime | Mint/seq/terminal hooks, digest channel, analytics ON / promote OFF |
| Critique | MF gaps (e.g., LearningProposal stub, promote-when-enabled path) |

### Example UC Shape

```markdown
## UC-01: Bootstrap Factory Control Plane

**Actor:** Ops admin  
**Preconditions:** Fresh install  
**Claims:** 4, 9b  
**Steps:**
1. Run `factory init --env prod`
2. Verify config written to `~/.factory/config.toml`
3. Check `factory status` shows "ready"

**Expected result:** Control plane installed, no errors  
**Test hook:** `uc01_bootstrap`  
**Docs:** §3.1 Installation
```

---

## Tests (Verification Layer)

### Test Heuristic

- **If a UC has no test hook, it is not done.**
- **Split mega-suites** so one agent/PR can own a batch.
- **Tests assert architecture rules** (e.g., Factory cannot terminal; dry-run cannot mint)

### Test Sources

| Source | Role in Test Creation |
|--------|----------------------|
| Each UC "Test" / "Test hook" row | Names the acceptance suite entry (`uc01_…`, etc.) |
| BUILD-PLAN Epics | Docs + fail-closed + UC suite stories/tasks |
| Runtime monopoly rules | Tests assert Factory cannot terminal; dry-run cannot mint |

---

## BUILD-PLAN (Work Breakdown Layer)

### BUILD-PLAN Heuristic

- **Epics E-A…E-N** map to Claim clusters / surfaces
- **Stories/tasks** carry three directives (where to look, DoD, self-check)
- **Coverage matrix** (Claim×UC×Epic) produced **before** filing dozens of tasks
- **File only what the locked plan contains** — no invented tickets

### BUILD-PLAN Sources

| Source | Role |
|--------|------|
| Spine + UC coverage matrix | Epics map to Claim clusters |
| Critique | Task size, pack share, dependencies |
| Filer | Issues + labels + Project fields (Phase / Work type / Epic id) |

### Example BUILD-PLAN Shape

```markdown
## Epic E-A: Greenfield Control Plane

**Claims:** 1, 4, 9b  
**UCs:** UC-01, UC-02  
**Phase:** Foundation  

### Stories
- **ST-A-1:** Bootstrap CLI (3 directives: founder steers → `factory init` → test uc01)
- **ST-A-2:** Config validation (3 directives: Claims 4, 9b → parse TOML → test invalid input)
```

---

## Filing (Only After Lock)

### Filer Role

**Only after founder lock:**

1. File GitHub Issues from BUILD-PLAN
2. Report plan-id → issue# map
3. **No invented tickets** (scope = locked plan only)

### Filing Report Shape

```markdown
## BUILD-PLAN-GITHUB-FILED.md

**Filed:** 2026-09-19  
**Project:** https://github.com/orgs/thalixinc/projects/40  
**Pack SHA:** abc123def  

| Plan ID | Issue # | Title |
|---------|---------|-------|
| EP-A | #1 | Epic: Greenfield Control Plane |
| ST-A-1 | #15 | Bootstrap CLI |
| ST-A-2 | #16 | Config validation |
```

---

## Coverage Matrix (Pre-Filing)

**Write the coverage matrix BEFORE drafting dozens of tasks.**

| Claims | UCs | Epics |
|--------|-----|-------|
| 1, 4, 9b | UC-01, UC-02 | E-A |
| 2, 3, 5 | UC-14, UC-15 | E-G |

This ensures:

1. No orphan Claims (every Claim → at least one UC)
2. No orphan UCs (every UC → at least one test hook)
3. No orphan Epics (every Epic → Claims + UCs)

---

## What We Would Do Differently Next Time

1. Keep **one** authoritative spine path from day one (avoid WITHDRAWN pile-up).
2. Write the **coverage matrix** (Claim×UC×Epic) **before** drafting tasks.
3. Pre-agree Project field schema (Phase / Work type / Epic id) so filing scripts don't churn.
4. Split "human build SDLC board" vs "in-product providers" earlier.
5. Capture this process doc **during** the first lock, not after filing.

---

## Self-Check (This Skill)

| Check | Status |
|-------|--------|
| Naming lock: TAP ≠ product-specific vocab | ✓ Held |
| No "already built" claims | ✓ Held — cites packs/reports only |
| Every major artifact has provenance | ✓ §2–§9 tables |
| Playbook product-agnostic | ✓ Generic recipe |

---

## Example: Codefactory (Reference)

**Product:** Chris CF / `thalixinc/codefactory-v2`  
**Project:** https://github.com/orgs/thalixinc/projects/40  

**Chronology:**

| When | Step | Outcome |
|------|------|---------|
| Early Round 4 | Hard reset: greenfield, not salvage | Old clean-pack withdrawn |
| Claim cycles 1–9 | Challenge memos → Critique → Chair LOCKED | Spine grows claim-by-claim |
| 2026-09-19 | Claim 9 founder-locked; whole spine founder-locked | Architecture freeze |
| 2026-09-19 | Day-one UC challenge cycle → fold → UC pack founder-locked | Acceptance freeze |
| 2026-09-19–20 | BUILD-PLAN draft → Critique FAIL on mega-tasks → split → PASS | First board (#1–#73) |
| 2026-09-20–21 | Factory Providers pack → Critique → founder lock → file | Second pack (#74–#81) |

**Artifacts:**

- `AUTHORITATIVE-SPINE.md` (Claims 1–9 + E1–E4)
- `DAYONE-USE-CASES.md` (UC-01…16)
- `BUILD-PLAN.md` (Epics E-A…E-N / stories / tasks)
- `BUILD-PLAN-GITHUB-FILED.md` (report: plan-id → issue#)
- `FACTORY-PROVIDERS-PACK.md` (provider virtualization SoT)
- `FACTORY-PROVIDERS-GITHUB-FILED.md` (report: #74–#81)

---

## Templates

TAP includes starter templates for:

- **`CLAIM-TEMPLATE.md`** — empty Claim table
- **`UC-TEMPLATE.md`** — empty UC writeup
- **`BUILD-PLAN-TEMPLATE.md`** — epic/story/task skeleton

See `templates/` directory in the TAP installation.

---

## Installation

Install TAP skill into your agent skill directory:

```bash
cargo install --git https://github.com/thalixinc/tap
tap install --cursor  # or --claude / --codex / --path <dir>
```

Check installation status:

```bash
tap doctor
```

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

## License

TAP is MIT licensed. See LICENSE file.

**Repository:** https://github.com/thalixinc/tap
