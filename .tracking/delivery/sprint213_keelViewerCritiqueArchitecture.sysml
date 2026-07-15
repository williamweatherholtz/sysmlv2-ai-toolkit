// ProjectDeliveryS213 — Sprint 213: keelViewerCritiqueArchitecture — architecture for the accepted
// critique Needs N-15 + N-16 (keelViewerNeedsAccept3R1), under shape B (D0114). Records gate 3, derives
// 2 SystemRequirements + 3 UseCases (the human's #1/#2 + in-program edit) + 2 backlog actions. All reuse
// keel substrate (element-critique / agent-bridge / apply-review). EXECUTE. 1 pt.
package ProjectDeliveryS213 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from keelViewerCritiqueStory to d0114;

    part keelViewerCritiqueStory : Story {
        :>> id = "63940516-2c7d-4e8f-c213-51627384c001";
        :>> title = "Sprint 213: keelViewerCritiqueArchitecture (1 pt) - SRs + UseCases + backlog for accepted N-15/N-16 (AI critique)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS213 {
        action storyKeelViewerCritique;
        verification storyKeelViewerCritiqueDoD : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "Architecture for the accepted critique Needs (keelViewerNeedsAccept3R1: N-15 iterative AI critique, N-16 in-program modify/critique), under shape B (D0114). AUTHORED: (1) gate 3 recorded (human-judged); (2) 2 SystemRequirements (srViewerIterativeCritique, srViewerInProgramEdit), measurable, satisfy-linked; (3) 3 UseCases — UC-14 critique a process + downstream (best-practice, the human's #1), UC-15 critique requirement comprehensiveness (semantic sufficiency, #2), UC-16 modify+re-critique in-program — each with alt/exception scenarios; (4) 2 backlog actions. All REUSE keel substrate: element-critique + serve agent bridge (N-15), apply-review guarded write-back (N-16). needs-before-architecture held. VERIFY: keel validate clean; guard all + check-engine green; SRs satisfy-linked; use cases trace the human's examples; authored items not prose."; }
        part storyKeelViewerCritiqueDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    }

    verification kcaRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c004"; :>> title = "Sprint 213 refine gate (DoR)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: N-15/N-16 accepted (keelViewerNeedsAccept3R1) under shape B. Derive SRs + use cases (incl. #1/#2) + backlog; reuse element-critique/apply-review. estimatedPoints=1. #CharteredBy d0114. DoR met."; }
    part kcaRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification kcaStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c006"; :>> title = "Sprint 213 standup gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: gate 3; 2 SRs satisfy-linked to N-15/N-16; 3 UseCases (UC-14/15/16) with scenarios; 2 backlog actions. Reuse substrate. No blockers."; }
    part kcaStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification kcaImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c008"; :>> title = "Sprint 213 implement gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Gate 3 + 2 SRs + satisfy edges + 3 UseCases + 2 backlog actions authored. keel validate clean; guard all + check-engine green."; }
    part kcaImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification kcaReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c00a"; :>> title = "Sprint 213 review gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The refocused critique capability is now a clean Need->SR->UseCase->backlog slice, and it maps almost entirely onto EXISTING keel machinery (element-critique lenses + independent-critic discipline for N-15; apply-review guarded write-back for N-16) — so the SRs are mostly 'drive the existing thing over an axis / from the viewer', not new engines. The human's two concrete examples are captured as UC-14/UC-15. needs-before-architecture held. Clean."; }
    part kcaReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification kcaCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c00c"; :>> title = "Sprint 213 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; the critique capability (N-15/N-16) has a full Need->SR->UseCase->backlog spine, all reusing keel substrate. keel validate + guard + check-engine green. Self-evidencing (D0051). NEXT: implementation — viewerKeelApi (enabler) then viewerIterativeCritique (highest-value: the human's core goal); + open questions Q2-Q6."; }
    part kcaCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification kcaRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c213-51627384c00e"; :>> title = "Sprint 213 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the design converged on the human's REAL goal (AI-assisted critique of constrained systems) through iterative feedback — cut the parts they didn't want, refocused N-9, and landed on a capability that reuses keel's existing critique + write-back substrate almost wholesale (cheap to build, high value). The #1/#2 examples became concrete use cases. Three acceptance gates kept the human in control of scope throughout. AVOIDABLE-ISSUE SCAN: none. LESSON: iterating Needs with the human (accept/cut/refocus) produces a tighter, cheaper, more-wanted design than authoring a big set upfront. Estimate 1 pt held. NEXT: implementation, viewerKeelApi -> viewerIterativeCritique."; }
    part kcaRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c213-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "83993ce"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
}
