// =====================================================================
// D0066 — acceptance-events: Decision acceptance is a confirmation TestResult
// (dNNNNAccept) verifying the Decision. ALL accepted decisions get an event so
// tooling reads acceptance UNIFORMLY (no two-format split). The status-only
// decisions are attested now by the human's explicit directive.
// =====================================================================
package Decision0066 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0066 : Decision {
        :>> id = "63940516-2c7d-4e8f-a066-516273849566";
        :>> title = "D0066: Decision acceptance = a confirmation TestResult (dNNNNAccept) verifying the Decision; all accepted decisions migrated (uniform, no grandfather split)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0061 decomposed the attribution model to include acceptance-as-event. Today a Decision's acceptance lives as status=accepted + prose ('Accepted DATE by X'): not first-class. Target: acceptance = a method=confirmation Test + passing TestResult verifying the Decision (naming dNNNNAccept / dNNNNAcceptR1, like gate/DoD results). Of 64+ accepted decisions, ~22 have prose attestation and 26 a recorded createdBy; ~38 have only status=accepted. Grandfathering the status-only ones (event for new, status for old) was rejected because tooling could not then UNIFORMLY identify acceptance/resolution — the coverage view + guard would need two code paths.";
        :>> decision = "Record Decision acceptance as a confirmation event (dNNNNAccept : Test {method=confirmation}, verifying dNNNN by naming; dNNNNAcceptR1 : TestResult {outcome=pass}). Migrate ALL accepted decisions to an event (uniform — tooling reads acceptance from events, no format split). For the ~38 status-only decisions the human (wweatherholtz) attests acceptance NOW via the explicit 'Migrate all; you attest the ~38 now' AskUserQuestion answer (2026-06-17); decisions with recorded prose/createdBy provenance keep that as their historical record. All migration events: judgedBy=wweatherholtz, judgedAt=2026-06-17 (the attestation date), judgedAgainst=HEAD. NEW decisions carry an acceptance event at authoring.";
        :>> rationale = "Uniform event-based acceptance lets the coverage view + guard identify resolution with ONE mechanism (the grandfather split would defeat that). This is a representation refactor of an existing fact (status=accepted) PLUS an explicit human attestation for the status-only ones — given now, not inferred (D0016/D0051 forbid INVENTING acceptance, not re-recording it on an explicit directive, cf. the rust_sprint1-6 waiver D0062). ALTERNATIVES: (A) prospective + grandfather — rejected: two-format split, tooling can't uniformly identify resolution; (B) migrate only the ~26 provenance-backed — rejected: same split; (C CHOSEN) migrate all, human attests the status-only now. CRITERIA: tooling-uniformity x attestation-honesty (explicit not inferred) x refactor-not-reinvent.";
        :>> consequences = "migrate_acceptance_events.py appends dNNNNAccept + dNNNNAcceptR1 to every accepted decision (idempotent; adds EngineElement+EngineVerification imports). This Decision dogfoods it (d0066Accept below). The Decision-authoring convention (sysmlv2-syntax-notes) gains the acceptance-event pattern for new decisions. attrModelAcceptanceEventsDoD re-scoped to migrate-all. The events feed attrModelCoverageView (attestation coverage) and attrModelGuard (acceptance is an event, not prose). status=accepted remains the structured fact; the event carries who/when/commit.";
    }

    // Acceptance event for D0066 (dogfood). Linked to d0066 by the dNNNNAccept naming
    // convention. Attested by the explicit 'Migrate all; you attest the ~38 now'
    // AskUserQuestion answer (2026-06-17).
    verification d0066Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a066-516273849567";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of decision d0066 (acceptance-events model), via the explicit AskUserQuestion answer on 2026-06-17.";
    }
    part d0066AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a066-516273849568";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "e146621";
        :>> judgedAt = "2026-06-17";
        :>> judgedBy = "wweatherholtz";
    }
}
