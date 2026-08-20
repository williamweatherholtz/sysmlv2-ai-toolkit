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
}
