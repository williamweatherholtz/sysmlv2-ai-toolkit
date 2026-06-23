// =====================================================================
// EngineRenderView — the MODULAR interactive-artifact renderer over the view layer (D0086).
// Generalizes the diagram (D0085, now the graph preset): render ANY declared view as graph / table /
// review HTML, and round-trip a human review back as linked critique items via `apply-review`.
// A rendered artifact is NEVER authored or committed (§2.1/D0015 #View). Deployed by the render skill.
// =====================================================================
package EngineRenderView {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part renderView : Process {
        :>> id = "63940516-2c7d-4e8f-c086-516273849860";
        :>> title = "Modular Render + Review Round-trip (computed view)";
        :>> purpose = "Render any declared view (D0056/D0057) as a self-contained interactive artifact — graph, table, or review — regenerated on demand from authored facts; and round-trip a human review (an independent critique, D0080) back into the model as new linked critique items that attest state and induce suspicion. Never author or commit a rendered artifact.";
        :>> cadence = Cadence::adHoc;
    }

    part rvChoose : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c086-516273849861";
        :>> title = "Choose the view + mode";
        :>> order = 1;
        :>> triggerCondition = "Someone wants to explore, tabulate, or review part of the tracked model.";
        :>> actionText = "Pick a DECLARED view (a viewpoint, D0056/D0057 — e.g. decisions, issues, processes; or `model` for the whole graph) and a MODE: `graph` (the cytoscape neighborhood graph), `table` (sortable/searchable rows), or `review` (table + per-row accept/finding + rationale capture). The view selects WHAT; the mode selects HOW. Do not write a new renderer per view — declare the view, then render it.";
        :>> producedArtifact = "A (view, mode) choice.";
        :>> owner = Owner::ai;
    }
    part rvRender : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c086-516273849862";
        :>> title = "Render — never author";
        :>> order = 2;
        :>> triggerCondition = "A (view, mode) is chosen.";
        :>> actionText = "Run `keel render <view> --mode graph|table|review [ROOT] > <name>.html` to COMPUTE the artifact fresh from authored facts. Never hand-build an artifact and never edit the generated HTML as truth — it is a derived #View (§2.1/D0015). Every node type + edge kind is a toggle; the dense derived edges (contains/resultof) default off.";
        :>> producedArtifact = "A self-contained interactive HTML artifact (graph / table / review).";
        :>> owner = Owner::ai;
    }
    part rvReview : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c086-516273849863";
        :>> title = "Review round-trip — attest, induce suspicion";
        :>> order = 3;
        :>> triggerCondition = "A review-mode artifact was used to disposition elements (accept / finding + rationale).";
        :>> actionText = "Export the review batch (the in-page Export-JSON button) and run `keel apply-review --batch <batch.json> --sha <commit> --judged-by <you> --judged-at <date>`. The human is an INDEPENDENT critic (D0080): each disposition enters as a NEW LINKED `method=critique` verification + result + `#Verify` edge in .tracking/critiques.sysml — accept attests state (pass); a finding (fail) carries severity + lens and INDUCES SUSPICION (`keel suspect` critique_suspect) until cleared. The JSON is transport; .tracking is the only truth. Never paste the JSON in as a stored artifact, and never edit an element to record a review — append the critique.";
        :>> producedArtifact = "New linked critique items (verification + TestResult + #Verify) in the model; computed suspicion on findings; nothing else stored.";
        :>> owner = Owner::human;
    }
    part rvDispose : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c086-516273849864";
        :>> title = "Dispose — do not commit the artifact";
        :>> order = 4;
        :>> triggerCondition = "Rendering/review is done, or the model changed.";
        :>> actionText = "Treat the HTML as disposable: generated artifacts are git-ignored (/*.html) and MUST NOT be committed (committing a computed view stores a thing that drifts, §2.1/D0018). When the model changes, regenerate — never patch a stale artifact.";
        :>> producedArtifact = "Nothing stored (the artifact is regenerable; any finding is already a tracked critique).";
        :>> owner = Owner::ai;
    }
}
