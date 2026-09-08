// D0377 (PROPOSED — NOT YET ACCEPTED) — terseBrief: the brief's reader prose is written caveman-lite and the whole page is capped at 450 words, held by the check
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0377Accept event on sign-off.
package Decision0377 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: Read 2026-09-08 in this tree: scripts/check_templates.py check_brief (lines 212-296, no word count); the BUDGETS loop at line 125 (template-family check only); .claude/skills/asi-templates/SKILL.md section 2.1 (budgets 18/70/60/40/90/30/60); the caveman plugin's SKILL.md (levels lite/full/ultra; lite keeps articles and full sentences); the published page measured at 1334 words, 26-word headline, by stripping style, script and tags.
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0377 : Decision {
        :>> id = "34dfd20b-d684-4cbc-b645-8ee7d84f9e92";
        :>> title = "terseBrief: the brief's reader prose is written caveman-lite and the whole page is capped at 450 words, held by the check";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The human, 2026-09-08, verbatim (st109, st110): 'the decision brief is still far too verbose.' and 'make it use caveman?'. The brief contract (asi-templates section 2.1, D0355) budgets each section - 18-word headline, 70-word ask, 60-word provenance - but caps no page as a whole, and check_brief holds none of the budgets (issue413, fixed alongside). The page published that morning ran 1334 words of reader prose with a 26-word headline and passed the check clean. The caveman plugin on this machine defines a lite register: no filler, no hedging, short synonyms, full sentences kept, one idea per sentence, technical terms and numbers exact. NOT A FORK: the register is what the human asked for by name, and the ceiling is the sum of the section budgets the contract already declares (368) rounded up for figure labels.";
        :>> decision = "A brief's reader prose - every word outside style, script and the copy-for-AI digest, figure labels included - is written in the caveman-lite register and totals at most 450 words, and `python scripts/check_templates.py --brief` refuses a page over that ceiling, naming the count.";
        :>> rationale = "The reader is one person who answers the page in one read; every word past the argument is a cost they pay and a reason the page stops being read, which is the failure D0359 was written against. A ceiling on the whole page closes the gap a per-section budget leaves - sections multiply. The register is the human's stated preference and it costs no meaning: caveman-lite keeps grammar, numbers, terms and negations exact and cuts only filler.";
        :>> consequences = "scripts/check_templates.py check_brief gains the whole-page ceiling beside the section budgets; the asi-templates skill's section 2.1 names the register and the ceiling; every brief builder under scripts/exec_brief writes to it; the page published on 2026-09-08 is rewritten under it the same day. Withdrawal: a superseding Decision removes the clause and the check.";
    }
    #DerivedFrom dependency from d0377 to us099;
}
