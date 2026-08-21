---
name: intake
description: Use at the START of any turn where the human states direction — a request, correction, constraint, priority, verdict, or a terminology preference. Records their words verbatim as a Statement, translates them into UserStories, triages each by what it implicates, and links the downstream item. Deploys the Intake process (D0166).
---

# Intake

Deploys `.engine/processes/intake.sysml`. Their words become a tracked item; my reading becomes a
separate tracked item that cites it. Both are auditable, and only one of them is theirs.

## Why the two are separate

The model used to hold my transcription of what they said, inside a `procedureText` field, while the
conversation held the original. That failed twice in one session: a two-word phrase ("confirmed
otherwise") became a recorded confirmation it did not support, and three field reports about a
**destination** were read three times as reports about a **click**. In both cases their words were
exactly right and only my reading was wrong — and only my reading was written down.

## Procedure

1. **Record verbatim.** `Statement` with `text` character-for-character, plus `saidBy`, `saidAt`,
   `channel`. Typos included. Never tidy it.
2. **Translate.** One or more `UserStory` items, each with `#DerivedFrom` → the Statement. A compound
   statement becomes **several** stories — the triage verdict is singular by design.
3. **Triage.** Set `implication` and write `triageNote` saying why this kind and not a neighbouring
   one. **The note is the reviewable part**; a bare verdict cannot be argued with.
4. **Route.** Create the downstream item, author `#Implicates` from the story to it.
5. **Audit.** `keel intake` → unparsed / unrouted / unsourced.

## The vocabulary

Their nine: `need`, `useCase`, **`scopeConstraint`**, `bug`, `process`, `architecture`,
**`verifiedRequirement`**, `designChange`, `implementationChange`.

Two are renamed and the reason is not cosmetic: `constraint` and `requirement` are SysML **keywords**
and the kernel rejects them as enum members. The Rust parser accepts both — the exact gap CLAUDE.md §5
warns about — and `validate_schema.py` caught it because it blocks on schema changes. Their word maps to
the renamed member; say "constraint" and mean `scopeConstraint`.

Five more, each observed in real intake rather than invented: `question` (wants an answer, no change),
`priority` (reorders existing work), `attestation` (their sign-off — **the largest category by count**),
`correction` (retracts something previously recorded), `convention` (terminology or style).
Plus `none`, so that "this implicates nothing" is a recorded decision rather than an omission.

**A statement the vocabulary cannot express is a CHANGE to the vocabulary**, through a Decision — never
a forced fit into the nearest member. A wrong triage is worse than an absent one: it routes real
direction to the wrong place and looks handled.

## Guardrails

- **Only directive statements.** A transcript is a second corpus with no questions attached — the
  mistake D0161's first draft made. Record a request, a correction, a constraint, a priority, a verdict,
  a convention. Not commentary, not thinking aloud.
- **Never edit a Statement to agree with a story derived from it.** A misreading is corrected by a NEW
  story citing the same Statement; the old one is superseded and stays visible. Their words are theirs
  (D0108).
- **A story with no `#DerivedFrom` is an invention wearing a story's clothes.** `keel intake` reports it
  separately from unrouted, because the two need different fixes: one needs a source, the other an
  outcome.
- **`unsourced` measures the RECORD, not the human.** It is a floor: an item may be genuinely requested
  and simply have no Statement written down. Never report it as "work nobody asked for".

## Removal path

Delete `.tracking/intake/`, this skill, the process file, its registry entry, `EngineIntake` from
`schema/core`, and `view::intake` with its two call sites. Nothing else reads any of it; Needs,
Issues, Decisions and Stories are unaffected, since none of them was ever dependent on having a source.
