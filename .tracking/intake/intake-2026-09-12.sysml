// Intake for 2026-09-12 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260912 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st120 : Statement {
        :>> id = "a4078446-978e-4ac0-aab4-061d0f79c6bc";
        :>> title = "Hooks check bespoke processes granularly, not hardcoded ones";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> text = "We shouldn't hardcode modular processes - we'll want to add or change in the future. So we need hooks that can granularly check bespoke processes, not just specific ones";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-12"; :>> channel = StatementChannel::chat;
    }

    part us109 : UserStory {
        :>> id = "92999c9a-d548-4ed7-b131-6b36cbc8fe8f";
        :>> title = "Hooks check the processes the model declares, not a list written into the hook";
        :>> createdAt = "2026-09-12"; :>> createdBy = "claudeFable5";
        :>> asA = "supervisor who adds and changes processes over time";
        :>> iWant = "every hook check to be driven by the processes and gates declared in the model, at the grain of one process step, so a bespoke process I add later is checked the same way as the ones that exist today";
        :>> soThat = "adding or changing a process is a model edit, never a hook edit; nothing about a specific process is hardcoded in a hook";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "Said with the acceptance of d0452, whose fold rewrites the hooks in the same commit. My derivation: the pain is a hook naming specific processes or verbs; the check is what the hooks and gate read today (a fixed verb list, or the process model) - to be established by reading .claude/hooks and gate.rs before a Need is written.";
    }
    #DerivedFrom dependency from us109 to st120;
}
