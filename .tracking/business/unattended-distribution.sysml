// =====================================================================
// UnattendedDistributionNeeds — the project keeps itself current, and its autonomy stops exactly
// where proof stops.
//
// SOURCE. st047 (an issues pull+ingest pipeline; private acts, public plans; can it deploy globally;
// how do other projects pull the skill), st048 (is there a Django-migrations equivalent; antiquated
// data with no indication what process authored it), st049 (auto-update consent gated, IF correctness
// can really be guaranteed), translated as us059, us060 and us061. Elicited direction, not derivation:
// every Need here cites a UserStory that cites an utterance.
//
// CONSOLIDATED, applying the human's own stated principle to their own words: "having a use case for
// typing A, another for typing B, could be consolidated into the need for having a keyboard". Three
// statements spanning issue pulls, library sync, vintage archaeology and update consent produced TWO
// Needs, because they are two principles wearing several surfaces:
//
//   nUnattendedCurrency      <- us059 (pull pipeline, push/pull ergonomics), us060 (know what is
//                               antiquated and under what process it was authored)
//   nAutonomyBoundedByProof  <- us059 (private acts / public plans), us061 (auto-update only where
//                               correctness is guaranteed, consent where it is not)
//
// WHAT IS NOT HERE, deliberately. "See installed versus upstream at a glance" is a real gap I found
// and PROPOSED; the human has not asked for it. It is folded into nUnattendedCurrency rather than
// made its own Need, because a Need with no cited utterance is my judgment (D0216) and inventing one
// to hold my own proposal would dress my idea as their demand.
// =====================================================================
package UnattendedDistributionNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    part nUnattendedCurrency : Need {
        :>> id = "c4e4a2ea-9e23-4805-ad28-60cfd55be20f";
        :>> title = "A project stays current without the human driving each step, and can see where it is behind";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "Inbound reports, shared processes and the engine version reach a project without a person fetching each one by hand, and the project can state where it is behind: which issues are unrecorded, which units are older than the library's, and which engine version it is pinned to versus available. MEASURABLE: an unattended run records every open issue no Statement cites and records none twice; a drift report names every installed unit whose library version is higher; and the corpus-wide question 'what was authored under a process version since superseded' returns an answer rather than timing out. WHAT THIS EXCLUDES: currency is not conformance - being up to date says nothing about whether the project's own work is any good, which is what the gates are for.";
    }

    part nAutonomyBoundedByProof : Need {
        :>> id = "95bc2ab9-8538-4df6-a734-1859cd4bf194";
        :>> title = "Automatic action extends exactly as far as it can be proven safe, and asks beyond that";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "Where an action's outcome can be verified and reversed, the engine takes it unattended. Where it cannot, the engine PLANS it, records the plan, and waits for a human. The boundary is a property of the evidence available, not of how large the change looks. MEASURABLE: an input whose authority cannot be established is routed to a proposed Decision and never to an implementation; an engine update is retained only if the project's own gate passes after it and otherwise leaves the tree byte-identical to before; and every automatic action states which of the two it was before acting. WHAT THIS EXCLUDES: this is not a claim that the automatic actions are CORRECT - only that they are verified-or-reverted. A plan a human accepts can still be a bad plan; that judgment is theirs and this Need does not pretend to cover it.";
    }

    // ── the utterances these came from ───────────────────────────────────────────────────────────
    #DerivedFrom dependency from nUnattendedCurrency to us059;
    #DerivedFrom dependency from nUnattendedCurrency to us060;
    #DerivedFrom dependency from nAutonomyBoundedByProof to us059;
    #DerivedFrom dependency from nAutonomyBoundedByProof to us061;

    // ── UseCases: the concrete usage, where consolidation does NOT apply ─────────────────────────
    //
    // Needs are consolidated; use cases are deliberately not. Each below is a distinct interaction
    // someone actually performs, and three of them are FAILURE paths - because an autonomy boundary
    // that is only ever exercised on the happy path is a claim rather than a boundary.

    part ucScheduledPullBringsWorkIn : UseCase {
        :>> id = "4104bb05-19b8-4d11-9f3d-b37a8de72df0";
        :>> title = "An unattended pull records new issues at the authority they arrived with";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "On a schedule, the project pulls its repository's open issues. Each one no Statement already cites is recorded verbatim, attributed to the reporter's login, carrying its URL and its trust tier. A repeat run records nothing twice and says how many it skipped.";
    }

    part ucPublicIssuePlansOnly : UseCase {
        :>> id = "390fe918-3f1a-4e10-b173-3fd3f0921e10";
        :>> title = "An issue anyone could have filed becomes a proposed Decision, not a change";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "An issue from a public repository is triaged fully - reading is not obeying, and a stranger's report can be entirely correct - and anything that would change the project is written as a proposed Decision for a human to accept. Nothing is implemented from it unattended.";
    }

    part ucEngineUpdateAppliesItself : UseCase {
        :>> id = "788cc9c5-7797-43d1-8dc7-3897bbe64def";
        :>> title = "An engine update applies itself and the project gates green afterwards";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "A newer engine is available. The pre-update commit is recorded, the migration is applied, and the project's own gate runs. It passes, so the update is kept and the human is told what moved - including the pin.";
    }

    part ucEngineUpdateRevertsItself : UseCase {
        :>> id = "b3f8c07e-e649-4006-8120-d5046ba7e062";
        :>> title = "An engine update whose gate goes red reverts itself and says why";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "The same update, but the post-update gate fails - a new guard the project's existing tree violates, which is the issue089/issue090 class. The tree is restored to the recorded commit, the failure is reported with the gate's own output, and the update becomes a decision for the human rather than a broken project.";
    }

    part ucInterruptedMigrationLeavesNothing : UseCase {
        :>> id = "b03cf6a4-560c-49eb-9454-c4068f1f8846";
        :>> title = "A migration killed part-way leaves the tree exactly as it was";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "A migration is interrupted mid-apply - a timeout, a killed process, an evicted runner. Some files are already written. The tree is restored to its pre-migration state rather than left in a state that is neither vintage. Observed live during design: piping migrate's output to `head` closed the pipe, killed the command, and left one file written with the pin unstamped.";
    }

    part ucProjectSeesItsDrift : UseCase {
        :>> id = "b5e31bf7-0c6f-417b-bad8-771737dc8ef3";
        :>> title = "A project is told which of its units are behind the library";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "One command lists each installed unit with its version and the library's, flags the ones behind, and prints the update command. Today this requires checking each unit by hand against `keel library list`.";
    }

    part ucProcessTravelsBothWays : UseCase {
        :>> id = "008b5565-9787-4f8d-86e3-c2bfba629c44";
        :>> title = "A process authored in one project is adopted by another";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "A process written for one project's needs is published to the library and imported by a sibling that lacked it. The receiving project's local adaptations survive an update, and a genuine conflict refuses rather than merging silently.";
    }

    satisfy nUnattendedCurrency by ucScheduledPullBringsWorkIn;
    satisfy nUnattendedCurrency by ucProjectSeesItsDrift;
    satisfy nUnattendedCurrency by ucProcessTravelsBothWays;
    satisfy nAutonomyBoundedByProof by ucPublicIssuePlansOnly;
    satisfy nAutonomyBoundedByProof by ucEngineUpdateAppliesItself;
    satisfy nAutonomyBoundedByProof by ucEngineUpdateRevertsItself;
    satisfy nAutonomyBoundedByProof by ucInterruptedMigrationLeavesNothing;
}
