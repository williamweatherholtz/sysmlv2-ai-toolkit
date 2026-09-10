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
}
