// =====================================================================
// ProjectDeliveryS42 — Sprint 42: pglCharterEnforcement (D0068/D0069).
// Story: the CHARTER guard — validate_charter.py: a newly-added delivery Story with no
//        #CharteredBy edge fails the commit (charter lineage enforced at prep, not hoped);
//        plus charter-authoring folded into the sprint-planning / DoR skill.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049). Resolver (pglViews) is next.
// =====================================================================
package ProjectDeliveryS42 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // dogfood: chartered by D0068 (charter-lineage governance) — and this very edge is what
    // the new charter guard checks for on this commit.
    #CharteredBy dependency from pglCharterEnforcementStory to d0068;

    part pglCharterEnforcementStory : Story {
        :>> id = "63940516-2c7d-4e8f-b042-516273849501";
        :>> title = "Sprint 42: pglCharterEnforcement (2 pts) — charter guard + DoR charter-authoring; D0068/D0069";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS42 {
        action storyPglCharterEnforcement;
        verification storyPglCharterEnforcementDoD : Test { :>> id = "63940516-2c7d-4e8f-b042-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "pglCharterEnforcementDoD (pass): validate_charter.py added — a NEWLY ADDED delivery Story with no #CharteredBy edge fails the commit (D0068). FORWARD-ONLY (diff-filter=A) -> no grandfather set; editing old pre-charter sprints never triggers it. --selftest 3/3 (chartered-passes, uncharted-flagged, no-added-passes). Wired into pre-commit on .tracking/delivery. Charter-authoring folded into the sprint-planning skill (step 4b + DoR checklist item). This sprint's own Story is chartered (dogfood) -> the guard passes on its own commit. validate_all green."; }
        part storyPglCharterEnforcementDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
    }

    verification pglCharterEnforcementRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b042-516273849503";
        :>> title = "Sprint 42 refine gate (DoR)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0068/D0069 charter enforcement (the part split off pglProcessChangeGuard). Same forward-only insight as sprint41: gate only NEWLY ADDED delivery files (diff-filter=A), so no grandfather set + editing old sprints never false-positives. Kernel-free (D0048). Skill edit (sprint-planning SKILL.md) is the deployment per D0059 — not a process-def file, so the keystone guard does not fire and no new Decision is needed (D0068/D0069 charter this). No blocking dependency. estimatedPoints=2. DoD accepted.";
    }
    part pglCharterEnforcementRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEnforcementStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b042-516273849505";
        :>> title = "Sprint 42 standup gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: validate_charter.py with a pure evaluate(added_delivery_texts) core + git-plumbing wrapper (diff-filter=A, git show :path). _STORY + _CHARTER regexes (Story line-anchored). Wire into pre-commit on .tracking/delivery. sprint-planning SKILL.md: add step 4b (charter the Story) + DoR checklist item. No blockers.";
    }
    part pglCharterEnforcementStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEnforcementImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b042-516273849507";
        :>> title = "Sprint 42 implement gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "validate_charter.py added; --selftest 3/3 (one fixture bug self-caught: single-line test strings didn't match the line-anchored _STORY regex that real delivery files satisfy — fixed the fixtures, not the guard). Wired into .githooks/pre-commit (fires on .tracking/delivery staged; checks only added files). sprint-planning SKILL.md: step 4b + DoR item added. pglCharterEnforcement backlog DoD recorded pass. This sprint's Story carries #CharteredBy -> d0068 so the guard passes on its own commit.";
    }
    part pglCharterEnforcementImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEnforcementReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b042-516273849509";
        :>> title = "Sprint 42 review gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the two charter guards now bracket the governance model — validate_process_change (a process change must record a Decision) + validate_charter (a sprint must charter its Story). Both forward-only (no grandfather), both kernel-free, both with --selftest. The DoR skill update closes the loop per D0059 (process deployed by skill): charter-authoring is now a planning step, not vigilance. Phase-3 scan: the authored inputs the resolver needs (process-change markers + charter edges) are now both guaranteed by guards; pglViews can compute on solid ground.";
    }
    part pglCharterEnforcementReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEnforcementCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b042-51627384950b";
        :>> title = "Sprint 42 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + pglCharterEnforcement DoD recorded pass; validate_all green; guard --selftest 3/3. No human gate (D0051: a kernel-free guard + skill-doc edit, self-evidencing; charter-lineage was accepted in D0068).";
    }
    part pglCharterEnforcementCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEnforcementRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b042-51627384950d";
        :>> title = "Sprint 42 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. WELL: the forward-only pattern, proven in sprint41, transferred directly to the charter guard — consistent, grandfather-free enforcement. The --selftest caught my own unrealistic single-line fixture before commit (the guard was right; the test was wrong) — exactly what selftests are for. No avoidable issue. Next: pglViews — the git-traversal resolver (governing-version / process-as-it-was / valid-then? / reprocess-candidates), the payoff that reads the now-guaranteed markers + charter edges. Estimate 2 pts held.";
    }
    part pglCharterEnforcementRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b042-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "03d6824"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
}
