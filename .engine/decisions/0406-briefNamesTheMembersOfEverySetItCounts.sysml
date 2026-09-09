// D0406 (status: the `status` field below is the truth) — briefNamesTheMembersOfEverySetItCounts: a brief that says it collapses, folds, merges or removes a set names what is being collapsed, and the names reconcile with the count
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0406Accept event on sign-off.
package Decision0406 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0406 : Decision {
        :>> id = "e9e0f108-c8ef-4eac-a206-529d3c4840b4";
        :>> title = "briefNamesTheMembersOfEverySetItCounts: a brief that says it collapses, folds, merges or removes a set names what is being collapsed, and the names reconcile with the count";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The brief at 29fb76c described option A of the verb-count ask as 'five collapses' and its downstream exhibit as 'collapse five families ... 70 to 20', naming none of the commands that would fold and typing the 20 - the arithmetic of the Decision's own text gives 35. The human, 2026-09-09: 'how can i gauge that if you don't tell me what you're collapsing? 'collapsing 5 reads/writes' is irrelevant - i want to know *what* is being collapsed. totally avoiding the root of the issue.' A count of things the reader must judge, without the things, cannot be judged; the contract's own table rule already said a reader who must verify individual values needs a table. NOT A FORK: the human named the defect and the remedy in one sentence.";
        :>> decision = "Whenever a brief's reader prose applies an aggregate verb - collapse, fold, merge, consolidate, rename, remove, retire - to a counted set, the page carries a members table for that set, marked data-members with the count it reconciles to, listing every member by name with what becomes of it, and every number the aggregate implies - what remains after it - is computed from those members in the builder, never typed. scripts/check_templates.py --brief refuses an aggregate claim on a page with no members table, and a members table whose row count differs from its declared count.";
        :>> rationale = "The reader is judging whether a proposal is correct, and the unit of correctness is the member: which command, folded into which router, with which callers. A count hides exactly the thing that could be wrong, and a typed remainder can be wrong on its own - as 20 was. Naming the members is what lets the reader find the one that should not move, which is the whole reason the ask is theirs.";
        :>> consequences = "facts.py emits the live top-level verbs by family with their effect; the verb-count builder holds the family-to-router mapping quoted from the Decision under judgment and computes the members and the remainder from the facts; the checker gains the aggregate-without-members and count-mismatch refusals with known cases in its self-test; the asi-templates skill section 2.3 states the members table beside its existing table rule.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0406Accept : Test { :>> id = "5a992699-a2eb-44ab-a88f-72bbc51ea9cb"; :>> method = VerificationMethod::confirmation; :>> procedureText = "recorded from chat - their words, verbatim: “how can i gauge that if you don't tell me what you're collapsing? 'collapsing 5 reads/writes' is irrelevant - i want to know *what* is being collapsed. totally avoiding the root of the issue.”"; }
    part d0406AcceptR1 : TestResult { :>> id = "9d012d49-b3de-41d7-bf1d-ce49bde89e9f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0d5be97"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
