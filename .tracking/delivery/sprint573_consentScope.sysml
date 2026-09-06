// ProjectDeliveryS573 — Sprint 573: the human's D0324 answer delivered - standing consent is scoped to the processes it was promulgated under, and the rule bit its own Decision first. EXECUTE. 2 pts.
package ProjectDeliveryS573 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from consentScopeStory to d0337;

    part consentScopeStory : Story {
        :>> id = "59a321cb-047c-4d78-916b-538604ca035d";
        :>> title = "Sprint 573: the human's D0324 answer delivered - standing consent is scoped to the processes it was promulgated under, and the rule bit its own Decision first (2 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS573 {
        action storyConsentScope;
        verification storyConsentScopeDoD : Test { :>> id = "16ea7e39-4f4d-425c-bce6-da43387e7dc7"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcConsentIsScopedAndExpires completes on the human's answer (D0324 accepted on their verbatim words 'd0324 - standing consent is scoped only to the existing processes under which it was promulgated'; D0337 proposed, delivering it). DONE: at record time a Decision carrying a process-change or safety-change marker is OUTSIDE standing consent - recorded proposed, the reason printed, accepted only by the human; unmarked Decisions auto-accept as before. D0337 itself was recorded proposed by that rule. Arming: disguised_fork_stays_proposed.rs gains the marker case (proposed, no AcceptR1, the OUTSIDE message); the unmarked case still auto-accepts. CLAUDE.md §4 states the scope. The page carries D0337 for the human's acceptance and states my reading so it can be corrected."; }
        // RAN: cargo test --test disguised_fork_stays_proposed 3 passed; the full suite 534/0 ran before the record-path change - the change is exercised by that test binary; clippy clean
        part storyConsentScopeDoDR1 : TestResult { :>> id = "27f36218-c968-4f7f-abed-62cef37b62d1"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9a726d2f2ee58a2a9faf64f518871bdca4ca41c0"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification consentScopeRefineGate : Test { :>> id = "bb6ffb33-4c1f-44d3-993a-a6a8dcd7f92f"; :>> title = "Sprint 573 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY the moment the human spoke. Their answer was none of the four options - a scope, not a stamp, lapse or expiry - and the design question was the enforceable edge of 'the existing processes': the keystone marker, which already declares every change to the process and enforcement surface, needs no new fact and matches the edge a reader would use."; }
    verification consentScopeStandupGate : Test { :>> id = "1c629408-7403-45fd-a423-b144aee2d705"; :>> title = "Sprint 573 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THE RULE NARROWS THIS SESSION'S OWN AUTONOMY: every guard, hook, process, workflow and contract Decision now waits for the human's word - roughly one per sprint at today's rate. That is what they asked for, and D0337 is the first Decision to wait."; }
    verification consentScopeImplementGate : Test { :>> id = "5a4aac06-9f56-406f-b184-c673c5ffd1b3"; :>> title = "Sprint 573 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: keel accept d0324 on the verbatim quote (read-back passed: the quote names d0324); keel record decision --from d0337 printed OUTSIDE standing consent (D0337) and left it proposed; cargo test --test disguised_fork_stays_proposed 3 passed; clippy -D warnings clean after folding cmd_record's dispatch; the page republished at the same URL."; }
    verification consentScopeReviewGate : Test { :>> id = "73e66b05-89f5-451b-8e12-1d1876d1b330"; :>> title = "Sprint 573 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS THE READING THE HUMAN'S: stated in D0337 and on the page with the two other readings named; they can say otherwise in a word. (2) IS ANYTHING ACCEPTED THAT SHOULD NOT BE: D0337 is proposed; D0324's acceptance is the human's own sentence. (3) IS THE OLD RECORD RE-JUDGED: no - forward-only, D0129."; }
    verification consentScopeCloseOutGate : Test { :>> id = "0c7fb371-9f55-4ba6-9f07-d409b51faef5"; :>> title = "Sprint 573 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0324 accepted on the human's words; D0337 proposed and awaiting them; dcConsentIsScopedAndExpires done."; }
    verification consentScopeRetroGate : Test { :>> id = "285545ca-11b4-4643-a0cd-5145e5067974"; :>> title = "Sprint 573 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: d0337 is the open acceptance on the page; the marker Decisions this session recorded under consent before the rule stand as recorded (D0129). The lesson: when the human answers a fork in their own words, record the words, deliver the reading, and state the reading where they can correct it."; }
}
