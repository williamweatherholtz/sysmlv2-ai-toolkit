// =====================================================================
// D0085 — the comprehensive traceability diagram is a COMPUTED VIEW (interactive self-contained
// HTML), regenerated on demand and never committed as truth (§2.1/D0015 #View). Declared as the
// 'traceability' viewpoint, emitted by `sysmlv2 diagram`, deployed by the diagram skill.
// #ProspectiveChange: introduces a process-def (keystone co-commit, D0070). Accepted 2026-06-21.
// =====================================================================
package Decision0085 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0085 : Decision {
        :>> id = "63940516-2c7d-4e8f-a085-5162738496c0";
        :>> title = "D0085: comprehensive traceability diagram = a computed interactive-HTML #View (regenerate-don't-commit); the 'traceability' viewpoint + `sysmlv2 diagram` + diagram skill";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human asked to 'make a process for and implement a comprehensive diagram' showing full traceability, reliance/linkage to processes, effects, supersession, and full metadata with linkage — and, after weighing KISS vs usefulness, leaned to an INTERACTIVE HTML that can hide/highlight/focus/fit, generated ONCE and explored a part at a time, rather than a set of discrete CLI subgraph calls. A diagram is, by §2.1 + D0015, a computed VIEW (a derived rendering of authored facts), never an authored/committed artifact — so the right engine-native form is a regenerated view + a declared viewpoint (D0056/D0057) + a deploying skill (D0059), not a stored picture.";
        :>> decision = "The comprehensive traceability diagram is a COMPUTED VIEW emitted by `sysmlv2 diagram [ROOT]`: a single self-contained interactive HTML page (cytoscape) of the WHOLE model — every element as a typed node carrying its authored metadata (status/severity/lens/kind/priority/outcome/method/critiquedBy/createdBy/marker) and every typed edge (satisfy/verify/charteredby/resolves/supersede/allocate/dependency/dependson/succession/ordering/process-change) — with in-page controls: filter by node type + edge kind, search, click-a-node to focus its neighborhood, and fit-to-screen. Test/TestResult are toggled OFF by default for legibility (the edgeless ceremony leaves); all data is present and toggleable. PROCESS (regenerate-don't-author, .engine/processes/diagram.sysml, deployed by the diagram skill): the HTML is REGENERATED on demand (`sysmlv2 diagram . > x.html`) and NEVER committed as truth — generated diagram files are git-ignored; the authored facts + the tool are the source, the HTML is disposable (§2.1/D0015). DECLARE it as the 'traceability' viewpoint in the viewpoint-registry (D0056/D0057). Rendering note: the page loads cytoscape from a CDN (online to render); inlining the lib for offline use is a future option if needed.";
        :>> rationale = "A diagram that is generated, never stored, can never show a stale picture — same compute-don't-store guarantee as coverage/assured (a hand-drawn or committed diagram WOULD drift, the exact §2.1/D0018 failure). Interactive HTML serves the human's actual workflow (explore the comprehensive graph a part at a time: filter/focus/search) far better than discrete CLI subgraphs, and HTML is the universal viewer. Declaring it a viewpoint puts it in the same concern-coverage framework as the other views. ALTERNATIVES: (A) DOT/Graphviz static export — rejected as primary: not interactive, needs graphviz to render, no in-page focus/filter (kept available conceptually — the model JSON could feed DOT later); (B) Mermaid — rejected for the full graph: unreadable at ~2500 nodes; (C) commit a rendered SVG/PNG into the repo — REJECTED hard: stores a computed view that drifts (§2.1); (D CHOSEN) regenerated interactive HTML view + viewpoint + skill. KISS tension acknowledged (the human raised it): the HTML embeds a JS lib (some bloat) but stays a single regenerable file with no new stored truth — the bloat lives in a disposable artifact, not the model. CRITERIA: compute-don't-store x interactive-digestible x universal-viewer x one-regenerable-file x no-committed-picture.";
        :>> consequences = "New computed view `sysmlv2 diagram` (view.rs diagram_html + main.rs cmd_diagram): builds the Model, serializes nodes+edges+metadata to JSON, injects into an interactive cytoscape HTML template; edges with a missing endpoint are dropped (no dangling). New process-def .engine/processes/diagram.sysml (regenerate-don't-commit) + deploying skill .engine/skills/diagram (D0059) + a skills-registry entry; #ProspectiveChange marks the keystone co-commit (D0070). New 'traceability' viewpoint in the viewpoint-registry (D0056/D0057). .gitignore ignores generated /*.html so diagrams are never committed. doc-sync: usage.md row. Live: 2485 nodes / 270 edges, valid HTML, zero dangling, Test/TestResult default-off. Pairs with D0001/D0018 (compute-don't-store), D0015 (materialized #View), D0056/D0057 (viewpoints), D0059 (deploying skill), D0048 (Rust authority).";
    }

    verification d0085Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a085-5162738496c1";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0085 (the comprehensive traceability diagram is a computed interactive-HTML #View, regenerate-don't-commit, declared as the traceability viewpoint + emitted by sysmlv2 diagram + deployed by the diagram skill) by directing on 2026-06-21: 'make a process for and implement an comprehensive diagram ... full traceability, reliance and linkage to processes, effects, supersession, full metadata with linkage' and 'I'm leaning html ... a comprehensive digestable view created once that can serve a full (part at a time analyzed) analysis'.";
    }
    part d0085AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a085-5162738496c2";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "e39e035";
        :>> judgedAt = "2026-06-21";
        :>> judgedBy = "wweatherholtz";
    }
}
