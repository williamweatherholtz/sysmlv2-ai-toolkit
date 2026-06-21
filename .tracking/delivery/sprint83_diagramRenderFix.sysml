// =====================================================================
// ProjectDeliveryS83 — Sprint 83: diagramRenderFix (resolves issue028; D0085/D0047). The Sprint 82
// diagram rendered BLANK: (1) CDN dependency (offline fail) + (2) cose layout on all ~2500 nodes
// (browser freeze). Fix: inline cytoscape (self-contained) + lay out only the visible subset.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS83 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from diagramRenderFixStory to d0085;

    part diagramRenderFixStory : Story {
        :>> id = "63940516-2c7d-4e8f-b083-51627384b401";
        :>> title = "Sprint 83: diagramRenderFix (2 pts) — inline cytoscape + layout-visible-only; resolves issue028 (blank diagram)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS83 {
        action storyDiagramRenderFix;
        verification storyDiagramRenderFixDoD : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b402"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves issue028: (1) inline cytoscape.js (vendored sysmlv2-cli/assets, include_str!) — no CDN, offline self-contained; (2) constructor grid layout + cose only on cy.elements(':visible') (~650), not all 2500 (froze the browser). Regression test diagram_is_self_contained_and_layouts_visible_only. VERIFY: emitted HTML 0 CDN refs + inlined lib + ':visible').layout + 0 unreplaced placeholders; ~896KB, 2519 nodes/275 edges valid JSON; clippy --workspace -D warnings + cargo test (33 lib) + rust validate + guard all (9) green. Re-delivered to the human for render-confirmation."; }
        part storyDiagramRenderFixDoDR1 : TestResult { :>> id = "b4675425-93c2-4b01-907d-ddac7cd3c553"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification diagramRenderFixRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b403"; :>> title = "Sprint 83 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the human reported the Sprint 82 diagram showed NO DATA. Diagnosis (issue028): (1) CDN-loaded cytoscape -> blank offline/blocked; (2) cose layout over all ~2500 nodes -> browser freeze. Both produce a blank. Fix: inline the vendored lib + layout only the visible subset. D0047: a recurrable defect gets a regression guard (the unit test). estimatedPoints=2. #CharteredBy d0085. DoD accepted."; }
    part diagramRenderFixRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b083-51627384b404"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification diagramRenderFixStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b405"; :>> title = "Sprint 83 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: vendor cytoscape.min.js to sysmlv2-cli/assets; CYTOSCAPE_LIB = include_str!; template <script src CDN> -> <script>/*CYTOSCAPE_LIB*/</script>; constructor layout cose-on-all -> grid; after refresh() run cose on cy.elements(':visible'); diagram_html replaces both placeholders. Add regression test. No blockers."; }
    part diagramRenderFixStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b083-51627384b406"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification diagramRenderFixImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b407"; :>> title = "Sprint 83 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented: vendored sysmlv2-cli/assets/cytoscape.min.js (373KB) + CYTOSCAPE_LIB include_str! inlined into the page; grid constructor + cose-on-visible; both placeholders replaced. VERIFY (live): `sysmlv2 diagram .` -> ~896KB self-contained HTML, 0 unpkg/CDN refs, cytoscape lib present, ':visible').layout present, 0 unreplaced placeholders, JSON parses 2519 nodes/275 edges; regression test diagram_is_self_contained_and_layouts_visible_only PASS; clippy --workspace --all-targets -D warnings green; cargo test 33 lib pass; rust validate clean; guard all (9) PASS. Re-sent the file to the human."; }
    part diagramRenderFixImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b083-51627384b408"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification diagramRenderFixReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b409"; :>> title = "Sprint 83 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. Inlining the vendored lib makes the page genuinely self-contained (the D0085 critique's flagged CDN caveat is now closed, not just noted) — and removes the offline-blank failure mode. Layout-on-visible removes the cose-on-2500 freeze while the default-hidden leaves keep it digestible; toggling all on uses the precomputed grid (no re-freeze). The regression test asserts no-CDN + inlined-lib + visible-layout, so this defect class can't silently return (D0047). Honest scope: the STRUCTURAL fixes are machine-verified; the in-browser render is the human's confirmation (re-delivered). issue028 resolves by computation (#Resolves from this done action). The vendored asset adds ~373KB to the binary — acceptable for a self-contained tool."; }
    part diagramRenderFixReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b083-51627384b40a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification diagramRenderFixCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b40b"; :>> title = "Sprint 83 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyDiagramRenderFix + backlog diagramRenderFix DoDs recorded pass; diagram inlined + layout-on-visible; regression test added; clippy + cargo test (33) + rust validate + guard all (9) green. issue028 computes RESOLVED (resolver done). Self-evidencing for the structural fixes (D0051); browser render-confirmation pending the human. D0047 honored: regression test guards the defect class."; }
    part diagramRenderFixCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b083-51627384b40c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification diagramRenderFixRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b083-51627384b40d"; :>> title = "Sprint 83 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the human caught the blank render immediately, and the two root causes (CDN + cose-on-all) were both real + fixable; inlining the lib turned the D0085-critique's KNOWN CAVEAT (CDN dependency) into a closed gap — the critique had pre-identified the weakness, which made the fix fast. CAUGHT (avoidable-issue candidate): I shipped Sprint 82 with TWO untested in-browser assumptions — that a CDN would load AND that cose could handle 2500 nodes — neither verifiable by my green rust tests; the gap was that 'valid HTML + parses' is NOT 'renders'. Lesson: a generated UI artifact's DoD must include a render-path check or an explicit human render-confirm gate, not just structural validity. The regression test now covers the structural invariants (no-CDN, layout-on-visible); the render itself still needs human eyes. Estimate 2 pts held. issue028 resolved; the diagram is self-contained + non-freezing."; }
    part diagramRenderFixRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b083-51627384b40e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd7dd07"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
