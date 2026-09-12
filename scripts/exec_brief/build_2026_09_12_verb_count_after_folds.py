#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py, section 18's cliFamilyCensus) or a count over this page's own members table;
# it renders, it does not measure. Style, copy machinery and the tab strip are the previously published
# shell (D0404).
"""Build the standing decision brief for the 2026-09-12 (eighteenth) queue change: the three asks of the
seventeenth publish were accepted and one fork replaced them - the five folds the human chose under the
2026-09-09 brief are landed, and the top-level verb count they were to bring under 25 reads 34 (35 arms with
help). The clause that promised under 25 was never derived from a count, and the option that promised
"roughly 20" repeated the miss beside arithmetic that gave 35. The fork: fold more names away, or amend the
clause to the measured number. Recommend B. One tab, one ask (D0404); the 34 remaining verbs named by family
(D0406); every count is a facts.py fact, never typed.

Usage: python scripts/exec_brief/build_2026_09_12_verb_count_after_folds.py <facts.json> <previous.html> <out.html>
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


# ---- the queue: one fork ----------------------------------------------------------------------------
pending = v("pendingAcceptances")
members = v("pendingMembers")
if len(members) != pending or v("pendingForks") != 1:
    sys.exit(f"refusing: facts disagree - {len(members)} members, {pending} pending, {v('pendingForks')} forks")
if [p["slug"] for p in members] != ["d0457"] or not members[0]["fork"]:
    sys.exit(f"refusing: this page is written for the verb-count fork; the queue is {[p['slug'] for p in members]}")

# ---- the census: what remains, by family; what the five folds removed -------------------------------
C = v("cliFamilyCensus")
verbs, lenses, arms = v("cliTopLevel"), v("cliShowLenses"), v("cliDispatchArms")
if C["topLevel"] != verbs or C["dispatchArms"] != arms:
    sys.exit(f"refusing: the census ({C['topLevel']} / {C['dispatchArms']}) and the surface facts ({verbs} / {arms}) disagree")
PROMISE = 25
to_go = C["namesToUnder25"]                    # names that must leave for the count to read under 25
arms_to_go = C["armsToUnder25"]                # the same, counted as --help lists it (with help)
base, removed, added = C["baselineTopLevel"], C["removedSinceBaseline"], C["addedSinceBaseline"]
if base - len(removed) + len(added) != verbs:
    sys.exit("refusing: baseline - removed + added does not reach the current count")
fams = {f["family"]: f for f in C["families"]}
eff = C["byEffect"]
if sum(eff.values()) != verbs or sum(f["count"] for f in C["families"]) != verbs:
    sys.exit("refusing: the effect and family partitions do not sum to the verb count")


def fam(name):
    return fams[name]["count"], [m["name"] for m in fams[name]["members"]]


gov_n, gov = fam("governance")
int_n, integ = fam("integration")
dist_n, dist = fam("distribution")
gate_n, gating = fam("gating")
rend_n, rend = fam("rendering")
chan_n, chan = fam("channel")
singles = sorted(f for f in fams if fams[f]["count"] == 1)
single_names = [fams[f]["members"][0]["name"] for f in singles]
if gov_n + int_n + dist_n + gate_n + rend_n + chan_n + len(singles) != verbs:
    sys.exit("refusing: the family rows do not cover every verb")
# option A's candidates, by the Decision's text: the three largest families under one router each
a_three = gov_n - 1 + int_n - 1 + dist_n - 1          # three routers remain
a_any_two = min(gov_n - 1 + int_n - 1, gov_n - 1 + dist_n - 1, int_n - 1 + dist_n - 1)
assert a_any_two >= to_go, "the Decision says any two of the three folds reach the number"
tests, failing = v("suiteTests"), v("suiteFailed")
dirty = v("treeUncommitted")["modified"]


def m(names):
    return " ".join(f'<span class="m">{n}</span>' for n in names)


census_table = (
    f'<div class="tbl-wrap"><table data-members="{verbs}"><thead><tr><th>family</th><th>verbs that remain</th></tr></thead><tbody>'
    f'<tr><td>governance ({gov_n})</td><td>{m(gov)}</td></tr>'
    f'<tr><td>integration ({int_n})</td><td>{m(integ)}</td></tr>'
    f'<tr><td>distribution ({dist_n})</td><td>{m(dist)}</td></tr>'
    f'<tr><td>gating ({gate_n})</td><td>{m(gating)}</td></tr>'
    f'<tr><td>rendering ({rend_n})</td><td>{m(rend)}</td></tr>'
    f'<tr><td>channel ({chan_n})</td><td>{m(chan)}</td></tr>'
    f'<tr><td>one-verb families ({len(singles)})</td><td>{m(single_names)}</td></tr>'
    f'</tbody></table></div>')


def courses(rows):
    body = "".join(f"<tr><td>{a}</td><td>{b}</td><td>{c}</td></tr>" for a, b, c in rows)
    return (f'<div class="tbl-wrap"><table><thead><tr><th>course</th><th>what changes</th><th>what it costs</th></tr></thead>'
            f'<tbody>{body}</tbody></table></div>')


# ---- exhibits: the arithmetic, and where option A would land -----------------------------------------
fig1 = logic_lanes(
    f"The folds removed {len(removed)} names and the count reads {verbs}; the promise was written, not derived",
    ("what the folds did", [
        (f"{base} verbs when you chose", "", "accent", ""),
        (f"{len(removed)} folded, {len(added)} arrived", f"{verbs} now", "ok", ""),
    ], ["", ""]),
    ("what was promised", [
        (f"under {PROMISE}, then roughly 20", "", "accent", ""),
        (f"{to_go} more must go", "no fold named", "bad", ""),
    ], ["", ""]),
)
fig2 = downstream(
    f"Reaching under {PROMISE} lands on the words you type; amending lands on no code",
    ("fold further, or amend", "", ""),
    [
        (f"governance: {gov_n} verbs", "your own acts", f"{gov_n - 1} go", "bad"),
        (f"integration: {int_n} verbs", "the commit hook", f"{int_n - 1} go", "bad"),
        (f"distribution: {dist_n} verbs", "adopters' skills", f"{dist_n - 1} go", "warn"),
        ("amend the clause", "no code", "0 go", "ok"),
    ],
)


def words(fragment):
    return len(field_text(re.sub(r"<[^>]+>", " ", markup_only(fragment))).split())


ASKS = [("d0457", "ask-count", "Verb count", "count")]
panel_bodies = {
    "count": f"""<h2>Fold on, or amend</h2>
<p>The item: <q>the five folds land at {arms} top-level verbs, not under {PROMISE}: fold {arms_to_go} more names away or amend the clause to the measured number.</q></p>
<p><strong>A binds:</strong> {to_go} more names leave the typed surface, from the three largest families below. <strong>B binds:</strong> the promise is corrected to {verbs} verbs and {lenses} lenses; the task that owns it closes.</p>
{census_table}
{fig1}
{fig2}
{courses([
    ("A: fold further", f"{a_any_two} to {a_three} names go under judge, repo, adopt", "accept and judge-set renamed; one fold touches the commit hook"),
    ("B: amend the clause", "two clauses corrected; no code", f"--help lists {verbs} names"),
    ("Do nothing", "the task stays open", "a rule stands false"),
])}
<p><strong>True in the model:</strong> {base} verbs when you chose; {len(removed)} folded, each delta read back; {verbs} remain. <strong>My assumption:</strong> what remains are distinct acts; folding buys a shorter list only. <strong>Wrong if</strong> you read --help to recall a verb, or an adopter calls the count friction.</p>
<p><strong>A:</strong> one byte-compared transform per family. <strong>B:</strong> two corrections quoting you. <strong>Neither:</strong> the fork stays here.</p>
<div class="opts" data-records="d0457"><label><input type="radio" name="ask-count" value="A: fold {to_go} or more of the remaining {verbs} names away, to under {PROMISE} - governance under keel judge, integration under keel repo, distribution under keel adopt, one transform each, byte-compared">A: fold further</label><label><input type="radio" name="ask-count" value="B: amend the clause to the measured number - {verbs} verbs, {lenses} lenses behind show - by two clause corrections, no code (recommended)">B: amend the clause (recommended)</label></div>""",
}


def frame(panels_html, tabs_html, title, sub):
    return f"""<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">{title}</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">{sub}</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>One fork.</strong> You chose five folds to bring the verb count under {PROMISE}. All five are landed; the count reads {verbs}, as their own arithmetic said. I recommend <strong>B</strong>: correct the promise to the measured number. <strong>Wrong if</strong> you want a short surface for its own sake.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip"><b>Verbs</b> {base} &rarr; {verbs}</span><span class="chip"><b>Promised</b> under {PROMISE}</span></div>

<div class="tabs" role="tablist" aria-label="The ask">{tabs_html}</div>
{panels_html}
<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on {DATE}; {dirty} files carried uncommitted edits; {tests} tests, {failing} failing. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
"""


TITLE = f"The five folds land at {verbs} verbs, not under {PROMISE}: correct the promise, do not fold on"
SUB = "One fork"


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
print(f"wrote {out_path}: {len(page)} bytes; frame {frame_words} words; panel {panel_words['count']}; "
      f"per tab {frame_words + panel_words['count']}; summed {words(page)}; members table names {verbs} verbs "
      f"({base} at the baseline, {len(removed)} removed, {len(added)} added; {to_go} to go for under {PROMISE})")
assert_fits(out_path)   # probe first, then measure in a browser with and without web fonts; findings remove the page
