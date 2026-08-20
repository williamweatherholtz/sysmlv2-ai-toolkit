// =====================================================================
// D0079 — Quality doctrine: the engine optimizes for COMPUTED ASSURANCE, not traceability
// artifacts. Build analyses that defensibly raise confidence the deliverable is correct +
// complete; refuse compliance-only / legacy artifacts. Frames the assurance milestone
// (assurance-coverage -> critique mechanism -> critique coverage -> assurance-readiness gate)
// + the severity taxonomy. Accepted 2026-06-20 by wweatherholtz (MBSE-analysis quality-lens dialogue).
// =====================================================================
package Decision0079 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // Doctrine/representation decision (like D0075): edits no process-def in its own commit, so no
    // process-change marker (D0070). The critique mechanism's schema+process land under D0080.
    part d0079 : Decision {
        :>> id = "63940516-2c7d-4e8f-a079-51627384966b";
        :>> title = "D0079: computed-assurance doctrine — assurance analyses over traceability artifacts; refuse compliance-only artifacts; severity taxonomy; antagonistic critique = lens-tagged verification";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "An MBSE-analysis comparison (Capella/Arcadia, Cameo, DOORS) through a QUALITY lens. Finding: the RTM-the-grid, churn metrics, and document generation are compliance/legacy artifacts (the engine already refuses authored matrices, sec2); the timeless substance is COVERAGE / IMPACT / V&V analysis. The user reframed the goal: a framework that GUARANTEES supreme quality, with no artifacts that do not defensibly contribute (antiquated workflows / regulatory compliance). Through that lens our work/process/evidence analyses (suspect, audit, guards, governance) already MEET or EXCEED MBSE tools; the system-analysis categories are either out of scope by design (the deliverable's domain never enters the engine) or thin. The one defensible gap is verification COVERAGE (C) — already a written-but-unimplemented contract (computed-state.md). And we have adversarial-critique skills (premortem/devil's-advocate/traceability/stakeholder) that no MBSE tool has, but they are not wired to the tracked model.";
        :>> decision = "The engine optimizes for COMPUTED ASSURANCE: analyses that defensibly raise confidence the deliverable is correct + complete, computed from authored facts + git (never stored, askable as-of any commit). The assurance set: (1) COVERAGE/satisfaction — every Need/Requirement/Decision has a COMPLETE, PASSING, NON-STALE verification chain (C); (2) IMPACT/staleness — nothing falsely-verified after change (suspect, have); (3) INVARIANTS held at commit (guards, have); (4) ANTAGONISTIC CRITIQUE — defects/risks anticipated + adversarially reviewed (new, D0080); (5) DECISION soundness (have); (6) PROCESS adherence (audit, have). EXPLICIT NON-GOALS (scope boundaries, not oversights): the RTM grid, churn metrics, document generation; AND — per the work-not-deliverable boundary — quantitative/parametric/trade-study + behavioral/simulation analysis (the deliverable's domain vocabulary never enters the engine). SEVERITY taxonomy (critique findings + issues): Critical (defeats purpose/unsafe — must be fixed or human-risk-accepted), High + Medium (need human disposition: act/accept-risk/dismiss), Low (AI-dispositioned, surfaced not gated); the human-disposition gate fires at >= Medium. ANTAGONISTIC CRITIQUE is a LENS-TAGGED VERIFICATION (a Test/TestResult with a lens + critic identity, verify-linked to the target) so it reuses ALL assurance machinery: coverage (C), staleness (suspect -> re-critique + re-open disposition), human gate (confirmation), findings -> Issues (#Resolves loop, D0077). Required lenses are BROAD (Needs, Requirements, Decisions, Architecture); the critic must be INDEPENDENT of the author. H's value (release-gate regression of verified claims) falls out of the git-temporal substrate (run C + suspect as-of two baselines); churn is excluded.";
        :>> rationale = "Quality lens beats feature parity: build only what defensibly raises assurance. Compute-don't-store means assurance can never show a stale green + can be asked as-of any commit (a temporal edge over DOORS baselines). Modelling critique as verification is the key economy — one mechanism (verify/suspect/confirmation/coverage/issue-loop) serves it, instead of a parallel critique subsystem. Refusing compliance-only artifacts (RTM grid, churn, doc-gen) and out-of-domain analysis (parametric/simulation) keeps the engine lean + honest (D0054 friction is the #1 risk). ALTERNATIVES: (A) chase MBSE feature parity (RTM, parametrics, simulation) — REJECTED: most do not defensibly add quality and parametric/behavioral analysis violates the work-not-deliverable boundary; (B) build a standalone critique subsystem — REJECTED: duplicates suspect/coverage/confirmation; (C CHOSEN) computed-assurance doctrine + critique-as-verification. CRITERIA: defensibly-raises-confidence x compute-don't-store x reuse-one-mechanism x refuse-compliance-theatre.";
        :>> consequences = "Milestone (chained): assuranceCoverage (C — the satisfaction/verification-coverage view, implements computed-state.md) -> critiqueMechanism (D0080: frozen-schema lens/severity/critic-identity + VerificationMethod for critique + the antagonistic-critique process-def + deploying skill; findings -> Issues) -> critiqueCoverage (per-element x required-lens coverage + a critique guard; staleness via suspect) -> assuranceReadiness (composite gate: coverage complete AND no stale verification AND no stale critique AND every >=Medium finding dispositioned AND Critical fixed/risk-accepted AND invariants green). NON-GOALS recorded: RTM grid, churn, doc-gen, parametric/trade, simulation. Pairs with D0050 (suspect), D0046 (audit), D0077/D0078 (issue loop), D0054 (friction/decision-analysis), D0048 (Rust authority), and the existing critic-* skills (premortem/devil's-advocate/traceability/stakeholder/arbiter).";
    }

    verification d0079Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a079-51627384966c";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0079 (computed-assurance doctrine: assurance analyses over compliance artifacts; RTM-grid/churn/doc-gen + parametric/simulation are explicit non-goals; severity Critical/High/Medium/Low with the human gate at >=Medium; antagonistic critique = lens-tagged verification with broad required lenses + critic independence) across the MBSE-analysis quality-lens dialogue on 2026-06-20: 'these findings are great, carry out a,b,c', 'I want a framework that guarantees supreme quality ... I don't need artifacts that ... don't contribute to that quality', and the design selections (lens-tagged verification; broad required lenses; human gate at medium-or-high).";
    }
    part d0079AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a079-51627384966d";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "76d8ae1";
        :>> judgedAt = "2026-06-20";
        :>> judgedBy = "wweatherholtz";
    }
}
