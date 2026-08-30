// Intake for 2026-08-29 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260829 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st031 : Statement {
        :>> id = "c2bdd9a4-977d-4617-811f-06e24d229bc2";
        :>> title = "Stricter process launching, verification and assurance is the right direction - but the assurance must be REAL";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> text = "continue. anything to bolster stricter management of process launching, verification, and assurance is a good direction. but I need real assurance";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-29"; :>> channel = StatementChannel::chat;
    }

    part st032 : Statement {
        :>> id = "55da1f1e-5ac4-4f63-bf94-3fbe4ba866b8";
        :>> title = "Hold a propriety panel over every existing and proposed solution: overfit, class coverage, robustness, circumvention, measurability - research best practices, define the process, define the work well first";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> text = "let's flesh things out. for each existing and proposed solution, hold a panel to assess solution propriety. is it overfit? does it prevent the class of bad behavior properly, or just this one example? is it robust to future code changes, or is it inappropriately dependent? can it be circumvented? is it possible to be objectively measured, or must it be qualitatively measured as verified? research best practices for this kind of critique panel, define a process, then let's get critiquing. define the work to be done well, first.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-29"; :>> channel = StatementChannel::chat;
    }

    part us048 : UserStory {
        :>> id = "40ac61ef-af14-4ce3-9cd2-656be0af676f";
        :>> title = "A propriety panel process over enforcement mechanisms, five lenses, defeaters mandatory";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "every existing and proposed solution assessed by a defined panel process under the five propriety questions";
        :>> soThat = "a control's soundness is an argued, defeatable claim rather than an assumption";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not useCase: they explicitly ordered a process defined before critiquing (D0040 agrees independently - it recurs on every future control). The five questions became the lens enum VERBATIM rather than being mapped onto the requirement-quality canon, because force-fitting is the triage sin the intake rule names.";
    }
    #DerivedFrom dependency from us048 to st032;

    #Implicates dependency from us048 to d0257;

    part st033 : Statement {
        :>> id = "b27d1376-6b2e-4ca5-8dd7-55a9aef8ce6c";
        :>> title = "Close the readiness gaps: a TDD implementation plan for the pin, the wrapper, the library and the exec-summary unit - then roll it out";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> text = "well, let's get there.  define a clear implementation plan with TDD, then let's roll it out";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-29"; :>> channel = StatementChannel::chat;
    }

    part us049 : UserStory {
        :>> id = "872313a6-45e1-413a-b26f-eb5bf01e4ebf";
        :>> title = "The readiness gaps close as five TDD increments, then the new project kicks off on the proven path";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a clear TDD implementation plan for the pin, wrapper, library and exec-summary unit, rolled out increment by increment";
        :>> soThat = "the new project starts on the improved workflow rather than the manual one";
        :>> implication = ImplicationKind::priority;
        :>> triageNote = "priority and not process: the processes and requirements all exist (D0250-D0252, the srs); this orders DELIVERY of already-chartered work. The plan is recorded as five ranked backlog tasks (declaration order IS priority, D0052) rather than a prose plan document (D0018), each DoD carrying its Gherkin so tests precede implementation per the established rigor (st027).";
    }
    #DerivedFrom dependency from us049 to st033;

    #Implicates dependency from us049 to dcPinBites;

    part st034 : Statement {
        :>> id = "44285b0a-74d3-4169-9050-fa1a35866bb9";
        :>> title = "The library's home is github.com/williamweatherholtz/keel-lib";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> text = "use this for the repo https://github.com/williamweatherholtz/keel-lib";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-29"; :>> channel = StatementChannel::chat;
    }

    part us050 : UserStory {
        :>> id = "efffbc02-2c35-41a7-8a86-37a203153f2b";
        :>> title = "The library's remote is github.com/williamweatherholtz/keel-lib";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the portable-content library homed at the keel-lib GitHub repository";
        :>> soThat = "every machine syncs the working material from one owned remote";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not convention: it binds D0250's open parameter - WHERE the library lives - which sprint 494's closeOut had named as the residual awaiting exactly this choice. No decision file needed: the remote is machine state discovered from the clone's origin (the D0245 rule), so the binding act was git remote set-url plus a push, and this story is its provenance.";
    }
    #DerivedFrom dependency from us050 to st034;

    #Implicates dependency from us050 to d0250;

    part st035 : Statement {
        :>> id = "d3cfe062-667b-4569-8cfb-372fcd9193ee";
        :>> title = "Reorder the backlog now, and make priority assessment a step of the agile process";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> text = "reorder - assessing priority should be part of our agile process.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-29"; :>> channel = StatementChannel::chat;
    }

    part us051 : UserStory {
        :>> id = "3495cad7-f0e6-43c4-979f-1ec37645c650";
        :>> title = "Priority assessment is a refinement step, driven by the computed signals";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the backlog reordered now, and priority assessment made a standing part of the agile process";
        :>> soThat = "the frontier's order stays a judgment rather than decaying to arrival order";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not priority: their sentence carries both, and the reorder itself is the priority half - executed under the new step so the recurring half does not stay conversational (D0040). The step obliges the AI to read the signal the engine already computes; the priority-inversion guard had been warning for weeks into the void.";
    }
    #DerivedFrom dependency from us051 to st035;

    #Implicates dependency from us051 to d0258;

    part st036 : Statement {
        :>> id = "d5630885-7faf-408d-98c6-86f0fcd6d6f9";
        :>> title = "Program summary returned: kick off now, follow the frontier, define stewardship, acceptRisk the event-less residual - continue through all remaining";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> text = "continue through all remaining\n[x] Kick off now on v0.3.1 - RECOMMENDED\n[x] Follow the assessed frontier: Critical resolver first, then the Highs - RECOMMENDED\n[x] Define library-stewardship now, one process for publish + consume - RECOMMENDED\n[x] Accept the residual as a recorded disposition (acceptRisk) - RECOMMENDED\nMy notes:";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-29"; :>> channel = StatementChannel::chat;
    }

    part us052 : UserStory {
        :>> id = "23a0761a-0784-44cb-8472-0bf7e48148af";
        :>> title = "All four summary picks executed; the frontier is being worked Critical-first";
        :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the four selections carried out and the remaining frontier worked through";
        :>> soThat = "the program moves on the assessed order rather than waiting on me per item";
        :>> implication = ImplicationKind::attestation;
        :>> triageNote = "attestation and not priority: the digest is their verdict on four specific claims - kick off now, follow the frontier, define stewardship, acceptRisk pf07. Each selection routed to its act: pf07Disp1 records the disposition, D0259 the process, the frontier work is underway, and the kickoff waits on exactly one un-inventable input (the project's name), asked rather than invented.";
    }
    #DerivedFrom dependency from us052 to st036;

    #Implicates dependency from us052 to pf07Disp1;
}
