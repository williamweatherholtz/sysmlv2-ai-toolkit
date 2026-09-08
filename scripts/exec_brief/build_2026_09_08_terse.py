#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (fourth) queue change, in the terse
register the human asked for: two asks, both about this page - the empty-queue receipt shape and
the caveman-lite register with a whole-page cap.

Usage: python scripts/exec_brief/build_2026_09_08_terse.py <facts.json> <previous.html> <out.html>
"""
import json
import re
import sys
from html import escape

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
from charts import bars                                # noqa: E402
from logic_exhibits import downstream, logic_lanes     # noqa: E402

facts_path, prev_path, out_path = sys.argv[1:4]
J = json.load(open(facts_path, encoding="utf-8"))
F = {k: v["value"] for k, v in J["facts"].items()}
prev = open(prev_path, encoding="utf-8").read()
style = re.search(r"<style>[\s\S]*?</style>", prev).group(0)
script = re.search(r"<script>[\s\S]*?</script>", prev).group(0)

pending = F["pendingAcceptances"]
if pending != 2:
    sys.exit(f"refusing: this page states TWO asks and facts.json says {pending} are pending")
human, auto, days = F["scopeDecisionsHumanAccepted"], F["scopeDecisionsAutoAccepted"], F["scopeDaysSinceRule"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
ready = F["readyItems"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")

THEIR_WORDS = ["the decision brief is still far too verbose.", "make it use caveman?"]

fig_empty = logic_lanes(
    "An empty queue today shows a stale ask.",
    ("today", [("last ask answered", "", "muted", ""),
               ("queue empty", "", "muted", ""),
               ("an ask is required", "", "bad", "gap"),
               ("stale page", "", "bad", "")],
     ["", "", ""]),
    ("after", [("last ask answered", "", "muted", ""),
               ("queue empty", "", "muted", ""),
               ("receipt", "no ask", "accent", ""),
               ("next ask replaces it", "", "accent", "")],
     ["", "", ""]))

fig_empty_lands = downstream(
    "The receipt lands on four places.",
    ("receipt shape", ""),
    [("page check", "", "one branch", "ok"),
     ("receipt builder", "", "new", "ok"),
     ("surfacing skill", "", "one step", "ok"),
     ("this page", "", "same day", "accent")])

fig_terse = logic_lanes(
    "A check now holds the budgets; the cap waits on you.",
    ("today", [("section budgets", "18, 70, 60", "muted", ""),
               ("no check", "", "bad", "gap"),
               ("long page passes", "", "bad", "")],
     ["", ""]),
    ("after", [("check holds budgets", "fixed", "ok", ""),
               ("page capped at 450", "your word", "accent", ""),
               ("check refuses more", "", "accent", "")],
     ["", ""]))

fig_census = bars(
    "Since your 5 September rule, your word and standing consent are equal.",
    [("your word", human, "warn", f"in {days} days"),
     ("standing consent", auto, "muted", ""),
     ("waiting", pending, "accent", "")],
    unit="")

quotes = "".join(f"<blockquote><em>{escape(w)}</em></blockquote>" for w in THEIR_WORDS)

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Accept two changes to this page: a receipt when nothing waits, and a 450-word cap</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One page &middot; two asks &middot; both about this page</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Two asks.</strong> One: when nothing waits, the page becomes a receipt with no ask. Two: the page is terse and capped at 450 words; a check refuses more. I recommend both. Each changes what the process publishes, so your 5 September rule holds them for your word.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Signed by you since the rule</b> {human}</span><span class="chip"><b>Auto-accepted since the rule</b> {auto}</span><span class="chip ok"><b>Tests</b> {tests} passing, {failed} failing</span><span class="chip"><b>Checks</b> {guards}, {viol} violations</span></div>

<h2>What you said</h2>
{quotes}

<h2>Ask one &mdash; the receipt shape</h2>
{fig_empty}
<p>The contract demands an ask on every page. After your last answer the queue was empty, so the page showed an answered question. A receipt states what you answered and what it set in motion. <strong>Wrong if</strong> the queue is never empty a full day.</p>
{fig_empty_lands}
<div class="opts" data-records="d0376"><label><input type="radio" name="ask-empty" value="Accept: the brief becomes a receipt when nothing waits">Accept: receipt when nothing waits</label><label><input type="radio" name="ask-empty" value="Reverse: do not republish when the queue is empty">Reverse: do not republish when empty</label><label><input type="radio" name="ask-empty" value="Discuss">Discuss</label></div>

<h2>Ask two &mdash; terse and capped</h2>
{fig_terse}
<p>The contract budgets the headline, the ask and the provenance at 18, 70 and 60 words. No check held them; the last page passed clean over all three. That check is fixed. The 450-word cap on the whole page, in the caveman-lite register you named, needs your word. <strong>Wrong if</strong> a fork's costs cannot fit in 450 words; then the cap rises, not the register.</p>
{fig_census}
<div class="opts" data-records="d0377"><label><input type="radio" name="ask-terse" value="Accept: caveman-lite register and a 450-word cap">Accept: caveman-lite, 450-word cap</label><label><input type="radio" name="ask-terse" value="Cap only: 450 words, plain register">Cap only, plain register</label><label><input type="radio" name="ask-terse" value="Discuss">Discuss</label></div>

<h2>Since the last page</h2>
<p>Your consent answer is recorded and built. Two asks joined the queue.</p>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {ready} ready items. {human} decisions on your word and {auto} auto-accepted since 2026-09-05. {pending} await your word. Numbers come from the computed facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
