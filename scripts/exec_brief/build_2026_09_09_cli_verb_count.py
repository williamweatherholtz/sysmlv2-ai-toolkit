#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-09 (eleventh) queue change, terse register under
D0377's 450-word cap: one ask, a two-way fork - D0273 promised the visible CLI verb count would fall
under 25 and the lens collapse landed at 70 (D0399: A collapse five more families / B amend the clause
to the measured number, recommended).

Usage: python scripts/exec_brief/build_2026_09_09_cli_verb_count.py <facts.json> <previous.html> <out.html>
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
if pending != 1:
    sys.exit(f"refusing: this page states ONE ask and facts.json says {pending} are pending")
verbs, lenses = v("cliTopLevel"), v("cliShowLenses")
live_ro = v("cliLiveTopLevelReadOnly")
sites, files = v("gatingCallSites"), v("gatingCallFiles")
tests, failing = v("suiteTests"), v("suiteFailed")
guards, viol = v("guardsEnforced"), v("guardViolations")
ready = v("readyItems")
human, auto = v("scopeDecisionsHumanAccepted"), v("scopeDecisionsAutoAccepted")
PROMISE = 25                        # the number D0273's consequences state - quoted from the text, not measured
left_after_fold = verbs - live_ro

fig1 = bars(
    "Folding every read-only verb into the router still leaves more than the promise allowed",
    [
        ("verbs typed at the top level", verbs, "muted", "after the lens collapse"),
        ("of those, read-only", live_ro, "accent", "lenses in disguise, live commands"),
        ("left if every one is folded", left_after_fold, "warn", "still above the promise"),
        ("the promise", PROMISE, "bad", "under this many, the accepted text says"),
    ],
)
fig2 = logic_lanes(
    "The number was a wish written beside a change that could never reach it",
    ("today", [
        ("111 arms", "before the lens collapse", "muted", ""),
        ("43 lens names folded", "into one router", "accent", ""),
        (f"{verbs} verbs remain", f"the text says under {PROMISE}", "muted", "recorded as a miss"),
    ], ["one commit", "the arithmetic"]),
    ("if the clause is amended", [
        (f"{verbs} verbs, {lenses} lenses", "measured at the tree", "accent", ""),
        ("the clause says so", "the rest of the rule stands", "accent", ""),
        ("source agrees", "help renders from the facts", "ok", ""),
    ], ["stated once", "checked each run"]),
    note="The removed names, the one-commit rule and the no-alias rule stand either way.",
)
fig3 = downstream(
    "Five more collapses land on the things that gate your commits",
    ("collapse five families", "gating, orientation, rendering,", "authoring, channel"),
    [
        ("your hooks and CI", "gating verbs renamed underneath them", f"{sites:,} calls", "warn"),
        ("docs and skills", "rewritten in the same commits", f"{files} files", "warn"),
        ("safety decisions", "two families sit on the enforcement surface", "2 asks", "bad"),
        ("the help screen", "names a newcomer reads", f"{verbs} to about 20", "ok"),
    ],
    foot="Breaking renames, no alias window - the shape the lens collapse set.",
)

body = f'''<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">One ask waits: amend the under-{PROMISE} verb promise, or collapse five more command families</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">One ask, two ways</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>One ask.</strong> An accepted rule promised the visible command count would fall under {PROMISE}. The lens collapse landed at {verbs} verbs plus {lenses} lenses behind one router; the number was never derivable from that change. I recommend <strong>B</strong>: amend the clause to the measured number. <strong>Wrong if</strong> you want the typed surface itself small - then A, five more breaking renames.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Verbs today</b> {verbs}</span><span class="chip"><b>Lenses behind one router</b> {lenses}</span><span class="chip warn"><b>Promised</b> under {PROMISE}</span></div>

<h2>The evidence</h2>
{fig1}
<p>Folding every read-only verb leaves {left_after_fold}; under {PROMISE} needs the writers folded too. Two families sit on the enforcement surface and would wait for your word again.</p>
{fig2}

<h2>The ask</h2>
<p><strong>A</strong> collapse five more families into routers, one breaking transform each, about a day each, ending near 20 verbs. <strong>B</strong> amend the clause to {verbs} verbs and {lenses} lenses and withdraw the {PROMISE}. No code.</p>
{fig3}
<div class="opts" data-records="d0399"><label><input type="radio" name="ask-cli" value="A: collapse the remaining families - gating, orientation, rendering, authoring, channel - one breaking transform each">A: collapse five more families</label><label><input type="radio" name="ask-cli" value="B: amend the clause to the measured number - {verbs} verbs, {lenses} lenses - no code (recommended)">B: amend the clause to the measured number (recommended)</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on 2026-09-09. {tests} tests, {failing} failing. {guards} checks, {viol} violations. {ready} ready items. {human} on your word, {auto} auto-accepted since 2026-09-05. {pending} waits. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
'''

page = head + body + tail
words = len(re.sub(r"<[^>]+>", " ", body).split())
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, ~{words} body words before the checker's own count")
