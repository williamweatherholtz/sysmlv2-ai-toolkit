// =====================================================================
// D0047 — corrections-become-guards: a defect/correction found mid-work is
// logged as an Issue AND given a permanent automated guard (not patched
// silently); plus the first such guard — the ceremony-gate ordering check.
// Accepted 2026-06-16 by wweatherholtz (directive: "corrections … should be
// logged as a bug and the processes should be improved permanently").
// =====================================================================
package Decision0047 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0047 : Decision {
        :>> id = "d0ebfc01-2d3e-4f40-156f-91021324a5b6";
        :>> title = "D0047: corrections become permanent guards (logged Issue + automated check); ceremony-gate ordering guard added";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "During Sprint 16 a ceremony gate (Standup) was skipped — gates were recorded Refine->Implement->Review out of order. orient surfaced it (pending=Standup while later gates passed), but the AI initially just recorded the missing gate and moved on: symptom patched, no bug logged, no recurrence guard. The user identified this as the SAME failure mode as the Sprint 14 retro finding (tracked-but-still-recurring, resolved by automated guards in D0043) and directed: 'corrections like the one you just identified should be logged as a bug and the processes should be improved permanently.' We had VIEWS that surface ceremony subversion (orient pending, Sprint-15 traceability orange highlight) but no GUARD that prevents it at author/commit time.";
        :>> decision = "STANDING RULE: a defect or correction discovered mid-work that reveals a recurrable process gap MUST (1) be logged as a tracked Issue in .tracking/issues.sysml with relatedTask routing, and (2) receive a permanent automated guard (validator / pre-commit check / lint) — never be patched silently (trivial one-off typos/wording exempt; the test is 'could this class recur?'). FIRST GUARD: validate_ceremony.py enforcing no ceremony gate may pass while an earlier-ordered gate in the same file is unpassed.";
        :>> rationale = "We had VIEWS that surface ceremony subversion (orient pending, Sprint-15 traceability orange highlight) but no GUARD that prevents it at author/commit time, and the same failure mode recurred (Sprint 14 retro to Sprint 16) — manual vigilance is not a control. validate_ceremony.py is pure-Python (no kernel), wired into the pre-commit hook on .tracking/delivery/ changes, hard-fails (exit 1) on new violations, with historical anomalies grandfathered as WARN (sprint11_nativeSpikes), mirroring validate_actors LEGACY_ACTORS. Logged as issue010, addressed by backlog item ceremonyGateGuard.";
        :>> consequences = "A skipped or out-of-order ceremony gate now aborts the commit instead of relying on someone running orient. Future corrections follow the rule: Issue + guard. validate_ceremony.py joins the validator suite; the recurring-task-skill rule (D0040) and the retro dispatch (sprint-retro) gain a sibling discipline for defects. New delivery sprints must record gates in canonical order (Refine->Standup->Implement->Review->CloseOut->Retro); a legitimate in-progress prefix is allowed, a gap is not.";
    }

    verification d0047Accept : Test { :>> id = "be4120b2-3bc0-4741-8806-36805c0f3b2c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0047 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0047AcceptR1 : TestResult { :>> id = "1d72e790-2253-4e93-8439-f1564b096ae3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
