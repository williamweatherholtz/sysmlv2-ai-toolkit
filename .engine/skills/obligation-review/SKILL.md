---
name: obligation-review
description: Use when the human wants to review or give feedback on what is waiting on THEM, away from the terminal — "what needs my sign-off", "let me review the queue", "give me something I can use on my phone", "a board I can mark up". Generates a self-contained static HTML review deck from the computed obligation views and publishes it as an artifact; parses their marked-up export back into recorded facts.
---

# Obligation Review Deck

Deploys `.engine/tools/obligation_canvas.py`. The human's queue is computed by `keel`; this turns it
into a page they can operate on a phone and hand back.

## Why this exists

The console (`keel serve`) is the human's oversight lens, and it needs a terminal, a running server,
and a browser on the same machine. Their obligations do not wait for those conditions — 86 items were
outstanding when this skill was written. A static deck travels: no server, no localhost, no shell.

It is a **`#View`** in the §1 sense — regenerable, never truth. Nothing it displays is authored by it,
and nothing the human taps is a recorded fact until step 4 writes one through the write API.

## When to use it

Use it when the human asks to see or judge outstanding work outside the terminal, or asks for
something usable on mobile. Do **not** use it to record their judgment directly — the deck collects,
step 4 records, and only their pasted-back words become a `method=confirmation` result.

## Procedure

### 1. Generate

```
python .engine/tools/obligation_canvas.py . -o <scratchpad>/obligations.html
```

Prints a per-class count. It shells out to the `keel` binary for `orient`, `authority-queue`,
`dispositions` and `sitting-coverage`, so the deck cannot disagree with `keel orient`, and it needs no
running server. Four classes, one hue each: Decisions awaiting acceptance, authority queue, findings
awaiting disposition, sitting reviews due.

### 2. Check before publishing

- **Counts match the computed views.** Compare the printed totals against `keel orient` and
  `/api/obligations` if a console is up. A deck that under-reports is worse than no deck.
- **Every card resolves to a declared item.** There is no legitimate synthetic uid (issue158). A row
  whose name is not a declared item is a SUMMARY - the authority queue emits rows like "274 sprint(s)
  awaiting a sitting review" - and a summary rendered as a card with verdict buttons invites a judgment
  that cannot be recorded. The human accepted one such row twice before this was fixed. `collect` drops
  them and prints what it dropped; read that output, because a silent exclusion is how a deck starts
  under-reporting. An earlier version of this checklist called synthetic uids legitimate for exactly
  this case, which is how the defect survived being looked at.
- **No item appears twice.** `uids = grep -o 'data-uid="[^"]*"'`; unique count must equal card count.
  The authority queue is the AGGREGATE of what awaits a person, so its rows overlap every specific
  class — it is collected LAST for that reason. Collect it first and every finding is silently
  reclassified as "authority queue"; the first version of this tool showed each Decision twice and the
  human judged all four twice before saying so.
- **Per-class counts equal the computed views.** `pendingAcceptances`, `undispositioned` and `due` must
  match `keel orient`, `keel dispositions` and `keel sitting-coverage` exactly. A dedupe bug reads as a
  plausible total while a whole class silently goes to zero.
- **Output is pure ASCII.** `python -c "print(sum(1 for b in open(F,'rb').read() if b>127))"` must
  print 0 — the artifact runtime supplies the `<head>`, so a non-ASCII byte relies on a charset
  declaration this file cannot make.

### 3. Publish

Publish with the Artifact tool. Keep the same file path on later runs so the URL is stable and their
stored judgments (localStorage, keyed per artifact origin) survive. Hand them the URL, what the four
classes are, and the paste-back instruction.

### 4. Record what comes back — and only what comes back

Their export is markdown: `ACCEPT` / `NEEDS WORK` / `REJECT` sections, each item as
`**<name>** (<class>) - <title>` with an optional `- note:` line.

Route each verdict by class, through the write API:

| class | ACCEPT | REJECT | NEEDS WORK |
|---|---|---|---|
| acceptance | `keel accept <decision>` | record the rejection on the Decision | leave proposed; record their note as the reason |
| finding | `keel record` a disposition | same | same |
| authority | depends on the row's `kind` | same | same |
| sitting | a sitting review record | same | same |

**Their note is the attestation text, quoted verbatim.** A verdict with no note records the verdict
and nothing more — do not invent a rationale for them. A tap in a browser is their word only for the
specific item it sits on; it is not licence to infer their position on anything else, and it is not
sign-off on a claim the deck did not state.

## The export must actually leave the page

The clipboard API REJECTS inside a sandboxed artifact frame without `clipboard-write` permission. A
`.then()` with no rejection handler therefore fails silently and the button does not even change its
label — which is what shipped first, and the human had to select the text by hand. `copyOut()` now
passes `manual` as the rejection handler, lifts `readonly` so iOS can select programmatically, and ends
every path in either `Copied` or `Text selected - copy it`. Never leave a control that can appear to do
nothing.

## Guardrails

- **Never record a `method=confirmation` result from a tap you did not receive.** The deck is a
  collection surface; the export is the record. If it did not come back, it did not happen (§4).
- **Never author state into the deck.** It renders computed views. If a class needs a number the
  engine does not compute, add the view — do not compute it in the generator.
- **Key state by UUID, never by label.** Already true in the tool; keep it true. The deck is
  regenerated against a moving model, and label-keyed judgments orphan exactly when the human has
  invested the most in them.
- **Report what the deck cannot do.** It cannot record; it cannot show an item the computed views
  omit; and its verdicts live in one browser's localStorage, so a different device starts empty.

## Removal path

Delete `.engine/tools/obligation_canvas.py` and this skill's directory, and drop the registry entry —
nothing else depends on either, no state is stored outside the human's browser, and no recorded fact
becomes unreadable. The obligations remain computed by `keel orient` and visible in `keel serve`.
