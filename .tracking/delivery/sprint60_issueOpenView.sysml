// =====================================================================
// ProjectDeliveryS60 — Sprint 60: issueOpenView (D0077 migration IRL-b).
// Story: Rust open-issues compute — an Issue is RESOLVED iff a #Resolves resolver is complete
//        (action done OR Decision accepted), else OPEN. `sysmlv2 open-issues` + surface
//        open_issues in `sysmlv2 orient`. Precedes IRL-c (issueGuard).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS60 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from issueOpenViewStory to d0077;

    part issueOpenViewStory : Story {
        :>> id = "63940516-2c7d-4e8f-b060-516273849d01";
        :>> title = "Sprint 60: issueOpenView (3 pts) — open-issues compute + orient open_issues surfacing; D0077 IRL-b";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS60 {
        action storyIssueOpenView;
        verification storyIssueOpenViewDoD : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d02"; :>> method = VerificationMethod::test; :>> procedureText = "issueOpenViewDoD: view::compute_issue_resolution (an Issue is RESOLVED iff a #Resolves resolver is complete — action in orient's done-set OR Decision status=accepted; else OPEN; no #Resolves edge => OPEN+untriaged) + `sysmlv2 open-issues` JSON (total/open/resolved + each open issue's resolvers + untriaged flag) + orient::done_names (the single done-set authority) + an open_issues field in `sysmlv2 orient` output. VERIFY: orient emits open_issues (25 pre-backfill); `sysmlv2 open-issues` = total 25/open 25/resolved 0, all untriaged (correct — #Resolves edges land at IRL-d backfill); unit tests issue_resolution_open_vs_resolved + issue_resolved_by_accepted_decision; clippy --workspace -D warnings + cargo test (28) green; rust validate clean; sysmlv2 guard ALL PASS."; }
        part storyIssueOpenViewDoDR1 : TestResult { :>> id = "dab8258b-8ec8-491a-96e1-12898cdb4b62"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification issueOpenViewRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d03"; :>> title = "Sprint 60 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: IRL-b — compute issue open/resolved from the #Resolves edges (IRL-a) + orient's done-set + Decision-accepted, and SURFACE open_issues in orient so the frontier can't read empty while issues are open (the exact gap reported). Reuses the view Model + orient done logic. estimatedPoints=3. #CharteredBy to d0077. DoD accepted."; }
    part issueOpenViewRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b060-516273849d04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueOpenViewStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d05"; :>> title = "Sprint 60 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: view.rs compute_issue_resolution(model, done) over Issue items + resolves edges (resolver complete = done-action OR accepted-Decision); open_issues(root) JSON via the Json emitter; open_issue_names(root, done) for orient. orient.rs: done_names(root) standalone + compute_orient derives its done-set + calls open_issue_names + new Output.open_issues + to_json. main.rs `open-issues` subcommand. Unit tests for the compute. No blockers."; }
    part issueOpenViewStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b060-516273849d06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueOpenViewImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d07"; :>> title = "Sprint 60 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented view::{compute_issue_resolution, open_issues, open_issue_names} (generic over the HashSet hasher) + orient::done_names + orient Output.open_issues + to_json + main.rs `open-issues`. VERIFY (live): `sysmlv2 orient` now emits open_issues with 25 entries; `sysmlv2 open-issues` = {total 25, open 25, resolved 0}, all untriaged (correct pre-backfill). Unit tests issue_resolution_open_vs_resolved (resolved-by-done-action; open; untriaged) + issue_resolved_by_accepted_decision pass. clippy --workspace --all-targets -D warnings green; cargo test 28 pass; rust validate clean; sysmlv2 guard ALL PASS."; }
    part issueOpenViewImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b060-516273849d08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueOpenViewReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d09"; :>> title = "Sprint 60 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. Resolution is COMPUTED (no stored status, D0001/D0018): the compute reuses orient's done-set (single authority — orient::done_names) + the view Model (#Resolves edges + Decision-accepted). orient now SURFACES open_issues so the frontier can no longer read 'empty' while issues are open — directly closing the reported gap. Pre-backfill all 25 read untriaged (honest — edges arrive at IRL-d). The open-issues JSON uses the shared Json emitter. Next: IRL-c (guard: every issue must carry a #Resolves edge)."; }
    part issueOpenViewReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b060-516273849d0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueOpenViewCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d0b"; :>> title = "Sprint 60 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyIssueOpenView + backlog issueOpenView DoDs recorded pass; clippy + cargo test (28) + rust validate + sysmlv2 guard ALL PASS green; orient open_issues + `sysmlv2 open-issues` verified live (25 open pre-backfill). Self-evidencing (D0051)."; }
    part issueOpenViewCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b060-516273849d0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueOpenViewRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b060-516273849d0d"; :>> title = "Sprint 60 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: reusing orient's done-set + the view Model kept the compute tiny, and the live run immediately demonstrated the fix — 25 open issues now surface in orient where the frontier previously read 'empty'. The Json emitter (M2.2b) paid off again for the open-issues output. CAUGHT (avoidable): clippy implicit_hasher flagged the public open_issue_names taking &HashSet<String>; generalized it (and compute_issue_resolution) over <S: BuildHasher> — mechanical, no systemic issue, no new tracked item. Estimate 3 pts held. Next IRL-c (sysmlv2 guard issues + hook), then IRL-d (backfill — needs the human's per-issue wontfix-vs-fix calls)."; }
    part issueOpenViewRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b060-516273849d0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a29f76f"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
