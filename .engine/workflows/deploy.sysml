// =====================================================================
// DeployWorkflow — release, configuration management, deployment, release-level
// V&V. Release is authored; baseline / release notes / safety case are VIEWS.
// See spec §8.
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

    part declare : Phase {
        :>> title = "Declare release";
        :>> produces = (releaseType, baselineType);
    }
    part systemVnV : Phase {
        :>> title = "System V&V";
        :>> consumes = releaseType;
    }
    part safetyValidation : Phase {
        :>> title = "Safety-case validation";
        :>> consumes = releaseType;
        :>> produces = safetyCaseType;
    }
    part deployPhase : Phase {
        :>> title = "Deploy";
        :>> consumes = (releaseType, safetyCaseType);
        :>> produces = releaseNotesType;
    }

    part deploy : Workflow {
        :>> title = "Deploy";
        :>> purpose = "Release, configuration management, deployment, and release-level V&V.";
        :>> cadence = "per-release";
        :>> phases = (declare, systemVnV, safetyValidation, deployPhase);
    }
}
