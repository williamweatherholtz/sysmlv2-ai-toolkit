---
name: exec-summary
description: Use whenever findings, a proposal, an analysis, or a decision request should reach the author as a page rather than terminal text — "make me an html", "report this graphically", "exec summary", "show me the options" — or whenever a turn's deliverable is feedback the author will react to. Produces an AI-interactive augmented executive summary from templates/exec-summary/exec-summary.html: recommendation-first tabs, pros/cons tables, a mechanism diagram, and one-tap Copy-for-AI controls verified on desktop and mobile. Recorded need: needExecSummary.
---

# exec-summary — AI-interactive augmented executive summaries

The author's standing direction (stExecSummary, 2026-08-28): AI feedback should arrive as a
graphical page — extremely concise, recommendation-first, tabbed, diagrammed, and one-click
copyable back into an AI conversation. This skill is how every such page gets made.

## Procedure

1. **Start from `templates/exec-summary/exec-summary.html`** — the SOURCE TEMPLATE (a clean
   skeleton of REPLACE marks). A worked instance lives beside it as
   `templates/exec-summary/example-icd-panel.html`; read it for tone and density, never edit
   it as a starting point. The template is a project asset in `templates/`; this skill
   consumes it and never owns it — format improvements go to the source template, and every
   rendered report's footer links back to that source, not to itself. It is self-demonstrating (filled
   with real content) and every structural piece is load-bearing and TESTED — keep the
   machinery (tabs JS, copy JS, theme tokens, viewport meta), replace the content at the
   `<!-- REPLACE -->` marks. Never regress: sync-first copy (the async-first ordering breaks
   one-tap Safari), the viewport meta (without it mobile taps mis-register), 44px targets.
2. **One tab per DECISION** (3–6). Rename tab ids/labels; keep `role` and `aria-*` wiring and
   the `data-digest-tab` name on each panel. A tab that asks nothing is an appendix, not a tab.
3. **Inside every tab, in this order, with these budgets:**
   - **The question** (`legend`): what this tab decides, as a question.
   **Every budget below is ONE LINE and is enforced by `test_prose_stays_within_executive_budgets`
   — a report whose prose grows paragraphs fails the suite. Write to the budget, not to the topic.**
   - **What rides on it** (`data-d="stake"`, ≤ 20 words): what goes wrong, and *when* it lands.
   - **The choices** (`data-d="choices"`): 2–4 cards sharing one radio `name`. Recommended
     first, chipped, **never pre-selected** (a pre-selection fabricates a decision).
     Every card carries both:
     - `data-d="steel"` — **Strongest case**, ≤ 18 words, as that option's best advocate would
       put it. See the steelman rule below; this is the load-bearing part.
     - `data-d="cost"` — **Why not**, ≤ 14 words, the real cost. Never "none", including for
       the recommendation — an option with no downside is an option nobody analyzed.
   - **Note** (`data-d="note"`): a textarea; flows into the digest.
   - **The reasoning chain** — four one-liners, and the report is not finished without them:
     - `data-d="driver"` — **What decides it** (≤ 20): the single criterion the options are
       scored against. Name it before comparing, or the comparison has no referee.
     - `data-d="why"` — **Why this wins** (≤ 32): how the recommendation beats *the strongest
       alternative by name* on that criterion. Compare; never assert. The one place worth
       a longer line — it carries the whole argument.
     - `data-d="flip"` — **What would change this** (≤ 20): the concrete condition that would
       make a different option correct. If nothing could, say so — but say it.
     - `data-d="conf"` — **Confidence** (≤ 16): high/medium/low plus the unverified thing.
   - **Pros/cons table** (`data-d="proscons"`): ≤ 6 rows, fragments not sentences.

   Depth belongs in the choice of words, not the count of them. If a line will not fit its
   budget, the thinking is not finished — find the sentence that makes the others unnecessary.

   **The sufficiency rule — a tab must be judgeable WITHOUT leaving the page (pf52).**

   The reader is adjudicating, not approving. If they must open the repository, the ticket or the
   spec to know what they are deciding, the tab has failed however well-formed it is. The failure in
   the field, verbatim: *"I get some semblance of 'we need Need1 & Need2 judged. WHAT DECIDES IT:
   picking which ones are good.'"* That restates the question as its own answer.

   Every tab asking for a judgment on an ITEM carries the item's substance inline:

   - **Quote it.** The statement — or the contested clause — in the item's own words. A NAME is not
     an item: `nAutonomyBoundedByProof` tells the reader nothing; the sentence inside it tells them
     everything.
   - **Say what accepting BINDS.** What it legitimises, what it forecloses. The reader is signing a
     contract later work will cite, so show them the clause that gets cited back.
   - **Make the fork concrete.** *This clause versus that clause*, in the item's own language. Never
     "accept / reject / revise" — those are verdicts, not alternatives.
   - **Separate evidence from assumption.** What is already TRUE in the model (a count, a passing
     test, an observed failure) versus what the author believes. Label them apart.
   - **Show what happens on each answer.** The concrete next action per option, so the cost of a
     choice is visible before it is made.

   **The `driver` test.** *What decides it* must name an **observable**, or a **value only the reader
   holds**. "Which one is better", "picking the good ones", "whether these are right" all fail: they
   restate the question. If the criterion can be settled by reading the model, the author has not
   finished their work — settle it and stop asking.

   **A green suite does not mean a good page (pf54).** Everything the tests measure is a count:
   words per field, cards per tab. Everything that decides whether the page WORKS is qualitative.
   Passing the budgets is the floor, not the verdict.

   **The steelman rule.** Write each alternative as its intelligent advocate would — the version
   a person who chose it would recognize and endorse. A one-line dismissal is a strawman and
   makes the whole report untrustworthy: if the alternatives are obviously bad, the reader
   learns nothing from your recommendation winning. Test each: *would someone who prefers this
   option feel fairly represented?* If not, rewrite it. Corollary: if steelmanning an
   alternative makes it look better than your recommendation, change the recommendation.

   **Citation is not reasoning.** "The panel converged", "best practice", "the docs say" tell
   the reader who believes something, not why it is true. Every claim gets its mechanism: what
   happens, to what, with what consequence. Provenance goes *after* the mechanism, never
   instead of it.
4. **Diagram wherever it helps — and it usually helps.** One diagram per page minimum when the
   subject has structure (a flow, a hierarchy, a boundary). Inline SVG taking every color from
   the CSS tokens (`var(--accent)` etc.) so both themes work; give it `role="img"` and a real
   `aria-label`; wide diagrams live in the `figure.diagram` scroll container. Draw the
   MECHANISM (what connects to what, where the gate sits), never decoration. When the page is
   published as a claude.ai artifact, a `<pre class="mermaid">` block is also natively
   rendered — fine for quick flow/sequence diagrams; inline SVG remains the portable choice.
5. **Copy-for-AI is the product's point.** The digest (`buildDigest`) emits the question, the
   stakes, every choice with its steelman and cost, the reasoning chain, the reader's
   selections as checkbox markdown (`- [x] chosen`), their notes, and a trailing `My notes: `.
   Pasted back to an AI, that is a decision record complete with the reasoning behind it — the
   receiving session can argue with the logic, not just read a verdict. Selections persist in
   `localStorage`. If you add sections, extend the digest builder so nothing on the page is
   missing from the copy. Buttons stay at top AND bottom.
6. **Test before delivering.** `EXEC_SUMMARY_FILE=<your.html> .venv/Scripts/python -m pytest
   tests/exec_summary -q` (from the repo root; plain run tests the template itself). The suite
   is real browsers: Chromium desktop click→clipboard read-back, WebKit iPhone-class one-tap,
   44px targets, structure order, self-containment, both themes.
7. **Deliver** as a claude.ai artifact (private by default) or send the file. Title is a short
   noun-phrase name; provenance line and footer state who generated it, when, from what.

## Wording rules (the "extremely concise" contract)

Verdict first, evidence second, never the reverse. Fragments beat sentences in tables; full
sentences beat fragments everywhere else. No filler ("it should be noted"), no hedging stacked
on hedging, no restating the question. Numbers get units. If a section exceeds its budget, the
content belongs in an appendix tab or in the repo — not in more words.

Concise means *dense*, not *thin*. Cut adjectives and throat-clearing, never the mechanism:
"Roboto Condensed for headings (brand practice)" is thin; "condensed headings survive long
document titles without wrapping, which the brand's own manual already does" is dense and the
same length. The reader should be able to disagree with a specific claim after reading — if
they can only defer to your authority, the writing failed.

## Best practices baked in (the author asked; these are the answers)

- **BLUF everywhere**: the page, each tab, and the digest all lead with the recommendation.
- **Decision-ready, not just informative**: every tab ends with the specific ask; the reader
  should never wonder what response is wanted.
- **The copy loop is the interactivity**: a static page whose copy output is structured FOR an
  AI beats a fragile app; the human annotates in their own words, the AI gets clean context.
- **Diagrams as compression**: one mechanism diagram replaces paragraphs; it must be data-true
  (drawn from the thing described, theme-aware, labeled for screen readers).
- **Glanceable semantics**: verdict/severity colors are separate tokens from the accent;
  chips and tags carry state so scanning works before reading.
- **Both themes, always** (the three-state token pattern in the template); the page is read on
  phones at night as often as desks at noon.
- **Self-contained**: no CDN scripts, no external images; Google Fonts is the one allowed host.
- **Provenance and honesty**: generation date, source of findings, and open questions are on
  the page; a summary that hides its uncertainty is a worse summary.

## Do not

- Do not hand the reader an untested page: the copy control and tabs are behavioral claims,
  and the suite exists so they are verified, not asserted.
- Do not let the digest drift from the page (new section ⇒ extend `buildDigest`).
- Do not bury the recommendation below context, ever — context is what tabs two through N
  are for.
