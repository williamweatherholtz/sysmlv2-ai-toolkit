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

### 3. Publish as a LIVE DOC

Publish with `capabilities: {artifact: {}}`. On a live doc the page's markup IS the document, so the
human's verdict taps and typed notes are appended as them and reach this session with **no copy step** —
which is the point (issue159; they hand-copied four exports before this shipped).

**ORDERING RULE, and getting it backwards destroys their work:** a live doc holds their marks in the
PUBLISHED page, so republishing a regenerated deck OVERWRITES them. Always **read their marks, act on
them, and only then regenerate**. Never regenerate first to "refresh" a deck they may have been marking up.

The export stays as a fallback for a read-only view and copies on open. Keep the same file path so the
URL is stable.

### 3b. Publish (legacy: static, paste-back)

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

## The export is a FALLBACK, and hidden unless it is the only thing that works

`#exp` is `display:none` until `data-local-readonly=true`. An always-visible copy button teaches the
reader that copying is the route, which is the habit the live doc exists to remove — and it was asked
for twice. It is not deleted: a read-only viewer has no other way to return anything, and there it is
the only thing that works.

### When it does show, it must actually leave the page

The clipboard API REJECTS inside a sandboxed artifact frame without `clipboard-write` permission. A
`.then()` with no rejection handler therefore fails silently and the button does not even change its
label — which is what shipped first, and the human had to select the text by hand. `copyOut()` now
passes `manual` as the rejection handler, lifts `readonly` so iOS can select programmatically, and ends
every path in either `Copied` or `Text selected - copy it`. Never leave a control that can appear to do
nothing.

## The live doc switches itself off if script touches the DOM (issue188)

**This is the defect that shipped a deck whose buttons did nothing, twice reported as working.** Read
this section before changing the generator; every rule here cost a broken publish.

On a live doc the runtime treats the `<body>` as a sync region and saves what a **gesture** changes. A
DOM change made by **script** is not a save — it is a signal that the region is no longer a faithful
record, so the runtime switches the region **off**. Everything below follows from that one sentence.

### 1. Never write to the DOM at load

`count()` was called at the end of the script and set `#cnt.textContent`. That single line ran before
the human touched anything, took the body's region off, and made every later tap report `not saved`.

- Render initial content **in the generator**, as HTML. The contract's own words: *write content as HTML
  in the page and mutate it directly in handlers.*
- A DOM write may only happen **inside a gesture handler**.
- Per-viewer chrome (`data-local-*`, `<artifact-local>`) is exempt — that is where a self-report marker
  and any restored UI state belong.

### 2. Read-only is concluded from a REJECTED WRITE, never from `claude:sync-off`

A region can go off for reasons that say nothing about whether this viewer may write — including rule 1
firing. The handler that flagged `data-local-readonly` on any body-level sync-off turned a self-inflicted
region-off into a page that declared itself unwritable.

- `claude:sync-off` may only **report**.
- Read-only follows a `sync()` rejection carrying `not_writer` / `not_granted`, and nothing else.

### 3. The page must say whether its own script survived

A script that throws leaves every listener below the throw unregistered, and the page looks **identical**.
That is indistinguishable from a working page with nothing to do, which is how a dead deck was published
and reported as fine — twice.

- The last statement sets a `data-local-*` marker; CSS shows `script ok` when present and
  `script did NOT finish` when absent, so **absence is the signal**.
- The marker writes an attribute, not text, so the self-report cannot itself trip rule 1.

### 4. Verify from the EMITTED HTML, never from the generator

Reading the python settles what was intended. Counting the output settles what the reader gets. Every
claim in the sprint that shipped this was checked against the generator; the two defects that mattered
were both visible in the output.

Check, on the generated file: zero top-level DOM mutations before the click listener registers; the
count present as text; the marker as the last statement; `Sign` occurring exactly as often as
`data-cls="acceptance"`; `#exp{display:none}` present; a `--custom-property` used by CSS actually
declared (`var(--acc)` was not, and the ribbon silently lost its colour).

### 5. Publish, then ask — do not assert

A deck cannot be tested from here. Hand over the URL and say what to look for: whether the header reads
`script ok`, and whether a tap turns a card and shows `saved`. **Never report the buttons as working on
the strength of having written them.**

### 6. NOTHING asynchronous may write synced DOM — the feedback can destroy the save

Round 2 of the same defect, reported as "they press but don't latch". The tap path was clean: setting
`data-verdict` synchronously in the click handler is gesture-attributed and IS the save. But
`confirmSaved`'s **promise callbacks** then wrote `sv.textContent = 'saved'` — an async script write to
the synced region — so the runtime cut the region and **reverted the very tap the message was
confirming**. The confirmation mechanism was destroying the thing it confirmed.

- Synced DOM is written **only synchronously inside a gesture handler**. Promise callbacks, timers,
  `sync-lost`/`sync-off` events, `window.onerror` — all of it writes `data-local-*` attributes
  (exempt), rendered by CSS `content: attr(...)`, or an `<artifact-local>` element (exempt entirely).
- Re-assert the gesture's change **inside `sync(fn)`** — changes made in `fn` are attributed beyond
  doubt, and the re-assertion is idempotent when the tap already held.
- A **readback** a second later compares the attribute to what was set and says `REVERTED` on the card
  if it moved — so the next field report names the step, not the symptom.
- The mechanical check for pass 4: no `.then(`, `.catch(` or `setTimeout(` body may assign
  `textContent`/`innerHTML` outside the `<artifact-local>` panel. Count it in the emitted HTML.

### 7. `data-id` is the sync key — an element without one cannot be saved

Third latch failure, diagnosed from the runtime itself: fetching the published artifact returns the
frame preamble, whose patch layer addresses every synced element by `[data-id="..."]` with an attribute
allowlist of `data-*`, `aria-*`, `class`, `hidden`, `value`, `checked`. **A mutation on an element with
no `data-id` is unaddressable, therefore unrecordable, therefore reverts.** The canvas template said
`data-id`; this generator had renamed it `data-uid` and every tap silently un-happened.

- Every element whose state must persist carries `data-id` (cards, note inputs — anything mutated).
- Mutations are `data-*` attribute sets or input `value`/`checked` — the allowlist, nothing else.
- Rule 1's corollary is now absolute: script writes **no text** into the synced region under ANY code
  path, gesture included. The pill and the count render via CSS `content` from attributes. One rule
  with no exceptions survives; a rule with a defensible carve-out gets carved.

### 8. Verify the SERVED page for anything the human is told to look at

I told the human "the header says deck v3" — the stamp existed only inside a JS variable and **never
rendered**. My check searched the file and passed; the screen never showed it. And the served CSS
revealed an escape (`\00b7`) mangled to `b7`, invisible in the local file's raw bytes.

- A claim about what the human will SEE is checked against the **published artifact** (WebFetch its
  URL), not the local file.
- Version stamps go in **rendered text**, ASCII-only, in the sub-header — a stamp is for eyes.
- The served page also carries the runtime preamble: when sync misbehaves, READ IT before theorizing.

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
