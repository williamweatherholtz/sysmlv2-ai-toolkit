// =====================================================================
// ProjectDeliveryS137 — Sprint 137: refreshRequirementText (resolves issue043) — completes the D0095
// rename for LIVING requirement statements. Fixed stale artifact names in 9 living statements (sr07
// validate_all.py; sr10/sr11 binary/CLI sysmlv2; sr12/sr13 + ssr05/06/07/08 crates sysmlv2-parser/cli)
// -> keel names. The 5 SystemRequirements went stale per D0084 (statement edit, commit 8c49b9c) and
// were RE-CRITIQUED at that commit (append-new-result; the 4 stale-name correctness fails now pass) to
// restore 120/120. "SysML v2" methodology + the repo name preserved; historical records untouched.
// Two commits (edit 8c49b9c, then this re-critique+record). Total: 2 pts. Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS137 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from refreshRequirementTextStory to d0095;

    part refreshRequirementTextStory : Story {
        :>> id = "63940516-2c7d-4e8f-c137-51627384c001";
        :>> title = "Sprint 137: refreshRequirementText (2 pts) — fix stale artifact names in living requirements + re-critique (resolves issue043)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::documentation;
        :>> priority = WorkPriority::p3;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS137 {
        action storyRefreshRequirementText;
        verification storyRefreshRequirementTextDoD : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves issue043 (Low, AI-dispositioned ACT): complete the D0095 rename for LIVING requirement contracts. Fixed 9 statements: sr07AgentLoop (validate_all.py -> keel validate/guard via .githooks), sr10RustCli + sr11FastStart (binary/CLI sysmlv2 -> keel), sr12DenyWarnings + sr13SpecPin + ssr05Workspace + ssr06NoUnwrap + ssr07SpecBuild + ssr08TestCoverage (crates sysmlv2-parser/sysmlv2-cli -> keel-parser/keel-cli, incl. titles). 'SysML v2' (methodology) + the 'sysmlv2-ai-toolkit' repo name + historical sprint/decision records preserved. The 5 SystemRequirements (sr07/10/11/12/13) are critique TARGETS, so the statement edits re-suspected their 15 critiques per D0084 (semantic field=statement); committed the edit (8c49b9c), then RE-CRITIQUED at that commit via append-new-result (element-critique skill: never edit a stale critique) — the 4 correctness critiques that failed on the stale names (sr07/10/12/13) now PASS, all others re-verify pass. ssr* are SubsystemRequirements (non-targets) -> no re-critique. VERIFY: critique-coverage 115->120/120 (100% restored); keel validate 150; guard all (13) PASS; open_issues empty (issue043 closed by this resolver)."; }
        part storyRefreshRequirementTextDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification refreshRequirementTextRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c004"; :>> title = "Sprint 137 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue043 (stale artifact names in living requirement statements) AI-dispositioned ACT. Completes D0095's rename for living contracts. Resolver: refreshRequirementText. estimatedPoints=2. #CharteredBy d0095. DoD accepted."; }
    part refreshRequirementTextRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification refreshRequirementTextStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c006"; :>> title = "Sprint 137 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach (2-commit D0084 cycle): commit A edits the 9 living statements (preserve SysML-v2 + repo name + history); the 5 sr* targets go stale -> coverage dips (grandfathered, guards stay green); commit B re-critiques them at A's sha (append-new-result) -> 120/120, mark the task done. ssr* (non-targets) need no re-critique. No blockers."; }
    part refreshRequirementTextStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification refreshRequirementTextImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c008"; :>> title = "Sprint 137 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Commit A (8c49b9c): 9 statements fixed. Commit B: 15 R2 re-critique results at 8c49b9c (4 correctness fails -> pass). VERIFY: no stale 'sysmlv2'/'validate_all.py' in living requirements; critique-coverage 120/120; keel validate 150; guard all (13) PASS; open_issues empty."; }
    part refreshRequirementTextImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification refreshRequirementTextReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c00a"; :>> title = "Sprint 137 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The living requirement contracts now name the real artifacts (keel / keel-parser / keel-cli / keel validate-guard) — completing D0095 where the rename migration had missed living statements. The D0084 re-critique was done correctly (append-new-result, not editing the stale critiques per the skill anti-pattern), and the 4 previously-failing correctness critiques now legitimately pass because the names ARE fixed. 'SysML v2' / repo name / history preserved. 120/120 restored. Clean."; }
    part refreshRequirementTextReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification refreshRequirementTextCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c00c"; :>> title = "Sprint 137 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyRefreshRequirementText DoD pass; living requirement names corrected, 5 sr* re-critiqued, critique-coverage 120/120, open_issues empty, assured READY, guard all (13) green. Self-evidencing (D0051). ALL THREE post-campaign ACT resolvers now complete: cargoTestGate (CI green), writePolicyResolve (D0096), refreshRequirementText (this)."; }
    part refreshRequirementTextCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification refreshRequirementTextRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c137-51627384c00e"; :>> title = "Sprint 137 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the 2-commit D0084 cycle worked cleanly — editing a target's semantic field re-suspects its critiques, so committing the edit FIRST (to get a sha where the new statement exists) then appending R2 results at that sha restored coverage without faking anything; the grandfathered status of sr07-14 meant the intermediate dip didn't break assured. Caught + fixed MORE than issue043's enumerated scope (the ssr05-08 SubsystemRequirements had the same stale crate names) without extra churn (non-targets). AVOIDABLE-ISSUE SCAN: none new; the re-critique followed the skill's append-not-edit anti-pattern guard. LESSON (carried): editing a Need/SystemRequirement/accepted-Decision statement always triggers a D0084 re-critique cycle — budget the 2-commit sequence. All ACT resolvers from the critique campaign are now closed; open_issues empty. Estimate 2 pts held."; }
    part refreshRequirementTextRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c137-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c49b9c"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
