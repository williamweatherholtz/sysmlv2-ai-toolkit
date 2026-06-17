// =====================================================================
// D0047 — corrections-become-guards: a defect/correction found mid-work is
// logged as an Issue AND given a permanent automated guard (not patched
// silently); plus the first such guard — the ceremony-gate ordering check.
// Accepted 2026-06-16 by wweatherholtz (directive: "corrections … should be
// logged as a bug and the processes should be improved permanently").
// =====================================================================
package Decision0047 {
    private import EngineWork::*;

    part d0047 : Decision {
        :>> id = "d0ebfc01-2d3e-4f40-156f-91021324a5b6";
        :>> title = "D0047: corrections become permanent guards (logged Issue + automated check); ceremony-gate ordering guard added";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "During Sprint 16 a ceremony gate (Standup) was skipped — gates were recorded Refine->Implement->Review out of order. orient surfaced it (pending=Standup while later gates passed), but the AI initially just recorded the missing gate and moved on: symptom patched, no bug logged, no recurrence guard. The user identified this as the SAME failure mode as the Sprint 14 retro finding (tracked-but-still-recurring, resolved by automated guards in D0043) and directed: 'corrections like the one you just identified should be logged as a bug and the processes should be improved permanently.' We had VIEWS that surface ceremony subversion (orient pending, Sprint-15 traceability orange highlight) but no GUARD that prevents it at author/commit time.";
        :>> decisionText = "STANDING RULE: a defect or correction discovered mid-work that reveals a recurrable process gap MUST (1) be logged as a tracked Issue in .tracking/issues.sysml with relatedTask routing, and (2) receive a permanent automated guard (validator / pre-commit check / lint) — never be patched silently. Trivial one-off edits (typos, wording) are exempt; the test is 'could this class recur?'. FIRST GUARD (this Decision): .engine/tools/validate/validate_ceremony.py enforces the invariant 'within a delivery file, no ceremony gate may have a passing TestResult while an earlier-ordered gate defined in the same file is unpassed.' Pure-Python (no kernel); wired into the pre-commit hook on .tracking/delivery/ changes; hard-fails (exit 1) on new violations; historical anomalies grandfathered as WARN (sprint11_nativeSpikes), mirroring validate_actors LEGACY_ACTORS. Logged as issue010, addressed by backlog item ceremonyGateGuard. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "A skipped or out-of-order ceremony gate now aborts the commit instead of relying on someone running orient. Future corrections follow the rule: Issue + guard. validate_ceremony.py joins the validator suite; the recurring-task-skill rule (D0040) and the retro dispatch (sprint-retro) gain a sibling discipline for defects. New delivery sprints must record gates in canonical order (Refine->Standup->Implement->Review->CloseOut->Retro); a legitimate in-progress prefix is allowed, a gap is not.";
    }
}
