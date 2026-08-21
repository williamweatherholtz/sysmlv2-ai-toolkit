// D0178 (PROPOSED — NOT YET ACCEPTED) — D-P1c: writes recording human judgment are human-channel-only (kernel invariant K6)
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0178Accept event on sign-off.
package Decision0178 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0178 : Decision {
        :>> id = "2e432cd5-3a12-4dfc-85b7-d28e02515609";
        :>> title = "D-P1c: writes recording human judgment are human-channel-only (kernel invariant K6)";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The human sign-off is the one record the entire decision system rests on, and today it is protected only by prose - an attestation is a WRITE, not a SAY, so unlike the D0151 residual it IS gateable. Three panel members independently found the same seam: an agent on the human's machine, where .keel/actor binds a Person, could run keel accept via Bash and record the human's sign-off; the actor binding is agent-mutable state (keel actor set, KEEL_ACTOR, --by), so a write-layer kind-check alone is circular. Fabricated attestation is a recorded failure class (issue099, issue144, D0130's contract). Reviewed by a 9-member adversarial panel (2 pessimists, 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) across three rounds under an honesty rule; round 3 verdict 9 ACCEPT / 0 OBJECT, uncompelled. Full evidence and finding-to-resolution map: docs/proposals/2026-08-21-production-readiness-design.md sections 10-12. NOTE per the 2026-08-14 panel precedent (.tracking/panel-critiques.sysml): the panelists are subagent contexts of the authoring model, a rigor framing, NOT independent critics - this record claims convergence, never critic-independence.";
        :>> decision = "Three layers, no single point of trust: (1) write layer - accept_decision and method=confirmation results refuse when the resolved actor is ActorKind::ai; (2) harness layer - the Bash matcher's keel-argv0 exemption carves OUT the human-judgment and actor-identity subcommands (keel accept, confirmation-method result writes, keel actor set, Person enrollment, and invocations carrying KEEL_ACTOR/--by/--judged-by naming a Person), routing them through the ask tier plus a recorded fact; (3) channel layer - accept in a session bearing agent-environment markers requires a console-issued or TTY-interactive confirmation. The protected-subcommand set should be DERIVED from a declared attribute on write-API operations rather than hand-enumerated (apply-review is the case a hand list misses); the marker heuristic stays best-effort, never a reactive arms race - the carve-out and tree-derived audit are the real controls. No agent-callable surface ever exposes accept, on any present or future transport.";
        :>> rationale = "A forced-process design whose human sign-off is agent-invocable has not forced its kernel: K6 is the invariant the problem statement names verbatim. The layering exists because each individual check is defeatable (the binding is mutable, markers are scrubable) but the conjunction plus unconditional human diff review plus audit-history covers the cooperative-drift threat model honestly.";
        :>> consequences = "Acceptance events become structurally human. Acceptance test: keel accept under an AI-bound actor refuses, and via an agent session under the human's binding it is intercepted before the write layer. Charters p1HumanChannel.";
    }
}
