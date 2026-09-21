# Claims Template

Use this template to document architecture Claims (locked principles / technical contracts).

## Claims Table

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 1 | [Brief, testable assertion] | [Why this matters; founder steer or design decision] | UC-XX, UC-YY |
| 2 | [Brief, testable assertion] | [Why this matters; founder steer or design decision] | UC-ZZ |

## Guidelines

### Statement

- **Brief** (one sentence)
- **Testable** (can write acceptance test against it)
- **Durable** (small contracts, not frameworks)
- **Specific** (avoid "should usually" — make hard rules or drop)

### Rationale

- **Why** this Claim matters
- **Source:** founder steer, Platform/Runtime proposal, Audit CARRY
- **Trade-offs** (if any)

### Tagged UCs

- **Every Claim must tag at least one UC** (no orphan Claims)
- **Format:** `UC-01, UC-02` (comma-separated)

## Example: Real Claims

### Example 1: Runtime Monopoly

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 1 | Factory must not mint terminal runs | Runtime monopoly: only Appender terminals runs to prevent factory self-approval | UC-07, UC-13 |

### Example 2: Founder Steer

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 2 | RuleSets are missions (no Mission object) | Founder lock: simplicity over abstraction; avoid Mission-like inheritance | UC-05, UC-14 |

### Example 3: Platform Design

| Claim | Statement | Rationale | Tagged UCs |
|-------|-----------|-----------|------------|
| 3 | Templates as first-class artifacts | Pack reuse across products; registry lookup | UC-04, UC-05, UC-11 |

## Common Anti-Patterns

- ❌ **"Factory should usually not mint terminal runs"** → Too vague (make hard rule)
- ❌ **"Support good UX"** → Slogan, not testable
- ❌ **Claim with no UCs tagged** → Orphan Claim (fail Critique)
- ❌ **Claim invented after founder lock** → Out of scope (file as follow-up pack)

## Coverage Check

- [ ] Every Claim has at least one UC tagged
- [ ] Every UC tags at least one Claim (see UC template)
- [ ] No "should usually" (make hard rules)
- [ ] Founder hard non-goals respected (e.g., no Mission object if founder forbade)

## Process

1. **Platform + Runtime** propose Claims independently
2. **Critique** flags orphan Claims (no UCs) or ambiguous wording
3. **Chair** folds Platform + Runtime into single Claims table
4. **Founder** locks (or rejects with changes)
5. **Only after lock:** Filer creates Issues from BUILD-PLAN
