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

    part st065 : Statement {
        :>> id = "d5add5ad-c2ed-4737-a27a-7d87ec7cde4e";
        :>> title = "Revamp verification: automatic tests self-verify, everything else defaults to human judgment on AI-proposed evidence in sets, by method kind - and an STPA on the keel workflow";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> text = "so let's revamp verification.  if there's an automatic test, it can be automatically verified and that should satisfy its parent.  Otherwise, we need some form of verification and by default that should be human verification.  if there's something that can't be proven to be programatically determined to be done (e.g. unit tests, integration tests, etc.) then AI should propose to the human evidence of it being done and let them judge/verify. At the same time, I don't want to be hammered with items, so maybe we can do sets of functions.  And there are different kinds of verification - analysis, test, demonstration, etc.  Let's leverage those.  We just need a way to really verify things.  If AI can demonstrate things, then that's a feasible verification.  Though we should also have a strong STPA process to identify control gaps, if we're going to be that cavalier with AI.  Which I'm actually happy to do.      Hmm.  Maybe we should do an STPA analysis on our keel workflow.  Clear authorities, what connects to what to do what, UCA analysis and clear state analysis between systems...";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-02"; :>> channel = StatementChannel::chat;
    }

    part st066 : Statement {
        :>> id = "505bc89a-7cf4-4538-bcbc-77b3a14e0d89";
        :>> title = "First step: a defined system for keel itself - authorities, interactions, data sent";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> text = "i'm not sure we have a defined system for keel itself.  that should be the first step.  what authorities are present, and how do they interact, what data it sent, etc.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-02"; :>> channel = StatementChannel::chat;
    }

    part us069 : UserStory {
        :>> id = "7a1e46fe-1d5c-41a5-97c9-37680ea8beec";
        :>> title = "keel's own control structure is an authored fact";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> asA = "the human deciding how far to trust AI in the process";
        :>> iWant = "the authorities in keel, what each may issue on which process, what data each action carries and what feedback returns, recorded as STPA step 2 instances I can read and correct";
        :>> soThat = "UCA and loss-scenario analysis has a defined system to run against, and the verification revamp binds authority to method on a structure rather than on impressions";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "Authored the same hour as engine-control-structure.sysml: 8 controllers, 6 controlled processes, 19 control actions, 9 feedback paths, 5 process models with known-false beliefs named. Read back to the human for what is WIDER than meant (D0157), not for whether it is good.";
    }
    #DerivedFrom dependency from us069 to st066;

    part us070 : UserStory {
        :>> id = "61e550fa-35ba-48c0-aa19-0f83a16eaa1d";
        :>> title = "Verification by authority and method: machine-verified tests satisfy; everything else is human judgment on AI-proposed evidence, in sets";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> asA = "the human who trusts but verifies";
        :>> iWant = "a passing TestResult to mean one of three things and nothing else: a machine ran a test and CI can re-derive it; a human judged AI-proposed evidence (demo, analysis, inspection) presented in batches; or a human confirmed a claim outright";
        :>> soThat = "AI testimony stops counting as verification, tests that can verify themselves do so without asking me, and I am asked in sets rather than hammered";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "MEASURED the same day: 4954 AI-judged results stand, 9 percent of the exercised ones carry a receipt, 3694 examined-by-AI are testimony. This is a Change Request against the verification model and the frozen VerificationMethod vocabulary; it goes to the human as a costed fork AFTER the control structure is reviewed, because the revamp binds authority to method and the authorities were undefined until st066.";
    }
    #DerivedFrom dependency from us070 to st065;

    part us071 : UserStory {
        :>> id = "2fab3800-df23-4757-b3a6-88a5a602ae00";
        :>> title = "An STPA process for keel's own workflow";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> asA = "the human being deliberately cavalier with AI";
        :>> iWant = "a repeatable STPA process - control structure, UCAs per control action, loss scenarios, controller constraints - run against keel's own workflow and re-run when the structure changes";
        :>> soThat = "control gaps are identified by analysis before an incident names them, not after";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "Recurring by the human's words (a strong process), so D0040 makes it a CHANGE: define the process and skill, then run it. The schema (EngineSafety, STPA steps 1-4) already exists and steps 1 and 3 have instances; step 2 landed today. The process definition is the next fork item.";
    }
    #DerivedFrom dependency from us071 to st065;
}
