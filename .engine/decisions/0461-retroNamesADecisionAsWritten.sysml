// D0461 (PROPOSED — NOT YET ACCEPTED) — retroNamesADecisionAsWritten: the retro-backlog guard reads D0388 as naming d0388 - a Decision named the way every document names one is a named item
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0461Accept event on sign-off.
package Decision0461 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0461 : Decision {
        :>> id = "fbbe057c-43dc-4e36-9830-b8f923d97813";
        :>> title = "retroNamesADecisionAsWritten: the retro-backlog guard reads D0388 as naming d0388 - a Decision named the way every document names one is a named item";
        :>> createdAt = "2026-09-12";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The retro-backlog guard (D0131) satisfies itself two ways: the commit adds a tracked item the retro names, or the retro says why none is needed AND names an existing item that carries the finding (D0293, issue364). Its named_items reader matched dc-tasks, issueNNN and the Decision form d0NNN - lowercase only - while CLAUDE.md, every commit message, every DoD and the guard's own refusal text write a Decision as D0NNN. So sprint 627's retro, justified by 'already tracked by D0388 and its probe rule', was refused as naming no item at all, and sprint 626's identical phrase passed only because the same sentence also named an issue (issue424). Today, 2026-09-12, sprint 687's first retro was refused the same way: it named D0131 and D0337 and the guard read nothing. A justification the guard cannot read is a justification nobody checks - the issue364 class with a different mask.";
        :>> decision = "NOT A FORK: issue424's DO clause names the change and forbids the alternative. named_items in guards.rs gains a fourth needle, D0 followed by a digit at a word boundary, and returns the match with its first letter lowered - d0388 - so the exists check hits the tree by the item's real name. Only the Decision form folds: Issue and DC in prose stay words, not items, and ID0388 is no boundary. The refusal text quotes the forms it accepts in the case it accepts them: `issueNNN`, `dcTaskName`, `d0NNN` (or `D0NNN`). Two unit tests hold it: named_items reads the uppercase form as the real name and leaves the lookalikes unmatched; and the sprint 627 case constructed both ways - the same retro whose only named item is D0388 passes when d0388 exists and fails when it does not, the refusal quoting the accepted forms.";
        :>> rationale = "The guard is the control that makes a retro's justification checkable; a form it cannot read turns the control into prose that happens to be present. Matching the documented spelling costs one needle and one case fold, and folding only the Decision prefix keeps the boundary that stops prose lookalikes from counting. Widening the whole match to case-insensitive would accept Issue 42 and DC as items, which issue424's DO explicitly refuses.";
        :>> consequences = "Guard source changes under D0209 clause 2, so this Decision rides the commit as its #ProspectiveChange; process-change (D0337): held proposed for the human's word, the code lands with it as D0435 did. Resolves issue424 through dcRetroNamesDecisionsAsWritten (sprint 688). guards.md's retro-backlog row names the accepted forms. STATED: the guard still cannot judge that the named Decision COVERS the finding - form, not relevance (D0293) - and a retro may now discharge itself by naming any existing Decision in either case; the sitting review reads relevance.";
    }
    #DependsOn dependency from d0461 to d0131;
}
