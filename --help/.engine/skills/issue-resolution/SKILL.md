---
name: issue-resolution
description: |
  Deploys the Issue Resolution process (D0078/D0077): link every Issue to the work
  or Decision that resolves it via a #Resolves edge, let resolution COMPUTE from
  resolver-completeness, and record decision-moots-item as a typed edge (never
  prose). Use when recording or triaging an Issue, when asked "resolve issue X,"
  "what issues are open," "is issue NNN still open," "this decision makes X moot /
  obsolete / subsumed," or at the start of issue work. Also triggers on the `issues`
  guard failing (untriaged issue). Do NOT use for backlog story refinement (use
  backlog-refinement) or for authoring a Decision's content (just record the
  #Resolves edge alongside it).
metadata:
  version: 0.1.0
  domain: [issue-tracking, traceability, computed-state, SysMLv2, D0077, D0078]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# issue-resolution

Runs the engine's Issue Resolution process (`.engine/processes/issue-resolution.sysml`).
Its defining move: an Issue's open/resolved state is **computed**, never stored — an
Issue is RESOLVED iff a `#Resolves` resolver (an action that's **done** or a Decision
that's **accepted**) is complete. The loop is record → triage → resolve → decision-moots-item.

## Expert Vocabulary Payload

**`#Resolves` edge (D0077):** `#Resolves dependency from <resolver> to <issueNNN>;`
(import `EngineRelationships::*`). `<resolver>` is a backlog **action** OR a **Decision**.
**relatedTask** (D0029) is the *caused-by / related* link — NOT the resolver; an issue can
relate to a done task yet still be open (issue014/025 were exactly this).

**Resolution = computed (D0001/D0018):** `keel open-issues` (the open set + each
resolver + `untriaged`), `keel orient` (`open_issues`). There is **no** authored
`status` field; a prose "RESOLVED …" clause in a description is *informational only*.

**Triage:** every Issue must carry a `#Resolves` edge — `keel guard issues` fails on an
untriaged issue. **Decision-moots-item:** an Issue mooted by a Decision → `#Resolves` from
the Decision; a Need/Requirement → the existing `supersede` edge (D0004). [Issue-scoped, D0078.]

## Anti-Pattern Watchlist

1. **Prose "RESOLVED" as truth** — Detection: closing an issue by appending "RESOLVED…" to
   its description with no resolver edge. Resolution: add a `#Resolves` edge to the
   completing resolver; the prose is at most a note. Resolution computes.
2. **relatedTask = resolver** — Detection: assuming an issue is resolved because its
   `relatedTask` action is done. Resolution: the resolver is the `#Resolves` target; if the
   related/causing task is done but the issue persists (issue014/025), create a NEW resolving
   action and `#Resolves` to it. The issue stays OPEN until that work lands.
3. **Untriaged issue** — Detection: a recorded Issue with no `#Resolves` edge (`guard issues`
   fails). Resolution: link it to a resolving action (create one) or a mooting Decision.
4. **Decision-moots-item left as prose** — Detection: "subsumed by DXXXX / superseded by
   DXXXX" in text with no typed edge (the issue017 lapse). Resolution: record `#Resolves`
   from the Decision (Issue) or `supersede` (Need/Requirement) in the same change.
5. **Authoring a status field** — Detection: adding `status`/`resolved` to an Issue.
   Resolution: rejected (§2.1 compute-don't-store); resolution is computed from `#Resolves`.

## Behavioral Instructions

1. **Recording an Issue:** author the `Issue` part (description, discoveredInField,
   relatedTask); then IMMEDIATELY triage it (step 2) — never leave it untriaged.
2. **Triage:** decide the resolver. Is there an existing OPEN action that will fix it? Link
   `#Resolves` to it. Is it wontfix / resolved-by-design? Link `#Resolves` to the accepted
   Decision. Is there no fixing work yet? CREATE a backlog action first, then `#Resolves` to it.
3. **Resolving:** never hand-close. When the resolver completes, `keel open-issues` /
   `orient open_issues` recompute. Report the open set from the tool, not from memory.
4. **Decision-moots-item:** when a Decision moots an item, record the typed edge in the SAME
   change as the Decision — `#Resolves` from the Decision (Issue) or `supersede`
   (Need/Requirement). Do not write "subsumed by DXXXX" as the only record.
5. **Verify:** `keel guard issues` green (all triaged) and `keel open-issues` shows the
   intended open set before committing.

## Output Format

```yaml
issue: <issueNNN>
resolver: <action|decision name>
resolver_kind: action | decision
state: open | resolved        # computed: resolved iff resolver complete
action_taken: linked #Resolves | created resolving action + linked | recorded decision-moot
guard_issues: pass | fail
```

## Questions This Skill Answers

- "Record / triage this issue"
- "Resolve issue NNN" / "is issue NNN still open?"
- "What issues are open?" (→ `keel open-issues`)
- "This decision makes issue X moot / obsolete — how do I record that?"
- "The `issues` guard is failing — what's untriaged?"
- "How do I link an issue to the work that fixes it?"
