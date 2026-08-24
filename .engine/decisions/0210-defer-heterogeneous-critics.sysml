// D0210 (PROPOSED — NOT YET ACCEPTED) — deferHeterogeneousCritics: park the different-model critic until the human supplies a second model - the trigger is a person, not a date
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0210Accept event on sign-off.
package Decision0210 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0210 : Decision {
        :>> id = "6e123821-beef-4ec5-8eb9-4b262ebfbd2b";
        :>> title = "deferHeterogeneousCritics: park the different-model critic until the human supplies a second model - the trigger is a person, not a date";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "Reviewing the ready frontier on 2026-08-23, dcHeterogeneousCritics stood at #1 by declaration order, but its own DoD records BLOCKED ON: the human supplying access to a second (non-Claude-lineage) model. The block is real and external: enrolling a different-model AI critic (actor-enrollment, kind ai, model identity recorded) so panels can claim cognition-distinct review rather than only role-distinct review is the debate-literature's strongest lever, but it cannot start without credentials only the human can provide. The human directed: 'defer #1 for now. proceed with others.' Left unmodeled, the item keeps surfacing as ready #1 - state asserted in prose (the DoD note) rather than computed, exactly the D0018 failure.";
        :>> decision = "DEFER dcHeterogeneousCritics for now. It stays on the backlog, undone, and is scoped out of the ACTIONABLE frontier until its content gate clears. NO WORK is derived or scheduled by this; the deferral is a decision not to build now, not a plan to build later. The TRIGGER is a PERSON, not a date (mirroring D0157): the human reraises it by supplying second-model access - a stronger trigger than a date for a blocker that only they can clear, since a date would come due while the credentials still had not.";
        :>> rationale = "A date-triggered defer ages badly when the blocker is an external input nobody scheduled - it would come due, the model would still be absent, and the honest answer would be to defer again. Binding the trigger to the person who owns the blocker is the D0157 pattern and it is the accurate one here. Recording it as a Decision (not leaving it in DoD prose) makes the park COMPUTED: the frontier then shows only what I can actually act on, which is exactly the human's standing preference - surface blockers and critical items, not a queue of things I cannot start.";
        :>> consequences = "The ready frontier drops dcHeterogeneousCritics and shows the three D0209 clauses I can execute now (dcFreezeEnforcementSurface, dcProcessCursor, dcProactivePostEdit). When the human supplies a second model, this Decision is superseded and the item returns to the frontier with its blocker cleared - no work was unwound because none was derived. The convergence-not-independence caveat in the adversarial-panel-review process doc remains true until then.";
    }

    // The engine's only scope-out mechanism is a #Supersede edge (§1.4/§2.4; issue100). It parks the
    // item OUT of the ready frontier while leaving it OUTSTANDING (not done — issue100: a retired item
    // is not done). Reversible: superseding D0210 when the human supplies a second model returns the
    // item to the frontier. This is the computed form of "defer", not a claim the work is finished.
    #Supersede dependency from d0210 to dcHeterogeneousCritics;
}
