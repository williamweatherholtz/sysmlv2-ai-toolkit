// =====================================================================
// ProjectDeliveryS62 — Sprint 62: issueBackfill (D0077 migration IRL-d; D0067 migration).
// Story: backfill #Resolves edges onto ALL 25 issues + turn ON the issues-guard enforcement.
//        Surfaces the genuinely-open set (issue014, issue025) each linked to a NEW open resolving
//        action (suspectDiagnostics, readdDroppedViews). Precedes IRL-e (issueResolutionSkill).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049) — dispositions human-directed (issue014/025/003).
// =====================================================================
package ProjectDeliveryS62 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from issueBackfillStory to d0077;

    part issueBackfillStory : Story {
        :>> id = "63940516-2c7d-4e8f-b062-516273849f01";
        :>> title = "Sprint 62: issueBackfill (3 pts) — #Resolves onto all 25 issues + enforce issues guard; D0077 IRL-d";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS62 {
        action storyIssueBackfill;
        verification storyIssueBackfillDoD : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f02"; :>> method = VerificationMethod::test; :>> procedureText = "issueBackfillDoD: migrated (D0067) #Resolves edges onto ALL 25 issues — 22 resolved-by-work -> their done relatedTask action; issue003 -> D0053 (resolved-by-design, the formal supersession the human asked for); issue014 -> suspectDiagnostics (NEW open action) + issue025 -> readdDroppedViews (NEW open action). Reconcile: 25 issues == 25 edges. Turned ON enforcement: added 'issues' to guards::GUARD_NAMES + wired `sysmlv2 guard issues` into the pre-commit hook on .tracking/issues.sysml changes. VERIFY: `sysmlv2 guard issues` PASS (25 triaged, 0 untriaged); `sysmlv2 open-issues` = {total 25, open 2, resolved 23} with the 2 open = issue014->suspectDiagnostics, issue025->readdDroppedViews; `sysmlv2 guard all` ALL PASS (7 guards); clippy -D warnings + cargo test (28) + rust validate green."; }
        part storyIssueBackfillDoDR1 : TestResult { :>> id = "a6aa9bc7-0329-4b06-a2a9-e5f4db06649a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification issueBackfillRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f03"; :>> title = "Sprint 62 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: IRL-d — backfill #Resolves over all 25 issues (D0067 migration: reconcile 25==25, never fabricate) + enforce the issues guard. Per-issue dispositions human-directed: issue014 (suspect mechanism is correct-but-coarse + opaque -> NEW suspectDiagnostics action: --explain/log-levels + per-task manifest + re-verify), issue025 (NEW readdDroppedViews action), issue003 (-> D0053, resolved-by-design). estimatedPoints=3. #CharteredBy to d0077. DoD accepted."; }
    part issueBackfillRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b062-516273849f04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueBackfillStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f05"; :>> title = "Sprint 62 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: (1) add suspectDiagnostics + readdDroppedViews backlog actions (open resolvers for 014/025); (2) migration script inserts 25 #Resolves edges inside issues.sysml (override 003->d0053, 014->suspectDiagnostics, 025->readdDroppedViews; else relatedTask), reconcile 25==25; (3) verify guard issues PASS + open-issues shows {014,025}; (4) enforce: GUARD_NAMES += issues + hook wiring. No blockers."; }
    part issueBackfillStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b062-516273849f06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueBackfillImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f07"; :>> title = "Sprint 62 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Added suspectDiagnostics + readdDroppedViews actions. Inserted 25 #Resolves edges (reconcile 25 issues == 25 edges; resolvers all real). GUARD_NAMES now 7 (issues enforced) + hook runs guard issues on issues.sysml changes. VERIFY (live): rust validate clean; `sysmlv2 guard issues` PASS 25/0; `sysmlv2 open-issues` = total 25, open 2, resolved 23 (open: issue014->suspectDiagnostics, issue025->readdDroppedViews); `sysmlv2 guard all` ALL PASS (7 guards); clippy --workspace -D warnings + cargo test (28) green."; }
    part issueBackfillImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b062-516273849f08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueBackfillReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f09"; :>> title = "Sprint 62 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The backfill realizes D0077's goal: resolution is now COMPUTED — 23 issues resolved (resolver done/accepted), 2 genuinely OPEN (014, 025) each linked to an open resolving action, surfaced by open-issues + orient. The reported gap (frontier empty while issues open) is closed: the engine now KNOWS there are 2 open issues + their fixing work. issue003->D0053 is the formal decision-supersedes-item representation the human asked for; the broader 'formalize when a Decision moots an item' is carried into IRL-e (the issue-resolution skill). issue014's investigation found the suspect tool correct-but-coarse + opaque (one manifest path-set over-flags all 13; no diagnostics) -> suspectDiagnostics scoped to fix that (observability + per-task manifest + re-verify) rather than rubber-stamp. Migration followed D0067 (reconcile 25==25, real resolvers, never fabricated). Next: IRL-e (issueResolutionSkill)."; }
    part issueBackfillReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b062-516273849f0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueBackfillCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f0b"; :>> title = "Sprint 62 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyIssueBackfill + backlog issueBackfill DoDs recorded pass; 25 #Resolves edges in; guard issues enforced + PASS; open-issues = 2 open / 23 resolved; clippy + cargo test (28) + rust validate + `sysmlv2 guard all` (7) ALL PASS. Self-evidencing (D0051); per-issue dispositions were human-directed. Next: IRL-e."; }
    part issueBackfillCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b062-516273849f0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueBackfillRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b062-516273849f0d"; :>> title = "Sprint 62 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the read-only audit BEFORE the migration turned a vague '25 issues' into a precise plan (22 mechanical, 3 needing a human call), and surfaced exactly 2 genuinely-open issues — the engine now computes what was previously invisible. Deferring guard enforcement to post-triage (IRL-c decision) meant turning it on here was a no-op-risk: all 25 were triaged first, so `guard all` stayed green. CAUGHT (the avoidable issue, human-flagged): issue014 was nearly closed as 'working as intended' — the human pushed to dig in, and investigation found the suspect tool is correct-but-COARSE (one manifest path-set over-flags all deliverable tasks) + OPAQUE (no --explain). LESSON: a 'flagged' item is not resolved by trusting the flagger; verify the tool's precision + observability. Captured as suspectDiagnostics (not swept under the rug). Estimate 3 pts held. Next IRL-e: the issue-resolution process + skill, which must codify decision-moots-item (issue003/D0053 pattern)."; }
    part issueBackfillRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b062-516273849f0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3d3da35"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
