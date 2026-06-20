// =====================================================================
// ProjectDeliveryS66 — Sprint 66: safeTrackingFallback (resolves issue026; D0048/D0047).
// Story: remove the unsafe kernel validate_tracking fallback from the pre-commit hook — it
//        per-file false-fails the #CharteredBy charter edges (.tracking -> .engine Decisions).
//        The Rust authority (D0048) is the sole correct .tracking validator. Audit follow-up.
// Total: 1 pt  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS66 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from safeTrackingFallbackStory to d0048;

    part safeTrackingFallbackStory : Story {
        :>> id = "63940516-2c7d-4e8f-b066-51627384a301";
        :>> title = "Sprint 66: safeTrackingFallback (1 pt) — remove the unsafe kernel validate_tracking hook fallback; resolves issue026";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 1;
    }

    action def DeliveryRunS66 {
        action storySafeTrackingFallback;
        verification storySafeTrackingFallbackDoD : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a302"; :>> method = VerificationMethod::test; :>> procedureText = "safeTrackingFallbackDoD (resolves issue026): .githooks/pre-commit no longer falls back to kernel validate_tracking when the rust binary is missing (it per-file false-failed the 31 #CharteredBy charter-bearing tracking files + baselines — cross-package refs the kernel cannot resolve). Instead it SKIPS .tracking validation with a clear warning to build target/release/sysmlv2; the Rust authority (D0048) is the sole correct .tracking validator. VERIFY: binary-present path still runs `sysmlv2 validate` (75->76 clean); the kernel-fallback else-branch is removed (inspection); guard issues PASS (issue026 triaged); sysmlv2 guard all + rust validate green."; }
        part storySafeTrackingFallbackDoDR1 : TestResult { :>> id = "034f83bb-0e2c-468c-82aa-2672336dc428"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification safeTrackingFallbackRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a303"; :>> title = "Sprint 66 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: resolves issue026 (sitting02 audit finding A — the hook's kernel validate_tracking fallback false-fails charter-bearing files; latent commit-blocker on a fresh clone). Fix: skip-with-warning instead of kernel-fallback (D0048 rust is the sole authority). estimatedPoints=1. #CharteredBy to d0048. DoD accepted."; }
    part safeTrackingFallbackRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b066-51627384a304"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification safeTrackingFallbackStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a305"; :>> title = "Sprint 66 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: edit the .tracking block of .githooks/pre-commit — replace the `else sysml_run validate_tracking.py` (unsafe kernel fallback) with a skip-warning (matching guard_run's binary-missing behavior). One-line behavioral change. No blockers."; }
    part safeTrackingFallbackStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b066-51627384a306"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification safeTrackingFallbackImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a307"; :>> title = "Sprint 66 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Edited .githooks/pre-commit: the .tracking else-branch now SKIPS with a clear warning (build target/release/sysmlv2) instead of kernel validate_tracking. VERIFY: binary-present path unchanged (`sysmlv2 validate` = 76 clean); the unsafe kernel fallback is gone (inspection); `sysmlv2 guard issues` PASS (issue026 triaged); `sysmlv2 guard all` ALL PASS; rust validate clean. (The else-branch is a trivial skip — no kernel run to test; the hazard was the kernel false-failing, now removed.)"; }
    part safeTrackingFallbackImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b066-51627384a308"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification safeTrackingFallbackReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a309"; :>> title = "Sprint 66 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. Removing the kernel fallback is correct: kernel validate_tracking is per-file and cannot resolve #CharteredBy cross-package refs (issue021/D0070 limitation), so it false-failed 31 files — falling back to it would block commits on a fresh clone. Skip-with-warning matches the guard_run binary-missing behavior + keeps D0048 (rust is the .tracking authority). issue026 -> resolved (its #Resolves resolver safeTrackingFallback done). Audit finding A closed. Resolution computed via the issue-resolution loop."; }
    part safeTrackingFallbackReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b066-51627384a30a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification safeTrackingFallbackCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a30b"; :>> title = "Sprint 66 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storySafeTrackingFallback + backlog safeTrackingFallback DoDs recorded pass; hook fixed; rust validate + sysmlv2 guard all (incl issues) green. issue026 now computes RESOLVED (resolver done) -> open-issues empty again. Self-evidencing (D0051). Audit finding A resolved."; }
    part safeTrackingFallbackCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b066-51627384a30c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification safeTrackingFallbackRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b066-51627384a30d"; :>> title = "Sprint 66 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the audit (sitting02) surfaced a latent hazard the per-commit guards missed (they use the rust path, which is correct; the kernel cross-check exposed the fallback's unsafety) — and it flowed straight through the new issue-resolution loop: recorded issue026, triaged it (#Resolves -> safeTrackingFallback), fixed + resolved by computation. No avoidable issue in the fix itself; the deeper note is that the kernel cross-check (validate_all) is a useful periodic audit even though it is not the per-commit authority. Estimate 1 pt held. Audit follow-up complete: finding A (issue026) resolved; finding B (sitting02 review) recorded on human attestation; finding C (actualHours) already issue022/D0072."; }
    part safeTrackingFallbackRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b066-51627384a30e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f361630"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
