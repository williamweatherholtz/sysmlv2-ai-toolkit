// =====================================================================
// D0119 (PROPOSED — NOT YET ACCEPTED) — how the viewer leverages the STANDARD SysML v2 representations.
// Research (2026-07-15, cited spike sprint224): OMG finalized SysML v2 + KerML 1.0 + Systems Modeling
// API & Services 1.0 on 21 Jul 2025 (formal/25-09-03). Direction: adopt the standard at the DATA/API +
// view-kind-vocabulary layer; treat the graphical notation as an approximation target (no reusable web
// lib exists). Human asked (2026-07-15) to leverage/adopt the standard. Acceptance = sitting review.
// =====================================================================
package Decision0119 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0119 : Decision {
        :>> id = "10a1b0c1-4119-42a3-ab5c-516273849719";
        :>> title = "D0119: leverage standard SysML v2 at the API/JSON + view-kind layer; graphical notation is an approximation target";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::proposed;
        :>> context = "The user (2026-07-15) asked to research the STANDARD textual/graphical representations that ship with SysML v2 and make sure we leverage them (ideally wholesale). FINDINGS (cited research spike, sprint224): OMG approved final adoption of SysML v2.0, KerML 1.0, and the Systems Modeling API & Services 1.0 on 2025-07-21 (formal/25-09-03). (1) The .sysml TEXTUAL notation is normative (EBNF, on the KerML foundation) — we are ALREADY on the canonical surface. (2) A GRAPHICAL notation is standardized via Graphical BNF (GBNF) but is explicitly a VIEW, not the source of truth; there is NO reusable web/JS library for it (SysON = heavy Sirius/EPL web platform; DeciSym = Rust/egui MIT/Apache; Pilot = LGPL/GPL PlantUML). (3) The standard machine-readable interchange is JSON via the Systems Modeling API & Services 1.0 (OpenAPI 3.1); XMI was dropped. Resources: Project/Branch/Tag/Commit/Element/Relationship/Query; element envelope @type/@id/declaredName/qualifiedName/ownedRelationship[]. (4) View kinds via View&Viewpoint: General(=v1 BDD)/Interconnection(=v1 IBD)/Action/State/Sequence/UseCase/Requirement/Tree/Table.";
        :>> decision = "LEVERAGE THE STANDARD AT THE DATA/API + VIEW-KIND LAYER, wholesale; treat the graphical notation as an approximation target, not a library to adopt. Concretely, in priority order: (1) HIGH — align keel's model + a read endpoint to the standard Systems Modeling API JSON envelope (@type/@id/ownedRelationship), so keel data is readable by any SysML v2 tool (SysON, Pilot MDK, generated clients) with zero custom notation work; our UUID-typed-item + typed-edge model already maps closely. (2) MEDIUM — name the viewer's viewpoint presets after the standard view kinds (General/Interconnection/Action/State/Sequence/Requirement/Tree/Table) for practitioner legibility. (3) OPTIONAL/presentation-only — approximate the GBNF visual encodings (definition/usage box styles, named compartments, port/connector glyphs) as cytoscape.js styles ONLY if 'looks like standard SysML v2' becomes a stakeholder requirement; reference the GBNF spec + the DeciSym renderer.";
        :>> rationale = "ALTERNATIVES: (i) adopt a standard graphical-notation web library wholesale — NOT POSSIBLE: none exists; SysON is a full platform, not an embeddable widget, and the Pilot uses PlantUML/GraphViz (server-side, GPL). (ii) invent our own interchange — REJECTED: the standard JSON API is finalized, XMI-free, and tool-interoperable; inventing our own forfeits interoperability for no gain. (iii) ignore the standard and stay generic — REJECTED: the user explicitly wants to leverage the standard, and the DATA-layer adoption is low-cost + high-interoperability. CHOSEN: adopt where the standard is strong and reusable (textual — already done; API/JSON + view-kind vocabulary — cheap, durable, interoperable) and only approximate where it is not reusable (graphical notation — a view with no web lib). This maximizes standard-alignment without taking on a heavy platform dependency, and keeps the viewer's generative/buildless architecture (D0117/D0118).";
        :>> consequences = "Recorded PROPOSED; acceptance = sitting review. IF accepted, spawns downstream (create as backlog once accepted): keel exposes a standard-JSON read endpoint (Systems Modeling API envelope) alongside the current /api/*; the viewer names its viewpoint presets after the standard view kinds. The axis-encoding viewpoints (N-18/D0120) compose ON TOP of the standard view kinds (e.g. an Interconnection view with a colour<-attribute + swimlane encoding). VERIFY-LATER item flagged by the research: confirm the exact normative/informative status of the graphical-notation clause against the spec PDF (Part 2 / GBNF)."; }

    #DependsOn dependency from d0119 to d0114;
}
