// Intake for 2026-08-26 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260826 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st021 : Statement {
        :>> id = "c22e3a04-5eb1-4e38-b7d8-336a7071d0bf";
        :>> title = "is there really no write api for statements or user stories";
        :>> createdAt = "2026-08-26"; :>> createdBy = "claudeFable5";
        :>> text = "project claims there is no write api surface for statements or user stories.  is that accurate?  fill gap if so";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-26"; :>> channel = StatementChannel::chat;
    }

    part us026 : UserStory {
        :>> id = "918ab15f-0dec-495e-a3d3-b7d2682a01dd";
        :>> title = "intake has no write path, so every recorded utterance was hand-edited";
        :>> createdAt = "2026-08-26"; :>> createdBy = "claudeFable5";
        :>> asA = "supervisor whose words are the source of every Need";
        :>> iWant = "a write API for Statements and UserStories, like decisions and issues have";
        :>> soThat = "the verbatim rule is enforced by the path rather than by my care";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process, and the gap is real: keel record covers decision and issue only. It matters more than a missing convenience because D0216 makes the verbatim Statement the precondition for every Need, so the one record type with the strictest rule had the weakest path - hand-edited, no actor validation, no lock, no refusal on a defaulted date, and nothing stopping a paraphrase typed from memory being stored as verbatim.";
    }
    #DerivedFrom dependency from us026 to st021;

    part st022 : Statement {
        :>> id = "52ecb6fe-9fb0-4eb8-83e1-be7316508333";
        :>> title = "tab headings should flag what needs me, not count what exists";
        :>> createdAt = "2026-08-26"; :>> createdBy = "claudeFable5";
        :>> text = "putting the number of views in the keel console tab heading is bad.  Each tab should have actionable items in the tab heading.  When i open up keel console, i'm trying to resolve items.  So if a view has something I need to work on, I want it to be flagged";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-26"; :>> channel = StatementChannel::chat;
    }

    part us027 : UserStory {
        :>> id = "2d966a05-ce81-4537-9a5d-beb99b42ca3e";
        :>> title = "the console tab told me how many lenses exist, not what needs me";
        :>> createdAt = "2026-08-26"; :>> createdBy = "claudeFable5";
        :>> asA = "supervisor who opens the console to resolve items";
        :>> iWant = "each tab to flag what is waiting on ME, not count the viewpoints it holds";
        :>> soThat = "I can see where my work is without opening every surface";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug, not need: the console had the right data all along. /api/obligations already computes what awaits the human, derived from viewpoints declaring surface=act, and the tab rendered s.count - the number of VIEWPOINTS - instead. So the most-read surface in the engine showed the one number that is never the question. Their framing is the fix: a tab is a call to act, not an inventory.";
    }
    #DerivedFrom dependency from us027 to st022;
}
