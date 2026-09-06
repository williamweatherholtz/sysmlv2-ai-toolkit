---
name: asi-templates
description: Use whenever a deliverable should reach a reader as a DOCUMENT rather than terminal text. Two surfaces. (1) An interactive exec-summary decision page — "make me an html", "exec summary", "show me the options", "report this graphically", or any findings/proposal/sign-off request terminal prose would bury. (2) A formal requirements-doc PDF — a specification, interface document, or anything that will be baselined and cited clause by clause. Also carries the brand-drift check ("brand watch", "has marketing changed anything", "recheck branding") that must run before building against the brand tokens. Deploys .engine/processes/asi-templates.sysml; supersedes the standalone exec-summary unit.
---

# asi-templates — the ASI document kit

Two surfaces, one brand, one content contract, one gate. Choose the surface from **what the
reader must DO**, never from preference:

| Reader must… | Surface | Source template |
|---|---|---|
| **Decide** — findings, options, a sign-off request | exec-summary HTML | `templates/exec-summary/exec-summary.html` |
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

## 2. Surface A — the exec-summary decision page

The author's standing direction: AI feedback arrives as a graphical page — extremely concise,
recommendation-first, tabbed, diagrammed, one-click copyable back into an AI conversation.

1. **Start from `templates/exec-summary/exec-summary.html`** — the source template, a clean
   skeleton of `<!-- REPLACE -->` marks. A worked instance sits beside it as
   `example-icd-panel.html`: read it for tone and density, **never** edit it as a starting point.
   The template is self-demonstrating and every structural piece is load-bearing and TESTED — keep
   the machinery (tabs JS, copy JS, theme tokens, viewport meta), replace the content. **Never
   regress:** sync-first copy (async-first ordering breaks one-tap Safari), the viewport meta
   (without it mobile taps mis-register), 44px targets.
2. **Name the source project — in the tab and in the digest.** The author reads these with many
   pages open at once, and a signature given against the wrong project's claims is the failure this
   prevents. Three carriers, all gated:
   - the `data-digest="project"` badge in the header (`<b>Project</b>` is the label; the project
     name and `owner/repo` are the content — `<b>` content is stripped as a label everywhere else
     in this template, so putting the name inside it deletes it from the field);
   - the `<title>`, as `<Name> · <project>` — the browser tab is where it is actually read;
   - the **first line of the digest**, `**Source project:** …`, because a paste arrives in a
     session that cannot see which tree produced it.
   `scripts/check_templates.py` fails on all three, and asserts the digest *emits* the project
   rather than merely querying it.
3. **One tab per DECISION** (3–6). Rename tab ids/labels; keep `role`, `aria-*` and each panel's
   `data-digest-tab`. A tab that asks nothing is an appendix, not a tab.
4. **Inside every tab, in this order, within these budgets.** Every budget is ONE LINE and is
   enforced by `test_prose_stays_within_executive_budgets` — a report whose prose grows paragraphs
   fails the suite. Write to the budget, not to the topic.
   - **The question** (`legend`) — what this tab decides, as a question.
   - **What rides on it** (`data-d="stake"`, ≤ 20 words) — what goes wrong, and *when* it lands.
   - **The choices** (`data-d="choices"`) — 2–4 cards sharing one radio `name`. Recommended
     first, chipped, **never pre-selected** (a pre-selection fabricates a decision the reader
     never made). Every card carries both:
     - `data-d="steel"` — **Strongest case**, ≤ 18 words, as that option's best advocate would put it.
     - `data-d="cost"` — **Why not**, ≤ 14 words, the real cost. Never "none", *including for the
       recommendation* — an option with no downside is an option nobody analyzed.
   - **Note** (`data-d="note"`) — a textarea; flows into the digest.
   - **The reasoning chain** — four one-liners; the report is not finished without them:
     - `data-d="driver"` — **What decides it** (≤ 20): the single criterion the options are scored
       against. Name it before comparing, or the comparison has no referee.
     - `data-d="why"` — **Why this wins** (≤ 32): how the recommendation beats *the strongest
       alternative by name* on that criterion. Compare; never assert. The one place worth a longer
       line — it carries the whole argument.
     - `data-d="flip"` — **What would change this** (≤ 20): the concrete condition that would make
       a different option correct. If nothing could, say so — but say it.
     - `data-d="conf"` — **Confidence** (≤ 16): high/medium/low plus the unverified thing named.
   - **Pros/cons table** (`data-d="proscons"`) — ≤ 6 rows, fragments not sentences.

   Depth belongs in the choice of words, not the count of them. If a line will not fit its budget,
   the thinking is not finished — find the sentence that makes the others unnecessary.

   **The steelman rule.** Write each alternative as its intelligent advocate would — the version a
   person who chose it would recognize and endorse. A one-line dismissal is a strawman and makes
   the whole report untrustworthy: if the alternatives are obviously bad, the reader learns nothing
   from your recommendation winning. Test each: *would someone who prefers this option feel fairly
   represented?* If not, rewrite it. **Corollary: if steelmanning an alternative makes it look
   better than your recommendation, change the recommendation.**

   **Citation is not reasoning.** "The panel converged", "best practice", "the docs say" tell the
   reader who believes something, not why it is true. Every claim gets its mechanism: what happens,
   to what, with what consequence. Provenance goes *after* the mechanism, never instead of it.
5. **Diagram wherever it helps — and it usually helps.** One diagram minimum when the subject has
   structure (a flow, a hierarchy, a boundary). Inline SVG taking every colour from the CSS tokens
   (`var(--accent)`) so both themes work; `role="img"` and a real `aria-label`; wide diagrams live
   in the `figure.diagram` scroll container. Draw the MECHANISM (what connects to what, where the
   gate sits), never decoration.
6. **Copy-for-AI is the product's point.** `buildDigest` emits the question, the stakes, every
   choice with its steelman and cost, the reasoning chain, the reader's selections as checkbox
   markdown (`- [x] chosen`), their notes, and a trailing `My notes: `. Pasted back to an AI that
   is a decision record complete with the reasoning, so the receiving session can argue with the
   logic rather than just read a verdict. Selections persist in `localStorage`. **Add a section ⇒
   extend the digest builder**, or the page and the copy drift. Buttons stay at top AND bottom.
7. **Test before delivering** (§4).
8. **Deliver** as a claude.ai artifact (private by default) or send the file. Title is a short
   noun-phrase name; the footer states who generated it, when, and from which source template.

### Wording rules (the "extremely concise" contract)

Verdict first, evidence second, never the reverse. Fragments beat sentences in tables; full
sentences beat fragments everywhere else. No filler, no hedging stacked on hedging, no restating
the question. Numbers get units.

Concise means **dense**, not thin. Cut adjectives and throat-clearing, never the mechanism:
"Roboto Condensed for headings (brand practice)" is thin; "condensed headings survive long
document titles without wrapping, which the brand's own manual already does" is dense and the same
length. The reader should be able to **disagree with a specific claim** after reading — if they can
only defer to your authority, the writing failed.

### Baked in, deliberately

- **BLUF everywhere** — the page, each tab, and the digest all lead with the recommendation.
- **Decision-ready** — every tab ends with the specific ask; the reader never wonders what response
  is wanted.
- **The copy loop IS the interactivity** — a static page whose copy output is structured FOR an AI
  beats a fragile app: the human annotates in their own words, the AI gets clean context.
- **Glanceable semantics** — verdict/severity colours are separate tokens from the accent, so
  scanning works before reading.
- **Both themes, always** — the page is read on phones at night as often as desks at noon.
- **Self-contained** — no CDN scripts, no external images; Google Fonts is the one allowed host.
- **Provenance and honesty** — generation date, source of findings, and open questions on the page.
  A summary that hides its uncertainty is a worse summary.

---

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
