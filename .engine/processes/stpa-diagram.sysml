// stpaDiagram — how a control structure is DRAWN, so that the picture can be read (D0285).
//
// WHY A PROCESS AND NOT A STYLE NOTE. The control structure is computed (D0284); the first three
// attempts to draw it were each wrong in a way the human had to name: an auto-laid graph with no
// authority axis and edges labelled "an action"; curved edges whose labels collided with other edges;
// straight diagonals that crossed everything. The human set the conventions one message at a time -
// authority on the vertical axis descending, control down and feedback up, every edge labelled with
// WHAT IS PASSED, ortholinear, nothing overlapping, a semicircle where lines cross - and then said
// "save this as the stpa graphing process". A convention that lives only in one session's chat is
// re-derived wrongly next time (D0040); as a process with a tool that enforces it by construction,
// the fourth attempt is the last.
//
// THE TOOL IS THE ENFORCEMENT. `.engine/tools/stpa_diagram.py` takes the JSON of
// `keel show control-structure` and cannot draw a diagonal, a shared segment, an unlabelled edge or a
// junction where a crossing belongs: each convention below is a property of its layout, not an
// instruction to the person drawing. The steps therefore describe what the reader may rely on, and
// the one judgment that remains (sd6Verify) is a read-back against the five properties, not a
// re-drawing. Porting the renderer into the binary is chartered (dcStpaDiagramInTheBinary, D0221:
// deterministic logic goes in the binary); until then the tool is the sanctioned interim.
package ProcessStpaDiagram {
    private import EngineElement::*;
    private import EngineProcess::*;

    action stpaDiagram : Process {
        :>> id = "797ba21e-484c-4539-9407-c23933144494";
        :>> title = "STPA diagram";
        :>> purpose = "Render a computed control structure as a diagram a reader can trust: authority descending on the vertical axis, control actions down and feedback up, every edge labelled with what it carries, right-angle routing with no overlapping segments, and a semicircular hop wherever two edges cross.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: a control structure (this project's own, from `keel show control-structure`,
        // or any EngineSafety Controller/ControlAction/Feedback set) is to be shown to a human. NOT
        // for hand-drawn architecture sketches - the input is always computed JSON.
    }

    action sd0Compute : ProcessStep {
        :>> id = "b8ca7d2e-b2a1-474d-aacb-78bc1648f3d6";
        :>> title = "Start from the computed structure, never from a drawing";
        :>> triggerCondition = "A control structure is to be shown.";
        :>> actionText = "Run `keel show control-structure [ROOT]` and feed its JSON to the tool. Nothing about a specific controller, action or feedback path is typed by hand: the diagram is a VIEW of the JSON and redraws when the facts change (a removed hook event disappears from the picture on the next run). If the picture needs a box the JSON lacks, the fix is in the view or the authored residue, not in the drawing.";
        :>> producedArtifact = "The JSON of the structure to be drawn.";
        :>> owner = Owner::ai;
    }

    action sd1Axis : ProcessStep {
        :>> id = "c2177df0-72ae-4c21-8f29-7001ae64a924";
        :>> title = "Authority is the vertical axis, descending";
        :>> triggerCondition = "The structure is loaded.";
        :>> actionText = "The highest authority sits at the top and the controlled processes at the bottom; each controller row is labelled with what kind of authority it holds (human; delegated and independent controls; the local commit gate; the turn boundary; the operators). A reader learns who outranks whom from vertical position alone. Controllers are placed in the GAPS between process columns so that no vertical line ever passes through a box.";
        :>> producedArtifact = "Rows by authority; processes in a bottom row; controllers in inter-column gaps.";
        :>> owner = Owner::ai;
    }

    action sd2Direction : ProcessStep {
        :>> id = "7d42877b-189f-4bb6-a947-18cf033659f7";
        :>> title = "Control goes down on the left, feedback comes up on the right";
        :>> triggerCondition = "Rows are placed.";
        :>> actionText = "A control action is a solid line that leaves the issuer's bottom-LEFT and enters the process's top-LEFT; feedback is a dashed line that leaves the process's top-RIGHT and enters the receiver's bottom-RIGHT. The two never share a side of a box, so direction is legible without reading an arrowhead, and the arrowheads agree with it anyway.";
        :>> producedArtifact = "Every edge assigned a side and a direction.";
        :>> owner = Owner::ai;
    }

    action sd3Ortholinear : ProcessStep {
        :>> id = "e73fa47e-a8c0-4b8b-abda-42f9f61b0323";
        :>> title = "Right angles only, and no two segments share a line";
        :>> triggerCondition = "Directions are assigned.";
        :>> actionText = "Each edge is exactly vertical, horizontal, vertical. Each edge owns its horizontal channel (a unique y in the band below its issuer's row) and its own trunk and drop x (a unique slot on the box edge), so no two segments are ever collinear and a reader can follow any line end to end. Curves and diagonals are refused because they read as routing rather than relation and because their label position is ambiguous.";
        :>> producedArtifact = "A set of non-overlapping orthogonal segments, one channel per edge.";
        :>> owner = Owner::ai;
    }

    action sd4Label : ProcessStep {
        :>> id = "ced69bbb-2723-4ba4-b708-f2700017698e";
        :>> title = "Every edge says WHAT PASSES along it";
        :>> triggerCondition = "Segments exist.";
        :>> actionText = "The label is the point of the diagram. It names the data or verdict the edge carries - the acceptance record and its fields, the verdict JSON per hook event, the commands that write, the refusal and what each hook runs, the run status and how it reaches the human - never 'an action' or a count. It sits ON the edge's horizontal channel, at the first position along it that no other edge's vertical passes behind; a channel shorter than its label is extended to the label as a leader so the label is never disconnected from its line.";
        :>> producedArtifact = "One label per edge, on its channel, stating what is passed.";
        :>> owner = Owner::ai;
    }

    action sd5Crossings : ProcessStep {
        :>> id = "cebd172a-8d84-4401-a4b6-309378173611";
        :>> title = "A crossing is a semicircle, never a junction";
        :>> triggerCondition = "Segments and labels are placed.";
        :>> actionText = "Wherever a vertical segment crosses another edge's horizontal, the vertical hops it with a small semicircle. The reader is told once, in the legend, that a semicircle is a crossing and not a connection; nothing else in the picture may look like a junction, because in a control structure a false junction is a false authority.";
        :>> producedArtifact = "Hops at every crossing; a legend line stating the convention.";
        :>> owner = Owner::ai;
    }

    action sd6Verify : ProcessStep {
        :>> id = "b88c4ef7-2351-44a4-914b-716c608d37c1";
        :>> title = "Look at it before anyone else does";
        :>> triggerCondition = "The SVG is rendered.";
        :>> actionText = "Render the SVG in a browser and read it back against the five properties above - axis, direction, right angles with nothing shared, labels that state what passes, hops at crossings - at full size and at a zoomed crop of the densest region. The tool enforces the properties by construction, but a label can still hide a line it was placed over or float past a short channel; those are the two defects the first review found, and they are visible only by looking. Fix in the tool, never by hand-editing the SVG.";
        :>> producedArtifact = "A reviewed diagram, embedded where the human reads (the console or the published page).";
        :>> owner = Owner::ai;
    }
}
