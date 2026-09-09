#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py) or a count over that page's own members table; it renders, it does not
# measure. The style and copy machinery are lifted from the previously published page so the brief
# keeps one shell; the tab strip is added to that shell here (D0404).
"""Build the standing decision brief for the 2026-09-09 (fourteenth) queue change: five asks, one tab
each (D0404), no implementation duration anywhere the reader reads (D0405), and the set the verb ask
counts named member by member with its remainder computed (D0406). d0399 (the under-25 verb promise),
d0400 (a Release carries its tag), d0401 (tests bind to properties) stand; d0403 (step 5 names the fit
measurement) and d0407 (the ceiling is measured per tab) join them.

Usage: python scripts/exec_brief/build_2026_09_09_five_asks_tabbed.py <facts.json> <previous.html> <out.html>
"""
import re
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
from logic_exhibits import downstream, logic_lanes     # noqa: E402
sys.path.insert(0, "scripts")
from artefact import claim, require_complete   # noqa: E402  (D0387: a stale answer is refused, a dead run leaves no page)
from check_templates import TERSE_CEILING, field_text, markup_only  # noqa: E402  (the ceiling this page states is the checker's own)
from fit_check import assert_fits              # noqa: E402  (D0402: a page whose exhibits hide text is removed, not published)

facts_path, prev_path, out_path = sys.argv[1:4]
if prev_path == out_path:
    sys.exit("refusing: the style source and the output are the same file - copy the previous page aside first")
J = require_complete(facts_path)    # refuses a running, failed, or pre-D0387 facts file
claim(out_path)                     # the previous page is gone before anything is built
TREE = J["tree"]
prev = open(prev_path, encoding="utf-8").read()
head = prev[: prev.index('<div class="page">')]
tail = prev[prev.index("<script>"):]


def v(name):
    x = J["facts"][name]["value"]
    if x is None:
        sys.exit(f"refusing: fact {name} is null - {J['facts'][name]['how']}")
    return x


pending = v("pendingAcceptances")
if pending != 5:
    sys.exit(f"refusing: this page states FIVE asks and facts.json says {pending} are pending")
verbs, lenses, live = v("cliTopLevel"), v("cliShowLenses"), v("cliLiveTopLevel")
by_family = v("cliLiveTopLevelByFamily")
sites, files = v("gatingCallSites"), v("gatingCallFiles")
tests, failing = v("suiteTests"), v("suiteFailed")
guards, viol = v("guardsEnforced"), v("guardViolations")
tags, records, tagged = v("versionTags"), v("releaseRecords"), v("releaseRecordsWithTagField")
multi, rr_warn = v("tagsWithSeveralTitleMatches"), v("releaseGuardWarnings")
reading_b, asserts_b, offenders_b = v("sourceReadingTestsBefore"), v("codeShapedAssertsBefore"), v("sourceBoundOffendersBefore")
offenders_n = v("sourceBoundOffendersNow")
probe_cases, builders, builders_fit = v("fitProbeCases"), v("briefBuilders"), v("briefBuildersEndingInFit")
PROMISE = 25                        # the number D0273's consequences state - quoted from the text, not measured
legit = reading_b - offenders_b

# ---- D0406: the members of the set option A folds, and what becomes of each -------------------------
# The mapping is D0399 option A's text, quoted: "gating becomes keel gate <check> or keel audit <kind>
# (12 verbs to 2), the read-only orientation verbs join show (11 to 0), rendering becomes keel render
# <what> (4 to 1), the authoring writers become keel record <what> (9 to 1), channel becomes keel github
# <verb>". The MEMBERS are the fact; which of them fold is the rule below, applied to the fact.
FOLD = [
    # family, folds-if, kept routers (existing verbs that survive), new router (a verb that does not exist yet)
    ("gating", lambda r: r["effect"] == "reads", ("gate", "audit"), None),
    ("orientation", lambda r: r["effect"] == "reads", ("show",), None),
    ("rendering", lambda r: r["effect"] == "reads", ("render",), None),
    ("authoring", lambda r: r["effect"] == "writes", ("record",), None),
    ("channel", lambda r: r["name"].startswith("github-"), (), "github"),
]
def m(names):
    return " ".join(f'<span class="m">{n}</span>' for n in names)


rows, folded, new_routers, named = [], 0, 0, 0
for fam, folds, routers, new in FOLD:
    members = by_family[fam]
    into = " / ".join(f"<code>{r}</code>" for r in routers) or f"<code>{new}</code> (new)"
    gone = [r["name"] for r in members if folds(r) and r["name"] not in routers]
    stay = [r["name"] for r in members if r["name"] not in gone]
    folded += len(gone)
    new_routers += 1 if new else 0
    named += len(members)
    rows.append(f"<tr><td>{fam}</td><td>{m(gone)}</td><td>{into}</td><td>{m(stay)}</td></tr>")
untouched = [f for f in by_family if f not in {f[0] for f in FOLD}]
untouched_names = [r["name"] for f in untouched for r in by_family[f]]
named += len(untouched_names)
rows.append(f'<tr><td>{", ".join(untouched)}</td><td>&mdash;</td><td>&mdash;</td><td>{m(untouched_names)}</td></tr>')
if named != live:
    sys.exit(f"refusing: the members table names {named} verbs and the live top-level count is {live}")
remain = live - folded + new_routers
members_table = (f'<div class="tbl-wrap"><table data-members="{named}"><thead><tr><th>family</th>'
                 f'<th>folds ({folded})</th><th>into</th><th>stays</th></tr></thead><tbody>{"".join(rows)}</tbody></table></div>')

# ---- exhibits: two per ask ----------------------------------------------------------------------------
fig1 = logic_lanes(
    "The promise was a wish the change could not reach",
    ("today", [
        ("111 arms", "", "muted", ""),
        ("43 lenses folded", "", "accent", ""),
        (f"{verbs} verbs", f"text says under {PROMISE}", "muted", ""),
    ], ["", ""]),
    ("amended", [
        (f"{verbs} verbs, {lenses} lenses", "", "accent", ""),
        ("clause says so", "", "accent", ""),
        ("help from facts", "", "ok", ""),
    ], ["", ""]),
)
fig2 = downstream(
    f"Folding {folded} verbs lands on what gates your commits",
    (f"{folded} of {live} verbs fold", f"{remain} remain", ""),
    [
        ("hooks, CI, docs", f"{sites:,} calls name a gating verb", f"{files} files", "warn"),
        ("help screen", "", f"{live} to {remain}", "ok"),
    ],
)
fig3 = logic_lanes(
    "A title can truthfully name another version",
    ("before", [
        ("tag cut", "", "muted", ""),
        ("title contains it", "first hit", "warn", ""),
        ("wrong record vouches", "", "bad", ""),
    ], ["", ""]),
    ("after", [
        ("tag cut", "", "muted", ""),
        ("record says tag =", "exact", "accent", ""),
        ("title never read", "", "ok", ""),
    ], ["", ""]),
)
fig4 = downstream(
    "One optional field lands on the frozen core",
    ("tag on Release", "String [0..1]", ""),
    [
        ("the guard", "reads the field", f"{rr_warn} warnings", "ok"),
        ("existing records", "tagged", f"{tagged} of {records}", "ok"),
    ],
)
fig5 = logic_lanes(
    "A test that quotes its source breaks on rewording, not on defects",
    ("before", [
        ("test reads a .rs file", "", "muted", ""),
        ("asserts a line's spelling", "", "warn", ""),
        ("reword fails", "a defect passes", "bad", ""),
    ], ["", ""]),
    ("after", [
        ("test drives the code", "", "accent", ""),
        ("asserts what it does", "", "accent", ""),
        ("quoting is refused", "names the property", "ok", ""),
    ], ["", ""]),
)
fig6 = downstream(
    "The refusal lands on three tests, all rebound",
    ("refuse quoted source", f"{reading_b} tests read source", ""),
    [
        (f"{offenders_b} tests, {asserts_b} asserts", "rebound to behaviour", f"{offenders_n} left", "ok"),
        (f"{legit} legitimate readers", "untouched", "0 false hits", "ok"),
    ],
)
fig7 = logic_lanes(
    "An eye passed a title that ran past its box; a browser did not",
    ("today", [
        ("page built", "", "muted", ""),
        ("screenshot read", "", "warn", ""),
        ("overrun published", "", "bad", ""),
    ], ["", ""]),
    ("measured", [
        ("page built", "", "muted", ""),
        (f"{probe_cases} known cases", "then every label", "accent", ""),
        ("fits, or no page", "", "ok", ""),
    ], ["", ""]),
)
fig8 = downstream(
    "Naming the step lands on every builder, not on the one that has it",
    ("step 5 names fit_check", "process text", ""),
    [
        ("brief builders", "must end in it", f"{builders_fit} of {builders} do", "warn"),
        ("CI", "no browser", "probe by hand", "muted"),
    ],
)
# the words this page carries, counted the way the checker counts them, so the ceiling ask is about THIS page
def words(fragment):
    # the checker's own reading: style and script stripped first (a chart carries its CSS inline), then tags
    return len(field_text(re.sub(r"<[^>]+>", " ", markup_only(fragment))).split())

# ---- panels --------------------------------------------------------------------------------------------
ASKS = [
    ("d0399", "ask-cli", "The verb promise", "verbs"),
    ("d0400", "ask-tag", "A tag field on releases", "tag"),
    ("d0401", "ask-tests", "Tests bind to properties", "tests"),
    ("d0403", "ask-fit", "The fit step is named", "fit"),
    ("d0407", "ask-ceiling", "The ceiling per tab", "ceiling"),
]
panel_bodies = {
    "verbs": f'''<h2>The verb promise</h2>
<p>Under {PROMISE} means folding writers too. <strong>A</strong> folds {folded} of the {live} live verbs into <code>gate</code>, <code>audit</code>, <code>show</code>, <code>render</code>, <code>record</code> and a new <code>github</code>; {remain} remain. The record's own estimate was about 20; naming the members gives {remain}. <strong>B</strong> amends the clause to {verbs} verbs and {lenses} lenses; no code.</p>
{fig1}
{fig2}
<p>What A folds, verb by verb, and what stays:</p>
{members_table}
<div class="opts" data-records="d0399"><label><input type="radio" name="ask-cli" value="A: fold the {folded} verbs named in the table into gate, audit, show, render, record and github; {remain} of {live} remain">A: fold the {folded} named verbs</label><label><input type="radio" name="ask-cli" value="B: amend the clause to the measured number - {verbs} verbs, {lenses} lenses - no code (recommended)">B: amend the clause (recommended)</label></div>''',
    "tag": f'''<h2>A tag field on releases</h2>
<p>A title truthfully named the next version; the guard bound that tag to it. {multi} of {tags} tags sit in several titles. On trunk.</p>
{fig3}
{fig4}
<div class="opts" data-records="d0400"><label><input type="radio" name="ask-tag" value="Accept: Release carries tag [0..1]; the guard matches on the field, never the title; records migrated (recommended)">Accept: the tag is a field (recommended)</label><label><input type="radio" name="ask-tag" value="Reverse: no schema change; anchor the substring match on the title instead">Reverse: keep the title match</label></div>''',
    "tests": f'''<h2>Tests bind to properties</h2>
<p>{offenders_b} tests read a source file and asserted how a line was spelled. Now refused, naming the property; {legit} tests reading source for a real reason are untouched. On trunk.</p>
{fig5}
{fig6}
<div class="opts" data-records="d0401"><label><input type="radio" name="ask-tests" value="Accept: a test asserting program source contains a code-shaped literal is refused under cargo test, naming the property to bind to (recommended)">Accept: refuse it (recommended)</label><label><input type="radio" name="ask-tests" value="Warn only: report the shape in the suite output without failing it">Warn only</label><label><input type="radio" name="ask-tests" value="Reverse: no rule; delete the control and let tests quote source">Reverse: no rule</label></div>''',
    "fit": f'''<h2>The fit step is named</h2>
<p>A brief's figures were checked by screenshot; a box title ran past its box and the eye passed it. A browser now measures every label, web fonts on and blocked, {probe_cases} known cases first; a page that fails is removed. The instrument ships. This ask is whether the process step <em>names</em> it, so a builder cannot skip it unnoticed; a process change waits for your word.</p>
{fig7}
{fig8}
<div class="opts" data-records="d0403"><label><input type="radio" name="ask-fit" value="Accept: step 5 of decision-surfacing runs fit_check beside the contract checker and every builder ends in assert_fits (recommended)">Accept: the step names the measurement (recommended)</label><label><input type="radio" name="ask-fit" value="Reverse: the step names the contract checker only; the measurement stays a builder's private choice">Reverse: builders choose</label></div>''',
}
# the ceiling panel is written last: it states this page's own counts
FRAME_STUB = "__PANELS__"


def frame(panels_html, tabs_html, title, sub):
    return f'''<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">{title}</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">{sub}</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Five asks, one tab each.</strong> A rule promised under {PROMISE} commands; it landed at {verbs}: <strong>B</strong>, amend. A release finds its tag by prose: <strong>accept</strong> the field. Tests quoted their source: <strong>accept</strong> the refusal. Fit was judged by eye: <strong>accept</strong> the measured step. Five tabs cannot share one {TERSE_CEILING}-word cap: <strong>accept</strong> it per tab. <strong>Wrong if</strong> you want the typed surface small (A) or a brief on one screen.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span></div>

<div class="tabs" role="tablist" aria-label="The asks">{tabs_html}</div>
{panels_html}
<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on 2026-09-09. {tests} tests, {failing} failing. {guards} checks, {viol} violations. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
'''


TITLE = "Five asks: amend the verb promise; accept the tag field, test rule, fit step, per-tab ceiling"
SUB = "Five asks"


def render(panel_bodies):
    tabs = "".join(
        f'<button role="tab" aria-selected="{"true" if i == 0 else "false"}" aria-controls="panel-{key}" id="tab-{key}" type="button">{label}</button>'
        for i, (_d, _n, label, key) in enumerate(ASKS))
    panels = "".join(
        f'<section role="tabpanel" id="panel-{key}" aria-labelledby="tab-{key}"{"" if i == 0 else " hidden"}>{panel_bodies[key]}</section>'
        for i, (_d, _n, _label, key) in enumerate(ASKS))
    return frame(panels, tabs, TITLE, SUB), panels


# pass 1: the four finished panels plus a stub, to measure the frame and the largest panel
draft = dict(panel_bodies, ceiling="")
body1, _ = render(draft)
frame_words = words(body1) - sum(words(p) for p in draft.values())
panel_words = {k: words(p) for k, p in panel_bodies.items()}
largest_key = max(panel_words, key=panel_words.get)
# pass 2 below renders the finished page; its summed count is stated after it exists (see SUMMED_STUB)
SUMMED_STUB = "__SUMMED__"

fig9 = logic_lanes(
    "Summed, the cap grows with the queue; per tab it says how terse the writing is",
    ("summed", [
        (f"{pending} asks", "", "muted", ""),
        ("every panel added", "", "warn", ""),
        (f"over {TERSE_CEILING}", "refused", "bad", ""),
    ], ["", ""]),
    ("per tab", [
        ("frame + one panel", f"frame {frame_words} words", "accent", ""),
        ("each in turn", "", "accent", ""),
        (f"all under {TERSE_CEILING}", "", "ok", ""),
    ], ["", ""]),
)
fig10 = downstream(
    "Measuring per tab lands on the checker and on how many asks a page may hold",
    ("ceiling per tab", f"{TERSE_CEILING} words in view", ""),
    [
        ("the checker", "names the tab over", f"{len(ASKS)} tabs read", "ok"),
        ("asks per page", "no cap from the ceiling", "one queue", "ok"),
    ],
)
panel_bodies["ceiling"] = f'''<h2>The ceiling per tab</h2>
<p>The {TERSE_CEILING}-word cap was set for a page read top to bottom. Tabs show one panel at a time. This page: frame {frame_words} words; the largest tab ({largest_key}) adds {panel_words[largest_key]}; all five tabs together {SUMMED_STUB}. Summed it is refused, while no single view is near the cap. Measure the frame plus one tab, every tab must pass. Until you accept, this page cannot be published under the summed rule.</p>
{fig9}
{fig10}
<div class="opts" data-records="d0407"><label><input type="radio" name="ask-ceiling" value="Accept: the 450-word ceiling is measured over the shared frame plus each tab in turn, every tab must pass (recommended)">Accept: measure per tab (recommended)</label><label><input type="radio" name="ask-ceiling" value="Reverse: keep the ceiling summed over the whole page; the queue splits across pages when it grows">Reverse: keep it summed</label></div>'''

body, _panels = render(panel_bodies)
summed = words(head + body + tail)            # the checker's reading: the whole document, style and script stripped; the stub is one token, as its number will be
assert body.count(SUMMED_STUB) == 1
body = body.replace(SUMMED_STUB, str(summed))

# ---- the shell gains a tab strip: CSS into the head, wiring into the tail --------------------------------
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
assert tail.count("</script>") == 1
tail = tail.replace("</script>", TAB_JS)
# the digest walks EVERY panel, hidden or not, in page order (D0404): it is the record of the whole brief
DIGEST_OLD = ".page > h2, .page > h3, .page > p, .page > ul, .page > blockquote, .page > .ask, .page > .turn, .page > .tbl-wrap, .page > .opts, figure.diagram > .msg"
assert tail.count(DIGEST_OLD) == 1
tail = tail.replace(DIGEST_OLD, ".page > .ask, .page > p, [role=\"tabpanel\"] > h2, [role=\"tabpanel\"] > p, [role=\"tabpanel\"] > .tbl-wrap, [role=\"tabpanel\"] > .opts, figure.diagram > .msg")

page = head + body + tail
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes; frame {frame_words} words; panels " +
      ", ".join(f"{k} {n}" for k, n in sorted(panel_words.items(), key=lambda kv: -kv[1])) +
      f", ceiling {words(panel_bodies['ceiling'])}; members table names {named} verbs, {folded} fold, {remain} remain")
assert_fits(out_path)   # probe first, then measure in a browser with and without web fonts; findings remove the page
