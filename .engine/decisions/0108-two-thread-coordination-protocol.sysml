// =====================================================================
// D0108 (PROPOSED — NOT YET ACCEPTED) — two-thread coordination protocol. Resolves issue056 C12: when
// more than one AI thread edits the same keel model concurrently (e.g. this analysis thread + the ignite
// thread that owns issue054's resolver), there was no recorded rule for who may change what or how
// divergence is adjudicated. Drafted for the human's acceptance (governance). status=proposed.
// =====================================================================
package Decision0108 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0108 : Decision {
        :>> id = "08a1b0c1-4108-42a3-ab5c-516273849708";
        :>> title = "D0108 (PROPOSED): two-thread coordination protocol — additive-only, owner-of-record, human adjudicates divergence";
        :>> createdAt = "2026-07-03";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::proposed;
        :>> context = "issue056 C12 (from the independent D0105 critique): two AI threads now co-edit the engine model with no recorded coordination protocol. This session hit it concretely — the analysis thread and the ignite thread both touched issue054/D0105 work; the analysis thread kept its writes purely ADDITIVE (appended issue055/056/058, D0105/D0107, never edited the ignite thread's issue054) to avoid a merge hazard, but that was convention-by-vigilance, not a recorded rule. As multi-agent use grows this needs a stated protocol.";
        :>> decision = "Adopt a lightweight two-thread (multi-thread) coordination protocol: (1) OWNER-OF-RECORD — each tracked item (Issue/Decision/backlog action/sprint) is owned by the thread that created it (its createdBy); only the owner edits its fields. (2) ADDITIVE ACROSS THREADS — a non-owner thread may only ADD new items + typed edges that REFERENCE another thread's item (e.g. a #DependsOn/#Resolves/supersede edge), never edit that item's fields in place. (3) SUPERSEDE, DON'T OVERWRITE — to change another thread's Decision, author a NEW superseding Decision (D0070), don't edit theirs. (4) SHARED FILES — append-only where possible (issues.sysml, backlog.sysml actions); if two threads must edit the same file region, the second rebases on the first (git), never force-overwrites. (5) DIVERGENCE — when two threads reach conflicting conclusions on the same subject (e.g. issue054's resolver vs D0105's reframing), the conflict is recorded as an Issue and the HUMAN adjudicates; neither thread silently wins. (6) COMMIT HYGIENE — main-only, each thread pushes its own commits; a thread checks `git fetch` state before a shared-file edit.";
        :>> rationale = "ALTERNATIVES: (i) no protocol (status quo) — REJECTED (relies on vigilance; the near-miss this session shows it will eventually corrupt shared state). (ii) hard file-locking / one-writer — REJECTED (kills the parallelism that multi-thread work exists for; heavy). (iii) CHOSEN — additive-only + owner-of-record + supersede-not-overwrite + human-adjudicated divergence: matches the engine's existing grain (typed edges, append-only results, supersede-don't-mutate, human-gated decisions), so it needs no new mechanism — just a stated rule. CRITERIA: preserves-parallelism x reuses-existing-grain (edges/supersede/append) x no-silent-overwrite x human-owns-conflict.";
        :>> consequences = "UPON ACCEPTANCE: add the protocol to CLAUDE.md (a multi-thread section) + the engine-triage skill (a check before editing a non-owned item: add/reference/supersede, don't overwrite). Resolves issue056 C12 (with C11 already dispositioned by the D0105 acceptance). No schema change (owner = existing createdBy; references = existing typed edges). doc-sync on acceptance. Acceptance verdict/provenance will be the discrete d0108Accept TestResult (not prose, D0058/issue058).";
    }
}
