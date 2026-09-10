// D0422 (PROPOSED — NOT YET ACCEPTED) — hookLandRunsFromACopy: the post-commit hook copies the self-build's binary to a land-only image before running keel land, so an armed touched-test run can relink the binary it would otherwise be running from
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0422Accept event on sign-off.
package Decision0422 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0422 : Decision {
        :>> id = "ae829fbb-b80b-460d-86e1-8592640a2284";
        :>> title = "hookLandRunsFromACopy: the post-commit hook copies the self-build's binary to a land-only image before running keel land, so an armed touched-test run can relink the binary it would otherwise be running from";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "issue436: .githooks/post-commit resolves keel to ./target/release/keel(.exe) in the self-build and runs land from it. D0421 makes land run the touched test set once accepted, and cargo test --release relinks that very file, which Windows refuses while it is running (issue386) - so the control's own home could not run the control: on 9d574f8 the hook's land reached the own-image refusal and pushed nothing (the arming was issue435's false positive, but the path is the same when the gate is legitimately armed). Accepting D0421 without this change arms a refusal that fires on every hook-driven push of a change touching a named module.";
        :>> decision = "NOT A FORK: one copy, one place, one fallback. In .githooks/post-commit, when the binary resolves to ./target/release/keel or ./target/release/keel.exe, the hook copies it to ./target/release/keel-land(.exe) and runs land from the copy; the test is on the RESOLVED file - a keel found on PATH that resolves into ./target/release is the same image and is copied too (issue437), one that resolves elsewhere runs as it is; a copy that cannot be made falls back to the original, whose own-image refusal then names why nothing ran. Nothing else in the hook changes, and no downstream project is affected - the case matches only the self-build's own target path.";
        :>> rationale = "The copy is the issue386 answer already given to keel suite and to the console (keel-serve.exe); the hook is the one caller that could not be told to run from a copy because nothing types its command. Making the copy in the hook keeps one land loop (D0134) and adds no state: the copy is overwritten on every commit, so it is always the binary that was current at the commit. It is on the enforcement surface (D0209 clause 2), so it carries this Decision and stays proposed under D0337 until the human's word; until then the hook change is committed but, since D0421 is also proposed, its only visible effect is that the hook's land runs from keel-land.exe. Its acceptance should precede or accompany D0421's.";
        :>> consequences = ".githooks/post-commit: the case block and its comment. CLAUDE.md names the copy where it names the own-image rule. What accepting binds: the hook writes one file under target/release on every commit. What it forecloses: nothing. What would change it: a land that no longer shells out to cargo, or a host where a running image can be replaced.";
    }
}
