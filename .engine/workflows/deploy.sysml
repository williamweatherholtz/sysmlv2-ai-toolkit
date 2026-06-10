// =====================================================================
// DeployWorkflow — release, configuration management, deployment, and
// release-level V&V. The Release is authored (a declaration + membership
// rule); the baseline, release notes, and safety case are VIEWS. Spec §8.
// =====================================================================
package DeployWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part releaseType : ArtifactType { :>> title = "Release"; :>> schemaType = "Release"; :>> nature = "real"; }
    part baselineType : ArtifactType {
        :>> title = "Baseline"; :>> schemaType = "Baseline"; :>> nature = "view";
        :>> derivation = "from Release + commit match";
    }
    part releaseNotesType : ArtifactType {
        :>> title = "Release notes"; :>> schemaType = "ReleaseNotes"; :>> nature = "view";
        :>> derivation = "from release membership + commit diffs";
    }
    part safetyCaseType : ArtifactType {
        :>> title = "Safety case"; :>> schemaType = "SafetyCase"; :>> nature = "view";
        :>> derivation = "from loss->hazard->goal->req->V&V graph";
    }

    part chkVnV : GateCheck {
        :>> title = "System V&V passed"; :>> criterion = "release-level verification + validation complete";
    }
    part chkSafety : GateCheck {
        :>> title = "Safety-case validation"; :>> criterion = "safety goals validated; ASIL/SIL confirmed";
    }
    part releaseGate : Gate { :>> title = "Release approved"; :>> kind = "exit"; :>> checks = (chkVnV, chkSafety); }

    part cTrace : CriticBinding { :>> critic = "traceability"; :>> severityThreshold = "medium"; }

    // ---- Phases ----
    part declare : Phase {
        :>> title = "Declare release"; :>> order = 1; :>> produces = (releaseType, baselineType);
    }
    part systemVnV : Phase { :>> title = "System V&V"; :>> order = 2; }
    part safetyValidation : Phase { :>> title = "Safety-case validation"; :>> order = 3; :>> produces = safetyCaseType; }
    part deployPhase : Phase {
        :>> title = "Deploy"; :>> order = 4; :>> produces = releaseNotesType;
        :>> exitGate = releaseGate; :>> critics = cTrace;
    }

    part deploy : Workflow {
        :>> title = "Deploy";
        :>> purpose = "Release, configuration management, deployment, and release-level V&V.";
        :>> cadence = "per-release";
        :>> phases = (declare, systemVnV, safetyValidation, deployPhase);
    }
}
