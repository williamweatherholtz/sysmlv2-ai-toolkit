// ProjectDeliveryS564 — Sprint 564: the standing-consent fork written and surfaced - does D0207 bind to the revision it was granted under, and does it expire. EXECUTE. 2 pts.
package ProjectDeliveryS564 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from consentForkStory to d0324;

    part consentForkStory : Story {
        :>> id = "fda60bae-8937-4de1-9f13-0959f2c890d1";
        :>> title = "Sprint 564: the standing-consent fork written and surfaced - does D0207 bind to the revision it was granted under, and does it expire (2 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS564 {
        action storyConsentFork;
        verification storyConsentForkDoD : Test { :>> id = "0498042b-a14c-42cb-9bfd-cd70c5074a70"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcConsentIsScopedAndExpires (the FORK D0324, proposed for the human; resolves issue342 on the human's answer). DONE: measured - 111 auto-acceptances under D0207, none recording the D0207 commit or the guard count at acceptance; guards 49 at the grant (2026-08-23) to 63 today; guards.rs changed on 13 of the 14 days since - four options costed (A keep and say why, B stamp the revision, C bind and lapse, D expiry), recommendation B with C file-scoped as the follow-on, and what would change it; the research line cites the knowledge record (D0282 landscape, GH#29's commenter, D0207/D0289/D0290/D0308/D0322, stpa-self UCA-R3); judgment-request-quality passes; published on the pending-forks page in the same turn."; }
        // RAN: keel guard judgment-request-quality . PASS; keel validate . clean; no engine source changed in this sprint
        part storyConsentForkDoDR1 : TestResult { :>> id = "249f4b4f-13db-4b0a-8379-d5560a617af9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e04bf076c14114690fc656775c052e77b40ff8e3"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification consentForkRefineGate : Test { :>> id = "21a5fc81-72f0-4eae-b69d-955d5b6bec48"; :>> title = "Sprint 564 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: issue342 and UCA-R3 had named the gap, the landscape spike had the outside evidence, and the DoD listed the four options. The work was the measurement - what the record actually holds about each consent - and the cost of C on the guard set, which the commit history answers."; }
    verification consentForkStandupGate : Test { :>> id = "e1fd436b-986a-4ea6-80d6-7ac620010526"; :>> title = "Sprint 564 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE NUMBER THAT SHAPED THE RECOMMENDATION: C bound to the guard set would have paused autonomy on 13 of the 14 days since the grant. That is why the recommendation is B now and C scoped to the D0207 file, not the guard set."; }
    verification consentForkImplementGate : Test { :>> id = "e403ae79-7123-429d-9bf2-f06fd887ca3c"; :>> title = "Sprint 564 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: grep AUTO-ACCEPTED over .engine/decisions (111); git show of guards.rs at 2026-08-23 (GUARD_NAMES 49); git log --since 2026-08-23 on guards.rs (13 distinct days); keel record decision --from d0324 (fork, stays proposed); keel guard judgment-request-quality . PASS 1 scanned; the page republished at the same URL with D0324 pending and D0312/D0321 kept as answered record."; }
    verification consentForkReviewGate : Test { :>> id = "41aa8bd9-ffc4-4f6a-8b15-942a6a122350"; :>> title = "Sprint 564 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS IT A REAL FORK: yes - it is the human's own pre-commitment and every option except A costs them interruptions in a pattern only they can choose. (2) ARE THE NUMBERS IN THE RECORD: the research line carries them with the commands that produced them. (3) DOES ANYTHING CHANGE BEFORE THEY ANSWER: no."; }
    verification consentForkCloseOutGate : Test { :>> id = "ee2ed9a5-59ad-4652-a683-6f1614677896"; :>> title = "Sprint 564 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0324 proposed and surfaced; dcConsentIsScopedAndExpires's DoD is the proposal and passes."; }
    verification consentForkRetroGate : Test { :>> id = "82162ded-dd9f-4abe-b5d5-c3495b746fa3"; :>> title = "Sprint 564 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the delivery of whichever option the human chooses is chartered by d0324 when accepted; issue342 stays open until then, which is the honest state. The lesson: cost an interruption option against the commit history before recommending it - the number was 13 of 14, not a guess."; }
}
