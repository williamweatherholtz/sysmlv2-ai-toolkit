// D0356 (PROPOSED — NOT YET ACCEPTED) — pushGateWithdrawn: the suite receipt stops gating the push and stays as the measurement that convicted it
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0356Accept event on sign-off.
package Decision0356 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0356 : Decision {
        :>> id = "781ca196-3499-47e8-bbfb-cddeae02f70c";
        :>> title = "pushGateWithdrawn: the suite receipt stops gating the push and stays as the measurement that convicted it";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0353 made a green full-suite receipt a precondition of every push, on the argument that CI is detective and the commit is already on main when it speaks. That argument was made against an ESTIMATED cost of about four minutes, which I stated on the published brief without measuring. The receipt records only a start time, so the number was never checked. Measured against the run log the same day, a full run takes about eleven wall minutes, and it is paid every time the deliverable moves - which on a code day is nearly every push. Against that: fourteen of the last one hundred and eleven pushes since the current release went red in CI, the remote answers in about three minutes, and two of the three most recent failures were platform faults this machine physically cannot reproduce, so the local gate would have waved them through. The owner's instruction on the published brief, verbatim: 'Keep five, revert the push gate, switch the pull on.'";
        :>> decision = "The push gate is withdrawn: keel land no longer consults the suite receipt and no push is refused for a missing, stale or red one, and the KEEL_LAND_UNTESTED override it needed is gone with it. keel suite remains and still writes the receipt - the fingerprint, the counts, the outcome and the wall time of the run - because that measurement is what answered the question, and 'was this exact tree tested' stays worth knowing even when nothing refuses on the answer. CI remains the check a push must survive. NOT A FORK: the owner instructed the withdrawal after reading the measured cost.";
        :>> rationale = "Why withdraw rather than tune: the cost is the whole run and cannot be trimmed without splitting the suite, which is a larger change than the gate is worth; and the catch rate is not a tuning parameter but a property of what fails here - platform faults dominate, and a Windows machine cannot see them. Why keep the receipt: it is the only thing that knows what a full run costs, it is what convicted the gate, and a later control may want the answer. Why the tests change rather than disappear: the withdrawal itself is now pinned - a test asserts that a push is NOT refused for a missing or stale receipt - so the gate cannot creep back unnoticed. Arming (D0298): push_gate_is_withdrawn holds both halves; keel suite --help now prints usage instead of running the suite, which is how the 185-second discovery of that defect ends.";
        :>> consequences = "A push is as fast as it was before yesterday. The suite is run deliberately rather than under compulsion, and the receipt still says what the last run covered. This reverses a recommendation I made this morning and defended on the same page a few hours later; the reversal is recorded here rather than quietly implemented, because the measurement that changed it is the interesting part.";
    }
    #Supersede dependency from d0356 to d0353;
}
