// D0405 (status: the `status` field below is the truth) — briefStatesNoImplementationDuration: the brief never tells the reader how long things should take to implement - cost is what changes and what breaks, never time
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0405Accept event on sign-off.
package Decision0405 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0405 : Decision {
        :>> id = "b637c758-d547-4e7e-a1e5-033dbedc6181";
        :>> title = "briefStatesNoImplementationDuration: the brief never tells the reader how long things should take to implement - cost is what changes and what breaks, never time";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The brief at 29fb76c priced option A of the verb-count ask as 'five collapses, a day each' - an effort estimate, my judgment from one earlier sprint, standing in the sentence where the reader looks for what the option does. The human, 2026-09-09: 'don't tell me how long things should take to implement - i really don't care. i care about correctness.' The contract's courses row asked for 'a quantified cost', and a duration is the easiest quantity to type and the one the reader cannot check. NOT A FORK: the human said what they do not want; the clause removes it and says what stands in its place.";
        :>> decision = "The reader prose of a decision brief states no implementation duration - no hours, days, weeks, sprints or 'a day each' - anywhere the reader reads, figure labels and tables included. An option's cost is stated as what it changes, what it breaks and what it forecloses, each as a named, countable thing; scripts/check_templates.py --brief refuses a page whose reader prose carries a duration phrase, naming it. Dates remain: 'since 2026-09-01' is a fact, 'three days' is an estimate.";
        :>> rationale = "A duration is a forecast the reader has no way to check and the author has no record to compute from, so it is the one number on the page that is typed rather than measured - the exact class section 2.2 forbids. What an option changes is a fact of the tree and can be counted, named and verified; that is the cost a reader judging correctness can use.";
        :>> consequences = "The asi-templates skill's courses row reads 'places-to-change and what breaks' in place of 'a quantified cost'; the checker gains the duration refusal with a known-positive and a known-negative case in its self-test; the standing brief is rebuilt without the estimate. Decision records keep their COST clause (guard 48) - this binds the reader surface, not the record.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0405Accept : Test { :>> id = "f6bd6e16-f6e8-460b-bd4b-4ce516c33f6f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "recorded from chat - their words, verbatim: “don't tell me how long things should take to implement - i really don't care. i care about correctness.”"; }
    part d0405AcceptR1 : TestResult { :>> id = "73419bc4-06f8-4d4a-b0e7-9d8ccbbf78aa"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0d5be97"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
