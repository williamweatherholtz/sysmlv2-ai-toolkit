// =====================================================================
// BusinessWorkflow — the Business workflow, authored as DATA.
//
// The onboarding / "what-why" workflow: capture subjective needs and how
// the system is used. Run once; re-enter on a pivot. Instance of the
// EngineWorkflow meta-model. See the design spec §5.
//
// This is the template the other five workflows follow.
// =====================================================================
package BusinessWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    // ---- Artifact types this workflow produces (all authored / real) ----
    part briefType : ArtifactType {
        :>> title = "Brief / Opportunity";
        :>> schemaType = "Brief";
        :>> nature = "real";
    }
    part personaType : ArtifactType {
        :>> title = "Persona";
        :>> schemaType = "Persona";
        :>> nature = "real";
    }
    part needType : ArtifactType {
        :>> title = "Need";
        :>> schemaType = "Need";
        :>> nature = "real";
    }
    part useCaseType : ArtifactType {
        :>> title = "Use Case";
        :>> schemaType = "UseCase";
        :>> nature = "real";
    }

    // ---- Gate A: needs baselined ----
    part chkTraced : GateCheck {
        :>> title = "Every Need traced to a persona/use case";
        :>> criterion = "no orphan Need";
    }
    part chkCriteria : GateCheck {
        :>> title = "Verification criteria present";
        :>> criterion = "each Need has verify-linked criteria";
    }
    part gateA : Gate {
        :>> title = "Needs baselined";
        :>> kind = "exit";
        :>> checks = (chkTraced, chkCriteria);
    }

    // ---- Critics that run at Gate A ----
    part cStakeholder : CriticBinding {
        :>> critic = "stakeholder";
        :>> severityThreshold = "medium";
    }
    part cDevilsAdvocate : CriticBinding {
        :>> critic = "devils-advocate";
        :>> severityThreshold = "medium";
    }
    part cPremortem : CriticBinding {
        :>> critic = "premortem";
        :>> severityThreshold = "medium";
    }

    // ---- Phases: Brief -> Personas -> Needs -> Use Cases ----
    part briefPhase : Phase {
        :>> title = "Brief";
        :>> order = 1;
        :>> produces = briefType;
    }
    part personasPhase : Phase {
        :>> title = "Personas";
        :>> order = 2;
        :>> produces = personaType;
    }
    part needsPhase : Phase {
        :>> title = "Needs";
        :>> order = 3;
        :>> produces = needType;
    }
    part useCasesPhase : Phase {
        :>> title = "Use Cases";
        :>> order = 4;
        :>> produces = useCaseType;
        :>> exitGate = gateA;
        :>> critics = (cStakeholder, cDevilsAdvocate, cPremortem);
    }

    // ---- The workflow ----
    part business : Workflow {
        :>> title = "Business";
        :>> purpose = "Capture subjective needs and how the system is used (what / why).";
        :>> cadence = "once";
        :>> phases = (briefPhase, personasPhase, needsPhase, useCasesPhase);
    }
}
