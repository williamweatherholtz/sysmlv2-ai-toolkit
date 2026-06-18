// =====================================================================
// ProjectDeliveryS41 — Sprint 41: pglProcessChangeGuard (D0070, Increment 2c).
// Story: the KEYSTONE hard-lock guard — validate_process_change.py: a staged change to a
//        process-def file (.engine/processes|workflows) must be co-committed with a
//        #ProspectiveChange/#SafetyChange-marked Decision, or the commit fails (typos
//        included, no intent interpretation). This is what makes the governing process
//        version reliably git-computable (D0069/D0070) — no per-item stamp.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Charter guard + DoR = pglCharterEnforcement.
// =====================================================================
package ProjectDeliveryS41 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // dogfood: chartered by D0070 (which authorized the hard lock).
    #CharteredBy dependency from pglProcessChangeGuardStory to d0070;

    part pglProcessChangeGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-b041-516273849501";
        :>> title = "Sprint 41: pglProcessChangeGuard (3 pts) — keystone hard-lock guard; D0070 Increment 2c";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS41 {
        action storyPglProcessChangeGuard;
        verification storyPglProcessChangeGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-b041-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "pglProcessChangeGuardDoD (pass): validate_process_change.py added — a process-def change (.engine/processes|workflows) without a co-committed #ProspectiveChange/#SafetyChange-marked Decision fails the commit (D0070 hard lock). FORWARD-ONLY -> no grandfather set. --selftest: 5/5 (pass-with-marked, fail-unmarked, fail-no-decision, neutral, prose-marker-doesn't-count). Demonstrated live: staged a process-def change with no decision -> FAIL exit 1, then restored. Wired into pre-commit on .engine/processes|workflows. validate_all green."; }
        part storyPglProcessChangeGuardDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
    }

    verification pglProcessChangeGuardRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b041-516273849503";
        :>> title = "Sprint 41 refine gate (DoR)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0070 Increment 2c, KEYSTONE guard only (charter guard + DoR authoring split to pglCharterEnforcement). Hard-lock policy pre-accepted (user 2026-06-18: 'a hard lock ... including typos ... No need to interpret intent'). Design realization: a FORWARD-ONLY pre-commit check (this commit's staged process-def change needs this commit's marked Decision) needs NO grandfather set — simpler than the original DoD assumed. Kernel-free (D0048). No blocking dependency (the marker carrier landed in sprint40). estimatedPoints=3. DoD accepted.";
    }
    part pglProcessChangeGuardRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessChangeGuardStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b041-516273849505";
        :>> title = "Sprint 41 standup gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: validate_process_change.py with a PURE evaluate(changed, decision_texts) core (so --selftest needs no git index) + a git-plumbing wrapper (git diff --cached / git show :path). Marker regex line-anchored (prose examples don't count). Wire into pre-commit on .engine/processes|workflows. Mirrors validate_sprint_coverage.py conventions (--selftest, ERROR/PASS, SKIP_VALIDATE). No blockers.";
    }
    part pglProcessChangeGuardStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessChangeGuardImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b041-516273849507";
        :>> title = "Sprint 41 implement gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "validate_process_change.py added. --selftest 5/5 pass (pass-with-marked, fail-unmarked, fail-no-decision, neutral-no-procdef, prose-marker-does-not-count). LIVE demo: appended a scratch line to .engine/processes/doc-sync.sysml + staged it with no decision -> guard FAIL exit 1 ('process-def file(s) changed ... NO co-committed process-change Decision'); then unstaged + restored clean. Wired into .githooks/pre-commit (fires on .engine/processes|workflows staged). pglProcessChangeGuard backlog DoD recorded pass.";
    }
    part pglProcessChangeGuardImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessChangeGuardReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b041-516273849509";
        :>> title = "Sprint 41 review gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the guard closes the D0069/D0070 loop — recording is now UNAVOIDABLE (process-def change without a marked Decision cannot commit), which is precisely what makes governing-version git-computable without a per-item stamp. The forward-only design (no grandfather set) is honest: it never fabricates history (D0067), only gates new changes. The pure/plumbing split keeps --selftest deterministic. SKIP_VALIDATE remains the documented escape hatch (reverts/emergencies). Phase-3 scan: the git-traversal RESOLVER (pglViews) consumes these now-guaranteed markers; charter enforcement (pglCharterEnforcement) is the parallel guard for #CharteredBy — both correctly separate.";
    }
    part pglProcessChangeGuardReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessChangeGuardCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b041-51627384950b";
        :>> title = "Sprint 41 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + pglProcessChangeGuard DoD recorded pass; validate_all green; guard --selftest 5/5. No human gate (D0051: a kernel-free guard, self-evidencing via --selftest + the live demo; the hard-lock policy was already accepted in D0070).";
    }
    part pglProcessChangeGuardCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessChangeGuardRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b041-51627384950d";
        :>> title = "Sprint 41 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. WELL: the DoD assumed a grandfather cutoff, but designing forward-only made it unnecessary — questioning the inherited assumption simplified the guard (no stale set to maintain). The pure-core + git-plumbing split let --selftest cover all branches without a fake index. No avoidable issue. Next: pglCharterEnforcement (charter guard + DoR), then pglViews (the git-traversal resolver — the payoff that consumes the now-guaranteed markers). Estimate 3 pts held.";
    }
    part pglProcessChangeGuardRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b041-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d09307"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
}
