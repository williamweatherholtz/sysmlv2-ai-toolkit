// =====================================================================
// ProjectDeliveryS88 — Sprint 88: flowMetricsGovernance (D0087). Enrich the flow report with the
// canonical agile/DORA flow metrics (cycle time, time/story-point, lead time, predictability, aging
// WIP; flow --trend = delivered-points burnup) and add the 5th GOVERNANCE report. Grounded in flow
// metrics research (Vacanti four-flow + DORA lead time). Extends the report layer (D0087).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049); chartered by D0087 (report-layer enhancement).
// =====================================================================
package ProjectDeliveryS88 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from flowMetricsGovernanceStory to d0087;

    part flowMetricsGovernanceStory : Story {
        :>> id = "63940516-2c7d-4e8f-b088-51627384b901";
        :>> title = "Sprint 88: flowMetricsGovernance (3 pts) — cycle time / time-per-point / lead time / predictability / aging WIP + burnup trend on the flow report; new governance report (D0087)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS88 {
        action storyFlowMetricsGovernance;
        verification storyFlowMetricsGovernanceDoD : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b902"; :>> method = VerificationMethod::test; :>> procedureText = "Enrich the flow report (view.rs) with the canonical flow/agile/DORA metrics computed from gate judgedAt timestamps + Story createdAt + estimatedPoints: cycle time (refine->retro), time per story point (cycle-days/points, human-requested), lead time (created->retro, DORA), predictability (per-sprint point spread), aging WIP (as-of latest date - unfinished refine), keeping ready/WIP/velocity/throughput/open-issues; flow --trend headline = delivered-points BURNUP. Add a 5th GOVERNANCE report [health] (decisions accepted/superseded, acceptance integrity %, process-change decisions, supersession). Date math via Hinnant days_from_civil (no deps). Grounded in researched flow metrics (Vacanti four-flow: WIP/cycle/throughput/age + DORA lead time). VERIFY (live): report flow -> 10 cards incl cycle 0d / time-per-point 0.07d / lead 0d / predictability 1-8 pts / aging 0d (same-day autonomous sprints = ~0, honest); report governance -> 87 decisions / 100% acceptance / 10 process-change / 0 supersession; clippy --workspace --all-targets -D warnings + cargo test (38 lib, headline_labels updated) + rust validate (100) + guard all (9) green; doc-sync skill/usage/CLAUDE. No new decision/process (under D0087)."; }
        part storyFlowMetricsGovernanceDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b903"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification flowMetricsGovernanceRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b904"; :>> title = "Sprint 88 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the human asked to add all indicated metric/report improvements + burndown/velocity/time-per-story-point to the flow report + research other great metrics. Research (agile flow metrics + DORA: cycle/lead time, throughput, WIP, work-item age, predictability, burnup) -> add cycle time, time/point, lead time, predictability, aging WIP + burnup trend, computed from gate timestamps + points; plus the 5th governance report indicated earlier. Extends D0087 (no new decision). estimatedPoints=3. #CharteredBy d0087. DoD accepted."; }
    part flowMetricsGovernanceRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b905"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification flowMetricsGovernanceStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b906"; :>> title = "Sprint 88 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: view.rs — days_from_civil + parse_ymd + quoted_after + sprint_flow (per delivery file: points, createdAt, refineGate/retroGate judgedAt); rewrite flow_cards to compute cycle/time-per-point/lead/predictability/aging from those; governance_cards from Decision items + attestation; report_cards + headline_at + headline_label gain governance + flow-burnup; cmd_report usage + UnknownReport message + doc-sync. Watch: clippy digit separators in date constants; str-slicing (use split/chars). No blockers."; }
    part flowMetricsGovernanceStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b907"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification flowMetricsGovernanceImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b908"; :>> title = "Sprint 88 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the date helpers + sprint_flow + enriched flow_cards (10 metrics) + governance_cards + dispatch/headline/burnup + doc-sync. VERIFY (live): report flow -> velocity 3 / cycle 0d / time-per-point 0.07d / lead 0d / predictability 1-8 pts / throughput 89 / aging 0d; report governance -> 87 decisions / 100% acceptance / 10 process-change / 0 supersession; --trend flow = delivered-points burnup. clippy --workspace --all-targets -D warnings green (digit separators fixed); cargo test 38 lib pass (headline_labels_are_stable updated for burnup+governance); rust validate 100 clean; guard all (9) PASS."; }
    part flowMetricsGovernanceImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b909"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification flowMetricsGovernanceReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b90a"; :>> title = "Sprint 88 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The flow metrics are the recognized canon (Vacanti four-flow + DORA lead time + scrum velocity/burnup) computed purely from authored facts (gate timestamps + points) with exact dep-free date math; same-day autonomous sprints honestly yield ~0 cycle/lead (the metric is correct, the data is just fast) — would be meaningful on a multi-day project. governance is a pure-model aggregate. All reuse the per-element/model data (no new measurement, no stored metric). Honest scope: flow-efficiency (active vs idle) is not computed (we don't track idle time) and is omitted rather than faked. doc-sync: skill/usage/CLAUDE. Extends D0087; no new decision/process."; }
    part flowMetricsGovernanceReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b90b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification flowMetricsGovernanceCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b90c"; :>> title = "Sprint 88 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyFlowMetricsGovernance DoD recorded pass; flow report enriched with cycle/time-per-point/lead/predictability/aging + burnup trend; governance report live; clippy + cargo test (38) + rust validate (100) + guard all (9) green; doc-sync done. Self-evidencing (D0051). The report layer now covers the canonical flow/agile/DORA metrics + governance."; }
    part flowMetricsGovernanceCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b90d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification flowMetricsGovernanceRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b088-51627384b90e"; :>> title = "Sprint 88 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: research-first again paid off — the flow card set maps cleanly to the recognized canon (Vacanti four-flow + DORA), and ALL of it was computable from facts we already author (gate judgedAt + estimatedPoints) with a tiny dep-free date helper, so no new measurement or storage; the time-per-point metric the human asked for fell straight out of cycle-days/points. CAUGHT (avoidable-issue candidate): same-day autonomous sprints make cycle/lead/time-per-point read ~0, which could look broken — an avoidable misread I pre-empted by labelling each card honestly ('same-day autonomous = ~0') rather than hiding or inflating the number; flow-efficiency was OMITTED (not faked) because idle time isn't tracked. Lesson: a metric whose value is degenerate on THIS dataset must say WHY in its detail line, so a 0 reads as 'fast/not-tracked', not 'broken'. Estimate 3 pts held. The report layer is now metric-complete for the project's current scope; next: the load-bearing critique campaign."; }
    part flowMetricsGovernanceRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b088-51627384b90f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64fa59e"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
