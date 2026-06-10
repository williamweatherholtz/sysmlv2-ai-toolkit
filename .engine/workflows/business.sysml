// =====================================================================
// BusinessWorkflow — the onboarding / "what-why" workflow, as DATA.
// Brief -> Personas -> Needs -> Use Cases (chain expressed via consumes).
// Gates/critics are NOT authored here: a phase's DoD is the verify-linked
// Tests on its output (added as instance Tests during dogfood). See spec §5.
// =====================================================================
package BusinessWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    // ---- Artifact types this workflow produces (all authored / real) ----
    part briefType : ArtifactType { :>> title = "Brief / Opportunity"; :>> schemaType = "Brief"; :>> nature = "real"; }
    part personaType : ArtifactType { :>> title = "Persona"; :>> schemaType = "Persona"; :>> nature = "real"; }
    part needType : ArtifactType { :>> title = "Need"; :>> schemaType = "Need"; :>> nature = "real"; }
    part useCaseType : ArtifactType { :>> title = "Use Case"; :>> schemaType = "UseCase"; :>> nature = "real"; }

    // ---- Phases: dependency chain via consumes/produces ----
    part briefPhase : Phase {
        :>> title = "Brief";
        :>> produces = briefType;
    }
    part personasPhase : Phase {
        :>> title = "Personas";
        :>> consumes = briefType;
        :>> produces = personaType;
    }
    part needsPhase : Phase {
        :>> title = "Needs";
        :>> consumes = personaType;
        :>> produces = needType;
    }
    part useCasesPhase : Phase {
        :>> title = "Use Cases";
        :>> consumes = needType;
        :>> produces = useCaseType;
    }

    part business : Workflow {
        :>> title = "Business";
        :>> purpose = "Capture subjective needs and how the system is used (what / why).";
        :>> cadence = "once";
        :>> phases = (briefPhase, personasPhase, needsPhase, useCasesPhase);
    }
}
