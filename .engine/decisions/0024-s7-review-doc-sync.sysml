package Decision0024 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0024 : Decision {
        :>> id = "e2f3a4b5-c6d7-4e8f-9a1c-3d4e5f6a7b8c";
        :>> title = "S7 review doc-sync: update docs for dual-naming detection + whats-next subcommand";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 7 review (doc-sync step). Three documentation gaps after Sprint 7: (1) sysmlv2-syntax-notes.md §TestResult naming still referred to orient_root (renamed to orient::compute in Sprint 6) and said only DoDR infix was accepted — Sprint 7 added legacy R-suffix fallback. (2) usage.md 'See what's next' row listed only sysmlv2 orient [ROOT] — Sprint 7 added sysmlv2 whats-next [ROOT]. (3) CLAUDE.md §1 and §4 referenced only sysmlv2 orient — same gap.";
        :>> decision = "Apply Sprint 7 review doc-sync: (1) update sysmlv2-syntax-notes.md 'TestResult naming and enum conventions' — change orient_root to orient::compute and add a paragraph documenting the legacy {task}R{n} fallback accepted since Sprint 7 (DoDR canonical for new work); (2) update usage.md 'See what's next' row to list both sysmlv2 orient [ROOT] (JSON) and sysmlv2 whats-next [ROOT] (ready list, one per line); (3) update CLAUDE.md §1 orient line and §4 resume line to reference both subcommands.";
        :>> rationale = "Three docs were stale after Sprint 7: syntax-notes still named orient_root (renamed to orient::compute in Sprint 6) and omitted the legacy R-suffix fallback added in Sprint 7, and both usage.md and CLAUDE.md listed only sysmlv2 orient without the new whats-next subcommand. Future authors need to know DoDR is canonical for new work while legacy {task}R{n} files are tolerated. Accepted: wweatherholtz standing instruction (continue through sprints), 2026-06-15.";
        :>> consequences = "Documentation now accurately reflects the Sprint 7 dual-naming done-detection and the whats-next subcommand. Future authors know to use {task}DoDR{n} for new work and that legacy {task}R{n} files are tolerated. orient::compute is the correct function name reference.";
    }

    verification d0024Accept : Test { :>> id = "0a8da029-a0ac-455c-bb21-63fbc2c15b24"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0024 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0024AcceptR1 : TestResult { :>> id = "0be4514a-13f3-4354-9a39-f480cfb8cdd3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
