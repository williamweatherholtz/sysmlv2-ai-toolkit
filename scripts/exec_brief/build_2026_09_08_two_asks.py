#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (ninth) queue change, terse register under
D0377's 450-word cap: two asks - which mark says a decision is retired (D0384, four options, B
recommended) and what the recall hook does with facts that arrive after its cap (D0390, three
options, A recommended).

Usage: python scripts/exec_brief/build_2026_09_08_two_asks.py <facts.json> <previous.html> <out.html>
"""
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
J = require_complete(facts_path)
claim(out_path)
F = {k: v["value"] for k, v in J["facts"].items()}
prev = open(prev_path, encoding="utf-8").read()
style = re.search(r"<style>[\s\S]*?</style>", prev).group(0)
script = re.search(r"<script>[\s\S]*?</script>", prev).group(0)

pending = F["pendingAcceptances"]
if pending != 2:
    sys.exit(f"refusing: this page states TWO asks and facts.json says {pending} are pending")
human, auto = F["scopeDecisionsHumanAccepted"], F["scopeDecisionsAutoAccepted"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
ready = F["readyItems"]
total, sup_status = F["decisionsTotal"], F["decisionsStatusSuperseded"]
edges, edges_dec = F["supersedeEdges"], F["supersedeEdgesToDecisions"]
acc_edge, sup_noedge, disagree = F["acceptedWithSupersedeEdge"], F["supersededWithoutEdge"], F["supersessionDisagreements"]
fires, med, p90, p99, mx, over = (F["userPromptFires"], F["userPromptMedian"], F["userPromptP90"],
                                  F["userPromptP99"], F["userPromptMax"], F["userPromptOverCap"])
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready),
                ("total", total), ("sup_status", sup_status), ("acc_edge", acc_edge), ("sup_noedge", sup_noedge),
                ("fires", fires), ("med", med), ("p99", p99), ("mx", mx), ("over", over)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")
over_pct = f"{over * 100 / fires:.1f}"

fig_census = bars(
    f"{disagree} records show the two marks apart.",
    [("edges reverse a clause, record stands", acc_edge, "warn", "accepted + edge"),
     ("retired by status, no edge", sup_noedge, "warn", ""),
     ("retired by status", sup_status, "muted", f"of {total}"),
     ("edges to a decision", edges_dec, "muted", f"of {edges}")],
    unit="")

fig_sup_lands = downstream(
    "Each answer lands on a different bill.",
    ("the choice", ""),
    [("A edge", "", "schema + migration, a day", "warn"),
     ("B status", "", "one guard, two hours", "accent"),
     ("C both", "", "seven records amended", "warn"),
     ("D as is", "", "the defect kept", "bad")])

fig_cap = logic_lanes(
    "The cap is checked after the walk, so a drop saves nothing.",
    ("today", [("the walk runs", f"median {med} ms", "muted", ""),
               ("then the cap is read", f"{over} of {fires} were over", "bad", "gap"),
               ("the facts", "dropped, time already spent", "bad", "")],
     ["", ""]),
    ("after A", [("the walk runs", f"median {med} ms", "muted", ""),
                 ("the cap is read", "slow is counted", "accent", ""),
                 ("the facts", "pushed, latency named", "ok", "")],
     ["", ""]))

fig_tail = bars(
    "The tail is machine load, not the corpus.",
    [("median", med, "muted", "ms"),
     ("p90", p90, "muted", "ms"),
     ("p99", p99, "warn", "ms"),
     ("max", mx, "bad", "ms")],
    unit="")

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Two asks wait: which mark retires a decision, and what happens to late memory</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">Two asks</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Two asks.</strong> One: a retired decision has two marks - a status field and an edge from its reverser - and either is written alone. I recommend <strong>B</strong>: status rules, the gate refuses a reversal of a proposed target. Two: recalled facts that arrive after the 2.5 s cap are thrown away after the time was spent. I recommend <strong>A</strong>: push them late, count them slow.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Marks disagree</b> {disagree}</span><span class="chip warn"><b>Turns that paid and got nothing</b> {over}</span></div>

<h2>Ask one &mdash; which mark says retired</h2>
{fig_census}
<p>{acc_edge} accepted decisions carry a reversal edge and stand - the edge undid a clause, not the record. {sup_noedge} retired decisions have no edge. <strong>Wrong if</strong> you want &ldquo;in force?&rdquo; answered from the graph alone - then A, which costs the edge a scope.</p>
{fig_sup_lands}
<div class="opts" data-records="d0384"><label><input type="radio" name="ask-sup" value="A: the edge is authoritative; status leaves the vocabulary; the edge gains a scope">A: edge rules, schema change</label><label><input type="radio" name="ask-sup" value="B: the status field is authoritative; a guard refuses a reversal of a proposed target (recommended)">B: status rules, one guard (recommended)</label><label><input type="radio" name="ask-sup" value="C: both marks held equal by a guard; clause reversals say so">C: both, held equal</label><label><input type="radio" name="ask-sup" value="D: as it stands; the queue alone hides a reversed proposal">D: as it stands</label></div>

<h2>Ask two &mdash; late memory</h2>
{fig_cap}
<p>The recall hook reads its cap <em>after</em> the walk. {over} of {fires} turns ({over_pct}%) paid and received nothing. Median {med} ms, max {mx} ms - the tail is the machine building, not the corpus. <strong>Wrong if</strong> a late 4,000-character payload on a busy turn is worse to you than none.</p>
{fig_tail}
<div class="opts" data-records="d0390"><label><input type="radio" name="ask-cap" value="A: push the facts late with the latency named, count the fire as recall-slow (recommended)">A: push late, count slow (recommended)</label><label><input type="radio" name="ask-cap" value="B: raise the cap to 10,000 ms and keep dropping past it">B: raise the cap to 10 s</label><label><input type="radio" name="ask-cap" value="C: as it stands - keep dropping past 2,500 ms, now counted">C: as it stands, counted</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {ready} ready items. {human} on your word, {auto} auto-accepted since 2026-09-05. {pending} wait. Numbers come from the facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
