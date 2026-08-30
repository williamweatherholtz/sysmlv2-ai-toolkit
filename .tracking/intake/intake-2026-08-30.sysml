// Intake for 2026-08-30 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260830 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st037 : Statement {
        :>> id = "09a73dab-09fe-4593-9e18-1b710e94056e";
        :>> title = "Why is finished work read as open work - is it the dropped unenforceable checks, or unverifiable work flagged unverified?";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "why is finished work being confused for open work now? we should be preventing this for sure. we just dropped some unenforceable checks right? seems like we need a strong process here. or is the issue that unverifiable work is being flagged as unverified?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us053 : UserStory {
        :>> id = "61d9ae42-5ef0-431b-a049-8b9db84c366f";
        :>> title = "The finished-vs-open confusion is measured, not assumed, and its one real instance gets a ratchet guard";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the cause of finished work reading as open work established from the data and prevented";
        :>> soThat = "the frontier's ready list can be trusted without me spot-checking it";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug and not process: their question named two candidate causes and both are refuted by measurement (the dropped gate was about DoD WORDING, never stamping; and the inverse of their second hypothesis is what happened). The residue is one real defect in 496 sprints with a mechanical cause, so it routes as a bug with a ratchet control - D0260 - rather than the new ceremony a process implication would imply. The correction to my own mis-report rides in the same Decision.";
    }
    #DerivedFrom dependency from us053 to st037;

    #Implicates dependency from us053 to d0260;

    part st038 : Statement {
        :>> id = "552fde3c-fdaa-437f-868a-6bd77ed63ed7";
        :>> title = "What control prevents the regenerate-from-model class in future?";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "solid what to prevent this in the future?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us054 : UserStory {
        :>> id = "48088f73-5b26-47de-b8c2-c3aac0813fc8";
        :>> title = "No-op invariance becomes the control for regenerate-from-model writers";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a control that prevents a writer silently dropping facts it cannot represent";
        :>> soThat = "the next such defect fails a test instead of waiting for a locked file to change";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not bug: the defect itself (issue293) is already resolved; what they are asking for is the standing prevention, which is a control plus its coverage floor. Routed to issue310, the generalisation filed from issue293's retro, rather than opening a new item - one governance home per defect class.";
    }
    #DerivedFrom dependency from us054 to st038;

    #Implicates dependency from us054 to issue310;
}
