# BUILD-PLAN Template

Use this template to create work breakdown (epics/stories/tasks) from locked Claims + UCs.

## Build Plan Structure

```markdown
# BUILD-PLAN: [Product Name]

**Status:** [Draft / Founder-Locked]  
**Date:** [YYYY-MM-DD]  
**Source:** [AUTHORITATIVE-SPINE.md + DAYONE-USE-CASES.md]  

## Coverage Matrix (Claims × UCs × Epics)

| Claims | UCs | Epics |
|--------|-----|-------|
| 1, 4, 9b | UC-01, UC-02 | E-A |
| 2, 3 | UC-14, UC-15 | E-G |
| ... | ... | ... |

**No orphan Claims. No orphan UCs. No orphan Epics.**

---

## Epics

### Epic E-A: [Epic Title]

**Claims:** [Claim numbers covered by this epic]  
**UCs:** [UC numbers covered by this epic]  
**Phase:** [Foundation / Development / Testing / Deployment]  

**Description:** [Brief — what this epic accomplishes]

#### Stories

##### ST-A-1: [Story Title]

**Three directives:**

1. **Where to look:** [Founder steers, locked Claims, reference code]
2. **Definition of done:** [Concrete deliverable — e.g., `factory init` works, test passes]
3. **How to self-check:** [Quality gate — e.g., "uc01_bootstrap test passes"]

**Tasks:**

- **TS-A-1-1:** [Task title — concrete unit of work]
- **TS-A-1-2:** [Task title — concrete unit of work]

---

## Filing Map (Placeholder)

| Plan ID | Issue # | Title |
|---------|---------|-------|
| EP-A | TBD | Epic: [Epic Title] |
| ST-A-1 | TBD | [Story Title] |
| TS-A-1-1 | TBD | [Task Title] |

**Filled by Filer after founder lock.**
```

## Guidelines

### Coverage Matrix

**Write this BEFORE drafting tasks.** Ensures:

1. No orphan Claims (every Claim → at least one UC)
2. No orphan UCs (every UC → at least one Epic)
3. No orphan Epics (every Epic → Claims + UCs)

### Epics

- **E-A, E-B, E-C…** (or EP-A, EP-B…)
- **Theme-based** (e.g., "Greenfield Control Plane", "Learning Stub", "Observability")
- **Claims + UCs explicit**

### Stories

- **ST-A-1, ST-A-2…** (under Epic E-A)
- **Three directives on every story:**
  1. Where to look
  2. Definition of done
  3. How to self-check

### Tasks

- **TS-A-1-1, TS-A-1-2…** (under Story ST-A-1)
- **One PR per task** (or small batch)
- **Split mega-tasks** (Critique MF: task too large → split)

## Example: Real Epic

```markdown
## Epic E-A: Greenfield Control Plane

**Claims:** 1, 4, 9b  
**UCs:** UC-01, UC-02  
**Phase:** Foundation  

**Description:** Bootstrap CLI, config validation, status command. Day-one install surface.

### Stories

#### ST-A-1: Bootstrap CLI

**Three directives:**

1. **Where to look:** Founder steers (CLI-first), Claim 1 (CLI-first), UC-01 (bootstrap steps)
2. **Definition of done:** `factory init --env prod` works; config written to `~/.factory/config.toml`
3. **How to self-check:** `uc01_bootstrap` test passes

**Tasks:**

- **TS-A-1-1:** Scaffold CLI binary (clap args, version subcommand)
- **TS-A-1-2:** Implement `init` subcommand (write config TOML)
- **TS-A-1-3:** Write `uc01_bootstrap` acceptance test

---

#### ST-A-2: Config Validation

**Three directives:**

1. **Where to look:** Claims 4, 9b (config schema), UC-02 (validation steps)
2. **Definition of done:** `factory status` validates config; errors on invalid TOML
3. **How to self-check:** `uc02_config_invalid` test passes (expect error)

**Tasks:**

- **TS-A-2-1:** Define config schema (Zod or serde)
- **TS-A-2-2:** Implement `status` subcommand (read + validate config)
- **TS-A-2-3:** Write `uc02_config_invalid` acceptance test
```

## Common Anti-Patterns

- ❌ **Mega-tasks** ("Implement all UCs") → Split (one UC per story/task)
- ❌ **No three directives** → Fail Critique
- ❌ **Filing before founder lock** → Gate failure
- ❌ **Invented scope** (not in locked Claims/UCs) → Gate failure

## Coverage Check

- [ ] Coverage matrix complete (Claim×UC×Epic)
- [ ] No orphan Claims / UCs / Epics
- [ ] Every story has three directives
- [ ] Tasks sized for one PR (or small batch)
- [ ] Founder lock before filing

## Process

1. **Chair** drafts BUILD-PLAN from locked Claims + UCs
2. **Critique** flags mega-tasks, missing directives, orphan Epics
3. **Chair** revises BUILD-PLAN (split tasks, add directives)
4. **Founder** locks BUILD-PLAN
5. **Filer** creates GitHub Issues from locked plan (fills "Filing Map")
6. **Filer** reports plan-id → issue# + Project URL + SHA

## Filing Example

After Filer completes:

```markdown
## Filing Map

| Plan ID | Issue # | Title | Phase | Work Type | Epic |
|---------|---------|-------|-------|-----------|------|
| EP-A | #1 | Epic: Greenfield Control Plane | Foundation | Architecture | — |
| ST-A-1 | #15 | Bootstrap CLI | Foundation | Development | EP-A |
| TS-A-1-1 | #60 | Scaffold CLI binary | Foundation | Development | EP-A |
| TS-A-1-2 | #61 | Implement init subcommand | Foundation | Development | EP-A |
```

## Three Directives on Every Story/Task

**Template:**

1. **Where to look:** [Context — founder steers, locked Claims, reference code]
2. **Definition of done:** [Deliverable — e.g., command works, test passes, docs updated]
3. **How to self-check:** [Quality gate — e.g., test passes, lint clean, PR approved]

**Example:**

1. **Where to look:** Founder steers (CLI-first), Claim 1, UC-01
2. **Definition of done:** `factory init` writes valid config TOML
3. **How to self-check:** `uc01_bootstrap` test passes

## Phase / Work Type / Epic Fields (Example)

| Field | Options |
|-------|---------|
| **Phase** | Foundation, Development, Testing, Deployment |
| **Work type** | Architecture, Development, Testing, Documentation |
| **Epic** | EP-A, EP-B, EP-C, … |

Customize for your product.
