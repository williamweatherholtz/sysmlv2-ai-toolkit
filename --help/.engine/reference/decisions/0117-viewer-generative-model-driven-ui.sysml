// =====================================================================
// D0117 — the keel-viewer UI is MODEL-DRIVEN / GENERATIVE: derived at runtime from keel's DECLARED model
// (item types + attribute fields, declared processes/skills, the write API), NOT hardcoded views/actions.
// Model evolution (new process, new attribute, new type) => manipulable in the UI with no viewer code.
// Refines shape B (D0114); generalises srServeModelDrivenRegistry (D0109) from the launchable set to the
// WHOLE UI. Serves N-17. Human-directed 2026-07-15. Accepted.
// =====================================================================
package Decision0117 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0117 : Decision {
        :>> id = "10a1b0c1-4117-42a3-ab5c-516273849717";
        :>> title = "D0117: the viewer UI is model-driven / GENERATIVE (from declared schema + processes + write API), not hardcoded";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Deciding the write/manipulation surface (N-16), the human reframed the whole UI (2026-07-15): 'it should dynamically integrate with keel data and processes and intelligently generate a UI... we're not hard coding actions here, we're developing an engine. when a process is added, it should be manipulable in the UI. same for new attribute fields. evolution!' keel already applies this to the LAUNCHABLE set (srServeModelDrivenRegistry/D0109: only declared processes/skills are launchable, no freeform). N-17 extends the principle to the ENTIRE viewer UI.";
        :>> decision = "The viewer UI is GENERATED at runtime from keel's declared model — (1) item TYPES + their ATTRIBUTE fields (schema/core) drive the display + edit forms; (2) declared PROCESSES/SKILLS (the launchable/actionable set, already /api/launchables) drive the available actions; (3) the guarded write API + keel PROCESSES (record-Decision, supersede, apply-review — N-16) drive how changes are made. NOTHING is hardcoded per type/process/field: adding a process, an attribute, or an item type in the model makes it viewable + manipulable with no viewer code change. keel's job (this repo) is to EXPOSE the declared model machine-readably (a /api/schema of types+attributes, alongside /api/launchables) so the viewer app (shape B) can generate itself; the generation + rendering live in the viewer app.";
        :>> rationale = "ALTERNATIVES: (i) hardcode views + write forms per type/process (the conventional app) — REJECTED: it does not evolve; every new process/attribute needs viewer code, and it contradicts the engine premise (the model is the source of truth; the UI should follow it, like keel's declared views/launchables already do). (ii) generate everything from a fixed template ignoring the schema — REJECTED: wouldn't reflect real attributes/constraints. CHOSEN — model-driven generation from the declared schema + processes + write API: the UI is a projection of the model, so the model's evolution IS the UI's evolution. HARD (the human acknowledged) but it's the value. CRITERIA: evolves-with-the-model x no-hardcoded-actions x reuses-declared-registry-pattern (D0109) x engine-not-app.";
        :>> consequences = "ACCEPTED by the human 2026-07-15 (discrete d0117Accept TestResult). Refines shape B (D0114): the versioned keel API must expose the declared model machine-readably — item types + attribute fields (a /api/schema; parser bodies are skipped today, so a text-scan of schema/core is the first mechanism) alongside the existing /api/launchables (processes/skills). N-16 (modify via keel processes) is realised THROUGH the generated surface. The generation/rendering is the viewer app's (shape B), a large ongoing effort; keel's deliverable is the machine-readable declared-model API. First keel-side increment: /api/schema (declared types + attributes). Serves N-17 (#DerivedFrom)."; }

    #DerivedFrom dependency from d0117 to nViewerGenerativeUi;

    verification d0117Accept : Test { :>> id = "10a1b0c1-4117-42a3-bb5c-516273849717"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz directed D0117 (2026-07-15): the viewer must 'dynamically integrate with keel data and processes and intelligently generate a UI... not hard coding actions... when a process is added, it should be manipulable in the UI. same for new attribute fields. evolution!' — accepting the model-driven generative-UI architecture (refines shape B)."; }
    part d0117AcceptR1 : TestResult { :>> id = "10a1b0c1-4117-42a3-cb5c-516273849717"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "38a31a2"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "wweatherholtz"; }
}
