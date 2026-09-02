// Intake for 2026-09-02 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260902 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st064 : Statement {
        :>> id = "8f09e473-8ce3-4b59-a25d-2f7629334170";
        :>> title = "Trust but verify: the DoD is the gold standard, and keel's win is a short-term vision so granular that the right solution emerges even from a sparse model";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> text = "that's ok though.  this is really how people work, and I don't think there's a better way to introduce a non-deterministic program into processes that are deterministic.  We simply trust but verify.  If we're verifying the work, that's really the gold standard - DoD.  The main win in keel is to (micro?)manage AI such that it has a very clear short term vision, goal, and DoD.  To describe next steps in such detail and granularity that the correct solution emerges, even with relatively sparse models like Haiku";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-02"; :>> channel = StatementChannel::chat;
    }

    part us067 : UserStory {
        :>> id = "f2422a7a-864c-4cda-a599-dee6833e2112";
        :>> title = "A DoD is granular enough when a sparse model delivers it";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> asA = "the human directing AI work";
        :>> iWant = "to know whether a task's vision, goal and DoD are specified finely enough that a relatively sparse model, given only orient and the DoD, produces work that passes the gate";
        :>> soThat = "granularity is measured rather than felt, and the DoD is improved where the sparse model fails instead of the model being blamed";
        :>> implication = ImplicationKind::verifiedRequirement;
        :>> triageNote = "The human states the win as granularity of short-term direction, and names Haiku as the bar. That is a falsifiable claim keel can run: the same rig as dcSkillsRouteBehaviourally (a real model against a fixed prompt) pointed at DoD sufficiency instead of skill routing. Recorded as the intent behind that item; the DoD-drift issue (issue343) is the other half - the standard must also be the standard that was agreed.";
    }
    #DerivedFrom dependency from us067 to st064;

    part us068 : UserStory {
        :>> id = "2ca2ea8e-2831-461a-bd99-3648582615fb";
        :>> title = "The standard verified is the standard that was agreed";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> asA = "the human trusting but verifying";
        :>> iWant = "the DoD a pass was judged against to be the DoD the tree carries, or the pass to show as suspect";
        :>> soThat = "trust-but-verify cannot be satisfied by rewriting the criterion after the pass";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "Confirmed by experiment the same hour: a done task's own DoD edited after its pass stays done (issue343, resolver dcOwnDoDDriftIsSuspect, ranked first).";
    }
    #DerivedFrom dependency from us068 to st064;
}
