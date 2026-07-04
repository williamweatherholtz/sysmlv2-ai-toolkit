// ProjectDeliveryS185 — Sprint 185: serveBusinessNeedsView (D0105 / the serve gap the human hit).
// `keel serve` had no Business/Needs view — the Brief/Need/Persona/UseCase layer was invisible ("serve
// isn't showing me the business sysml data"). Add view::business + `keel business` + /api/business +
// a console Business tab. EXECUTE. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS185 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveBizStory to d0105;

    part serveBizStory : Story {
        :>> id = "63940516-2c7d-4e8f-c185-51627384c001";
        :>> title = "Sprint 185: serveBusinessNeedsView (2 pt) - render the Business layer (Brief/Needs/Personas/UseCases) in keel + serve";
        :>> createdAt = "2026-07-03"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS185 {
        action storyServeBiz;
        verification storyServeBizDoD : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves the serve Business-layer gap (the human: 'serve isn't showing me the business sysml data'). (1) view::business(root): a computed #View JSON of the Brief(s) (problem/opportunity/constraints), Personas (description/goals), Needs (statement/priority/source + a `decomposed` flag = some SystemRequirement satisfy-links it), and UseCases. (2) `keel business [ROOT]` CLI. (3) /api/business serve route + api_business handler (cached like the other views). (4) console.html Business tab rendering Brief/Needs/Personas/UseCases (Needs flagged 'no SR yet' vs 'decomposed'). VERIFIED LIVE: `keel business .` -> 1 brief, 5 personas, 17 needs. clippy -D warnings clean; cargo test (10 suites) green; keel validate (198) + guard all (14) green. Serves downstream (ignite) + this repo."; }
        part storyServeBizDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveBizRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c004"; :>> title = "Sprint 185 refine gate (DoR)"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the serve Business/Needs browser (queued serveBusinessNeedsView; the gap the human hit). estimatedPoints=2. #CharteredBy d0105. DoR met."; }
    part serveBizRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification serveBizStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c006"; :>> title = "Sprint 185 standup gate"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: view::business over the Model (Brief/Persona/Need/UseCase by type); keel business CLI; /api/business cached route; console Business tab. No blockers."; }
    part serveBizStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification serveBizImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c008"; :>> title = "Sprint 185 implement gate"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "view::business + keel business + /api/business + console tab landed. LIVE: keel business . -> 1 brief/5 personas/17 needs. clippy -D clean; cargo test 10 suites ok; keel validate 198 + guard green."; }
    part serveBizImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification serveBizReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c00a"; :>> title = "Sprint 185 review gate"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Closes the concrete gap the human reported. The view is a pure computed #View (no stored state), the Need `decomposed` flag surfaces the trace frontier (the point of a Needs browser), and it reuses the Model substrate + the standard cached serve route. Works for this repo + downstream (ignite). Clean."; }
    part serveBizReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification serveBizCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c00c"; :>> title = "Sprint 185 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; serve now renders the Business layer (CLI + API + console tab). clippy + cargo test + keel validate + guard green. Self-evidencing (D0051)."; }
    part serveBizCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
    verification serveBizRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c185-51627384c00e"; :>> title = "Sprint 185 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-03"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: a concrete gap the human actually hit is closed, and it reused the Model + cached-route patterns so it was small; the Need decomposed-flag makes the browser useful (shows the trace frontier), not just a dump. AVOIDABLE-ISSUE SCAN: the recurring clippy doc-backtick lints on new doc comments remain the only friction (now a known tax on every new view fn). Estimate 2 pt held. Remaining queue: keel-check name reconciliation, viewpoint-renderer (borderline rule), issue056 C12 (two-thread protocol — human), issue058 detecting rule."; }
    part serveBizRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c185-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "51fa537"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "claudeOpus"; }
}
