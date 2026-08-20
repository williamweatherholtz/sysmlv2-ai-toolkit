// =====================================================================
// D0061 — decision-fields-rationale: finalize the Decision field set (decision +
// context + rationale + consequences; decisionText retired; context and rationale
// KEPT as distinct fields) and decompose the attributionModel migration (D0060)
// into six ordered sub-tasks.
// NOTE: this file is authored in the CURRENT schema (context/decisionText/consequences)
// — adding `decision` + `rationale` (and retiring decisionText) lands in attrModelSchema, not here.
// Accepted 2026-06-17 by wweatherholtz ("context really should be rationale ...
// reframe the work then kick off").
// =====================================================================
package Decision0061 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0061 : Decision {
        :>> id = "61728394-0a5b-4c6d-e719-405162738440";
        :>> title = "D0061: Decision fields = decision/context/rationale/consequences (context+rationale BOTH kept; decisionText retired); attributionModel decomposed";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Refines D0060 on the Decision field set + defines the migration as ordered work so a fresh context can execute it. Settled in the dig: decisionText (prose catch-all) is deprecated for structured fields; acceptance becomes a confirmation TestResult; the decision record stays. The naming question (context vs rationale) resolved on the recognition that they are NOT synonyms: context = the forces/systems/situation present (the setup, Nygard ADR 'Context'); rationale = the justification for the choice (ISO 42010 'Rationale').";
        :>> decision = "DECISION: final Decision field set = `decision` (the choice statement — ADR 'Decision') + `context` (the forces/systems/situation present — the basis the rationale builds on) + `rationale` (the justification — why this choice, with the D0058 alternatives/criteria as its structured core) + `consequences` (impact); `decisionText` retired (its content splits into `decision` + `rationale`); authoredBy->createdBy (D0060); acceptance -> a downstream confirmation TestResult.";
        :>> rationale = "ALTERNATIVES on the naming: (A) keep only `context` — rejected: it conflates the situation with the justification and undersells the 'why'; (C) rename context->rationale (drop a separate context) — rejected: context and rationale are genuinely DIFFERENT (situation vs justification), so collapsing loses the forces/systems setup; (B) CHOSEN: keep BOTH as separate fields — `context` sets up the basis (what forces/systems are present), `rationale` gives the justification on that basis. CRITERIA: correctness (context != rationale) x standard-practice (Nygard Context + ISO 42010 Rationale) x clarity. (User: 'keep context and rationale — context for setting up the basis of rationale: what forces, systems, etc. are present.')";
        :>> consequences = "attributionModel (D0060) DECOMPOSED into six ordered sub-tasks (backlog NextWork), each with its own DoD: (1) attrModelSchema — authoredBy->createdBy on Element + the three Tracked* bases; retire decisionText (split into decision+rationale), ADD `decision` + `rationale`, KEEP `context` (distinct from rationale); keep id+title+consequences (frozen-schema, pre-authorized). (2) attrModelInstances — bulk-migrate every .engine + .tracking instance: authoredBy->createdBy; decision files KEEP context + split decisionText->decision+rationale. (3) attrModelAcceptanceEvents — Decision acceptance as a confirmation TestResult (verify edge); migrate existing 'Accepted: X' prose. (4) attrModelTooling — validate_actors (createdBy), query.py (read_issues/read_viewpoints), Rust parser/indexer + validate where the renamed fields appear, report.py; re-verify the deliverable (source drift). (5) attrModelCoverageView — process-required-attestation coverage view (items missing a required attestation), pairing with formalViewpoints. (6) attrModelGuard — guard that Decision acceptance is a TestResult, never prose (D0047). Order: schema -> {instances, tooling} -> acceptance-events -> {coverage, guard}. Supersedes the old `trackedMetadata` collapse (removed from backlog); resolves `trackedMetadataReplan` (do-vs-drop decided = do the attribution-events migration). Pre-authorized breaking change; kicked off in a fresh context. This Decision is authored in the current schema; it becomes the first thing the migration restructures.";
    }

    verification d0061Accept : Test { :>> id = "63f18f60-c8e2-4448-8d30-af5df5637953"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0061 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0061AcceptR1 : TestResult { :>> id = "87a64460-0ac0-4793-a34e-b23dc7f81f72"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
