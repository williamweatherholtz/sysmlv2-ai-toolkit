// D0344 (PROPOSED — NOT YET ACCEPTED) — charterMustResolve: an activation charter names a Decision this project holds, or the activation-manifest guard fails it and onboard prints UNRESOLVED CHARTER
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0344Accept event on sign-off.
package Decision0344 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0344 : Decision {
        :>> id = "8826b389-a792-46a4-9098-d25d32285cac";
        :>> title = "charterMustResolve: an activation charter names a Decision this project holds, or the activation-manifest guard fails it and onboard prints UNRESOLVED CHARTER";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "GH#56 (st095, us094, issue380). A v0.3.1 keel migrate rewrote a downstream project's chartered activation.toml with the engine's own - re-arming two processes the project had declined for stated reasons - and keel onboard then reported the set CHARTERED by d0226, a Decision that project does not contain. The overwrite is the issue314 class and is fixed on main since 6583e08 (2026-08-30), which is not an ancestor of v0.3.1, so adopters on the release still meet it until a release ships the fix. The dangling charter was fixed nowhere: nothing checked that charteredBy resolves, so the command whose job is to answer on what basis these processes were chosen printed a provenance claim it could not read, as fact.";
        :>> decision = "onboard::charter_resolves asks whether charteredBy names a dNNNN with a file .engine/decisions/NNNN-*.sysml in THIS project; the activation-manifest guard fails a charter that does not resolve, naming the Decision and the path it looked for; keel onboard prints UNRESOLVED CHARTER <id> with the remedy instead of CHARTERED, and its JSON carries charterResolves with chartered false. NOT A FORK: a claim the tree cannot back is stated as such - there is no second honest reading.";
        :>> rationale = "Why the guard and not only onboard: onboard is read when a human asks; the guard runs at every commit and in CI, so the silent form of GH#56 - a project living for weeks with another project's charter - becomes loud at the next gate. Why hard: the check is exact - a file exists or it does not - and the failure it catches is a recorded human decision replaced by nobody's. Arming (D0298): two binary tests on a fresh scaffold - a charteredBy of d0226 fails the guard naming 0226-*.sysml and onboard prints UNRESOLVED CHARTER with chartered false in JSON; a charter naming the project's own D0001 passes and reads CHARTERED - and this repository's own charter (d0226, which it does hold) stays green.";
        :>> consequences = "A project that met GH#56 sees a red gate naming the foreign charter instead of a green onboard, and re-charters through project-onboarding or restores its own file from history. Marked safety-change: guard source is enforcement surface, so under D0337 this waits for the human's acceptance; the code ships because the failing state is a true state. The release gap is stated here as a fact for the deploy workflow and not performed: fixes on main since 2026-08-29 - issue314's overwrite among them - reach adopters only through a release. issue380 resolves on dcCharterMustResolve's pass.";
    }
}
