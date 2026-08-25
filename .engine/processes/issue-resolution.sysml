// =====================================================================
// EngineIssueResolution — every Issue is linked to resolving work and resolved by computation.
//
// Added 2026-06-20 (D0078, IRL-e; implements D0077). Closes the gap where the ready frontier
// read "empty" while issues were open: Issues live outside the action DAG, had no computed
// open/resolved state, and relatedTask pointed at the caused-by task not the resolver. The loop:
// record -> triage (#Resolves to a resolving action OR a mooting Decision) -> resolve (computed)
// -> decision-moots-item (typed edge, not prose). Deployed by the issue-resolution skill (D0059).
// =====================================================================
package EngineIssueResolution {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;
    private import EngineGuardConstraints::*;  // D0139(D): the guards this process brings

    action issueResolution : Process {
        :>> id = "63940516-2c7d-4e8f-c078-516273849700";
        :>> title = "Issue Resolution";
        :>> purpose = "Link every Issue to the work or Decision that resolves it (#Resolves), compute its open/resolved state, and record decision-moots-item as a typed edge — never prose.";
        :>> cadence = Cadence::perItem;
        // APPLIES-WHEN: defects, findings and snags accumulate, and whether one is still open is currently asserted by a human rather than computed from what resolves it
        assert constraint enforcesIssues : issues;
    }

    action irRecord : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c078-516273849701";
        :>> title = "Record the Issue";
        :>> triggerCondition = "A defect, gap, or correction surfaces that reveals a recurrable problem (D0047) — or any tracked Issue is authored.";
        :>> actionText = "Author a `part <name> : Issue { description; discoveredInField; relatedTask }` in .tracking/issues.sysml. relatedTask names the originating/related backlog task (provenance) — NOT necessarily the resolver.";
        :>> producedArtifact = "An Issue item in .tracking/issues.sysml.";
        :>> owner = Owner::ai;
    }
    action irTriage : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c078-516273849702";
        :>> title = "Triage — link to a resolver (#Resolves)";
        :>> triggerCondition = "An Issue exists with no #Resolves edge (untriaged — `keel guard issues` fails).";
        :>> actionText = "Give the Issue a resolver via `#Resolves dependency from <resolver> to <issueNNN>;` (import EngineRelationships::*). The resolver is EITHER a backlog ACTION that will fix it (create the action first if none exists) OR an accepted DECISION that moots/wontfixes it. Never leave an Issue untriaged.";
        :>> producedArtifact = "A #Resolves edge from a resolving action or Decision to the Issue.";
        :>> owner = Owner::ai;
    }
    action irResolve : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c078-516273849703";
        :>> title = "Resolve — computed, never prose";
        :>> triggerCondition = "A resolver completes (its action becomes done, or its Decision is accepted).";
        :>> actionText = "Do NOTHING to 'close' the Issue by hand: resolution is COMPUTED (D0001/D0018). An Issue is RESOLVED iff a #Resolves resolver is complete; otherwise OPEN. Read it with `keel open-issues` (open set + resolvers) and `keel orient` (open_issues). A prose 'RESOLVED' note in the description is informational only, never the source of truth.";
        :>> producedArtifact = "Computed open/resolved state (no stored status); the open frontier surfaced in orient.";
        :>> owner = Owner::ai;
    }
    action irDecisionMoots : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c078-516273849704";
        :>> title = "Decision-moots-item — record the typed edge, not prose";
        :>> triggerCondition = "A Decision makes an item moot / superseded / subsumed (e.g. it resolves-by-design or wontfixes the concern).";
        :>> actionText = "Record the moot relation as a TYPED EDGE in the SAME change as the Decision — never as a 'subsumed by DXXXX' prose string (the issue017 lapse). For an ISSUE: `#Resolves dependency from <decision> to <issueNNN>;` (the issue resolves-by-design). For a Need/Requirement: the existing `supersede` edge (D0004 scope). [Issue-scoped, D0078: backlog-action-moot is out of scope for now.]";
        :>> producedArtifact = "A #Resolves (issue) or supersede (need/requirement) edge from the Decision; the mooted item computes as resolved/superseded.";
        :>> owner = Owner::ai;
    }

    // issueResolution: step order, native succession (D0143 — was `order : Integer`)
    first irRecord then irTriage;
    first irTriage then irResolve;
    first irResolve then irDecisionMoots;
}
