---
name: doc-sync
description: |
  Deploys the Documentation Sync process: whenever a change creates or alters an item
  type, schema, workflow, process, skill, tool, template, decision, or convention, find
  and fix EVERY doc claim the change invalidates — IN THE SAME COMMIT. Use on any CHANGE
  (§3a) or schema/process/skill/tool edit, and before committing such a change. This is
  the deploying skill for .engine/processes/doc-sync.sysml (D0059: every process has a
  downstream skill).
metadata:
  version: 0.1.0
  domain: [documentation, doc-sync, consistency, change-management, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# doc-sync

The deploying skill for the Documentation Sync process. Documentation drift was a
recorded HIGH critique finding (2026-06-11); the rule ("doc-sync rides every change",
CLAUDE.md §4) existed but had no skill, so it ran on human vigilance. This makes it the
AI's job, on every change, same-commit.

## Trigger

Any change that alters: an item type / schema, a workflow / phase / gate, a process, a
skill, a tool, a template, a Decision that supersedes prior guidance, or a standing
convention. (Pure data edits — recording a TestResult, adding a backlog item — do not
invalidate doc claims and need no doc-sync.)

## The doc surface (where claims live)

| Doc | Claims it makes |
|-----|-----------------|
| `CLAUDE.md` | invariants, the interaction loop, bootstrap rules, validation commands, env notes, standing principles + Decision refs |
| `.engine/README.md`, `.engine/docs/*` | architecture, syntax notes (e.g. the Decision-authoring convention), how-to |
| `.engine/skills/skills-registry.sysml` | the registered skill set + each purpose |
| `.engine/processes/*.sysml` | process step text + producedArtifact |
| `.tracking/README.md`, `docs/design-history/*` | tracking layout, retired specs |
| schema/decision header comments | "what this is" docs that can go stale (e.g. a count, a 'can't do X' claim) |

## Behavioral Instructions

1. **Name the change's surface.** What did you add/alter (type/schema/process/skill/tool/
   convention/superseding-decision)?
2. **Grep the doc surface for every claim it touches.** Search CLAUDE.md + .engine/docs +
   registry purposes + process step text + header comments for: the old name/count/rule,
   the superseded behavior, the validator/command list, any "X is frozen / X can't / there
   are N skills" assertion. Don't trust memory — grep.
3. **Fix every invalidated claim in the SAME commit.** Update the text to match the new
   reality; add the governing `Decision` reference (e.g. "(D00NN)"). If a doc asserts a
   count or a "can't", correct it (these rot silently — cf. the CR-10 skill-count, the
   untested "can't specialize Element" comment).
4. **If the change is itself a new process/skill/tool**, register it (skills-registry,
   validator wiring) and add its CLAUDE.md reference.
5. **Validate green** (the touched layers) before commit; the `CR:` commit carries the
   doc fixes alongside the change — never a follow-up "fix docs" commit.

## Anti-Patterns

- **Deferring doc-sync to "later."** Later = drift. Same commit or it didn't happen.
- **Trusting memory over grep.** Stale counts/claims hide; search the surface.
- **Updating the obvious doc, missing the header comments.** Schema/decision/spike header
  comments make "what this is" claims that rot (the "can't specialize Element" comment was
  asserted-untested for sprints).
- **A "fix docs" commit after the change.** The audit trail must show the change and its
  doc-sync together.

## Output Format

```yaml
change_surface: "<what type/schema/process/skill/tool/convention changed>"
doc_claims_checked:
  - doc: "<file §>"
    claim: "<old assertion>"
    action: updated | added-ref | still-valid
registered: <skill/tool/process registered? n/a>
validated: green | <layer:fail>
same_commit: true
```

## Questions This Skill Answers

- "Did I update the docs for this change?" / "Run doc-sync"
- "What docs does this change invalidate?"
- "Is CLAUDE.md / the registry consistent with what we just did?"
