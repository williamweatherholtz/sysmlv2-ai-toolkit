// D0335 (status: the `status` field below is the truth) — chatAcceptanceReadsBack: a delegated chat acceptance is recorded only when the human's quoted words name the decision - the read-back half of D0201 option B
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0335Accept event on sign-off.
package Decision0335 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0335 : Decision {
        :>> id = "b7ff17a9-feee-4486-b4a1-45c63a40f56d";
        :>> title = "chatAcceptanceReadsBack: a delegated chat acceptance is recorded only when the human's quoted words name the decision - the read-back half of D0201 option B";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0201 OPTION B's second clause: read-back ratification for chat approvals. Under the D0192 delegation (D0289) keel accept from an agent session records the human's acceptance when the note quotes their words - a single-quoted span of ten characters. A bare 'yes, accept it please' satisfies that, and the agent chooses which Decision the quote is attached to; nothing tied the words to the thing being accepted. Today's real answers - 'd0312 B / d0321 A', 'i do like C the best' - already name what they accept; the rule makes that the requirement.";
        :>> decision = "In the delegated arm of keel accept, the quoted spans must name the Decision being accepted - its id in any spelling (d0312, D0312, decision 312), one of its OPTION letters when it is a fork, or three consecutive words of its title or decision text - or the command refuses with the reason and writes nothing; a cited human-surface gesture (deck, console, GitHub comment, TTY) carries its own binding and is unaffected, as is the human's own terminal (D0315). Forward-only by construction: it binds new records and re-reads none.";
        :>> rationale = "Why naming rather than a second confirmation: the human already spoke once; asking them to confirm again is the hammering they refused (st065), while checking that what they said refers to what is recorded costs them nothing when they spoke plainly and blocks exactly the case D0201 named - a generic approval attached by the agent to a Decision of its choosing. Why the decision's own words count: quoting the decision text back ('yes, do exactly this') is the purest read-back there is. Why forward-only: 111 standing acceptances were recorded under the earlier rule and re-judging them is the D0129 class. Arming (D0298): read_back_names unit tests on the real quotes and on the generic yes; the acceptance_binds_to_text and disguised-fork tests exercise the delegated path end to end with notes that name the decision.";
        :>> consequences = "An agent recording a chat acceptance must quote words that name the decision; the refusal says what would. CLAUDE.md §4 states it beside the device binding (D0334). dcAttestationBindsToDevice is done with D0334; D0201 B is fully carried. STATED RESIDUAL: the check reads words, not meaning - 'd0312 no' names the decision and would record an acceptance if the agent chose to record one; the quote is still the human's and stands in the record for a reader to see.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0335Accept : Test { :>> id = "6f02d9d7-03ce-4130-82b3-7d5eac99242e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0335AcceptR1 : TestResult { :>> id = "103b30bf-881e-4232-9883-c0dbc4e21d65"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c850aff"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
