// ProjectDeliveryS233 — Sprint 233: encodingOverrides — value-level overrides on the auto-generated
// encoding (per-value colour + swimlane/enum reorder), finishing N-18 (srEncodingOverride). Plus the
// TRIVIAL user-facing rename Seed→Focus. EXECUTE. 2 pts.
package ProjectDeliveryS233 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from encodingOverrideStory to d0120;

    part encodingOverrideStory : Story {
        :>> id = "63940516-2c7d-4e8f-c233-51627384c001";
        :>> title = "Sprint 233: encodingOverrides (2 pts) - per-value colour + swimlane reorder overrides; finishes N-18";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    satisfy srEncodingOverride by encodingOverrideStory;

    action def DeliveryRunS233 {
        action storyEncodingOverride;
        verification storyEncodingOverrideDoD : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "Satisfies srEncodingOverride (N-18/D0120) — the manual override layer over the auto-generated encoding. IMPLEMENTED (sysmlv2-viewer@5be9da8): (1) per-VALUE colour override — a colour picker per categorical value in the legend; colourScale consults state.encOverrides.colour[attr][value] (palette now hex via hslToHex). (2) swimlane/enum REORDER — ↑/↓ buttons per band; attrDomain honours state.encOverrides.order[attr] (moveInOrder swaps + stores). Both re-apply live; resetEncoding clears overrides. Also TRIVIAL: user-facing 'Seed'→'Focus' (direction-agnostic; API param `seed` unchanged). VERIFY (demo, live browser): applied colour←priority + swimlane←kind on 29 Story nodes; pinned p2 → #ff0000 (nodes turned red, confirmed rgb(255,0,0)); reordered kind bands (code,decision,… → decision,code,…); colour pickers + ↑↓ render in the legend; 0 console errors. keel validate + guard + check-engine green. WITH THIS, N-18 (axis→visual-encoding viewpoints) is FULLY satisfied: srEncodingSchemaSemantics + srEncodingSpec + srEncodingRender + srEncodingModular + srEncodingOverride."; }
        part storyEncodingOverrideDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; :>> notes = "viewer code at sysmlv2-viewer@5be9da8."; }
    }

    verification eoRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c004"; :>> title = "Sprint 233 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: finish N-18 — the override layer (the user's 'sort the enum, define colour mappings for enumerated values'). Scope: per-value colour picker + band reorder + the Seed→Focus rename. #CharteredBy d0120. estimatedPoints=2. DoR met."; }
    part eoRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification eoStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c006"; :>> title = "Sprint 233 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: state.encOverrides {colour, order}; colourScale + attrDomain consult overrides; interactive legend (colour inputs + ↑↓) wired after render → re-apply; hex palette (hslToHex). No blockers."; }
    part eoStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification eoImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c008"; :>> title = "Sprint 233 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "Override map + colourScale/attrDomain/moveInOrder + interactive legend + hex palette + rename implemented. Live-verified (p2→red; bands reordered; controls render; 0 errors). validate/guard/check-engine green."; }
    part eoImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification eoReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c00a"; :>> title = "Sprint 233 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Delivers exactly the override the human specified (sort the enum + define colour mappings for enumerated values), layered on the semantic auto-defaults — so the encoding degrades to sensible defaults while allowing precise, shareable control. N-18 is now complete across all five SRs. Rename to Focus is apt (direction-agnostic) and left the API contract untouched. Clean."; }
    part eoReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification eoCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c00c"; :>> title = "Sprint 233 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; encoding overrides delivered; N-18 fully satisfied. Self-evidencing. NEXT: remaining keel-side viewer bricks — viewerChangeImpact (distance-grouped slice), viewerExportShare (N-12), viewerBaselineCompare (N-13); or in-graph swimlane band boxes/labels polish."; }
    part eoCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification eoRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c233-51627384c00e"; :>> title = "Sprint 233 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: closed N-18 end-to-end — semantics → engine → modular registry → ELK layout → overrides — each increment verified live in a browser + recorded via governance in the same effort. The override layer honours the auto/override split the design (D0120) called for. AVOIDABLE-ISSUE SCAN: none new (the earlier comma-selector + cose-vs-layered lessons already banked). LESSON: an override layer is cheap once the encoding is data-driven — it's just a map consulted by the scales. Estimate 2 pts. NEXT: viewerChangeImpact / ExportShare / BaselineCompare."; }
    part eoRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c233-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db9b3cb"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
