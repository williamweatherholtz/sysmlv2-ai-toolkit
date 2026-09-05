// D0307 (status: the `status` field below is the truth) — A done task whose OWN criterion changed since its passing result computes as suspect, the same way it already does when a dependency's criterion changes
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0307Accept event on sign-off.
package Decision0307 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0307 : Decision {
        :>> id = "0f7be9bf-d90c-4f88-990c-8bb61619b033";
        :>> title = "A done task whose OWN criterion changed since its passing result computes as suspect, the same way it already does when a dependency's criterion changes";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human's direction (st064, st065): trust but verify, with the DoD as the gold standard - the thing verified must be the thing that was agreed. Measured 2026-09-02: orient's criterion_suspects compared only a task's DEPENDENCIES' criteria against the verified commit; editing a done task's own procedureText produced no suspect entry and the task stayed done, so the owner of a task - usually the AI - could widen the criterion after the pass under D0108 with the pass still standing. The post-edit advisory mentioned the drift at edit time and was non-blocking, so nothing computed carried it. issue343 recorded the class.";
        :>> decision = "criterion_suspects compares each done task's own DoD text at HEAD against its text at the result's judgedAgainst SHA, in the same batched blob fetch that already serves the dependency comparison, and marks the task suspect with a reason naming the task, both SHAs and that it is the OWN criterion. A criterion that did not yet exist at the verified SHA - a DoD recorded in the same commit as its result - has no historical text and is not judged; that limit is stated here and shared with the acceptance-text comparison issue341 asks for. Measured cost: orient 2.4s to about 4s on this tree, one batched cat-file carrying roughly five hundred more blobs.";
        :>> rationale = "WHY THE SAME MECHANISM: the dependency check already answered the question 'does the verified text still stand' for one side of an edge; the own side is the same question with the same evidence, and one helper for both is one place to be wrong. WHY SUSPECT AND NOT A GATE: suspicion is the honest-state verdict (D0098) - the pass was true when judged and the words moved; the gate that follows is reverify, which re-runs the declared test against the new words or refuses. WHY THE COST IS ACCEPTED: orient is the human's orientation, not a hook on every edit, and the alternative - trusting that nobody rewrote what was verified - is the failure this engine exists to remove. WHAT IT DOES NOT COVER: a criterion recorded with its result in one commit, which is how every sprint story's DoD is written; those are protected by the retro-backlog and evidence-cited guards reading the same text, and by the human's review of the sprint record.";
        :>> consequences = "dcOwnDoDDriftIsSuspect is delivered; issue343 resolves. The live experiment that motivated it - widen a done task's DoD, orient, revert - goes suspect and clean; the integration test own_dod_drift_is_suspect holds it. Orient's stated timing changes. The acceptance-text sibling (dcAcceptanceBindsToTheText, issue341) is the next frontier item and reuses the shape.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0307Accept : Test { :>> id = "c48fedbe-6809-41a5-82db-ac655ee5307e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0307AcceptR1 : TestResult { :>> id = "56a40854-70e0-4e4c-94d6-fd9ec6f3a247"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b1a1f99"; :>> judgedAt = "2026-09-04"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
