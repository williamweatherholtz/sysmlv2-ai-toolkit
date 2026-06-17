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
    private import EngineWork::*;

    part d0049 : Decision {
        :>> id = "012d3e4f-5061-4237-376f-1324a5b6c7de";
        :>> title = "D0049: autonomous per-sprint closeOut+retro; human confirms per-sitting review";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The per-sprint ceremony (D0039) made closeOut AND retro method=confirmation, each needing explicit human sign-off (D0016). In a multi-sprint sitting that means many confirmations for routine closures the human does not need to gate. User directive: 'I don't need to confirm the end of a sprint. after a sitting, we do a sprint review and I confirm content. sitting retro doesn't really need my input. just make a skill to identify avoidable issues for the previous sprint, and make items for those changes.'";
        :>> decisionText = "Defines SITTING = one continuous work session, containing one or more sprints. NEW ceremony model: (1) Per-sprint gates refine/standup/implement/review/closeOut/retro all run AUTONOMOUSLY (AI-recorded; methods inspect/test/analysis — NOT confirmation). closeOut auto-records once the sprint DoD passes; it is no longer a human gate. (2) retro is autonomous: the sprint-retro skill identifies AVOIDABLE issues from the just-finished sprint (things a guard/skill/doc could have prevented) and creates tracked Issue + backlog items for the changes, then records the retro gate (method=analysis, judgedBy AI) — no human input. (3) The single HUMAN touchpoint is a per-SITTING sprint review (method=confirmation): after a sitting the human reviews the sitting's content and confirms (batchable across the sitting's sprints, D0019). This is acceptance of the sitting, not a per-sprint blocker. Supersedes the per-sprint confirmation gates of D0039 (closeOut/retro confirmation). D0016 still holds for the confirmations that remain (the per-sitting review): never inferred, always explicit. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "Sprints close end-to-end without per-sprint human sign-off; the human confirms once per sitting at the sprint review. sprint-closeout skill → autonomous (method=inspect). sprint-retro skill → autonomous avoidable-issue identifier + item creator (method=analysis). sprint-review skill → per-sitting human-confirmation of content (keeps the Phase-3 transcript scan). Delivery files: closeOut gate method=inspect, retro gate method=analysis, both AI-judged; a per-sitting review confirmation is recorded when the human reviews. CLAUDE.md confirmation-discipline + ceremony sections updated. The ceremony-ordering guard (D0047) is unchanged (it checks order, not method).";
    }
}
