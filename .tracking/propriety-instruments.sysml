// =====================================================================
// ProprietyRound6 - the FIRST TRANCHE over the INSTRUMENTS (dcInstrumentsEnterThePropriety, D0361).
//
// Every earlier round assessed a thing that BLOCKS - a guard, a hook probe, a launcher probe, the
// pre-push fix, a skill. Six defects in one day (issue384, 386, 389, 390, 391, 392) came from things
// that MEASURE, and none had been assessed. This round takes the two whose numbers reach the human
// with no gate between: the facts script the standing decision brief is built from, and the chart
// builders that render those facts as figures. `target` is `instrument:<Sensor title>` so
// `keel show control-structure` computes each sensor's state from these rows (control_structure.rs
// propriety_of); the eighteen sensors this round does not reach stay UNASSESSED there, which is the
// number to read - not the two below.
//
// THE TARGET IS THE SHIPPED FORM. Every defeater below was tried against scripts/exec_brief/facts.py
// and scripts/exec_brief/charts.py as committed at 584df9a, run by path - the facts script over this
// tree and over a git worktree of the same commit with one shape mutated, the chart builders through
// the shipped fit_check measurer in headless Chromium. No paraphrase was assessed.
//
// INDEPENDENCE: both targets were authored by claudeOpus5; this assessor is claudeFable5 - a different
// model, the same AI lineage, and the same actor kind. The two `unsound` verdicts are convictions,
// which an author's lineage may always deliver against its own work; the one `sound` verdict is
// testimony (D0080) and is named as such in its defeater.
// =====================================================================
package ProprietyRound6 {
    private import EngineElement::*;
    private import EnginePropriety::*;

    // ── T1: execBriefFacts - scripts/exec_brief/facts.py ────────────────────────────────────────

    part pf56 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000056";
        :>> title = "execBriefFacts x overfit";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefFacts";
        :>> lens = ProprietyLens::overfit;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "The control written for issue389 (_no_version_is_typed_here, facts.py:667) kills the SPECIMEN and nothing else: its predicate is the regex v\\d+\\.\\d+\\.\\d+, so a typed figure of any other shape passes it. The class issue389 named is 'a typed figure behind a provenance string that reads as authoritative', and the shipped file carries three members today: the `how` of cliShowLenses (facts.py:137-140) tells the reader '37 is the number to publish' while the fact's value on this tree is 51; routingThreeSamplesMinutes is the literal 60 (facts.py:956) presented under a `how` that says it was read from issue392; scopeDaysSinceRule subtracts the typed date(2026, 9, 5) (facts.py:374) under a `how` that says 'D0337's own createdAt', which nothing reads. The first of these is a false figure standing in the provenance the human is told to trust.";
        :>> defeater = "TRIED: ran the shipped script over 584df9a (1m53s, 143 facts, complete=true) and read cliShowLenses back: value 51, `how` ending 'so 37 is the number to publish'. Grepped the shipped file for every typed-figure control: one, the version regex; mutating the specimen from a version string to a bare integer, a date or a short sha passes it. STANDS until the control is rephrased to the class - a numeral in a `how` or a fact value that is not the result of a command or a file read is refused at the write unless an explicit allowance carries its reason - and the three present members are derived or removed. Routed as issue511 with resolver dcFactsTypedFiguresAreRefused.";
    }

    part pf57 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000057";
        :>> title = "execBriefFacts x classCoverage";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefFacts";
        :>> lens = ProprietyLens::classCoverage;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "The class is 'a number on the published brief with no honest provenance'. The instrument covers the members that pass through it: a builder's v(name) refuses a null fact and prints the `how` (build_2026_09_12_verb_count_after_folds.py:40-44). Outside its reach is every numeral a builder types into page prose: build_2026_09_11_two_safety_folds.py:185 publishes 'ten names leave the dispatch' as a word in an f-string, and scripts/check_templates.py --brief holds no rule comparing the page's numerals with the facts file (grep 'facts' over check_templates.py: no hit). A number can reach the human with no `how` at all and every control on the path is silent.";
        :>> defeater = "STANDS: a numeral typed into a builder's HTML string. The skill text (asi-templates s2) forbids it, which is a reminder, not a control (D0047). A cheap control exists and is not built: check_brief extracts every numeral from the page's visible text and refuses one that is not the value of some fact or a fact-derived arithmetic the builder declares. Not routed as an Issue: the verdict is on the instrument's REACH, and the fix is a rule on the checker, which pf56's resolver names as its second clause.";
    }

    part pf58 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000058";
        :>> title = "execBriefFacts x changeRobustness";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefFacts";
        :>> lens = ProprietyLens::changeRobustness;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "The file-parsed counts are regexes over shapes the tree happens to have today, and a scan that matches nothing publishes 0 as a measurement rather than null: cliRecords is len(records) (facts.py:134), decisionsTotal is len(a glob), issuesTotal is a count of one literal pattern. The change-detector antipattern - a check that scans 0 and passes has died without a sound. The CLI counts alone have a second source: the builder compares cliTopLevel with the binary's own census (cliFamilyCensus.topLevel) and refuses on disagreement; the decision, issue and edge counts have none.";
        :>> defeater = "TRIED: a detached git worktree of 584df9a with one ordinary evolution - the type name in .engine/cli/commands.sysml renamed CliCommand to CliVerb in all 85 records - and the shipped facts.py run from that tree with KEEL_BIN naming this repository's binary. It completed (complete=true) and published cliRecords=0, cliShowLenses=0, cliTopLevel=0, cliLive=0 while the binary's census in the same file read topLevel=34; treeUncommitted honestly reported modified=1. Only the builder's cross-check would have stopped the page. STANDS for every parsed count without a second source: the fix is `or None` on a zero scan (the pattern the routing section already uses at facts.py:931) so the builder refuses instead of publishing a zero.";
    }

    part pf59 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000059";
        :>> title = "execBriefFacts x circumventability";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefFacts";
        :>> lens = ProprietyLens::circumventability;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Two routes around a fresh measurement. (1) A STALE ANSWER: artefact.require_complete (scripts/artefact.py:96) refuses a running, failed or pre-D0387 file and accepts any complete one; no builder compares the file's `tree` with HEAD, so a brief can be built from last week's facts and publish last week's numbers under last week's sha - the sha is printed on the page, and a reader is not a control. (2) ANOTHER BINARY: KEEL_BIN names whatever binary the environment says (facts.py:31), and nothing in the output records which one answered.";
        :>> defeater = "TRIED: copied this run's facts file, set tree=0000000 and generatedAt=2026-01-01, and called the shipped require_complete on it - accepted, returned the document. STANDS: a builder run against a stale file. Backstop today: `tree` and the date are printed in the page's provenance line, so the defeat is visible to a reader who checks the sha against `git rev-parse`. The one-line control is a `--tree` check in require_complete's callers or a freshness field the builder refuses when it is not HEAD.";
    }

    part pf60 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000060";
        :>> title = "execBriefFacts x measurability";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefFacts";
        :>> lens = ProprietyLens::measurability;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Most of the 143 facts carry a `how` a third party can re-run, and four re-derived by independent greps matched exactly. Three groups do not re-derive: the perf facts (perfOrientMs 1847, perfGuardFullMs 5613) are wall-clock on an unnamed host - 'in this tree' is the only locator, so another machine's number is a different measurement with the same name; the routing-cost facts are regexes over the PROSE of issue392's description (facts.py:940-965), a hand-written sentence from 2026-09-06 re-read as a figure while a measured record now exists (.keel/metrics/routing-rubric.json, D0459); and the binary: eight `how` strings name ./target/release/keel.exe while _keel_bin resolved keel-serve.exe for this run, the two both answer `keel 0.4.1` and differ by content (sha256 4cf849bd... vs 003548f2...), and no fact records which produced the numbers - the issue406 class.";
        :>> defeater = "TRIED: re-derived cliRecords (85 = grep of `part cli\\w*\\s*:\\s*CliCommand\\s*{`), cliShowLenses (51 = grep of `invocation = \"show `), issuesTotal (297 = grep of `part issueNNN : Issue {` over issues-*.sysml), decisionsTotal (458 = file count of .engine/decisions/*.sysml) - all four equal the published values. Hashed both binaries and read both versions. STANDS for the three groups named; the cheapest repair is one fact `keelBinary` carrying the resolved path, `keel version` and a content hash, written before any command runs.";
    }

    // ── T2: execBriefCharts - scripts/exec_brief/charts.py ──────────────────────────────────────

    part pf61 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000061";
        :>> title = "execBriefCharts x overfit";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefCharts";
        :>> lens = ProprietyLens::overfit;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "issue390's defect class is 'a figure with correct numbers hides text'. The fix measured the `stacked` legend's width (charts.py:80-88) and D0402 added fit_check, which measures four shapes of hiding: OVERFLOW past the canvas, OVERLAP of two texts, OVERRUN of a text past the box it starts in, SCROLL. `bars` (charts.py:52-62) writes each label at x=0 in the .lb face and then paints the bar's rect from x=210 AFTER it in document order, so a label wider than 210px is painted over by its own bar - and that hiding is none of the four kinds: the text does not start inside the rect, and text-under-shape is not measured.";
        :>> defeater = "TRIED: the shipped bars() with a 70-character label ('decisions accepted on the human's own word since the consent-scope rule') and a second short row, written to a page carrying the brief's tokens and measured by the shipped fit_check.measure_raw in headless Chromium, both font passes: CLEAN. The same session's dots(), two_lane() and timeline() with long labels WERE caught (OVERLAP 7px, OVERRUN 101px, OVERLAP 12px) - the backstop reaches those shapes and not this one. STANDS: any bars row whose label exceeds the 210px column. Routed as issue512 with resolver dcFiguresRefuseOccludedText.";
    }

    part pf62 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000062";
        :>> title = "execBriefCharts x classCoverage";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefCharts";
        :>> lens = ProprietyLens::classCoverage;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "The class is 'a figure that tells the reader something its numbers do not'. Members the builders and fit_check reach: text past the canvas, overprinted text, text past its box. Members outside: text occluded by a shape drawn later (pf61); a zero rendered as a bar - bars() clamps width to max(2, ...) (charts.py:57), so a value of 0 is drawn as a 2px bar and reads as a small nonzero; a stacked segment narrower than 46px carries no count on the segment (charts.py:76), by design, with the count in the legend.";
        :>> defeater = "TRIED: bars('m', [('none', 0, 'bad', '')]) - the rect for the zero row has width=2. STANDS for the zero bar and the occlusion; the hidden segment count is a deliberate choice with the legend as the carrier and is not a finding. The zero-bar repair is a stroke-only or absent rect for 0 with the numeral still printed.";
    }

    part pf63 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000063";
        :>> title = "execBriefCharts x changeRobustness";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefCharts";
        :>> lens = ProprietyLens::changeRobustness;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Every colour in the SVG is a CSS custom property the PAGE must define - ten of them (--card --fill --accent --muted --head --ok --bad --warn --line --paper, charts.py:14-36) - and the page's head is not authored by the builder: it is copied forward from the previous publish (build_2026_09_12_verb_count_after_folds.py:33-35). An unresolved var() in an SVG fill falls to the initial value, black, silently: a renamed token in one publish paints every later figure's text black on a dark segment and nothing measures colour. The typeface is a second incidental - the .sm width estimate in stacked() assumes Roboto Condensed at 6px per glyph - and that one IS covered: fit_check's second pass blocks the font hosts and measures the fallback face.";
        :>> defeater = "TRIED: grepped the last published page (decision-brief.html, 166 lines) for each of the ten tokens - all ten defined, three times each (light, dark media query, dark toggle). Holds today by inheritance, not by check. STANDS: a page whose head drops or renames a token. The repair is a tokens probe in check_brief: every var(--x) inside an <svg> resolves to a definition in the page's <style>.";
    }

    part pf64 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000064";
        :>> title = "execBriefCharts x circumventability";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefCharts";
        :>> lens = ProprietyLens::circumventability;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Every builder takes an `aria` argument that REPLACES the data-derived label (charts.py:63, 92, 107, 137, 161), and the aria-label is the one form of the figure a screen reader, the copy-for-AI digest and a re-deriver read. check_brief requires only role=img and an aria-label of 25+ characters (check_templates.py:361); a builder may also place a hand-written <svg> in a figure, and the only rule on it is that it contains 'var(--' (check_templates.py:177).";
        :>> defeater = "TRIED: bars('m', [('a', 17, 'accent', '')], aria='nothing is waiting on you and every count is zero') - emitted verbatim as the aria-label of a figure whose one bar reads 17. STANDS: a caller passing aria. Backstop: none of the fifteen shipped builders passes aria today (grep 'aria=' over scripts/exec_brief/build_*.py: no call site), which is a habit, not a control. The repair is to drop the parameter or have check_brief require every numeral in a figure's <text> to appear in its aria-label.";
    }

    part pf65 : ProprietyFinding {
        :>> id = "c1a60006-0912-4c1e-9f10-000000000065";
        :>> title = "execBriefCharts x measurability";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> target = "instrument:execBriefCharts";
        :>> lens = ProprietyLens::measurability;
        :>> verdict = ProprietyVerdict::sound;
        :>> rationale = "A figure is drawn from the numbers handed to it and nothing else: no builder reads a file, the tree or a clock, and each emits its numbers twice - as <text> in the SVG and as a sentence in the aria-label - so a third party can re-derive what the picture claims from the page alone and compare it with the facts file the builder read. The geometry is arithmetic on the same numbers (longest bar sets the scale, segments proportional to the total, marks at day/days_total), re-derivable from the viewBox.";
        :>> defeater = "TRIED: bars, stacked, timeline and dots with known numbers - every number handed in appears in the generated aria-label (17, 4; 9, 2 and the total 11; 45, 3; 33, 7). SOUGHT AND NOT FOUND: a path by which a figure carries a number it was not given, with aria unforced. The aria override is pf64's finding, not this lens's. TESTIMONY (D0080): the assessor is of the authoring lineage, and this is the one verdict in the round that credits the target.";
    }
}
