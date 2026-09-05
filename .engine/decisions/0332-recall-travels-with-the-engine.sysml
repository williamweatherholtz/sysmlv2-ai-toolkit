// D0332 (status: the `status` field below is the truth) — recallTravelsWithTheEngine: a downstream project gets recall and injection from the engine alone, and the knowledge-graph-memory skill says which steps are the engine's and which are optional
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0332Accept event on sign-off.
package Decision0332 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0332 : Decision {
        :>> id = "b40d53d3-2984-471a-ac8b-9b392d671b57";
        :>> title = "recallTravelsWithTheEngine: a downstream project gets recall and injection from the engine alone, and the knowledge-graph-memory skill says which steps are the engine's and which are optional";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "kgTravelsWithTheEngine: the DoD asked that a downstream project get recall and injection with no authored vocabulary and no bespoke script, that the human-owned half (.knowledge/ questions and aliases) be optional now that D0243 made seeding corpus-derived, and that the skill say so without shipping this repository's questions (the issue243 class). Verified on a fresh keel init scaffold on 2026-09-05: the UserPromptSubmit hook is wired by claude_surface; keel recall answers over the scaffold's own 326 items with an empty .knowledge/; none of this repository's questions are present; sync-claude --check is clean. The mechanism was already the engine's; the skill did not say so and read as eight steps a project must build.";
        :>> decision = "The knowledge-graph-memory skill gains a section for a keel project stating, step by step, which of its eight steps the engine performs (vocabulary and shapes by corpus-derived seeding, identity and seed-plus-walk by keel recall and keel why, injection by the hook keel init and sync-claude wire, verification by this repository's benches) and which are optional for a project (.knowledge/questions for question-coverage, .knowledge/lexicon for aliases), and states that .knowledge/ is instance data keel init ships none of.";
        :>> rationale = "A skill that describes the generic method without saying what the engine already does sends an adopter to rebuild the engine's own work - the D0040 leak in reverse. The optional half is optional because D0243 moved seeding to the corpus; a project that never declares a question still recalls, and coverage is simply not measured until it does. Nothing of this repository's .knowledge/ travels, by construction (init does not copy it), which the scaffold check confirms. NOT A FORK: one course, the statement in the skill; the table's rows are the process's steps, not alternatives.";
        :>> consequences = "kgTravelsWithTheEngine is done on the scaffold evidence; the skill's registry entry is unchanged. STATED: this Decision adds no mechanism - it records that the mechanism was already engine-borne and makes the skill say so.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0332Accept : Test { :>> id = "c1e8cf1b-b82a-4338-88ff-f4ce7a008b7a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0332AcceptR1 : TestResult { :>> id = "bee07d45-9b40-4c54-a23c-fafd15b0d48b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "18270a8"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
