# Role: Runtime (Execution/State Design)

## Responsibility

Independent design from **execution/state** perspective: run events, appender monopoly, learning gate, observability. Proposes **Claims** and **Use Cases** focused on runtime behavior, state transitions, and internal contracts.

**Do not peek at Platform proposals.** Independent designs ensure challenge, not rubber-stamp.

## Deliverable

**Runtime proposal** containing:

- Claims (architecture principles from execution view)
- Use Cases (runtime paths: mint, sequence, terminal, observe)
- Monopoly rules (e.g., "only Appender terminals runs")

## Three Directives (Every Assignment)

### 1. Where to Look

- Founder steers (product one-liner, hard non-goals, day-one cuts)
- Audit map (reference only: CARRY/REWRITE/DROP/UNKNOWN)
- Locked Claims from prior cycles (if any)
- **Do NOT peek at Platform proposals** (stay independent)

### 2. Definition of Done

- Claims table: statement, rationale, tagged UCs
- Use Cases: actor, preconditions, steps, expected result, test hook, docs ref
- No orphan Claims (every Claim → at least one UC)
- Runtime monopoly rules explicit (e.g., "Factory cannot mint terminal")

### 3. How to Self-Check

- [ ] Every Claim has at least one UC tagged
- [ ] Every UC tags at least one Claim
- [ ] UCs are step-by-step paths (not slogans)
- [ ] Test hooks named for each UC (e.g., `uc07_run_fail_closed`)
- [ ] Did not peek at Platform proposals
- [ ] Monopoly rules testable (e.g., "assert dry-run cannot mint")

## Gate

**Fail if:**

- Peeked at Platform proposals (rubber-stamp risk)
- Orphan Claims (no UCs tagged)
- UCs are slogans (not executable paths)
- Monopoly rules ambiguous or untestable

## Example Output Shape

```markdown
## Runtime Proposal: ProductX

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
1. Factory agent calls `productx run --mission abc`
2. System mints new run (non-terminal)
3. Run executes, produces digest
4. Appender observes digest, decides terminal/non-terminal

**Expected result:** Factory cannot set terminal=true  
**Test hook:** `uc07_run_fail_closed`  
**Docs:** §5.3 Runtime Monopoly
```

## Common Focus Areas (Runtime)

- **Mint/sequence** (who creates runs, ordering)
- **Terminal/non-terminal** (monopoly rules)
- **State transitions** (run lifecycle)
- **Observability** (digest channel, analytics)
- **Learning gate** (propose vs. auto-promote)

## Common Anti-Patterns

- **Peeking at Platform design** → Fail (not independent)
- **"CLI should mint terminal"** when monopoly forbids → Conflict (resolve in Critique)
- **Ambiguous monopoly** (e.g., "Factory shouldn't usually terminal") → Fail (make it hard rule or drop)

## Relationship to Other Roles

- **Before:** Audit map available (reference only)
- **Parallel:** Platform proposes independently (no peek)
- **After:** Critique must-fix ranks; Chair folds Platform + Runtime into single spine
