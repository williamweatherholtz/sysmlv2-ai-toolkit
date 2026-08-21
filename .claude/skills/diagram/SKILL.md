---
name: diagram
description: |
  Deploys the Traceability Diagram process (D0085): generate the comprehensive traceability
  diagram as a regenerated, interactive, self-contained HTML view via `keel diagram` — never
  author or commit a diagram (it's a computed #View, §2.1/D0015). Use when asked to "see/draw/
  visualize/diagram" the model, traceability, reliance, linkage, supersession, effects, or
  metadata; to map how decisions/needs/requirements/work/processes connect; or to explore the
  graph a part at a time. Do NOT hand-draw a diagram, embed a picture in docs, or commit a
  generated .html — regenerate it.
metadata:
  version: 0.1.0
  domain: [traceability, visualization, computed-view, viewpoint, SysMLv2, D0085]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# diagram

Runs the engine's Traceability Diagram process (`.engine/processes/diagram.sysml`). Its defining
move: a diagram is a **computed view**, regenerated on demand from authored facts and **never stored
as truth** — so it can never drift (the same compute-don't-store guarantee as `coverage`/`assured`).

## Expert Vocabulary Payload

**`keel diagram [ROOT]`** — emits a single self-contained interactive HTML page (cytoscape) of the
WHOLE model: every element as a typed node carrying its authored metadata (`status`/`severity`/`lens`/
`kind`/`priority`/`outcome`/`method`/`critiquedBy`/`createdBy`/`marker`) and every typed edge
(`satisfy`/`verify`/`charteredby`/`resolves`/`supersede`/`allocate`/`dependency`/`dependson`/
`succession`/`ordering`/`prospectivechange`/`safetychange`). In-page controls: type + edge-kind
filters, id search, click-a-node to **focus** its neighborhood, **Fit**. `Test`/`TestResult` are
toggled **off** by default (edgeless ceremony leaves) — flip them on for the full ~2500-node view.

**Generate, don't commit:** `keel diagram . > traceability.html` then open it. Generated `*.html`
is git-ignored (D0085); committing a rendered diagram stores a computed view that drifts (§2.1/D0018).
It's the `traceability` viewpoint (D0056/D0057).

## Anti-Pattern Watchlist

1. **Committing a generated diagram** — Detection: a `.html`/`.svg`/`.png` of the graph staged for
   commit. Resolution: it's a `#View`; regenerate on demand, keep it git-ignored.
2. **Hand-drawing / embedding a picture in docs** — Detection: an authored diagram or pasted image
   as the "source of truth." Resolution: the authored `.sysml` model is the truth; `keel diagram`
   renders it. Patch the model, not the picture.
3. **Patching a stale diagram** — Detection: editing an old HTML after the model changed. Resolution:
   regenerate; never edit the derived artifact.
4. **Recording a finding on the picture** — Detection: annotating the diagram with an issue.
   Resolution: a finding becomes a tracked `Issue`/critique (issue-resolution / element-critique),
   not an annotation on a disposable view.

## Behavioral Instructions

1. **Generate:** `keel diagram [ROOT] > <name>.html` (absolute ROOT per §6). Hand the file to the
   human to open in a browser (no server needed).
2. **Explore:** point them at the controls — type/edge filters, search, click-to-focus, Fit; note
   `Test`/`TestResult` are off by default.
3. **Dispose:** don't commit it; regenerate when the model changes. Any finding → a tracked
   Issue/critique.

## Questions This Skill Answers

- "Show me / draw / visualize the model / traceability / how X connects."
- "What relies on / links to / supersedes / is affected by Y?" (generate + focus Y).
- "Give me a comprehensive picture I can explore."
