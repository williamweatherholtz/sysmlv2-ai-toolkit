// =====================================================================
// D0123 (PROPOSED — NOT YET ACCEPTED) — obligation-views respect creation-time GOVERNING-VERSION (no
// grandfathering); sprint-ceremony phases (incl. retro) need NO human confirmation; declare `retro`
// as a Delivery phase. Fixes issue068. Human-directed 2026-07-15 ("items should only be governed by
// the process at the time of creation... historical work shouldn't be grandfathered into changed
// processes"; "the sprint retro process itself needs no confirmation gate"). Acceptance = human (§3a).
// =====================================================================
package Decision0123 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0123 : Decision {
        :>> id = "10a1b0c1-4123-42a3-ab5c-516273849723";
        :>> title = "D0123: obligation-views honor governing-version (no grandfathering); ceremony phases (incl. retro) need no confirmation; declare retro as a Delivery phase";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue068: the new review-queue view (D0121) surfaced legacy pre-D0049 retro confirmation gates (rustS5/6/7) as CURRENT human obligations, ignoring the engine's existing governing-version resolver (govern.rs; D0068/D0069/D0070/D0034) — which resolves, by git ancestry, the process version that governed each item AS-OF its charter, with forward-only retroactivity markers. The human's principle (2026-07-15): items are governed by the process in force at their CREATION; historical work is NOT grandfathered into changed processes, nor flagged deficient when a process expands (exceptions — safety-related items, high-level Needs — MAY be deliberately re-governed). Also: retro is AUTONOMOUS (D0049); the sprint-retro process GENERATES items (each individually assessed) and itself needs NO confirmation gate.";
        :>> decision = "(1) OBLIGATION-VIEWS + GUARDS HONOR GOVERNING-VERSION. A computed view of human obligations (the review queue) and any enforcement guard must reflect CURRENT-process obligations and MUST NOT resurrect, nor flag as deficient, historical items governed by a superseded process version — they consult the existing governing-version resolver / respect forward-only retroactivity. Any NEW guard is forward-only. (2) CEREMONY PHASES NEED NO HUMAN CONFIRMATION. Sprint-ceremony phase gates (refine/standup/implement/review/closeOut/retro) are autonomous (D0049, method=inspect/analyze); the retro generator is not gated — its OUTPUTS (issues/decisions/tasks) are each independently assessed. (3) DECLARE `retro` as a Delivery workflow phase, so the model-driven ceremony filter (is_ceremony_gate, from declared phases) recognizes retro gates and drops them from the review queue's ACTIONABLE set. (4) Legacy pre-D0049 confirmation ceremony gates KEEP their creation-time method (NOT migrated/rewritten); the view simply stops treating them as current obligations.";
        :>> rationale = "ALTERNATIVES: (i) reclassify the legacy gates confirmation->analyze — REJECTED (+reverted): rewrites their creation-time truth = the grandfathering the principle forbids. (ii) a guard that FAILS any ceremony confirmation gate — REJECTED: retroactively flags historical items deficient (the same anti-pattern), and duplicates work the governing-version machinery already does. (iii) leave the queue naive — REJECTED: it surfaces false obligations and erodes trust in the oversight surface. CHOSEN: fix the VIEW to honor governing-version + declare retro — creation-time history is preserved, current obligations stay accurate, and it REUSES the engine's existing versioning machinery rather than bolting on a retroactive control. Root-causes the miss (D0047): a new computed obligation-view was built without applying the governing-version discipline; the durable fix is the design rule (2)+(1), tracked as issue068, not a data patch.";
        :>> consequences = "PROPOSED; acceptance = human (§3a — alters a computed view's meaning + the Delivery workflow def). On acceptance: `retro` is declared in .engine/workflows/delivery.sysml (forward-looking — governing-version keeps historical sprints under their charter-time process); the review queue drops retro gates from actionable via the existing ceremony filter; resolver action reviewQueueGoverningVersion completes -> issue068 resolved. Legacy gates untouched. META-LESSON (review checklist): every NEW computed obligation-view/guard must be checked against governing-version + retroactivity before shipping. Broader per-item governing-version integration for obligation-views is a follow-on if a non-ceremony case arises. Relates D0121, D0049, D0047, D0034, D0068/D0069/D0070, issue068."; }

    #DependsOn dependency from d0123 to d0049;
    #DependsOn dependency from d0123 to d0121;

    // acceptance event (D0066/D0106 — human-judged, not fabricated): wweatherholtz accepted via chat 2026-07-15 ("accept d0123")
    verification d0123Accept : Test { :>> id = "10a1b0c1-4123-42a3-bb5c-516273849723"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz explicitly accepted D0123 (2026-07-15, 'accept d0123'): obligation-views honor governing-version (no grandfathering); sprint-ceremony phases incl. retro need no confirmation; declare retro as a Delivery phase. The human's word IS the attestation."; }
    part d0123AcceptR1 : TestResult { :>> id = "10a1b0c1-4123-42a3-cb5c-516273849723"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d36c57"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "wweatherholtz"; }
}
