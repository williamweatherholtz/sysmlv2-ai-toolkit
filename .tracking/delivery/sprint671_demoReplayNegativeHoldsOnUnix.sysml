// ProjectDeliveryS671 — Sprint 671: The demo-replay integration test's unix negative case exits non-zero for the reason the test states, so CI's Linux run is red only when the replay is wrong (issue484).. EXECUTE. 1 pts.
package ProjectDeliveryS671 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from demoReplayNegativeHoldsOnUnixStory to d0388;

    part demoReplayNegativeHoldsOnUnixStory : Story {
        :>> id = "e42759d8-f92f-4ac7-8248-c322fed6d24b";
        :>> title = "Sprint 671: The demo-replay integration test's unix negative case exits non-zero for the reason the test states, so CI's Linux run is red only when the replay is wrong (issue484). (1 pts)";
        :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS671 {
        action storyDemoReplayNegativeHoldsOnUnix;
        verification storyDemoReplayNegativeHoldsOnUnixDoD : Test { :>> id = "7eab644d-7184-4eea-8cd2-890fe1227cfc"; :>> method = VerificationMethod::test; :>> procedureText = "reverify_demos_replays_each_receipt passes on Linux CI with `sh -c false` (exit 1) as its negative and `cmd /C exit 3` on Windows; the assertion reads the expected code per host; evidenced by the ci-run receipt of the commit carrying the fix."; }
        // RAN: ci-run id=34594558856 workflow=ci
        part storyDemoReplayNegativeHoldsOnUnixDoDR1 : TestResult { :>> id = "3e60f457-3319-4347-a4f8-5adbc509313a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
    }

    verification demoReplayNegativeHoldsOnUnixRefineGate : Test { :>> id = "fc6571f9-40a7-4c80-a8df-899992d6526b"; :>> title = "Sprint 671 refine gate (DoR)"; :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "Chartered by D0388: the D0388 pair had been named for both hosts but probed on one. 1 point: one test file, one CI run as the receipt. NOT A FORK: the shell semantics decide it."; }
    // RAN: ci-run id=34594558856 workflow=ci
    part demoReplayNegativeHoldsOnUnixRefineGateR1 : TestResult { :>> id = "2c7c946e-3585-48e8-9c11-34de4a390ce2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
    verification demoReplayNegativeHoldsOnUnixStandupGate : Test { :>> id = "6024828c-0cbb-409f-9e3d-b8224e5fdda7"; :>> title = "Sprint 671 standup gate"; :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "Probe first: `sh -c exit 3` -> 0 (the 3 binds to $0), `sh -c false` -> 1. Then the edit, the Windows run, the commit, CI."; }
    // RAN: ci-run id=34594558856 workflow=ci
    part demoReplayNegativeHoldsOnUnixStandupGateR1 : TestResult { :>> id = "e73d46a8-ddeb-42c4-9f56-31f8401b15d3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
    verification demoReplayNegativeHoldsOnUnixImplementGate : Test { :>> id = "c6cb23f1-3c37-4db8-b75e-5bd841292a84"; :>> title = "Sprint 671 implement gate"; :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "keel-cli/tests/reverify_demos_replays_each_receipt.rs: fail_prefix/fail_command/fail_exit per host, the assertion reads fail_exit(), the doc comment names both negatives. Commit 2358fd6."; }
    // RAN: ci-run id=34594558856 workflow=ci
    part demoReplayNegativeHoldsOnUnixImplementGateR1 : TestResult { :>> id = "dddd1444-345c-45f3-9cba-5547ddd9a692"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
    verification demoReplayNegativeHoldsOnUnixReviewGate : Test { :>> id = "b2eaad21-1080-4b5e-9047-6cf447c5ced7"; :>> title = "Sprint 671 review gate — three passes per D0172"; :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CI run 34594558856 on 2358fd6: success. The two red runs before it (df90e5d, 238adb9) failed on this line and nothing else."; }
    // RAN: ci-run id=34594558856 workflow=ci
    part demoReplayNegativeHoldsOnUnixReviewGateR1 : TestResult { :>> id = "d53e861b-a09c-483b-a3b9-1efd960332fc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
    verification demoReplayNegativeHoldsOnUnixCloseOutGate : Test { :>> id = "f722c85e-2398-46bf-881c-54b3fe9ffa29"; :>> title = "Sprint 671 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "dcDemoReplayNegativeHoldsOnUnix carries the ci-run receipt. issue484 resolves by computation."; }
    // RAN: ci-run id=34594558856 workflow=ci
    part demoReplayNegativeHoldsOnUnixCloseOutGateR1 : TestResult { :>> id = "806305d5-021d-4d81-a8ee-e978b5aaf9fa"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
    verification demoReplayNegativeHoldsOnUnixRetroGate : Test { :>> id = "653c95ae-ff59-4efb-97de-62999a8d8cbd"; :>> title = "Sprint 671 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "AVOIDABLE-ISSUE SCAN: issue484 was avoidable - the negative was stated for Linux without a Linux probe. No new item - already tracked: issue484 and its resolver dcDemoReplayNegativeHoldsOnUnix carry this finding, and d0388 is the rule it broke. FINDING: a D0388 pair stated for a host it never ran on is the same defect as no pair. The control that caught it is CI's Linux runner, which exists; the gap is that nothing local says 'this pair has run on N of the hosts the test names'. Not tracked as a new item: the fix is to write host-conditional negatives only with a probe on each host in the standup, and land's touched run cannot supply a Linux host. Named here so the retro says why not (D0131)."; }
    // RAN: ci-run id=34594558856 workflow=ci
    part demoReplayNegativeHoldsOnUnixRetroGateR1 : TestResult { :>> id = "ce83fd9f-c829-4132-8d97-5c3319e70399"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2358fd6"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "claudeOpus5"; }
}
