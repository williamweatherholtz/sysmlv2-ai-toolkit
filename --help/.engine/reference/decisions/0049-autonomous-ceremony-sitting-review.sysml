// =====================================================================
// D0049 — autonomous-ceremony-sitting-review: per-sprint closeOut + retro run
// autonomously (AI, no human gate); the human confirmation moves to a per-SITTING
// sprint review. A retro skill autonomously identifies AVOIDABLE issues from the
// previous sprint and creates tracked items.
// Accepted 2026-06-16 by wweatherholtz (directive: "I don't need to confirm the
// end of a sprint ... sitting retro doesn't really need my input ... make a skill
// to identify avoidable issues for the previous sprint, and make items").
// =====================================================================
package Decision0049 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // D0070: this Decision changed the Delivery ceremony (prospective; D0062 default). Grandfathered for resolution (pre-guard).
    #ProspectiveChange part d0049 : Decision {
        :>> id = "012d3e4f-5061-4237-376f-1324a5b6c7de";
        :>> title = "D0049: autonomous per-sprint closeOut+retro; human confirms per-sitting review";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The per-sprint ceremony (D0039) made closeOut AND retro method=confirmation, each needing explicit human sign-off (D0016). In a multi-sprint sitting that means many confirmations for routine closures the human does not need to gate. User directive: 'I don't need to confirm the end of a sprint. after a sitting, we do a sprint review and I confirm content. sitting retro doesn't really need my input. just make a skill to identify avoidable issues for the previous sprint, and make items for those changes.'";
        :>> decision = "Defines SITTING = one continuous work session containing one or more sprints. NEW ceremony model: (1) per-sprint gates refine/standup/implement/review/closeOut/retro all run AUTONOMOUSLY (AI-recorded; methods inspect/test/analysis, NOT confirmation) — closeOut auto-records once the sprint DoD passes and is no longer a human gate; (2) retro is autonomous — the sprint-retro skill identifies AVOIDABLE issues and creates tracked Issue + backlog items, then records the retro gate (method=analysis, AI-judged); (3) the single HUMAN touchpoint is a per-SITTING sprint review (method=confirmation, batchable across the sitting's sprints).";
        :>> rationale = "The per-sprint ceremony (D0039) made closeOut AND retro method=confirmation, so a multi-sprint sitting meant many confirmations for routine closures the human does not need to gate. Supersedes the per-sprint confirmation gates of D0039 (closeOut/retro confirmation); D0016 still holds for the confirmations that remain (the per-sitting review): never inferred, always explicit. User directive: 'I don't need to confirm the end of a sprint. after a sitting, we do a sprint review and I confirm content. sitting retro doesn't really need my input. just make a skill to identify avoidable issues for the previous sprint, and make items for those changes.'";
        :>> consequences = "Sprints close end-to-end without per-sprint human sign-off; the human confirms once per sitting at the sprint review. sprint-closeout skill → autonomous (method=inspect). sprint-retro skill → autonomous avoidable-issue identifier + item creator (method=analysis). sprint-review skill → per-sitting human-confirmation of content (keeps the Phase-3 transcript scan). Delivery files: closeOut gate method=inspect, retro gate method=analysis, both AI-judged; a per-sitting review confirmation is recorded when the human reviews. CLAUDE.md confirmation-discipline + ceremony sections updated. The ceremony-ordering guard (D0047) is unchanged (it checks order, not method).";
    }

    verification d0049Accept : Test { :>> id = "4d3c2bf9-9410-4f5b-ac7c-99cb060f5a9c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0049 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0049AcceptR1 : TestResult { :>> id = "e2be53b6-24dd-4a82-bcdc-bbf10179c371"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
