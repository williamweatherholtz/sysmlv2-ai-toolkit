// =====================================================================
// D0069 — process-governance resolution mechanism (refines D0068). Resolution is by
// GUARDED process-change recording + a git-traversal resolver — NOT per-item version
// stamps and NOT a separate content-hash. Captures the output-first design dialogue.
//
// SUPERSEDED by D0070 (2026-06-18): D0069's edge-CARRIER sub-mechanism (a #Changes
// dependency edge from the Decision to the Process, which implementation forced into a
// floating .tracking file — issue021) is replaced by a @ProspectiveChange/@SafetyChange
// MEMBER MARKER on the Decision itself + git-derived linkage under a hard-lock guard.
// D0069's surviving architecture (guarded recording + git-traversal resolver; no per-item
// stamps; no content-hash; work->process by convention) is RESTATED in D0070. status flipped
// to superseded (the formal record); D0070 names this Decision.
// =====================================================================
package Decision0069 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0069 : Decision {
        :>> id = "63940516-2c7d-4e8f-a069-516273849590";
        :>> title = "D0069: process governance resolved by guarded process-change recording + git-traversal (no per-item stamps, no content-hash); refines D0068, reshapes Inc 2/3";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::superseded;
        :>> context = "D0068 set the charter-lineage direction but left the concrete mechanism underspecified (the 'two options' weren't separable; 'charter commit' undefined). Worked through output-first. The three desired OUTPUTS: (1) know when a process output should be ACCEPTED even if it doesn't fit the CURRENT process (i.e. it satisfied the process in force WHEN it was chartered); (2) know HOW an output was generated, to audit / regenerate / push back on it; (3) MINIMAL authored metadata describing the state of the data — everything else computed. The keystone realization (user): resolution is only possible if process changes are RECORDED so they can be referred to — 'that's unavoidable.'";
        :>> decision = "Govern by GUARDED process-change recording + a git-traversal RESOLVER; reject per-item process-version stamps and a separate process content-hash. AUTHORED (minimal): (a) the #CharteredBy work->origin edge (Inc 1, done), SET AT WORK-PREP by sprint-planning/DoR; (b) on a process-change Decision ONLY: a #Changes link to the Process it changes + a retroactivity class {prospective | safety}; the effective commit = that Decision's acceptance-event judgedAgainst (no new field). work->process is by CONVENTION/kind (a sprint Story is governed by Delivery; a migration by the migration process) — NOT a per-item field. KEYSTONE GUARD (D0047): a change to a process definition MUST carry a recorded #Changes-linked Decision, or it fails — this makes recording unavoidable, which is what makes the governing-version reliably COMPUTABLE (so no per-item stamp is needed). Second guard: a work item (Story) with no #CharteredBy edge is flagged (charter-authoring enforced, not hoped). COMPUTED (git-traversal resolver, reusing the suspicion primitive _is_ancestor + git show): governing-version(item) = the latest #Changes Decision for the item's process whose effective commit is a git-ancestor of the item's commit; process-as-it-was = git show <that commit>:<process def> (audit/regenerate); valid-then? = the item's recorded evidence satisfies THAT version's requirements, not today's (a sprint's own gates already encode the ceremony it followed); reprocess-candidates = items under a superseded version x downstream fan-out x criticality, mandatory iff retroactivity=safety else advisory.";
        :>> rationale = "Recording process changes is both UNAVOIDABLE and SUFFICIENT: given recorded, committed, referable changes, git already gives as-of resolution (ancestry) + the process-as-it-was (show) for free — so the version need not be stamped per item. Per-item stamps are the workaround for UNreliable recording; eliminating that unreliability with a guard (rather than papering over it with stamps) is what honors output #3 (minimal metadata). ALTERNATIVES: (A) stamp each work item with its process version — rejected: redundant metadata floating around (against D0001/D0005 compute-don't-store + the minimalism the user asked for); a scalar also can't describe a durative item. (B) give each process a content-hash separate from its id — rejected: git already content-addresses every committed state (D0013 git-only temporal model); a parallel hash duplicates git and adds metadata. (C CHOSEN) record changes rigorously (guarded) + compute the version by git traversal; work->process by convention. CRITERIA: minimal-authored (output 3) x reliable-resolution (the guard guarantees the data exists to compute from) x audit/regenerate-for-free (git show) x faithful-to-prospective-governance (D0062: accept what satisfied the then-process).";
        :>> consequences = "Refines D0068's decomposition. Inc 2 (pglProcessRef) = the AUTHORED + ENFORCEMENT layer: #Changes Decision->Process link + retroactivity class on process-change Decisions; the keystone guard (process-def change must carry a #Changes Decision); the charter guard (Story must have #CharteredBy); charter-authoring folded into sprint-planning/DoR. Inc 3 (pglViews) = the git-traversal RESOLVER (pure computation): governing-version, valid-then?, reprocess-candidates — reusing query.py _is_ancestor + git show; declared as viewpoints (D0056/D0057). Serves all three outputs with no per-item stamp and no content-hash. Pairs with D0050 (suspicion = the same git-ancestry machinery), D0062 (prospective doctrine), D0047 (corrections->guards). This Decision is itself resolvable by the very mechanism it defines.";
    }

    verification d0069Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a069-516273849591";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of decision d0069 (guarded-recording + git-traversal resolution mechanism; no per-item stamps / no content-hash; reshaped Inc 2/3), via the explicit design dialogue + 'yes i agree on all counts' + 'yes capture all inputs here' on 2026-06-17.";
    }
    part d0069AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a069-516273849592";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "26cf539";
        :>> judgedAt = "2026-06-17";
        :>> judgedBy = "wweatherholtz";
    }
}
