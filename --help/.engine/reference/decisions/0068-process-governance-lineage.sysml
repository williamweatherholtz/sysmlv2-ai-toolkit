// =====================================================================
// D0068 — process-governance-lineage: govern items by an explicit charter→work
// lineage edge + a computed governing-version, NOT per-artifact process stamps.
// Resolves issue002. Decomposes the work into 3 ordered increments.
// =====================================================================
package Decision0068 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0068 : Decision {
        :>> id = "63940516-2c7d-4e8f-a068-516273849580";
        :>> title = "D0068: process governance by charter-lineage (charter edge + computed governing-version, charter-time freeze); decomposed into 3 increments";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue002: the engine has no machine-readable link from an item to the process that governed it, so when a process evolves (e.g. D0049 changed closeOut confirmation->inspect) 'which rules apply to old items' is undecidable from the model — the rust_sprint1-6 'two viable paths' smell. Research (PROV-O hadPlan-on-Activity + non-transitive wasDerivedFrom; PLM effectivity; MIL-STD-961/NASA applicable-docs flow-down; DO-178C change-impact; bitemporal as-of; CMMI/ISO prospective process improvement) converged on: a charter carries process IDENTITY (stable), the VERSION is resolved by an as-of query against a temporal axis, and re-processing old work is criticality-weighted + prospective (mandatory only for safety).";
        :>> decision = "Govern items by an explicit charter→work LINEAGE edge + a COMPUTED governing-version — never per-artifact process stamps (which drift + split tooling). AUTHOR two stable things: (1) a typed charteredBy edge (work item -> originating Need/Requirement/Decision); (2) a governing-process reference on the charter + process-change Decisions carrying an effective commit + a retroactivity class {prospective|advisory|mandatory-safety}; plus reuse criticality (safetyLevel/WorkPriority). COMPUTE everything else: governing-process-of(X) = traverse lineage to charter, resolve the process supersede-chain AS-OF the charter's immutable creation commit (CHARTER-TIME FREEZE — an in-flight item is immune to mid-flight process change); generalized suspect carrying reason {criterion-drift|deliverable-drift|process-version-drift} + severity; reprocess-candidates = (items under a superseded version) x downstream fan-out x criticality, mandatory iff retroactivity-class=safety else advisory. Escape-hatch: an explicit per-edge effectivity override for reuse-across-charters / partial-supersession. Reuse `supersede` for process versions (a version = the head of its governing-Decision chain) — no new versioning primitive.";
        :>> rationale = "An explicit charter edge makes 'which process governed this' a queryable fact, not a guessed fingerprint (the research's core lesson: the governing-version reference must be recorded + queryable, and is the user-authored dimension that can't be inferred). Charter-time freeze (vs per-event re-resolution) is what all four traditions do — past/in-flight work stays bound to the baseline it was produced under; changes apply to the next charter. ALTERNATIVES: (A) per-artifact process-version stamp — rejected: redundant, drifts, splits tooling, and a scalar can't describe a durative item; (B) grandfather-by-name-set forever — rejected: doesn't scale, no lineage; (C CHOSEN) charter edge + computed version + retroactivity class. CRITERIA: queryable-not-inferred x charter-time-freeze-correctness x criticality-weighted-prospective (safety = the only forced retroactivity, mirroring D0050).";
        :>> consequences = "Decomposed into 3 ordered increments, each its own sprint/DoD: (1) pglCharterEdge — add the typed charteredBy edge (schema), demonstrate >=1 work->charter link, validate green (resolves the core of issue002). (2) pglProcessRef — charter->process reference + retroactivity class on process-change Decisions + supersede-chain authored/walkable; governing-version resolvable as-of the charter commit. (3) pglViews — computed governing-process-of(X), generalized suspect (reason+severity), reprocess-candidates (superseded x fan-out x criticality), declared as viewpoints (D0056/D0057). Pairs with D0050 (suspicion) + D0062 (prospective doctrine). Pre-authorized breaking-schema changes for the increments; first build = pglCharterEdge.";
    }

    verification d0068Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a068-516273849581";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of decision d0068 (charter-lineage governance design + 3-increment decomposition; sprint Increment 1 first), via the explicit AskUserQuestion answer on 2026-06-17.";
    }
    part d0068AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a068-516273849582";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "10988b0";
        :>> judgedAt = "2026-06-17";
        :>> judgedBy = "wweatherholtz";
    }
}
