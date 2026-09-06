// D0337 (PROPOSED — NOT YET ACCEPTED) — consentScopedToProcesses: standing consent auto-accepts only Decisions within the existing processes; a Decision that changes the process or enforcement surface stays proposed for the human
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0337Accept event on sign-off.
package Decision0337 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0337 : Decision {
        :>> id = "b53a4fa4-c099-4544-b023-f43835fee73b";
        :>> title = "consentScopedToProcesses: standing consent auto-accepts only Decisions within the existing processes; a Decision that changes the process or enforcement surface stays proposed for the human";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The human's answer to D0324 on 2026-09-05, verbatim: 'd0324 - standing consent is scoped only to the existing processes under which it was promulgated'. Not one of the four lettered options: not a revision stamp (B), not a lapse on every guard change (C), not an expiry (D). Consent given under a set of processes covers work within them and does not cover rewriting them - a Decision that adds or changes a process, a guard, a hook, a workflow or a contract is a change to the ground the consent stands on. Since D0291 moved acceptance to record time, 111 Decisions auto-accepted, and those carrying a process-change or safety-change marker were among them, this session's included.";
        :>> decision = "At record time, a Decision carrying a process-change or safety-change marker is OUTSIDE standing consent: keel record decision records it proposed and says why, and it is accepted only by the human - their quoted word (D0289, with D0335's read-back), the console or deck (D0334), or their terminal (D0315); a Decision with no marker - work within the existing processes - auto-accepts under D0207 as before, subject to the fork rules (D0291, D0322). MY READING OF THE HUMAN'S WORDS, stated so it can be corrected: 'the existing processes under which it was promulgated' is read as the process and enforcement surface as a whole, so the boundary is the marker the keystone lock already requires for changing that surface (D0209) - not the calendar date of the grant, and not the identity of individual process files.";
        :>> rationale = "Why the marker is the boundary: the keystone lock already makes every change to the process or enforcement surface declare itself with exactly this marker, so the scope the human named has a machine-readable edge that needs no new fact - and it is the same edge a reader uses to find such changes. Why this Decision is itself proposed: it carries the marker, so under its own rule it is the human's to accept; recording it accepted by consent would be the thing it forbids. Why the reading is stated: the human's sentence admits a narrower reading (files that existed on 2026-08-23) and a wider one (any Decision touching a process); the marker is the reading that makes the rule enforceable at record time, and the human can say otherwise in a word. Arming (D0298): tests/disguised_fork_stays_proposed.rs gains the case - a process-change Decision on a scaffold with standing consent is recorded proposed with no acceptance result and the output says OUTSIDE standing consent; an unmarked Decision still auto-accepts.";
        :>> consequences = "This session's autonomy narrows where the human said it should: every guard, hook, process, workflow and contract change now waits for their word before it is accepted, which is roughly one Decision per sprint at today's rate; the decision-surfacing page carries them. issue342 is resolved by dcConsentIsScopedAndExpires. STATED: the marker Decisions accepted under consent before today stand as recorded (D0129); this rule binds forward. CC-R3 (ccRecordConsentScoped) binds to this control at the next stpa-self run.";
    }
}
