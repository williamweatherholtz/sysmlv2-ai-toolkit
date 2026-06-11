// =====================================================================
// EngineCritiqueProcess — the antagonistic architectural critique.
//
// A repeatable, full-scale adversarial review of the ENGINE (schema, tooling,
// invariants) and the PROCESSES established on top of it. Antagonistic by
// design: every step tries to BREAK the architecture, not affirm it. The
// process ends in CHANGE REQUEST PROPOSALS — it never applies changes itself
// (each CR still needs human acceptance per CLAUDE.md §3a/§4).
//
// Findings are judged against: (1) the SysML v2 spec + pilot reality (what we
// misuse / leave unused), (2) end-to-end use-case walkthroughs (does the
// workflow actually work?), (3) the engine's own invariants (§2 — do we violate
// ourselves?).
// =====================================================================
package EngineCritiqueProcess {
    private import EngineProcess::*;

    part critique : Process {
        :>> title = "Antagonistic Architectural Critique";
        :>> purpose = "Adversarially audit the engine + its processes; end in CR proposals, never applied changes.";
        :>> cadence = "ad-hoc";
    }

    part criScope : ProcessStep {
        :>> title = "Scope inventory";
        :>> order = 1;
        :>> actionText = "Enumerate what exists: schema packages, workflows, processes, "
                       + "tooling, instance data. Fix the critique surface so nothing hides.";
        :>> producedArtifact = "Inventory list (ephemeral input to later steps).";
        :>> owner = "ai";
    }
    part criMisuse : ProcessStep {
        :>> title = "SysML v2 misuse audit";
        :>> order = 2;
        :>> actionText = "For each schema/workflow construct, ask: is this how SysML v2 "
                       + "intends it? Where do we fight the language (wrong metaclass, "
                       + "stringly-typed where natives exist, reinvented natives)?";
        :>> producedArtifact = "Misuse findings, each with evidence.";
        :>> owner = "ai";
    }
    part criUnused : ProcessStep {
        :>> title = "Unused-feature audit";
        :>> order = 3;
        :>> actionText = "Sweep the SysML v2 feature set (incl. KerML + std library) for "
                       + "constructs we could leverage but don't. Check each against the "
                       + "PILOT grammar before proposing (spike or cite prior spikes).";
        :>> producedArtifact = "Feature-gap findings with pilot-feasibility verdicts.";
        :>> owner = "ai";
    }
    part criUseCases : ProcessStep {
        :>> title = "Top-10 use-case walkthrough";
        :>> order = 4;
        :>> actionText = "Identify the top 10 use cases of the work-tracking workflow and "
                       + "play each END TO END against the current schema + tooling. Record "
                       + "every gap, friction, or dead end encountered.";
        :>> producedArtifact = "Per-use-case walkthrough verdicts + gap list.";
        :>> owner = "ai";
    }
    part criAntiPatterns : ProcessStep {
        :>> title = "Anti-pattern scan";
        :>> order = 5;
        :>> actionText = "Hunt structural anti-patterns: invariant violations (does the "
                       + "engine break its own §2 rules?), state that can lie, write paths "
                       + "that bypass discipline, views that got authored, procedure that "
                       + "got embedded.";
        :>> producedArtifact = "Anti-pattern findings.";
        :>> owner = "ai";
    }
    part criProcesses : ProcessStep {
        :>> title = "Process-layer critique";
        :>> order = 6;
        :>> actionText = "Critique the processes ON TOP of the engine (workflows, DoR/DoD, "
                       + "agile process, triage, CR path): are they followable, enforced, "
                       + "consistent with each other and the engine?";
        :>> producedArtifact = "Process findings.";
        :>> owner = "ai";
    }
    part criSynthesize : ProcessStep {
        :>> title = "Synthesize change-request proposals";
        :>> order = 7;
        :>> actionText = "Dedupe + rank all findings by severity/leverage; convert each "
                       + "actionable finding into a CR PROPOSAL (change, rationale, impact). "
                       + "Present for human acceptance; apply NOTHING.";
        :>> producedArtifact = "Ranked CR proposal list awaiting human acceptance.";
        :>> owner = "ai";
    }
    part criAccept : ProcessStep {
        :>> title = "Human disposition";
        :>> order = 8;
        :>> actionText = "Human accepts / rejects / defers each CR proposal. Accepted ones "
                       + "enter the normal CHANGE path (§3a); rejections are recorded "
                       + "Decisions (capture even when no action results).";
        :>> producedArtifact = "Accepted CRs as backlog work items + recorded Decisions.";
        :>> owner = "human";
    }
}
