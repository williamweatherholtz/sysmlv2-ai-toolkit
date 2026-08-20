package Decision0036 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0036 : Decision {
        :>> id = "da6b1bae-5c6d-4e7f-fab8-9c0d1e2f3a4b";
        :>> title = "safetyCaseViews: Release + computed baseline view + Decision-as-signoff; no new schema types";
        :>> status = DecisionStatus::accepted;
        :>> context = "The `safetyCaseViews` task asked: what is the design for curated views (safety cases, baselines, signoffs) and frozen snapshots? Options included: (1) a new Baseline/Signoff schema type; (2) reuse existing types (Release + Decision); (3) a metadata-annotated view definition.";
        :>> decision = "Adopted option (2) — reuse existing types, no new schema: (a) Baseline view = `Release :> Element` holding a `membershipRule` string, with the computed baseline a query over `.tracking/**/*.sysml` at that commit (deferred to the `baselineView` task); (b) Signoff = an existing `Decision` with `status=accepted` referencing the Release in its `decisionText`; (c) Safety case view = a materialized view (D0015) filtered to items with `NeedSource::safety` need roots, rendered by a future query subcommand.";
        :>> rationale = "No new `Baseline`/`Signoff`/curated-view schema type is needed: a Decision IS a recorded attestation, and a Release anchor plus a need-source filter suffices for safety cases. ALTERNATIVES considered: (1) a new Baseline/Signoff schema type; (3) a metadata-annotated view definition. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "EngineWork schema unchanged. The `baselineView` implementation task (which computes a baseline from a Release) remains on the ready frontier. Future `query.py baseline <release-id>` subcommand will read the Release membershipRule + git SHA and render the view. Safety case views are filtered query outputs, not authored items.";
    }

    verification d0036Accept : Test { :>> id = "76669edb-53e5-447b-8916-3c88d70b0668"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0036 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0036AcceptR1 : TestResult { :>> id = "eca11f82-436c-475c-9177-84e49859705c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
