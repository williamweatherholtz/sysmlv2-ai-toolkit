// =====================================================================
// ProjectDeliveryS61 — Sprint 61: issueGuard (D0077 migration IRL-c).
// Story: build `sysmlv2 guard issues` (every Issue must carry a #Resolves edge; untriaged =
//        violation) + add it to guards::run_one. Enforcement (hook + guard-all) is DEFERRED to
//        IRL-d, turned on after backfill triages the issues (enforcing pre-triage would block
//        every commit). Precedes IRL-d (issueBackfill — the human's per-issue stop point).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS61 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from issueGuardStory to d0077;

    part issueGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-b061-516273849e01";
        :>> title = "Sprint 61: issueGuard (2 pts) — build `sysmlv2 guard issues` (untriaged = violation); enforcement deferred to IRL-d; D0077 IRL-c";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS61 {
        action storyIssueGuard;
        verification storyIssueGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e02"; :>> method = VerificationMethod::test; :>> procedureText = "issueGuardDoD: `sysmlv2 guard issues` built (guards::issues over view::untriaged_issues — an Issue with no #Resolves edge is a violation) + added to guards::run_one (runnable). VERIFY: `sysmlv2 guard issues` flags all 25 untriaged issues (FAIL, exit nonzero) pre-backfill — proving the mechanism; `sysmlv2 guard all` still ALL PASS (issues NOT yet in GUARD_NAMES — enforcement deferred to IRL-d, since enforcing before backfill triages issues would block every commit). clippy --workspace -D warnings + cargo test (28) green; rust validate clean."; }
        part storyIssueGuardDoDR1 : TestResult { :>> id = "303aa37d-607a-4ebb-b9dc-3e8f3295ed49"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification issueGuardRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e03"; :>> title = "Sprint 61 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: IRL-c — build the issues guard (untriaged Issue = no #Resolves edge = violation). SEQUENCING (refined from the D0077 plan): the guard MECHANISM ships here; ENFORCEMENT (hook + guard-all) is turned on at IRL-d after backfill triages the existing issues — enforcing a rule nothing yet satisfies would block every commit (chicken-and-egg). estimatedPoints=2. #CharteredBy to d0077. DoD accepted."; }
    part issueGuardRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b061-516273849e04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueGuardStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e05"; :>> title = "Sprint 61 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: view::untriaged_issues(root) -> (total, untriaged Issue names with no #Resolves edge); guards::issues wraps it into a GuardReport; add 'issues' to guards::run_one (NOT GUARD_NAMES — keeps `guard all` green until IRL-d enforces). Verify by running `sysmlv2 guard issues` (flags 25) + `guard all` (PASS). No blockers."; }
    part issueGuardStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b061-516273849e06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueGuardImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e07"; :>> title = "Sprint 61 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented view::untriaged_issues + guards::issues + 'issues' in guards::run_one. VERIFY (live): `sysmlv2 guard issues` = FAIL, 25 scanned, 25 violation(s) (every issue untriaged pre-backfill) — mechanism proven; `sysmlv2 guard all` = ALL PASS (issues deliberately NOT in GUARD_NAMES — enforcement deferred to IRL-d). clippy --workspace --all-targets -D warnings green; cargo test 28 pass; rust validate clean. The untriaged predicate (resolvers empty) is unit-covered via issue_resolution_open_vs_resolved (i3)."; }
    part issueGuardImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b061-516273849e08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueGuardReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e09"; :>> title = "Sprint 61 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. guards::issues is a thin wrapper over view::untriaged_issues (pure structure: Issue items with no resolves edge). The SEQUENCING call — ship the mechanism, defer enforcement to IRL-d after triage — is the right way to avoid enforcing a rule nothing yet satisfies (would block all commits); the issueGuard + issueBackfill backlog DoDs were refined to record this. Scoped to the untriaged check (the load-bearing 'every issue is linked' guarantee); the prose-RESOLVED-stale check was dropped as informational (D0077 makes resolution computed, not prose). Next: IRL-d (backfill) — the human's per-issue wontfix-vs-fix stop point, which also turns on enforcement."; }
    part issueGuardReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b061-516273849e0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueGuardCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e0b"; :>> title = "Sprint 61 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyIssueGuard + backlog issueGuard DoDs recorded pass; clippy + cargo test (28) + rust validate + `sysmlv2 guard all` ALL PASS; `sysmlv2 guard issues` correctly flags the 25 untriaged. Self-evidencing (D0051)."; }
    part issueGuardCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b061-516273849e0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueGuardRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b061-516273849e0d"; :>> title = "Sprint 61 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: catching the enforce-before-triage chicken-and-egg during implement (not after wiring + a blocked commit) let me ship the mechanism cleanly and defer enforcement to IRL-d — and I refined both backlog DoDs to record the sequencing rather than silently deviate. CAUGHT (avoidable): another too-long-first-doc-paragraph clippy lint on the new guard fn — fixed; this recurs across sprints, but it is mechanical + always caught by the clippy gate, so no avoidable systemic issue warrants a new tracked item. Estimate 2 pts held. NEXT IS THE HUMAN STOP POINT: IRL-d (issueBackfill) needs per-issue wontfix-vs-fix calls (e.g. issue014/025) — pausing for the human there."; }
    part issueGuardRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b061-516273849e0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cc6968"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
