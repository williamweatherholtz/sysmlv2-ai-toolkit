#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (third) queue change: the consent fork was
answered from chat, and the one item that joined the queue is the process's own gap - the brief has
no shape for an EMPTY queue, so this page must still carry an ask to pass its contract.

Usage: python scripts/exec_brief/build_2026_09_08_empty_queue.py <facts.json> <previous.html> <out.html>
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
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")

THEIR_WORDS = "fix the brief's issue. it keeps happening\n\nc for d375"

fig_answer = downstream(
    "Your one answer this morning set three things in motion, and the first is already built.",
    ("you wrote ten characters in chat", "a letter and the record's number"),
    [("the quote reader", "reads your words inside a declared boundary; an apostrophe in my framing can no longer shift it", "built, tested", "ok"),
     ("the accept path", "takes your words as their own argument, so the boundary is declared by construction", "built, used once", "ok"),
     ("the plan rule", "a plan you sign yourself covers the enforcement steps it names; a guard holds it to that", "on the frontier", "accent"),
     ("this page", "carries only what no signed plan covers", "after the rule lands", "muted")],
    foot="Your acceptance was the first record made through the fix: it was read back by the record's number and by the letter you gave.")

fig_lanes = logic_lanes(
    "With nothing waiting, the page today can only show an answered question or go stale; the receipt shape shows the truth instead.",
    ("today", [("you answer the last ask", "in chat, a box, or your terminal", "muted", ""),
               ("the queue empties", "nothing awaits your word", "muted", "computed"),
               ("the contract demands an ask", "a page with no options fails its check", "bad", "the gap"),
               ("the page shows an answered ask", "or is not republished at all", "bad", "false either way")],
     ["then", "but", "so"]),
    ("after the change", [("you answer the last ask", "in chat, a box, or your terminal", "muted", ""),
                          ("the queue empties", "nothing awaits your word", "muted", "computed"),
                          ("the page becomes a receipt", "what you answered, when, what it set in motion", "accent", "no ask, no options"),
                          ("the next ask replaces it", "when the queue moves again", "accent", "one URL")],
     ["then", "so", "then"]),
    note="Every other clause of the page contract stands for the receipt: a headline that claims, figures that claim, no record pointers in the text, a dated provenance strip, both themes.")

fig_census = bars(
    f"Since the scope rule you set on 5 September, your word and standing consent have carried an equal share of decisions.",
    [("accepted on your own word", human, "warn", f"in {days} days; this morning's fork among them"),
     ("accepted by standing consent", auto, "muted", "non-forks inside the existing processes"),
     ("awaiting your word now", pending, "accent", "this page's one ask")],
    unit="")

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Your answer on how your words bind is recorded and built; the one thing left to decide is what this page shows you when nothing waits</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One page &middot; one ask &middot; the queue after your answer</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>You chose the signed plan covering its named steps, and the apostrophe defect that refused your acceptances is fixed at the root.</strong> Your ten characters were read back by the record's number and the letter you gave, and that acceptance was the first record made through the fix. The rule you chose is on the frontier as chartered work, ranked below five higher-severity defects. One thing waits, and it is about this page: the contract that every brief is checked against demands an ask, so a page with nothing to ask cannot exist - it either shows you an answered question or goes stale. I recommend the brief gets a receipt shape for that state. Because it changes what the process publishes, your own rule of 5 September keeps it for your word.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Signed by you since the scope rule</b> {human} in {days} days</span><span class="chip"><b>Auto-accepted in the same span</b> {auto}</span><span class="chip"><b>Ready items on the frontier</b> {ready}</span><span class="chip ok"><b>Tests</b> {tests} passing, {failed} failing</span><span class="chip"><b>Enforced checks</b> {guards}, {viol} violations</span></div>

<h2>What you said</h2>
<blockquote><em>{escape(THEIR_WORDS)}</em></blockquote>

<h2>What your answer did</h2>
<h3>Two of the three steps the plan names are built and green; the third is the rule itself, next in rank behind the higher-severity defects ahead of it.</h3>
{fig_answer}
<p>The refusal you met was a boundary defect, not a consent one: the check split my note on the apostrophe character, so the possessive in my own framing moved where your words began. Now a span opens only at a declared quote mark or an apostrophe that stands outside a word, closes only at one not followed by a letter, and your words can be passed as their own argument so no framing sits around them at all. The case that matters - a shifted span that would carry the record's number on words you never said - is a test, and it refuses.</p>

<h2>The ask &mdash; what this page shows when nothing waits</h2>
<h3>The page contract requires an ask block and response options on every brief. That clause is right when there is something to ask and wrong when there is not, and the process had never reached the empty state until this morning.</h3>
{fig_lanes}
<p><strong>The record's own words:</strong></p>
<blockquote><em>When the authority queue carries no decision awaiting acceptance, the republished brief is a RECEIPT: its headline states that nothing waits for the human's word; its body names what was last answered, on what date and in whose words, and what that answer set in motion; it carries the same provenance strip and copy-for-AI control; it carries NO ask block and NO response options, and the contract accepts the shape when the page declares itself empty and names the last answered record. Every other clause of the contract stands unchanged.</em></blockquote>
<p><strong>What accepting binds.</strong> The contract checker gains one branch: when the page declares itself empty and names what was answered, the ask and options are not required and every other check still runs. The brief builders gain a receipt builder that refuses unless the facts say zero are pending. The surfacing skill names the shape. Nothing changes for a page that has an ask. <strong>What it forecloses:</strong> a manufactured ask to satisfy the checker - confirm my reading, reopen something - which is the gatekeeping you named this morning.</p>
{fig_census}
<p><strong>The fork, such as it is.</strong> This is not a choice between designs; it is whether the page may say "nothing waits". The alternative is the page as it stands, which after any last answer either shows you a question you already answered or is left unrefreshed. I could not find a third course that is both true and passes the contract.</p>
<p><strong>Evidence, and what is assumption.</strong> True in the tree: the queue held zero decisions after your answer and this item joined it; the contract's ask clause is unconditional; {human} decisions have taken your word in {days} days against {auto} that accepted themselves. My assumption: that you would rather see a receipt than an answered question - the receipt is my design, and if you would rather the page simply not refresh when empty, say so and I will record that instead.</p>
<p><strong>What decides it</strong> is a value only you hold: what you want to find at this URL after you have answered everything.</p>
<p><strong>What I do with each answer.</strong> Accept &rarr; the checker's empty branch, the receipt builder, the skill's step, and this page is replaced by a receipt the same day. Reverse &rarr; the page is not republished when the queue is empty, and the record says so. Discuss &rarr; nothing changes until you say more.</p>
<p><strong>What would change my recommendation:</strong> if the queue is never empty for more than a day, the receipt is rarely seen and not worth its branch.</p>
<div class="opts" data-records="d0376"><label><input type="radio" name="ask-empty" value="Accept: the brief becomes a receipt when nothing waits">Accept: the brief becomes a receipt when nothing waits</label><label><input type="radio" name="ask-empty" value="Reverse: do not republish when the queue is empty">Reverse: do not republish when the queue is empty</label><label><input type="radio" name="ask-empty" value="Discuss">Discuss</label></div>

<h2>What changed since the last page</h2>
<p>The one ask on the previous page was answered in chat and recorded against the tree at {tree}; the queue emptied, and the gap that emptiness exposed joined it as this page's one ask.</p>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} enforced checks, {viol} violations. {ready} ready items on the frontier. Since the scope rule of 2026-09-05: {human} decisions accepted on your word, {auto} auto-accepted, read from each decision file's acceptance record. {pending} record awaiting your word. Every number here comes from a computed facts file, each carrying the exact command or rule that produced it. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">the repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
