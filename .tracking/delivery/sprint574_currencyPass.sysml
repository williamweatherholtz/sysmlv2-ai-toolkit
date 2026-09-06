// ProjectDeliveryS574 — Sprint 574: the unattended currency pass in one command, with a declared removable schedule that is inert until the human accepts it. EXECUTE. 3 pts.
package ProjectDeliveryS574 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from currencyPassStory to d0338;

    part currencyPassStory : Story {
        :>> id = "bad8b8a6-dd41-461a-8ff0-ae5818da5791";
        :>> title = "Sprint 574: the unattended currency pass in one command, with a declared removable schedule that is inert until the human accepts it (3 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS574 {
        action storyCurrencyPass;
        verification storyCurrencyPassDoD : Test { :>> id = "cc6b5947-a1a1-471b-b755-1dae0836b8aa"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcScheduledCurrency (D0338, proposed - a process-surface change waits for the human under D0337; verifies srUnattendedPullIsIdempotent). DONE: keel currency [ROOT] --repo --by runs github-pull (verbatim Statements, trust tier recorded, never triaged), library sync (unreachable stated stale) and the drift read-back, printing one report with one exit code; .github/workflows/currency.yml is the declared removable schedule (daily + dispatch) running that one command as githubRecorder, gating and committing only .tracking/intake - and INERT until d0338AcceptR1 exists in the tree. Live: the first pass ingested GH#54-59 as untrusted Statements (st092-st097); the second pass over the same open issues ingested nothing and reported 18 already recorded."; }
        // RAN: dcScheduledCurrency pass recorded (suite 536/0; live double run 6 then 0 ingested); D0338 recorded proposed, outside standing consent (D0337); currency.yml inert step verified by reading the file
        part storyCurrencyPassDoDR1 : TestResult { :>> id = "7dc7c1d8-c60c-492b-97b7-4db1c9b3e50a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c8a200e658d4d6d0c031ea68ddf66c3ef51c7852"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification currencyPassRefineGate : Test { :>> id = "4d17a05c-2fa6-4b03-9dc2-60c94f391776"; :>> title = "Sprint 574 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the three commands existed; the DoD asked for one place and one removable file. The design question was D0337, an hour old: a scheduled job that commits to main is a process-surface change, so the file ships inert with the human's acceptance as its switch."; }
    verification currencyPassStandupGate : Test { :>> id = "fe1f7870-fee5-4d61-a6a1-402cb89cd152"; :>> title = "Sprint 574 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "SIX NEW ISSUES ARRIVED THROUGH THE PASS: GH#54-59, all filed by the decider from downstream adoption (keelw cache seeding, three binary resolutions disagreeing, migrate overwriting a chartered activation.toml, the channel script hardcoding an actor and another human's words, the standing label never bootstrapped, the removed-type scanner reading prose). Recorded as words; their triage is the next sprint, through github-intake."; }
    verification currencyPassImplementGate : Test { :>> id = "195bd4a1-1c57-48ec-b30a-6493f24b8b86"; :>> title = "Sprint 574 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --lib currency 1 passed (summary shape and exit codes); keel currency . --by githubRecorder twice (6 ingested then 0 ingested / 18 already recorded); keel guard cli-surface-declared . PASS 105; clippy -D warnings clean; the full suite --no-fail-fast 536/0."; }
    verification currencyPassReviewGate : Test { :>> id = "10c74b7c-8bee-4b0d-9dda-c1886d43994f"; :>> title = "Sprint 574 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) DOES THE PASS TRIAGE: no - it records words and says so in its summary line; the workflow commits only .tracking/intake. (2) IS THE SCHEDULE REMOVABLE: one file, no other reference. (3) IS ANYTHING RUNNING ON MY WORD: the workflow's first step reads the acceptance from the tree and exits inert without it."; }
    verification currencyPassCloseOutGate : Test { :>> id = "91426a5d-e5f7-4606-8913-4adfe8cdbbb4"; :>> title = "Sprint 574 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0338 proposed and surfaced; dcScheduledCurrency done; the schedule is armed by the human's acceptance."; }
    verification currencyPassRetroGate : Test { :>> id = "2dd8798a-893d-4462-8f99-01ad0dee24b3"; :>> title = "Sprint 574 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the six statements st092-st097 are untriaged intake for the next sprint (github-intake); GH#55 overlaps d0316's RESOLUTION_ORDER, which probes a path keelw never writes - a finding to carry into that triage. The lesson: an unattended pass is safe exactly to the extent it records and never judges."; }
}
