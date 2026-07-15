// =====================================================================
// D0120 (PROPOSED — NOT YET ACCEPTED) — design direction for N-18 axis->visual-encoding viewpoints.
// A DECLARATIVE visual-encoding SPEC per viewpoint (channel<-attribute bindings: colour / x-position /
// swimlane / nested-swimlane) that AUTO-GENERATES from schema+data semantics, with a manual override
// layer. Derives from nViewerEncodingViewpoints (N-18); composes on the standard view kinds (D0119);
// realises the generative principle (D0117). Awaits N-18 acceptance (needs-before-architecture).
// =====================================================================
package Decision0120 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0120 : Decision {
        :>> id = "10a1b0c1-4120-42a3-ab5c-516273849720";
        :>> title = "D0120: encoding viewpoints = a declarative channel<-attribute spec, auto-generated from semantics + overridable";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::proposed;
        :>> context = "N-18 (nViewerEncodingViewpoints) asks that a viewpoint map model axes/attributes to visual encoding channels — colour<-attribute (continuous/enum), x-position<-date/continuous, swimlane bands<-binned-continuous or enum, a nested swimlane layer<-a second attribute — defined semantically enough to AUTO-GENERATE, with manual override (enum sort, colour mappings for enum values, colour ranges/bins for continuous). This is the durable expressive power of the explorer and realises N-1 (viewpoint = query + form) concretely. Needs a design direction BEFORE building.";
        :>> decision = "Model an encoding viewpoint as a DECLARATIVE ENCODING SPEC: an ordered set of CHANNEL bindings, each binding a visual channel (colour | x-position | y-position | swimlane | nested-swimlane | size | shape) to a model ATTRIBUTE (or edge/derived measure) plus a SCALE. The scale is chosen from the attribute's declared SEMANTICS (from /api/schema: enum -> ordinal/categorical scale over the enum members; date/Timestamp -> temporal scale; Integer/Real -> continuous/binned scale; String -> categorical) so the spec AUTO-GENERATES a sensible default. An OVERRIDE layer (authored, layered on top of the auto defaults) can: reorder/relabel enum members, pin a colour to a specific enum value, set continuous colour ranges + bin boundaries, and set swimlane nesting order. keel's job: expose the semantic metadata needed to auto-generate (attribute kind, enum members, and value min/max/distribution) — an extension of /api/schema (e.g. enum members + per-attribute value ranges). The viewer's job: the encoding engine (spec -> cytoscape positions/colours/compound swimlane nodes) + the override editor. The spec is itself model-declarable (a Viewpoint's form), consistent with the generative principle (D0117) and composable with the standard view kinds (D0119).";
        :>> rationale = "ALTERNATIVES: (i) hardcode a fixed set of layouts (e.g. one timeline view, one swimlane view) — REJECTED: not reusable, doesn't evolve with the model, contradicts D0117. (ii) adopt a full charting/grammar-of-graphics library (Vega-Lite) wholesale — CANDIDATE but heavier; Vega-Lite's channel/encoding model is the proven precedent and MAY be the implementation substrate, but the DECISION here is the declarative-channel-binding SHAPE, not the library (revisit lib choice at build). (iii) purely manual per-view configuration with no auto-generation — REJECTED: the user explicitly wants semantic AUTO-generation with override, not hand-building every view. CHOSEN: a declarative channel<-attribute spec auto-derived from declared semantics + an override layer — it is model-driven (fits generative UI), reusable/shareable (a viewpoint's form), and degrades to sensible defaults while allowing precise control. The grammar-of-graphics channel model (colour/position/facet=swimlane) is well-established, lowering risk.";
        :>> consequences = "Recorded PROPOSED and BLOCKED on N-18 acceptance (needs-before-architecture; Business gate 5 is pending). IF N-18 accepted + this direction accepted, downstream (create then): keel — extend /api/schema (or a new /api/semantics) with enum members + per-attribute value ranges; the viewer — an encoding-spec model, the auto-generate-from-semantics defaulter, the spec->cytoscape rendering (compound nodes for swimlanes, nested compounds for the superseding layer, position mapping, colour scales), and the override editor. Big feature; will decompose into SystemRequirements + multiple sprints. Composes with D0119 (standard view kinds) and D0117 (generative)."; }

    #DerivedFrom dependency from d0120 to nViewerEncodingViewpoints;
    #DependsOn dependency from d0120 to d0119;
}
