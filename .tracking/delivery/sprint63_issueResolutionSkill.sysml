// =====================================================================
// ProjectDeliveryS63 — Sprint 63: issueResolutionSkill (D0077 IRL-e; D0078; D0059).
// Story: activate the issue-resolution PROCESS (.engine/processes/issue-resolution.sysml) +
//        deploying SKILL (D0059), codifying record -> triage(#Resolves) -> resolve(computed) ->
//        decision-moots-item (typed edge, not prose). Completes the D0077 IRL milestone (a..e).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049) except D0078 (human-accepted CHANGE).
// =====================================================================
package ProjectDeliveryS63 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from issueResolutionSkillStory to d0077;

    part issueResolutionSkillStory : Story {
        :>> id = "63940516-2c7d-4e8f-b063-51627384a001";
        :>> title = "Sprint 63: issueResolutionSkill (3 pts) — activate the issue-resolution process + deploying skill; completes IRL; D0077/D0078";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS63 {
        action storyIssueResolutionSkill;
        verification storyIssueResolutionSkillDoD : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a002"; :>> method = VerificationMethod::inspect; :>> procedureText = "issueResolutionSkillDoD: D0078 (#ProspectiveChange-marked, human-accepted) + the issueResolution process-def (.engine/processes/issue-resolution.sysml: record -> triage(#Resolves) -> resolve(computed) -> decision-moots-item, issue-scoped) + the deploying skill (.engine/skills/issue-resolution/SKILL.md) registered in skills-registry (D0059). doc-synced CLAUDE.md sec3c + sec5 (issues guard D0077/D0078) + usage.md (Track an issue + See open issues rows). VERIFY: kernel validate_instances 88/88 (incl D0078 + process-def + registry); guard acceptance-events PASS (75, incl D0078); the keystone (process-change guard) passes — D0078 co-committed with the process-def; rust validate clean; sysmlv2 guard all ALL PASS. Completes the D0077 IRL milestone."; }
        part storyIssueResolutionSkillDoDR1 : TestResult { :>> id = "87dc4c65-e06a-44a2-a49f-f7488f5d5876"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification issueResolutionSkillRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a003"; :>> title = "Sprint 63 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: IRL-e — codify the issue-resolution loop as a process-def + deploying skill (D0059), with D0078 the #ProspectiveChange keystone decision (human-accepted, issue-scoped decision-moots-item). Design was reviewed + signed off in the IRL-e dialogue. estimatedPoints=3. #CharteredBy to d0077. DoD accepted."; }
    part issueResolutionSkillRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b063-51627384a004"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolutionSkillStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a005"; :>> title = "Sprint 63 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: (1) D0078 (#ProspectiveChange, accepted + human acceptance event); (2) .engine/processes/issue-resolution.sysml (Process + 4 ProcessSteps mirroring doc-sync's structure); (3) .engine/skills/issue-resolution/SKILL.md + skills-registry entry (D0059); (4) doc-sync CLAUDE.md sec3c/sec5 + usage.md. Verify kernel validate_instances + acceptance-events + keystone (D0078 co-commits the process-def). No blockers."; }
    part issueResolutionSkillStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b063-51627384a006"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolutionSkillImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a007"; :>> title = "Sprint 63 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Authored D0078 (#ProspectiveChange-marked; needed `import EngineRelationships` for the marker to resolve — caught + fixed by validate_instances) + .engine/processes/issue-resolution.sysml + .engine/skills/issue-resolution/SKILL.md + skills-registry entry. doc-synced CLAUDE.md sec3c (Issue triage via #Resolves) + sec5 (seven guards incl issues) + usage.md (Track an issue + See open issues). VERIFY: kernel validate_instances 88/88 PASS (incl D0078 + process-def + registry); guard acceptance-events PASS 75; rust validate 72 clean; sysmlv2 guard all ALL PASS (7). Keystone verified at commit (D0078 co-staged with the process-def)."; }
    part issueResolutionSkillImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b063-51627384a008"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolutionSkillReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a009"; :>> title = "Sprint 63 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The process-def mirrors doc-sync's Process/ProcessStep structure; the skill follows the registry convention + has the anti-pattern watchlist (prose-RESOLVED, relatedTask-as-resolver, untriaged, decision-moot-as-prose, status-field). Step 4 (decision-moots-item) is the formalization the human asked for — issue017's 'subsumed by DXXXX' prose lapse becomes a required typed edge (#Resolves for an Issue; supersede for a Need/Requirement, issue-scoped per D0078). D0059 satisfied: the process now has a deploying skill, so issue-handling is reliable not ad hoc. Completes IRL (a..e): #Resolves edge + computed open/resolved + orient surfacing + guard + backfill + process/skill. Live open set unchanged: issue014->suspectDiagnostics, issue025->readdDroppedViews."; }
    part issueResolutionSkillReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b063-51627384a00a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolutionSkillCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a00b"; :>> title = "Sprint 63 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyIssueResolutionSkill + backlog issueResolutionSkill DoDs recorded pass; D0078 human-accepted (acceptance event passes); kernel validate_instances 88/88 + guard acceptance-events + rust validate + sysmlv2 guard all (7) green; keystone satisfied (D0078 co-commits the process-def). D0077 IRL milestone COMPLETE. Self-evidencing (D0051) except D0078 (human sign-off held)."; }
    part issueResolutionSkillCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b063-51627384a00c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolutionSkillRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b063-51627384a00d"; :>> title = "Sprint 63 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the whole IRL arc started from the human's sharp observation ('how is the backlog empty with open issues?') and ends with a closed loop — edge + computed state + orient surfacing + guard + backfill + a deployed process/skill — so the gap is structurally prevented, not just patched. Step 4 (decision-moots-item) directly answers 'formalize when a decision moots an item'. CAUGHT (the avoidable issue): D0078's #ProspectiveChange marker failed kernel validation because the file didn't import EngineRelationships (where the marker type lives) — validate_instances caught it; fixed by adding the import. LESSON: a marked Decision must import EngineRelationships; the marked-decision template should carry that import. No new tracked item (validate_instances is the guard + it worked), but worth noting for the decision template. Estimate 3 pts held. D0077 IRL milestone COMPLETE; remaining ready frontier = suspectDiagnostics + readdDroppedViews (the two open issues' resolvers)."; }
    part issueResolutionSkillRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b063-51627384a00e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9f7d0ec"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
