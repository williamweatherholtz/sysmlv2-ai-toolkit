// =====================================================================
// D0125 (PROPOSED — NOT YET ACCEPTED) — the explorer graph uses a MATURE LAYERED layout (ELK, in
// cytoscape), NOT Graphviz. The model is a DAG; force-directed (cose) scattered it (overlap/crossings/
// fanout). ELK layered (Sugiyama) fixes that while keeping cytoscape's interactivity + the encoding's
// manual positioning. Graphviz (viz.js) is browser-renderable but reserved for a future static export.
// Human question 2026-07-15 ("build on graphviz? avoid text/overlap/fanout via a mature product?").
// Acceptance = sitting review.
// =====================================================================
package Decision0125 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0125 : Decision {
        :>> id = "10a1b0c1-4125-42a3-ab5c-516273849725";
        :>> title = "D0125: explorer uses ELK layered layout (in cytoscape); Graphviz reserved for a future static-export view";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::accepted;
        :>> context = "The explorer's force-directed (cose) layout produced label/edge overlap, crossings, and poor fanout. The human asked (2026-07-15) whether to build on Graphviz — is it browser-renderable, and are these solved problems in a more mature product? FINDINGS: Graphviz IS browser-renderable (viz.js / @hpcc-js/wasm -> SVG). But the ROOT CAUSE was the layout ALGORITHM, not cytoscape: the model is largely a DAG (dependency/derivation/satisfy), which wants a LAYERED (Sugiyama) layout, not force-directed. Mature layered engines: Graphviz `dot`, and ELK (Eclipse Layout Kernel) — ELK is the engine the SysML v2 web ecosystem uses (SysON/Sirius).";
        :>> decision = "Adopt a LAYERED layout via ELK inside cytoscape (cytoscape-elk + elkjs, vendored offline): runLayout uses ELK 'layered' (direction DOWN), falling back to cose if the extension is absent. Keep cytoscape as the renderer — it preserves the interactive model (seed-and-expand, click-detail, live re-style) AND the encoding engine's MANUAL positioning (D0120/D0124), which a Graphviz-owned layout would fight. Declutter labels independently (min-zoomed-font-size + chip backgrounds; hover/select reveals). Graphviz (viz.js) is NOT adopted for the interactive explorer — reserve it for a possible FUTURE static 'export/publish' view where publication-quality `dot` output matters and interactivity does not.";
        :>> rationale = "ALTERNATIVES: (i) Graphviz (viz.js) as the explorer renderer — REJECTED for the interactive view: it emits a static SVG (lose expand/drag/live encoding), needs a full relayout per change + SVG event wiring, and the encoding view's manual x/swimlane positioning doesn't map to `dot` (would need neato -n pinned + clusters). Best-in-class static layout, but wrong for an interactive + encoding surface. (ii) stay on cose + tune it — REJECTED: force-directed is the wrong family for a DAG; tuning won't match layered quality. (iii) dagre (cytoscape-dagre) — viable lighter layered option; ELK preferred because it is the SysML-v2 ecosystem's engine (D0119 leverage-the-standard) and richer. CHOSEN: ELK-in-cytoscape — mature layered layout (minimal crossings, clean fanout, edge routing) with cytoscape's interactivity + our encoding preserved, vendored offline (buildless). Root-causes the complaint (algorithm, not toolkit).";
        :>> consequences = "Recorded PROPOSED; acceptance = sitting review. Advances srViewerGraphExpand ('stable layout, scalable'). Viewer app (sysmlv2-viewer@e104bb4) vendors elk.bundled.js (~1.6MB) + cytoscape-elk. A future static-export view MAY use viz.js/Graphviz (tracked if/when needed). Relates D0119 (ELK is the SysML-v2 ecosystem layout), D0120/D0124 (encoding manual positioning preserved), D0118 (buildless/offline)."; }

    #DependsOn dependency from d0125 to d0119;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0125Accept : Test { :>> id = "f239fc82-2db9-42ad-ae5b-bf06100a342d"; :>> method = VerificationMethod::confirmation; :>> procedureText = "accepted"; }
    part d0125AcceptR1 : TestResult { :>> id = "5e9c23b0-3574-43ef-bfd8-80bd372c51e6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "wweatherholtz"; }
}
