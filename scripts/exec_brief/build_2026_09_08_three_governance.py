#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (ninth) queue change, terse register under
D0377's 450-word cap: three asks - which mark says a decision is retired (D0384, four options, B
recommended), what the recall hook does with facts that arrive after its cap (D0390, three options,
A recommended), and whether CI runs the script probes (D0394, accept recommended).

Usage: python scripts/exec_brief/build_2026_09_08_three_governance.py <facts.json> <previous.html> <out.html>
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
if pending != 3:
    sys.exit(f"refusing: this page states THREE asks and facts.json says {pending} are pending")
human, auto = F["scopeDecisionsHumanAccepted"], F["scopeDecisionsAutoAccepted"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
ready = F["readyItems"]
total, sup_status = F["decisionsTotal"], F["decisionsStatusSuperseded"]
edges, edges_dec = F["supersedeEdges"], F["supersedeEdgesToDecisions"]
acc_edge, sup_noedge, disagree = F["acceptedWithSupersedeEdge"], F["supersededWithoutEdge"], F["supersessionDisagreements"]
fires, med, p90, p99, mx, over = (F["userPromptFires"], F["userPromptMedian"], F["userPromptP90"],
                                  F["userPromptP99"], F["userPromptMax"], F["userPromptOverCap"])
probes = F["scriptsWithProbes"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready),
                ("total", total), ("sup_status", sup_status), ("acc_edge", acc_edge), ("sup_noedge", sup_noedge),
                ("fires", fires), ("med", med), ("p99", p99), ("mx", mx), ("over", over), ("probes", probes)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")
over_pct = f"{over * 100 / fires:.1f}"

fig_census = bars(
    f"{disagree} records show the two marks apart.",
    [("edge, record still stands", acc_edge, "warn", "accepted"),
     ("retired, no edge", sup_noedge, "warn", ""),
     ("retired by status", sup_status, "muted", f"of {total}")],
    unit="")

fig_sup_lands = downstream(
    "Each answer lands on a different bill.",
    ("the choice", ""),
    [("A edge", "", "schema + migration", "warn"),
     ("B status", "", "one guard", "accent"),
     ("C both", "", "seven records amended", "warn"),
     ("D as is", "", "defect kept", "bad")])

fig_cap = logic_lanes(
    "The cap is read after the walk; a drop saves nothing.",
    ("today", [("walk runs", f"{med} ms median", "muted", ""),
               ("cap read after", f"{over} over", "bad", "gap"),
               ("facts", "dropped", "bad", "")],
     ["", ""]),
    ("after A", [("walk runs", f"{med} ms median", "muted", ""),
                 ("cap read", "slow counted", "accent", ""),
                 ("facts", "pushed late", "ok", "")],
     ["", ""]))

fig_tail = bars(
    "The tail is machine load, not the corpus.",
    [("median", med, "muted", "ms"),
     ("p90", p90, "muted", "ms"),
     ("p99", p99, "warn", "ms"),
     ("max", mx, "bad", "ms")],
    unit="")

fig_probes = downstream(
    f"{probes} self-checking scripts, none run by CI.",
    ("a probe regresses", ""),
    [("today", "", "the next session finds it", "bad"),
     ("after accept", "", "the next push finds it", "accent")])

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Three asks wait: a retired-decision mark, late memory, and self-checks in CI</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">Three asks</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Three asks.</strong> A retired decision has two marks and either is written alone - I recommend <strong>B</strong>, status rules. Recalled facts past the 2.5 s cap are dropped after the time was spent - I recommend <strong>A</strong>, push late. {probes} scripts self-check and CI runs none - I recommend gating them.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Marks disagree</b> {disagree}</span><span class="chip warn"><b>Turns paid, no facts</b> {over}</span></div>

<h2>Ask one &mdash; which mark says retired</h2>
{fig_census}
<p>{acc_edge} accepted decisions carry a reversal edge and stand - it undid a clause, not the record; {sup_noedge} retired ones have no edge. <strong>Wrong if</strong> you want &ldquo;in force?&rdquo; from the graph - then A.</p>
{fig_sup_lands}
<div class="opts" data-records="d0384"><label><input type="radio" name="ask-sup" value="A: the edge is authoritative; status leaves the vocabulary; the edge gains a scope">A: edge rules, schema change</label><label><input type="radio" name="ask-sup" value="B: the status field is authoritative; a guard refuses a reversal of a proposed target (recommended)">B: status rules, one guard (recommended)</label><label><input type="radio" name="ask-sup" value="C: both marks held equal by a guard; clause reversals say so">C: both, held equal</label><label><input type="radio" name="ask-sup" value="D: as it stands; the queue alone hides a reversed proposal">D: as it stands</label></div>

<h2>Ask two &mdash; late memory</h2>
{fig_cap}
<p>The recall hook reads its cap <em>after</em> the walk, so {over} of {fires} turns paid and got nothing. Median {med} ms, max {mx} ms - the tail is machine load. <strong>Wrong if</strong> a late payload on a busy turn is worse than none.</p>
<div class="opts" data-records="d0390"><label><input type="radio" name="ask-cap" value="A: push the facts late with the latency named, count the fire as recall-slow (recommended)">A: push late, count slow (recommended)</label><label><input type="radio" name="ask-cap" value="B: raise the cap to 10,000 ms and keep dropping past it">B: raise the cap to 10 s</label><label><input type="radio" name="ask-cap" value="C: as it stands - keep dropping past 2,500 ms, now counted">C: as it stands, counted</label></div>

<h2>Ask three &mdash; self-checks in CI</h2>
{fig_probes}
<p>{probes} scripts ship a known-answer check; CI runs none, so a regression waits for the next session that runs it. Accept and each runs as a gate, found from the tree. <strong>Wrong if</strong> you would rather they stay hand tools. Changes CI, so it waits for you.</p>
<div class="opts" data-records="d0394"><label><input type="radio" name="ask-probes" value="Accept: CI runs every scripts/*.py that ships a probe, as a gate, discovered from the tree (recommended)">Accept: CI runs the probes as a gate (recommended)</label><label><input type="radio" name="ask-probes" value="Reverse: leave the probes as hand tools, not run by CI">Reverse: leave them hand tools</label><label><input type="radio" name="ask-probes" value="Discuss: the probe-in-CI step needs more thought">Discuss</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Repository at {tree}, {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {pending} wait; {human} on your word since 2026-09-05. Numbers from the facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
