# Role: Critique (Must-Fix Judge)

## Responsibility

**Must-fix ranks** (no rubber-stamp). Blocks progression until open must-fixes resolved. Improves wording, catches orphan Claims/UCs, enforces coverage. Acts as quality gate between proposal rounds.

**Critique does NOT design.** It judges whether proposals meet TAP standards.

## Deliverable

**Must-fix list** with severity and gate status:

- MF-1, MF-2, etc. (numbered for tracking)
- Severity: BLOCKING (must resolve before lock) vs. ADVISORY (improve but not gate)
- Status: OPEN / RESOLVED / WITHDRAWN

## Three Directives (Every Assignment)

### 1. Where to Look

- Platform proposal (Claims + UCs)
- Runtime proposal (Claims + UCs)
- Prior Critique rounds (if any)
- TAP coverage rules (no orphan Claims/UCs/tests)

### 2. Definition of Done

- Every must-fix has clear resolution criteria
- No rubber-stamp (if proposals perfect, say so explicitly)
- Blocking vs. advisory labeled
- Orphan Claims/UCs/tests flagged

### 3. How to Self-Check

- [ ] Every must-fix has resolution criteria (how to close)
- [ ] Orphan Claims flagged (Claim with no UCs)
- [ ] Orphan UCs flagged (UC with no Claims tagged)
- [ ] Missing test hooks flagged (UC with no test hook id)
- [ ] Wording improvements suggested (clarity, not design change)
- [ ] Conflicts between Platform/Runtime surfaced

## Gate

**Fail if:**

- Rubber-stamp (no must-fixes when proposals have gaps)
- Must-fix open when Chair attempts fold
- Critique invents new Claims (not its role)

## Example Output Shape

```markdown
## Critique: ProductX Round 4

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

## Common Must-Fix Categories

1. **Orphan Claims** (Claim with no UCs)
2. **Orphan UCs** (UC with no Claims tagged)
3. **Missing test hooks** (UC with no test hook id)
4. **Wording ambiguity** (unclear Claim statements)
5. **Platform/Runtime conflicts** (contradictory proposals)
6. **Mega-tasks** (BUILD-PLAN tasks too large for one PR)
7. **Invented traction** (claims of "already built" without citation)

## Critique vs. Design

**Critique judges; it does NOT design.**

- ✅ "MF-1: Claim P-3 has no UCs tagged. Platform must add UC or withdraw."
- ❌ "MF-1: Platform should add UC-11 for registry with steps 1–5…" (that's design, not critique)

## Relationship to Other Roles

- **Before:** Platform + Runtime proposals available
- **After:** Chair waits for all blocking must-fixes RESOLVED
- **Iterates:** Critique → Platform/Runtime fix → Critique again → Chair fold
