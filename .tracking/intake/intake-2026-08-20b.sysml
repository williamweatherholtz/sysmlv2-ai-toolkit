// ProjectIntake20260820B — the hardening request. Their words, my translation, the triage verdict.
// Split from intake-2026-08-20.sysml only because that file had grown past comfortable reading; the
// process is the same one (D0166) and the numbering continues.
package ProjectIntake20260820B {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st007 : Statement {
        :>> id = "f30a67c4-9b52-4d18-8e70-24ca15bd7e93";
        :>> title = "critique keel's own purpose, needs, use cases, workflow and performance; it is not doing what it says";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "ok let's step back for a second and do a critqiue pass on keel's own purpose, needs, use cases, workflow, and performance.  i feel like it's not doing what it says it will do, and there are some half-baked implementations floating out there.  a hardening, tech-debt-paying pass.  start with a panel of critiques";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }

    part us011 : UserStory {
        :>> id = "5c81b04e-7f39-4a26-9d53-61ba38ce0472";
        :>> title = "the engine's own claims are audited against refuting evidence";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "a critique pass over keel's purpose, needs, use cases, workflow and performance";
        :>> soThat = "I can trust that what it says it does is what it does";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process, and it maps to an EXISTING one rather than a new one: `.engine/processes/architectural-critique.sysml` under D0046 is exactly this - CAE claims-arguments-evidence, GQM metrics, ATAM scenarios, adversarial by construction, findings recorded as tracked Issues and never as prose. Section 3 says define a process only when a request maps to none, so defining one here would have been the wrong creative output. The word `panel` is read as the multi-lens critique this process already runs, NOT as a request to spawn agents.";
    }

    part us012 : UserStory {
        :>> id = "a4e07d13-6c85-4b91-8f20-53da96ce1748";
        :>> title = "keel does not do what it claims, and half-finished work is loose in the model";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the gap between what keel claims and what it does closed, and the half-baked implementations found";
        :>> soThat = "the engine's own record is trustworthy";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug rather than need: it asserts something is WRONG with a shipped surface, not that something is missing from the roadmap. IT WAS TREATED AS A CLAIM TO TEST, NOT A BRIEF TO AGREE WITH, and it came out split. SUPPORTED: 47 percent of subcommands absent from `--help` (issue172), 13 of 19 processes asserting no guard (issue173), D0161 accepted with its artifact unbuilt (issue174), actualHours at 1 percent (issue175). REFUTED: every command CLAUDE.md names resolves, so the docs do not promise vapour - the gap runs the other way, the engine does MORE than it says. Recording the refutation matters as much as the findings: `half-baked` was right about the mechanism and wrong about the direction.";
    }

    part us013 : UserStory {
        :>> id = "d6b93f02-1a47-4c58-8e61-72ca04be5d19";
        :>> title = "pay down tech debt rather than add capability";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "a hardening pass that pays tech debt";
        :>> soThat = "the next capability lands on something solid";
        :>> implication = ImplicationKind::priority;
        :>> triageNote = "priority, not scopeConstraint: it does not bound WHAT counts as in scope, it reorders the frontier - hardening before new capability. Declaration order IS priority (D0052), so honouring this means the six hardening items are declared AHEAD of the existing ready set rather than appended after it. Self-terminating: applying a priority produces no downstream item, it moves existing ones.";
    }

    #DerivedFrom dependency from us011 to st007;
    #DerivedFrom dependency from us012 to st007;
    #DerivedFrom dependency from us013 to st007;

    #Implicates dependency from us011 to d0046;
    #Implicates dependency from us012 to issue171;
    #Implicates dependency from us012 to issue172;
    #Implicates dependency from us012 to issue173;
    #Implicates dependency from us012 to issue174;
    #Implicates dependency from us012 to issue175;
    #Implicates dependency from us012 to issue176;

    // -- fifth intake: retros must produce items; reviews must critique; the buttons still fail ----
    part st008 : Statement {
        :>> id = "a95c03e7-4b61-4d28-8f50-72ba16ce9d40";
        :>> title = "retros should identify recurring issues as items; reviews should run the 3 critique passes; new process";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "are we identifying these recurring issues in sprint retros? we should be. and have sprint reviews with the 3 critique pass and add those items two. new process";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }
    part st009 : Statement {
        :>> id = "c07d5b29-8e43-4a16-9f62-31ba84ce5d07";
        :>> title = "buttons still do not work: they press but do not latch";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> text = "buttons still funny work. I press them but they don't latch. they indicate state do they should latch [corrected:] still don't* work";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-20"; :>> channel = StatementChannel::chat;
    }

    part us014 : UserStory {
        :>> id = "e63b90f4-2a57-4c81-8d05-46ca72be1d38";
        :>> title = "a recurring retro finding becomes a tracked item, enforced";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "sprint retros to turn recurring issues into tracked items, and sprint reviews to run the critique passes and track what they find";
        :>> soThat = "a lesson learned five times costs five sprints once, not five times";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process, and the evidence for the gap is exact: the retro-backlog guard passes when ANY tracked item is co-staged in the commit, so a retro finding never needs ITS OWN item as long as the sprint recorded something else - and every hardening commit did. The heredoc failure appears in five retros and zero backlog items. Their word `two` is read as `too`.";
    }
    part us015 : UserStory {
        :>> id = "f24a81c5-9d30-4e67-8b19-50ca63be7f24";
        :>> title = "a tapped verdict latches visibly";
        :>> createdAt = "2026-08-20"; :>> createdBy = "claudeOpus5";
        :>> asA = "reviewer on a phone";
        :>> iWant = "a verdict button to latch and show its state when pressed";
        :>> soThat = "I can tell what I have judged";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug, second report on the same surface after a fix I verified only from the emitted HTML. Their correction narrows it: the tap registers ('funny work', 'they press') but state does not LATCH - so the click path runs and either the attribute write or its CSS rendering is undone or never lands. That distinguishes it from issue188's dead-at-load script, which the script-ok marker now rules in or out on sight.";
    }

    #DerivedFrom dependency from us014 to st008;
    #DerivedFrom dependency from us015 to st009;

    // -- sixth intake: the deck moves into the engine and gets tested before deploy ---------------
    part st010 : Statement {
        :>> id = "b70e94c2-5a38-4d61-8f27-40ca85be1d59";
        :>> title = "make the artifact locally in rust, test thoroughly, then deploy; use httpx";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "how is this so hard? make the artifact locally in rust, test rhoroughally, then deploy it. use httpx";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }
    part st011 : Statement {
        :>> id = "c81f05d3-6b49-4e72-9038-51db96cf2e60";
        :>> title = "advertise the deck in the cli";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "advertize this in the cli";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }

    part us016 : UserStory {
        :>> id = "d92a16e4-7c50-4f83-a149-62ec07da3f71";
        :>> title = "the deck is built by the engine and its save loop is tested before any deploy";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the review deck generated in Rust, its whole save loop tested locally with httpx, and only then deployed";
        :>> soThat = "a broken deck is caught by a test instead of by me, on my phone, five times";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture, not implementationChange: the instruction moves WHERE THE SAVE PATH LIVES - out of the untestable artifact runtime and into the engine's own HTTP API, with the artifact mechanism demoted to fallback. 'How is this so hard' is answered honestly in issue192: the previous architecture put the critical path in the one component that cannot run locally, so every fix was inference-then-deploy-then-ask. The rhetorical question is treated as the architectural instruction it is.";
    }
    part us017 : UserStory {
        :>> id = "e03b27f5-8d61-4a94-b250-73fd18eb4082";
        :>> title = "the deck is discoverable from the CLI";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the deck advertised in the cli";
        :>> soThat = "I can find it without reading a chat transcript";
        :>> implication = ImplicationKind::implementationChange;
        :>> triageNote = "implementationChange: two lines on existing surfaces - the help CATALOGUE (whose coverage test would have failed the build without one of them) and the keel serve startup banner. The catalogue control caught its own inverse gap along the way: a help line for a command that did not dispatch.";
    }

    #DerivedFrom dependency from us016 to st010;
    #DerivedFrom dependency from us017 to st011;
    #Implicates dependency from us016 to issue192;

    // -- seventh intake: the mobile transport must WORK, with both ends verifiable -----------------
    part st012 : Statement {
        :>> id = "a3f81c25-9d47-4b06-8e13-52cb70da4f96";
        :>> title = "the phone verdicts are gone; no VPN is possible; make the remote artifact pipeline work and test it";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "#1 they're not recoverable it seems. nothing is there. #2 can't do- the device I remote control from can't be guaranteed to be on my vpn. just make the remote artifact pipeline worked. test again if you made fixes";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }

    part us018 : UserStory {
        :>> id = "b4a92d36-0e58-4c17-9f24-63dc81eb5aa7";
        :>> title = "mobile verdicts reach keel through a transport both ends can verify";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "taps on the published deck to actually arrive, from any network, with an acknowledgment the receiver side can check";
        :>> soThat = "I can clear my queue from my phone without wondering whether my input was recorded";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture: their #1 closed the diagnosis (the live-doc appends never persisted - nothing on their own reload either, so the write itself dropped) and their #2 closed the tunnel option (the controlling device is not guaranteed on their VPN). The remaining channel that works from any network AND carries a receiver-side acknowledgment is the artifact mcp capability: the page delivers each tap as a ROW to a Smartsheet inbox with the viewer's own connector credentials, the STORE acks with a row id the page shows as its receipt, and the AI session pulls unprocessed rows and records them through the tested write API with computed-view verification. Every leg except the single in-page callTool is machine-testable, and that call's request/response shape was observed live before publish, per the capability contract.";
    }

    #DerivedFrom dependency from us018 to st012;
    #Implicates dependency from us018 to issue194;

    // -- eighth intake: the noted needs-work verdicts from the first full deck review (2026-08-21) --
    part st013 : Statement {
        :>> id = "c5ba3e58-2a70-4b17-9f24-85fe03ad7cd0";
        :>> title = "UUIDs should be minted programmatically, not by AI diligence";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "we should be programatically creating UUIDs, not via AI diligence";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }
    part st014 : Statement {
        :>> id = "d6cb4f69-3b81-4c28-a035-96af14be8de1";
        :>> title = "whether unguarded process assertions matter depends on what is being asserted";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "depends on what is being asserted.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }
    part st015 : Statement {
        :>> id = "e7dc5a7a-4c92-4d39-b146-a7ba25cf9ef2";
        :>> title = "decisions are landing without their downstream scaffolding - backlog items, user stories, verification";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "because the scaffolding under a decision isn't being made - is there no downstream backlog items?  user stories?  verification test cases to those items?  sounds like we're not following processes again.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }
    part st016 : Statement {
        :>> id = "f8ed6b8b-5da3-4e40-c257-b8cb36da0f03";
        :>> title = "sparse actualHours reported as discipline is more process ignorance";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "again more process ignorance";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }

    part us019 : UserStory {
        :>> id = "0a1f7c92-6eb4-4f51-8368-c9dc47eb1014";
        :>> title = "the engine mints identity; no write path depends on an AI generating a UUID by hand";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "every id minted programmatically - the write API already assigns ids, and the paths where an AI authors items directly (backlog entries, sprints, decisions) get a keel command or template that mints them";
        :>> soThat = "identity correctness stops depending on model diligence, which is the same class of vigilance D0047 says is not a control - guards 37/38 catch absence and malformation after the fact, but nothing prevents authoring one by hand";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture: their note on issue170. The gap is the DIRECT-AUTHORING paths: write-API writes already mint ids (uuid v4 in write.rs), but sprint files, backlog items, and decisions are authored as text where the AI invents each uuid - two were mangled, which is what issue170 recorded. Candidate shape: `keel new sprint|item|decision` scaffolding with minted ids, or `keel mint [n]` for use inside authoring scripts.";
    }
    part us020 : UserStory {
        :>> id = "1b2a8da3-7fc5-4062-9479-daed58fc2125";
        :>> title = "an accepted Decision carries its downstream scaffolding - backlog items, stories, and verification - or the gap is computed and visible";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "the promise-to-work chain enforced: a Decision whose consequences promise artifacts must trace to backlog items / user stories, and those to verification, with the gaps surfaced as a computed view and the worst class guarded";
        :>> soThat = "a Decision cannot promise an artifact that is never built with nothing noticing (issue174, which I accepted-with-note), and process conformance is checked by the engine rather than asserted by the AI";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process: their issue174 note, verbatim in st015, plus st014 (the guard question depends on WHAT is asserted) and st016 (sparse actualHours reported as discipline). These three notes share one root: the engine asserts process-following in places where nothing computes it. Existing partial coverage: decision-requirement-link (warning-only), verification-trace (warning-only), keel hardening decision-follow-through lens. The work: decide with the human what is being asserted per D0173-style proposal, then promote the right subset from warning to computed-view-plus-guard. Routes to a proposed Decision before any guard changes (D0070).";
    }

    #DerivedFrom dependency from us019 to st013;
    #DerivedFrom dependency from us020 to st014;
    #DerivedFrom dependency from us020 to st015;
    #DerivedFrom dependency from us020 to st016;
    #Implicates dependency from us019 to issue170;
    #Implicates dependency from us020 to issue174;

    #DerivedFrom dependency from dcMintCommand to us019;
    #DerivedFrom dependency from dcSprintScaffold to us019;
    #DerivedFrom dependency from dcDecisionFollowThroughView to us020;
    #DerivedFrom dependency from dcDecisionScaffoldingGuard to us020;

    // -- ninth intake: the production-readiness proposal is ingested, all the way down --------------
    part st017 : Statement {
        :>> id = "f8f40512-6c1b-49bb-8fce-46cb7e5bf6bb";
        :>> title = "ingest the production-readiness proposal and plan to implement every bit of it - needs, use cases, all the way down";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> text = "you should have received a new backlog item from anotehr AI process here - C:/Users/WilliamWeatherholtz/claude_code/sysmlv2-ai-toolkit/docs/proposals/2026-08-21-production-readiness-design.md.  ingest it and let's carefully plan to impelemtn every bit of it.  new needs, use cases, ... all the way down.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-21"; :>> channel = StatementChannel::chat;
    }
}
