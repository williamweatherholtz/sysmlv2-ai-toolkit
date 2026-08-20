// =====================================================================
// D0045 — drop-cursor-computed-orient: retire the StateCursor hand-maintained
// pointer; orient computes sprint ceremony status from delivery file TestResults.
// Accepted 2026-06-16 by wweatherholtz.
// =====================================================================
package Decision0045 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0045 : Decision {
        :>> id = "7d8e9f0a-1b2c-4d3e-af4f-5a6b7c8d9e0f";
        :>> title = "D0045: drop StateCursor; orient computes sprint ceremony status from delivery TestResults";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The state cursor (.tracking/state.sysml, CR-6, D0009) was a hand-maintained pointer to the active workflow phase. It violated the core principle that authored facts are irreducible and computation is a view: phase position IS computable from which ceremony gate TestResults exist in delivery files. The cursor drifted, required manual updates each gate transition, and added noise to orient output without adding information not already in the data. User feedback 2026-06-16: 'i don't need or want a cursor. Next steps need to fall out of outstanding items directly.'";
        :>> decision = "Retire the hand-maintained StateCursor: remove its part def from schema/core/workflow.sysml and tombstone .tracking/state.sysml (empty package). Extend orient to compute sprint ceremony status — for each delivery file, detect which of the 6 ceremony gates (Refine/Standup/Implement/Review/CloseOut/Retro) have a passing TestResult and surface in-progress sprints with gate status; orient output replaces the 'cursor' key with 'in_progress_sprints'; report.py tabs updated to use the computed status.";
        :>> rationale = "The state cursor (.tracking/state.sysml, CR-6, D0009) was a hand-maintained pointer to the active workflow phase that violated the core principle that authored facts are irreducible and computation is a view: phase position IS computable from which ceremony gate TestResults exist in delivery files. The cursor drifted, required manual updates each gate transition, and added noise to orient output without adding information not already in the data. User feedback 2026-06-16: 'i don't need or want a cursor. Next steps need to fall out of outstanding items directly.' state.sysml is retained as an empty-package tombstone so existing references in git history stay traceable.";
        :>> consequences = "No more manual cursor updates on phase transitions. orient.in_progress_sprints replaces orient.cursor. report.py workflow tab highlights active phase from computed in-progress data. validate_schema must pass without StateCursor. state.sysml becomes a tombstone (retained as empty package so existing references in git history are traceable).";
    }

    verification d0045Accept : Test { :>> id = "272bfa7d-2569-411f-8cd8-4314cee755e9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0045 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0045AcceptR1 : TestResult { :>> id = "3413434e-bc6a-4a78-b0ce-e3790751cef2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
