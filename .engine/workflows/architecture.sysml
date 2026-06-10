// =====================================================================
// ArchitectureWorkflow — technical "how" that satisfies the Needs.
// Three TOGAF domains (Data / Application / Technology) run in PARALLEL
// (no consumes between them); Allocation & Interfaces consumes their
// outputs (a fan-in). ICDs + allocation matrix are VIEWS. See spec §6.
// DoD/critics are verify-linked Tests (added during dogfood), not authored here.
// =====================================================================
package ArchitectureWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    // ---- Artifact types ----
    part systemReqType : ArtifactType { :>> title = "System Requirement"; :>> schemaType = "SystemRequirement"; :>> nature = "real"; }
    part subsystemReqType : ArtifactType { :>> title = "Subsystem Requirement"; :>> schemaType = "SubsystemRequirement"; :>> nature = "real"; }
    part componentReqType : ArtifactType { :>> title = "Component Requirement"; :>> schemaType = "ComponentRequirement"; :>> nature = "real"; }
    part componentType : ArtifactType { :>> title = "Component"; :>> schemaType = "Component"; :>> nature = "real"; }
    part designElementType : ArtifactType { :>> title = "Design Element"; :>> schemaType = "DesignElement"; :>> nature = "real"; }
    part decisionType : ArtifactType { :>> title = "Architecture Decision"; :>> schemaType = "Decision"; :>> nature = "real"; }
    part icdType : ArtifactType {
        :>> title = "Interface (ICD)"; :>> schemaType = "ICD"; :>> nature = "view";
        :>> derivation = "from Component port defs + connections";
    }
    part allocType : ArtifactType {
        :>> title = "Allocation matrix"; :>> schemaType = "AllocationMatrix"; :>> nature = "view";
        :>> derivation = "from allocate edges";
    }

    // ---- Phases: 3 parallel domains + a fan-in Allocation phase ----
    part dataArch : Phase {
        :>> title = "Data Architecture";
        :>> produces = (systemReqType, designElementType);
    }
    part appArch : Phase {
        :>> title = "Application Architecture";
        :>> produces = (componentType, componentReqType, designElementType);
    }
    part techArch : Phase {
        :>> title = "Technology Architecture";
        :>> produces = (subsystemReqType, decisionType);
    }
    part allocPhase : Phase {
        :>> title = "Allocation & Interfaces";
        :>> consumes = (systemReqType, componentType, subsystemReqType);
        :>> produces = (icdType, allocType);
    }

    part architecture : Workflow {
        :>> title = "Architecture";
        :>> purpose = "Set the technical direction that satisfies the Needs (Data / Application / Technology).";
        :>> cadence = "once";
        :>> phases = (dataArch, appArch, techArch, allocPhase);
    }
}
