// =====================================================================
// ProjectDeliveryS64 — Sprint 64: suspectDiagnostics (resolves issue014; D0050/D0077).
// Story: make deliverable-suspicion PRECISE + EXPLAINABLE — per-task source manifest (stop the
//        coarse over-flagging) + `sysmlv2 suspect --explain` (per-task WHY) + re-verify the
//        genuinely-stale tasks at HEAD to clear the board. Resolver for the open issue014.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS64 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from suspectDiagnosticsStory to d0077;

    part suspectDiagnosticsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b064-51627384a101";
        :>> title = "Sprint 64: suspectDiagnostics (5 pts) — per-task suspect manifest + suspect --explain + re-verify; resolves issue014";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS64 {
        action storySuspectDiagnostics;
        verification storySuspectDiagnosticsDoD : Test { :>> id = "63940516-2c7d-4e8f-b064-51627384a102"; :>> method = VerificationMethod::test; :>> procedureText = "suspectDiagnosticsDoD (resolves issue014): (1) PER-TASK deliverable manifest — .engine/deliverable-manifest.txt maps each task to its OWN source paths (orient.rs load_deliverable_manifest + apply_deliverable_suspicion per-task), so a CLI change no longer flags the lexer task; verified the suspect set dropped 13->9 (rustS1Lexer/rustS3Semantic/rustS4SpecCompat/specVersionRuntimeCheck were false positives). (2) OBSERVABILITY — `sysmlv2 suspect --explain` prints per suspect task WHY (which path drifted, at which commit, vs the verified-at commit); orient carries suspect_reasons. (3) RE-VERIFY the 9 genuinely-stale deliverable tasks at HEAD (cargo test + clippy green -> append fresh TestResults judgedAgainst=HEAD) so `sysmlv2 suspect` is empty. clippy --workspace -D warnings + cargo test green."; }
    }

    verification suspectDiagnosticsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b064-51627384a103"; :>> title = "Sprint 64 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: resolves issue014 (human-directed: the suspect tool was correct-but-coarse + opaque). Three parts: per-task manifest (precision), suspect --explain (observability), re-verify at HEAD (clear the board). estimatedPoints=5. #CharteredBy to d0077. DoD accepted."; }
    part suspectDiagnosticsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b064-51627384a104"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e092d81"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification suspectDiagnosticsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b064-51627384a105"; :>> title = "Sprint 64 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: (1) rewrite deliverable-manifest.txt to per-task `task: <name> | <paths>`; orient.rs load_deliverable_manifest -> Vec<(task,paths)>, deliverable_drift_commit (most-recent drifting commit), apply_deliverable_suspicion per-task + record reasons. (2) Output.suspect_reasons (criterion/transitive/deliverable); govern::suspect(root, explain) + cmd_suspect --explain. (3) commit the source FIRST (SHA_X), then re-verify the 9 still-stale tasks at SHA_X (append-result --sha SHA_X) so source is current; commit the TestResults (backlog-only, no re-drift). No blockers."; }
    part suspectDiagnosticsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b064-51627384a106"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e092d81"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification suspectDiagnosticsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b064-51627384a107"; :>> title = "Sprint 64 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the per-task manifest + orient.rs (load_deliverable_manifest Vec<(task,paths)>, deliverable_drift_commit, per-task apply_deliverable_suspicion + suspect_reasons in Output) + `sysmlv2 suspect --explain` (govern::suspect(root, explain), cmd_suspect parses --explain). VERIFY (live): suspect set 13->9 (false positives rustS1Lexer/rustS3Semantic/rustS4SpecCompat/specVersionRuntimeCheck removed — their own source never drifted); `sysmlv2 suspect --explain` prints per-task drift (path + commit + verified-at). clippy --workspace --all-targets -D warnings + cargo test (28) green. doc-sync usage.md suspect row. RE-VERIFY of the 9 follows in closeOut (judgedAgainst = the source commit)."; }
    part suspectDiagnosticsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b064-51627384a108"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e092d81"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification suspectDiagnosticsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b064-51627384a109"; :>> title = "Sprint 64 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The per-task manifest fixes the coarseness the human suspected: a CLI change no longer flags parser/lexer tasks; the suspect set is now accurate (9 genuinely-stale, 4 false positives removed). `--explain` makes every flag accountable (path + commit + verified-at) — the observability gap closed. Resolution stays computed (no stored suspect state). The re-verify (closeOut) clears the genuinely-stale 9 at HEAD where cargo test + clippy are green — honest (the deliverable IS verified at HEAD). issue014 -> resolved once the board clears. Note for the marked-decision template: a #Prospective decision must import EngineRelationships (lesson from sprint63)."; }
    part suspectDiagnosticsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b064-51627384a10a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e092d81"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
