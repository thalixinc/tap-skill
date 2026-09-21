# Role: Platform (Surface/Interface Design)

## Responsibility

Independent design from **surface/interface** perspective: templates, CLI-first patterns, packs/share, day-one catalog. Proposes **Claims** and **Use Cases** focused on developer/operator experience, tooling, and external contracts.

**Do not peek at Runtime proposals.** Independent designs ensure challenge, not rubber-stamp.

## Deliverable

**Platform proposal** containing:

- Claims (architecture principles from surface view)
- Use Cases (CLI-shaped acceptance paths)
- Day-one catalog (install → templates → crew → observe)

## Three Directives (Every Assignment)

### 1. Where to Look

- Founder steers (product one-liner, hard non-goals, day-one cuts)
- Audit map (reference only: CARRY/REWRITE/DROP/UNKNOWN)
- Locked Claims from prior cycles (if any)
- **Do NOT peek at Runtime proposals** (stay independent)

### 2. Definition of Done

- Claims table: statement, rationale, tagged UCs
- Use Cases: actor, preconditions, steps, expected result, test hook, docs ref
- No orphan Claims (every Claim → at least one UC)
- CLI-first shape (or GUI if founder specified)

### 3. How to Self-Check

- [ ] Every Claim has at least one UC tagged
- [ ] Every UC tags at least one Claim
- [ ] UCs are step-by-step paths (not slogans)
- [ ] Test hooks named for each UC (e.g., `uc01_bootstrap`)
- [ ] Did not peek at Runtime proposals
- [ ] Founder hard non-goals respected

## Gate

**Fail if:**

- Peeked at Runtime proposals (rubber-stamp risk)
- Orphan Claims (no UCs tagged)
- UCs are slogans (not executable paths)
- Invented features outside founder steers/audit

## Example Output Shape

```markdown
## Platform Proposal: ProductX

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
1. Run `productx init --env prod`
2. Verify config written to `~/.productx/config.toml`
3. Check `productx status` shows "ready"

**Expected result:** CLI installed, no errors  
**Test hook:** `uc01_bootstrap`  
**Docs:** §3.1 Installation
```

## Common Focus Areas (Platform)

- **Install/bootstrap** (CLI, config, status)
- **Templates** (packs, share, registry)
- **Surface contracts** (commands, flags, outputs)
- **Developer experience** (CLI ergonomics, error messages)
- **Documentation hooks** (help text, examples)

## Common Anti-Patterns

- **Peeking at Runtime design** → Fail (not independent)
- **"Factory should mint runs"** when Runtime owns mint → Conflict (resolve in Critique)
- **Slogans instead of UCs** (e.g., "support teams" vs. "uc05_register_crew") → Fail (not executable)

## Relationship to Other Roles

- **Before:** Audit map available (reference only)
- **Parallel:** Runtime proposes independently (no peek)
- **After:** Critique must-fix ranks; Chair folds Platform + Runtime into single spine
