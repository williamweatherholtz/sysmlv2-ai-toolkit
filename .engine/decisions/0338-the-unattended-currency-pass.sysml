// D0338 (PROPOSED — NOT YET ACCEPTED) — scheduledCurrency: keel currency is the unattended pass - pull as verbatim Statements, library sync, drift - and a declared removable schedule runs it, inert until this Decision is accepted
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0338Accept event on sign-off.
package Decision0338 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0338 : Decision {
        :>> id = "f7c41c3d-53da-483f-b23f-52772c08f258";
        :>> title = "scheduledCurrency: keel currency is the unattended pass - pull as verbatim Statements, library sync, drift - and a declared removable schedule runs it, inert until this Decision is accepted";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "nUnattendedCurrency's unattended half (srUnattendedPullIsIdempotent): a project's currency - its open issues as Statements, its library, its drift - kept current without a session. The pieces existed as three commands; nothing ran them together, and a schedule that runs three commands has three places to be silently wrong. D0264 bounds what an unattended pass may do: record words, never triage. Verified live before the schedule: the first pass over this repository ingested six new issues (GH#54-59) as untrusted Statements; the second pass over the same open issues ingested nothing and reported 18 already recorded - identical Statements, no duplicate ids, the skip count said aloud.";
        :>> decision = "keel currency [ROOT] --repo O/N --by ACTOR runs the three passes and prints one report with one exit code (0 all ran, 1 a pass failed, 2 a pass could not start): github-pull recording verbatim Statements idempotently on the issue URL with the trust tier on each and no triage; library sync stating an unreachable remote as stale rather than up to date; the library drift lines read back; and .github/workflows/currency.yml is the declared, removable schedule (daily, plus manual dispatch) that runs the one command as githubRecorder, gates the tree and commits only .tracking/intake - a file whose first step reads this Decision's acceptance from the tree and, until d0338AcceptR1 exists, exits saying the schedule is declared but INERT, because a change to the process surface is outside standing consent (D0337) and this Decision waits for the human's word.";
        :>> rationale = "Why one command: the report is the schedule's only output, and one command's output is one place to read. Why the schedule is inert until accepted: D0337 was the human's answer an hour ago, and a scheduled job committing to main is precisely the kind of process change it names; building the file now and letting the acceptance arm it means the human's yes is the switch and nothing runs on my word. Why githubRecorder: the recorder of an utterance is never defaulted (D0129), the actor already exists as the channel's recording identity, and the speaker stays the GitHub login. Why commit only .tracking/intake: words are all the pass may write. Arming (D0298): currency::summary unit test (verdict per pass, worst code wins, the no-triage sentence); the live double run above is the idempotency evidence srUnattendedPullIsIdempotent asks for, recorded on the DoD result.";
        :>> consequences = "Until the human accepts D0338 the workflow runs daily and does nothing but say so. On acceptance it pulls, syncs, reports and commits new Statements, which a session then triages under the github-intake process. dcScheduledCurrency is done. STATED: a library unreachable from the runner is reported stale by the pass, never up to date, which is the requirement's second clause read back from library::cmd_sync.";
    }
}
