// =====================================================================
// D0072 — actualHours is KEPT and populated with AI time-spent; AI vs human time is
// distinguished in analysis by the work item's `owner`, not by dropping the field.
// Resolves issue022 (actualHours dormant project-wide since the AI-autonomous transition).
// Accepted 2026-06-18 by wweatherholtz ("keep actual hours, just record time used by AI ...
// differentiate AI vs human time later in analysis").
// =====================================================================
package Decision0072 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0072 : Decision {
        :>> id = "63940516-2c7d-4e8f-a072-516273849650";
        :>> title = "D0072: keep actualHours, populate with AI time-spent; split AI vs human by owner in analysis (resolves issue022)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Audit (issue022) found actualHours recorded by 0/46 sprints: D0038 established an estimate-vs-actual feedback discipline, but when D0049/D0064 made sprints AI-autonomous, wall-clock 'hours' lost obvious meaning for an AI executor and the clause silently went dormant (never retired by a decision). The estimate-vs-actual loop is therefore inert.";
        :>> decision = "KEEP actualHours; populate it with the AI time-spent on the sprint (wall-clock minutes of the AI working the sprint). Do NOT conflate AI and human effort: the work item already carries `owner` (e.g. claudeOpus vs a human actor), so analysis SPLITS or compares AI-time vs human-time by owner — no separate field, no lost signal. estimatedPoints stays the sizing/velocity unit; actualHours becomes the AI-effort actual. Wire 'record actualHours at closeOut' into the sprint-planning / sprint-closeout skills.";
        :>> rationale = "Dropping actualHours would discard a real feedback signal (and silently, which is the issue020 anti-pattern); reviving it with the owner-split keeps one field serving both AI and human effort, deferring the AI-vs-human comparison to analysis-time rather than baking a premature distinction into the schema. ALTERNATIVES: (A) retire actualHours for AI sprints — rejected: loses the estimate/actual loop the user wants kept; (B) add a separate aiHours field — rejected: redundant, since owner already distinguishes the executor (compute-don't-duplicate, D0001/D0005); (C CHOSEN) one actualHours field, split by owner in analysis. CRITERIA: keep-the-signal x minimal-metadata x no-premature-distinction.";
        :>> consequences = "actualHoursDisposition backlog task resolved by this decision; implementation (record actualHours at closeOut; update sprint-planning/sprint-closeout skills) is a small follow-on sprint. Recent sprints (38-44) lack actualHours — NOT backfilled (D0067: never fabricate provenance / recorded time); the discipline applies going forward. Cites D0038's original estimation intent (D0038 itself already superseded on the cap by D0044). issue022 closed by this disposition.";
    }

    verification d0072Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a072-516273849651";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0072 (keep actualHours; populate with AI time; split AI vs human by owner in analysis) via 'keep actual hours, just record time used by AI. we can differentiate time spent by AI vs time spent by humans later in analysis' on 2026-06-18.";
    }
    part d0072AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a072-516273849652";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "909764e";
        :>> judgedAt = "2026-06-19";
        :>> judgedBy = "wweatherholtz";
    }
}
