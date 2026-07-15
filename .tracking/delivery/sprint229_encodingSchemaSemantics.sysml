// ProjectDeliveryS229 — Sprint 229: encodingSchemaSemantics — the keel-side FOUNDATION for the axis→visual-
// encoding viewpoints (N-18 accepted; D0120). Extends GET /api/schema with enum MEMBERS + per-attribute
// VALUE STATS (kind/distinct/min-max) from the instances — the semantic metadata an auto-encoder needs to
// pick a scale. Satisfies srEncodingSchemaSemantics. EXECUTE. 2 pts.
package ProjectDeliveryS229 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from encodingSemanticsStory to d0120;

    part encodingSemanticsStory : Story {
        :>> id = "63940516-2c7d-4e8f-c229-51627384c001";
        :>> title = "Sprint 229: encodingSchemaSemantics (2 pts) - /api/schema exposes enum members + per-attribute value stats (N-18 foundation)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p1; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    satisfy srEncodingSchemaSemantics by encodingSemanticsStory;

    action def DeliveryRunS229 {
        action storyEncodingSemantics;
        verification storyEncodingSemanticsDoD : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Satisfies srEncodingSchemaSemantics (N-18/D0120 — the keel-side foundation for encoding viewpoints). IMPLEMENTED (view::schema_json, refactored into scan_schema_defs + attribute_stats + enum_def): GET /api/schema now also emits (a) `enums`: every enum def with its MEMBERS (text-scan of schema/); (b) per-attribute `isEnum`; (c) `attributeStats`: per (type,attribute) observed from the INSTANCES — kind (enum|numeric|temporal|categorical), count, distinct values (capped 64) for enum/categorical, numeric min/max, temporal min/max. This is the semantic metadata the auto-encoder uses to choose a scale (enum -> ordinal over members; numeric -> binnable range; temporal -> time; categorical -> distinct set). Unit tests enum_def_parses_members + schema_scan_parses_defs_and_attributes. VERIFY (test): clippy --workspace -D warnings clean; cargo test green; live curl /api/schema -> 93 types, 26 enums (DecisionStatus = proposed/accepted/rejected/superseded), 61 attributeStats (Story.estimatedPoints numeric min 1 max 8; Need.priority enum distinct could/must/should; Measurement.measuredAt temporal min/max dates). keel validate + guard + check-engine green. Viewer-side SRs (srEncodingSpec/Render/Override) remain — the encoding engine + override editor are the next, larger build."; }
        part storyEncodingSemanticsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    verification esRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c004"; :>> title = "Sprint 229 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: N-18 accepted (wweatherholtz 'go') → build the keel-side foundation first (expose encoding semantics). Scope: extend /api/schema with enum members + per-attribute value stats. #CharteredBy d0120. estimatedPoints=2. DoR met."; }
    part esRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c006"; :>> title = "Sprint 229 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: reuse the schema text-scan for enum members; add instance-derived value stats via Model::build; classify by declared type (enum/Integer/Real/Timestamp/String); cap distinct at 64. Factor into scan_schema_defs + attribute_stats for clippy. No blockers."; }
    part esStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c008"; :>> title = "Sprint 229 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "enum_def + scan_schema_defs + attribute_stats + enriched schema_json implemented + unit-tested. clippy -D warnings clean; cargo test green; live-verified (enums + numeric/enum/temporal stats). validate/guard/check-engine green."; }
    part esImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c00a"; :>> title = "Sprint 229 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The right foundation: the encoder's scale choice is DATA-DRIVEN from declared semantics (enum members, numeric/temporal ranges, categorical distinct) — computed, nothing hardcoded, and it grows as the model does (generative, D0117). Kept the increment keel-side + test-verifiable; the viewer-side encoding engine (srEncodingSpec/Render/Override) is scoped as SRs for the next build. Clean."; }
    part esReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c00c"; :>> title = "Sprint 229 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; /api/schema exposes the encoding semantics (srEncodingSchemaSemantics satisfied). Self-evidencing. NEXT (viewer app, larger): the encoding-spec model + auto-defaulter + spec→cytoscape renderer (swimlanes/nesting) + override editor (srEncodingSpec/Render/Override)."; }
    part esCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c229-51627384c00e"; :>> title = "Sprint 229 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: on N-18 acceptance, decomposed it into SRs (needs-before-architecture) and shipped the smallest load-bearing increment first — the DATA the encoder needs — keel-side + test-verifiable, before the big viewer-side build. The stats are computed from the model (generative), so they evolve with it. AVOIDABLE-ISSUE SCAN: clippy flagged the enriched fn (too-long + complex tuple types) — factored into scan_schema_defs/attribute_stats + a DefAcc/AttrAgg struct; banked: extract helpers + name tuple types up front for view functions. Estimate 2 pts. NEXT: viewer-side encoding engine."; }
    part esRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c229-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ceba09e"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
