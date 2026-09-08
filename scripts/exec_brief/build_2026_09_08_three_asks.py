#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (fifth) queue change, terse register:
three asks - the empty-queue receipt shape (D0376), the 450-word cap (D0377), and what the project
buys from the skill-routing probe (D0378, a four-way fork on spend).

Usage: python scripts/exec_brief/build_2026_09_08_three_asks.py <facts.json> <previous.html> <out.html>
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
if pending != 3:
    sys.exit(f"refusing: this page states THREE asks and facts.json says {pending} are pending")
human, auto, days = F["scopeDecisionsHumanAccepted"], F["scopeDecisionsAutoAccepted"], F["scopeDaysSinceRule"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
ready = F["readyItems"]
prompts = F["routingPrompts"]
cost_lo, cost_hi = F["routingRunCostLowUsd"], F["routingRunCostHighUsd"]
flipped, compared = F["routingVerdictsFlipped"], F["routingVerdictsCompared"]
sweep_usd, sweep_min = F["routingThreeSamplesUsd"], F["routingThreeSamplesMinutes"]
subset_usd, subset_min = F["routingSubsetThreeSamplesUsd"], F["routingSubsetThreeSamplesMinutes"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready),
                ("prompts", prompts), ("cost_lo", cost_lo), ("cost_hi", cost_hi), ("flipped", flipped),
                ("compared", compared), ("sweep_usd", sweep_usd), ("sweep_min", sweep_min),
                ("subset_usd", subset_usd), ("subset_min", subset_min)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")

THEIR_WORDS = ["the decision brief is still far too verbose.", "make it use caveman?"]

fig_empty = logic_lanes(
    "An empty queue today shows a stale ask.",
    ("today", [("queue empty", "", "muted", ""),
               ("an ask is required", "", "bad", "gap"),
               ("stale page", "", "bad", "")],
     ["", ""]),
    ("after", [("queue empty", "", "muted", ""),
               ("receipt", "no ask", "accent", ""),
               ("next ask replaces it", "", "accent", "")],
     ["", ""]))

fig_empty_lands = downstream(
    "The receipt lands on three places.",
    ("receipt shape", ""),
    [("page check", "", "one branch", "ok"),
     ("receipt builder", "", "new", "ok"),
     ("this page", "", "same day", "accent")])

fig_terse = logic_lanes(
    "A check holds the budgets; the cap waits on you.",
    ("today", [("section budgets", "18, 70, 60", "muted", ""),
               ("no check", "", "bad", "gap"),
               ("long page passes", "", "bad", "")],
     ["", ""]),
    ("after", [("check holds budgets", "fixed", "ok", ""),
               ("page capped at 450", "your word", "accent", ""),
               ("check refuses more", "", "accent", "")],
     ["", ""]))

fig_census = bars(
    "Your word and standing consent are equal since 5 September.",
    [("your word", human, "warn", f"in {days} days"),
     ("standing consent", auto, "muted", ""),
     ("waiting", pending, "accent", "")],
    unit="")

fig_routing = logic_lanes(
    "One sample is not a measurement; three on one skill are.",
    ("today", [("one sample per skill", "", "muted", ""),
               ("verdict flips", f"{flipped} of {compared}", "bad", "gap"),
               ("number looks real", "", "bad", "")],
     ["", ""]),
    ("after C", [("a suspected skill", "", "muted", ""),
                 ("three samples", "before, after", "ok", ""),
                 ("no standing number", "", "accent", "")],
     ["", ""]))

fig_cost = bars(
    "What each course costs per refresh, in dollars.",
    [(f"A: all {prompts}, three samples", sweep_usd, "warn", f"{sweep_min} min"),
     ("B: 16 prompts, three samples", subset_usd, "muted", f"{subset_min} min"),
     ("C: on demand", 0, "accent", "")],
    unit="$")

quotes = "".join(f"<blockquote><em>{escape(w)}</em></blockquote>" for w in THEIR_WORDS)

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Three asks wait: a receipt shape, a 450-word cap, and the routing probe&#8217;s price</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">Three asks</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Three asks.</strong> One: when nothing waits, this page becomes a receipt. Two: the page is capped at 450 words. Three: what the routing probe should produce. I recommend the receipt, the cap, and a diagnostic with no standing number. Two change what the process publishes; one spends your money.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Signed by you since the rule</b> {human}</span><span class="chip"><b>Auto-accepted since the rule</b> {auto}</span></div>

<h2>Your words</h2>
{quotes}

<h2>Ask one &mdash; the receipt shape</h2>
{fig_empty}
<p>Every page must carry an ask. When the queue is empty, that ask is stale. A receipt states what you answered and what it set in motion. <strong>Wrong if</strong> the queue is never empty a full day.</p>
{fig_empty_lands}
<div class="opts" data-records="d0376"><label><input type="radio" name="ask-empty" value="Accept: the brief becomes a receipt when nothing waits">Accept: receipt when nothing waits</label><label><input type="radio" name="ask-empty" value="Reverse: do not republish when the queue is empty">Reverse: do not republish when empty</label><label><input type="radio" name="ask-empty" value="Discuss">Discuss</label></div>

<h2>Ask two &mdash; terse and capped</h2>
{fig_terse}
<p>The contract budgets the headline, ask and provenance at 18, 70 and 60 words. No check held them; that is fixed. The 450-word cap needs your word. <strong>Wrong if</strong> a fork's costs cannot fit; then the cap rises.</p>
{fig_census}
<div class="opts" data-records="d0377"><label><input type="radio" name="ask-terse" value="Accept: caveman-lite register and a 450-word cap">Accept: caveman-lite, 450-word cap</label><label><input type="radio" name="ask-terse" value="Cap only: 450 words, plain register">Cap only, plain register</label><label><input type="radio" name="ask-terse" value="Discuss">Discuss</label></div>

<h2>Ask three &mdash; what the routing probe buys</h2>
{fig_routing}
<p>The probe asks a real model one request per process and reads which skill it invoked. {flipped} of {compared} verdicts flipped between identical runs. A run costs ${cost_lo} to ${cost_hi}. I recommend C: the rig serves one suspected skill, three samples before and after a rewrite, no standing number. <strong>Wrong if</strong> you want to compare skills across releases; then A.</p>
{fig_cost}
<div class="opts" data-records="d0378"><label><input type="radio" name="ask-routing" value="A: repeated sweep, three samples of every prompt, a standing rate">A: all {prompts}, three samples</label><label><input type="radio" name="ask-routing" value="B: stratified subset, three samples, a standing rate over the subset">B: 16 prompts, three samples</label><label><input type="radio" name="ask-routing" value="C: diagnostic rig only, no standing number (recommended)">C: diagnostic, no number</label><label><input type="radio" name="ask-routing" value="D: as it stands">D: as it stands</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {ready} ready items. {human} on your word, {auto} auto-accepted since 2026-09-05. {pending} wait. Numbers come from the facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
