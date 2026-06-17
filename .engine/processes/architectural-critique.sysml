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
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;

    part critique : Process {
        :>> id = "078920a4-2b97-4a19-96ef-f834319b4cbe";
        :>> title = "Antagonistic Architectural Critique";
        :>> purpose = "Adversarially audit the engine + its processes using the CAE+GQM+ATAM framework (D0046); record findings as tracked Issue/Decision items (not prose). Operationalized by the architectural-critique skill.";
        :>> cadence = Cadence::adHoc;
    }

    part criScope : ProcessStep {
        :>> id = "44087f52-aaa9-4ee6-9719-3dd6b25ab70b";
        :>> title = "Scope inventory";
        :>> order = 1;
        :>> actionText = "Enumerate what exists: schema packages, workflows, processes, "
                       + "tooling, instance data. Fix the critique surface so nothing hides.";
        :>> producedArtifact = "Inventory list (ephemeral input to later steps).";
        :>> owner = Owner::ai;
    }
    part criMisuse : ProcessStep {
        :>> id = "e95d59cd-ebd9-4961-bac1-d8fc30a98505";
        :>> title = "SysML v2 misuse audit";
        :>> order = 2;
        :>> actionText = "For each schema/workflow construct, ask: is this how SysML v2 "
                       + "intends it? Where do we fight the language (wrong metaclass, "
                       + "stringly-typed where natives exist, reinvented natives)?";
        :>> producedArtifact = "Misuse findings, each with evidence.";
        :>> owner = Owner::ai;
    }
    part criUnused : ProcessStep {
        :>> id = "f3a224fc-cb24-40d7-944a-12757c20e702";
        :>> title = "Unused-feature audit";
        :>> order = 3;
        :>> actionText = "Sweep the SysML v2 feature set (incl. KerML + std library) for "
                       + "constructs we could leverage but don't. Check each against the "
                       + "PILOT grammar before proposing (spike or cite prior spikes).";
        :>> producedArtifact = "Feature-gap findings with pilot-feasibility verdicts.";
        :>> owner = Owner::ai;
    }
    part criUseCases : ProcessStep {
        :>> id = "cd3e78d4-446e-4df0-8732-f61ccab99db9";
        :>> title = "Top-10 use-case walkthrough";
        :>> order = 4;
        :>> actionText = "Identify the top 10 use cases of the work-tracking workflow and "
                       + "play each END TO END against the current schema + tooling. Record "
                       + "every gap, friction, or dead end encountered.";
        :>> producedArtifact = "Per-use-case walkthrough verdicts + gap list.";
        :>> owner = Owner::ai;
    }
    part criAntiPatterns : ProcessStep {
        :>> id = "0bf26c26-53b7-4d37-845b-cab55c57f45e";
        :>> title = "Anti-pattern scan";
        :>> order = 5;
        :>> actionText = "Hunt structural anti-patterns: invariant violations (does the "
                       + "engine break its own §2 rules?), state that can lie, write paths "
                       + "that bypass discipline, views that got authored, procedure that "
                       + "got embedded.";
        :>> producedArtifact = "Anti-pattern findings.";
        :>> owner = Owner::ai;
    }
    part criProcesses : ProcessStep {
        :>> id = "f903132c-0670-4f90-8bd6-9c51057f8b75";
        :>> title = "Process-layer critique";
        :>> order = 6;
        :>> actionText = "Critique the processes ON TOP of the engine (workflows, DoR/DoD, "
                       + "agile process, triage, CR path): are they followable, enforced, "
                       + "consistent with each other and the engine?";
        :>> producedArtifact = "Process findings.";
        :>> owner = Owner::ai;
    }
    part criSynthesize : ProcessStep {
        :>> id = "01a47b90-0f27-4f9d-92a9-cff44e7d1431";
        :>> title = "Record findings as tracked items";
        :>> order = 7;
        :>> actionText = "Dedupe + rank findings by severity x leverage. Record EACH as a "
                       + "tracked Issue in .tracking/issues.sysml with a concrete preventing "
                       + "change + relatedTask (NOT a prose doc). An accepted architectural "
                       + "change becomes a Decision file; a cheap+obvious guard is built now "
                       + "(D0047). No design-history report blob (prose findings were a HIGH "
                       + "critique finding; D0046).";
        :>> producedArtifact = "Tracked Issue items (+ Decisions / guards) ranked by leverage.";
        :>> owner = Owner::ai;
    }
    part criAccept : ProcessStep {
        :>> id = "66e48601-b18f-4deb-acf0-5f690c4f6ac5";
        :>> title = "Disposition at the per-sitting review";
        :>> order = 8;
        :>> actionText = "Present the ranked tracked findings at the per-sitting sprint "
                       + "review (D0049); the human accepts / defers / rejects. Accepted "
                       + "changes enter the normal CHANGE path (§3a); rejections are recorded "
                       + "Decisions (capture even when no action results). The audit does not "
                       + "block per-finding — items are already tracked.";
        :>> producedArtifact = "Human disposition recorded against the tracked findings.";
        :>> owner = Owner::human;
    }
}
