// D0288 (status: the `status` field below is the truth) — The exec-summary process, its skill, its template and its unit record are removed from the engine; decision-surfacing renders its page directly
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0288Accept event on sign-off.
package Decision0288 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0288 : Decision {
        :>> id = "ba722495-d4ab-4cb9-8f22-c15dd3c2c38f";
        :>> title = "The exec-summary process, its skill, its template and its unit record are removed from the engine; decision-surfacing renders its page directly";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human (st073): remove exec-summary as a process. It was the process that filled an HTML template (templates/exec-summary) with tabbed decision pages, ran a browser suite against the template's word budgets, and published the result as an artifact; decision-surfacing (D0269) deployed it as its render step. In this repository it existed as a process file, a skill with its registry, a root-level template and a Python test directory, an installed-unit record at version 3 and a minted unit id; library-stewardship named it as the worked example and stpa-diagram named its page as a place the human reads.";
        :>> decision = "Delete the process, the skill and its registry, the template and its tests, the .claude mirror, the installed-unit record and the unit id. Rewrite every dependent claim in the same commit, with no tombstones: decision-surfacing's render step authors the published page directly - one section per pending decision with stake, steelmanned options with honest costs, the recommendation and what would change it, republished to the same path so the URL stays one queue, with the console deck as the second surface; library-stewardship's worked example becomes decision-channel; stpa-diagram embeds where the human reads, the console or the published page; my own DoDs that named the page name decision-surfacing instead; the one unit-test fixture path that used the unit's template as its example uses a neutral name. Incident history that names the unit - comments in process_cmd.rs about bugs found while importing it, the identity test's preamble, done sprints and their receipts, the propriety record, the landscape review - stays, because history is what happened, not a claim that the unit exists.";
        :>> rationale = "WHY DELETE RATHER THAN DEACTIVATE: deactivation (D0138) is a project choosing not to run a process the engine still ships; the human's instruction is that the engine not carry it. WHY THE PAGE SURVIVES WITHOUT THE PROCESS: the value of exec-summary was the content contract - stake, steelmanned options, cost, recommendation, what would change my mind - and decision-surfacing already states that contract in its own steps; what goes is the template, its word-budget suite and the skill that wrapped them, which had become a second home for the same rules. WHY NO TOMBSTONES: a doc that says 'exec-summary used to do this' is a claim about a thing that no longer exists, and the doc-removal rule is to state what IS. WHAT THIS DOES NOT DO: it does not remove the unit from the portable-content LIBRARY, which is the human's repository and holds exec-summary at version 3; retiring it there is a library-stewardship act of its own, and a downstream project that imported it keeps working until it chooses otherwise.";
        :>> consequences = "The engine ships one fewer process (32); decision-surfacing has no dependency outside itself; the installed-unit record and unit-ids table no longer name the unit, so adoption-check and the identity guard see a consistent tree. The library still carries the published unit - stated, not hidden. REMOVAL PATH (D0163): this IS a removal; restoring means importing the unit from the library, which is exactly what the library is for.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0288Accept : Test { :>> id = "dd774f77-a9fa-453b-8655-38fc901c0b44"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/29."; }
    part d0288AcceptR1 : TestResult { :>> id = "4f18a08b-5452-4754-9afe-fc1c763a74e9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "27f6944"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "wweatherholtz"; }
}
