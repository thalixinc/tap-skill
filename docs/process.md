# TAP Process Deep Dive

This document provides a detailed walkthrough of the TAP (Traceable Acceptance Pack) process.

---

## Table of Contents

1. [Process Overview](#process-overview)
2. [Inputs from Founder](#inputs-from-founder)
3. [Audit Phase](#audit-phase)
4. [Platform Design](#platform-design)
5. [Runtime Design](#runtime-design)
6. [Critique Phase](#critique-phase)
7. [Chair Fold](#chair-fold)
8. [Founder Lock](#founder-lock)
9. [BUILD-PLAN](#build-plan)
10. [Filing](#filing)
11. [Second Packs](#second-packs)
12. [Coverage Matrix](#coverage-matrix)
13. [Common Anti-Patterns](#common-anti-patterns)
14. [Lessons Learned](#lessons-learned)

---

## Process Overview

TAP follows a sequential **counsel pattern** with quality gates:

```
Founder steer → Audit → Platform + Runtime (independent) → Critique → Chair → Founder lock → File
```

**Key principles:**

- **Independent designs** (Platform and Runtime do not peek at each other)
- **Must-fix gates** (Critique blocks progression until resolved)
- **Founder lock required** before filing Issues
- **Three directives** on every assignment (where to look, DoD, self-check)

---

## Inputs from Founder

Before starting TAP, gather these inputs from the founder (or product authority):

### 1. Product One-Liner + Hard Non-Goals

**What:** Brief product vision and explicit non-goals.

**Example:**

- **Product:** "Assembly-line AI factory for training, execution, observability"
- **Non-goals:** "No GUI (CLI-first); no auto-promote training (human-in-loop)"

**Why:** Prevents scope creep and design drift.

### 2. Legacy Code Status

**What:** Is legacy code **reference** or **source of truth (SoT)**?

**Example:**

- **Codefactory:** "v1 repo is reference only; greenfield v2 from scratch"
- **ProductX:** "v1 CLI is SoT; migrate tests as-is"

**Why:** Audit needs to know whether to CARRY, REWRITE, or DROP.

### 3. Day-One Cuts

**What:** What's in scope for day-one acceptance paths?

**Example:**

- **In scope:** Install, templates, crew, run, observe, train stub
- **Out of scope:** Auto-promote learning, multi-org support, GUI

**Why:** UCs must cover day-one basics; defer later UCs explicitly.

### 4. Who Locks Packs

**What:** Who has authority to lock Claims, UCs, BUILD-PLAN?

**Example:**

- **Codefactory:** "Founder via FCoA (Founder's Chief of AI)"
- **ProductX:** "Product owner + CTO"

**Why:** Prevents premature filing or scope expansion.

### 5. When Filing Is Allowed

**What:** When can Filer create GitHub Issues?

**Example:**

- **Codefactory:** "Only after founder lock on BUILD-PLAN"
- **ProductX:** "After PM approval + QA green light"

**Why:** Prevents "filing to get ahead" before design locked.

---

## Audit Phase

**Role:** Audit (Current-State Inventory)

**Deliverable:** Audit map (CARRY / REWRITE / DROP / UNKNOWN)

### Process

1. **Review legacy code** (if exists)
2. **Classify components:**
   - **CARRY:** Intent + implementation solid; keep as-is
   - **REWRITE:** Intent solid; implementation replaced
   - **DROP:** Out of scope or superseded
   - **UNKNOWN:** No legacy exists
3. **Cite paths/issues** or mark UNKNOWN
4. **Label legacy as reference-only** (if founder specified)

### Example Audit Output

```markdown
## Current-State Audit: Codefactory v1

**Reference repository:** `thalixinc/codefactory-v1` (reference only, not SoT)

| Component | Status | Path/Issue | Notes |
|-----------|--------|------------|-------|
| CLI bootstrap | CARRY | `src/cli/init.rs` | Intent solid; may refactor |
| Config parser | REWRITE | `src/config.rs` | Uses deprecated TOML lib |
| Mission object | DROP | `src/mission.rs` | Founder forbade Mission abstraction |
| Learning gate | UNKNOWN | — | No v1 implementation |
```

### Gate

**Fail if:**

- Invented features (not in legacy or founder steers)
- Vocab bleed from other products (e.g., SP Subway terms in CF audit)
- Salvage treated as SoT without explicit label

---

## Platform Design

**Role:** Platform (Surface/Interface Design)

**Deliverable:** Platform proposal (Claims + UCs)

### Process

1. **Read founder steers + Audit map** (reference only)
2. **Propose Claims** from surface/interface perspective
3. **Propose UCs** (CLI-shaped acceptance paths)
4. **Tag Claims in every UC**
5. **Do NOT peek at Runtime proposals** (stay independent)

### Example Platform Output

```markdown
## Platform Proposal: Codefactory v2

### Claims

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| P-1 | CLI-first, GUI optional | Founder steer: agents + ops scripts | UC-01, UC-02, UC-03 |
| P-2 | Templates as first-class artifacts | Pack reuse across products | UC-04, UC-05 |

### Use Cases

#### UC-01: Bootstrap CLI

**Actor:** Ops admin  
**Preconditions:** Fresh install  
**Claims:** P-1  
**Steps:**
1. Run `factory init --env prod`
2. Verify config written to `~/.factory/config.toml`
3. Check `factory status` shows "ready"

**Expected result:** CLI installed, no errors  
**Test hook:** `uc01_bootstrap`  
**Docs:** §3.1 Installation
```

### Gate

**Fail if:**

- Peeked at Runtime proposals (rubber-stamp risk)
- Orphan Claims (no UCs tagged)
- UCs are slogans (not executable paths)

---

## Runtime Design

**Role:** Runtime (Execution/State Design)

**Deliverable:** Runtime proposal (Claims + UCs)

### Process

1. **Read founder steers + Audit map** (reference only)
2. **Propose Claims** from execution/state perspective
3. **Propose UCs** (runtime paths: mint, sequence, terminal)
4. **Tag Claims in every UC**
5. **Do NOT peek at Platform proposals** (stay independent)

### Example Runtime Output

```markdown
## Runtime Proposal: Codefactory v2

### Claims

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| R-1 | Factory must not mint terminal runs | Runtime monopoly: only Appender terminals | UC-07, UC-13 |
| R-2 | Learning gate: propose only, no auto-promote | Founder steer: human-in-loop training | UC-14, UC-16 |

### Use Cases

#### UC-07: Run Factory (Fail-Closed)

**Actor:** Factory agent  
**Preconditions:** Harness registered, templates available  
**Claims:** R-1  
**Steps:**
1. Factory agent calls `factory run --mission abc`
2. System mints new run (non-terminal)
3. Run executes, produces digest
4. Appender observes digest, decides terminal/non-terminal

**Expected result:** Factory cannot set terminal=true  
**Test hook:** `uc07_run_fail_closed`  
**Docs:** §5.3 Runtime Monopoly
```

### Gate

**Fail if:**

- Peeked at Platform proposals (rubber-stamp risk)
- Orphan Claims (no UCs tagged)
- Monopoly rules ambiguous or untestable

---

## Critique Phase

**Role:** Critique (Must-Fix Judge)

**Deliverable:** Must-fix list (MF-1, MF-2, etc.)

### Process

1. **Read Platform + Runtime proposals**
2. **Flag orphan Claims** (Claim with no UCs)
3. **Flag orphan UCs** (UC with no Claims tagged)
4. **Flag missing test hooks** (UC with no test hook id)
5. **Surface conflicts** (Platform vs. Runtime contradictions)
6. **Improve wording** (clarity, not design change)
7. **Block if must-fixes open**

### Example Critique Output

```markdown
## Critique: Codefactory v2 Round 4

### Must-Fixes (Blocking)

**MF-1: Orphan Claim P-3**  
**Status:** OPEN  
**Issue:** Claim P-3 "Templates registry" has no UCs tagged.  
**Resolution:** Platform adds UC-11 (registry lookup) or withdraws P-3.

**MF-2: Missing test hook UC-07**  
**Status:** OPEN  
**Issue:** UC-07 has no test hook id.  
**Resolution:** Runtime adds test hook `uc07_run_fail_closed`.

### Must-Fixes (Advisory)

**MF-3: Wording clarity Claim R-1**  
**Status:** OPEN  
**Issue:** "Factory must not mint terminal runs" — clarify "terminal" definition.  
**Resolution:** Runtime adds footnote or docs ref for "terminal=true semantics".

### Conflicts

**Platform P-1 vs. Runtime R-2:**  
Platform proposes CLI auto-promotes; Runtime proposes human-in-loop gate.  
**Resolution:** Chair fold; founder decides.
```

### Gate

**Fail if:**

- Rubber-stamp (no must-fixes when proposals have gaps)
- Must-fix open when Chair attempts fold
- Critique invents new Claims (not its role)

---

## Chair Fold

**Role:** Chair (Fold & Synthesize)

**Deliverable:** Single authoritative pack file (Claims + UCs)

### Process

1. **Wait for Critique RESOLVED** (all blocking must-fixes closed)
2. **Merge Platform + Runtime Claims** (deduplicate, reconcile conflicts)
3. **Number Claims** (1, 2, 3… or P-1, R-1, etc.)
4. **Build coverage matrix** (Claim×UC)
5. **Check for orphans** (Claim with no UCs, UC with no Claims)
6. **Mark superseded docs WITHDRAWN**
7. **Submit to founder for lock**

### Example Chair Output

```markdown
# AUTHORITATIVE-SPINE.md

**Status:** Chair synthesis for founder lock  
**Date:** 2026-09-21  
**Supersedes:** `platform-proposal-round4.md`, `runtime-proposal-round4.md`  

## Claims

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 1 | CLI-first, GUI optional | Founder steer + Platform P-1 | UC-01, UC-02, UC-03 |
| 2 | Factory must not mint terminal runs | Runtime monopoly R-1 | UC-07, UC-13 |
| 3 | Templates as first-class artifacts | Platform P-2 | UC-04, UC-05 |

## Coverage Matrix (Claim×UC)

| Claims | UCs |
|--------|-----|
| 1 | UC-01, UC-02, UC-03 |
| 2 | UC-07, UC-13 |
| 3 | UC-04, UC-05 |

**No orphan Claims. No orphan UCs.**
```

### Gate

**Fail if:**

- Critique blocking must-fixes still OPEN
- Multiple competing "authoritative" docs
- Orphan Claims/UCs in coverage matrix

---

## Founder Lock

**Authority:** Founder (or product authority)

**Decision:** Lock or reject the pack.

### Process

1. **Review Chair pack** (spine + UCs)
2. **Check coverage matrix** (no orphans)
3. **Verify founder steers respected** (hard non-goals, day-one cuts)
4. **Lock or reject:**
   - **Lock:** Architecture freeze (proceed to BUILD-PLAN)
   - **Reject:** Specify changes; return to Platform/Runtime/Critique

### Lock Signal

```markdown
**Status:** Founder-locked 2026-09-19 14:30 ET
```

Or commit message:

```
chore: founder lock AUTHORITATIVE-SPINE + DAYONE-USE-CASES

Signed-off-by: Founder <founder@example.com>
```

### Gate

**Do NOT proceed to BUILD-PLAN or filing until founder lock confirmed.**

---

## BUILD-PLAN

**Role:** Chair (Work Breakdown)

**Deliverable:** BUILD-PLAN (epics/stories/tasks + coverage matrix)

### Process

1. **Read locked Claims + UCs**
2. **Draft epics** (E-A, E-B, … or EP-A, EP-B, …)
3. **Map epics to Claim clusters**
4. **Draft stories** (under each epic)
5. **Add three directives** to every story (where to look, DoD, self-check)
6. **Draft tasks** (under each story)
7. **Build coverage matrix** (Claim×UC×Epic) **before filing**
8. **Submit to Critique** (check for mega-tasks, missing directives)
9. **Revise** (split mega-tasks, add directives)
10. **Submit to founder for lock**

### Example BUILD-PLAN

```markdown
# BUILD-PLAN: Codefactory v2

**Status:** Draft  
**Date:** 2026-09-19  

## Coverage Matrix (Claim×UC×Epic)

| Claims | UCs | Epics |
|--------|-----|-------|
| 1, 4, 9b | UC-01, UC-02 | E-A |
| 2, 3 | UC-14, UC-15 | E-G |

---

## Epic E-A: Greenfield Control Plane

**Claims:** 1, 4, 9b  
**UCs:** UC-01, UC-02  
**Phase:** Foundation  

### Stories

#### ST-A-1: Bootstrap CLI

**Three directives:**

1. **Where to look:** Founder steers (CLI-first), Claim 1, UC-01
2. **Definition of done:** `factory init` writes valid config TOML
3. **How to self-check:** `uc01_bootstrap` test passes

**Tasks:**

- **TS-A-1-1:** Scaffold CLI binary (clap args)
- **TS-A-1-2:** Implement `init` subcommand
- **TS-A-1-3:** Write `uc01_bootstrap` test
```

### Critique Check

Critique reviews BUILD-PLAN for:

- **Mega-tasks** (task too large → split)
- **Missing directives** (no three directives → add)
- **Orphan epics** (epic not in coverage matrix → add or drop)

### Founder Lock

After Critique PASS:

```markdown
**Status:** Founder-locked 2026-09-19 18:00 ET
```

---

## Filing

**Role:** Filer (GitHub Issues from Locked Plan)

**Deliverable:** Filing report (plan-id → issue# map)

### Process

1. **Confirm founder lock** (BUILD-PLAN has lock timestamp)
2. **Read BUILD-PLAN** (epics EP-*, stories ST-*, tasks TS-*)
3. **Create Issues** (one per plan id)
4. **Set Project fields** (Phase, Work type, Epic id)
5. **Report plan-id → issue#**
6. **Commit report to repo** (SHA in filing report)

### Example Filing Commands

```bash
# Epic
gh issue create --title "Epic: Greenfield Control Plane" \
  --body-file epic-a.md \
  --label epic --label foundation \
  --project 40

# Story
gh issue create --title "Bootstrap CLI" \
  --body "## Story\n\n...\n\n## Epic\n\n#1" \
  --label story --label foundation \
  --project 40
```

### Example Filing Report

```markdown
# BUILD-PLAN-GITHUB-FILED.md

**Filed:** 2026-09-19  
**Project:** https://github.com/orgs/thalixinc/projects/40  
**Pack SHA:** abc123def456  

| Plan ID | Issue # | Title | Phase | Work Type | Epic |
|---------|---------|-------|-------|-----------|------|
| EP-A | #1 | Epic: Greenfield Control Plane | Foundation | Architecture | — |
| ST-A-1 | #15 | Bootstrap CLI | Foundation | Development | EP-A |
| TS-A-1-1 | #60 | Scaffold CLI binary | Foundation | Development | EP-A |

**Total:** 73 issues filed
```

### Gate

**Fail if:**

- Filing before founder lock
- Scope beyond locked plan (invented tickets)
- Missing plan-id → issue# map

---

## Second Packs

**Same process:** counsel→lock→file.

If a second pack (e.g., Factory Providers) is needed after the first board:

1. **Founder steers** second pack scope
2. **Audit** (if relevant)
3. **Platform + Runtime** propose (independent)
4. **Critique** must-fix
5. **Chair** fold
6. **Founder lock** second pack
7. **Filer** creates Issues (extend Project or new Project)

**Example:** Codefactory filed #1–#73 from BUILD-PLAN, then #74–#81 from FACTORY-PROVIDERS-PACK.

---

## Coverage Matrix

**Write this BEFORE filing tasks.**

### Claim×UC

| Claims | UCs |
|--------|-----|
| 1, 4, 9b | UC-01, UC-02 |
| 2, 3, 5 | UC-14, UC-15 |

**Check:** No orphan Claims (every Claim → at least one UC).

### Claim×UC×Epic

| Claims | UCs | Epics |
|--------|-----|-------|
| 1, 4, 9b | UC-01, UC-02 | E-A |
| 2, 3, 5 | UC-14, UC-15 | E-G |

**Check:** No orphan Claims / UCs / Epics.

### Benefits

1. **No orphan Claims** (every Claim validated by at least one UC)
2. **No orphan UCs** (every UC maps to BUILD-PLAN epic)
3. **No orphan Epics** (every Epic traces to Claims + UCs)
4. **Traceability** (bidirectional links: Claim ↔ UC ↔ Epic ↔ Issue)

---

## Common Anti-Patterns

### 1. Filing Before Lock

**Anti-pattern:** "Let's file Issues now to get ahead."

**Why fail:** Scope not frozen; will churn when founder rejects.

**Fix:** Wait for founder lock on BUILD-PLAN.

### 2. Peeking (Platform ↔ Runtime)

**Anti-pattern:** Platform reads Runtime proposal before submitting.

**Why fail:** Rubber-stamp risk; no independent challenge.

**Fix:** Platform and Runtime work in isolation; Chair folds after Critique.

### 3. Orphan Claims

**Anti-pattern:** Claim P-3 has no UCs tagged.

**Why fail:** Untestable; no acceptance path.

**Fix:** Critique flags MF; Platform adds UC or withdraws Claim.

### 4. Mega-Tasks

**Anti-pattern:** "TS-A-1: Implement all UCs" (one task, 50 files).

**Why fail:** Too large for one PR; blocks parallel work.

**Fix:** Critique flags MF; Chair splits into TS-A-1-1, TS-A-1-2, etc.

### 5. Invented Traction

**Anti-pattern:** "We already have X" (without citation).

**Why fail:** Confuses reference code with implemented v2.

**Fix:** Audit cites paths/issues or marks UNKNOWN.

### 6. Vocab Bleed

**Anti-pattern:** CF audit cites "SP Subway mission objects."

**Why fail:** Wrong product; confuses counsel.

**Fix:** Respect naming lock (Chris CF ≠ SP Subway).

---

## Lessons Learned

### 1. One Authoritative Spine

**Problem:** Multiple competing "spine" docs (WITHDRAWN pile-up).

**Fix:** Chair withdraws superseded docs; single `AUTHORITATIVE-SPINE.md`.

### 2. Coverage Matrix First

**Problem:** Filed 50 tasks before realizing orphan Claims.

**Fix:** Write Claim×UC×Epic matrix **before** drafting tasks.

### 3. Pre-Agree Project Fields

**Problem:** Filed Issues with wrong labels; had to re-file.

**Fix:** Lock Project field schema (Phase / Work type / Epic id) before filing.

### 4. Split SDLC vs. Providers

**Problem:** Mixed "human build board" tasks with "in-product providers."

**Fix:** First pack = SDLC board; second pack = providers (separate but same pattern).

### 5. Document During Lock

**Problem:** Wrote this process doc **after** filing; lost context.

**Fix:** Capture TAP process **during** first lock cycle, not after.

---

## Summary

TAP process:

1. **Founder steer** → inputs (one-liner, hard non-goals, day-one cuts)
2. **Audit** → CARRY/REWRITE/DROP/UNKNOWN
3. **Platform + Runtime** → independent Claims + UCs
4. **Critique** → must-fix ranks (no rubber-stamp)
5. **Chair** → fold into single spine + coverage matrix
6. **Founder lock** → architecture freeze
7. **BUILD-PLAN** → epics/stories/tasks + three directives
8. **Founder lock** → work breakdown freeze
9. **Filer** → GitHub Issues (only after lock)
10. **Optional second packs** → same counsel→lock→file loop

**Key gates:**

- No peeking (Platform ↔ Runtime)
- No filing before founder lock
- Coverage matrix before tasks
- Three directives on every story

**Result:** Traceable Claims ↔ UCs ↔ Tests ↔ Issues.
