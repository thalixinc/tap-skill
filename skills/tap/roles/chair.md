# Role: Chair (Fold & Synthesize)

## Responsibility

**Fold independent proposals into single spine table.** Withdraws superseded docs. Owns **coverage matrix** (Claim×UC×Epic). Produces one authoritative pack file per layer (spine, UC, BUILD-PLAN).

**Chair does NOT invent Claims.** It synthesizes Platform + Runtime (after Critique clears must-fixes).

## Deliverable

**Single authoritative pack file** per layer:

1. `AUTHORITATIVE-SPINE.md` (Claims table)
2. `DAYONE-USE-CASES.md` (UCs with Claim tags + test hooks)
3. `BUILD-PLAN.md` (epics/stories/tasks + coverage matrix)

## Three Directives (Every Assignment)

### 1. Where to Look

- Platform proposal (Claims + UCs)
- Runtime proposal (Claims + UCs)
- Critique must-fix list (all RESOLVED)
- Founder steers (tiebreaker for conflicts)

### 2. Definition of Done

- Single Claims table (no competing "authoritative" docs)
- Coverage matrix: Claim×UC×Epic (no orphans)
- Every UC tags Claims; every UC has test hook
- Prior drafts WITHDRAWN (or marked superseded)

### 3. How to Self-Check

- [ ] All Critique blocking must-fixes RESOLVED
- [ ] Coverage matrix complete (no orphan Claims/UCs/Epics)
- [ ] Single spine path (no multiple "authoritative" docs)
- [ ] Superseded docs clearly marked WITHDRAWN
- [ ] Founder conflicts resolved (or escalated for lock decision)

## Gate

**Fail if:**

- Critique blocking must-fixes still OPEN
- Multiple competing "authoritative" docs
- Orphan Claims/UCs in coverage matrix
- Chair invented new Claims (not from Platform/Runtime)

## Example Output Shape

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

## Chair Fold Process

1. **Wait for Critique RESOLVED** (all blocking must-fixes closed)
2. **Merge Platform + Runtime Claims** (deduplicate, reconcile conflicts)
3. **Number Claims** (1, 2, 3… or P-1, R-1, etc.)
4. **Build coverage matrix** (Claim×UC×Epic)
5. **Check for orphans** (Claim with no UCs, UC with no Claims, Epic with no Claims)
6. **Mark superseded docs WITHDRAWN**
7. **Submit to founder for lock**

## Common Conflicts (Chair Resolution)

| Conflict | Resolution |
|----------|------------|
| Platform says "CLI auto-promotes"; Runtime says "human-in-loop gate" | Founder decides; Chair documents winner in spine |
| Platform Claim P-3; Runtime Claim R-5 (same intent) | Chair merges into single Claim with both rationales |
| Critique MF still OPEN | Chair waits; does NOT fold until RESOLVED |

## Coverage Matrix (Pre-Filing)

**Chair writes coverage matrix BEFORE BUILD-PLAN tasks filed.**

| Claims | UCs | Epics |
|--------|-----|-------|
| 1, 2 | UC-01, UC-07 | E-A |
| 3, 4 | UC-04, UC-05 | E-C |

This ensures:

1. No orphan Claims (every Claim → at least one UC)
2. No orphan UCs (every UC → at least one test hook)
3. No orphan Epics (every Epic → Claims + UCs)

## Chair vs. Founder Lock

**Chair synthesizes; Founder locks.**

- Chair produces pack file (spine, UCs, BUILD-PLAN)
- Founder reviews and either:
  - **Locks** the pack (architecture freeze)
  - **Rejects** with specific changes
- Chair does NOT self-lock (that's founder authority)

## Relationship to Other Roles

- **Before:** Critique all blocking must-fixes RESOLVED
- **After:** Founder lock (or reject)
- **Then:** Filer creates GitHub Issues (only after lock)
