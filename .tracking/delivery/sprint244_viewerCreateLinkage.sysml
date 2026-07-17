// ProjectDeliveryS244 — Sprint 244: create linkage/traceability between items (srViewerCreateLinkage,
// D0126) — /api/edge authors the closed algebra (native satisfy/allocate + markers) + a viewer Connect
// action. EXECUTE. 2 pts.
package ProjectDeliveryS244 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerCreateLinkageStory to d0126;

    part viewerCreateLinkageStory : Story {
        :>> id = "63940516-2c7d-4e8f-c244-51627384c001";
        :>> title = "Sprint 244: viewerCreateLinkage (2 pts) - author typed traceability edges between items (Connect)";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS244 {
        action storyViewerCreateLinkage;
        verification storyViewerCreateLinkageDoD : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "Delivers srViewerCreateLinkage (D0126) — the human's (2026-07-17) 'no way to create linkage/traceability to other items'. KEEL (1e7f524): write::author_edge + edge_line author the CLOSED edge algebra — native `satisfy <a> by <b>` / `allocate <a> to <b>` and the marker form `#Kind dependency from..to..`; /api/edge takes {kind|marker, from, to, file?}, file defaults to .tracking/authored.sysml (created with imports if absent); AUTHORABLE_EDGE_KINDS whitelist; N-16 supersede (marker) unchanged. VIEWER (bae7f1d): a Connect action on the node context menu — pick direction (upstream/downstream), edge kind, and a target searched by display name over the index; submits via /api/edge, runs /api/check inline, and reflects the new edge in the graph. VERIFY (demo, live): authored `#DependsOn` between two SRs end-to-end with guards-pass feedback; native satisfy + marker both write to authored.sysml + validate clean; bad kind rejected; target search resolves by displayLabel. clippy + validate + guard + check-engine green. HONEST: connect-on-create (edges within the New-item dialog) is not added — a just-created item is focusable then Connect-able; and WHICH kinds are valid per type-pair is refined by the relationship grammar (srViewerRelationshipGrammar, next)."; }
        part storyViewerCreateLinkageDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification clRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c004"; :>> title = "Sprint 244 refine gate (DoR)"; :>> createdAt = "2026-07-17"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: viewerCreateLinkage ready; scope = author native+marker edges via /api/edge + a Connect action. #CharteredBy d0126. estimatedPoints=2. DoR met."; }
    part clRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
    verification clStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c006"; :>> title = "Sprint 244 standup gate"; :>> createdAt = "2026-07-17"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: generalize edge authoring (edge_line by kind, author_edge with authored.sysml default) + a viewer Connect dialog (direction/kind/target-search). Reuse /api/check. No blockers."; }
    part clStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
    verification clImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c008"; :>> title = "Sprint 244 implement gate"; :>> createdAt = "2026-07-17"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "Implemented (keel@1e7f524, viewer@bae7f1d). Live-verified: DependsOn authored end-to-end; native satisfy + marker validate clean; bad kind rejected. clippy + validate + guard + check-engine green."; }
    part clImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
    verification clReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c00a"; :>> title = "Sprint 244 review gate"; :>> createdAt = "2026-07-17"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Linkage goes through the guarded write path over the closed algebra (native + markers), identifier-shaped, idempotent, never auto-committed; /api/check surfaces obligations. The honest caveat (per-pair validity awaits the grammar) is noted, not hidden. Clean."; }
    part clReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
    verification clCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c00c"; :>> title = "Sprint 244 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-17"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): Connect + edge authoring delivered + verified (srViewerCreateLinkage). Self-evidencing. NEXT: the relationship grammar (srViewerRelationshipGrammar) + scope-aware creation (srViewerScopedAuthoring) — a metamodel Decision, pending the human's end-of-sitting sign-off."; }
    part clCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
    verification clRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c244-51627384c00e"; :>> title = "Sprint 244 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-17"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: factored one edge-authoring primitive (edge_line + author_edge) covering native + marker forms, so /api/edge, supersede, and Connect share it; kept the algebra closed + whitelisted. AVOIDABLE-ISSUE: static server dropped again (restarted) — recurring; candidate for a tiny supervisor script if it keeps happening. LESSON: unify the write primitive across all edge kinds rather than special-casing markers. Estimate 2 pts. NEXT: relationship grammar + scope-aware creation."; }
    part clRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c244-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1e7f524"; :>> judgedAt = "2026-07-17"; :>> judgedBy = "claudeOpus"; }
}
