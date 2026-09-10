// D0433 (PROPOSED — NOT YET ACCEPTED) — stpaDiagramIsDrawnByTheBinary: the stpa-diagram process draws the control-structure picture with keel show control-structure --svg, and .engine/tools/stpa_diagram.py is removed
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0433Accept event on sign-off.
package Decision0433 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0433 : Decision {
        :>> id = "3c5e8612-4048-428f-b4b9-f2d70fb9e2b7";
        :>> title = "stpaDiagramIsDrawnByTheBinary: the stpa-diagram process draws the control-structure picture with keel show control-structure --svg, and .engine/tools/stpa_diagram.py is removed";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0285 declared the diagram's five layout properties and the stpa-diagram process deployed them through a Python tool, .engine/tools/stpa_diagram.py, read by the skill's prose. dcStpaDiagramInTheBinary (chartered by D0285, accepted) ports that tool into keel-cli/src/view/stpa_diagram.rs, byte-equal to the tool on the same JSON, and the process file's header, the skill (both renderings) and the registry now name the binary command. The process file is under the D0070 lock, so the doc-sync of its renderer needs a marked Decision co-committed - the process-change guard refused the sprint 659 commit for exactly this, correctly.";
        :>> decision = "NOT A FORK: the direction is D0285's and the port is the accepted backlog item's DoD; this Decision records the process-definition change the lock requires. The stpa-diagram process's renderer is the binary: keel show control-structure --svg draws the SVG, keel render control-structure --mode graph puts it on a page, the console serves it at /view/control-structure, and the five D0285 properties are unit tests on the layout in stpa_diagram.rs rather than prose a reader checks by eye. The Python tool is deleted, and the snStpaDiagram Sensor and fbStpaDiagram Feedback with it, because a Sensor is a measure outside the CLI (D0363) and this is now a CLI lens whose channel the control structure derives from the CliCommand facts.";
        :>> rationale = "A picture drawn by a second program lags the structure it draws; the same binary that computes the structure drawing it cannot. The port's correctness is a fact (the golden fixture and the live comparison, both byte-equal once Python's CRLF stdout is normalised), not a resemblance, and the layout properties the skill used to ask a human to verify are asserted on every cargo test. The process file changes only in what it names as the renderer; its steps and their order are unchanged.";
        :>> consequences = ".engine/processes/stpa-diagram.sysml, .engine/skills/stpa-diagram/SKILL.md, .claude/skills/stpa-diagram/SKILL.md and registry.sysml name the binary command; .engine/tools/stpa_diagram.py is gone and migrate.rs PORTABLE_TOOLS and init_smoke.rs no longer carry it; engine-instruments.sysml loses the Sensor/Feedback pair. Stated: this Decision carries the process-change marker and so waits for the human under D0337; the code and records it names are already the accepted item's deliverable, and the commit rides the marker alone.";
    }
}
