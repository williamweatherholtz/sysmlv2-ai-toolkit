// D0427 (status: the `status` field below is the truth) — gestureWordCheckBindsTheAgent: the gesture-word-only refusal stays a refusal - it binds the agent's act of typing a citation it did not observe, so D0426's own criterion puts it outside D0423's dissolution
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0427Accept event on sign-off.
package Decision0427 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0427 : Decision {
        :>> id = "6b7771a0-1b2b-4346-b895-7c2619626619";
        :>> title = "gestureWordCheckBindsTheAgent: the gesture-word-only refusal stays a refusal - it binds the agent's act of typing a citation it did not observe, so D0426's own criterion puts it outside D0423's dissolution";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0423 dissolved three delegated-record checks into WARN lines: fewer than ten characters, no read-back, and a gesture word as the only evidence. Implementing it against D0426's criterion - a control gatekeeps a human when its refusal falls on an act a PERSON performs - the third check does not meet the criterion. The words 'approved at the console' typed into a delegated note are the AGENT's act: the human's chat channel is --words, which never passes through this check, and a console or deck record is self-recorded and exempt (checks.rs, the substance rule's recorder == judge exemption). What the check refuses is an agent asserting it saw a gesture it did not observe - the class D0411/issue426 recorded, where the citation is written by the surface that observed it and never typed.";
        :>> decision = "NOT A FORK: this applies the criterion D0426 states. The gesture-word-only check in the delegated accept/reject path keeps refusing, with a `refused` ledger line (control gesture-word-typed) so its every firing is evidence. The other two D0423 checks - short words and read-back - are WARN lines in the record as D0423 says. This reverses the one clause of D0423 that named the gesture-word check among the checks that report in the note; the rest of D0423 stands.";
        :>> rationale = "A control is kept or dissolved by whose act it falls on, not by how it irritates (D0426). The human never types a gesture citation; an agent under delegation is the only actor who reaches this refusal, and its firing is exactly the fabrication D0198 names - a receipt for a gesture that the recorder did not observe. Dissolving it would have made D0411's citation rule unenforceable on the one path where an agent writes the note. The ledger line makes the choice falsifiable: if the control is ever seen to fall on a human's act, that line is the evidence for a Decision that dissolves it.";
        :>> consequences = "keel accept / keel reject under delegation: --words records as given (a WARN for short or non-read-back words, D0423); a note whose only evidence is a gesture word is refused and the refusal is a ledger fact. The test gesture_citation_is_the_commands keeps its refusal case. D0423's title still lists three checks; its text is read with this clause reversed, and governing-version shows both.";
    }
    #SupersedeClause dependency from d0427 to d0423;
    #DerivedFrom dependency from d0427 to us107;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0427Accept : Test { :>> id = "ce45e17e-a203-4f2e-b778-e2fde24500ea"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0427 (D0201 B); recorded as given (D0423)."; }
    part d0427AcceptR1 : TestResult { :>> id = "45c19a5d-ff8d-4a23-bf1a-6f22f90c286c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
