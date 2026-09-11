// D0407 (status: the `status` field below is the truth) — briefCeilingIsMeasuredPerTab: the 450-word ceiling is measured over what the reader sees at once - the shared frame plus one tab - not over every tab summed
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0407Accept event on sign-off.
package Decision0407 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0407 : Decision {
        :>> id = "0fcf4a64-6803-43ec-92fc-a1d24e609331";
        :>> title = "briefCeilingIsMeasuredPerTab: the 450-word ceiling is measured over what the reader sees at once - the shared frame plus one tab - not over every tab summed";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "d0377 capped the whole page at 450 reader words so that a brief stays terse; it was written for a page read top to bottom. D0404 makes each ask its own tab, and D0406 makes an ask that counts a set name its members - the verb-count ask alone names 67 commands. Five asks wait today. Measured on the built tabbed page (scripts/exec_brief/build_2026_09_09_five_asks_tabbed.py, checked by scripts/check_templates.py --brief): 927 words summed over five tabs, the shared frame 144 and the largest tab 236 more, so every view is under 450 while the sum is refused. The two clauses cannot both hold on a tabbed page that names its members, so one has to be said again. NOT A FORK: the ceiling's own rationale is the reader's attention at once, and a tab is what the reader has in front of them at once; the alternative - keep the summed ceiling - was measured as unable to carry the four asks correctly.";
        :>> decision = "The 450-word ceiling of d0377 is measured, for a tabbed brief, over the shared frame plus each tab in turn - headline, verdict, chips, provenance and that one panel, figure labels and table cells included - and every tab must pass; the register clause of d0377 is untouched. scripts/check_templates.py --brief measures each tab against the ceiling and names the tab that is over. This reverses the 'whole page' wording of d0377 only; d0377 stands.";
        :>> rationale = "What a reader takes in at once is the frame and one panel; that is the quantity terseness protects, and it is unchanged by how many asks wait. A summed ceiling over tabs would make a brief with four asks terser per ask than one with two, which measures the queue, not the writing.";
        :>> consequences = "The checker's ceiling reading gains the per-tab mode, active when this Decision is accepted and the summed mode otherwise (the D0337 shape d0377 used); the asi-templates skill 2.1 states the per-tab measurement. Until accepted, a tabbed brief that names its members and exceeds 450 summed words is refused and cannot be published, and the standing page stays as it is.";
    }
    #SupersedeClause dependency from d0407 to d0377;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0407Accept : Test { :>> id = "e3c2d965-73b4-4229-b99e-393f88f5c36d"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0407 (D0201 B); recorded as given (D0423)."; }
    part d0407AcceptR1 : TestResult { :>> id = "80707707-bfab-4815-8e19-dd6582c52841"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
