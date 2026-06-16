package Decision0036 {
    private import EngineWork::*;

    part d0036 : Decision {
        :>> id = "da6b1bae-5c6d-4e7f-fab8-9c0d1e2f3a4b";
        :>> title = "safetyCaseViews: Release + computed baseline view + Decision-as-signoff; no new schema types";
        :>> status = DecisionStatus::accepted;
        :>> context = "The `safetyCaseViews` task asked: what is the design for curated views (safety cases, baselines, signoffs) and frozen snapshots? Options included: (1) a new Baseline/Signoff schema type; (2) reuse existing types (Release + Decision); (3) a metadata-annotated view definition.";
        :>> decisionText = "Adopted: option (2) — reuse existing types, no new schema. (a) **Baseline view**: `Release :> Element` (already in EngineWork) holds a `membershipRule` string (e.g. 'items matching sprint=S11 at commit 146bc25'). The computed baseline view is a query over `.tracking/**/*.sysml` at that commit — deferred to the `baselineView` implementation task. (b) **Signoff**: an existing `Decision` instance with `status=accepted` that references the Release in its `decisionText` serves as signoff. No new `Signoff` type needed — a Decision IS a recorded attestation. (c) **Safety case view**: a safety case is a materialized view (D0015) filtered to items with `NeedSource::safety` need roots, rendered by a future query subcommand. No curated-view schema type is needed; the filter + Release anchor is sufficient. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "EngineWork schema unchanged. The `baselineView` implementation task (which computes a baseline from a Release) remains on the ready frontier. Future `query.py baseline <release-id>` subcommand will read the Release membershipRule + git SHA and render the view. Safety case views are filtered query outputs, not authored items.";
    }
}
