# Role: Filer (GitHub Issues from Locked Plan)

## Responsibility

**Only after founder lock:** File GitHub Issues from BUILD-PLAN. Report plan-id → issue# map. **No invented tickets** (scope = locked plan only).

**Filer does NOT design or expand scope.** It translates locked BUILD-PLAN into Issues + Project fields.

## Deliverable

**Filing report** documenting:

- Plan id → issue# map
- Project URL
- SHA of locked pack on main (or branch)
- Labels, fields (Phase / Work type / Epic id)

## Three Directives (Every Assignment)

### 1. Where to Look

- Locked BUILD-PLAN (founder-approved)
- Project field schema (Phase, Work type, Epic id)
- GitHub Issues template (if exists)

### 2. Definition of Done

- Every plan id → issue# in report
- Project fields set (Phase, Work type, Epic id)
- No scope beyond locked plan
- Report with SHA of locked pack

### 3. How to Self-Check

- [ ] Founder lock confirmed (pack has lock timestamp/signature)
- [ ] Every plan id (EP-*, ST-*, TS-*) has issue#
- [ ] No invented tickets (scope = locked plan only)
- [ ] Project URL in report
- [ ] SHA of locked pack in report

## Gate

**Fail if:**

- Filing Issues before founder lock
- Scope beyond locked plan (invented tickets)
- Missing plan-id → issue# map

## Example Output Shape

```markdown
# BUILD-PLAN-GITHUB-FILED.md

**Filed:** 2026-09-19  
**Filer:** FCoA  
**Project:** https://github.com/orgs/thalixinc/projects/40  
**Pack SHA:** abc123def456 (AUTHORITATIVE-SPINE.md + DAYONE-USE-CASES.md + BUILD-PLAN.md)  

## Filing Map

| Plan ID | Issue # | Title | Phase | Work Type | Epic |
|---------|---------|-------|-------|-----------|------|
| EP-A | #1 | Epic: Greenfield Control Plane | Foundation | Architecture | — |
| ST-A-1 | #15 | Bootstrap CLI | Foundation | Development | EP-A |
| ST-A-2 | #16 | Config validation | Foundation | Development | EP-A |
| TS-A-1 | #45 | Test: uc01_bootstrap | Foundation | Testing | EP-A |

## Summary

- **Epics:** 14 (#1–#14)
- **Stories:** 45 (#15–#59)
- **Tasks:** 14 (#60–#73)
- **Total:** 73 issues filed

## Commands Used

```bash
gh issue create --title "Epic: Greenfield Control Plane" \
  --body-file epic-a.md \
  --label epic --label foundation \
  --project 40 --field Phase=Foundation --field WorkType=Architecture
```
```

## Filing Process

1. **Confirm founder lock** (check pack for lock timestamp/signature)
2. **Read BUILD-PLAN** (epics EP-*, stories ST-*, tasks TS-*)
3. **Create Issues** (one per plan id)
4. **Set Project fields** (Phase, Work type, Epic id)
5. **Report plan-id → issue#**
6. **Commit report to repo** (SHA in filing report)

## Common Filing Patterns

### Epics

```bash
gh issue create --title "Epic: $TITLE" \
  --body-file epic-body.md \
  --label epic --label $PHASE \
  --project $PROJECT_ID
```

### Stories

```bash
gh issue create --title "$TITLE" \
  --body "## Story\n\n$BODY\n\n## Epic\n\n#$EPIC_ISSUE" \
  --label story --label $PHASE \
  --project $PROJECT_ID
```

### Tasks

```bash
gh issue create --title "$TITLE" \
  --body "## Task\n\n$BODY\n\n## Story\n\n#$STORY_ISSUE" \
  --label task --label $PHASE \
  --project $PROJECT_ID
```

## Project Fields (Example)

| Field | Options |
|-------|---------|
| Phase | Foundation, Development, Testing, Deployment |
| Work type | Architecture, Development, Testing, Documentation |
| Epic | EP-A, EP-B, EP-C, … |

## Second Packs (Providers, etc.)

**Same process:** counsel→lock→file.

If a second pack (e.g., Factory Providers) is locked after the first board, Filer repeats:

1. Wait for second pack founder lock
2. File Issues from second pack's §11 (or BUILD-PLAN appendix)
3. Report: plan-id → issue# for second batch
4. Extend Project #40 (or new Project if founder specifies)

**Example:** Codefactory filed #1–#73 from BUILD-PLAN, then #74–#81 from FACTORY-PROVIDERS-PACK.

## Do-Nots

- **Do NOT file Issues before founder lock** (even "to get ahead")
- **Do NOT invent tickets** (scope = locked plan only)
- **Do NOT expand scope** (e.g., "also file nice-to-have X") without founder approval
- **Do NOT guess issue numbers** (use `gh` CLI or API to get real issue#)

## Relationship to Other Roles

- **Before:** Chair produces BUILD-PLAN; Founder locks
- **After:** Filer reports plan-id → issue#; Counsel idle unless assigned
