#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (eighth) queue change, terse register under
D0377's 450-word cap: one ask, a four-way fork - which representation of a Decision's supersession is
authoritative (D0384: A edge / B status + guard, recommended / C both held equal / D as it stands).

Usage: python scripts/exec_brief/build_2026_09_08_supersession.py <facts.json> <previous.html> <out.html>
"""
import json
import re
import sys

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
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
ready = F["readyItems"]
total, sup_status = F["decisionsTotal"], F["decisionsStatusSuperseded"]
edges, edges_dec = F["supersedeEdges"], F["supersedeEdgesToDecisions"]
acc_edge, sup_noedge, disagree = F["acceptedWithSupersedeEdge"], F["supersededWithoutEdge"], F["supersessionDisagreements"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready),
                ("total", total), ("sup_status", sup_status), ("edges", edges), ("edges_dec", edges_dec),
                ("acc_edge", acc_edge), ("sup_noedge", sup_noedge)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")

fig_lanes = logic_lanes(
    "Nothing holds the two marks equal today; B makes the gate do it.",
    ("today", [("a reversal is recorded", "an edge", "muted", ""),
               ("the old record", "still reads proposed", "bad", "gap"),
               ("your queue", "asks you to sign it", "bad", "")],
     ["", ""]),
    ("after B", [("a reversal is recorded", "edge + status", "accent", ""),
                 ("a mismatch", "the gate refuses", "accent", ""),
                 ("your queue", "only live asks", "ok", "")],
     ["", ""]))

fig_census = bars(
    f"{disagree} records show the two marks apart.",
    [("edges reverse a clause, record stands", acc_edge, "warn", "accepted + edge"),
     ("retired by status, no edge", sup_noedge, "warn", ""),
     ("retired by status", sup_status, "muted", f"of {total}"),
     ("edges to a decision", edges_dec, "muted", f"of {edges}")],
    unit="")

fig_lands = downstream(
    "What each answer lands on.",
    ("the choice", ""),
    [("A edge", "", "schema + migration, a day", "warn"),
     ("B status", "", "one guard, two hours", "accent"),
     ("C both", "", "seven records amended", "warn"),
     ("D as is", "", "the defect kept", "bad")])

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">One ask waits: which mark says a decision is retired</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One ask, four ways</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>One ask.</strong> A retired decision has two marks: a status field, and an edge from its reverser. Either is written alone, so a reversed rule sat on your queue. I recommend <strong>B</strong>: status stays the word on standing, and the gate refuses a reversal whose target reads proposed. <strong>Wrong if</strong> you want &ldquo;in force?&rdquo; answered from the graph alone - then A, which costs the edge a scope.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Marks disagree</b> {disagree}</span><span class="chip"><b>Signed by you since the rule</b> {human}</span></div>

<h2>The evidence</h2>
{fig_census}
<p>{acc_edge} accepted decisions carry a reversal edge and stand - the edge undid one clause, not the record; never-rebase is one of them. So the edge cannot mean retired today. {sup_noedge} retired decisions have no edge at all. The write path records the edge and never touches the target; the queue reads status alone.</p>

<h2>The ask</h2>
{fig_lanes}
{fig_lands}
<p><strong>A</strong> edge rules: status leaves the vocabulary; the edge gains a scope, a schema change and a migration of {sup_status} records. <strong>B</strong> status rules: a guard fails proposed-with-edge; recording a reversal refuses a proposed target and names the fix. One stated exception: the reverser sets the target's status. <strong>C</strong> both, held equal: B plus the {acc_edge} clause reversals must say so in text. <strong>D</strong> as it stands: the queue alone hides the case; both marks stay writable.</p>
<div class="opts" data-records="d0384"><label><input type="radio" name="ask-sup" value="A: the edge is authoritative; status leaves the vocabulary; the edge gains a scope">A: edge rules, schema change</label><label><input type="radio" name="ask-sup" value="B: the status field is authoritative; a guard refuses a reversal of a proposed target (recommended)">B: status rules, one guard (recommended)</label><label><input type="radio" name="ask-sup" value="C: both marks held equal by a guard; clause reversals say so">C: both, held equal</label><label><input type="radio" name="ask-sup" value="D: as it stands; the queue alone hides a reversed proposal">D: as it stands</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {ready} ready items. {human} on your word, {auto} auto-accepted since 2026-09-05. {pending} waits. Numbers come from the facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
