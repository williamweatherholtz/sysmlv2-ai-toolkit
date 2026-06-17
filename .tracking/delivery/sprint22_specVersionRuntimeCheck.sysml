// =====================================================================
// ProjectDeliveryS22 — Sprint 22: specVersionRuntimeCheck (2 pts).
// Story: `sysmlv2 check --spec-version` — report the baked grammar version + fetch the
//        live upstream SHA at runtime (dep-free, curl) and report drift; --no-fetch/offline
//        skips. The runtime counterpart to the build-time S4 spec pin (user's original ask).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049); test-backed, nothing to confirm (D0051).
// =====================================================================
package ProjectDeliveryS22 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part specVersionRuntimeCheckStory : Story {
        :>> id = "8e750264-ce10-4f6e-bd44-7d9e1f306131";
        :>> title = "Sprint 22: specVersionRuntimeCheck (2 pts) — runtime grammar-staleness check";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS22 {
        action storySpecVersionRuntimeCheck;
        verification storySpecVersionRuntimeCheckDoD : Test { :>> id = "9f861375-df21-4f6e-ce45-8e0f20417131"; :>> method = VerificationMethod::test; :>> procedureText = "specVersionRuntimeCheckDoD in backlog (pass): sysmlv2 check --spec-version reports baked version + live SHA + drift; --no-fetch/offline skips; cargo test + clippy green."; }
        part storySpecVersionRuntimeCheckDoDR1 : TestResult { :>> id = "a0972486-e032-4f6e-fe46-9f1031528131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    verification specVersionRuntimeCheckRefineGate : Test {
        :>> id = "b1083597-f143-4f6e-3574-0a2142639131";
        :>> title = "Sprint 22 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach: add `check --spec-version` to the CLI — print SYSML_V2_GRAMMAR_VERSION (baked) + pinned SHA + URL; if not offline/--no-fetch, curl the spec URL, sha256 it (spec_compat::sha256_hex), report CURRENT/STALE/not-pinned; exit 1 only on confirmed stale. Dep-free (curl shell-out, no new crate). estimatedPoints=2. DoD accepted.";
    }
    part specVersionRuntimeCheckRefineGateR1 : TestResult { :>> id = "c2194608-0254-4f6e-4685-1b3253740131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification specVersionRuntimeCheckStandupGate : Test {
        :>> id = "d32a5719-1365-4f6e-5796-2c4364850131";
        :>> title = "Sprint 22 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. specVersionRuntimeCheck: cmd_spec_version in main.rs + help + manifest. No blockers.";
    }
    part specVersionRuntimeCheckStandupGateR1 : TestResult { :>> id = "e43b682a-2476-4f6e-689a-3d5475960131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification specVersionRuntimeCheckImplementGate : Test {
        :>> id = "f54c793b-3587-4f6e-79ab-4e6586a70131";
        :>> title = "Sprint 22 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "cmd_spec_version added (main.rs): baked version + live SHA via curl + CURRENT/STALE/not-pinned + exit codes; --no-fetch/offline skip; help updated; cargo test 40 + clippy -D warnings green; demonstrated online (fetched live sha, 'not pinned') + --no-fetch (skipped).";
    }
    part specVersionRuntimeCheckImplementGateR1 : TestResult { :>> id = "065d804c-4698-4f6e-8abc-5f7697b80131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification specVersionRuntimeCheckReviewGate : Test {
        :>> id = "176e915d-57a9-4f6e-9bcd-6a87a8c90131";
        :>> title = "Sprint 22 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: dep-free curl shell-out; exit 1 only on confirmed stale; added to deliverable-manifest (D0050). Phase-3 scan: source change re-suspects deliverable tasks — re-verify follows (expected D0050 loop).";
    }
    part specVersionRuntimeCheckReviewGateR1 : TestResult { :>> id = "287a026e-68ba-4f6e-acde-7b98b9da0131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification specVersionRuntimeCheckCloseOutGate : Test {
        :>> id = "398b137f-79cb-4f6e-bdef-8ca9caeb0131";
        :>> title = "Sprint 22 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + backlog DoD recorded; .tracking validates green. No human gate (D0051: test-backed).";
    }
    part specVersionRuntimeCheckCloseOutGateR1 : TestResult { :>> id = "4a9c248a-8adc-4f6e-cdef-9dba0bdc0131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification specVersionRuntimeCheckRetroGate : Test {
        :>> id = "5a9d359b-9bed-4f6e-defa-aecb1ced0131";
        :>> title = "Sprint 22 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): no new avoidable issues. Recurring source-change re-suspect friction (D0050) reaffirms the auto-reverify automation idea noted in Sprint 21. Estimate 2 pts held.";
    }
    part specVersionRuntimeCheckRetroGateR1 : TestResult { :>> id = "6a9e469c-9bed-4f6e-efab-bedc2ede0131"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ffb47b6"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
