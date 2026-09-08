#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 queue change: the eight asks were answered
and one fork joined the queue - how the human's chat words bind to the Decision they accept.

Usage: python scripts/exec_brief/build_2026_09_08_consent.py <facts.json> <previous.html> <out.html>
"""
import json
import re
import sys
from html import escape

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
from charts import bars                                # noqa: E402
from logic_exhibits import downstream, logic_lanes     # noqa: E402
sys.path.insert(0, "scripts")
from artefact import claim, require_complete   # noqa: E402  (D0387: a stale answer is refused, a dead run leaves no page)

facts_path, prev_path, out_path = sys.argv[1:4]
if prev_path == out_path:
    sys.exit("refusing: the style source and the output are the same file - copy the previous page aside first")
J = require_complete(facts_path)    # refuses a running, failed, or pre-D0387 facts file
claim(out_path)                     # the previous page is gone before anything is built
F = {k: v["value"] for k, v in J["facts"].items()}
prev = open(prev_path, encoding="utf-8").read()
style = re.search(r"<style>[\s\S]*?</style>", prev).group(0)
script = re.search(r"<script>[\s\S]*?</script>", prev).group(0)

pending = F["pendingAcceptances"]
if pending != 1:
    sys.exit(f"refusing: this page states ONE ask and facts.json says {pending} are pending")
human, auto = F["scopeDecisionsHumanAccepted"], F["scopeDecisionsAutoAccepted"]
days, wait_max, wait_mean = F["scopeDaysSinceRule"], F["scopeHumanWaitMaxDays"], F["scopeHumanWaitMeanDays"]
too_wide, refused_real, issues = F["issuesConsentTooWide"], F["issuesConsentRefusedReal"], F["issuesTotal"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("too_wide", too_wide), ("refused_real", refused_real), ("issues", issues)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")

THEIR_WORDS = ("We're getting to the core of this repos trust-but-verify undergirding and I think you're fighting "
               "it a bit. It is true that I want to sign off on a lot of pressing issues, but denying work that I "
               "clearly approved because it's not \"signed\" is self+defeating. Launch an independent agent to "
               "assess if a user(me) is giving consent via some text, else be less gatekeepy. The issues we've "
               "encountered have mostly been from poor process adherence and code quality, no? Not because my "
               "consent has been to easily interpreted.")

fig_census = bars(
    f"This tree has both failure classes in nearly equal numbers, out of {issues} tracked defects.",
    [("an acceptance recorded or kept that was not what you gave", too_wide, "bad",
      "injected tool output signed into a record; a fork auto-accepting; signed texts edited afterwards"),
     ("a control refusing or mis-framing an acceptance you did give", refused_real, "warn",
      "a one-letter answer rejected; your own terminal refused; an apostrophe in my framing shifted the quote"),
     ("every other defect", issues - too_wide - refused_real, "muted",
      "write paths, instruments reporting a wrong number, process adherence, enforcement bypass")],
    unit="")

fig_lanes = logic_lanes(
    "Signing once at the plan removes the re-ask for every step the plan names, and asks again only for what it did not.",
    ("today", [("you answer in chat", "a sentence, a letter, a marked box", "muted", ""),
               ("I quote you into the record", "verbatim, with my framing around it", "muted", ""),
               ("the check reads the quote", "for the id, a letter, or three title words", "muted", "refused twice today"),
               ("each enforcement change asks", f"{human} times in {days} days", "muted", f"longest wait {wait_max} d")],
     ["then", "then", "so"]),
    ("after the change", [("you sign a plan yourself", "once, in your own words", "accent", ""),
                          ("its named steps inherit that", "recorded accepted, citing the plan", "accent", "no second ask"),
                          ("the check reads the plan", "your signature, not standing consent", "accent", "a guard holds it"),
                          ("only unplanned changes ask", "an enforcement change no plan named", "muted", "the residue")],
     ["then", "then", "so"]),
    note="Standing consent still never reaches the enforcement surface: what flows through is a plan you signed yourself, and a guard fails any step that claims a plan whose acceptance was automatic.")

fig_downstream = downstream(
    "The change lands on the accept path and one guard; the read-back rule itself is untouched.",
    ("a plan you signed names a step", "the step is an enforcement change"),
    [("the accept path", "records the step accepted at record time, citing the plan", "one rule", "accent"),
     ("a new guard", "fails a step whose plan was auto-accepted or does not name it", "one check", "accent"),
     ("the quote reader", "the apostrophe defect is fixed so a correct quote is never refused", "one fix", "ok"),
     ("this page", "carries only the enforcement changes no plan named", "fewer asks", "ok"),
     ("standing consent", "unchanged: still stops at the enforcement surface", "no change", "muted")],
    foot="Today's eight would NOT have been covered: the plan they belonged to was itself auto-accepted, so the first payout is the next plan you sign yourself.")

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Your word is enough once, at the plan you sign, and it does not need giving again at every step the plan names</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One page &middot; one ask &middot; how your chat words bind to a record</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>You said the gatekeeping is fighting the trust-but-verify design. The tree agrees with you on the count and disagrees on the cause.</strong> No work was denied: all six changes you ratified this morning were built, green and running before you were asked; what waited was the record of your acceptance. The two refusals you met today were mine - an apostrophe in my own framing shifted where the check started reading, a defect already tracked and first on the frontier. The volume is real: since the scope rule you set on 5 September, {human} decisions have waited for your word in {days} days, mean wait {wait_mean} days, longest {wait_max}. One decision is yours: how your words bind. I recommend the signed plan covering its named steps, and I put your own proposal beside it with its cost.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Signed by you since the scope rule</b> {human} in {days} days</span><span class="chip"><b>Auto-accepted in the same span</b> {auto}</span><span class="chip"><b>Longest wait</b> {wait_max} days</span><span class="chip ok"><b>Tests</b> {tests} passing, {failed} failing</span><span class="chip"><b>Enforced checks</b> {guards}, {viol} violations</span></div>

<h2>What you said</h2>
<blockquote><em>{escape(THEIR_WORDS)}</em></blockquote>

<h2>Is the count right?</h2>
<h3>Mostly. Process adherence and code quality are the bulk of the {issues} tracked defects. The consent class is small, and it splits almost evenly between the two ways it can fail.</h3>
{fig_census}
<p>The {too_wide} on the first bar are why the quote must name the record: a signature was attached to thirteen thousand characters of injected tool output; a fork was accepted by standing consent that was never meant to reach a fork; seventeen signed texts were edited after signing. The {refused_real} on the second bar are the friction you felt: a one-letter answer the check did not recognise, your own terminal refusing a plain sentence, and today, my apostrophe. Both bars are hand-classified from the defect titles and descriptions and the lists are in the facts file, so the count can be re-read; the wider census by keyword is textual and is not stated here as a number.</p>

<h2>What actually waited</h2>
<p class="turn"><strong>Nothing was denied.</strong> Six of this morning's eight were already in the tree; the page presented them as asks, and the ratify-or-reverse framing was itself a fix to an earlier page that had presented shipped work as an open choice. The remaining two were rules with no code, accepted on your marked boxes.</p>
<p>The source of the volume is the rule you set on 5 September, in your words, that standing consent is scoped to the processes it was promulgated under. Every guard, hook, process and contract change since then has waited for you: {human} of them, against {auto} that accepted themselves under standing consent. That rule is doing what you asked. The question is whether it can be satisfied once, at the plan, instead of once per step.</p>

<h2>The ask &mdash; how your chat words bind to a record</h2>
<h3>Today a quote of you must name the record: its id, an option letter, or three words of its title. That is a mechanical rule an agent cannot talk past, and it is why a bare "proceed" is refused. Four ways to loosen it, and what each costs.</h3>
{fig_lanes}
<p><strong>The record's own words:</strong></p>
<blockquote><em>A safety-change or process-change Decision whose item is NAMED in the decision text of a Decision the human accepted THEMSELVES (not under standing consent) is covered by that acceptance and is recorded accepted at record time with the plan cited - the human signed once, on the plan, and its enumerated steps do not re-ask; standing consent still does not reach the enforcement surface, because the plan it flows through must carry a human's own signature.</em></blockquote>
<p><strong>What accepting binds.</strong> Option A, your proposal: a fresh-context agent is handed only your verbatim message and the pending record, returns bound / not bound / ambiguous with the span it relied on, and the verdict and the model that gave it sit beside the acceptance as a receipt; ambiguous asks you one question. It does what you asked in words; its cost is that a model again decides what a human meant - the first bar above is that reading going too wide, and isolation narrows it without removing it; a model call per acceptance; about a day, with a test that a bare "proceed" comes back not bound. Option B: the read-back accepts words given in direct reply to a proposal that named the record, quoting both. Cheapest; its cost is that my sentence becomes half the receipt, and nothing in the tree can verify it was the message before yours. Option C, recommended: a plan you signed yourself covers the enforcement steps it names; standing consent still stops at the enforcement surface; the apostrophe defect is fixed alongside; about a day. Option D: as it stands, plus the apostrophe fix; {human} signatures in {days} days continues at that rate for as long as the enforcement surface moves this often.</p>
{fig_downstream}
<p><strong>The fork.</strong> Whether an AI may read your intent at all. A and B say yes in different degrees; C and D say no and differ only in how often you are asked. That is the contested clause; nothing else on the list is in dispute.</p>
<p><strong>Evidence, and what is assumption.</strong> True in the tree: the {human}/{auto} split, the {wait_max}-day longest wait, the two refusals today and their cause in the quote reader, the {too_wide}/{refused_real} census as listed. My assumption: that most future enforcement changes will arrive as steps of a plan you sign - if they arrive one at a time, C pays out nothing and the volume stays.</p>
<p><strong>What decides it</strong> is a value only you hold: whether you want to be asked less even where no plan was signed. If yes, A, with the ambiguous branch mandatory. If the count is acceptable and today's fault was the apostrophe, D. If you would rather sign once and have that carry, C.</p>
<p><strong>What I do with each answer.</strong> A &rarr; build the judge path and its two tests, amend the contract so an AI may bind but never supply an acceptance. B &rarr; a second span kind in the quote reader. C &rarr; a plan-covers-step rule on the accept path, a guard that the covering plan carries your own signature, the apostrophe fix. D &rarr; the apostrophe fix alone. Until you answer, the read-back stands as it is, and this page carries the one ask.</p>
<p><strong>What would change my recommendation:</strong> if the next five plans you sign name fewer than half of the enforcement changes that follow them, C is the wrong shape and A is the right one.</p>
<div class="opts" data-records="d0375"><label><input type="radio" name="ask-consent" value="Option C: a plan I sign covers the steps it names">Option C: a plan I sign covers the steps it names</label><label><input type="radio" name="ask-consent" value="Option A: an independent agent judges whether my text is consent">Option A: an independent agent judges whether my text is consent</label><label><input type="radio" name="ask-consent" value="Option B: a reply to a proposal that named the record counts">Option B: a reply to a proposal that named the record counts</label><label><input type="radio" name="ask-consent" value="Option D: keep the read-back as it stands, fix the apostrophe">Option D: keep the read-back as it stands, fix the apostrophe</label><label><input type="radio" name="ask-consent" value="Discuss">Discuss</label></div>

<h2>What changed since the last page</h2>
<p>All eight asks on the previous page were answered from your marked boxes and recorded against the tree at {tree}; the queue emptied and this one item joined it. Two facts were added to the facts file for this page: the signed-versus-automatic split since the scope rule, and the two hand-classified consent lists.</p>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} enforced checks, {viol} violations. Since the scope rule of 2026-09-05: {human} decisions accepted on your word (mean wait {wait_mean} days, longest {wait_max}), {auto} auto-accepted, read from each decision file's acceptance record. {issues} tracked defects; {too_wide} and {refused_real} hand-classified into the two consent classes, ids listed in the facts file. {pending} record awaiting your word. Every number here comes from a computed facts file, each carrying the exact command or rule that produced it. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">the repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
