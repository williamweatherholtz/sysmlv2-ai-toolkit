// D0442 (status: the `status` field below is the truth) — suiteReceiptIsStampedAtTheWrite: the suite receipt's at is the moment it was written and it carries seconds, the touched receipt's two words with the touched receipt's meaning (issue472)
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0442Accept event on sign-off.
package Decision0442 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0442 : Decision {
        :>> id = "e3cfb216-05df-46cd-b621-2cc13a7305ae";
        :>> title = "suiteReceiptIsStampedAtTheWrite: the suite receipt's at is the moment it was written and it carries seconds, the touched receipt's two words with the touched receipt's meaning (issue472)";
        :>> createdAt = "2026-09-11";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Two receipts sit beside each other under .keel/metrics and a verifier reads both under one rule: the touched receipt (D0421) stamps at when the file is written - the end of a done run, the start of a running stub (D0387) - and carries seconds, the run's wall clock. The suite receipt (D0356) stamped at with the launch epoch on both the stub and the done receipt, and carried no duration. A verifier told to check at against its own launch epoch therefore learned only that the receipt belonged to a run launched after it - the run's identity - and nothing about whether that run finished, and the test-verify skill (D0438) had to teach a workaround, the receipt file's mtime minus at, to derive what the touched receipt states in a field. issue472 records the divergence; the D0388 probe pair for it is a done receipt rendered from started=100 and finished=700 and the running stub rendered at its start.";
        :>> decision = "The suite receipt says what the touched receipt says with the same words: at is the moment the receipt was WRITTEN - the end of a done run, the start of a running stub - and seconds is the run's wall clock, 0 on a stub. suite.rs stamps the done receipt through one pure function, done_receipt(started, finished, ...), whose at is finished and whose seconds is finished minus started; the stub keeps at = started. A receipt from before this carries no seconds and parses as 0. The test-verify skill's SUITE clause drops the mtime workaround and reads seconds from the receipt, and the executive brief's suiteWallMinutes fact reads seconds when present and falls back to the mtime derivation only for a receipt that predates the field.";
        :>> rationale = "One reader, two files, one meaning per word. The verifier's rule - at is after my launch, so the run finished after I launched it - is true of the touched receipt and was false of the suite receipt by exactly the run's length, which is the quantity the reader most wants. Stamping at write also matches the file's own semantics: the fingerprint is taken after the run and the counts are the run's result, so the one field that named the launch was the odd one. The wall clock as a field replaces a derivation from the filesystem that a copied or re-saved file breaks silently.";
        :>> consequences = "Process-change: the test-verify skill's procedure text changes (D0337 holds this proposed for the human). keel-cli/src/suite.rs gains seconds on Receipt, done_receipt, and the pair; scripts/exec_brief/facts.py reads seconds first. No guard reads the suite receipt (D0356: it gates nothing), so no control's verdict moves. A stale-receipt check that compared at to a launch epoch is strictly stronger after this: it now proves completion as well as identity. NOT A FORK: the touched receipt's convention was already the documented one and the verifier already read it; the only question was which file changed, and the one whose field misled the reader is the one that changes.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0442Accept : Test { :>> id = "9000a527-d5e6-4d21-9805-92c25acc2cc2"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0442 (D0201 B); recorded as given (D0423)."; }
    part d0442AcceptR1 : TestResult { :>> id = "6b0d78a2-6edf-41de-8547-047223883322"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
