// ProjectDeliveryS232 — Sprint 232: elkLayeredLayout — the explorer graph adopts a mature LAYERED layout
// (ELK, in cytoscape) + label declutter, replacing force-directed cose (D0125). Answers the human's
// Graphviz question. Advances srViewerGraphExpand. EXECUTE. 2 pts.
package ProjectDeliveryS232 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from elkLayoutStory to d0125;

    part elkLayoutStory : Story {
        :>> id = "63940516-2c7d-4e8f-c232-51627384c001";
        :>> title = "Sprint 232: elkLayeredLayout (2 pts) - ELK layered layout in cytoscape + label declutter (not Graphviz)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    // advances the explorable-graph SR (stable, scalable layout)
    satisfy srViewerGraphExpand by elkLayoutStory;

    action def DeliveryRunS232 {
        action storyElkLayout;
        verification storyElkLayoutDoD : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "D0125: replace the explorer's force-directed (cose) layout with a mature LAYERED (Sugiyama) layout via ELK (cytoscape-elk + elkjs, vendored offline), keeping cytoscape's interactivity + the encoding's manual positioning. Answers the human's Graphviz question: Graphviz IS browser-renderable (viz.js) but the root cause was the layout ALGORITHM (a DAG wants layered, not force); ELK-in-cytoscape fixes it without losing interactivity/encoding (Graphviz reserved for a future static-export view). IMPLEMENTED (sysmlv2-viewer@e104bb4): runLayout uses ELK 'layered' (direction DOWN) with cose fallback; labels declutter via min-zoomed-font-size + chip backgrounds; hover/select reveals a node's label. Fixed a real bug (cytoscape style selectors don't allow comma groups — split node:selected/node.hover into two entries). VERIFY (demo, live browser): 12-node d0064 slice -> 3 clean layers (layered, not a force blob), minimal crossings, legible labels, ELK extension registered, 0 console errors. keel validate + guard + check-engine green."; }
        part storyElkLayoutDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; :>> notes = "viewer code at sysmlv2-viewer@e104bb4; vendors elk.bundled.js + cytoscape-elk."; }
    }

    verification elRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c004"; :>> title = "Sprint 232 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: human asked about Graphviz / solving text-overlap/crossings/fanout via a mature layout. Decided (D0125) ELK-in-cytoscape over Graphviz. Scope: swap runLayout to ELK + declutter labels. #CharteredBy d0125. estimatedPoints=2. DoR met."; }
    part elRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification elStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c006"; :>> title = "Sprint 232 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: vendor elkjs + cytoscape-elk (offline); register the extension; runLayout -> ELK layered with cose fallback; label style (min-zoomed-font-size + chips + hover reveal). No blockers."; }
    part elStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification elImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c008"; :>> title = "Sprint 232 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "ELK vendored + registered; runLayout ELK layered; labels decluttered; comma-selector bug fixed. Live-verified: 3 layers, legible, 0 errors. validate/guard/check-engine green."; }
    part elImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification elReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c00a"; :>> title = "Sprint 232 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Correct diagnosis + fix: the complaint was layout QUALITY, root-caused to the algorithm (force vs layered) not the toolkit; ELK-in-cytoscape delivers mature layered layout while preserving interactivity + the encoding's manual positioning — and it aligns with the SysML-v2 ecosystem (D0119). Graphviz was fairly considered + reserved for static export (documented in D0125), not adopted for the interactive surface. Clean."; }
    part elReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification elCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c00c"; :>> title = "Sprint 232 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; explorer uses ELK layered layout + decluttered labels. Self-evidencing. NEXT: srEncodingOverride (value-level overrides) + swimlane band boxes; or viewerChangeImpact/ExportShare/BaselineCompare."; }
    part elCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification elRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c232-51627384c00e"; :>> title = "Sprint 232 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: answered the human's Graphviz question with a root-cause diagnosis (algorithm, not toolkit) + a decision that fixes it while preserving interactivity/encoding — and leveraged the SysML-v2 ecosystem's own engine (ELK). AVOIDABLE-ISSUE (banked): a cytoscape style with a comma-grouped selector ('node:selected, node.hover') silently threw at cytoscape() init, nulling the graph — cytoscape selectors are NOT CSS; use one entry per selector. Verified live before committing. Estimate 2 pts. NEXT: encoding overrides / swimlane boxes / other bricks."; }
    part elRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c232-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "491b675"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
