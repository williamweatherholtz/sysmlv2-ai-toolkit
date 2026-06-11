// =====================================================================
// DeployWorkflow — release/config/deploy + release-level V&V, as a native action
// behavior. System V&V and Safety-case validation run in PARALLEL after declare,
// then converge into deliver. Baseline/notes/safety-case are views. See spec §8.
// =====================================================================
package DeployWorkflow {
    private import EngineWork::*;           // Release
    private import EngineArtifacts::*;      // SafetyCase, ReleaseNotes (#View)

    action def Deploy {
        action declare { out r : Release; }
        action systemVnV { in r : Release; }
        action safetyValidation { in r : Release; out sc : SafetyCase; }
        action deliver { in r : Release; in sc : SafetyCase; out rn : ReleaseNotes; }

        first declare then systemVnV;
        first declare then safetyValidation;
        first systemVnV then deliver;
        first safetyValidation then deliver;

        flow from declare.r to systemVnV.r;
        flow from declare.r to safetyValidation.r;
        flow from declare.r to deliver.r;
        flow from safetyValidation.sc to deliver.sc;
    }
}
