// =====================================================================
// ProjectDeliveryS78 — Sprint 78: loadBearingReport — formalize the load-bearing-decisions report
// as `sysmlv2 decisions` (no more ad-hoc python). Ranks accepted Decisions by dependence + flags
// antiquation (uncritiqued / references_retired / superseded_in_part). Serves the critique campaign
// + the user's "surface this going forward" ask.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS78 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from loadBearingReportStory to d0079;

    part loadBearingReportStory : Story {
        :>> id = "63940516-2c7d-4e8f-b078-51627384af01";
        :>> title = "Sprint 78: loadBearingReport (3 pts) — `sysmlv2 decisions`, formalized load-bearing report";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS78 {
        action storyLoadBearingReport;
        verification storyLoadBearingReportDoD : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af02"; :>> method = VerificationMethod::test; :>> procedureText = "`sysmlv2 decisions` (view.rs decisions_report + main.rs cmd_decisions): accepted Decisions ranked by charters-to x2 + cross-citations, with antiquation flags uncritiqued (no full Core-3 critique) / references_retired (curated retired-mechanism scan) / superseded_in_part (heuristic line co-occurrence). Replaces ad-hoc python ranking (user feedback). VERIFY (live): top d0066(65)/d0070(27)/d0077(23)/d0048(22)/d0054(21)...; flags populated; clippy --workspace -D warnings + cargo test (32 lib) + rust validate (90) + guard all (9) green; doc-sync usage.md."; }
        part storyLoadBearingReportDoDR1 : TestResult { :>> id = "e9fa98dd-f8d9-4dd9-8999-1845366e281f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification loadBearingReportRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af03"; :>> title = "Sprint 78 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the human wants the load-bearing ranking FORMALIZED as a report (not bespoke python) for ongoing use, flagging 'antiquated' decisions. Accepted flags: uncritiqued + stale-process(approx references_retired/superseded) + references_retired; zombies (status != accepted) explicitly OUT of scope (handled elsewhere). Build as a Rust subcommand (like coverage/assured). estimatedPoints=3. #CharteredBy d0079 (assurance analysis). DoD accepted."; }
    part loadBearingReportRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b078-51627384af04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification loadBearingReportStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af05"; :>> title = "Sprint 78 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: view.rs decisions_report — accepted-Decision targets; charters from charteredby edges; citations by scanning OTHER decision-file texts for the dNNNN/DNNNN token; uncritiqued from compute_critique_coverage; references_retired from a curated RETIRED_MECHANISMS list; superseded_in_part heuristic (name + supersede/retire/replace on one line). Wire `sysmlv2 decisions`. Unit-test the name/mention helpers. No blockers."; }
    part loadBearingReportStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b078-51627384af06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification loadBearingReportImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af07"; :>> title = "Sprint 78 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented decisions_report + helpers (find_decision_name/count_mentions/supersede_near) + cmd_decisions + usage.md row. Bug caught + fixed live: find_decision_name used `?` inside the line loop, so the first comment line returned None for the whole fn -> texts empty -> citations all 0; changed to `else continue` -> citations correct (matches the earlier scan: d0066=65, d0070=27). VERIFY (live): ranking + flags correct; references_retired catches query.py/parity_check (d0048/d0070/d0077/d0050/d0074); superseded_in_part labeled HEURISTIC in output. clippy --workspace --all-targets -D warnings green; cargo test 32 lib pass (+decision_name_and_mentions_helpers); rust validate 90 clean; guard all (9) PASS."; }
    part loadBearingReportImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b078-51627384af08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification loadBearingReportReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af09"; :>> title = "Sprint 78 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The report is a pure computed view (no stored data) consistent with coverage/critique-coverage/assured — directly honoring the 'formalize, don't bespoke-python' preference. Honest signal labeling: superseded_in_part is explicitly marked HEURISTIC (line co-occurrence, a hint not authority); zombies scoped out per the human. The uncritiqued flag doubles as the critique-campaign worklist (73/80 uncritiqued). doc-sync: usage.md row added. The `?`-in-loop bug is exactly the kind the new parser-package guard + tests catch elsewhere; here a unit test (decision_name_and_mentions_helpers) locks it. No schema/process change."; }
    part loadBearingReportReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b078-51627384af0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification loadBearingReportCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af0b"; :>> title = "Sprint 78 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyLoadBearingReport + backlog loadBearingReport DoDs recorded pass; `sysmlv2 decisions` live; clippy + cargo test + rust validate + guard all (9) green. Self-evidencing (D0051). The load-bearing ranking is now a first-class report; future critique-target selection runs `sysmlv2 decisions`, not python."; }
    part loadBearingReportCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b078-51627384af0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification loadBearingReportRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b078-51627384af0d"; :>> title = "Sprint 78 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: formalizing the ranking turned a throwaway python one-liner into a durable, reviewable, regenerable report (the user's standing preference, now also a memory) — and it immediately doubles as the critique worklist (uncritiqued flag). CAUGHT (avoidable-issue candidate): the find_decision_name `?`-inside-loop bug returned None on the first comment line, silently zeroing all citations — found by sanity-checking the live output against the prior scan (the value of always eyeballing a new report's numbers vs a known baseline). Locked with a unit test. Lesson: `?` in a scanning loop is a fail-quiet trap (same family as issue027's silent drop). Estimate 3 pts held. NEXT: the report says 73/80 decisions uncritiqued — the campaign continues 2 decisions/batch; D0005's material-change fix (option b) is the other accepted thread."; }
    part loadBearingReportRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b078-51627384af0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2033dd7"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
