// Intake for 2026-09-01 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260901 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st063 : Statement {
        :>> id = "d94ca0fc-bf3f-4aca-b769-4634f9e8cc26";
        :>> title = "Reversal: SPLIT nAutonomyBoundedByProof into two Needs; and strip cruft from the CLI surface";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> text = "[x] Split into two Needs - input trust and update consent diverge\n[x] Drift report first - RECOMMENDED\nNote: remove cruft from CLI.  tons of commands, when a smaller subset should do, and parentheticals embedded in descriptions.  tighten it up - what core functional hooks do we need in the CLI?  a lot of these seem to be variations of an idea cemented as separate cli hooks.  and critique all descriptions to trim them down to the real root of what is being done and what a user needs to know to run it\n[x] Asynchronous - land records the SHA, the next keel command surfaces a failure - RECOMMENDED\n[x] Keep repo-level trust exactly as written; --trust overrides case by case - RECOMMENDED";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-01"; :>> channel = StatementChannel::chat;
    }

    part us066 : UserStory {
        :>> id = "d361d784-db57-46bb-9d00-f546bd2b9097";
        :>> title = "The autonomy Need splits: input trust and update consent are separate contracts";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "input-trust and update-consent as two Needs rather than one";
        :>> soThat = "the two surfaces can carry different autonomy without one contract speaking for both";
        :>> implication = ImplicationKind::correction;
        :>> triageNote = "CORRECTION and not need: this REVERSES the adjudication recorded yesterday, where they chose 'keep ONE Need' from the same fork. Their earlier acceptance was genuine and is not deleted - it is superseded, because a recorded attestation is theirs and a change of mind is a new fact rather than an edit to an old one (D0108). The trigger is visible in the fork's own flip condition, which I wrote: 'wanting auto-act on trusted issues while never auto-updating - that is two Needs.'";
    }
    #DerivedFrom dependency from us066 to st063;
}
