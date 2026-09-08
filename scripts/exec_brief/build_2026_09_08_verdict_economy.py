#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (sixth) queue change, terse register under
D0377's 450-word cap: one ask - how verdicts are obtained (D0381, their own words on the routing
probe: objective once, a flaky check is a measurement defect, the subjective is sampled to them).

Usage: python scripts/exec_brief/build_2026_09_08_verdict_economy.py <facts.json> <previous.html> <out.html>
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
if pending != 1:
    sys.exit(f"refusing: this page states ONE ask and facts.json says {pending} are pending")
human, auto, days = F["scopeDecisionsHumanAccepted"], F["scopeDecisionsAutoAccepted"], F["scopeDaysSinceRule"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
ready = F["readyItems"]
cost_lo, cost_hi = F["routingRunCostLowUsd"], F["routingRunCostHighUsd"]
flipped, compared = F["routingVerdictsFlipped"], F["routingVerdictsCompared"]
sweep_usd = F["routingThreeSamplesUsd"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready),
                ("cost_lo", cost_lo), ("cost_hi", cost_hi), ("flipped", flipped), ("compared", compared),
                ("sweep_usd", sweep_usd)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")

THEIR_WORDS = [
    "some of these items are just poorly measured, and that the measurement itself should be critiqued.",
    "We don't really need a panel of agents to perform the same test, especially if that test is objective.",
    "Where it's not objective, I should probably be included in validating the result at some point",
]

fig_lanes = logic_lanes(
    "A flaky check is a defect to critique, not a rate to buy.",
    ("today", [("verdict flips", f"{flipped} of {compared}", "bad", ""),
               ("more samples", f"${sweep_usd} a run", "bad", "gap"),
               ("a rate", "", "muted", "")],
     ["", ""]),
    ("after", [("verdict flips", "", "muted", ""),
               ("issue against the rubric", "", "accent", ""),
               ("subjective: quote, you sample", "", "accent", "")],
     ["", ""]))

fig_lands = downstream(
    "The rule lands on three places.",
    ("verdict rule", ""),
    [("routing probe", "", "no sampling", "ok"),
     ("panels", "", "none new", "ok"),
     ("sitting review", "", "your sample", "accent")])

fig_census = bars(
    "Your word and standing consent are equal since 5 September.",
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
<h1 data-digest="title">One ask waits: your note on verdicts becomes the rule</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One ask</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>One ask.</strong> Your note on the routing probe, written as a rule: an objective check runs once; a check that disagrees with itself is a defect to critique, never averaged; a subjective verdict quotes its evidence and a sample comes to you. I recommend accepting. It changes how three processes obtain verdicts, so it needs your word.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Signed by you since the rule</b> {human}</span><span class="chip"><b>Auto-accepted since the rule</b> {auto}</span></div>

<h2>Your words</h2>
{quotes}

<h2>The ask &mdash; how verdicts are obtained</h2>
{fig_lanes}
<p>{flipped} of {compared} routing verdicts flipped between identical runs at ${cost_lo} to ${cost_hi} each. The three shapes I offered all bought more samples. Your note says the instrument is wrong, not undersampled. <strong>Wrong if</strong> a check turns out to need a tie-break no rubric can fix; then a second judge returns for that check only.</p>
{fig_lands}
{fig_census}
<div class="opts" data-records="d0381"><label><input type="radio" name="ask-verdict" value="Accept: objective once, flaky is a defect, subjective is quoted and sampled to me">Accept: objective once, subjective sampled to you</label><label><input type="radio" name="ask-verdict" value="Reverse: keep repeated sampling where a check is flaky">Reverse: keep repeated sampling</label><label><input type="radio" name="ask-verdict" value="Discuss">Discuss</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {ready} ready items. {human} on your word, {auto} auto-accepted since 2026-09-05. {pending} waits. Numbers come from the facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
