// D0127 (PROPOSED — NOT YET ACCEPTED) — Relationship grammar as a COMPUTED view (observed triples) + scope-aware creation
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0127Accept event on sign-off.
package Decision0127 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0127 : Decision {
        :>> id = "fdb97336-e5fd-446d-811b-415b700eccdf";
        :>> title = "Relationship grammar as a COMPUTED view (observed triples) + scope-aware creation";
        :>> createdAt = "2026-07-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human (2026-07-17) wants scope/process-aware creation: when a scope is filtered (e.g. safety artifacts), the viewer should offer only in-scope creations + the items they directly interface with upstream/downstream (a safety scope offers Loss Scenarios, up-Losses, down-Requirements — never a bare Need). This needs a relationship grammar: which (sourceType, edge, targetType) triples are valid. Two ways to source it: (a) AUTHOR an explicit metamodel of allowed triples per domain, or (b) COMPUTE it from the edges that already exist in the model. Option (a) is heavy, domain-specific, and freezes schema/core; option (b) is generative + zero-authoring + fits the engine invariant 'text is truth; everything derivable is a view' (CLAUDE.md §2.1). keel's edge algebra is a CLOSED set + there is already an EdgeRule/ElementRule constraint layer.";
        :>> decision = "The relationship grammar is a COMPUTED VIEW (keel view::grammar_json / GET /api/grammar): the distinct (sourceType, edgeKind, targetType) triples OBSERVED in the model, plus a per-type up/down summary. It is NOT authored metamodel and does NOT modify schema/core (which stays frozen). Scope-aware creation (srViewerScopedAuthoring) reads this grammar: given the active scope type T, the viewer offers creation of T + the types T interfaces with (grammar up/down of T) and constrains Connect's edge kinds to those valid for the (from,to) type pair. The deliverable's domain vocabulary (Loss/Hazard/…) stays in the DOWNSTREAM project's schema (§1); the grammar is read per-connected-project, so this is generic across domains, not hardcoded to safety. Because it is observational, the grammar offers patterns that already occur; new never-used pairings are reachable via the full closed algebra (Connect) + then become part of the observed grammar. The EdgeRule layer may later TIGHTEN the grammar to declared-required edges (promote-once-proven), but the computed-observed grammar is the v1 substrate.";
        :>> rationale = "Computed-observed grammar is honest, generative, and decision-light: no frozen-schema change, no per-domain authoring, and it automatically reflects how the model is actually used. It directly serves scope-aware creation + valid-edge offers while keeping the deliverable/engine separation. It is the minimal thing that makes 'focus the scope, focus the actionable context, modify at that scope + adjacent' real.";
        :>> consequences = "keel: view::grammar_json + GET /api/grammar (done, KEEL_API 1.16.0). Viewer: the New-item type picker + Connect kinds become scope-aware from the grammar (srViewerScopedAuthoring). Risk: an observational grammar cannot suggest a valid-but-never-used pairing (mitigated: full algebra remains available via Connect; and an EdgeRule tightening path exists). This Decision is PROPOSED; acceptance is the human's end-of-sitting per-sitting review (D0049/D0019).";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0127Accept : Test { :>> id = "99b16392-06b4-4ede-b875-749b1fbc9914"; :>> method = VerificationMethod::confirmation; :>> procedureText = "accepted"; }
    part d0127AcceptR1 : TestResult { :>> id = "06138ac0-0e70-4020-89f2-365246dec02d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9326a17"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "wweatherholtz"; }
}
