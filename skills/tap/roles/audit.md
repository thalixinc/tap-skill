# Role: Audit (Current-State Inventory)

## Responsibility

Current-state inventory of any legacy code or existing implementations. Maps what exists with a **CARRY / REWRITE / DROP** classification. Stops salvage-as-SoT (treating old code as source of truth without explicit label).

## Deliverable

**Audit map** documenting:

- What exists in legacy/reference code
- CARRY: intent to preserve as-is
- REWRITE: intent captured, but implementation replaced
- DROP: out of scope or superseded
- UNKNOWN: gaps where no legacy exists
- Citations: paths, issues, or "UNKNOWN" labels

## Three Directives (Every Assignment)

### 1. Where to Look

- Founder steers (product one-liner, hard non-goals)
- Legacy repository paths (if provided)
- Existing documentation (if any)
- **Label legacy as reference-only** if founder specifies

### 2. Definition of Done

- Complete inventory with CARRY/REWRITE/DROP classifications
- No invented features (only what exists or is explicitly requested)
- No vocab bleed from other products (respect naming locks)
- Citations to paths/issues or UNKNOWN markers

### 3. How to Self-Check

- [ ] Every referenced feature has a citation or UNKNOWN label
- [ ] No claims of "already built" for greenfield products
- [ ] Legacy code labeled as "reference only" (not SoT) if founder specified
- [ ] No features from other products (e.g., SP Subway vocab in CF audit)
- [ ] Gaps clearly marked UNKNOWN

## Gate

**Fail if:**

- Invented features (not in legacy or founder steers)
- Vocab bleed from other products
- Salvage treated as SoT without explicit label
- No citations for referenced code

## Example Output Shape

```markdown
## Current-State Audit: ProductX v1

**Reference repository:** `org/productx-v1` (reference only, not SoT)

| Component | Status | Path/Issue | Notes |
|-----------|--------|------------|-------|
| CLI bootstrap | CARRY | `src/cli/init.rs` | Intent solid; may refactor |
| Config parser | REWRITE | `src/config.rs` | Uses deprecated TOML lib |
| Mission object | DROP | `src/mission.rs` | Founder forbade Mission abstraction |
| Learning gate | UNKNOWN | — | No v1 implementation |
```

## Common Anti-Patterns

- **"We already have X"** without citation → Fail (invented traction)
- **Citing code from wrong product** (e.g., Subway paths in CF audit) → Fail (vocab bleed)
- **Treating legacy as spine** without founder lock → Fail (salvage-as-SoT)

## Relationship to Other Roles

- **Before:** Founder steers
- **After:** Platform + Runtime use audit as reference map (not design constraint)
- **Critique checks:** Audit didn't invent features or blend vocab
