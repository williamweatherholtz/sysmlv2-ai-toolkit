// =====================================================================
// ProjectRequirementsKeelViewer — Architecture-phase SystemRequirements for the keel-viewer, derived
// from the ACCEPTED Needs (keel-viewer.sysml; keelViewerNeedsAcceptR1) under architecture SHAPE B
// (D0114: separate viewer over a versioned keel API). Measurable thresholds; satisfy-linked to the
// Needs. verify-Test edges land with implementation (SRs read as unverified until then — honest).
// =====================================================================
package ProjectRequirementsKeelViewer {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    requirement srViewerKeelApi : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ac01";
        :>> title = "Committed, versioned keel read/query/graph/temporal API (shape B keystone)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel shall expose a STABLE, VERSIONED read/query/graph/temporal API (an explicit version string; documented request/response contracts) covering: run a declared viewpoint (select/traverse/project), fetch an element + its typed neighbours, expand a seed to configurable depth/edges, and read temporal state — sufficient for a separate viewer app to consume WITHOUT duplicating SysML-v2 management. Breaking changes bump the version.";
        :>> rationale = "D0114 shape B makes the versioned API the enabling first increment; much of keel serve's /api/* already exists, but is unversioned + uncommitted. A committed contract is what lets the viewer iterate/deploy independently.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }
    // (srViewerTemporalState removed — D0116: no due-date/overdue state; temporal = time-as-query-filter, served by N-5 srViewerDatePredicates.)
    requirement srViewerGraphExpand : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ac03";
        :>> title = "Seed-and-expand, incrementally-loaded, stable-layout graph";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The graph surface shall support seed-and-expand navigation (load neighbours on demand), drill-into-detail, and a STABLE layout that does not re-shuffle on expand — loading INCREMENTALLY rather than embedding the whole model, so it remains interactive past the ~2,500-node point where whole-graph layout froze.";
        :>> rationale = "N-2: the current graph is filter+search+focus over the whole embedded model; it does not scale to a system a stakeholder wants to WALK.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerVisualViewpoint : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ac04";
        :>> title = "Viewpoint declares its visual form + audience (beyond the 3 fixed modes)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "A viewpoint shall be declarable with a chosen VISUAL form (layout/grouping/styling) targeting its named audience, rendered accordingly — not limited to the three built-in modes (graph/table/review) with hardcoded palette. Unknown visual directives fail loud (keel's fail-loud stance).";
        :>> rationale = "N-1: viewpoints already declare an `audience` but cannot use it; presentation vocabulary is hardcoded. The authoring surface (extend .view.toml / new artifact / presentation layer) is open question Q4.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }
    requirement srViewerConfigurableSlice : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ac05";
        :>> title = "Configurable-depth, edge-selectable slice around a seed";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The system shall produce, from any seed element, a slice over CONFIGURABLE traversal depth and a SELECTABLE set of typed edges, returning only the connected subgraph and hiding the rest — generalizing today's fixed 1-hop element_neighbourhood and Need-only need_slice.";
        :>> rationale = "N-4: cross-cutting stakeholder viewpoints (safety-data-around-a-process, requirement trace) need configurable slices, not fixed 1-hop.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }
    requirement srViewerDatePredicates : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ac06";
        :>> title = "Bounded query expansion: date/range predicates (+ likely OR)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The view query vocabulary shall add date/range predicates (and likely OR) sufficient for temporal + stakeholder filtering, while remaining a BOUNDED, fixed vocabulary (not an unbounded query language) — the exact bound set by a follow-on Decision (open question Q3).";
        :>> rationale = "N-5: temporal state (N-3) and stakeholder filters need date/range comparisons the current equality/set-membership-only vocabulary cannot express; keel's 'not a sledgehammer' stance must be respected.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::should;
        :>> verificationMethod = VerificationMethod::test;
    }

    // ── satisfy edges: SR satisfies the upstream accepted Need (needs-before-architecture). ──
    satisfy nViewerSeparation by srViewerKeelApi;
    satisfy nViewerExplorableGraph by srViewerGraphExpand;
    satisfy nViewerViewpointForm by srViewerVisualViewpoint;
    satisfy nViewerCrossCuttingSlices by srViewerConfigurableSlice;
    satisfy nViewerQueryExpansion by srViewerDatePredicates;

    // ── Expansion SRs (from the accepted Needs N-10, N-12, N-13; the rest superseded by D0115). Shape B. ──
    requirement srViewerChangeImpact : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ad02";
        :>> title = "Change-impact: downstream dependents of a seed";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "From any seed element, the system shall compute the downstream dependents over configurable typed edges (the inverse-direction slice), grouped by distance, with cycles shown once (not infinitely expanded) — so a change's blast radius is visible before the change.";
        :>> rationale = "N-10: the inverse of the N-4 slice; keel's traversal engine already walks edges up/down.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srViewerExportShare : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ad04";
        :>> title = "Export / share a point-in-time viewpoint snapshot";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The system shall export a viewpoint as a point-in-time snapshot (link / file), STAMPED with its as-of date + scope + the source commit, so the snapshot is reproducible and a non-user gets the view; oversized viewpoints export the scoped/filtered subset with a note.";
        :>> rationale = "N-12: reuses keel's git commit as the reproducible as-of anchor.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerBaselineCompare : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ad05";
        :>> title = "Compare a viewpoint across two times / baselines";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The system shall diff a viewpoint's contents between two points (dates or baselines) and highlight added / removed / changed / re-verified elements, using keel's git history as the substrate; an element predating one point reads as 'added since'.";
        :>> rationale = "N-13: keel already computes git-derived trends/drift; this presents a viewpoint-scoped diff.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::could; :>> verificationMethod = VerificationMethod::test;
    }
    // ── Critique SRs (from accepted Needs N-15, N-16; keelViewerNeedsAccept3R1). Reuse keel substrate. ──
    requirement srViewerIterativeCritique : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ae01";
        :>> title = "Iterative AI critique over a query, axis-iterated";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The system shall run AI critique ITERATIVELY over a constrained query result, iterating over ONE chosen axis (each element along that dimension), applying a chosen lens, and recording ONE tracked critique per element (via apply-review, with the independent-critic + provenance discipline). Supports process+downstream (best-practice lens) and requirement+downstream (semantic-sufficiency lens). Reuses keel's element-critique + the serve agent bridge.";
        :>> rationale = "N-15: keel already critiques a single element with lenses + records findings; this drives that over a query set on an axis — the human's core goal.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srViewerInProgramEdit : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000ae02";
        :>> title = "In-program modify (via keel processes) + re-critique";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz";
        :>> statement = "The system shall let a user modify facts + (re)critique them on the same surface, where a modification is made THROUGH keel's disciplined write PROCESSES — record a Decision, supersede a Need (a change to a fact IS a superseding Decision, §2.4), apply-review, append-result, add-task — via the guarded write API, NEVER a raw field mutation or a second store. A guard-rejected write is refused inline with the guard message; a concurrent change flags a conflict (D0108). So critique -> disciplined-change -> re-critique iterates in one place without going rogue.";
        :>> rationale = "N-16 (human clarification 2026-07-15): 'we modify facts through keel processes like decisions, not going rogue'. keel's apply-review/append-result/add-task/record-decision already write through the guarded API; this surfaces those PROCESS operations in-program — the change is itself tracked + guarded, never a freeform edit.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ── Generative-UI SRs (from accepted N-17; D0117). keel EXPOSES the declared model; the viewer GENERATES. ──
    requirement srViewerSchemaApi : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000af01";
        :>> title = "keel exposes the declared model machine-readably (types + attributes)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel shall expose its DECLARED model machine-readably: each item TYPE (part/requirement/use-case/... def) with its ATTRIBUTE fields (name + type + enum members where applicable), via /api/schema — alongside the existing /api/launchables (processes/skills). New types/attributes in schema/core appear automatically (no hardcoded list). This is the substrate a generative UI reads to build display + edit forms.";
        :>> rationale = "N-17/D0117: the viewer can only GENERATE forms if keel tells it the types + fields. Parser skips type-def bodies today, so a text-scan of schema/core is the first mechanism (like build_dod_files).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srViewerGenerativeUi : SystemRequirement {
        :>> id = "6e51d1c0-0114-4c8b-9d2e-00000000af02";
        :>> title = "The viewer GENERATES its display + manipulation UI from the declared model";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz";
        :>> statement = "The viewer app shall GENERATE its display + manipulation surface from keel's declared model — types+attributes (/api/schema) drive forms/fields; declared processes/skills (/api/launchables) drive actions; the guarded write API + keel processes (N-16) drive changes — with NOTHING hardcoded per type/process/field, so model evolution is reflected without viewer code. (Mostly the viewer app, shape B; keel's part is srViewerSchemaApi + /api/launchables + the write API.)";
        :>> rationale = "N-17: the engine premise — the UI is a projection of the declared model, so the model's evolution IS the UI's. Generalises D0109 (launchable set) to the whole UI.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    satisfy nViewerChangeImpact by srViewerChangeImpact;
    satisfy nViewerExportShare by srViewerExportShare;
    satisfy nViewerBaselineCompare by srViewerBaselineCompare;
    satisfy nViewerIterativeCritique by srViewerIterativeCritique;
    satisfy nViewerInProgramEdit by srViewerInProgramEdit;
    satisfy nViewerGenerativeUi by srViewerSchemaApi;
    satisfy nViewerGenerativeUi by srViewerGenerativeUi;
}
