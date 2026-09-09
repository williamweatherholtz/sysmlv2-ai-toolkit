// D0403 (PROPOSED — NOT YET ACCEPTED) — decisionSurfacingStepFiveNamesTheFitMeasurement: step 5 of decision-surfacing runs fit_check beside the contract checker, and a builder ends in assert_fits
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0403Accept event on sign-off.
package Decision0403 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0403 : Decision {
        :>> id = "6efd30fe-dcac-4ebb-904a-de7f5d1d1eb0";
        :>> title = "decisionSurfacingStepFiveNamesTheFitMeasurement: step 5 of decision-surfacing runs fit_check beside the contract checker, and a builder ends in assert_fits";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0402 chose to MEASURE a brief's exhibits in a browser rather than estimate them, and built scripts/exec_brief/fit_check.py. That Decision was recorded without the process-change marker on the reading that adding an instrument leaves the process untouched; guard process-change refused the commit, because the decision-surfacing skill's step 5 is a locked process definition (D0070) and it now names the measurement. This Decision is the one clause that changes the process, and it carries the marker so it waits for the human as D0337 requires. NOT A FORK: the instrument and its cost are D0402; the only question here is whether the process names it, and a step that does not name the check it depends on is the issue390 shape - a page read as fitting because nothing said to measure it.";
        :>> decision = "Step 5 of the decision-surfacing process runs scripts/exec_brief/fit_check.py on the built page beside scripts/check_templates.py --brief, and every brief builder ends in assert_fits(out_path), so a builder that exits 0 has left a page whose text was measured to fit with web fonts on and blocked.";
        :>> rationale = "The contract checker is structural and browserless; the geometry check is the only one that knows where a glyph lands, and a process step that names one but not the other lets the second be skipped without anyone deciding to skip it. The skill text is where the agent reads the step, so the step's text is the control point.";
        :>> consequences = "SKILL.md step 5 carries both commands and the assert_fits paragraph; the .claude rendering follows by sync-claude. Until this Decision is accepted the step is proposed text, and the check still runs because the builder itself ends in it (D0402).";
    }
}
