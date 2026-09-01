// execSummary — AI feedback arrives as an interactive decision page, never terminal prose.
//
// The issue245 class, closed for this project's crown jewel: the skill, the template and the
// browser test suite existed while NO process definition did — so the kit appeared in no catalogue,
// was enforced by nothing, and could travel to no other project. This file is what makes it a UNIT
// (dcKickoffReady increment 5; the sysmlv2-ai-toolkit readiness plan, st033).
package ProcessExecSummary {
    private import EngineElement::*;
    private import EngineProcess::*;

    action execSummary : Process {
        :>> id = "7e4c2b91-58a3-4f6d-b2c7-9d0e1f2a3b4c";
        :>> title = "Exec-summary decision pages";
        :>> purpose = "Deliver findings, proposals and decision requests as an interactive HTML page - recommendation-first tabs, one decision per tab with steelmanned choices and costs, a reasoning chain with named budgets, and a Copy-for-AI digest that returns the reader's selections as a decision record.";
        :>> cadence = Cadence::perItem;
        // APPLIES-WHEN: a turn's deliverable is feedback a human will react to - findings, options,
        // a sign-off request - and terminal text would bury the decision
    }

    action esFill : ProcessStep {
        :>> id = "7e4c2b91-58a3-4f6d-b2c7-9d0e1f2a3b4d";
        :>> title = "Fill the template; keep the machinery";
        :>> triggerCondition = "A deliverable is feedback the author will react to.";
        :>> actionText = "Start from templates/exec-summary/exec-summary.html - the template is a project asset this process consumes and never owns; improvements go to the template file, not into copies. Keep the tested machinery (tabs JS, sync-first copy, theme tokens, viewport meta, 44px targets); replace the content. One tab per DECISION (3-6); a tab that asks nothing is an appendix, not a tab.";
        :>> producedArtifact = "A filled page, machinery intact.";
        :>> owner = Owner::ai;
    }

    action esSteelman : ProcessStep {
        :>> id = "7e4c2b91-58a3-4f6d-b2c7-9d0e1f2a3b4e";
        :>> title = "Steelman every choice; chain the reasoning; hold the budgets";
        :>> triggerCondition = "A tab exists.";
        :>> actionText = "Each choice card carries its STRONGEST CASE and its honest COST - a card without its best case is a strawman, one without a cost is unanalyzed, and either makes the recommendation unarguable. The recommended option comes first, chipped, NEVER pre-selected: a pre-selection fabricates a decision the reader never made. Each tab carries the reasoning chain - stake, deciding criterion, why-this-wins, what-would-flip-it, confidence with the unverified thing named - inside the enforced word budgets: an executive summary that grows paragraphs stops being one.";
        :>> producedArtifact = "Choice cards + reasoning chain, within budget.";
        :>> owner = Owner::ai;
    }

    action esVerify : ProcessStep {
        :>> id = "7e4c2b91-58a3-4f6d-b2c7-9d0e1f2a3b4f";
        :>> title = "Run the browser suite BEFORE delivering";
        :>> triggerCondition = "The page is filled.";
        :>> actionText = "EXEC_SUMMARY_FILE=<page> .venv/Scripts/python -m pytest tests/exec_summary -q. The suite is real browsers: one-click copy puts the digest on the clipboard (Chromium), one-tap works at iPhone scale (WebKit), targets are 44px, structure order holds, budgets hold, both themes render, the page is self-contained. The copy control and the tabs are BEHAVIORAL claims - the suite exists so they are verified, not asserted, and an untested page is not delivered.";
        :>> producedArtifact = "A 100%-passing suite run against the filled page.";
        :>> owner = Owner::ai;
    }

    action esDeliver : ProcessStep {
        :>> id = "7e4c2b91-58a3-4f6d-b2c7-9d0e1f2a3b50";
        :>> title = "Deliver; the returned digest is the decision record";
        :>> triggerCondition = "The suite passes.";
        :>> actionText = "Send the page. The Copy-for-AI digest the reader pastes back carries their selections as checkbox markdown plus their notes - a STATEMENT about specific claims, which is what an acceptance gate can honestly record (the keel intake discipline: their words verbatim, then the reading). Provenance rides the footer: who generated it, when, from what.";
        :>> producedArtifact = "A delivered page; on return, a recordable decision digest.";
        :>> owner = Owner::ai;
    }

    first esFill then esSteelman;
    first esSteelman then esVerify;
    first esVerify then esDeliver;
}
