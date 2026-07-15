// =====================================================================
// D0116 — keel-viewer temporal scope: NO due-date/overdue STATE machine (supersede N-3); the temporal
// need is TIME-AS-A-QUERY-FILTER, which is N-5 (date/range query predicates over authored date facts
// like judgedAt). Human answer to open question Q2 (2026-07-15). Supersedes nViewerTemporalState; its SR
// + backlog are removed; N-5 (nViewerQueryExpansion / srViewerDatePredicates) is the home. Accepted.
// =====================================================================
package Decision0116 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0116 : Decision {
        :>> id = "10a1b0c1-4116-42a3-ab5c-516273849716";
        :>> title = "D0116: viewer temporal = time-as-query-filter (N-5), NOT a due-date/overdue state machine (supersede N-3)";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Open question Q2 asked how to model 'review due' for N-3 (calendar temporal state): compute due/overdue in keel, author explicit next-due dates, or render in the viewer. The human answered (2026-07-15) that NONE fit: 'I don't need a review due date, I just want reviews to be tracked temporally, and for the query to be able to use time as a query filter.' Reviews are ALREADY tracked temporally (every TestResult/critique carries judgedAt). So the real need is not a due/overdue STATE machine (N-3) but TIME AS A QUERY FILTER — which is exactly N-5 (nViewerQueryExpansion / srViewerDatePredicates: date/range predicates over date attributes).";
        :>> decision = "Supersede N-3 (nViewerTemporalState) and drop its due/overdue computation entirely. The temporal capability is redefined as TIME-AS-QUERY-FILTER and served by N-5 (already accepted): the view/query vocabulary gains date/range predicates over authored date attributes (judgedAt etc.), so a user filters e.g. 'reviewed since DATE', 'not reviewed since DATE', 'in [d1,d2]'. Remove srViewerTemporalState + its satisfy edge + the viewerTemporalState backlog action (moot). Answers Q2: neither compute-due nor author-due — no due dates at all; just queryable temporal facts.";
        :>> rationale = "ALTERNATIVES (Q2 A/B/C): all three assumed a due/overdue concept the human explicitly does NOT want — so all rejected. A due-date model (computed or authored) would build machinery for a state nobody asked for (and B/authored-due drifts, against §2.1). CHOSEN — no due-state; lean on the date facts that already exist (judgedAt) + add date/range QUERY predicates (N-5). This is minimal (reuses authored facts), honest (§2.1: don't compute/store a state the user doesn't need), and collapses the temporal need into the query-expansion need already accepted. CRITERIA: build-only-what's-wanted x reuse-existing-date-facts x no-drift x fewer-needs.";
        :>> consequences = "ACCEPTED by the human 2026-07-15 (discrete d0116Accept TestResult — the Q2 answer). N-3 superseded (#Supersede edge); srViewerTemporalState + satisfy + viewerTemporalState backlog removed. N-5 (srViewerDatePredicates) is now THE temporal deliverable: date/range query predicates over date attributes. The compliance use-case UC-4 is re-served by querying (e.g. 'processes not reviewed since <date>') rather than a computed 'overdue' flag. Live keel-viewer Needs drop by one (N-3 out). Next build: N-5 date/range query filtering (first increment on the explore surface, e.g. /api/slice date filter)."; }

    #Supersede dependency from d0116 to nViewerTemporalState;

    verification d0116Accept : Test { :>> id = "10a1b0c1-4116-42a3-bb5c-516273849716"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz answered Q2 (2026-07-15): 'I don't need a review due date, I just want reviews to be tracked temporally, and for the query to be able to use time as a query filter' — accepting that N-3 (due-state) is superseded and the temporal need is time-as-query-filter (N-5)."; }
    part d0116AcceptR1 : TestResult { :>> id = "10a1b0c1-4116-42a3-cb5c-516273849716"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "wweatherholtz"; }
}
