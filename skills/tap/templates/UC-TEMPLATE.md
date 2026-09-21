# Use Case Template

Use this template to document acceptance Use Cases (step-by-step CLI/GUI paths with test hooks).

## Use Case Format

```markdown
## UC-XX: [Title]

**Actor:** [Who performs this UC — e.g., Ops admin, Factory agent, Developer]  
**Preconditions:** [What must be true before starting — e.g., Fresh install, Harness registered]  
**Claims:** [Claim numbers this UC validates — e.g., 1, 4, 9b]  

**Steps:**
1. [Concrete action — e.g., Run `productx init --env prod`]
2. [Expected intermediate result — e.g., Config written to `~/.productx/config.toml`]
3. [Concrete action — e.g., Check `productx status`]

**Expected result:** [Final state — e.g., CLI installed, no errors; status shows "ready"]  
**Test hook:** `ucXX_[descriptive_name]` [e.g., `uc01_bootstrap`]  
**Docs:** [Section reference — e.g., §3.1 Installation]
```

## Guidelines

### Title

- **Brief** (3–5 words)
- **Action-oriented** (e.g., "Bootstrap CLI", "Run Factory Fail-Closed")
- **Unique** (UC-01, UC-02, … no duplicates)

### Actor

- **Who** performs this UC
- Examples: Ops admin, Factory agent, Developer, Chair agent, Learning system

### Preconditions

- **What must be true** before starting
- Examples: Fresh install, Harness registered, Templates available, Config valid

### Claims

- **Every UC must tag at least one Claim** (no orphan UCs)
- **Format:** `1, 4, 9b` (comma-separated Claim numbers)

### Steps

- **Concrete actions** (not slogans)
- **CLI commands** with flags (e.g., `productx run --mission abc`)
- **Expected intermediate results** (e.g., "Config written to X")

### Expected Result

- **Final state** after all steps
- **Testable** (can assert in acceptance test)

### Test Hook

- **Format:** `ucXX_[descriptive_name]`
- **Examples:** `uc01_bootstrap`, `uc07_run_fail_closed`, `uc14_train_stub`
- **If no test hook, UC is not done** (fail Critique)

### Docs

- **Section reference** (e.g., §3.1, §5.3)
- **Or:** "TBD" if docs not written yet

## Example: Real Use Cases

### Example 1: Bootstrap (Platform)

```markdown
## UC-01: Bootstrap CLI

**Actor:** Ops admin  
**Preconditions:** Fresh install  
**Claims:** 1, 4, 9b  

**Steps:**
1. Run `productx init --env prod`
2. Verify config written to `~/.productx/config.toml`
3. Check `productx status` shows "ready"

**Expected result:** CLI installed, no errors  
**Test hook:** `uc01_bootstrap`  
**Docs:** §3.1 Installation
```

### Example 2: Runtime Monopoly (Runtime)

```markdown
## UC-07: Run Factory (Fail-Closed)

**Actor:** Factory agent  
**Preconditions:** Harness registered, templates available  
**Claims:** 1, 4, 9  

**Steps:**
1. Factory agent calls `productx run --mission abc`
2. System mints new run (non-terminal)
3. Run executes, produces digest
4. Appender observes digest, decides terminal/non-terminal

**Expected result:** Factory cannot set terminal=true (monopoly enforced)  
**Test hook:** `uc07_run_fail_closed`  
**Docs:** §5.3 Runtime Monopoly
```

### Example 3: Learning Stub (Day-One Cut)

```markdown
## UC-14: Propose Learning (Stub Only)

**Actor:** Learning system  
**Preconditions:** Run completed, digest available  
**Claims:** 2, 3  

**Steps:**
1. Learning system analyzes digest
2. Learning system proposes rule update
3. System logs proposal (does NOT auto-promote)

**Expected result:** Proposal logged, human review required (no auto-promote)  
**Test hook:** `uc14_train_stub`  
**Docs:** §6.2 Learning Gate
```

## Common Anti-Patterns

- ❌ **"Support teams"** → Slogan (not steps)
- ❌ **UC with no Claims tagged** → Orphan UC (fail Critique)
- ❌ **UC with no test hook** → Not done (fail Critique)
- ❌ **Steps too vague** ("User does X") → Be concrete (CLI command with flags)

## Coverage Check

- [ ] Every UC tags at least one Claim
- [ ] Every UC has test hook (format: `ucXX_name`)
- [ ] Steps are concrete (CLI commands, not slogans)
- [ ] Actor and preconditions clear
- [ ] Expected result testable

## Process

1. **Platform + Runtime** propose UCs independently (tag Claims)
2. **Critique** flags orphan UCs (no Claims), missing test hooks, vague steps
3. **Chair** folds Platform + Runtime into single UC pack
4. **Founder** locks UC pack (acceptance freeze)
5. **BUILD-PLAN** maps UCs to epics/stories/tasks
6. **Filer** creates Issues (only after founder lock)
