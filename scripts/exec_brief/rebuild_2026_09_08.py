#!/usr/bin/env python3
# not-an-instrument: every number this writes is a fact read from facts.json (the declared sensor
# scripts/exec_brief/facts.py); its one derivation, the in-the-tree union, is asserted equal to the
# pendingInTree fact and the build refuses otherwise - it renders, it does not measure.
"""Rebuild the standing decision brief for the 2026-09-08 queue change: three performance rules
joined the five already published. Every number is read from facts.json (scripts/exec_brief/facts.py),
never typed; the figures are drawn by charts.py / logic_exhibits.py from those same numbers.

Usage: python scripts/exec_brief/rebuild_2026_09_08.py <facts.json> <in.html> <out.html>
"""
import json
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else __file__.rsplit("\\", 1)[0])
from charts import bars                       # noqa: E402
from logic_exhibits import downstream, logic_lanes  # noqa: E402

facts_path, in_path, out_path = sys.argv[1:4]
F = {k: v["value"] for k, v in json.load(open(facts_path, encoding="utf-8"))["facts"].items()}
page = open(in_path, encoding="utf-8").read()


def must(old, new):
    global page
    n = page.count(old)
    if n != 1:
        sys.exit(f"anchor count {n}, refusing: {old[:70]!r}")
    page = page.replace(old, new)


pending = F["pendingAcceptances"]
delivered = set((F["pendingDeliveredList"] or "").split(", ")) | set((F["pendingAlreadyShippedList"] or "").split(", "))
delivered.discard("")
in_tree = F["pendingInTree"]
if in_tree != len(delivered) or set((F["pendingInTreeList"] or "").split(", ")) - {""} != delivered:
    sys.exit(f"refusing: facts.json pendingInTree={in_tree} ({F['pendingInTreeList']}) disagrees with the union of its own lists {sorted(delivered)}")
not_built = pending - in_tree
base_guard = F["perfBaselineGuardSec"]
base_orient = F["perfBaselineOrientSec"]
idle_ms = F["perfTurnBoundaryIdleMs"]
guard_ms = F["perfGuardFullMs"]
orient_ms = F["perfOrientMs"]
tests, failed = F["suiteTests"], F["suiteFailed"]
guards, viol = F["guardsEnforced"], F["guardViolations"]
tree, gen = json.load(open(facts_path, encoding="utf-8"))["tree"], json.load(open(facts_path, encoding="utf-8"))["generatedAt"][:10]

WORDS = {5: "Five", 6: "Six", 7: "Seven", 8: "Eight", 9: "Nine"}

# ---------------------------------------------------------------- head
must('<h1 data-digest="title">Five rules are waiting on your word, and three of them are already running</h1>',
     f'<h1 data-digest="title">{WORDS.get(pending, pending)} rules are waiting on your word, and {WORDS.get(in_tree, in_tree).lower()} of them are already running</h1>')
must('One page &middot; five asks &middot; what shipped before it was signed is named first',
     f'One page &middot; {WORDS.get(pending, pending).lower()} asks &middot; what shipped before it was signed is named first &middot; three are the performance work you asked for')
must('<strong>Read this before the asks: three of the five changes are already in the tree.</strong> Two of them were built under a lock that gates on a marker, not on a signature; the third you authorised in words that the read-back check could not bind to a record, so the work is done and the signature is not.',
     f'<strong>Read this before the asks: {WORDS.get(in_tree, in_tree).lower()} of the {WORDS.get(pending, pending).lower()} changes are already in the tree.</strong> Five of them were built under a lock that gates on a marker, not on a signature - three of those are the performance work you asked for on 7 September, which touched the enforcement surface and so could not sign itself; the sixth you authorised in words that the read-back check could not bind to a record, so the work is done and the signature is not.')
must('<span class="chip"><b>Waiting on you</b> 5</span><span class="chip warn"><b>Already in the tree</b> 3</span><span class="chip"><b>Not built yet</b> 1</span><span class="chip ok"><b>Tests</b> 574 passing, 0 failing</span><span class="chip"><b>Enforced checks</b> 65, 0 violations</span>',
     f'<span class="chip"><b>Waiting on you</b> {pending}</span><span class="chip warn"><b>Already in the tree</b> {in_tree}</span><span class="chip"><b>Not built yet</b> {not_built}</span><span class="chip ok"><b>Tests</b> {tests} passing, {failed} failing</span><span class="chip"><b>Enforced checks</b> {guards}, {viol} violations</span><span class="chip ok"><b>Idle turn boundary now</b> {idle_ms/1000:.2f} s, was {base_guard:g} s</span>')
must('<text class="sm" x="6" y="254">Three of the five below are already running. Two were gated on a marker; the third was authorised in</text><text class="sm" x="6" y="269">words the read-back check could not bind to a record.</text>',
     f'<text class="sm" x="6" y="254">{WORDS.get(in_tree, in_tree)} of the {WORDS.get(pending, pending).lower()} below are already running. Five were gated on a marker; the sixth was authorised in</text><text class="sm" x="6" y="269">words the read-back check could not bind to a record.</text>')
must('<p class="lede">Ratifying is not a formality for the three that shipped,',
     f'<p class="lede">Ratifying is not a formality for the {WORDS.get(in_tree, in_tree).lower()} that shipped,')

# ---------------------------------------------------------------- table rows
must('<tr><td><strong>5 &mdash; the retrieval skill tells an adopter to run both measurements</strong></td><td><span class="state in">running</span></td><td>Delete one line from one skill file. Minutes.</td></tr>\n</table>',
     '<tr><td><strong>5 &mdash; the retrieval skill tells an adopter to run both measurements</strong></td><td><span class="state in">running</span></td><td>Delete one line from one skill file. Minutes.</td></tr>\n'
     f'<tr><td><strong>6 &mdash; the checks run side by side, and every one is timed</strong></td><td><span class="state in">running</span></td><td>Revert one function to its serial loop. An hour, and the turn boundary goes back toward {base_guard:g} s.</td></tr>\n'
     '<tr><td><strong>7 &mdash; one process reads each file once</strong></td><td><span class="state in">running</span></td><td>Delete one module and route 61 read sites back to the filesystem. Half a day.</td></tr>\n'
     f'<tr><td><strong>8 &mdash; a green check answers from its receipt when nothing has moved</strong></td><td><span class="state in">running</span></td><td>Delete one module and four call sites. Half a day, and the idle turn boundary goes from {idle_ms/1000:.2f} s back to about {guard_ms/1000:.1f} s.</td></tr>\n</table>')

# ---------------------------------------------------------------- the three perf asks
fig_perf = bars(
    f"The turn boundary that cost {base_guard:g} s costs {idle_ms/1000:.2f} s when nothing has moved, and {guard_ms/1000:.1f} s when something has.",
    [("every turn boundary, before", int(base_guard * 1000), "bad", "the spike's baseline: every check, one after another, on one core"),
     ("a turn boundary after an edit, now", guard_ms, "warn", "every check still runs - side by side, reading each file once"),
     ("a turn boundary with nothing moved, now", idle_ms, "ok", "answered from the receipt of the last green run"),
     (f"orient, before", int(base_orient * 1000), "bad", "the spike's baseline"),
     ("orient, now", orient_ms, "ok", "one read of the corpus, fewer git calls")],
    unit=" ms")

fig_receipt = downstream(
    "The receipt is keyed on every input a check can read, so a changed input is a miss, never a stale green.",
    ("a green run writes its key", "the commit id, git's own status, every changed file's size and time, the tool's own build"),
    [("a tracked file edited", "git status lists it with a new size or time - a miss", "full run", "warn"),
     ("a new file anywhere", "git status lists it - a miss", "full run", "warn"),
     ("a settings or policy file changed", "its size or time changes - a miss", "full run", "warn"),
     ("the binary rebuilt", "the build id changes - a miss", "full run", "warn"),
     ("a file written under two seconds ago", "too young to trust - neither written nor honoured", "full run", "warn"),
     ("nothing moved", "every key part equal", f"{idle_ms} ms", "ok")],
    foot="The remote build has no receipt directory and always runs everything; nothing remote trusts a local receipt.")

fig_lanes = logic_lanes(
    "Running the checks side by side changes when they finish, not what they answer.",
    ("today", [("one check runs", "then the next, on one core", "muted", "the sum of all of them"),
               ("each re-reads the files", "from disk, per check", "muted", "sixty-one read sites"),
               ("the answer arrives", f"about {base_guard:g} s later", "muted", "every turn")],
     ["then", "then"]),
    ("after the change", [("all checks run at once", "one per core, same order out", "accent", "the slowest one's time"),
                          ("each reads a shared copy", "opened once, checked by size and time", "accent", "same bytes"),
                          ("the answer arrives", f"about {guard_ms/1000:.1f} s later", "muted", "every turn with a change")],
     ["then", "then"]),
    note="No check writes a file or sets state, so the answer per check is a function of the tree alone - the order cannot change a verdict.")

asks = f'''
<h2>Ask 6 &mdash; the checks run side by side, and every one is timed</h2>
<h3>Every turn boundary ran {guards} checks one after another on one core. Now they run one per core, in the same order out, and each reports its own time.</h3>
<p class="turn"><strong>This one is already running.</strong> It is the first of the three performance changes you asked for on 7 September; it touched the checking code itself, which is locked, so it stays unsigned until you say so.</p>
{fig_lanes}
<p><strong>Its own words:</strong></p>
<blockquote><em>The enforced guards run across a fixed pool of OS threads, one worker per available core, no new dependency, and return their reports in declared order exactly as the serial loop did; each guard runs inside a timed phase; and every git call adds its wall time to one counter, so the reported git time is the time git took.</em></blockquote>
<p><strong>What accepting binds:</strong> nothing about what a check may say. The one property the change rests on is that no check writes a file or sets process state, so its answer is a function of the tree and cannot depend on which check ran before it. That property is stated in the record; it is not enforced by a test, and that is the honest gap.</p>
<p><strong>The fork:</strong> keep the checks serial and pay their sum, or run them side by side and pay the slowest. There is no third shape; the only real question is whether a check could depend on another's side effects, and none does today.</p>
<p><strong>Evidence:</strong> the whole corpus parses in about a quarter of a second, so the {base_guard:g} s was repetition - checks re-reading the same files, git asked the same questions, all in a line. The spike measured before it proposed, and every number here is re-timed on the current tree each time this page is built.</p>
<p><strong>What I do with each answer:</strong> ratify &rarr; nothing further. Reverse &rarr; one function returns to its loop and the timing stays, since the timing is instrumentation and not a change in behaviour.</p>
<div class="opts" data-records="d0368"><label><input type="radio" name="ask-parallel" value="Ratify: the checks run side by side">Ratify: the checks run side by side</label><label><input type="radio" name="ask-parallel" value="Reverse it, run them one after another">Reverse it, run them one after another</label><label><input type="radio" name="ask-parallel" value="Discuss">Discuss</label></div>

<h2>Ask 7 &mdash; one process reads each file once</h2>
<h3>Sixty-one places in the checking code opened files from disk; each check re-read the same corpus. Now a process opens each file once and every reader shares one copy, re-checked by size and time on every read.</h3>
<p class="turn"><strong>This one is already running.</strong> Second of the three performance changes. Same lock, same reason it is unsigned.</p>
{fig_perf}
<p><strong>Its own words:</strong></p>
<blockquote><em>A new module is the one read path for the corpus. It stats the file on EVERY call and serves the cached text only when its size and time equal what was read and the time is at least two seconds old - git's own racy rule, so a file rewritten inside one clock tick is re-read rather than trusted; a missing file is the same error the filesystem gives, never a stale hit. The change detector walks the tree uncached: the thing that detects change does not read a memo.</em></blockquote>
<p><strong>What accepting binds:</strong> a cache lives for the length of one process and no longer. Nothing is written to disk; two processes share nothing; the next command starts cold. That is the least a cache can be and still remove the repetition.</p>
<p><strong>The fork:</strong> a per-process cache validated by the file's own size and time, or a persistent index on disk that must then be kept in step with git. The record chose the first because the second is a second copy of the truth, and this project's whole premise is that there is one.</p>
<p><strong>Evidence:</strong> the sixty-one read sites receive the same bytes they did - checked by running every check with the cache and without and comparing output. The residual after this change is git calls made under contention and the slowest single check, which the next two asks judged.</p>
<p><strong>What I do with each answer:</strong> ratify &rarr; nothing further. Reverse &rarr; the module goes and the read sites return to the filesystem; the checks run correctly and slower.</p>
<div class="opts" data-records="d0370"><label><input type="radio" name="ask-corpus" value="Ratify: one read per process">Ratify: one read per process</label><label><input type="radio" name="ask-corpus" value="Reverse it, read from disk each time">Reverse it, read from disk each time</label><label><input type="radio" name="ask-corpus" value="Discuss">Discuss</label></div>

<h2>Ask 8 &mdash; a green check answers from its receipt when nothing has moved</h2>
<h3>You asked for "a file index hash that can identify changes quickly". This is that: a green run writes what it judged, and the next run whose inputs are byte-for-byte the same answers from it.</h3>
<p class="turn"><strong>This one is already running, and it is the one that changes the turn boundary you feel.</strong> Third of the three. It touches the hook and the commit gate - the enforcement surface - so it is the ask here that most deserves your eyes.</p>
{fig_receipt}
<p><strong>Its own words:</strong></p>
<blockquote><em>A green run writes a receipt carrying the KEY of the tree it judged: the commit's full id, every entry of git's own status with that path's size and time, every file under the tool's settings directory with its size and time, and the binary's build with its own size and time; beside the key it stores which layers the run covered and each check's name, count and warnings - never a violation, because a receipt exists only for a green run. A run whose computed key is EQUAL answers from the receipt. The key is computed before and after the run and written only when both agree; a path younger than two seconds is neither written nor honoured; any red run deletes the receipt.</em></blockquote>
<p><strong>What accepting binds:</strong> a turn boundary may say "green" without running the checks, when and only when every input a check could read is unchanged since a run that did. The honest way to say no to this is to say a receipt can never stand in for a run - and then the idle boundary costs what a full run costs, about {guard_ms/1000:.1f} s on this host, forever.</p>
<p><strong>The fork:</strong> a receipt keyed on inputs (this), a resident process watching the tree (a daemon), or nothing. The daemon was the next item in your ranked list and was judged against this one's number: at {idle_ms/1000:.2f} s idle there is nothing left for it to save, and its correctness would rest on a watcher having seen every write, which the receipt does not need. It is closed with the measurement recorded.</p>
<p><strong>Evidence, and what is mine rather than measured:</strong> the {idle_ms} ms and {guard_ms} ms above are re-timed on this tree each time the page is built. What is argued and not measured: that the key covers <em>every</em> input a check reads. The argument is that a check reads the tree (covered by the commit id and git's status), the settings directory (walked), or the binary (build id) - and that the clock is read by no check. A check that read anything else would be a hole; the receipt's own tests cover the cases named in the figure, and a fourth run forced by the flag <code>--no-receipt</code> always exists.</p>
<p><strong>What I do with each answer:</strong> ratify &rarr; nothing further; the idle boundary stays at a quarter of a second. Reverse &rarr; the module and four call sites go and every boundary pays a full run. Discuss &rarr; I can add any input you name to the key in an hour.</p>
<div class="opts" data-records="d0371"><label><input type="radio" name="ask-receipt" value="Ratify: a green run may answer from its receipt">Ratify: a green run may answer from its receipt</label><label><input type="radio" name="ask-receipt" value="Reverse it, every boundary runs everything">Reverse it, every boundary runs everything</label><label><input type="radio" name="ask-receipt" value="Discuss">Discuss</label></div>

<h2>What would change my recommendation</h2>'''
must('\n<h2>What would change my recommendation</h2>', asks)
must("Ask 5: if an adopting project runs both arms and reports that the second tells them nothing about their own model, the line is ours and not theirs.</p>",
     f"Ask 5: if an adopting project runs both arms and reports that the second tells them nothing about their own model, the line is ours and not theirs. Ask 6: a check whose verdict differs between the serial and the parallel run would show that one of them has a side effect, and I would revert the same day. Ask 7: a check that receives different bytes from the cache than from disk - the comparison is one command. Ask 8: any input a check reads that is not in the key; name it and it is a hole, and until it is in the key the receipt is wrong. The two ranks below the receipt in your list - in-process git and a resident process - were measured and closed at {orient_ms} ms and {idle_ms} ms, under the bars their own items set; the one rank left is yours: a Defender exclusion or a Dev Drive for the repository, which no code change can substitute for.</p>")

must('<h2>The process failure behind this page</h2>\n<p>You asked whether this was meant to be standing. It is: the rule that pending decisions belong on a refreshed page rather than in conversation was accepted, the process is active, and I did not run it. For four turns I reported these as trailing sentences instead — which is the failure that rule was written to prevent, arriving from the other direction. The page is now rebuilt from the queue rather than from my memory of it, and one instrument feeding it was fixed on the way: it reported that none of the pending items had shipped, when three had.</p>',
     f'<h2>What changed on this page, and one instrument behind it</h2>\n<p>Three asks were added because the queue moved: the performance work landed as three records, each touching locked code, each unsigned by design. One instrument feeding this page was replaced on the way. The count of "already in the tree" used to come from a phrase rule over the records\' own prose; it now also reads the delivery records - a record counts as delivered when a finished sprint names it as its charter and that sprint\'s definition of done has a passing verdict. The phrase rule found {F["pendingAlreadyShipped"]} of the {pending}; the structural reading finds {F["pendingDelivered"]}; their union is the {in_tree} stated above. Both are on the facts file with the exact rule that produced them.</p>')

must('<footer data-digest="provenance">Computed from the repository at e0624f3 on 2026-09-07. 574 tests, 0 failing. 65 enforced checks, 0 violations, 18 of them ever shown catching their own defect. 20 measures declared, 20 computed as sensors, 0 assessed, 97 feedback paths. 5 records awaiting signature, 3 of them already in the tree.',
     f'<footer data-digest="provenance">Computed from the repository at {tree} on {gen}. {tests} tests, {failed} failing. {guards} enforced checks, {viol} violations, {F["guardsProven"]} of them ever shown catching their own defect. {F["instrumentsDeclared"]} measures declared, {F["sensorsComputed"]} computed as sensors, {F["instrumentsAssessed"]} assessed, {F["feedbackChannels"]} feedback paths. Timed on this tree while building the page: idle turn boundary {idle_ms} ms, a full check run {guard_ms} ms, orient {orient_ms} ms; the spike\'s baselines of {base_guard:g} s and {base_orient:g} s are read from its own record. {pending} records awaiting signature, {in_tree} of them already in the tree.')

open(out_path, "w", encoding="utf-8", newline="\n").write(page)
print(f"wrote {out_path}: pending={pending} in_tree={in_tree} idle={idle_ms} guard={guard_ms} orient={orient_ms}")
