// Declared Questions (D0161 part 2): what a human wants answerable from the model.
// Human-owned by design; the AI drafted these three at build time as the store's
// first content and the human prunes or extends them like any authored fact.
package KnowledgeQuestions {
    private import EngineElement::*;
    private import EngineKnowledge::*;

    part qWhoSignsWork : Question {
        :>> id = "63b4058b-83df-4ebf-89ce-83492d2b95ff";
        :>> title = "Who is allowed to say a piece of work is finished?";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> questionText = "Who is allowed to say a piece of work is finished?";
    }

    part qChatApproval : Question {
        :>> id = "0771e741-cb45-47a6-ae21-832b405e76e0";
        :>> title = "Why can I approve things in chat instead of clicking?";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> questionText = "Why can I approve things in chat instead of clicking?";
    }

    // THE TRAP (D0161 step-7 acceptance clause): the answer chain runs
    // green-evidence -> judgedAgainst SHA -> history rewriting -> the merge-only
    // integration decision, and none of those intermediate elements share a word
    // with this question. If seeding-plus-traversal reaches d0129 the trap passes;
    // if not, the failure axis is recorded, not hidden (kgRecordLimits).
    part qGreenElsewhere : Question {
        :>> id = "b0966f16-2d5b-45e3-b69e-b4cec760d745";
        :>> title = "Why does done on my laptop sometimes not count on the build server?";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> questionText = "Why does done on my laptop sometimes not count on the build server?";
    }
}
