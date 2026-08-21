// D0186 (PROPOSED — NOT YET ACCEPTED) — D-P2 (direction, not scope): an MCP surface is a future binding of deny-and-provide and harness portability, trigger-gated
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0186Accept event on sign-off.
package Decision0186 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0186 : Decision {
        :>> id = "19443aaa-9647-45cd-befd-52bd72c0a747";
        :>> title = "D-P2 (direction, not scope): an MCP surface is a future binding of deny-and-provide and harness portability, trigger-gated";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0128 ruled MCP a surface for discovery/ergonomics, NOT an enforcement layer, and sequenced it fourth; D0130 reaffirmed the rejection of MCP-as-governance. The one recorded independent critique of the justifying Need clause (critiques.sysml nAiGovernanceInLoopCorrectnessCritique, verdict fail) says the clause was never the requirement - and that failing critique has sat untriaged, violating the issues-must-be-triaged invariant. Registering tools cannot force anything: MCP adds options and removes none; forcing comes from denials, blocking hooks, and refusing servers. Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Recorded as DIRECTION with named triggers, D0157 pattern - no scheduled slot, resolving rather than papering over the D0128 sequencing question (MCP stays fourth-or-later in its own recorded queue). What MCP would bind when triggered: kernel invariant K13 (deny-and-provide - currently bound by the CLI's refusal paragraphs) and harness portability (.claude/ enforcement is Claude-Code-specific; a .mcp.json travels to any MCP harness - the one real argument). Triggers: a second harness in actual downstream use, or a named consumer (D0144). To make the trigger fire-able rather than passive, downstream docs state the harness-support matrix: on a non-Claude harness you get the CLI, commit/CI gates, and audit-history; in-loop enforcement is Claude-Code-bound; MCP is a recorded direction - ask if you need it. The write-path extraction (accept, apply-review, record-issue preflights, enroll, actor-set out of main.rs into write.rs) is good refactoring that rides whenever those handlers are next touched. keel accept is bound by K6 and never becomes agent-callable on any present or future surface. The untriaged critique Issue is recorded with THIS Decision as its Resolves target.";
        :>> rationale = "Building MCP now would jump the accepted queue, build against no requirement, and contradict the repo's own recorded critique - the exact confident-wrong-answer pattern the codebase documents. The portability value is real and fully preserved at zero build cost by recording it with an observable trigger.";
        :>> consequences = "The MCP question is settled on the record instead of hovering; the failing critique is finally triaged; the trigger can actually fire because downstream users are told what they are missing.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0186Accept : Test { :>> id = "b82e5fbf-00b5-4479-a6e4-3ce03c4db404"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'accepted all'. Said in response to the presented queue of fifteen proposed Decisions - D0174, D0175, D0176, D0177, D0178, D0179, D0180, D0181, D0182, D0183, D0184, D0185, D0186, D0187, D0188 - so 'all' is that queue and this acceptance covers this Decision as one of the fifteen. The open fork stated in the proposal (section 8-A, strict default for provably empty directories) was named to them alongside the queue with the invitation to object on D0174; they accepted without objection, so D0174 is accepted AS WRITTEN including its stated strict-for-empty default."; }
    part d0186AcceptR1 : TestResult { :>> id = "9692d538-0c3e-495f-bdb5-d83f0599f93b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10b66bd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
