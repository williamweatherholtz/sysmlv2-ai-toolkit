// =====================================================================
// D0043 — validator-auto-discover: shared schema order + decision lint.
// Sprint 14 retro; accepted 2026-06-16 by wweatherholtz.
// =====================================================================
package Decision0043 {
    private import EngineWork::*;

    part d0043 : Decision {
        :>> id = "70c068be-93cb-49c9-a733-a3c2cf3cb06e";
        :>> title = "D0043: shared _schema_files.py + coverage check + decision-import lint to prevent silent validator drift";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeSonnet46"; :>> createdBy = "claudeSonnet46";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 14 retro identified two recurring failure modes: (1) new schema files require manual PRELOAD/SCHEMA/ORDER updates in three validators — Sprint 14 required adding baseline.sysml to all three; (2) decision files used wrong imports (EngineElement instead of EngineWork), causing 'Couldn't resolve Type Decision' errors that required a rewrite. Both issues were previously tracked but recurred because there was no automated guard.";
        :>> decisionText = "Three changes: (1) Extract schema ORDER to a single _schema_files.py module in .engine/tools/validate/. All three validators import SCHEMA_ORDER from it. validate_schema.py calls check_coverage() before starting the kernel and exits hard (code 2) if any schema/*.sysml is unregistered — new schema files cannot be silently missed. (2) Add lint_decision_imports() to validate_instances.py: before the kernel starts, check every .engine/decisions/*.sysml for 'import EngineWork' and fail hard if missing. (3) Add Decision file authoring section to sysmlv2-syntax-notes.md: correct import, correct fields, template-copy instruction. Accepted: wweatherholtz 2026-06-16 at commit 07fed97.";
        :>> consequences = "New .engine/tools/validate/_schema_files.py. validate_schema.py, validate_tracking.py, validate_instances.py all simplified (no more duplicated lists). validate_instances.py gains a pre-kernel lint phase. sysmlv2-syntax-notes.md gains a Decision authoring section. Future: adding a schema file requires only one edit (_schema_files.py); authoring a decision file with wrong imports fails immediately at validate_instances.py lint, before the expensive kernel run.";
    }
}
