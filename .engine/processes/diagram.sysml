// =====================================================================
// EngineDiagramView — the comprehensive traceability diagram as a regenerated computed VIEW (D0085).
// A diagram is NEVER authored or committed; it is generated on demand from the authored model
// (§2.1/D0015 #View) and explored interactively. Deployed by the diagram skill (D0059).
// =====================================================================
package EngineDiagramView {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part diagramView : Process {
        :>> id = "63940516-2c7d-4e8f-c085-516273849850";
        :>> title = "Traceability Diagram (computed view)";
        :>> purpose = "Render the whole tracked model (every element + typed edge + metadata) as a regenerated interactive HTML view for part-at-a-time traceability analysis; never author or commit a diagram — regenerate it from authored facts.";
        :>> cadence = Cadence::adHoc;
    }

    part dgRegenerate : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c085-516273849851";
        :>> title = "Regenerate — never author";
        :>> order = 1;
        :>> triggerCondition = "Someone wants to see/analyze traceability, reliance, supersession, effects, or metadata linkage across the model.";
        :>> actionText = "Run `keel diagram [ROOT] > <name>.html` to COMPUTE the diagram fresh from the authored facts. Never hand-draw a diagram and never edit the generated HTML as if it were truth — it is a derived #View (§2.1/D0015). The authored .sysml model + the tool are the source.";
        :>> producedArtifact = "A self-contained interactive HTML diagram (cytoscape): typed nodes + metadata + every typed edge.";
        :>> owner = Owner::ai;
    }
    part dgExplore : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c085-516273849852";
        :>> title = "Explore — part at a time";
        :>> order = 2;
        :>> triggerCondition = "A generated diagram is open.";
        :>> actionText = "Use the in-page controls to analyze comprehensively but digestibly: filter by node type + edge kind, search an id, click a node to FOCUS its neighborhood (reliance/linkage), and fit. Test/TestResult are off by default (edgeless ceremony leaves); toggle them on when needed.";
        :>> producedArtifact = "An ephemeral analysis view (no stored artifact).";
        :>> owner = Owner::human;
    }
    part dgDispose : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c085-516273849853";
        :>> title = "Dispose — do not commit";
        :>> order = 3;
        :>> triggerCondition = "Analysis is done, or the model changed.";
        :>> actionText = "Treat the HTML as disposable: generated diagram files are git-ignored (/*.html) and MUST NOT be committed (committing a computed view stores a thing that drifts, §2.1/D0018). When the model changes, regenerate — never patch a stale diagram. If a finding emerges from the analysis, record it as a tracked Issue/critique, not as an annotation on the picture.";
        :>> producedArtifact = "Nothing stored (the diagram is regenerable); any finding becomes a tracked Issue/critique.";
        :>> owner = Owner::ai;
    }
}
