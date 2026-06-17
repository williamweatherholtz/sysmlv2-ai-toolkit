// =====================================================================
// ProjectDeliveryS17 — Sprint 17: rustToolchainFix (5 pts), W1 of the D0046 audit program.
// Story: repair the Rust CLI's parity with the Python authority (validate schema-preload;
//        orient drop-cursor + phantom-task + count drift), add a Python<->Rust parity gate,
//        make the no-kernel Rust path the default validator (ends the JVM leak). issue004/005.
// Total: 5 pts  Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS17 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 17 Story
    // ====================================================================
    part rustToolchainFixStory : Story {
        :>> id = "3f205c18-79cb-4f6e-bd0c-2e4f6a8b1c2d";
        :>> title = "Sprint 17: rustToolchainFix (5 pts) — restore Rust<->Python parity; make the no-kernel path default";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient (mirrors backlog rustToolchainFixDoD).
    // ====================================================================
    action def DeliveryRunS17 {
        action storyRustToolchainFix;
        verification storyRustToolchainFixDoD : Test { :>> id = "4a316d29-8adc-4a7f-ce1d-3f5a7b9c2d3e"; :>> method = VerificationMethod::test; :>> procedureText = "rustToolchainFixDoD in backlog (pass): sysmlv2 validate . preloads schema + exits 0 on green tree; sysmlv2 orient . matches query.py orient (counts + ready set, no cursor key, no phantom tasks); parity check fails on divergence; CLAUDE.md §5 makes the no-kernel Rust path default; validate_all + cargo test + clippy green."; }
        part storyRustToolchainFixDoDR1 : TestResult { :>> id = "5b427f30-9bed-4b80-df2e-4a6b8c0d6172"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6bc1810"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustToolchainFixRefineGate : Test {
        :>> id = "5b427e30-9bed-4b80-df2e-4a6b8c0d3e4f";
        :>> title = "Sprint 17 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed (diagnosis complete pre-sprint): three bugs, one root cause = Rust parser/CLI correctness gaps + pre-D0045 cursor. (1) sysmlv2 validate . reports unresolved TestResult — a schema file fails to parse so EngineVerification never registers; root-cause in sysmlv2-parser/src/parser.rs. (2) orient ready-list polluted with 'verification ...DoR : Test {...}' phantom tasks (count 197 vs 87) — parser mis-captures verification blocks; fix parser.rs + indexer.rs. (3) orient still emits retired StateCursor — drop Cursor plumbing (orient.rs/indexer.rs/lib.rs), add in_progress_sprints to match query.py post-D0045. Then add a Python<->Rust orient parity test (cargo test or script) that fails on divergence, and rewrite CLAUDE.md §5 to make the no-kernel Rust path default (kernel fallback) — ending the JVM leak (issue004). estimatedPoints=5 (large: cross-cutting parser+CLI+docs, ~1 day; parser-depth uncertainty noted for accuracy review). Unblocked. DoD accepted.";
    }
    part rustToolchainFixRefineGateR1 : TestResult { :>> id = "6c538f41-acfe-4c91-e03f-5b7c9d1e4f50"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6701ca4"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification rustToolchainFixStandupGate : Test {
        :>> id = "7d649052-bd0f-4da2-f140-6c8d0e2f5061";
        :>> title = "Sprint 17 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Sprint 17 refine gate passed (6701ca4). rustToolchainFix: parser correctness (validate + phantom-task), drop cursor + add in_progress_sprints, parity gate, CLAUDE.md §5. No blockers.";
    }
    part rustToolchainFixStandupGateR1 : TestResult { :>> id = "6c538f41-acfe-4c91-e03f-5b7c9d1e5f61"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6bc1810"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification rustToolchainFixImplementGate : Test {
        :>> id = "8e750163-ce10-4eb3-0241-7d9e1f306172";
        :>> title = "Sprint 17 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "sysmlv2 validate . green (schema preloaded); sysmlv2 orient . parity with query.py orient (counts + ready set, no cursor, no phantom tasks); parity test added + green; CLAUDE.md §5 updated; cargo test + clippy -D warnings green; validate_all green; DoDs recorded.";
    }
    part rustToolchainFixImplementGateR1 : TestResult { :>> id = "7d649153-bd0f-4da2-f140-6c8d0e2f6172"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6bc1810"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification rustToolchainFixReviewGate : Test {
        :>> id = "9f861274-df21-4fc4-1352-8e0f20417283";
        :>> title = "Sprint 17 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "sprint-review skill Phase-3 transcript scan RUN (issue011 discipline): metrics (velocity/efficiency/accuracy incl. parser-depth estimate check) + improvement_items produced and routed to retro; diff reviewed; doc-sync complete; no regressions.";
    }
    part rustToolchainFixReviewGateR1 : TestResult { :>> id = "8e750264-ce10-4eb3-0241-7d9e1f307283"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6bc1810"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification rustToolchainFixCloseOutGate : Test {
        :>> id = "a0972385-e032-40d5-2463-9f1031528394";
        :>> title = "Sprint 17 closeOut gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): storyRustToolchainFixDoD recorded (pass); rustToolchainFixDoD in backlog (pass); .tracking validates green (sysmlv2 validate .). No human gate — sitting acceptance is the per-sitting review.";
    }
    part rustToolchainFixCloseOutGateR1 : TestResult { :>> id = "b1083496-f143-41d5-2463-9f1031520516"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "af76464"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification rustToolchainFixRetroGate : Test {
        :>> id = "c2194507-0254-42e6-3574-0a2142630516";
        :>> title = "Sprint 17 retro gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049, sprint-retro skill): avoidable issues from Sprint 17 identified and turned into tracked items — cwd-drift between Bash/PowerShell tools (issue013); parity tool spawned the kernel it was meant to replace (fixed in-sprint: kernel-free rewrite). Already-tracked: query.py undercount (issue012), reviewInvocationGuard (issue011). One-off note: 5-pt estimate held but parser-depth + tooling friction consumed the budget — accuracy calibration. No human gate.";
    }
    part rustToolchainFixRetroGateR1 : TestResult { :>> id = "d32a5618-1365-43f7-4685-1b3253740516"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "af76464"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
