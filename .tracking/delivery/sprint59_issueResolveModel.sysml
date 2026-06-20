// =====================================================================
// ProjectDeliveryS59 — Sprint 59: issueResolveModel (D0077 migration IRL-a).
// Story: add the #Resolves typed edge to frozen schema/core (relationships.sysml; §2.5 sign-off
//        via D0077) + teach the Rust view engine to recognize 'resolves' as a known edge. First
//        of the Issue-Resolution-Loop sprints. Precedes IRL-b (open-issues view).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS59 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from issueResolveModelStory to d0077;

    part issueResolveModelStory : Story {
        :>> id = "63940516-2c7d-4e8f-b059-516273849c01";
        :>> title = "Sprint 59: issueResolveModel (2 pts) — add the #Resolves schema edge + Rust edge recognition; D0077 IRL-a";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS59 {
        action storyIssueResolveModel;
        verification storyIssueResolveModelDoD : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c02"; :>> method = VerificationMethod::test; :>> procedureText = "issueResolveModelDoD: added `metadata def Resolves;` to .engine/schema/core/relationships.sysml (the #Resolves edge, D0077; frozen-schema §2.5 sign-off via D0077) + added 'resolves' to view.rs KNOWN_EDGES so a traverse over it validates (no fail-loud UnknownEdge). VERIFY: kernel validate_schema 14/14 PASS; a view traversing #Resolves up from issue012 returned both endpoints + the edge {kind:resolves, from:retireQueryPy, to:issue012} (smoke-tested with a temp edge+view, reverted); unit test resolves_edge_is_known; clippy --workspace -D warnings + cargo test (26) green. The parser needs no change — #Resolves reuses the generic #Marker-dependency path proven by #CharteredBy."; }
        part storyIssueResolveModelDoDR1 : TestResult { :>> id = "ffd7fc5d-2723-49b9-a9a7-8e6cd661b8d3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification issueResolveModelRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c03"; :>> title = "Sprint 59 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: IRL-a — the smallest, foundational IRL step: declare the #Resolves edge (frozen schema, §2.5 sign-off given via D0077) + recognize it in the Rust view engine. Mirrors #CharteredBy exactly (metadata def + KNOWN_EDGES). estimatedPoints=2. #CharteredBy to d0077. DoD accepted."; }
    part issueResolveModelRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b059-516273849c04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolveModelStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c05"; :>> title = "Sprint 59 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: (1) `metadata def Resolves;` + doc in relationships.sysml; (2) 'resolves' in view.rs KNOWN_EDGES + unit test resolves_edge_is_known; (3) verify via kernel validate_schema + an end-to-end traverse smoke test (temp #Resolves edge + temp view, reverted). No parser change (generic #Marker-dependency path). No blockers."; }
    part issueResolveModelStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b059-516273849c06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolveModelImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c07"; :>> title = "Sprint 59 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Added `metadata def Resolves;` (relationships.sysml) + 'resolves' to view.rs KNOWN_EDGES + unit test resolves_edge_is_known. VERIFY: kernel validate_schema 14/14 PASS; end-to-end smoke test — a temp view traversing #Resolves up from issue012 returned issue012 + retireQueryPy + edge {kind:resolves} (reverted); clippy --workspace --all-targets -D warnings green; cargo test 26 pass; rust validate clean."; }
    part issueResolveModelImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b059-516273849c08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolveModelReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c09"; :>> title = "Sprint 59 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The #Resolves edge mirrors #CharteredBy (metadata def + one KNOWN_EDGES entry) — minimal frozen-schema footprint, exactly the §2.5-signed-off change. The parser already handles #Marker-dependency generically (edge_kind_from_marker lowercases), so no parser change — proven by the end-to-end traverse. Recognition is the deliverable here; the resolved/open COMPUTE that consumes these edges is IRL-b (issueOpenView). Next: IRL-b."; }
    part issueResolveModelReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b059-516273849c0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolveModelCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c0b"; :>> title = "Sprint 59 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyIssueResolveModel + backlog issueResolveModel DoDs recorded pass; kernel validate_schema 14/14 + rust validate + clippy + cargo test green; #Resolves traverse end-to-end verified. Self-evidencing (D0051)."; }
    part issueResolveModelCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b059-516273849c0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification issueResolveModelRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b059-516273849c0d"; :>> title = "Sprint 59 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: reusing the #CharteredBy marker-edge pattern made the schema edge a one-liner + zero parser change; the end-to-end traverse smoke test gave genuine proof cheaply. CAUGHT (the avoidable issue): my first smoke test appended the #Resolves edge AFTER the package's closing brace (outside the package), so it parsed to nothing and the traverse came back empty — I re-ran with the edge inserted INSIDE the package and it worked. LESSON: a marked-dependency edge must live inside the `package { }` block; a quick 'did the edge actually parse' check (endpoint appears) catches mis-placement. No new tracked item (the corrected smoke test is the check; verify-before-asserting already covers it). Estimate 2 pts held. Next IRL-b: the open-issues compute + orient surfacing that consume #Resolves."; }
    part issueResolveModelRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b059-516273849c0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "766936d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
