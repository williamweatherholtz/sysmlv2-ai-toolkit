// ProjectDeliveryS231 — Sprint 231: modularEncodingChannels — refactor the viewer's encoding engine to a
// modular channel REGISTRY (D0124): adding an encoding type = adding one module. Proven by dropping in
// `size` + `shape` channels seamlessly. Satisfies srEncodingModular (N-18). EXECUTE. 2 pts.
package ProjectDeliveryS231 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from modularEncodingStory to d0124;

    part modularEncodingStory : Story {
        :>> id = "63940516-2c7d-4e8f-c231-51627384c001";
        :>> title = "Sprint 231: modularEncodingChannels (2 pts) - encoding channels are a modular registry; add a channel = add one module";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p1; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    satisfy srEncodingModular by modularEncodingStory;

    action def DeliveryRunS231 {
        action storyModularEncoding;
        verification storyModularEncodingDoD : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "Satisfies srEncodingModular (N-18/D0124). Refactored the viewer's encoding engine (sysmlv2-viewer@258323e) from four hardcoded channels + branchy apply into a REGISTRY (ENCODE_CHANNELS): each channel is a self-contained module { id, label, role (style|x|row), kinds, prep, apply, legend }. The Encode panel (one select/channel), applyEncoding (position from x/row-role; style from style-role; row-role nests to any depth), and the legend are all GENERATED from the registry — no channel-specific branches elsewhere. PROVEN modular by adding two NEW channels with no other code change: `size` (numeric → node width/height) + `shape` (enum/categorical → node shape). VERIFY (demo, live browser): the panel generated 6 selects (colour/size/shape/x/row/nest); encoded 43 Story nodes across 5 channels (colour←priority, size←estimatedPoints → 5 distinct sizes, shape←kind → 6 distinct shapes, swimlane←kind); legend auto-generated per channel; 0 console errors. keel validate + guard + check-engine green. HONEST: a genuinely new ROLE (beyond style/x/row) needs a small role handler — the only non-one-entry case (D0124)."; }
        part storyModularEncodingDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; :>> notes = "viewer code at sysmlv2-viewer@258323e."; }
    }

    verification meRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c004"; :>> title = "Sprint 231 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the human required the encoding types be modular (add new types seamlessly). Scope: refactor to a channel registry + prove with new channels. #CharteredBy d0124. estimatedPoints=2. DoR met."; }
    part meRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification meStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c006"; :>> title = "Sprint 231 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: ENCODE_CHANNELS registry (role style|x|row); generate panel selects + apply + legend from it; style channels carry prep/apply/legend; row channels compose to nested bands; add size+shape as new modules to prove. No blockers."; }
    part meStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification meImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c008"; :>> title = "Sprint 231 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "ENCODE_CHANNELS + generated refreshEncodeControls/applyEncoding/legend + sizeScale/shapeScale implemented; panel generated from registry; size+shape channels added. Live-verified (6 selects, distinct shapes+sizes, 5 channels on 43 nodes). validate/guard/check-engine green."; }
    part meImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification meReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c00a"; :>> title = "Sprint 231 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The registry is the right abstraction: adding an encoding type is a one-module change, verified by seamlessly adding size + shape. The panel/apply/legend generality removes channel-specific branches (the rot the human pre-empted). Consistent with the generative principle (behaviour from a declared set, D0117). Honest limit noted (a new ROLE needs a small handler). Clean."; }
    part meReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification meCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c00c"; :>> title = "Sprint 231 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; encoding channels are modular (srEncodingModular satisfied). Self-evidencing. NEXT: srEncodingOverride (value-level overrides — reorder enum / pin palette / edit bins) + in-graph band labels; or the other viewer bricks (viewerChangeImpact/ExportShare/BaselineCompare)."; }
    part meCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification meRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c231-51627384c00e"; :>> title = "Sprint 231 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: took the human's modularity directive and made extensibility a first-class property (a channel registry) rather than a promise — then PROVED it by adding size + shape as one-module drops, verified live. This future-proofs the encoding feature against channel proliferation. AVOIDABLE-ISSUE SCAN: none. LESSON: when the human says 'make it modular so I can add X going forward', deliver the registry + demonstrate a new X through it, don't just tidy the existing cases. Estimate 2 pts. NEXT: overrides / other bricks."; }
    part meRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c231-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "16463f1"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
