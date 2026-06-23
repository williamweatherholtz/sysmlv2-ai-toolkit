// =====================================================================
// ProjectDeliveryS101 — Sprint 101: sittingModel (resolves issue040; the D0049 batch-4 finding). The
// per-sitting review's coverage is now MODELED + COMPUTABLE: a review attests its sprints via #Covers
// edges (review -> sprint Story), and `sysmlv2 sitting-coverage` reports which delivery sprints have a
// covering review vs await one — the human gate's unit is no longer unmodeled (issue040). A VIEW, not
// a gate (the human reviews per sitting at their cadence, D0019; a review is the human's explicit
// confirmation — never fabricated). Recorded via the sprint-review skill (doc-synced).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS101 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from sittingModelStory to d0049;

    part sittingModelStory : Story {
        :>> id = "63940516-2c7d-4e8f-c101-51627384c001";
        :>> title = "Sprint 101: sittingModel (3 pts) — #Covers review->sprint edge + sysmlv2 sitting-coverage; resolves the D0049 finding (issue040)";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS101 {
        action storySittingModel;
        verification storySittingModelDoD : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Resolve issue040 (d0049 finding: 'sitting' / review->sprints coverage uncomputable), ACT-dispositioned by the human. Model the per-sitting review's coverage as a #Covers marker edge (review -> sprint Story); add `sysmlv2 sitting-coverage` (view.rs sitting_coverage + covered_sprints) reporting sprints / covered / uncovered + each review's covered set; wire cmd_sitting_coverage + dispatch + VIEW_SUBCOMMANDS. Recording stays the human's confirmation via the sprint-review skill (doc-synced; never fabricated). VERIFY (live): `sysmlv2 sitting-coverage` = 102 sprints / 0 covered / 102 uncovered (honest current state — no per-sitting review recorded yet); unit test sitting_coverage_detects_covered_sprints proves a #Covers edge marks a sprint covered; sittingModel marked done -> issue040 RESOLVED (open 2->1); clippy --workspace -D warnings + cargo test (45 lib) + rust validate 114 + guard all (13) green. doc-sync: sprint-review skill + CLAUDE.md views + usage.md."; }
        part storySittingModelDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }
    }

    verification sittingModelRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c004"; :>> title = "Sprint 101 refine gate (DoR)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue040 (d0049 completeness finding) human-dispositioned ACT, with the approved design 'marker edge: review #Covers sprints + computed view'. Resolver: sittingModel. estimatedPoints=3. #CharteredBy d0049. DoD accepted."; }
    part sittingModelRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification sittingModelStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c006"; :>> title = "Sprint 101 standup gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: view.rs sitting_coverage + covered_sprints (pure, self-tested); cmd_sitting_coverage + dispatch + help + VIEW_SUBCOMMANDS; document the #Covers recording convention in the sprint-review skill (markdown — no process-def edit, so no keystone) + CLAUDE.md/usage. No write-API command needed (recording is the human's confirmation via the skill). No blockers."; }
    part sittingModelStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification sittingModelImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c008"; :>> title = "Sprint 101 implement gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the #Covers view + recording convention. VERIFY (live): `sysmlv2 sitting-coverage` 102/0/102; sitting_coverage_detects_covered_sprints unit test pass; sittingModel done -> issue040 RESOLVED (open 2->1, issue039 remains); clippy -D warnings green; cargo test 45 lib pass; rust validate 114; guard all (13) PASS."; }
    part sittingModelImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification sittingModelReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c00a"; :>> title = "Sprint 101 review gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The sitting UNIT is now computable: #Covers makes review->sprint coverage a typed edge, and sitting-coverage reads it (102 uncovered = the honest state — no per-sitting review recorded yet; the view is the worklist, not a blocker). Crucially the mechanism does NOT fabricate a review — recording stays the human's explicit confirmation (D0016) via the sprint-review skill. Resolves the d0049 finding (issue040) by closing the 'uncomputable' gap structurally. Clean."; }
    part sittingModelReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification sittingModelCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c00c"; :>> title = "Sprint 101 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storySittingModel DoD recorded pass; sitting-coverage view + #Covers convention live; issue040 RESOLVED (open issues 1: issue039); clippy + cargo test (45) + rust validate 114 + guard all (13) green. Self-evidencing (D0051). Of the two batch-4 findings, issue040 is closed; issue039 (correctionGuardCoverage) remains."; }
    part sittingModelCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification sittingModelRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c101-51627384c00e"; :>> title = "Sprint 101 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the sitting model reused the established marker-edge + coverage-view pattern (cf. concern-coverage, process-skill) for a fifth time, so it was fast + consistent; keeping it a VIEW (not a guard) + refusing to fabricate a per-sitting review honored the human-confirmation invariant (D0016) — the view honestly shows 102/102 uncovered because no formal sitting review has ever been recorded, which is itself a useful surfaced fact. Recording the convention in the sprint-review skill (markdown) rather than a process-def kept it keystone-free. CAUGHT (avoidable-issue candidate, deferred not raised): sitting-coverage counts ALL Story items as uncovered with no review, so the first real sitting-review will need to #Covers many sprints at once (the batchable-per-sitting design, D0019) — there is no helper to bulk-author #Covers edges; a future write-API verb (e.g. `sysmlv2 record-sitting-review --covers s84..s101`) would lower that friction (D0054). Estimate 3 pts held. issue040 closed; next: issue039 (correctionGuardCoverage) reframed onto the now-typed disposition lifecycle."; }
    part sittingModelRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c101-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fa18c22"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }
}
