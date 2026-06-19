// =====================================================================
// ProjectDeliveryS50 — Sprint 50: markerRetention (D0074/D0075 migration M2.0).
// Story: retain the #Marker prefix on a Part through AST -> parser -> view engine, so governance
//        views can be expressed as TOML declarations. Prerequisite for M2.1 (subsume structural
//        query.py views). Demonstrated: process-changes.view.toml == query.py process-changes.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Born under the M0/M0b fail-loud denies.
// =====================================================================
package ProjectDeliveryS50 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from markerRetentionStory to d0075;

    part markerRetentionStory : Story {
        :>> id = "63940516-2c7d-4e8f-b050-516273849501";
        :>> title = "Sprint 50: markerRetention (3 pts) — retain #Marker on Part (AST+parser+view); D0074/D0075 M2.0";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS50 {
        action storyMarkerRetention;
        verification storyMarkerRetentionDoD : Test { :>> id = "63940516-2c7d-4e8f-b050-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "markerRetentionDoD (pass): AST Part gains marker: Option<String>; parser parse_hash_item attaches the #Marker to the emitted Part (both constructors updated); view engine ItemInfo.marker + Select.marker predicate. Parser test marker_prefixed_part_is_preserved asserts marker == ProspectiveChange; view test select_by_marker. DEMONSTRATED: .engine/views/process-changes.view.toml (select Decision + marker in [ProspectiveChange,SafetyChange]) returns d0039/d0049/d0064/d0067/d0070 — identical to query.py process-changes (M2.1 subsumption proof). cargo test --workspace + clippy --workspace --all-targets -D warnings green."; }
        part storyMarkerRetentionDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification markerRetentionRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b050-516273849503"; :>> title = "Sprint 50 refine gate (DoR)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: M2.0, the prerequisite for the user-approved M2 split (subsume structural views as TOML, port algorithmic to Rust). sprint47 emitted marked parts but dropped the marker; M2.0 retains it. Bounded AST+parser+view change, test-guarded. estimatedPoints=3. DoD accepted."; }
    part markerRetentionRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification markerRetentionStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b050-516273849505"; :>> title = "Sprint 50 standup gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: AST Part.marker; parse_hash_item attaches marker to the delegated Part; both Part constructors set marker:None; view ItemInfo.marker + Select.marker; tests; process-changes.view.toml demonstration. No blockers."; }
    part markerRetentionStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification markerRetentionImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b050-516273849507"; :>> title = "Sprint 50 implement gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "AST + parser + view marker retention implemented. cargo test --workspace --lib green (parser marker_prefixed_part_is_preserved now asserts the marker; view select_by_marker passes). clippy --workspace --all-targets -D warnings green. process-changes.view.toml returns the same 5 decisions as query.py process-changes. markerRetention backlog DoD recorded pass; M2.1 portStructuralViews + M2.2 portAlgorithmicViews added to the backlog."; }
    part markerRetentionImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification markerRetentionReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b050-516273849509"; :>> title = "Sprint 50 review gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed: marker retention is the clean completion of the sprint47 fix (emit -> emit+retain). The process-changes.view.toml parity with query.py is the M2.1 proof that structural views subsume cleanly into TOML declarations (the approved split). Born under M0/M0b denies — no unwrap/panic/index. Next: M2.1 (subsume orphans/issues + retire the subsumed query.py commands), then M2.2 (port algorithmic views)."; }
    part markerRetentionReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification markerRetentionCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b050-51627384950b"; :>> title = "Sprint 50 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): story + markerRetention DoD recorded pass; cargo test + clippy + validate_all green. No human gate (D0051: tested AST/parser/view change + parity demonstration, self-evidencing)."; }
    part markerRetentionCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification markerRetentionRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b050-51627384950d"; :>> title = "Sprint 50 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: M2.0 was the right first M2 increment — small, test-guarded, and it unblocks the whole subsume-as-TOML strategy; the process-changes.view.toml==query.py parity is concrete proof the approved split works. The sprint47->M2.0 progression (emit -> retain) shows incremental hardening paying off. No avoidable issue. Next: M2.1 (subsume orphans/issues, retire subsumed query.py commands with parity checks), M2.2 (port algorithmic views to Rust). Estimate 3 pts held."; }
    part markerRetentionRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b050-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8788cc4"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
