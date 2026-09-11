#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py) or a count over that page's own members table; it renders, it does not
# measure. Style, copy machinery and the tab strip are the previously published shell (D0404).
"""Build the standing decision brief for the 2026-09-11 (fifteenth) queue change: the queue grew from
three asks to thirty-five while the page stood. Two of them open a weighed fork (their `decision` text
begins OPTION - D0322's marker); the rest were HELD under D0337 because each changes a process, and
each names itself NOT A FORK or states one clause. The page therefore carries THREE asks (D0404, one tab
each): the verb promise fork, the recall-hit fork, and the set of ratifications judged in one sitting
with every member named (D0406) and every answer recorded per item (the d0443 shape).

Usage: python scripts/exec_brief/build_2026_09_11_two_forks_and_a_set.py <facts.json> <previous.html> <out.html>
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


def v(name):
    x = J["facts"][name]["value"]
    if x is None:
        sys.exit(f"refusing: fact {name} is null - {J['facts'][name]['how']}")
    return x


pending = v("pendingAcceptances")
members = v("pendingMembers")
forks = [p for p in members if p["fork"]]
ratif = [p for p in members if not p["fork"]]
if len(members) != pending or len(forks) != v("pendingForks"):
    sys.exit(f"refusing: facts disagree - {len(members)} members, {pending} pending, {len(forks)} forks vs {v('pendingForks')}")
if [p["slug"] for p in forks] != ["d0399", "d0408"]:
    sys.exit(f"refusing: this page is written for the verb-promise and recall-hit forks; the forks are {[p['slug'] for p in forks]}")
n_set = len(ratif)
in_tree = [p for p in ratif if p["inTree"]]
no_rec = [p for p in ratif if not p["inTree"]]
verbs, lenses = v("cliTopLevel"), v("cliShowLenses")
sites, files = v("gatingCallSites"), v("gatingCallFiles")
tests, failing = v("suiteTests"), v("suiteFailed")
guards, viol = v("guardsEnforced"), v("guardViolations")
PROMISE = 25   # the number D0273's consequences state - quoted from that text, not measured


def m(names):
    return " ".join(f'<span class="m">{n}</span>' for n in names)


members_table = (
    f'<div class="tbl-wrap"><table data-members="{n_set}"><thead><tr><th>recorded as</th><th>rule</th></tr></thead><tbody>'
    f'<tr><td>delivery record ({len(in_tree)})</td><td>{m(p["name"] for p in in_tree)}</td></tr>'
    f'<tr><td>no record ({len(no_rec)})</td><td>{m(p["name"] for p in no_rec)}</td></tr>'
    f'</tbody></table></div>')

# ---- exhibits: two per ask ----------------------------------------------------------------------------
fig1 = logic_lanes(
    "The promise was a wish, not a measure",
    ("today", [
        (f"{verbs} verbs", f"text says under {PROMISE}", "muted", ""),
        ("clause false", "", "bad", ""),
    ], ["", ""]),
    ("amended", [
        (f"{verbs} verbs, {lenses} lenses", "", "accent", ""),
        ("clause says so", "", "ok", ""),
    ], ["", ""]),
)
fig2 = downstream(
    "A collapse lands on what gates your commits",
    ("collapse the writers", "", ""),
    [
        ("hooks, CI, docs", "", f"{sites:,} calls", "warn"),
        ("files", "", f"{files}", "warn"),
    ],
)
fig3 = logic_lanes(
    "A bar with no headroom makes every slip a regression",
    ("today", [
        ("bar met", "", "accent", ""),
        ("one miss, unnamed", "", "bad", ""),
    ], ["", ""]),
    ("declared", [
        ("bar met", "", "accent", ""),
        ("sentinel printed", "", "ok", ""),
    ], ["", ""]),
)
fig4 = downstream(
    "The sentinel lands on the harness alone",
    ("name the case", "", ""),
    [
        ("the harness", "", "one print", "ok"),
        ("the criterion", "", "unedited", "muted"),
    ],
)
fig5 = logic_lanes(
    "Each rule is enforcing before your word lands",
    ("today", [
        ("control on trunk", "", "accent", ""),
        ("held proposed", "", "warn", ""),
    ], ["", ""]),
    ("one sitting", [
        ("one word", "", "accent", ""),
        ("each recorded", "quoting you", "ok", ""),
    ], ["", ""]),
)
fig6 = downstream(
    f"Accepting the set lands on {n_set} records and on no code",
    ("accept the set", "", ""),
    [
        ("records", "", f"{n_set}", "ok"),
        ("code", "", "0", "muted"),
    ],
)


def words(fragment):
    return len(field_text(re.sub(r"<[^>]+>", " ", markup_only(fragment))).split())


set_records = ", ".join(p["slug"] for p in ratif)
ASKS = [
    ("d0399", "ask-cli", "Verbs", "verbs"),
    ("d0408", "ask-recall", "Recall", "recall"),
    (set_records, "ask-set", "The set", "set"),
]
panel_bodies = {
    "verbs": f'''<h2>The verb promise</h2>
<p>Under {PROMISE} means folding writers. <strong>A</strong> folds gating, orientation, rendering, authoring and channel into <code>gate</code>, <code>audit</code>, <code>show</code>, <code>render</code>, <code>record</code>, <code>github</code>. <strong>B</strong> amends the clause to {verbs} verbs, {lenses} lenses; no code.</p>
{fig1}
{fig2}
<div class="opts" data-records="d0399"><label><input type="radio" name="ask-cli" value="A: fold the gating, orientation, rendering, authoring and channel families into gate, audit, show, render, record and github - one breaking transform each">A: fold</label><label><input type="radio" name="ask-cli" value="B: amend the clause to the measured number - {verbs} verbs, {lenses} lenses - no code (recommended)">B: amend (recommended)</label></div>''',
    "recall": f'''<h2>The recall hit</h2>
<p>A hit is the named record among shown rows. The bar is met; one case nothing reaches: the record behind the obligation fact never says <em>discharge</em>. <strong>A</strong> print it as a declared sentinel. <strong>B</strong> count a row one edge away carrying the question's rarest word; nothing changes today. <strong>C</strong> you write the alias. <strong>D</strong> leave it.</p>
{fig3}
{fig4}
<div class="opts" data-records="d0408"><label><input type="radio" name="ask-recall" value="A: a hit is the named record among the shown rows; the obligation case is printed as a declared UNREACHABLE sentinel (recommended)">A: sentinel (recommended)</label><label><input type="radio" name="ask-recall" value="B: a hit is the record, or a shown row one typed edge from it that carries the question's rarest word">B: near row</label><label><input type="radio" name="ask-recall" value="C: I author the alias obligation/discharge in the lexicon with my createdBy; the definition stays as A">C: my alias</label><label><input type="radio" name="ask-recall" value="D: leave the harness, the criterion and the issue as they stand">D: leave</label></div>''',
    "set": f'''<h2>The set of {n_set}</h2>
<p>Each changes a process, so each was held for your word; its control is on trunk, the checks green. {len(in_tree)} carry a delivery record; {len(no_rec)} do not. One word accepts all, each recorded, quoting you.</p>
{fig5}
{fig6}
{members_table}
<div class="opts" data-records="{set_records}"><label><input type="radio" name="ask-set" value="Accept the set: each of the {n_set} rules named in the table is recorded accepted on its own, quoting these words (recommended)">Accept the set (recommended)</label><label><input type="radio" name="ask-set" value="Hold: the rules stay proposed and I answer them by row, in chat">Hold</label><label><input type="radio" name="ask-set" value="Reject the set: each rule is recorded rejected and its control comes out">Reject</label></div>''',
}


def frame(panels_html, tabs_html, title, sub):
    return f'''<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">{title}</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">{sub}</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Three asks.</strong> A rule promised under {PROMISE} commands; the surface is {verbs}: <strong>B</strong>, amend. A recall bar is met with one case nothing reaches: <strong>A</strong>, declare it. {n_set} process rules already enforce on trunk: <strong>accept the set</strong> or hold by row. <strong>Wrong if</strong> you want the surface small, nearness graded, or a row kept off.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span></div>

<div class="tabs" role="tablist" aria-label="The asks">{tabs_html}</div>
{panels_html}
<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on {DATE}. {tests} tests, {failing} failing. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
'''


TITLE = f"Two forks need a pick; {n_set} rules on trunk need one word"
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
      f"; summed {words(page)}; members table names {n_set} rules ({len(in_tree)} sprint delivered, {len(no_rec)} without)")
assert_fits(out_path)   # probe first, then measure in a browser with and without web fonts; findings remove the page
