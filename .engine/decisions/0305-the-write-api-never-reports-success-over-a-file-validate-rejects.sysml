// D0305 (status: the `status` field below is the truth) — The write API never reports success over a file validate rejects: every .sysml write parses its result before it lands, and a backslash in a prose field becomes a forward slash
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0305Accept event on sign-off.
package Decision0305 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0305 : Decision {
        :>> id = "ab68bf5e-207a-4605-8ae2-853072d44981";
        :>> title = "The write API never reports success over a file validate rejects: every .sysml write parses its result before it lands, and a backslash in a prose field becomes a forward slash";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue366 (2026-09-03): keel record decision --from with a Decision context naming C:/Program Files/ClaudeCode/managed-settings.json wrote the file, printed recorded and accepted at record time, and the next keel validate reported invalid escape sequence - every guard that reads decisions then failed. Within a minute add-task and record issue did the same over the same path. sanitize_field neutralised double quotes and newlines and not backslashes, and no writer parsed what it had produced. The write API's contract (issue255) is prose verbatim, safely; a governance record that does not parse, reported as a success, is the contract inverted.";
        :>> decision = "(1) sanitize_field replaces a backslash with a forward slash - the portable form of a path, which is what carries backslashes in this corpus; lossy for a regex, safe for a record, and a field that needs a literal backslash is code, not prose. (2) write_atomic, the one choke point every API writer of model text passes through, tokenises and parses the content of any .sysml target before writing; a result that would not parse is REFUSED with an InvalidData error naming the file and the parse message, nothing on disk changes and no temp file is left. Non-.sysml targets are untouched. (3) The three records broken on 2026-09-03 were repaired by hand the same day and stand.";
        :>> rationale = "WHY AT THE CHOKE POINT: fourteen call sites sanitise fields and more write model text (scaffold, accept, migrate); a check in each is a check some caller forgets, and the write path already funnels through write_atomic for the issue185 atomicity guarantee. WHY REPLACE RATHER THAN ESCAPE: an escaped backslash is valid SysML and renders as a backslash, but the kernel and the Rust parser disagree on escape vocabulary (issue097), so an escaped record is a portability question; a slash is a plain character everywhere. WHY REFUSE RATHER THAN REPAIR: a writer that silently rewrites what it cannot parse is a writer that changes the author's words; refusing hands the words back with the reason. COST: one tokenize+parse per write of the produced file - measured under a millisecond against a 600-line backlog - and a refusal on any caller that used to succeed over broken output, which is the point.";
        :>> consequences = "dcSanitizeMakesBackslashesSafe is delivered; issue366 resolves. A future write-path shape that produces unparsable text - a brace scanner bug, a new field, a new escape - is refused at the writer instead of found by the next validate; the D0295 class (misplaced results) would have been a refusal, not a red gate. write_atomic's doc carries the contract.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0305Accept : Test { :>> id = "e5d9ed33-b7fc-45b2-84f8-a871384f76b7"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0305AcceptR1 : TestResult { :>> id = "ed9e547c-bdc9-4e87-965b-ed4d0d8e8a8d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "936dabc"; :>> judgedAt = "2026-09-04"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
