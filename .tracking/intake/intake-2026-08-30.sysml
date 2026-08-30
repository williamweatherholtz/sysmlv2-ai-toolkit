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

    part st039 : Statement {
        :>> id = "f5a54604-ce0f-428a-9c5e-3c0d6f5e961d";
        :>> title = "Continue the frontier, and find where rigor has been lost in the processes";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "continue through all of these. I think we've lost some rigor in our processes. let's do this work and figure out where any gaps are";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us055 : UserStory {
        :>> id = "24649a02-b212-47e2-afae-11c61aa23828";
        :>> title = "The rigor audit: measure the hypothesis, clean the channel, discharge the debt it hid";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the process gaps found and closed rather than suspected";
        :>> soThat = "the signals I rely on mean something when I read them";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not question: they asked where the gaps are AND told me to do the work, so this routes to execution, not an answer. The measured finding is narrower than the hypothesis - three of four hardening dimensions were perfect - but the warning channel was 64 percent undischargeable noise and it hid real debt including a direction the human chose a week ago. Routed to D0261 and the three chartered tasks.";
    }
    #DerivedFrom dependency from us055 to st039;

    #Implicates dependency from us055 to d0261;

    part st040 : Statement {
        :>> id = "ddd1b3c3-62b8-45b5-a43e-aec68f699dcb";
        :>> title = "Build a comprehensive linked-project test platform: seven claims, designed, panel-critiqued, implemented, re-panelled";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "make a comprehensive test platform around linked projects. if another keel project is made, does everything work properly? pushing issues back to keel itself? picking up processes? if it publishes a new process, is that linked back upstream to keel? is the numbering scheme properly reset? other proper initialization steps followed? is the keel instantiated project confused on what keels purpose vs its own purpose is?\n\ndesign a test suite, hold a panel to critique, implement and test, then reaporoach panel with findings. strong empirical evidence recorded at every step.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part st041 : Statement {
        :>> id = "7b9ae63e-3afc-4d06-922d-c3276ac0f2cc";
        :>> title = "Add a process and AI skill to ingest GitHub issues";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "and add a new process and AI skill to ingest GitHub issues";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us056 : UserStory {
        :>> id = "bf53cae5-12d9-4035-986c-9c641591d819";
        :>> title = "A federation test platform: the seven linked-project claims, held empirically";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a test suite that proves a downstream keel project actually works in a federation";
        :>> soThat = "I can start new projects without discovering the failures myself";
        :>> implication = ImplicationKind::verifiedRequirement;
        :>> triageNote = "verifiedRequirement and not bug: they named seven CLAIMS and asked for them to be held by tests, which is a verification obligation rather than a defect report - though reconnaissance turned two of the seven into defects before the suite existed. The design-panel-implement-repanel sequence they specified IS the control-propriety process (D0257) applied to a test design.";
    }
    #DerivedFrom dependency from us056 to st040;

    part us057 : UserStory {
        :>> id = "2e48cf4d-6a61-404a-8846-8e29258b3b3d";
        :>> title = "GitHub issue ingestion becomes a process with a deploying skill";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "GitHub issues ingested into the model as tracked Issues with provenance";
        :>> soThat = "a defect reported from a downstream project or a collaborator becomes tracked work rather than a browser tab";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not implementationChange: they asked for a PROCESS and a SKILL by name, and the mechanism is the inbound half of the upstream loop st040 asks about - a downstream project filing against keel, and keel taking it in. Routed together with st040 for that reason.";
    }
    #DerivedFrom dependency from us057 to st041;

    part st042 : Statement {
        :>> id = "a2989034-b345-4401-a81b-b5a7b791b8b6";
        :>> title = "GH#38 The decision-channel workflows use third-party actions, so under an org Actions allowlist they startup_failure forever and the cron emails on every run";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "Verified: `.github/workflows/decision-issue.yml` and `.github/workflows/decision-record.yml` both reference `dtolnay/rust-toolchain` and `Swatinem/rust-cache`.\n\n## What happens in a project that adopts the channel\n\nMany GitHub organisations restrict Actions to GitHub-owned plus a named allowlist. Where neither third-party action is allowlisted, the workflow never parses into a run — every trigger is a `startup_failure`, not a failing step.\n\nIn the deployment we hit this in: **57 of 57 runs startup_failed, zero successes ever.** The channel had therefore never worked in that repo at all — including the `issue_comment` path, so a one-letter acceptance comment recorded nothing, silently.\n\nThe second cost is attention. The sweeper cron (`*/30`) retries twice an hour indefinitely, and GitHub emails the workflow actor on every failed scheduled run — roughly 48 mails a day. That is what surfaced it: the human asked why they were getting so much mail from a repo, not why decisions weren't recording.\n\n## Why this is worth fixing rather than documenting\n\nThe failure is invisible in the direction that matters. A channel that has never once run looks, from inside the project, exactly like a channel with nothing to say — there is no decision sitting unrecorded that anyone can see. The only signal is the mail volume, which reads as noise rather than as a broken mechanism.\n\n## Direction (yours)\n\n`ubuntu-latest` ships a Rust toolchain and `rustup` is preinstalled, so the third-party actions may not be needed at all for these two workflows — dropping them would make the channel work under any allowlist without asking every adopting org to change policy. Vendoring a release binary would also do it.\n\nWorth considering separately: a startup_failure is not observable from the project side, so if the channel is meant to be relied on, it may want a way to say ''I have never successfully run here''.\n\nFor context on the reporting project: we retired the channel locally rather than pursue an org allowlist change, so this is filed for other adopters, not as a blocker for us.";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/38";
    }

    part us058 : UserStory {
        :>> id = "136fa270-2398-434a-9f34-56996ce1bec6";
        :>> title = "Decision-channel workflows fail closed under an org Actions allowlist and email on every cron run";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "maintainer";
        :>> iWant = "the decision-channel workflows to run without third-party actions";
        :>> soThat = "the channel works under an org allowlist instead of startup_failing forever and emailing on every cron";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug and not process: the process is sound and its IMPLEMENTATION fails in an environment the design never considered - an org-level Actions allowlist that blocks third-party actions, which makes every run a startup_failure and turns the cron into a mail generator. INGESTED FROM GITHUB (issue 38), the first utterance recorded through the new github-ingest path, which is why it carries a sourceUrl a reader can check the transcription against.";
    }
    #DerivedFrom dependency from us058 to st042;
}
