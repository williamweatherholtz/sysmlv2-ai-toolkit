package Decision0024 {
    private import EngineWork::*;

    part d0024 : Decision {
        :>> id = "e2f3a4b5-c6d7-4e8f-9a1c-3d4e5f6a7b8c";
        :>> title = "S7 review doc-sync: update docs for dual-naming detection + whats-next subcommand";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 7 review (doc-sync step). Three documentation gaps after Sprint 7: (1) sysmlv2-syntax-notes.md §TestResult naming still referred to orient_root (renamed to orient::compute in Sprint 6) and said only DoDR infix was accepted — Sprint 7 added legacy R-suffix fallback. (2) usage.md 'See what's next' row listed only sysmlv2 orient [ROOT] — Sprint 7 added sysmlv2 whats-next [ROOT]. (3) CLAUDE.md §1 and §4 referenced only sysmlv2 orient — same gap.";
        :>> decisionText = "Applied as Sprint 7 review doc-sync. (1) sysmlv2-syntax-notes.md: updated 'TestResult naming and enum conventions' section — changed orient_root reference to orient::compute; added paragraph documenting legacy {task}R{n} fallback accepted since Sprint 7, with note that DoDR is canonical for new work. (2) usage.md: updated 'See what's next' row to list both sysmlv2 orient [ROOT] (JSON) and sysmlv2 whats-next [ROOT] (ready list, one per line). (3) CLAUDE.md §1 orient line and §4 resume line updated to reference both subcommands. Accepted: wweatherholtz standing instruction (continue through sprints), 2026-06-15.";
        :>> consequences = "Documentation now accurately reflects the Sprint 7 dual-naming done-detection and the whats-next subcommand. Future authors know to use {task}DoDR{n} for new work and that legacy {task}R{n} files are tolerated. orient::compute is the correct function name reference.";
    }
}
