// ProjectIntake20260820 — the first recorded intake. Their words, my translation, the triage verdict.
//
// D0166 accepted 2026-08-20. Every Statement here is VERBATIM from their message: a compound statement
// becomes SEVERAL UserStories rather than one story with several meanings, because the triage verdict is
// singular by design and a statement that implicates three things has three implications.
package ProjectIntake20260820 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    // ── the statements ───────────────────────────────────────────────────────────────────────────────
    part st001 : Statement {
        :>> id = "0f1a5c73-6e28-4b90-8d14-52ca7be03f61";
        :>> title = "identify issues from both critique items and follow through";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "identify issues from both these items and with through implementation.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }
    part st002 : Statement {
        :>> id = "8b40e2d5-1c76-4a38-9052-6ef1a7cd4b83";
        :>> title = "record statements, generate user stories, then triage them";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "note that we should now record user statements, generate user stories, then triage those stories. is the user identifying a need? outlining a use case? identifying a constraint? a bug? process or procedure? functional architecture change? requirement? a change in implementation design? a change in implementation directly? others?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }
    part st003 : Statement {
        :>> id = "3d97b6f1-84e0-4c52-a719-0b25fe8c37d4";
        :>> title = "keel as a powerhouse for organized project knowledge; rock-solid intake";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "big QoL pass here - let's make keel a powerhouse for storing organized knowledge about a project. the intake process should be rock solid.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }

    // ── the translations, one implication each ───────────────────────────────────────────────────────
    part us001 : UserStory {
        :>> id = "6c5f0a28-9b31-4e74-8d06-47ba1ec95f30";
        :>> title = "the UX critique's findings become tracked issues";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the findings from the output-format and artifact critiques recorded as tracked issues";
        :>> soThat = "they are worked rather than left as prose in a reply";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug rather than process: each finding is something WRONG with a shipped surface, and the engine's home for that is an Issue with a resolver. The instruction to follow through with implementation is a priority statement about the same items, not a separate implication - it says work them, not create something else.";
    }
    part us002 : UserStory {
        :>> id = "e2807b46-5d19-4a63-b8f5-91ce3a70d248";
        :>> title = "intake is a process: record the statement, translate it, triage the translation";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "my statements recorded, translated into user stories, and each story triaged by what it implicates";
        :>> soThat = "direction I give is tracked as data rather than surviving only in a transcript";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process, not implementationChange: the sentence defines a SEQUENCE of steps (record, translate, triage) which is a process definition, and the code that supports it follows from the process rather than the other way round. The enumerated question list is the triage vocabulary and is captured as ImplicationKind, not as a separate story.";
    }
    part us003 : UserStory {
        :>> id = "a71d3e05-4c82-4b96-9f27-58db60ea1c74";
        :>> title = "keel holds organized project knowledge, and intake is where it enters";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "keel to be a powerhouse for storing organized knowledge about a project, entered through a rock-solid intake";
        :>> soThat = "the project's knowledge is a queryable asset rather than scattered across records and transcripts";
        :>> implication = ImplicationKind::need;
        :>> triageNote = "need, not requirement: it states a desired outcome with no verifiable boundary - powerhouse and rock solid are not thresholds - so promoting it to a SystemRequirement now would invent a contract they did not set. Invariant 7 says it stays a Need until a defensible boundary emerges. It is also the parent of D0161's knowledge index, which is accepted and unbuilt.";
    }

    // ── what each translation cites, and what it produced ────────────────────────────────────────────
    #DerivedFrom dependency from us001 to st001;
    #DerivedFrom dependency from us002 to st002;
    #DerivedFrom dependency from us003 to st003;

    #Implicates dependency from us001 to issue161;
    #Implicates dependency from us001 to issue162;
    #Implicates dependency from us001 to issue163;
    #Implicates dependency from us001 to issue164;
    #Implicates dependency from us001 to issue165;
    #Implicates dependency from us002 to d0166;
    #Implicates dependency from us003 to d0161;

    // ── second intake, same day: their instruction to wire the new types into parsing ────────────────
    part st004 : Statement {
        :>> id = "5a9c1e37-2b04-4d68-9f15-73ae08cb254f";
        :>> title = "new items accepted; implement into parsing";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "new items accepted. implement into parsing please";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }

    part us004 : UserStory {
        :>> id = "c1b78f04-6d29-4a53-8e70-15fb92ce4370";
        :>> title = "the accepted intake vocabulary and item types are accepted";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the new intake item types and the added implication kinds accepted as they stand";
        :>> implication = ImplicationKind::attestation;
        :>> triageNote = "attestation, and the scope is stated because the phrase `new items` is not precise: read as the intake types (Statement, UserStory) and the five ImplicationKind members added last turn, since those are what was newly proposed and unsigned. It is NOT read as accepting issue161-165, which are findings rather than proposals and need dispositions instead. If they meant those too, the disposition is theirs to give and this triage is the thing to disagree with.";
    }

    part us005 : UserStory {
        :>> id = "e83d0a61-4f57-4b92-a608-27cb95e10df4";
        :>> title = "the new item types are enforced wherever types are enumerated";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the new item types wired into parsing";
        :>> soThat = "they are treated as first-class items rather than declared and ignored";
        :>> implication = ImplicationKind::implementationChange;
        :>> triageNote = "implementationChange rather than process: the sentence asks for code, not for a new way of working. WHAT IT TURNED OUT TO REQUIRE was wider than the words - the types parse fine, but they were absent from all three copies of the id-type list, and following that thread found that identity was not enforced for .tracking AT ALL (issue166). The story is recorded as they said it; the discovery lives in the issue.";
    }

    #DerivedFrom dependency from us004 to st004;
    #DerivedFrom dependency from us005 to st004;
    #Implicates dependency from us005 to issue166;

    // -- third intake, same day: where performance work counts ---------------------------------------
    part st005 : Statement {
        :>> id = "b47d0e59-3c81-4f26-9a05-71ef62cb840d";
        :>> title = "performance is about keel serve, the interactive surface";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "the performance i want is all targeted to the keel serve - the interactive surface.  i don't doubt the cli can be slow.  regardless, deploy performance improvements";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }

    part us006 : UserStory {
        :>> id = "6e91f3a4-2b07-4d58-8c16-40db93ae7512";
        :>> title = "performance work is measured on the interactive surface, not the CLI";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "performance effort targeted at keel serve, the interactive surface";
        :>> soThat = "the surface I actually operate is fast, whatever the CLI costs";
        :>> implication = ImplicationKind::scopeConstraint;
        :>> triageNote = "scopeConstraint, not priority: a priority statement reorders existing work, and this instead BOUNDS what counts as a performance improvement at all. It is the correction of a real misdirection - five prior performance sprints all measured CLI commands, and this session's instrumentation could only ever report at process exit, so the server was structurally unmeasurable (issue168). Reading it as merely 'do serve first' would have kept the same instrument pointed at the same wrong surface.";
    }

    part us007 : UserStory {
        :>> id = "0a5c47e2-8d63-4b19-9f70-25ba1ce30684";
        :>> title = "ship the performance improvements rather than another measurement";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the performance improvements deployed";
        :>> soThat = "the work turns into something I can feel, not another report";
        :>> implication = ImplicationKind::implementationChange;
        :>> triageNote = "implementationChange, and the word `regardless` is what makes it a separate story from the scope constraint: they are saying ship it EVEN IF my scoping argument differs from theirs. It reads as a response to sprint 358, which delivered a refutation and shipped no fix and said so - a defensible outcome once, and an instruction not to repeat it.";
    }

    #DerivedFrom dependency from us006 to st005;
    #DerivedFrom dependency from us007 to st005;
    #Implicates dependency from us006 to issue168;
    #Implicates dependency from us007 to d0167;
}
