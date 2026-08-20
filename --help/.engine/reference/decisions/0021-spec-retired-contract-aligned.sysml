package Decision0021 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0021 : Decision {
        :>> id = "e2e3f4a5-6b7c-4d8e-9f0a-1b2c3d4e5f60";
        :>> title = "specRefresh + contractAlign: retire 2026-06-04 spec to design-history; remove stale currentState reference from computed-state contract";
        :>> status = DecisionStatus::accepted;
        :>> context = "Two backlog items were ready: (1) specRefresh: the 2026-06-04 design spec was marked historical with a STATUS banner but still lived in docs/superpowers/specs/ alongside the CLAUDE.md reference pointing to it as 'HISTORICAL — superseded in places'; the backlog DoD required formal retirement to design-history. (2) contractAlign: the computed-state contract contained a stale reference to `currentState` (deleted in CR-7); the banner said 'STALE REFERENCE' but the stale text remained in the contract body (line 44).";
        :>> decision = "Retire the 2026-06-04 spec to design-history (git mv docs/superpowers/specs/2026-06-04-process-model-design.md to docs/design-history/2026-06-04-process-model-design-retired.md, update CLAUDE.md to reference the new path with 'retired' language), and align the computed-state contract by updating its STATUS banner and rewriting the line-44 body to say done is computed from appended TestResults (no `currentState`), annotating coverageState() and satisfaction() as explicitly queued.";
        :>> rationale = "The backlog DoD required formal retirement of the historical spec to design-history, and the computed-state contract still carried a stale `currentState` reference (deleted in CR-7) despite a STALE REFERENCE banner. Applied as part of Sprint 5 review phase; both items complete with no doc contradicting the model and no stale references remaining. Accepted: wweatherholtz standing instruction (fix internal processes in each sprint review), 2026-06-15.";
        :>> consequences = "No doc contradicts decisions 0001-0018. The computed-state contract is self-consistent: implemented views labelled implemented, unimplemented views labelled queued, no stale concepts. CLAUDE.md points to the right location for the retired spec.";
    }

    verification d0021Accept : Test { :>> id = "311d8117-f254-4602-a6da-f8c770a59875"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0021 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0021AcceptR1 : TestResult { :>> id = "87c5b7ab-709a-40ad-9b3e-485065c99bb9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
