---
name: render
description: |
  Deploys the Modular Render + Review process (D0086): render ANY declared view as a self-contained
  interactive HTML artifact via `keel render <view> --mode graph|table|review`, and round-trip a
  human review back into the model via `keel apply-review`. Use when asked to render/tabulate/
  explore a specific view, build a table from a view, review or disposition elements (accept/reject +
  rationale), capture critique findings, export a review batch, or apply one. A rendered artifact is a
  computed #View (§2.1/D0015) — never author or commit it (git-ignored). The diagram (D0085) is the
  graph preset; this generalizes it to any view + a review round-trip.
metadata:
  version: 0.1.0
  domain: [render, view, table, review, critique, round-trip, computed-view, viewpoint, SysMLv2, D0086]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# render

Runs the engine's Modular Render + Review process (`.engine/processes/render.sysml`). Its defining
move: **a viewpoint declares a view; an interactive artifact is just a *rendering* of one** — so ONE
renderer serves every declared view, and a human review is an **independent critique** (D0080) that
round-trips back as authored facts. Everything rendered is a computed `#View`, regenerated on demand
and **never stored as truth** (the compute-don't-store guarantee of `coverage`/`assured`/`diagram`).

## Expert Vocabulary Payload

**`keel render <view> --mode graph|table|review [--root ROOT]`** → self-contained interactive HTML
to stdout (redirect to a file). `<view>` is a declared view name (a viewpoint — e.g. `decisions`,
`issues`, `processes`) or `model` for the whole-model graph.
- **graph** — cytoscape neighborhood graph (the D0085 diagram when `view`=`model`; else the view's
  selected subgraph). Every node type + edge kind is a toggle; dense derived edges (`contains`/
  `resultof`) default **off**; `Test`/`TestResult` default off.
- **table** — the view's rows (name + type + projected fields) as a sortable/searchable table.
- **review** — that table + per-row **accept / finding** verdict, **lens**, **severity**,
  **actionable?**, and a **rationale** box, with an in-page **Export JSON** button.

**`keel apply-review --batch <batch.json> --sha <commit> --judged-by <you> --judged-at <date>`** →
ingests an exported review batch and writes each disposition as a NEW LINKED critique into
`.tracking/critiques.sysml`: a `method=critique` `verification <element>HRev<n>` + its `TestResult` +
a `#Verify` edge to the element (the human is an independent critic, D0080).
- **accept** → `outcome=pass` — attests the element's state.
- **finding** → `outcome=fail` + `severity` + `lens` — **induces suspicion**: the element shows in
  `keel suspect` (`critique_suspect`) until cleared by a later passing critique. An `actionable`
  finding is tagged for new implementation (planned through the normal sprint/issue flow).

**Generate, don't commit:** `keel render decisions --mode table > t.html` then open it. Generated
`*.html` is git-ignored (D0085/D0086); committing a rendered artifact stores a computed view that
drifts (§2.1/D0018). It's the `render` viewpoint (D0056/D0057).

## Round-trip, don't shadow-store

The exported JSON is **transport, not truth**. The only durable record of a review is the linked
critique items `apply-review` writes into the model. Never paste a review JSON in as a stored
artifact, and never edit an element to record a review — append the critique (that's how suspicion is
computed and how the finding stays defensible + traceable).

## Anti-Pattern Watchlist

1. **Committing a generated artifact** — a `.html` of a graph/table/review staged for commit. Fix:
   it's git-ignored; regenerate instead.
2. **A new bespoke renderer per view** — writing one-off HTML/CLI for each view. Fix: declare the
   view (a viewpoint) and `render` it; that's the whole point of D0086.
3. **Storing the review JSON as the record** — treating the exported batch as the source of truth.
   Fix: `apply-review` writes the linked critiques; the JSON is disposable.
4. **Recording a review by editing the element** — mutating the reviewed item. Fix: a review is an
   independent critique #Verify-linked to the element, never a mutation of it.
5. **Inferring a human disposition** — never synthesize accept/finding the human did not enter; the
   batch is the human's attestation (D0016/D0051).
