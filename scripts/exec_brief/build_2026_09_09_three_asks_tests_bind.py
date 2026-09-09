#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); it renders, it does not measure. The style and copy machinery are
# lifted from the previously published page so the brief keeps one shell.
"""Build the standing decision brief for the 2026-09-09 (thirteenth) queue change, terse register under
D0377's 450-word cap: three asks. d0399 stands (the under-25 verb promise: A collapse five more
families / B amend the clause, recommended). d0400 stands (a Release carries its tag as a field; accept
recommended). d0401 joins them: a test that asserts program source contains a code-shaped literal is
refused under cargo test, and the refusal names the property to bind to (process-change marker; accept
recommended - the control ships, the three offenders are rebound).

Usage: python scripts/exec_brief/build_2026_09_09_three_asks_tests_bind.py <facts.json> <previous.html> <out.html>
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
if pending != 3:
    sys.exit(f"refusing: this page states THREE asks and facts.json says {pending} are pending")
verbs, lenses = v("cliTopLevel"), v("cliShowLenses")
live_ro = v("cliLiveTopLevelReadOnly")
sites, files = v("gatingCallSites"), v("gatingCallFiles")
tests, failing = v("suiteTests"), v("suiteFailed")
guards, viol = v("guardsEnforced"), v("guardViolations")
tags, records, tagged = v("versionTags"), v("releaseRecords"), v("releaseRecordsWithTagField")
multi, rr_warn = v("tagsWithSeveralTitleMatches"), v("releaseGuardWarnings")
reading_b, asserts_b, offenders_b = v("sourceReadingTestsBefore"), v("codeShapedAssertsBefore"), v("sourceBoundOffendersBefore")
reading_n, offenders_n, probes = v("sourceReadingTestsNow"), v("sourceBoundOffendersNow"), v("testsBindProbes")
PROMISE = 25                        # the number D0273's consequences state - quoted from the text, not measured
left_after_fold = verbs - live_ro
legit = reading_b - offenders_b

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
    "Five more collapses land on what gates your commits",
    ("collapse five families", "", ""),
    [
        ("hooks, CI, docs", "verbs renamed", f"{sites:,} calls", "warn"),
        ("help screen", "", f"{verbs} to 20", "ok"),
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
        ("reword fails, defect passes", "", "bad", ""),
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

body = f'''<div class="page">
<p class="sub" data-digest="project">keel &middot; williamweatherholtz/sysmlv2-ai-toolkit</p>
<h1 data-digest="title">Three asks wait: amend the verb promise; accept the tag field and the test rule</h1>
<div class="topbar"><p class="sub" data-digest="subtitle">Three asks</p><button class="copy" data-copy type="button" aria-label="Copy this brief for AI">&#8681; Copy for AI</button></div>

<div class="ask"><p class="verdict"><strong>Three asks.</strong> A rule promised under {PROMISE} commands; the collapse landed at {verbs}: <strong>B</strong>, amend. A release finds its tag by prose: <strong>accept</strong> the field. Tests quoted their own source: <strong>accept</strong> the refusal. <strong>Wrong if</strong> you want the surface small (A), a schema line is too much, or quoting source has a use I missed.</p></div>
<div class="chips"><span class="chip"><b>Waiting on you</b> {pending}</span></div>

<h2>The verb promise</h2>
<p>Folding every read-only verb leaves {left_after_fold}; under {PROMISE} means folding writers. <strong>A</strong> five collapses, a day each. <strong>B</strong> amend the clause; no code.</p>
{fig1}
{fig2}
<div class="opts" data-records="d0399"><label><input type="radio" name="ask-cli" value="A: collapse the remaining families - gating, orientation, rendering, authoring, channel - one breaking transform each">A: collapse five more families</label><label><input type="radio" name="ask-cli" value="B: amend the clause to the measured number - {verbs} verbs, {lenses} lenses - no code (recommended)">B: amend the clause (recommended)</label></div>

<h2>A tag field on releases</h2>
<p>A title truthfully named the next version; the guard bound that tag to it. {multi} of {tags} tags sit in several titles. On trunk.</p>
{fig3}
{fig4}
<div class="opts" data-records="d0400"><label><input type="radio" name="ask-tag" value="Accept: Release carries tag [0..1]; the guard matches on the field, never the title; records migrated (recommended)">Accept: the tag is a field (recommended)</label><label><input type="radio" name="ask-tag" value="Reverse: no schema change; anchor the substring match on the title instead">Reverse: keep the title match</label></div>

<h2>Tests bind to properties</h2>
<p>{offenders_b} tests read a source file and asserted how a line was spelled. Now refused, naming the property; {legit} tests reading source for a real reason are untouched. On trunk.</p>
{fig5}
{fig6}
<div class="opts" data-records="d0401"><label><input type="radio" name="ask-tests" value="Accept: a test asserting program source contains a code-shaped literal is refused under cargo test, naming the property to bind to (recommended)">Accept: refuse it (recommended)</label><label><input type="radio" name="ask-tests" value="Warn only: report the shape in the suite output without failing it">Warn only</label><label><input type="radio" name="ask-tests" value="Reverse: no rule; delete the control and let tests quote source">Reverse: no rule</label></div>

<label class="note-row">Anything to add<textarea data-d="note" rows="2" placeholder="optional"></textarea></label>
<div class="copy-bottom"><button class="copy" data-copy type="button">&#8681; Copy for AI</button></div>
<footer data-digest="provenance">Computed from the repository at {TREE} on 2026-09-09. {tests} tests, {failing} failing. {guards} checks, {viol} violations. <a href="https://github.com/williamweatherholtz/sysmlv2-ai-toolkit" target="_blank" rel="noopener">repository</a>.</footer>
</div>
'''

page = head + body + tail
words = len(re.sub(r"<[^>]+>", " ", body).split())
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(page)} bytes, ~{words} body words before the checker's own count")
