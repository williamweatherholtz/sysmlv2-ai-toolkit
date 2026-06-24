// =====================================================================
// D0099 — requirement-rootedness rule (implements the rootedness half of D0098/issue047). Data-driven:
// running the rootedness computation showed 144/147 delivery Stories are decision-chartered, NOT
// need-rooted — because the engine is legitimately decision-driven (D0064 bootstrap). So a blanket
// "every Story must trace to a Need" hard gate would FLOOD + miscategorize legitimate engine evolution
// as a "lie". The honest, narrow rule: a user-facing CAPABILITY (marked `#Capability`) must state its
// driving Need (`#DerivedFrom`->Need) — that is the real traceability lie when omitted (the serve
// class, issue046). Unmarked decision-driven work is exempt. Accepted 2026-06-24 by wweatherholtz.
// =====================================================================
package Decision0099 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0099 : Decision {
        :>> id = "99a9b0c1-4001-42a3-ab5c-516273849699";
        :>> title = "D0099: requirement-rootedness via a #Capability marker — a declared capability must state its driving Need; decision-driven work exempt";
        :>> createdAt = "2026-06-24";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0098 made requirement-rootedness a HARD honesty gate (a capability with no driving Need is a lie-of-omission). Implementing it, the rootedness computation over the real model showed 144/147 delivery Stories are NOT need-rooted — almost all are chartered by Decisions (d0079/d0048/d0080/d0093...) with no #DerivedFrom->Need edge. That is not 144 lies: the engine was built DECISION-DRIVEN by design (D0064 bootstrap-over-mandate), where the Decision IS the justification. A blanket 'every Story -> Need' hard gate would flood (block everything or grandfather everything) AND miscategorize legitimate decision-driven engine evolution as dishonest. What is actually a lie is a USER-FACING CAPABILITY shipped with no stated stakeholder Need (serve, issue046) — a judgment about a few items, not a property of all 147 Stories.";
        :>> decision = "Scope the rootedness HARD gate to an explicit `#Capability` marker. RULE: an element marked `#Capability` (a user-facing feature) MUST carry a `#DerivedFrom` edge to a Need; otherwise `keel guard requirement-rootedness` FAILS (enforced; honest-state gate per D0098). UNMARKED work is EXEMPT — decision-driven engine evolution is legitimate (D0064). The marker is the honest, explicit opt-in signal of 'this is a feature', applied when you build something users touch (not a hidden WIP flag — it ADDS an obligation, it does not hide a gap). Because nothing is marked until a capability is declared, the gate never floods and never blocks existing work; it binds the next capability (and serve, when serveRequirementsTrace marks d0094 #Capability together with its Need). The full charter-source balance (need-rooted vs decision-chartered vs orphan) is the NON-BLOCKING `keel rootedness` burndown (D0098), surfaced for visibility. (The DOWNWARD tier-satisfaction comprehensiveness burndown + the orient surfacing remain the rest of requirementRootednessGuard.)";
        :>> rationale = "ALTERNATIVES: (i) blanket 'every Story must reach a Need', charter-time scoped — REJECTED on the DATA (144/147 unrooted; floods, and re-imposes a Need-link on every future decision-driven CR — high ceremony the human flagged as red-herring risk). (ii) burndown only, no hard gate — REJECTED (the human chose a hard gate; a declared capability without a Need IS a lie worth blocking). (iii) infer 'capability' from Story kind/heuristics — REJECTED (unreliable; 'user-facing' is a judgment). (iv) CHOSEN — an explicit #Capability marker carries the judgment honestly: marked => must state a Need (hard), unmarked => exempt, everything => visible on the rootedness burndown. CRITERIA: honesty (a declared capability states its why) x no-flood (data-grounded) x low-ceremony (only features opt in) x decision-driven-evolution-stays-legitimate (D0064).";
        :>> consequences = "New enforced honest-state guard `requirement-rootedness` (GUARD_NAMES 11 -> 12): a #Capability item lacking #DerivedFrom->Need fails. New `keel rootedness` burndown view (charter-source balance: need/decision/orphan + the #Capability gate set). Passes vacuously today (0 #Capability items). NO existing item is marked yet; serveRequirementsTrace will mark d0094 #Capability + add its #DerivedFrom->Need (gate then meaningful for serve). Downstream `keel init` inherits via `keel guard`. Partially resolves issue047 (the UPWARD rootedness half); the DOWNWARD tier-satisfaction comprehensiveness burndown + the orient surfacing remain open under requirementRootednessGuard. doc-sync: CLAUDE.md (guard set -> 12 + #Capability/rootedness), README (rootedness view). Refines D0098 (the rootedness gate it authorized); no .engine/processes edit (gate logic is guards.rs), so no #ProspectiveChange keystone.";
    }

    verification d0099Accept : Test { :>> id = "99a9b0c1-4001-42a3-bb5c-516273849699"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of d0099 (requirement-rootedness scoped to an explicit #Capability marker: a marked capability must #DerivedFrom a Need or `guard requirement-rootedness` fails; decision-driven work exempt; full balance on the `keel rootedness` burndown) via the explicit choice of the '#Capability marker' option on 2026-06-24, after the rootedness data (144/147 decision-chartered) showed a blanket gate would flood."; }
    part d0099AcceptR1 : TestResult { :>> id = "99a9b0c1-4001-42a3-cb5c-516273849699"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4df27d4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "wweatherholtz"; }
}
