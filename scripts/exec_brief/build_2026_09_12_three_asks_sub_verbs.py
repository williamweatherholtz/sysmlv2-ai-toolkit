#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py) or a count over that page's own members tables; it renders, it does not
# measure. Style, copy machinery and the tab strip are the previously published shell (D0404).
"""Build the standing decision brief for the 2026-09-12 (seventeenth) queue change: the two safety folds of
the sixteenth publish still wait, and a third ask joined them - the control-structure rule that a routed
write command counts one control action per sub-verb, read from the command's own declared invocation
(D0454). It carries the process-change marker because the shared CLI-fact reader sits on the enforcement
surface; the rule is on trunk with sprint 681 and the human's word governs it. None of the three is a fork:
each is accepted, held or rejected with a word. One tab per ask (D0404); members named (D0406); every count
is a facts.py fact (sections 18 and 19), never typed.

Usage: python scripts/exec_brief/build_2026_09_12_three_asks_sub_verbs.py <facts.json> <previous.html> <out.html>
"""
import re
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
from logic_exhibits import downstream, logic_lanes     # noqa: E402
sys.path.insert(0, "scripts")
from artefact import claim, require_complete   # noqa: E402  (D0387: a stale answer is refused, a dead run leaves no page)
from check_templates import field_text, markup_only  # noqa: E402
from fit_check import assert_fits              # noqa: E402  (D0402: a page whose exhibits hide text is removed, not published)

facts_path, prev_path, out_path = sys.argv[1:4]
if prev_path == out_path:
    sys.exit("refusing: the style source and the output are the same file - copy the previous page aside first")
J = require_complete(facts_path)
claim(out_path)
TREE = J["tree"]
DATE = J["generatedAt"][:10]
prev = open(prev_path, encoding="utf-8").read()
prev = prev[prev.index("<title>"):]                      # the host's own wrapper line, if the copy carries one, is not ours
head = prev[: prev.index('<div class="page">')]
tail = prev[prev.index("<script>"):]
tail = tail[: tail.index("</script>") + len("</script>")] + "\n"   # the host wraps the page in its own body; ours ends with the script


def v(name):
    x = J["facts"][name]["value"]
    if x is None:
        sys.exit(f"refusing: fact {name} is null - {J['facts'][name]['how']}")
    return x


# ---- the queue: the two safety folds and the sub-verb rule, none a fork ---------------------------------
pending = v("pendingAcceptances")
members = v("pendingMembers")
if len(members) != pending or v("pendingForks") != 0:
    sys.exit(f"refusing: facts disagree - {len(members)} members, {pending} pending, {v('pendingForks')} forks")
if [p["slug"] for p in members] != ["d0452", "d0453", "d0454"]:
    sys.exit(f"refusing: this page is written for the two folds and the sub-verb rule; the queue is {[p['slug'] for p in members]}")

# ---- the two families: members from the facts, the mapping from each Decision's text ----------------
G, C = v("familyCensusGating"), v("familyCensusChannel")
# D0452's text: the twelve gating facts, all reads and stable, and where each lands
GATE_SUB = ["validate", "check", "check-engine", "guard", "rules", "assured", "adoption-check"]
AUDIT_SUB = ["audit-history", "audit-adherence", "audit-ci-runs"]
ROUTERS = ["gate", "audit"]
TWELVE = sorted(GATE_SUB + AUDIT_SUB + ROUTERS)
g_reads = sorted(m["name"] for m in G["memberFacts"] if m["effect"] == "reads" and m["stability"] == "stable")
if g_reads != TWELVE:
    sys.exit(f"refusing: the Decision names twelve read-only stable gating verbs; the facts hold {g_reads}")
G_MOVE = GATE_SUB + AUDIT_SUB                     # the ten names removed from the dispatch
# D0453's text: the five github verbs move, currency keeps its name
C_MOVE = sorted(m["name"] for m in C["memberFacts"] if m["name"].startswith("github-"))
C_STAY = sorted(m["name"] for m in C["memberFacts"] if not m["name"].startswith("github-"))
if len(C_MOVE) != 5 or C_STAY != ["currency"]:
    sys.exit(f"refusing: the Decision moves five github verbs and keeps currency; the facts hold {C_MOVE} / {C_STAY}")
C_DEPRECATED = sorted(m["name"] for m in C["memberFacts"] if m["name"] in C_MOVE and m["stability"] == "deprecated")

NOT_REWRITTEN = ("decisions", "tracking")


def sites(census, names, *areas):
    return sum(census["sites"][a][n] for a in areas for n in names)


def files_naming(census, name):
    return sum(census["files"][a][name] for a in census["files"] if a not in NOT_REWRITTEN)


AREAS_REWRITTEN = [a for a in G["sites"] if a not in NOT_REWRITTEN]
g_hooks = sites(G, G_MOVE, "hooksAndCi")
g_engine = sites(G, G_MOVE, "engine", "claude")
g_rust = sites(G, G_MOVE, "rustSource", "rustTests")
g_docs = sites(G, G_MOVE, "rootDocs", "other")
g_rewritten = sites(G, G_MOVE, *AREAS_REWRITTEN)
g_history = sites(G, G_MOVE, *NOT_REWRITTEN)
assert g_rewritten == g_hooks + g_engine + g_rust + g_docs
c_hooks = sites(C, C_MOVE, "hooksAndCi")
c_engine = sites(C, C_MOVE, "engine", "claude")
c_rust = sites(C, C_MOVE, "rustSource", "rustTests")
c_docs = sites(C, C_MOVE, "rootDocs", "other")
c_rewritten = sites(C, C_MOVE, *AREAS_REWRITTEN)
c_history = sites(C, C_MOVE, *NOT_REWRITTEN)
c_currency_ci = sites(C, C_STAY, "hooksAndCi")
assert c_rewritten == c_hooks + c_engine + c_rust + c_docs

arms = v("cliDispatchArms")
arms_after = arms - len(G_MOVE) - len(C_MOVE) + 1      # ten names go, five go and one router arrives
tests, failing = v("suiteTests"), v("suiteFailed")
dirty = v("treeUncommitted")["modified"]

# ---- the sub-verb rule: what the structure lists now, what it listed when a router was one action ------
SV, SA, UC = v("subVerbActions"), v("subVerbActionsAnalysed"), v("ucaCensus")
unanalysed = v("stpaActionsUnanalysed")
if SV["bareRoutersPresent"]:
    sys.exit(f"refusing: the rule says no router stays an action; the structure lists {SV['bareRoutersPresent']}")
routers = SV["routers"]                                 # {"record": 13, "process": 8, "library": 3} on this tree
sv_total, sv_before, sv_after = SV["subVerbActions"], SV["actionsBeforeRule"], SV["actionsTotal"]
sv_added = sv_after - sv_before
analysed_by_router = {r: sum(1 for n in SA["analysed"] if n.lower().startswith("cmd" + r.replace("-", ""))) for r in routers}
open_by_router = {r: routers[r] - analysed_by_router[r] for r in routers}
assert sum(open_by_router.values()) == len(SA["open"])
if len(SA["analysed"]) + len(SA["open"]) != sv_total:
    sys.exit("refusing: the analysed and open sub-verb sets do not partition the sub-verb actions")


def m(names):
    return " ".join(f'<span class="m">{n}</span>' for n in names)


gating_members = (
    f'<div class="tbl-wrap"><table data-members="{len(TWELVE)}"><thead><tr><th>becomes</th><th>verb</th></tr></thead><tbody>'
    f'<tr><td>gate ({len(GATE_SUB)})</td><td>{m(GATE_SUB)}</td></tr>'
    f'<tr><td>audit ({len(AUDIT_SUB)})</td><td>{m(AUDIT_SUB)}</td></tr>'
    f'<tr><td>routers ({len(ROUTERS)})</td><td>{m(ROUTERS)}</td></tr>'
    f'</tbody></table></div>')
channel_members = (
    f'<div class="tbl-wrap"><table data-members="{len(C_MOVE) + len(C_STAY)}"><thead><tr><th>becomes</th><th>verb</th></tr></thead><tbody>'
    f'<tr><td>github ({len(C_MOVE)})</td><td>{m(C_MOVE)}</td></tr>'
    f'<tr><td>stays ({len(C_STAY)})</td><td>{m(C_STAY)}</td></tr>'
    f'</tbody></table></div>')
subverb_members = (
    f'<div class="tbl-wrap"><table data-members="{len(routers)}"><thead><tr><th>router</th><th>actions</th><th>analysed</th><th>open, named</th></tr></thead><tbody>'
    + "".join(f'<tr><td><span class="m">keel {r}</span></td><td>{routers[r]}</td><td>{analysed_by_router[r]}</td><td>{open_by_router[r]}</td></tr>'
              for r in sorted(routers, key=lambda k: -routers[k]))
    + f'<tr><td>total</td><td>{sv_total}</td><td>{len(SA["analysed"])}</td><td>{len(SA["open"])}</td></tr>'
    f'</tbody></table></div>')


def courses(rows):
    body = "".join(f"<tr><td>{a}</td><td>{b}</td><td>{c}</td></tr>" for a, b, c in rows)
    return (f'<div class="tbl-wrap"><table><thead><tr><th>course</th><th>what changes</th><th>what it leaves</th></tr></thead>'
            f'<tbody>{body}</tbody></table></div>')


# ---- exhibits: two per ask ----------------------------------------------------------------------------
fig1 = logic_lanes(
    "The word moves one slot right; the arm and its exit code do not move",
    ("today", [
        ("hook runs keel validate", "", "accent", ""),
        ("the arm answers", "exit code gates", "ok", ""),
    ], ["", ""]),
    ("changed", [
        ("hook runs keel gate validate", "", "accent", ""),
        ("the same arm answers", "same exit code", "ok", ""),
    ], ["", ""]),
)
fig2 = downstream(
    f"Removing ten names lands on {g_rewritten} call sites in one commit; history is left alone",
    ("remove ten names", "", ""),
    [
        ("hooks and CI", "", f"{g_hooks}", "bad"),
        ("engine and skills", "", f"{g_engine}", "warn"),
        ("Rust source and tests", "", f"{g_rust}", "warn"),
        ("docs and scripts", "", f"{g_docs}", "warn"),
        ("decisions and history", "not rewritten", f"{g_history:,}", "muted"),
    ],
)
fig3 = logic_lanes(
    "The trust boundary moves with the arm, or the bytes say it did not",
    ("today", [
        ("keel github-pull, public repo", "", "accent", ""),
        ("untrusted: plan only", "fails closed if undetermined", "ok", ""),
    ], ["", ""]),
    ("changed", [
        ("keel github pull, public repo", "", "accent", ""),
        ("same bytes: plan only", "same fail-closed case", "ok", ""),
    ], ["", ""]),
)
fig4 = downstream(
    f"Removing five names lands on {c_rewritten} call sites; no CI line changes",
    ("remove five names", "", ""),
    [
        ("engine and skills", "", f"{c_engine}", "warn"),
        ("Rust source and tests", "fixtures compared", f"{c_rust}", "warn"),
        ("hooks and CI", "currency only; it stays", f"{c_hooks}", "ok"),
        ("decisions and history", "not rewritten", f"{c_history}", "muted"),
    ],
)
fig5 = logic_lanes(
    "A fold used to hide write paths from the analysis; under the rule it adds them",
    ("before the rule", [
        ("nine verbs fold under keel record", "", "accent", ""),
        ("structure: one action", "seven paths vanish", "bad", ""),
    ], ["", ""]),
    ("with the rule", [
        ("the same fold", "", "accent", ""),
        (f"structure: {routers.get('record', 0)} actions", "each analysed or named open", "ok", ""),
    ], ["", ""]),
)
fig6 = downstream(
    f"The rule adds {sv_added} actions; {len(SA['analysed'])} analysed, {len(SA['open'])} open and named",
    ("one action per sub-verb", "", ""),
    [(f"keel {r}", "analysed" if open_by_router[r] == 0 else "open, named by the guard",
      f"{routers[r]}", "ok" if open_by_router[r] == 0 else "warn") for r in sorted(routers, key=lambda k: -routers[k])]
    + [("the two folds you hold", "land on this rule", f"{len(G_MOVE) + len(C_MOVE)} verbs", "muted")],
)


def words(fragment):
    return len(field_text(re.sub(r"<[^>]+>", " ", markup_only(fragment))).split())


ASKS = [
    ("d0452", "ask-gating", "Gating", "gating"),
    ("d0453", "ask-github", "Github", "github"),
    ("d0454", "ask-subverbs", "Sub-verbs", "subverbs"),
]
panel_bodies = {
    "gating": f"""<h2>Gating into gate and audit</h2>
<p>The item: <q>the twelve gating verbs become sub-verbs of keel gate and keel audit, each keeping its word; the hooks, CI workflows and every call site are rewritten in the same commit that removes the names.</q></p>
<p><strong>Accepting binds:</strong> ten names leave the dispatch; {g_rewritten} call sites rewritten in that commit, {g_hooks} in hooks and CI, which the commit passes rewritten. Criterion: stdout, stderr, exit code byte-equal to the pre-fold binary over a green tree, a red tree, a workspace root.</p>
{gating_members}
{fig1}
{fig2}
{courses([
    ("Accept", f"ten names go; dispatch reads {arms - len(G_MOVE)}", "downstream hooks naming a gating verb break until updated"),
    ("Hold or reject", "nothing moves", "the programme lands four families of five"),
])}
<p><strong>True in the model:</strong> twelve facts, all reads; counts computed; validate sits in {files_naming(G, "validate")} live files, guard in {files_naming(G, "guard")}. <strong>My assumption:</strong> three trees show every exit code moved with its arm. <strong>Wrong if</strong> a hook or runner must keep a bare name, or you want validate, guard and gate taught as separate words.</p>
<p><strong>Accept:</strong> transform, dry run reconciling these counts, rewrites, three-tree comparison, one commit. <strong>Hold:</strong> nothing moves; the covered folds proceed. <strong>Reject:</strong> recorded quoting you; the names stand.</p>
<div class="opts" data-records="d0452"><label><input type="radio" name="ask-gating" value="Accept: the twelve gating verbs become sub-verbs of keel gate and keel audit, each keeping its word; hooks, CI and every call site rewritten in the commit that removes the names (recommended)">Accept (recommended)</label><label><input type="radio" name="ask-gating" value="Hold: the gating fold stays proposed; no gating verb moves; the covered folds proceed">Hold</label><label><input type="radio" name="ask-gating" value="Reject: the gating verbs keep their names; the fold is recorded rejected">Reject</label></div>""",
    "github": f"""<h2>The github verbs into github</h2>
<p>The item: <q>the five github verbs become sub-verbs of keel github; currency keeps its name; the CI workflow and every call site are rewritten in the same commit that removes the names.</q></p>
<p><strong>Accepting binds:</strong> five names leave the dispatch, one router arrives; {c_rewritten} call sites rewritten in that commit. CI names only currency: {c_hooks} workflow lines change. Criterion: byte equality with the pre-fold binary over the Rust fixtures, the fail-closed undetermined-trust case included.</p>
{channel_members}
{fig3}
{fig4}
{courses([
    ("Accept", f"five names go, one arrives; dispatch reads {arms - len(C_MOVE) + 1}", "downstream skills naming a github verb update with it"),
    ("Hold or reject", "nothing moves", "five names stay a router spelled with hyphens"),
])}
<p><strong>True in the model:</strong> six channel facts; {len(C_DEPRECATED)} of the five moving names ({", ".join(C_DEPRECATED)}) already carry the deprecated mark; only currency is named in CI. <strong>My assumption:</strong> a deprecated verb is worth a router slot rather than removal. <strong>Wrong if</strong> you would rather retire the {len(C_DEPRECATED)} deprecated names than fold them, or an external caller types github-pull.</p>
<p><strong>Accept:</strong> transform, dry run, fixture comparison, one commit; the intake skill and the working rules name the new spellings. <strong>Hold:</strong> nothing moves. <strong>Reject:</strong> recorded quoting you; the names stand.</p>
<div class="opts" data-records="d0453"><label><input type="radio" name="ask-github" value="Accept: the five github verbs become sub-verbs of keel github, currency keeps its name; the CI workflow and every call site rewritten in the commit that removes the names (recommended)">Accept (recommended)</label><label><input type="radio" name="ask-github" value="Hold: the channel fold stays proposed; no channel verb moves">Hold</label><label><input type="radio" name="ask-github" value="Reject: the github verbs keep their names; the fold is recorded rejected">Reject</label></div>""",
    "subverbs": f"""<h2>One control action per sub-verb</h2>
<p>The item: <q>a routed write command yields one computed control action per sub-verb, read from its own declared invocation, never from a hand list; a sub-verb that writes nothing still appears until it declares its own effect.</q></p>
<p><strong>Accepting binds:</strong> the safety analysis works at the sub-verb, so a fold adds actions instead of hiding them. The structure lists {sv_after} actions where it listed {sv_before}; every open one is named by the guard. The rule is on trunk; your word governs it.</p>
{subverb_members}
{fig5}
{fig6}
{courses([
    ("Accept", f"{sv_added} actions stay; the folds you hold add theirs", f"{len(SA['open'])} named actions wait for their analysis run"),
    ("Hold", "the rule stays proposed; the code stands", "the two folds wait on it"),
    ("Reject", f"the derivation is reverted; {sv_added} actions collapse to {len(routers)}", "the analysis already written stands as history"),
])}
<p><strong>True in the model:</strong> {sv_total} sub-verb actions from {len(routers)} routers; {len(SA['analysed'])} analysed by a recorded run, {len(SA['open'])} open; {unanalysed} of {sv_after} actions unanalysed overall; {UC['total']} unsafe actions recorded, {UC['observed']} from an observed incident. <strong>My assumption:</strong> a sub-verb is the grain a hazard is written at. <strong>Wrong if</strong> you want each sub-verb declared as its own command fact, or the router kept as the unit.</p>
<p><strong>Accept:</strong> nothing more moves; the folds proceed on this rule. <strong>Hold:</strong> the folds wait. <strong>Reject:</strong> revert in one commit, recorded quoting you.</p>
<div class="opts" data-records="d0454"><label><input type="radio" name="ask-subverbs" value="Accept: a routed write command yields one computed control action per sub-verb, read from its declared invocation; a sub-verb that writes nothing appears until it declares its own effect (recommended)">Accept (recommended)</label><label><input type="radio" name="ask-subverbs" value="Hold: the sub-verb rule stays proposed; the code stands; the two folds wait">Hold</label><label><input type="radio" name="ask-subverbs" value="Reject: the per-sub-verb derivation is reverted; a routed command is one control action">Reject</label></div>""",
}


def frame(panels_html, tabs_html, title, sub):
    return f"""<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">{title}</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">{sub}</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Three asks.</strong> <strong>Gating:</strong> ten names go behind gate and audit, hooks and CI in the same commit. <strong>Github:</strong> five names go behind github; currency stays. <strong>Sub-verbs:</strong> a routed write command counts one control action per sub-verb, so a fold adds actions rather than hiding them; the folds land on this rule. <strong>Accept all three</strong>, or hold any. <strong>Wrong if</strong> a hook must keep a bare name.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip"><b>Dispatch arms</b> {arms} &rarr; {arms_after}</span><span class="chip"><b>Control actions</b> {sv_before} &rarr; {sv_after}</span></div>

<div class="tabs" role="tablist" aria-label="The asks">{tabs_html}</div>
{panels_html}
<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on {DATE}; {dirty} files carried uncommitted edits. Call sites: grep for keel plus a family verb over tracked files, by area; control actions from the computed structure. {tests} tests, {failing} failing. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
"""


TITLE = "Accept three: both safety folds, and one control action per sub-verb"
SUB = "Three asks"


def render(panel_bodies):
    tabs = "".join(
        f'<button role="tab" aria-selected="{"true" if i == 0 else "false"}" aria-controls="panel-{key}" id="tab-{key}" type="button">{label}</button>'
        for i, (_d, _n, label, key) in enumerate(ASKS))
    panels = "".join(
        f'<section role="tabpanel" id="panel-{key}" aria-labelledby="tab-{key}"{"" if i == 0 else " hidden"}>{panel_bodies[key]}</section>'
        for i, (_d, _n, _label, key) in enumerate(ASKS))
    return frame(panels, tabs, TITLE, SUB)


body = render(panel_bodies)
panel_words = {k: words(p) for k, p in panel_bodies.items()}
frame_words = words(body) - sum(panel_words.values())

# ---- the shell's tab strip: CSS into the head, wiring into the tail (added once; the source page may lack it) ----
TAB_CSS = '''
.tabs{display:flex;gap:4px;flex-wrap:wrap;border-bottom:1.5px solid var(--line);margin:22px 0 6px}
.tabs [role="tab"]{background:none;border:0;border-bottom:3px solid transparent;margin-bottom:-1.5px;padding:10px 12px;cursor:pointer;color:var(--muted);font:500 12px/1.3 "Roboto Condensed",sans-serif;letter-spacing:.12em;text-transform:uppercase;min-height:44px}
.tabs [role="tab"][aria-selected="true"]{color:var(--head);border-bottom-color:var(--accent)}
.tabs [role="tab"]:hover{color:var(--head)}
.tabs [role="tab"]:focus-visible{outline:2px solid var(--accent);outline-offset:-2px}
[role="tabpanel"]{display:block}
[role="tabpanel"][hidden]{display:none}
.m{display:inline-block;font:400 12.5px/1.2 "Roboto Mono",Consolas,monospace;border:1px solid var(--line);border-radius:3px;padding:1px 5px;margin:1px 2px}
'''
if ".tabs{" not in head:
    assert head.count("</style>") == 1
    head = head.replace("</style>", TAB_CSS + "</style>")
TAB_JS = '''
(function(){"use strict";
var tabs=Array.prototype.slice.call(document.querySelectorAll('[role="tab"]'));
function show(tab){tabs.forEach(function(t){var on=t===tab;t.setAttribute('aria-selected',on?'true':'false');t.tabIndex=on?0:-1;
  var p=document.getElementById(t.getAttribute('aria-controls'));if(p)p.hidden=!on});tab.focus();
  try{localStorage.setItem('keel-brief-tab:'+document.title,tab.id)}catch(e){}}
tabs.forEach(function(t,i){t.addEventListener('click',function(){show(t)});
  t.addEventListener('keydown',function(e){var j=e.key==='ArrowRight'?i+1:e.key==='ArrowLeft'?i-1:e.key==='Home'?0:e.key==='End'?tabs.length-1:null;
    if(j===null)return;e.preventDefault();show(tabs[(j+tabs.length)%tabs.length])})});
try{var k=localStorage.getItem('keel-brief-tab:'+document.title);var t=k&&document.getElementById(k);if(t)show(t)}catch(e){}
})();
</script>'''
if "keel-brief-tab:" not in tail:
    assert tail.count("</script>") == 1
    tail = tail.replace("</script>", TAB_JS)
DIGEST_OLD = ".page > h2, .page > h3, .page > p, .page > ul, .page > blockquote, .page > .ask, .page > .turn, .page > .tbl-wrap, .page > .opts, figure.diagram > .msg"
DIGEST_NEW = ".page > .ask, .page > p, [role=\"tabpanel\"] > h2, [role=\"tabpanel\"] > p, [role=\"tabpanel\"] > .tbl-wrap, [role=\"tabpanel\"] > .opts, figure.diagram > .msg"
if DIGEST_OLD in tail:
    tail = tail.replace(DIGEST_OLD, DIGEST_NEW)
assert DIGEST_NEW in tail, "the digest must walk every panel, hidden or not (D0404)"

page = head + body + tail
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes; frame {frame_words} words; panels " +
      ", ".join(f"{k} {n}" for k, n in sorted(panel_words.items(), key=lambda kv: -kv[1])) +
      f"; per tab " + ", ".join(f"{k} {frame_words + n}" for k, n in panel_words.items()) +
      f"; summed {words(page)}; members tables name {len(TWELVE)} gating, {len(C_MOVE) + len(C_STAY)} channel verbs and {len(routers)} routers"
      f" ({sv_total} sub-verb actions, {len(SA['analysed'])} analysed)")
assert_fits(out_path)   # probe first, then measure in a browser with and without web fonts; findings remove the page
