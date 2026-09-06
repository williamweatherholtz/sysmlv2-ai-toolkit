---
name: asi-templates
description: Use whenever a deliverable should reach a reader as a DOCUMENT rather than terminal text. Two surfaces. (1) An EXECUTIVE BRIEF — "make me an html", "exec summary", "show me the options", "report this graphically", or any findings/proposal/sign-off request terminal prose would bury. Answer first, everything needed to decide on one page, NO record ids in anything the reader reads, every number computed rather than typed, and per ask two mandatory logic exhibits: how it works today versus changed, and what lands downstream. (2) A formal requirements-doc PDF — a specification, interface document, or anything that will be baselined and cited clause by clause. Also carries the brand-drift check ("brand watch", "has marketing changed anything", "recheck branding") that must run before building against the brand tokens. Deploys .engine/processes/asi-templates.sysml; supersedes the standalone exec-summary unit.
---

# asi-templates — the ASI document kit

Two surfaces, one brand, one content contract, one gate. Choose the surface from **what the
reader must DO**, never from preference:

| Reader must… | Surface | Source template |
|---|---|---|
| **Decide** — findings, courses, a sign-off request | executive brief (HTML) | `templates/exec-summary/exec-summary.html` |
| **Cite** — a stable statement, baselined, reviewed clause by clause | requirements-doc PDF | `templates/requirements-doc/requirements-doc.typ` |

A decision request rendered as a formal PDF buries the ask. A specification rendered as tabs
cannot be cited.

Both draw metadata, header, footer and every colour from `brand/style-tokens.json`. **No brand
value is hardcoded in either template** — if you are typing a hex literal, you are doing it wrong
(there is a lint for it).

---

## 0. Before anything: is the brand current?

The visual identity derives from sources marketing owns and changes without telling anyone, so
drift is **computed**, not noticed.

1. Snapshot the watched sources (Microsoft 365 MCP `read_resource` per watched folder, recursing
   one level; for the Brand Field Manual PDF also extract text and record `textSha256` — binary
   bytes are not fetchable through the connector, extracted text is the content signal). Write it
   in the same JSON schema as `brand/manifest.json`.
2. `python .claude/skills/brand-watch/check_manifest.py brand/manifest.json <fresh>`
   Folder `size` is a **recursive** change signal — it moves when anything inside moves. A `null`
   on either side is UNOBSERVED: skipped, never reported as drift.
3. **No drift** → say so and touch nothing. Do not refresh the baseline to bump its date.
4. **Drift** → in ONE commit: an Issue naming exactly which sources moved; the baseline updated to
   the fresh snapshot; the derived mirrors (style tokens, `ingest/`) refreshed. The manifest update
   and the Issue travel together so the record never claims a baseline it has not reconciled.

**Never** hand-edit `brand/manifest.json` (it is regenerated from observation only), and **never**
fabricate a hash for content that could not be fetched — record `null` and say so.

---

## 1. The content contract (both surfaces)

`templates/CONTENT-FORMAT.md` admits exactly **two** field classes. Nothing else is legal.

**LITERAL fields** — `meta.*`, `requirements[].id`, `requirements[].statement`. Rendered
character for character, no markup interpretation in any renderer. A statement containing `$500`,
`#4`, `@10Hz`, `<200ms>` or `*emphasis*` renders exactly those characters. Mechanism (Typst): the
string is inserted directly as content, **never through `eval()`**.

> That route was **issue001**, and it is why this rule is absolute: `eval()` silently deleted a
> performance bound from a *must*-requirement (`under <200ms> at P99` shipped with the bound
> gone, exit 0), hard-failed on ordinary text like a dollar amount, and executed a `#read(...)`
> embedded in instance data — pulling a repo file's contents into the rendered PDF.
> `templates/requirements-doc/example-awkward.json` carries every character class issue001
> demonstrated, and the harness asserts each statement survives verbatim in extracted text.

**CONTENT fields** — `content`, `requirements[].rationale`, `requirements[].notes`. Parsed as the
declared CommonMark subset **only**: paragraph, emphasis, strong, inline code, link, bullet and
numbered list. Code spans are opaque (a `$` or `<` inside backticks is literal and needs no
escape). Anything outside the subset — setext headings, footnotes, tables, raw HTML including
comments — is refused **loudly at the renderer**, never silently dropped. Silent dropping was
issue003; the refusal is the fix.

---

## 2. Surface A — the executive brief

The author's standing direction, verbatim: *"I need all information critical to making the decision in
one place, no references, high amounts of visuals to illustrate ideas, and strong logical
recommendation to guide decision making."* And, on what the diagrams must carry: *"we're making
logical decisions here, so I want diagrams relating to the logic. how do things works today and how
would the change affect them? I don't want administrative changes ... I want downstream impact."*

**The rule the whole surface hangs on: the brief is a pyramid whose nodes are exhibit titles.** Delete
every graphic and the remaining sentences still decide it. Delete every sentence and the graphics say
nothing. If either half fails, the brief is not finished.

**Why the shape changed (2026-09-06).** The previous surface was tabbed and IBIS-shaped — one position
with pro/con arguments hanging off it — which affords the reader exactly two acts, accept and defer.
That is a property of the notation, not of the writing: no amount of care inside it produces a
reviewable document. The author's verdict on a page built that way was *"mostly just administrative
justification for the decision"*, and it was the second time the same complaint landed. It is
QOC-shaped now: courses crossed with the forces that bear on them, each independently disputable.

### 2.1 The section order

Answer first, always. A reader who reads only the title has the recommendation; one who reads only the
exhibit titles has the argument.

| # | Section | What it must carry | Budget |
|---|---|---|---|
| 0 | **Title** | The recommendation itself, verb-led. Never the topic. | ≤ 18 words |
| 1 | **The ask** | The verdict, then one line per ask with its response options and its reversibility | ≤ 70 words |
| 2 | **Why now** | Situation, then the one thing that changed and put a clock on it. If nothing has a clock, say so. | ≤ 60 words |
| 3 | **The decision rule** | The criteria, ranked, stated BEFORE any option is defended | ≤ 40 words |
| 4 | **Per ask: today → changed → downstream** | Two logic exhibits minimum (§2.3), then the courses table | — |
| 5 | **Courses** | 2–4 including a do-nothing, each with places-to-change and a quantified cost | table |
| 6 | **Why this wins / when the runner-up wins** | Beats the named alternative on the stated criterion; the runner-up as a conditional, not a rival | ≤ 90 words |
| 7 | **What would change my mind** | One falsifiable condition per ask | ≤ 30 words each |
| 8 | **Provenance strip** | How every number was measured, by whom, when, against which tree | ≤ 60 words |

### 2.2 Self-containment — the no-references rule

- **No record id, task name, issue number or process name appears in anything the reader reads.** An id
  is a pointer that costs a lookup; the fact it stands for is what the sentence should have said.
  "This resolves the migrate defect" is administrative; "an upgrade silently puts back controls the
  project switched off" is the same fact, told.
- **Identity rides in the copy-for-AI digest, never in the prose.** Each ask declares what it decides
  in `data-records`; the digest emits it beneath the reader's choice. That is what lets a pasted answer
  be attached to a record without an id ever facing the human — and without it, an answer cannot be
  recorded at all (the failure that produced this clause).
- **Every number is computed, never typed.** The builder reads a facts file produced by running
  commands against the tree; a fact that cannot be computed is emitted as null and the page refuses to
  assert it. Three of five figures in one hand-typed diagram were wrong within a day of publishing.
- **Numbers in one sentence come from one scope.** Pairing a top-level count with a whole-surface count
  published "72 of 69". The scope is part of the fact, so it belongs in the fact's own name.
- **State whether the change already ships.** Half of one queue was already running, so the only act
  left was revert — and no revert was priced. An ask that is really a ratification says so.

### 2.3 The visual playbook — logic, not decoration

**Two forms are mandatory per ask.** Both are generated from data by `scratchpad`-side chart helpers,
never hand-placed, so a figure cannot drift from the tree.

1. **Today → changed.** The same causal chain twice, identical geometry, the changed link highlighted.
   Reads: *condition → mechanism → outcome*. A single-state diagram cannot show a change and is a
   defect on a decision page.
2. **What lands downstream.** A fan from the change to each thing it hits, every branch carrying its
   own magnitude. This is the exhibit that answers "what does this do to everything else", and it is
   usually where the real argument turns out to live.

Supporting forms, chosen by what the claim says, never by preference:

| The claim says | Form |
|---|---|
| "costs N times more than" | proportional bars, sorted, zero baseline, value at the bar end |
| "is made of these parts, one dominates" | one 100% stacked bar, ≤4 segments |
| "has been true for N days" / "grows" | timeline with both ends labelled, or a line with the decision date marked |
| "N of M are X" | unit dots, one per item, so the proportion is counted rather than estimated |
| "the reader must verify individual values" | a table — verification is a symbolic task and a chart makes it harder |

**Every figure carries a message title that is a full sentence with a verb**, and the title must be
falsifiable by the figure beneath it. A title that names its contents ("CLI surface today") is
decoration; one that states what the figure proves is an argument. Untitled figures are worse than
none: the reader installs their own conclusion.

**Every label is a thing that happens or a thing that exists.** Never a record id, never a bare
command standing in for a behaviour. One accent per figure; quantities in the boxes, not the caption.

**Look at the rendered figure before shipping it.** Screenshot the page and read it. This is not
optional politeness — doing it caught a colliding legend, "1 sites", a timeline with no scale, and a
box overrunning the canvas, all of which passed every text-level check.

**When NOT to draw:** a counting or threshold decision (a table beats it), a single-clause
ratification with no branch, a change that alters a value rather than a path, and anything whose
diagram would need three ids to be legible — that last one means the mechanism is not understood yet.

### 2.4 Recommendation craft

- **Imperative, single clause, named actor.** Passive voice deletes the actor, and a recommendation
  with no actor is not one.
- **Criteria before verdict**, or the criteria read as rationalisation.
- **A do-nothing course, scored like the others.** The status quo always competes and wins by default
  when it is not priced.
- **Refute the strongest objection on the page.** Raising a downside without answering it is worse
  than not raising it.
- **Publish the falsifier**: the condition that would flip the recommendation, stated so it can be
  observed. A flip condition nothing in the tree can observe is not a flip condition.
- **If the measurement turns against the recommendation, change the recommendation and say that you
  did.** A brief that reverses its own advice on new measurement is the only kind worth reading twice.

## 3. Surface B — the requirements-doc PDF

1. **Author the instance JSON**, holding §1's two field classes. Three worked examples ship:
   `example-data.json` (the ordinary case), `example-awkward.json` (every issue001 character
   class — the verbatim-survival regression), `example-fidelity.json` (extraction-artifact ids).
2. **`templates/base/asi-base.typ`** supplies metadata, header, footer, page numbers and the brand
   tokens; `requirements-doc.typ` is the surface. Missing meta fields must fail loudly — a document
   shipping as UNTITLED at exit 0 was a real defect.
3. **`outputs.json` declares the field map** in a machine shape (field, surfaces, readback kind,
   transform) and `scripts/render.py` derives its read-back assertions FROM it. A declared entry
   the harness cannot exercise **fails the run** — the file is not documentation, it is executed.
4. **Fonts are vendored** (`fonts/`, Roboto + Noto fallbacks with their OFL licences). Render with
   `--font-path fonts --ignore-system-fonts` so output is reproducible; system-font substitution is
   silent, which is exactly why it is forbidden.
5. **Gate** (§4).

---

## 4. The gate — an untested document is not delivered

**Contract checker, both surfaces, on every commit touching `templates/`** (D0237, wired into
`.githooks/pre-commit`):

```
python scripts/check_templates.py
```

It **derives** its field inventory from the source template rather than a hand-list: every
`data-d` / `data-digest` key must exist in each instance, and no prose field may still read as the
template's own wording — an unreplaced placeholder is caught, not shipped. It also enforces
adjudication provenance: the project badge, the project in the `<title>`, and a digest that emits a
`**Source project:**` line. `tests/exec_summary`
calls the same module rather than restating its rules, so the gate and the suite cannot drift.

**Exec-summary suite** — real browsers, because one-click copy, tab wiring, 44px targets and
one-tap at iPhone scale are *behavioral* claims that must be verified rather than asserted:

```
EXEC_SUMMARY_FILE=<page> .venv/Scripts/python -m pytest tests/exec_summary -q
```

(A plain run tests the template itself.) Chromium click→clipboard read-back, WebKit iPhone-class
one-tap, structure order, budgets, self-containment, both themes.

**Requirements-doc harness:**

```
.venv/Scripts/python scripts/render.py <instance> --check
```

Schema validation refusing to render on failure; zero-warning compile; field-map read-back by PDF
text extraction asserting statements survive **verbatim**; golden pixel diff. The toolchain
manifest is asserted before any render, so a drifted typst/python silently producing different
output is refused.

**Re-baselining a golden requires the author's approval, never the AI's** — a golden diff that the
AI may re-baseline at will locks defects in rather than catching them, which is precisely how the
footer-collision defect (issue002) came to be baked into every committed baseline.

---

## Do not

- **Do not** deliver an untested page or an unrendered PDF: the copy control, the tabs and the
  verbatim guarantee are behavioral claims, and the suites exist so they are verified.
- **Do not** let the digest drift from the page (new section ⇒ extend `buildDigest`).
- **Do not** bury the recommendation below context, ever — context is what tabs two through N are for.
- **Do not** edit an example instance as a starting point; start from the source template.
- **Do not** hardcode a brand value in a template, or hand-edit `brand/manifest.json`.
- **Do not** pass a data string to `eval()`, in any renderer, for any reason (issue001).
- **Do not** ship an adjudication page that does not name its source project in the tab AND the
  digest — the reader has many pages open, and the digest is read where the tree is unknown.
- **Do not** treat a returned Copy-for-AI digest as a human sign-off. It is a *statement* about
  specific claims — recording `method=confirmation` still needs their explicit word on that claim.

## Removal path

Delete this skill + registry + `.engine/processes/asi-templates.sysml`. The templates, scripts and
suites keep working as ordinary repo files; only the discipline leaves the catalogue, and
`keel process list` stops offering the unit.
