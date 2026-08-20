// =====================================================================
// D0043 — validator-auto-discover: shared schema order + decision lint.
// Sprint 14 retro; accepted 2026-06-16 by wweatherholtz.
// =====================================================================
package Decision0043 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0043 : Decision {
        :>> id = "70c068be-93cb-49c9-a733-a3c2cf3cb06e";
        :>> title = "D0043: shared _schema_files.py + coverage check + decision-import lint to prevent silent validator drift";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeSonnet46";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 14 retro identified two recurring failure modes: (1) new schema files require manual PRELOAD/SCHEMA/ORDER updates in three validators — Sprint 14 required adding baseline.sysml to all three; (2) decision files used wrong imports (EngineElement instead of EngineWork), causing 'Couldn't resolve Type Decision' errors that required a rewrite. Both issues were previously tracked but recurred because there was no automated guard.";
        :>> decision = "Three guards against silent validator drift: (1) extract schema ORDER to a single _schema_files.py module that all three validators import, with validate_schema.py calling check_coverage() before the kernel and exiting hard (code 2) if any schema/*.sysml is unregistered; (2) add lint_decision_imports() to validate_instances.py to fail hard pre-kernel on any decisions/*.sysml missing 'import EngineWork'; (3) add a Decision-file authoring section to sysmlv2-syntax-notes.md.";
        :>> rationale = "Sprint 14 retro identified two recurring failure modes: new schema files require manual PRELOAD/SCHEMA/ORDER updates in three validators (Sprint 14 required adding baseline.sysml to all three), and decision files used wrong imports (EngineElement instead of EngineWork) causing 'Couldn't resolve Type Decision' errors that required a rewrite. Both were previously tracked but recurred because there was no automated guard. After: adding a schema file requires only one edit (_schema_files.py); authoring a decision file with wrong imports fails immediately at validate_instances.py lint, before the expensive kernel run.";
        :>> consequences = "New .engine/tools/validate/_schema_files.py. validate_schema.py, validate_tracking.py, validate_instances.py all simplified (no more duplicated lists). validate_instances.py gains a pre-kernel lint phase. sysmlv2-syntax-notes.md gains a Decision authoring section. Future: adding a schema file requires only one edit (_schema_files.py); authoring a decision file with wrong imports fails immediately at validate_instances.py lint, before the expensive kernel run.";
    }

    verification d0043Accept : Test { :>> id = "1a90f09a-ebd6-47b8-bad7-4d6132aef133"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0043 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0043AcceptR1 : TestResult { :>> id = "2b7b8fdf-99e0-468a-9923-a3b8cd97cda2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
