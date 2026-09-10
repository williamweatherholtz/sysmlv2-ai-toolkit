// D0432 (PROPOSED — NOT YET ACCEPTED) — verifierRunsTheGatesTestSet: the verifier subagent's test run is the set the land will run - keel suite --touched - never a filter the primary chose
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0432Accept event on sign-off.
package Decision0432 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0432 : Decision {
        :>> id = "edf8d5d8-5605-4f79-8218-555208d307e4";
        :>> title = "verifierRunsTheGatesTestSet: the verifier subagent's test run is the set the land will run - keel suite --touched - never a filter the primary chose";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0425 splits a session into a primary that does the substance and a verifier that runs the checks. Its text names the verifier's test run as 'cargo test on the touched binaries', and in sprint 657 the primary translated that into a six-test filter naming the tests it had written. Those six passed. The land's armed touched run (D0421) runs the whole lib whenever keel-cli/src moved, and there the new test failed: it read a process-global static that another test's run_all was writing at the same moment. The commit stood, the push was refused, and the sprint's Implement gate carried a pass the gate's own run contradicted. The verifier had done exactly what it was asked; what it was asked was narrower than what the gate asks.";
        :>> decision = "NOT A FORK: this narrows one clause of D0425 and names the command. The verifier's test run is `keel suite --touched` - the touched set the land computes from the paths changed since origin/main (the integration binaries whose text names a changed module, plus the lib's own unit tests whenever any keel-cli/src path changed), writing .keel/metrics/touched-receipt.toml - and the verifier's receipt names that file's outcome, passed and failed counts. A cargo test filter the primary wrote is not a verifier run; when the touched set is empty the verifier says so and runs nothing more.";
        :>> rationale = "The land already computes the set a push must survive; a verifier that runs a different set can only agree with the gate by luck. Making the verifier run the same command removes the primary's choice of what to test from the chain - which is the independence D0425 was for - at no cost to the human and one command to the verifier. Tests that pass alone and fail together (shared statics, shared fixtures, port and file contention) are precisely the class a filter hides, and the lib run is where they show. What would change it: a touched run whose cost makes the verifier the sprint's critical path - the measured range is 7-11 minutes - in which case the answer is a faster suite, not a narrower verifier.";
        :>> consequences = "CLAUDE.md sec 4's D0425 rule names keel suite --touched as the verifier's test run; the verifier prompt written in a session names it too. No guard changes: the land remains the backstop and refused correctly here. issue459 resolves here. Process-change (D0337): it changes what the D0425 verifier runs, so it waits for the human's word. What accepting binds: a sprint's Implement gate passes on the gate's own set. What it forecloses: a verifier receipt built from a filter.";
    }
    #SupersedeClause dependency from d0432 to d0425;
}
