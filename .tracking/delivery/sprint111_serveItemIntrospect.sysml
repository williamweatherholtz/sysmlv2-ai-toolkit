// =====================================================================
// ProjectDeliveryS111 — Sprint 111: serveItemIntrospect (D0094 console deepening). GENERIC item
// introspection: GET /api/item/:name returns ANY item's type + authored attrs + incoming/outgoing
// typed edges (with the neighbor on each), computed uniformly from the Model — one endpoint for every
// type (the uniform item/edge substrate). Frontend: every item reference (orient ready/suspect/open-
// issues, decisions, findings, processes) is now CLICK-TO-EXPAND into a detail panel, whose neighbor
// links drill further. Read-only; cached per fingerprint. Proves the substrate is strong enough.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS111 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveItemIntrospectStory to d0094;

    part serveItemIntrospectStory : Story {
        :>> id = "63940516-2c7d-4e8f-c111-51627384c001";
        :>> title = "Sprint 111: serveItemIntrospect (5 pts) — generic /api/item/:name + click-to-expand any item (D0094)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS111 {
        action storyServeItemIntrospect;
        verification storyServeItemIntrospectDoD : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0094 generic introspection (the human's 'everything introspectable/expandable'). view.rs item_detail(root,name): from the Model, return found + type + marker + sorted authored attrs + sorted/deduped incoming & outgoing typed edges (kind + neighbor) — ONE computation for ALL types (the uniform item/edge substrate; found:false if unknown). serve.rs GET /api/item/:name -> cached(item:<name>); processes_json gains the `part <X> : Process` name so processes are introspectable. Frontend: an itemLink/linkList helper renders any item name as a clickable span; a global click-delegation opens a detail MODAL (attrs table + outgoing/incoming edges, each neighbor itself an itemLink -> drill); orient (ready/suspect/open-issues/sprints), decisions, findings, and processes all render their items as clickable links; card() detail switched to trusted-HTML (label/value still escaped; the one dynamic caller — reports — escapes its detail). VERIFY (LIVE, ran this sprint): /api/item/d0094 = found Decision, 9 attrs, 7 incoming; /api/item/issue039 = found Issue, 2 incoming; /api/item/introduction = found Process, 4 attrs; the frontend serves itemLink/openItem/api/item; cached + event-driven still hold. clippy --workspace -D warnings + cargo test (46 lib) + keel validate + guard all (13) green."; }
        part storyServeItemIntrospectDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveItemIntrospectRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c004"; :>> title = "Sprint 111 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: serveLiveCache shipped (fast + event-driven); next per the plan is the generic introspect/expand tier the human wants ('everything introspectable'). Resolver: serveItemIntrospect. estimatedPoints=5. #CharteredBy d0094. DoD accepted."; }
    part serveItemIntrospectRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveItemIntrospectStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c006"; :>> title = "Sprint 111 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: view.rs item_detail (attrs+edges over the Model); serve route /api/item/:name (cached); processes_json += part name; frontend itemLink/linkList + a detail modal + global .il click delegation + wire orient/decisions/findings/processes; card() detail -> trusted HTML (escape the reports caller). Verify /api/item across a Decision/Issue/Process. No .engine/schema change -> no kernel/keystone. No blockers."; }
    part serveItemIntrospectStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveItemIntrospectImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c008"; :>> title = "Sprint 111 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented item_detail + /api/item/:name + processes name + the click-to-expand modal + clickable item references. VERIFY (live): /api/item/d0094 found Decision 9 attrs 7 incoming; issue039 found Issue 2 incoming; introduction found Process 4 attrs; frontend has itemLink/openItem/api/item. clippy -D warnings green (fixed: item_detail doc first-paragraph length); cargo test 46 lib pass; keel validate 124; guard all (13) PASS."; }
    part serveItemIntrospectImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveItemIntrospectReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c00a"; :>> title = "Sprint 111 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. One generic endpoint (item_detail) introspects EVERY item type — the uniform item/edge substrate did the work, confirming the 'are our processes strong enough' answer in practice. The console is now navigable: click any item (suspect/stale, a decision, a finding, a process) to expand its attrs + edges, then click a neighbor to drill. Read-only + cached (#View). The per-item ACTIONS (critique/add-testresult/disposition) are the final tier (serveItemActions) — flagged in the modal. Clean."; }
    part serveItemIntrospectReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveItemIntrospectCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c00c"; :>> title = "Sprint 111 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyServeItemIntrospect DoD recorded pass; generic /api/item + click-to-expand-any-item live + verified across Decision/Issue/Process. clippy + cargo test + keel validate + guard all (13) green. Self-evidencing (D0051). The console is introspectable everywhere; serveItemActions (per-item critique/add-testresult/disposition) is the last console-deepening milestone."; }
    part serveItemIntrospectCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveItemIntrospectRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c111-51627384c00e"; :>> title = "Sprint 111 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: ONE item_detail computation introspects every type because the model is uniform (id/type/attrs/typed-edges) — the 'are our processes strong enough?' answer, demonstrated; reusing the serveLiveCache `cached()` made the new endpoint instant on repeat + event-driven for free; the click-to-expand modal + clickable neighbors turn the console into a navigable graph without per-type UI. AVOIDABLE-ISSUE SCAN: one avoidable defect caught — switching card() detail to trusted HTML (so itemLink renders) silently un-escaped the report card details (server text); fixed by escaping the one dynamic caller (reports) while label/value stay escaped; lesson: when relaxing an escape, re-audit every caller's dynamic inputs. Also re-confirmed: stop the running keel.exe before `cargo build` (it locks the binary). NON-blocking: serve.rs not yet in deliverable-manifest. Estimate 5 pts held. Everything is introspectable; serveItemActions next."; }
    part serveItemIntrospectRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c111-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1a8d007"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
