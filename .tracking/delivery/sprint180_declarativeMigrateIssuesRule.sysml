// ProjectDeliveryS180 — Sprint 180: declarativeMigrateIssuesRule (D0105 migration, MIGRATE step 1).
// Express guard:issues as a declared EdgeRule (issuesTriagedRule) + prove PARITY. Extends the evaluator
// to handle INCOMING edge direction (an Issue must carry an incoming #Resolves edge). Guards still run
// (nothing retired). EXECUTE. 1 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS180 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from declMigrateIssuesStory to d0105;

    part declMigrateIssuesStory : Story {
        :>> id = "63940516-2c7d-4e8f-c180-51627384c001";
        :>> title = "Sprint 180: declarativeMigrateIssuesRule (1 pt) - guard:issues -> issuesTriagedRule EdgeRule at parity (D0105 MIGRATE)";
        :>> createdAt = "2026-07-03"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p1; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS180 {
        action storyDeclMigrateIssues;
        verification storyDeclMigrateIssuesDoD : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0105 MIGRATE step 1. (1) edge_rule_violations gains DIRECTION (outgoing/incoming): a rule can require an incoming edge (subject is the TARGET). (2) New rule .engine/rules/rules.sysml issuesTriagedRule (subject Issue, requiredEdge resolves, edgeDirection incoming, object *, atLeastOne), #JustifiedBy d0078 - mirrors guard:issues (D0077/D0078). (3) PARITY proven: unit test edge_rule_incoming_flags_untriaged_issue (an Issue with no incoming #Resolves is flagged) + LIVE: `keel rules` issuesTriagedRule 0 violations == `keel guard issues` 0 violations (56 scanned). NOTHING retired - guard:issues still runs. VERIFY: clippy -D warnings clean; cargo test (edge_rule_* both pass) green; keel validate + guard all (14) green; rule instance rust-validated (cross-file #JustifiedBy, D0048)."; }
        part storyDeclMigrateIssuesDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    }

    verification declMigrateIssuesRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c004"; :>> title = "Sprint 180 refine gate (DoR)"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: EXPAND done (178/179); MIGRATE begins with the cleanest typed-edge conformance guard (issues) + the incoming-direction the evaluator needs. estimatedPoints=1. #CharteredBy d0105. DoR met."; }
    part declMigrateIssuesRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification declMigrateIssuesStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c006"; :>> title = "Sprint 180 standup gate"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add direction to edge_rule_violations; author issuesTriagedRule (incoming resolves); prove parity vs guard:issues (unit + live). No guard retired."; }
    part declMigrateIssuesStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification declMigrateIssuesImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c008"; :>> title = "Sprint 180 implement gate"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "direction added; issuesTriagedRule authored. LIVE: keel rules issuesTriagedRule 0 == guard issues 0 (56 scanned). clippy clean; cargo test edge_rule_incoming_flags_untriaged_issue + parity test pass; keel validate + guard green."; }
    part declMigrateIssuesImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification declMigrateIssuesReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c00a"; :>> title = "Sprint 180 review gate"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Second guard reaches parity as a declared EdgeRule; the incoming-direction extension is small + unit-tested and unblocks all future 'subject is the edge target' rules. Pure EXPAND/MIGRATE alongside the guard (zero risk). Clean."; }
    part declMigrateIssuesReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification declMigrateIssuesCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c00c"; :>> title = "Sprint 180 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; 2 conformance guards now have parity rules (requirement-rootedness, issues). clippy + cargo test + keel validate + guard green. Self-evidencing (D0051)."; }
    part declMigrateIssuesCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification declMigrateIssuesRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c180-51627384c00e"; :>> title = "Sprint 180 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the second migrated guard reached parity quickly because the EdgeRule shape generalizes cleanly; the incoming-direction extension is reusable for every 'subject is target' rule. AVOIDABLE-ISSUE SCAN: none new; steady parity-gated progress. LESSON: the remaining conformance guards (charter/sprint-coverage/acceptance-events) need the SCOPE sub-language (newlyAdded/whereStatus), and process-skill is a text-reference not a typed edge - those are bigger MIGRATE increments; the pure typed-edge all-scope guards (rootedness, issues) are now done. Estimate 1 pt held. NEXT: the scope sub-language (unblocks charter/sprint-coverage), then ElementRule predicates (decision-rationale, critic-independence), then OrderingRule (ceremony); then CONTRACT retires each guard at parity."; }
    part declMigrateIssuesRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c180-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8c43446"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
}
