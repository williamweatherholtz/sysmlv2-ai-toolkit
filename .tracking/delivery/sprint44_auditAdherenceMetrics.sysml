// =====================================================================
// ProjectDeliveryS44 — Sprint 44: auditAdherenceMetrics (D0046).
// Story: operationalize the architectural-critique GQM adherence metrics as ONE command
//        (query.py audit) — retrospective, grandfather-aware; wire into the architectural-
//        critique skill; run it; file the cross-cutting findings as tracked Issues (D0046:
//        findings -> tracked items, not prose). Triggered by the user's process audit.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS44 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // chartered by D0046 (the CAE+GQM+ATAM audit framework this operationalizes).
    #CharteredBy dependency from auditAdherenceMetricsStory to d0046;

    part auditAdherenceMetricsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b044-516273849501";
        :>> title = "Sprint 44: auditAdherenceMetrics (2 pts) — query.py audit operationalizes the architectural-critique GQM adherence metrics; D0046";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS44 {
        action storyAuditAdherenceMetrics;
        verification storyAuditAdherenceMetricsDoD : Test { :>> id = "63940516-2c7d-4e8f-b044-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "auditAdherenceMetricsDoD (pass): query.py audit added (retrospective, kernel-free) — charter coverage + ceremony completeness + estimation discipline + sitting-review currency, each split ACTIONABLE vs grandfathered (charter since sprint38; ceremony post-issue010). Wired into architectural-critique SKILL.md (3 GQM rows + one-shot note). RAN on full history: actionable charter=0, actionable ceremony=0 (recent sprints comply); 2 findings filed as tracked Issues (issue022 actualHours dormant, issue023 sitting-review informal) + their disposition tasks (actualHoursDisposition, sittingReviewRecord, pending user). validate_all green."; }
        part storyAuditAdherenceMetricsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
    }

    verification auditAdherenceMetricsRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b044-516273849503";
        :>> title = "Sprint 44 refine gate (DoR)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: triggered by the user's request to audit the recent sprints + check the auditing process. Investigation found we ALREADY have a robust audit process (architectural-critique, D0046, CAE+GQM+ATAM) + the per-commit guard stack — so NOT a new process (avoid duplicate orphan). The real gap: the GQM adherence metrics were computed ad hoc, no single command. Scope = operationalize them as query.py audit + wire into the existing skill + file findings as Issues. Kernel-free; no process-def change (no keystone-guard trigger). Chartered by D0046. estimatedPoints=2. DoD accepted.";
    }
    part auditAdherenceMetricsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification auditAdherenceMetricsStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b044-516273849505";
        :>> title = "Sprint 44 standup gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: query.py audit_report() reusing read_charter_edges + delivery-file scan; grandfather cutoffs mirror the guards (charter sprint38, ceremony post-issue010); CLI `audit`. Wire into architectural-critique SKILL.md GQM table. File issue022/023 + disposition tasks. No blockers.";
    }
    part auditAdherenceMetricsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification auditAdherenceMetricsImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b044-516273849507";
        :>> title = "Sprint 44 implement gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "query.py audit added + grandfather-aware (re-run: actionable charter=0, actionable ceremony=0, 38 grandfathered charter / 2 grandfathered ceremony; 45/46 no actualHours). SELF-CAUGHT during implement: a loose sitting-review prose grep false-positived on THIS sprint's own text (re-running the audit after adding sprint44 made the finding vanish for the wrong reason) — removed that unreliable heuristic; sitting-review currency stays tracked by issue023 until sittingReviewRecord defines a real artifact to match. architectural-critique SKILL.md: 3 GQM rows + one-shot adherence note. issue022 (actualHours) + issue023 (sitting-review) filed; actualHoursDisposition + sittingReviewRecord tasks added (method=confirmation, pending user). auditAdherenceMetrics backlog DoD recorded pass. Story chartered to d0046.";
    }
    part auditAdherenceMetricsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification auditAdherenceMetricsReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b044-516273849509";
        :>> title = "Sprint 44 review gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the right discipline was applied — checked for an existing audit capability BEFORE building, found architectural-critique/D0046, and ENHANCED it (operationalized its GQM metrics) rather than creating a duplicate process+decision (which would itself be an orphan against D0059). The audit is grandfather-aware so it reports actionable signal, not historical noise. Findings became tracked Issues (D0046 mandate), not prose. The two findings (actualHours, sitting-review) are genuine and correctly escalated to the user as decisions (method=confirmation) rather than auto-resolved (D0016/D0051). Phase-3 scan: the audit verdict — recent sprints (38-43) hold to ceremony + charter + all guards — is honest and evidenced.";
    }
    part auditAdherenceMetricsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification auditAdherenceMetricsCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b044-51627384950b";
        :>> title = "Sprint 44 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + auditAdherenceMetrics DoD recorded pass; validate_all green. No human gate for the TOOLING (D0051: kernel-free command + skill-doc + issues, self-evidencing). The two FINDINGS, however, escalate genuine human decisions (issue022/023) — surfaced, not closed.";
    }
    part auditAdherenceMetricsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification auditAdherenceMetricsRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b044-51627384950d";
        :>> title = "Sprint 44 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. WELL: the 'check for existing before building' discipline prevented a duplicate audit process — the user's 'make one' premise was false (we have architectural-critique/D0046), and the honest move was to enhance it. The audit found two REAL latent gaps (actualHours dormant since the AI-autonomous transition; sitting-review never recorded) that had silently drifted — exactly what a periodic audit is for. AVOIDABLE-ISSUE NOTE: both gaps were silent doctrine-drift (a discipline lapsed without a superseding decision) — the same class as issue020; the audit command now makes that class visible going forward. ALSO WELL: re-running the audit after adding this sprint exposed a false-positive in my own sitting-review heuristic (it matched prose, not records) — caught + removed before commit; the lesson: an audit metric must match a real ARTIFACT, not a prose mention, else it lies. No new avoidable rework. Next: the user disposes issue022/023; then queryDrivenViewpoints (sprint, D0071). Estimate 2 pts held.";
    }
    part auditAdherenceMetricsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b044-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "696bfd6"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
}
