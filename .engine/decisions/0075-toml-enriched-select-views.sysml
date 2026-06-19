// =====================================================================
// D0075 — viewpoints are concise TOML files: a FILTER over an enriched item-table
// (authored + computed attrs) + optional traverse(+far-endpoint) + project. Supersedes D0071's
// SysML-schema-fields carrier; keeps D0071's intent (declared, executable, resolves issue018).
// Stress-tested against 12 real use cases before commit. Accepted 2026-06-18/19 by wweatherholtz
// ("maybe toml is better" + "i like it. clean and concise, without becoming a sledgehammer").
// =====================================================================
package Decision0075 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // D0075 itself changes the view MODEL (a representation/tooling decision); it edits no
    // .engine/processes|workflows file, so no process-change marker is required (D0070).
    part d0075 : Decision {
        :>> id = "63940516-2c7d-4e8f-a075-516273849659";
        :>> title = "D0075: viewpoints = TOML filters over an enriched item-table (+traverse +project); supersedes D0071's SysML-schema carrier; resolves issue018";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0071 accepted query-driven viewpoints but carried the spec as FIELDS on the frozen SysML Viewpoint schema, and modelled a view as a thin seed+filter+traverse. The user wanted views as concise, trivially-modifiable files, and full-Rust (D0074) made the YAML ecosystem a liability (serde_yaml deprecated; serde_yml unsound+unmaintained, RUSTSEC-2025-0068; yaml-rust unmaintained, RUSTSEC-2024-0320) — TOML is Rust-native (Cargo's own format), maintained, footgun-free (no significant whitespace, no Norway problem). A STRESS TEST of 12 real use cases (ICD, trace-by-attribute, incomplete items, AI-completed, velocity source, timeframe, next, recently-completed, only/antiquated/incomplete processes, original+superseded decisions) showed the thin model handles only ~3 — most need attribute / computed-status / temporal / edge-absence / boundary predicates.";
        :>> decision = "A viewpoint is a concise TOML file (.engine/views/<name>.view.toml). A view = a FILTER over an ENRICHED ITEM-TABLE — every item carries AUTHORED attrs (type, owner, status, kind, createdAt, points, title) AND COMPUTED attrs (done/ready/suspect from orient, completedAt from git, governingVersion/antiquated from the resolver). Form: `[select]` predicate (type; equality; `in`-set; time-range; `has`/`missing`-edge) -> optional `[traverse]` (edges, direction, depth, + optional far-endpoint `target` predicate, which expresses ICD-style BOUNDARIES) -> `project` (types/fields). LIGHTWEIGHT LINE (so it never becomes a query language / sledgehammer): a FIXED predicate vocabulary only — NO joins beyond traverse, NO aggregation, NO arbitrary expressions. AGGREGATION (e.g. velocity = points-per-sprint) is NOT a view; it is a computed analysis ON a view's output. This UNIFIES D0071's two kinds: 'computed views' (orient/suspect/governing-version) become the enrichment FUNCTIONS that populate the computed columns; everything else is a filter over the enriched table. Presentation/rendering is a SEPARATE layer, later (the view's output is data — items+edges as JSON). No third party (no Arcadia/Capella). Executor is Rust (D0074 M1); processes + skills to Read/Write/eXecute the view files.";
        :>> rationale = "The 12-case stress test (run before commit) proved the thin seed+filter+traverse is insufficient and the enriched-select model is sufficient (11/12 as selection±traverse; ICD via the far-endpoint predicate; velocity correctly excluded as aggregation). TOML over YAML is forced by D0074 (full-Rust, fail-loud): the maintained-Rust-YAML story is poor (active RUSTSEC advisories), TOML is native + safe + deny_unknown_fields makes typos fail at parse. Files over SysML-schema-fields: trivially modifiable, decoupled from the frozen schema, and a view is data not model. ALTERNATIVES: (A) D0071's SysML Viewpoint schema fields — superseded: less modifiable, couples views to a frozen schema; (B) thin seed+filter+traverse — rejected by the stress test (handles ~3/12); (C) a full query language / 3rd-party (Arcadia/SPARQL) — rejected: sledgehammer, against D0054; (D CHOSEN) bounded TOML enriched-select. CRITERIA: covers-the-real-use-cases x lightweight/no-sledgehammer x trivially-modifiable x Rust-native-safe x compute-don't-store.";
        :>> consequences = "SUPERSEDES D0071 (status -> superseded; D0071's intent — declared, executable, resolves issue018, query/computed distinction — is RESTATED here, refined: the distinction collapses to enrichment-functions + filters). The viewpoint-registry (D0056/D0057) stays the concern-coverage AUDIT (each view still declares concern/audience), but the executable spec moves to the .view.toml files; the SysML Viewpoint parts become the concern-coverage index (no query-spec fields added — D0071's planned schema change is cancelled). Execution = D0074's migration increment M1 (Rust TOML view executor + typed-edge extraction + 2-3 example views + RWX skill) + a small extension for the far-endpoint/boundary predicate. issue018 resolved by M1. Velocity, next, recently, antiquated, etc. are expressed per the table in the design dialogue. Pairs with D0074 (Rust/fail-loud), D0056/D0057 (concern coverage), D0050/D0069 (the computed enrichment functions).";
    }

    verification d0075Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a075-516273849660";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0075 (TOML enriched-select view model; supersedes D0071; far-endpoint predicate for ICD; velocity=aggregation-not-a-view; fixed predicate vocabulary) via the 12-use-case stress-test dialogue + 'maybe toml is better' + 'i like it. clean and concise, without becoming a sledgehammer' on 2026-06-18/19.";
    }
    part d0075AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a075-516273849661";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "909764e";
        :>> judgedAt = "2026-06-19";
        :>> judgedBy = "wweatherholtz";
    }
}
