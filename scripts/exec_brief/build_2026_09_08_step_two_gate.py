#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-08 (seventh) queue change, terse register under
D0377's 450-word cap: one ask - the STPA step-2 gate is a computed row set the self-run reads, not a
sentence it asserts (D0383, process-change: the SOP and the stpa-self skill now name it).

Usage: python scripts/exec_brief/build_2026_09_08_step_two_gate.py <facts.json> <previous.html> <out.html>
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
clauses, holds = F["stepTwoClauses"], F["stepTwoHolds"]
ctl, absent = F["controllersPresent"], F["rolesAbsent"]
actuators, oio, resp = F["actuatorsComputed"], F["otherInputsOutputs"], F["responsibilitiesComputed"]
unanalysed = F["stpaActionsUnanalysed"]
tree, gen = J["tree"], J["generatedAt"][:10]
for name, v in [("human", human), ("auto", auto), ("tests", tests), ("guards", guards), ("ready", ready),
                ("clauses", clauses), ("holds", holds), ("ctl", ctl), ("actuators", actuators),
                ("oio", oio), ("resp", resp), ("unanalysed", unanalysed)]:
    if v is None:
        sys.exit(f"refusing: fact {name} is null")
if tests == 0:
    sys.exit("refusing: suiteTests is 0 - the receipt was read mid-run; rebuild facts after the suite")

DECISION_TEXT = ("In a keel self-run, STPA step 2 is complete when every stepTwoGate row of keel show "
                 "control-structure holds; the stpa-self skill reads those rows and does not re-argue them, "
                 "and when a row does not hold the run's first record is that clause.")

fig_lanes = logic_lanes(
    "The gate moves from a paragraph the run argues to rows the run reads.",
    ("today", [("SOP gate", "a paragraph", "muted", ""),
               ("the run asserts", "step 2 complete", "bad", "gap"),
               ("a testimony", "", "bad", "")],
     ["", ""]),
    ("after", [("SOP gate", f"{clauses} rows", "accent", ""),
               ("the run reads", f"{holds} of {clauses} hold", "accent", ""),
               ("a receipt", "re-runnable", "ok", "")],
     ["", ""]))

fig_lands = downstream(
    "The text change lands on two process files.",
    ("the gate rule", ""),
    [("stpa SOP", "", "step-2 gate names the rows", "accent"),
     ("stpa-self skill", "", "reads, never re-argues", "accent"),
     ("the view", "", "already ships", "ok")])

fig_gate = bars(
    "What the rows say on this project today.",
    [("clauses hold", holds, "ok", f"of {clauses}"),
     ("controllers wired", ctl, "muted", f"{absent} absent"),
     ("actuators derived", actuators, "muted", ""),
     ("other inputs/outputs", oio, "muted", "authored"),
     ("actions unanalysed", unanalysed, "warn", "next STPA run")],
    unit="")

page = f'''<title>Keel Decision Brief &middot; williamweatherholtz/sysmlv2-ai-toolkit</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Condensed:wght@400;500&family=Roboto+Mono:wght@400&display=swap">
{style}
<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">One ask waits: the STPA step-2 gate is read, not argued</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One ask</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>One ask.</strong> The STPA SOP and the stpa-self skill now say the step-2 gate is the {clauses} computed rows, and a run reads them instead of arguing completeness. The code shipped; only the process text needs your word. I recommend accepting. <strong>Wrong if</strong> a future SOP clause is added in prose and not in the code - the two gates then diverge and nothing detects it.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Signed by you since the rule</b> {human}</span><span class="chip"><b>Auto-accepted since the rule</b> {auto}</span></div>

<h2>The decision, quoted</h2>
<blockquote><em>{escape(DECISION_TEXT)}</em></blockquote>

<h2>The ask &mdash; what the change binds</h2>
{fig_lanes}
<p>Accepting binds the next self-run to the rows: a clause that does not hold is the run's first record. Rejecting keeps the prose gate, and the rows stay a view nobody is bound to read. The alternative I did not take: a guard that fails when the SOP's clause count and the code's differ - a text-count rule, brittle, so it stays a stated residual.</p>
{fig_lands}
{fig_gate}
<div class="opts" data-records="d0383"><label><input type="radio" name="ask-gate" value="Accept the gate rule: the step-2 gate is the computed stepTwoGate rows; the self-run reads them">Accept: the run reads the rows</label><label><input type="radio" name="ask-gate" value="Reverse the gate rule: keep the prose step-2 gate in the SOP and skill">Reverse: keep the prose gate</label><label><input type="radio" name="ask-gate" value="Discuss the gate rule">Discuss</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} checks, {viol} violations. {ready} ready items. {human} on your word, {auto} auto-accepted since 2026-09-05. {pending} waits. Numbers come from the facts file. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
{script}
'''
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, {page.count('<figure')} figures")
