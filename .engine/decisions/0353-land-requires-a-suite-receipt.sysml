// D0353 (PROPOSED — NOT YET ACCEPTED) — landRequiresASuiteReceipt: keel suite runs the full suite and writes a machine-local receipt over the deliverable's fingerprint, and keel land refuses a self-build tree whose deliverable changed since the last green receipt
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0353Accept event on sign-off.
package Decision0353 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0353 : Decision {
        :>> id = "3e75da5f-0765-4d73-a2ad-5a671e2d3ecc";
        :>> title = "landRequiresASuiteReceipt: keel suite runs the full suite and writes a machine-local receipt over the deliverable's fingerprint, and keel land refuses a self-build tree whose deliverable changed since the last green receipt";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "dcLandRequiresASuiteReceipt. On 2026-09-02 sprint 521 landed on validate plus guards alone; the full suite had last run at sprint 520; CI went red for twenty minutes on a test the suite would have caught locally. The computed control structure says CI is detective - the commit is already on main when it speaks. Today the same shape recurred twice: two pushes (0d70e27, 0fd3173) went red in CI on a Linux-only exec-permission defect the local suite could not show, and every sprint since has run the suite by hand from a background shell with the result copied into a receipt by the author - a discipline, not a control.";
        :>> decision = "keel suite [ROOT] [-- cargo args] runs cargo test --release --no-fail-fast for the self-build, writes the log under .keel/metrics/ and a receipt at .keel/metrics/suite-receipt.toml holding the deliverable fingerprint (SHA-256 over the on-disk content of every file git tracks or would add under keel-cli/, .engine/, keelw, Cargo.toml and Cargo.lock), the head, the time, the counts and the outcome, and exits as cargo did; keel land, after the workspace gate and before the first push, refuses a self-build tree with no receipt, a red receipt, or a receipt whose fingerprint is not the deliverable's now, naming which and printing keel suite as the remedy; KEEL_LAND_UNTESTED=1 pushes anyway and records a land-untested obligation in the working tree, never silently; a project with no keel-cli/Cargo.toml is not a self-build and is untouched. NOT A FORK: the receipt is the only local fact that can stand between a source edit and a push, and the fingerprint over the deliverable is what makes a docs-only change need none.";
        :>> rationale = "Why fingerprint the deliverable and not HEAD: the suite is run on a working tree before the commit exists, and a docs-only or .tracking commit after a green run does not change what was tested - a HEAD-bound receipt would demand a fifteen-minute rerun for a sentence. Why machine-local: the receipt is evidence about this machine's run; CI reruns the suite itself and remains the second, independent check. Why after the gate: the override's obligation is an untriaged Issue, and recorded before the gate it would make the gate refuse the very push it authorises. Arming (D0298): two binary tests on a self-build-shaped scaffold with a bare origin - no receipt refuses naming keel suite, a green receipt for the tree lands, a docs-only commit still lands, a source commit is refused as CHANGED, a red receipt is refused as RED; the override lands and leaves exactly one land-untested obligation - plus unit tests on the receipt parse and the result counting; and this Decision's own sprint was landed through the new gate on a receipt keel suite wrote.";
        :>> consequences = "Every push from this machine follows a green full suite over the tree it carries, or leaves a tracked obligation saying it did not. Marked safety-change: land is the integration path (D0129) and cmd_land carries an audit hash, so under D0337 this waits for the human's acceptance; the code ships because it can only refuse a push that would otherwise be tested later by CI. dcLandRequiresASuiteReceipt is done on the tests above.";
    }
}
