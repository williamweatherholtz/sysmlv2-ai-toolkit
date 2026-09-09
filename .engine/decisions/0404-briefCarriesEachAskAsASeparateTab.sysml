// D0404 (status: the `status` field below is the truth) — briefCarriesEachAskAsASeparateTab: the decision brief makes each ask a separate tab, with the verdict, chips and provenance shared outside the tabs
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0404Accept event on sign-off.
package Decision0404 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0404 : Decision {
        :>> id = "5a749a5d-b7b7-4f69-968c-308ca3e804e1";
        :>> title = "briefCarriesEachAskAsASeparateTab: the decision brief makes each ask a separate tab, with the verdict, chips and provenance shared outside the tabs";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The brief published at 29fb76c stacked three asks down one page: headline, one verdict paragraph naming all three, then each ask's prose, two exhibits and options in sequence. The human's direction on reading it, 2026-09-09: 'for the decision panel, make each ask a separate tab.' The surface had been tabbed once before, on 2026-09-06, and was un-tabbed because those tabs partitioned one argument IBIS-style - position, then pro and con - and afforded only accept or defer; that verdict was about what each tab HELD, not about tabs. Partitioning by ask keeps the QOC shape whole inside every tab. NOT A FORK: the human named the shape; the only choice left was what stays outside the tabs, and the answer follows from what a reader needs before choosing a tab - the answer, the count waiting, and where the numbers came from.";
        :>> decision = "The decision brief renders one tab per ask, role=tab paired with role=tabpanel, each panel holding that ask's whole argument - its prose, its two logic exhibits, its members table when it names a set, and its response options - with the first tab selected on load and the tab label being the ask's short name. The headline, the verdict paragraph, the waiting chip, the copy-for-AI controls and the provenance strip sit outside the tabs, shared by every ask. scripts/check_templates.py --brief refuses a brief whose tab count differs from its panel count or from the number of asks it declares, and one where any tab but one starts selected; the copy-for-AI digest walks every panel, selected or not, so a pasted answer set is complete.";
        :>> rationale = "One tab per ask gives the reader one decision at a time with everything that decides it in view, which is the reading the brief has always promised and a stacked page delivered only by scrolling past the other two. Sharing the verdict and provenance keeps answer-first: a reader who opens no tab still has the recommendation and the tree it was computed against. The 2026-09-06 objection was to tabs that split an argument; tabs that split asks leave every argument intact.";
        :>> consequences = "The asi-templates skill section 2.1 states the tabbed shape and the reconciliation with the 2026-09-06 un-tabbing; the builder emits the tab strip and panels; the checker counts them. The page ceiling of d0377 is unchanged by this Decision; whether it is measured per tab is its own question, recorded apart.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0404Accept : Test { :>> id = "ac557171-ced9-41b1-964a-dfb6dcc14d84"; :>> method = VerificationMethod::confirmation; :>> procedureText = "recorded from chat - their words, verbatim: “for the decision panel, make each ask a separate tab.”"; }
    part d0404AcceptR1 : TestResult { :>> id = "b3eecfbc-b20b-4250-bb60-e0ad1321411b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0d5be97"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
