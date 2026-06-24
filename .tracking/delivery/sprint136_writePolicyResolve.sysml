// =====================================================================
// ProjectDeliveryS136 — Sprint 136: writePolicyResolve (resolves issue045, a CHANGE) — author D0096
// superseding d0003. The critique campaign found d0003's "writePolicy ENFORCED by the API, no
// self-promotion" unrealized AND its "same identical API for GUI+AI" overtaken by D0093's dual-surface.
// D0096 records the realized model (D0093 dual-surface + D0094 commit-gate authz); d0003 -> superseded;
// skills.sysml comments doc-synced; d0096 (a new GOVERNED decision) Core-3 critiqued -> 120/120 holds.
// Pre-accepted via the human's issue045 ACT disposition. Total: 3 pts. Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS136 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from writePolicyResolveStory to d0094;

    part writePolicyResolveStory : Story {
        :>> id = "63940516-2c7d-4e8f-c136-51627384c001";
        :>> title = "Sprint 136: writePolicyResolve (3 pts) — author D0096 superseding d0003 (resolves issue045; CHANGE)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::decision;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS136 {
        action storyWritePolicyResolve;
        verification storyWritePolicyResolveDoD : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves issue045 (Medium, human ACT-dispositioned 'supersede d0003 to the commit-gate model'). CHANGE: authored .engine/decisions/0096 superseding d0003 — its model (one identical API for GUI+AI; per-skill API-ENFORCED writePolicy; no self-promotion) is replaced by the REALIZED dual-surface + commit-gate model: CLI=authority/automation + HTML=oversight lens (differentiated, D0093); authorization = the human commit-gate, agent never auto-commits, no per-skill API enforcement (D0094); the skill writePolicy field stays a DECLARED convention. d0003 -> status superseded (note + computed back-ref). DOC-SYNC: skills.sysml header/doc comments updated to the convention-not-enforced model (the only live doc asserting the old model; CLAUDE.md + others clean). d0096 acceptance event = the issue045 ACT (judgedBy=wweatherholtz). Because d0096 is a NEW governed Decision (post-D0080), it was Core-3 critiqued (all survived) so critique-coverage stays 120/120 (d0003 drops as a superseded target, d0096 added+covered). VERIFY: keel validate 149; critique-coverage 120/120; guard critique + assured READY (issue045 closed by this resolver); guard all (13) green; the kernel validate_schema + validate_instances pass in pre-commit; no code change (write.rs already has no per-skill authz; the commit-gate is already live)."; }
        part storyWritePolicyResolveDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification writePolicyResolveRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c004"; :>> title = "Sprint 136 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue045 (d0003 writePolicy unrealized) was human ACT-dispositioned to 'supersede d0003 to the commit-gate model'. That acceptance authorizes this CHANGE. Resolver: writePolicyResolve = author D0096. estimatedPoints=3. #CharteredBy d0094. DoD accepted."; }
    part writePolicyResolveRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification writePolicyResolveStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c006"; :>> title = "Sprint 136 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach (CHANGE / §4): author D0096 (supersede d0003, re-state realized model) + acceptance event (the issue045 ACT) + flip d0003 status + doc-sync skills.sysml + Core-3 critique d0096 (new governed target). No code change. Kernel validate_schema + validate_instances run in pre-commit. No blockers."; }
    part writePolicyResolveStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification writePolicyResolveImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c008"; :>> title = "Sprint 136 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Authored D0096 (+ d0096Accept by wweatherholtz); flipped d0003 -> superseded; doc-synced skills.sysml; critiqued d0096 Core-3 (survived). VERIFY: keel validate 149; critique-coverage 120/120 (d0003 dropped, d0096 added+covered); guard critique PASS; assured READY (issue045 closed); guard all (13) PASS."; }
    part writePolicyResolveImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification writePolicyResolveReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c00a"; :>> title = "Sprint 136 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. D0096 records what is ACTUALLY true (the commit-gate authz + dual-surface) rather than leaving d0003's unrealized API-enforcement + superseded same-API claim to mislead a newcomer — a documentation-accuracy CHANGE with no code impact (write.rs already matches). Full supersession is justified (BOTH of d0003's claims were overtaken, by D0093 and by reality/D0094). The new governed decision was itself critiqued (120/120 holds). Pre-accepted via the human's issue045 ACT (D0016). Clean."; }
    part writePolicyResolveReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification writePolicyResolveCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c00c"; :>> title = "Sprint 136 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyWritePolicyResolve DoD pass; D0096 authored + accepted + critiqued, d0003 superseded, doc-synced; issue045 resolved; critique-coverage 120/120; assured READY; guard all (13) green. The human gate (issue045 ACT) authorized the CHANGE. ALL THREE post-campaign ACT resolvers now done: cargoTestGate (CI green), refreshRequirementText (next), writePolicyResolve (this)."; }
    part writePolicyResolveCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification writePolicyResolveRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c136-51627384c00e"; :>> title = "Sprint 136 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: superseding d0003 surfaced that BOTH its claims were stale (not just the writePolicy one — its 'identical API' was overtaken by D0093's dual-surface), so the full supersession is well-justified; recording the realized model fixes a misleading foundational decision. Critiquing the NEW decision (d0096) in the same sprint kept 100% coverage intact (a new governed Decision is itself a critique target). AVOIDABLE-ISSUE SCAN: one avoidable surprise caught + handled — authoring d0096 + superseding d0003 silently shifted the critique target set (d0003 out, d0096 in), tripping the critique guard until d0096 was critiqued; LESSON: a new accepted Decision is a governed critique target — critique it in the same change (like the in-commit dogfood for d0081). NON-blocking: refreshRequirementText (issue043) is the last ACT resolver. Estimate 3 pts held."; }
    part writePolicyResolveRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c136-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
