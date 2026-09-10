// Intake for 2026-09-10 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260910 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st115 : Statement {
        :>> id = "ce42cbb2-71d2-490d-9e57-36115b44ffcc";
        :>> title = "guards that handcuff the human and do little to agents; build evidence of how processes are subverted today, into STPA, out to needs and requirements";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> text = "I thought we got rid of these guards that handcuff me and do very little to AI agents. are there indications that agents have attempted to accept decisions under my authority? if not, then this guard is moot and should be removed along with others like it.\n\nregardless - accept D0421 and D0422. but users like me shouldn't be the entities strictly regulated here.\n\nbuild evidence around the ways processes are subverted today and let's with to address empirical issues. integrate into STPA and spring out implementation asking with upstream needs, use cases, and requirements";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-10"; :>> channel = StatementChannel::chat;
    }

    part us105 : UserStory {
        :>> id = "a3cf1f20-63a3-4f88-a5ed-533fd0129098";
        :>> title = "Controls bind the AI, never the human, and each is kept on evidence";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> asA = "human decider working through an AI agent";
        :>> iWant = "every control that refuses an act of mine to show the incidents it prevented, and to be removed when it shows none";
        :>> soThat = "the process regulates the actor that has actually subverted it, and I am not the one handcuffed";
        :>> implication = ImplicationKind::need;
        :>> triageNote = "the human's direction 2026-09-10; the record shows 453 AI-recorded acceptances that all passed the gate, eight refusals of the human (issue397), and no instrument that could have counted an attempt (issue445)";
    }
    #DerivedFrom dependency from us105 to st115;

    part us106 : UserStory {
        :>> id = "6ab6d66d-82ce-4327-9a19-e8b02848a64e";
        :>> title = "Subversion evidence feeds STPA and comes out as needs, use cases and requirements";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> asA = "human decider";
        :>> iWant = "the ways the process is subverted today measured from the record and carried into the STPA run, so the asks that come out of it trace to a need, a use case and a requirement";
        :>> soThat = "we address empirical issues, not hypothesised ones";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "no existing process runs census -> STPA evidence -> Business workflow end to end; the process is the deliverable";
    }
    #DerivedFrom dependency from us106 to st115;

    part st116 : Statement {
        :>> id = "ff109db8-86a2-4af6-89f0-377302087870";
        :>> title = "any unabused guard that gatekeeps me is reconsidered or dissolved; AI check-lists instead of solving; farm management work to haiku agents that verify against keel";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> text = "accept d0424 and d0423.\n\nand you're kind of missing the point - any guards that have not been abused that gatekeep me should be reconsidered or dissolved outright. it's not this one edge case that's the issue, really.\n\nand I feel like AI spends so much time check listing that it doesn't have any brain power to burn on actually solving problems. all the test is in terms of ''I did X process and the guards worked great'' but the actual meat of the content seems to have gone downhill. fire the next several prompts, let's try farming out the management work to other haiku agents that can independently check and verify work against keel.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-10"; :>> channel = StatementChannel::chat;
    }

    part us107 : UserStory {
        :>> id = "33a7e9d6-53ba-4460-866f-b91426f8b550";
        :>> title = "The primary agent solves; independent subagents check, verify and record against keel";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> asA = "human decider";
        :>> iWant = "the agent's effort on the problem and the ceremony, checking and recording done by separate agents that verify the work against keel independently";
        :>> soThat = "the meat of the content stops going downhill and the checks are independent of the one who did the work";
        :>> implication = ImplicationKind::process;
    }
    #DerivedFrom dependency from us107 to st116;

    part st117 : Statement {
        :>> id = "1e44adee-170e-45f0-9958-280bb11447f6";
        :>> title = "GH#70 [bug] add-task rejects --method analyze (the schema's member) and accepts 'analysis', writing an enum value keel validate rejects";
        :>> createdAt = "2026-09-10"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed (keel 0.3.1).** `keel add-task --method analyze` is refused: `error: invalid method 'analyze'`. `keel add-task --method analysis` is accepted and writes `:>> method = VerificationMethod::analysis;` — which `keel validate` then flags as a semantic diagnostic: `hint: valid members: analyze, confirmation, critique, demo, inspect, test`. So the CLI's method vocabulary and the schema's `VerificationMethod` enum disagree on the one member; the write path accepts the invalid spelling and rejects the valid one.\n\n**Expected.** `add-task` validates `--method` against the schema enum it writes (`analyze`), or maps the CLAUDE.md prose form `analysis` to it. As it stands the sanctioned write path produces a record the validator rejects, and the only fix is a hand edit of the `.sysml`.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-09).\n";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-09"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/70";
        :>> sourceTrust = SourceTrust::untrusted;
    }
}
