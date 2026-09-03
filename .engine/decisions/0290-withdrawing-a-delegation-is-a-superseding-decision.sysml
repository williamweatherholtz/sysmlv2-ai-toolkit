// D0290 (PROPOSED — NOT YET ACCEPTED) — Withdrawing a delegation or a standing consent is a superseding Decision with the policy line deleted in the same commit - the Decision is the record, the line is the mechanism
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0290Accept event on sign-off.
package Decision0290 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0290 : Decision {
        :>> id = "bdab4875-ece2-4afd-935f-bdff526656fe";
        :>> title = "Withdrawing a delegation or a standing consent is a superseding Decision with the policy line deleted in the same commit - the Decision is the record, the line is the mechanism";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The human asked whether D0289 had to be superseded to be ineffective going forward. It does, and the tree showed two gaps at once. First, D0289 - auto-accepted under standing consent - carried no edge to the Decision whose clause it reversed (D0178 layer 3: accept in an agent session requires a console or TTY confirmation), no edge to the delegation it rides on (D0192), and no edge to the statement it came from; the model still showed D0178's clause in force. Second, attestation-policy.toml's four TO WITHDRAW comments said 'delete the line', describing the mechanism as if it were the record - which is the Invariant 4 defect: a 'we will no longer do X' that leaves no Decision. The human then gave the authoritative word, verbatim (st075): 'D0289 supersedes layer 3 of D0178: from an agent session, my words quoted verbatim in the acceptance note are the channel, under the D0192 delegation, until a better non-local channel exists'.";
        :>> decision = "Two records and one rule. RECORDS: D0289 carries a quote receipt (d0289AcceptAttest1, D0198 shape) holding the human's words, so its acceptance is theirs and not standing consent's; and the edges it lacked - #Supersede to d0178 (scoped in the receipt to layer 3; layers 1 and 2 stand), a dependency on d0192, #DerivedFrom us076. RULE, written into the policy contract where the delegations live: withdrawing any delegation or standing consent (d0192, d0198, d0165, d0207) is a Decision that #Supersedes the granting Decision, with the policy line deleted or the kinds/roles restored IN THE SAME COMMIT. A deleted line without the Decision is a silent policy change; a Decision without the deletion is a policy that still permits what it says it withdrew.";
        :>> rationale = "WHY BOTH HALVES: the policy file is what the guards read, so the line is what makes withdrawal EFFECTIVE; the Decision is what makes it a FACT the model can compute over - governing-version, decision-follow-through and supersede traversal all read Decisions and none read TOML comments. Invariant 4 already says a non-action is a first-class Decision that supersedes; the four comments contradicted it in the one file where a reader would look for how to withdraw. WHY THE RECEIPT ON D0289: standing consent accepted a change to an acceptance control; the human's word is the right record for that, and the receipt also fixes the supersession's SCOPE, which an edge alone cannot carry - the edge says d0289 supersedes d0178, the receipt says layer 3 only. WHY NOT A CLAUSE-LEVEL EDGE: the schema has none (issue331/issue336 - a Decision's clauses are not addressable); the receipt text is the honest carrier until they are.";
        :>> consequences = "A reader asking 'is D0178 layer 3 still in force' gets the answer from the model (superseded by d0289, scope in the receipt) rather than from memory. Every future withdrawal in this policy file has a stated shape. D0289's own removal-path sentence ('delete the line') is accepted history and stays as written; this Decision governs forward. REMOVAL PATH (D0163): a Decision superseding this one, and the comments rewritten - which is the rule applied to itself.";
    }
}
