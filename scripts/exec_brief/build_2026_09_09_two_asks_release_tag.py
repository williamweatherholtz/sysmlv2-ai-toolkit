#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-09 (twelfth) queue change, terse register under
D0377's 450-word cap: two asks. d0399 stands (D0273 promised under 25 visible verbs, the lens collapse
landed at 70: A collapse five more families / B amend the clause, recommended). d0400 joins it: a
Release carries its tag as a field and the guard binds on that field, never on a title (frozen-schema
addition, process-change marker; accept recommended).

Usage: python scripts/exec_brief/build_2026_09_09_two_asks_release_tag.py <facts.json> <previous.html> <out.html>
"""
import re
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
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
if pending != 2:
    sys.exit(f"refusing: this page states TWO asks and facts.json says {pending} are pending")
verbs, lenses = v("cliTopLevel"), v("cliShowLenses")
live_ro = v("cliLiveTopLevelReadOnly")
sites, files = v("gatingCallSites"), v("gatingCallFiles")
tests, failing = v("suiteTests"), v("suiteFailed")
guards, viol = v("guardsEnforced"), v("guardViolations")
tags, records, tagged = v("versionTags"), v("releaseRecords"), v("releaseRecordsWithTagField")
multi, misbound, rr_warn = v("tagsWithSeveralTitleMatches"), v("tagsMisboundByTitle"), v("releaseGuardWarnings")
PROMISE = 25                        # the number D0273's consequences state - quoted from the text, not measured
left_after_fold = verbs - live_ro

fig2 = logic_lanes(
    "The number was a wish beside a change that could never reach it",
    ("today", [
        ("111 arms", "before the collapse", "muted", ""),
        ("43 lens names folded", "one router", "accent", ""),
        (f"{verbs} verbs remain", f"text says under {PROMISE}", "muted", ""),
    ], ["one commit", "arithmetic"]),
    ("amended", [
        (f"{verbs} verbs, {lenses} lenses", "measured", "accent", ""),
        ("the clause says so", "rest stands", "accent", ""),
        ("help renders from facts", "checked each run", "ok", ""),
    ], ["stated once", "checked"]),
)
fig3 = downstream(
    "Five more collapses land on what gates your commits",
    ("collapse five families", "gating, orientation, rendering,", "authoring, channel"),
    [
        ("hooks and CI", "verbs renamed under them", f"{sites:,} calls", "warn"),
        ("docs and skills", "rewritten alongside", f"{files} files", "warn"),
        ("safety decisions", "two families gate commits", "2 asks", "bad"),
        ("help screen", "what a newcomer reads", f"{verbs} to about 20", "ok"),
    ],
)
fig4 = logic_lanes(
    "A title can truthfully name another version, so reading it for identity fails",
    ("before", [
        ("tag cut", "vN.N.N", "muted", ""),
        ("title contains it", "first hit wins", "warn", ""),
        ("note names next version", "wrong record vouches", "bad", ""),
    ], ["substring", "first hit"]),
    ("after", [
        ("tag cut", "vN.N.N", "muted", ""),
        ("record says tag = vN.N.N", "equal or nothing", "accent", ""),
        ("title says anything true", "never read", "ok", ""),
    ], ["one field", "exact"]),
)
fig5 = downstream(
    "One optional field lands on the frozen core, one guard, and thirteen records",
    ("tag on Release", "String [0..1]", "frozen core: waits for you"),
    [
        ("the guard", "reads the field, never the title", f"{rr_warn} warnings", "ok"),
        ("existing records", "tagged from their first word", f"{tagged} of {records}", "ok"),
        ("release skill", "step 4: record the tag", "1 line", "ok"),
    ],
)

body = f'''<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Two asks wait: amend the under-{PROMISE} verb promise; accept a tag field on releases</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">Two asks</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Two asks.</strong> A rule promised under {PROMISE} commands; the collapse landed at {verbs}: <strong>B</strong>, amend the clause. A release finds its tag by prose containing it: <strong>accept</strong> the field. <strong>Wrong if</strong> you want the surface small (then A), or a schema line is too much for a warning.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Verbs today</b> {verbs}</span></div>

<h2>Ask one &mdash; the verb promise</h2>
<p>{live_ro} of {verbs} verbs are read-only; folding them all leaves {left_after_fold}. Under {PROMISE} needs the writers folded too.</p>
{fig2}
<p><strong>A</strong> five more collapses, a breaking transform and a day each. <strong>B</strong> amend the clause to {verbs} verbs, {lenses} lenses; no code.</p>
{fig3}
<div class="opts" data-records="d0399"><label><input type="radio" name="ask-cli" value="A: collapse the remaining families - gating, orientation, rendering, authoring, channel - one breaking transform each">A: collapse five more families</label><label><input type="radio" name="ask-cli" value="B: amend the clause to the measured number - {verbs} verbs, {lenses} lenses - no code (recommended)">B: amend the clause to the measured number (recommended)</label></div>

<h2>Ask two &mdash; a tag field on releases</h2>
{fig4}
<p>A v0.4.0 record's title truthfully said its payload shipped as v0.4.1; the guard bound that tag to it and called two correct records wrong. {multi} of {tags} tags sit in more than one title. Applied on trunk; your word makes it stand.</p>
{fig5}
<div class="opts" data-records="d0400"><label><input type="radio" name="ask-tag" value="Accept: Release carries tag [0..1]; the guard matches on the field, never the title; records migrated (recommended)">Accept: the tag is a field (recommended)</label><label><input type="radio" name="ask-tag" value="Reverse: no schema change; anchor the substring match on the title instead">Reverse: keep the title match</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on 2026-09-09. {tests} tests, {failing} failing. {guards} checks, {viol} violations. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
'''

page = head + body + tail
words = len(re.sub(r"<[^>]+>", " ", body).split())
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, ~{words} body words before the checker's own count")
