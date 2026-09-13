#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py) or a count over that page's own members tables; it renders, it does not
# measure. Style, copy machinery and the tab strip are the previously published shell (D0404).
"""Build the standing decision brief for the 2026-09-12 (nineteenth) queue change: the answered fork left the
page and four asks joined it. Three are process-change ratifications held under D0337, each already on trunk or
applied verbatim on acceptance - the skill wording of the result-binding rule (D0460), the retro guard reading a
Decision written in capitals (D0461), and the direction-cited guard (D0463). The fourth is a fork: the recall
ranker's dominance setting fails the no-regression rule it was chosen by, and the hand-set bar fails at every
setting the rule admits (D0464) - keep the rule and let the bar fail, or weight the rule so the setting stands.
One tab per ask (D0404); every count is a facts.py fact (section 20), never typed.

Usage: python scripts/exec_brief/build_2026_09_12_three_controls_and_the_recall_fork.py <facts.json> <previous.html> <out.html>
"""
import re
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
from charts import bars                          # noqa: E402
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


# ---- the queue: three ratifications and one fork -------------------------------------------------------
pending = v("pendingAcceptances")
members = v("pendingMembers")
if len(members) != pending or v("pendingForks") != 1:
    sys.exit(f"refusing: facts disagree - {len(members)} members, {pending} pending, {v('pendingForks')} forks")
if [p["slug"] for p in members] != ["d0460", "d0461", "d0463", "d0464"]:
    sys.exit(f"refusing: this page is written for the three controls and the recall fork; the queue is {[p['slug'] for p in members]}")
if [p["fork"] for p in members] != [False, False, False, True]:
    sys.exit("refusing: the fork must be the fourth member and the only one")

tests, failing = v("suiteTests"), v("suiteFailed")
dirty = v("treeUncommitted")["modified"]

# ---- the binding wording (D0460): the lines the skills carry today --------------------------------------
SK = v("skillHeadEqualityLines")
sk_lines = sum(SK.values())

# ---- the retro needle (D0461): how many retro gates write the capital form ------------------------------
RT = v("retrosNamingDecisionUpper")
retros, retros_upper = RT["retros"], RT["namingD0NNN"]

# ---- the direction-cited guard (D0463): its own print at HEAD --------------------------------------------
DC = v("directionCited")
if DC["verdict"] != "PASS" or DC["violations"] != 0:
    sys.exit(f"refusing: the page says the guard passes at HEAD; it printed {DC}")

# ---- the recall fork (D0464): the sweep, the constant, the bar ------------------------------------------
SW = v("dominanceSweep")
H1, H2, HAND = SW["hop1"], SW["hop2"], SW["handSet"]
SETTINGS = sorted(H1, key=float)
if SETTINGS != sorted(H2, key=float):
    sys.exit("refusing: the two arms were not swept over the same settings")
in_force = SW["constant"]
OFF, MID, TOP = "0", "1.25", "1.5"
if f"{in_force:g}" != MID:
    sys.exit(f"refusing: the page says {MID} is in force; the source reads {in_force}")
near_off = H1[OFF]
# the rule as written (no regression on the near arm): a setting is admissible when hits, median and top-3 hold
admissible = [s for s in SETTINGS if H1[s]["hits"] == near_off["hits"] and H1[s]["median"] == near_off["median"]]
refused = [s for s in SETTINGS if s not in admissible]
if admissible != [OFF, TOP]:
    sys.exit(f"refusing: the Decision says the rule admits 0 and 1.5 only; the facts admit {admissible}")
if any(H2[s]["hits"] != H2[OFF]["hits"] for s in admissible):
    sys.exit("refusing: the Decision says the admissible settings tie on the far arm; the facts do not")
far_gain_mid = H2[MID]["hits"] - H2[OFF]["hits"]
first_buying = min((s for s in SETTINGS if H2[s]["hits"] > H2[OFF]["hits"]), key=float)
last_buying = max((s for s in SETTINGS if H2[s]["hits"] > H2[OFF]["hits"]), key=float)
if not (HAND[MID]["bar"] == "MET" and HAND[TOP]["bar"] == "NOT MET" and HAND[OFF]["bar"] == "NOT MET"):
    sys.exit(f"refusing: the page says the bar is met at {MID} only; the readings are {HAND}")
hand_total = 8
hand_reach = 7
if "7/8 or better" not in (SW["barText"] or ""):
    sys.exit("refusing: the bar's text no longer says 7/8 or better")



def courses(rows):
    body = "".join(f"<tr><td>{a}</td><td>{b}</td><td>{c}</td></tr>" for a, b, c in rows)
    return (f'<div class="tbl-wrap"><table><thead><tr><th>course</th><th>what changes</th><th>what it leaves</th></tr></thead>'
            f'<tbody>{body}</tbody></table></div>')


# ---- exhibits: two per ask ----------------------------------------------------------------------------
fig1 = logic_lanes(
    "The skill's suspect signal moves from HEAD-equality to the binding commit the code already uses",
    ("today", [
        ("skill: suspect when the SHA is not HEAD", "", "accent", ""),
        ("every result reads suspect", "one commit after it lands", "bad", ""),
    ], ["", ""]),
    ("changed", [
        ("skill: suspect when a dependency changed", "since the binding commit", "accent", ""),
        ("the skill says what the code computes", "", "ok", ""),
    ], ["", ""]),
)
fig2 = downstream(
    f"Three lines are rewritten; the recording instruction keeps HEAD and nothing in code moves",
    ("three skill edits, verbatim", "", ""),
    [
        ("test-result skill", "two anti-patterns reworded", f"{SK['test-result']}", "warn"),
        ("sprint-standup skill", "the suspect line reworded", f"{SK['sprint-standup']}", "warn"),
        ("code, guards, lenses", "unchanged", "0", "muted"),
    ],
)
fig3 = logic_lanes(
    "A retro that names a Decision in capitals is read as naming the item its file names",
    ("before", [
        ("retro: already tracked by a Decision, in capitals", "", "accent", ""),
        ("guard: names no item", "refused", "bad", ""),
    ], ["", ""]),
    ("with the rule", [
        ("the same retro", "", "accent", ""),
        ("guard: the item exists", "form only, not relevance", "ok", ""),
    ], ["", ""]),
)
fig4 = downstream(
    f"The wider needle lands on {retros_upper} of {retros} retro gates; words in prose stay unmatched",
    ("read the capital form", "", ""),
    [
        ("retro gates in the tree", "", f"{retros}", "muted"),
        ("naming a Decision in capitals", "now read as an item", f"{retros_upper}", "ok"),
        ("Issue and DC as words", "stay unmatched", "0", "muted"),
    ],
)
fig5 = logic_lanes(
    "A quote of you in a Decision gets a Statement it is checked against, or is refused",
    ("today", [
        ("Decision: their words, quoted", "", "accent", ""),
        ("nothing to check the quote against", "", "bad", ""),
    ], ["", ""]),
    ("changed", [
        ("your words recorded first, edge authored", "", "accent", ""),
        ("guard: span found verbatim", "or the record is refused", "ok", ""),
    ], ["", ""]),
)
fig6 = downstream(
    f"The guard reads {DC['scanned']:,} records at HEAD and finds {DC['violations']}; history is counted, not judged",
    ("guard direction-cited", "hard, forward-only", ""),
    [
        ("Decisions and DoDs scanned", "", f"{DC['scanned']:,}", "muted"),
        ("violations today", "", f"{DC['violations']}", "ok"),
        ("citing Decisions before the cutoff", "counted, immutable", f"{DC['historyDecisions']}", "warn"),
        ("citing DoDs with no date", "counted until dates are stamped", f"{DC['historyDoDs']}", "warn"),
    ],
)
fig7 = logic_lanes(
    "A leaves the bar failing; B costs the rule one bend and takes the bar",
    ("option A", [
        (f"rule stands: {TOP}", f"far {H2[TOP]['hits']}, same as off", "accent", ""),
        (f"hand set {HAND[TOP]['reachable']}/{hand_reach}", "fail recorded", "bad", ""),
    ], ["", ""]),
    ("option B", [
        (f"rule weighted: {MID}", f"far {H2[MID]['hits']}", "accent", ""),
        (f"hand set {HAND[MID]['reachable']}/{hand_reach}", "bar met", "ok", ""),
    ], ["", ""]),
)
fig8 = bars(
    f"Every setting that buys a far hit costs the near median; {TOP} buys nothing",
    [(s, H2[s]["hits"], "ok" if s in admissible else "warn", f"median {H1[s]['median']}") for s in SETTINGS],
    unit="",
)


def words(fragment):
    return len(field_text(re.sub(r"<[^>]+>", " ", markup_only(fragment))).split())


ASKS = [
    ("d0460", "ask-binding", "Binding", "binding"),
    ("d0461", "ask-retro", "Retro", "retro"),
    ("d0463", "ask-cited", "Cited", "cited"),
    ("d0464", "ask-dominance", "Recall fork", "dominance"),
]
panel_bodies = {
    "binding": f"""<h2>The skills say what the binding rule says</h2>
<p>The item: <q>the test-result and sprint-standup skills state the binding rule: a pass binds to the commit that lands it, and a suspect is a change since that commit, never the recorded SHA differing from HEAD.</q></p>
<p><strong>Accepting binds:</strong> {sk_lines} skill lines rewritten verbatim as the item spells them, then the generated skill copies refreshed. No code, guard or lens moves; the recording instruction (write the current HEAD) stays.</p>
{fig1}
{fig2}
{courses([
    ("Accept", f"{sk_lines} lines reworded; copies regenerated", "the code rule already in force, now matched by its skill"),
    ("Hold", "nothing moves", "skills that call every landed result suspect"),
    ("Reject", "recorded quoting you", "the same disagreement, standing"),
])}
<p><strong>True in the model:</strong> the binding rule is in code and tested; {SK['test-result']} lines of one skill and {SK['sprint-standup']} of the other still tie suspicion to HEAD-equality. <strong>My assumption:</strong> a skill read before every result is where the wrong signal is learned. <strong>Wrong if</strong> you want HEAD-equality kept as a deliberately stricter signal.</p>
<p><strong>Accept:</strong> three edits, regenerate, verify no skill claims HEAD-equality alone. <strong>Hold:</strong> nothing. <strong>Reject:</strong> recorded.</p>
<div class="opts" data-records="d0460"><label><input type="radio" name="ask-binding" value="Accept: the test-result and sprint-standup skills state the binding rule - a pass binds to the commit that lands it, a suspect is a change since that commit, never judgedAgainst differing from HEAD (recommended)">Accept (recommended)</label><label><input type="radio" name="ask-binding" value="Hold: the skill wording stays proposed; the skills keep their HEAD-equality lines">Hold</label><label><input type="radio" name="ask-binding" value="Reject: the skills keep HEAD-equality as the suspect signal; recorded rejected">Reject</label></div>""",
    "retro": f"""<h2>A Decision named in capitals is a named item</h2>
<p>The item: <q>the retro-backlog guard reads a Decision written in capitals, the way every document writes one, as the item its file names in lower case; only the Decision form folds - Issue and DC in prose stay words.</q></p>
<p><strong>Accepting binds:</strong> a fourth needle in the guard, on trunk already; a retro may discharge itself by naming an existing Decision in either case. The guard still reads form, not whether the Decision covers the finding.</p>
{fig3}
{fig4}
{courses([
    ("Accept", "nothing more moves", f"{retros_upper} retro gates read as they were written"),
    ("Hold", "the code stands unratified", "the same"),
    ("Reject", "one revert; a retro must write the lower-case name", "a spelling rule no document follows"),
])}
<p><strong>True in the model:</strong> {retros_upper} of {retros} retro gates write a Decision in capitals; the refusal text quotes the forms it accepts. <strong>My assumption:</strong> the capital form is the project's spelling, not an error to correct. <strong>Wrong if</strong> you want retros discharged only by a task or an Issue, never by a Decision.</p>
<p><strong>Accept:</strong> nothing. <strong>Hold:</strong> nothing. <strong>Reject:</strong> revert, recorded quoting you.</p>
<div class="opts" data-records="d0461"><label><input type="radio" name="ask-retro" value="Accept: the retro-backlog guard reads a Decision written in capitals as the item its file names; only the Decision form folds (recommended)">Accept (recommended)</label><label><input type="radio" name="ask-retro" value="Hold: the retro needle stays proposed; the code stands">Hold</label><label><input type="radio" name="ask-retro" value="Reject: the widened needle is reverted; a retro names a Decision in lower case or not at all">Reject</label></div>""",
    "cited": f"""<h2>A quote of you links the Statement holding your words</h2>
<p>The item: <q>a Decision or task DoD that quotes the human links a Statement holding their words verbatim - a hard guard, forward-only from {DC['cutoff']}; earlier citing records are one counted line.</q></p>
<p><strong>Accepting binds:</strong> from that date a Decision I record quoting you is preceded by recording your words and carries the edge, or the guard refuses it. The guard is on trunk; your word governs it.</p>
{fig5}
{fig6}
{courses([
    ("Accept", "nothing more moves", f"{DC['historyDecisions']} earlier Decisions stay uncheckable, counted"),
    ("Hold", "the guard stands unratified", "the same"),
    ("Reject", "guard removed", "a quote of you with nothing to check it against"),
])}
<p><strong>True in the model:</strong> the guard passes at HEAD, {DC['scanned']:,} scanned, {DC['violations']} violations, {DC['historyDecisions']} Decisions and {DC['historyDoDs']} undated DoDs counted. <strong>My assumption:</strong> the closed list of attribution phrases catches how citations are actually written here; a bare name and a quote is not read. <strong>Wrong if</strong> you want quoting you in a Decision forbidden outright, or the phrase list open.</p>
<p><strong>Accept:</strong> nothing. <strong>Hold:</strong> nothing. <strong>Reject:</strong> guard removed, recorded quoting you.</p>
<div class="opts" data-records="d0463"><label><input type="radio" name="ask-cited" value="Accept: a Decision or task DoD that quotes the human links a Statement holding their words verbatim - guard direction-cited, hard, forward-only from 2026-09-12 (recommended)">Accept (recommended)</label><label><input type="radio" name="ask-cited" value="Hold: the direction-cited guard stays proposed; the code stands">Hold</label><label><input type="radio" name="ask-cited" value="Reject: the direction-cited guard is removed; recorded rejected">Reject</label></div>""",
    "dominance": f"""<h2>The recall rule, or the bar it was built to meet</h2>
<p>The item: <q>on today's tree the no-regression rule and the hand-set bar cannot both hold - keep the rule and let the bar fail at {TOP}, or weight the rule so {MID} stands.</q></p>
<p><strong>The fork:</strong> the rule, fixed before any sweep: most far hits, no near regression. Now {first_buying} to {last_buying} buy far hits and move the near median {near_off['median']} to {H1[MID]['median']}; off and {TOP} hold it and tie at {H2[OFF]['hits']}. Your {hand_total} questions, {hand_reach} reachable: {HAND[MID]['reachable']} at {MID}, {HAND[TOP]['reachable']} at {TOP} and at off.</p>
{fig8}
{fig7}
{courses([
    ("A: rule stands", f"constant {MID} to {TOP}; the bar fails", "your rebase question, grep only"),
    ("B: rule weighted", f"one clause: median may move one where far gains three; {MID} stands", "a rule bent when first bound"),
    ("Do nothing", "nothing", "a setting in force under a rule it fails"),
])}
<p><strong>True in the model:</strong> the bars; the tie is the bench's own line. <strong>Mine:</strong> B - one median position is the whole near cost; only B meets your cases. <strong>What decides it</strong> is yours: a near hit at {H1[MID]['median']} not {near_off['median']}, against {far_gain_mid} far hits in fifty and that question. <strong>Wrong if</strong> a setting held the near median while buying a far hit - none does.</p>
<div class="opts" data-records="d0464"><label><input type="radio" name="ask-dominance" value="Option B: the rule gains a weight - the near arm's hits and mean rows hold and its median may move by at most one, only where the far arm gains three or more hits; 1.25 stands (recommended)">B: weight the rule (recommended)</label><label><input type="radio" name="ask-dominance" value="Option A: the rule stands as written; the setting moves to 1.5 and the hand-set bar takes a failing result">A: keep the rule</label><label><input type="radio" name="ask-dominance" value="Do nothing: 1.25 stays in force under a rule it fails; the Issue stays open">Do nothing</label></div>""",
}


def frame(panels_html, tabs_html, title, sub):
    return f"""<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">{title}</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">{sub}</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Accept the three controls</strong> - the skills say the binding rule, a Decision in capitals is a named item, a quote of you links your words - and on the recall fork <strong>weight the rule</strong> so its setting stands. <strong>Wrong if</strong> a rule that bends once is worth less than the cases you wrote.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip"><b>Forks</b> 1</span><span class="chip"><b>Guard at HEAD</b> {DC['scanned']:,} scanned, {DC['violations']} violations</span></div>

<div class="tabs" role="tablist" aria-label="The asks">{tabs_html}</div>
{panels_html}
<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on {DATE}; {dirty} files carried uncommitted edits; {tests} tests, {failing} failing; the sweep is quoted from the fork's own research line. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
"""


TITLE = "Accept three controls, and weight the recall rule so its setting stands"
SUB = "Three ratifications and one fork"


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
      f"; summed {words(page)}; the bars name {len(SETTINGS)} settings ({len(admissible)} admissible, {len(refused)} refused)")
assert_fits(out_path)   # probe first, then measure in a browser with and without web fonts; findings remove the page
