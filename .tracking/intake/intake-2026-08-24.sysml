// ProjectIntake20260824 — the penumbra-adoption answers. Their words, my translation, the triage verdict.
// Recorded BEFORE any Need or Brief (D0216): I asked three questions about migrating to penumbra and they
// answered all three in one message, so one Statement carries it verbatim and the stories split it.
package ProjectIntake20260824 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st018 : Statement {
        :>> id = "8a1d589e-c664-427a-949c-63f5b980f969";
        :>> title = "leave me as the only decision maker, don't barrage the other one with CI notifications, and build an onboarding process that picks the process set";
        :>> createdAt = "2026-08-24"; :>> createdBy = "claudeFable5";
        :>> text = "#2 nope, there's another decision maker in penumbra, but I don't want to barrage him with GitHub CI notifications the way I'm getting now. just leave me as the decision maker.

#3 I want an onboarding process when processes haven't been defined. figure out what technical needs or workflows the author wants for the project, or research best practices to add new processes or pick a good set (are they developing HW, SW, all, business processes, safety-related, art, video games, construction plans, whatever.)

if these are in place, then yes let's proceed with #1";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-24"; :>> channel = StatementChannel::chat;
    }

    part us021 : UserStory {
        :>> id = "b8fe64b8-0db7-4535-ad8c-c7f23fb3fd7b";
        :>> title = "a second decider exists but must not be enrolled into the notification stream";
        :>> createdAt = "2026-08-24"; :>> createdBy = "claudeFable5";
        :>> asA = "project owner adopting keel onto a second repository";
        :>> iWant = "the decider table to list only me, because the other decider must not be barraged with GitHub CI notifications";
        :>> soThat = "adopting the engine does not impose a notification cost on a colleague who never asked for it";
        :>> implication = ImplicationKind::scopeConstraint;
        :>> triageNote = "scopeConstraint on the penumbra adoption, and it needs NO code: penumbra's github-actors.toml already maps only williamweatherholtz, so the constraint is satisfied by not enrolling anyone else. WHAT IS NOT SATISFIED is the reason they gave. 'the way I'm getting now' is a report of PRESENT pain on this repo, volunteered rather than asked for, and it is the first friction report about a control I built. It is recorded separately as us022 because a constraint that is already met and a pain that is live are different facts, and folding them would let the live one disappear behind the met one.";
    }

    part us022 : UserStory {
        :>> id = "f6d592b4-fb8b-4c2f-9d02-7091c8f83aed";
        :>> title = "the decision channel is barraging its own owner with notifications";
        :>> createdAt = "2026-08-24"; :>> createdBy = "claudeFable5";
        :>> asA = "project owner running keel on my own repository";
        :>> iWant = "to stop being barraged with GitHub CI notifications";
        :>> soThat = "the notifications I do get are ones I need to act on";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug, not need: a control I shipped is imposing a cost its design did not account for, which is the D0054 adoption-friction class - the dominant MBSE failure mode is friction, not bad architecture. Volunteered as an aside while answering a different question, which is how friction usually surfaces. NOT YET DIAGNOSED at triage time: candidate sources are one GitHub issue opened per proposed Decision (D0207's override thread), a 30-minute cron sweeper, and CI on every push. Measure before changing anything - and the fix must not weaken the override thread, which is the human's only route to reverse an auto-accepted Decision.";
    }

    part us023 : UserStory {
        :>> id = "5b240590-8461-4660-a5b5-76563c82542c";
        :>> title = "a project whose processes are undefined should be walked through choosing them";
        :>> createdAt = "2026-08-24"; :>> createdBy = "claudeFable5";
        :>> asA = "author starting a keel project";
        :>> iWant = "an onboarding process that works out what I am building and either elicits the workflows I need or researches good practice for my domain, then picks a process set";
        :>> soThat = "I am not left to choose from a catalogue I have no basis to judge";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process, and it maps to NO existing one, so section 3 says defining it IS the creative output. The domains they enumerated - hardware, software, both, business processes, safety-related, art, video games, construction plans, 'whatever' - are the requirement, not examples: the process cannot be a fixed table of domains, because 'whatever' is explicitly in scope. TENSION TO RESOLVE, and it is the design problem: D0216 forbids offering a menu, since a chosen option is evidence about the menu rather than about their work - but picking a process set from a catalogue IS a menu. The resolution has to be that onboarding asks about the WORK and what must not go wrong, then RECOMMENDS with stated reasoning the author can reject. Gated by them as a precondition on the penumbra migration.";
    }

    // The routing: us021 needed no work (the constraint was already met), us022 became the
    // notification-volume bug, us023 became issue257 and the project-onboarding process.
    #Implicates dependency from us022 to dcChannelNotificationCost;
    #Implicates dependency from us023 to dcProjectOnboarding;
    #DerivedFrom dependency from us021 to st018;
    #DerivedFrom dependency from us022 to st018;
    #DerivedFrom dependency from us023 to st018;

    part st019 : Statement {
        :>> id = "965468c5-648d-4470-a493-e5a580150b3c";
        :>> title = "25 approved";
        :>> createdAt = "2026-08-24"; :>> createdBy = "claudeFable5";
        :>> text = "25 approved";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-24"; :>> channel = StatementChannel::chat;
    }

    part us024 : UserStory {
        :>> id = "81270338-18f1-49b6-a0df-4b8f88e859ac";
        :>> title = "the fork is approved, but the approval names no option";
        :>> createdAt = "2026-08-24"; :>> createdBy = "claudeFable5";
        :>> asA = "project owner deciding on a fork";
        :>> iWant = "issue 25 approved";
        :>> soThat = "the channel stops spending my attention on decisions that need nobody";
        :>> implication = ImplicationKind::question;
        :>> triageNote = "QUESTION rather than attestation, and the distinction is the point. d0227 enumerated three options with materially different costs, and 'approved' names none of them - the reply vocabulary I had given them was A, B, C or reject. Reading it as agreement with my recommendation would have been exactly the inference I stopped to avoid one turn earlier, so it was put back to them and they chose OPTION A: one standing thread. Their choice is recorded here as the source of the change; the method=confirmation ACCEPTANCE is not recorded by me, because keel accept refuses from an agent session by design (D0178/K6) - it is theirs to make, by replying A on the issue or from their own terminal.";
    }

    #DerivedFrom dependency from us024 to st019;
    #Implicates dependency from us024 to dcChannelNotificationCost;

}
