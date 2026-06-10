// =====================================================================
// ArchitectureWorkflow — the technical "how" that satisfies the Needs.
// Three TOGAF solution-side domains (Data / Application / Technology)
// plus a closing Allocation & Interfaces phase. See the spec §6.
// ICDs and the allocation matrix are VIEWS (computed). Gate B exits.
//
// NOTE: phase/domain -> artifact-type assignment below is a modeling
// choice, easy to revise as data.
// =====================================================================
package ArchitectureWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    // ---- Artifact types ----
    part systemReqType : ArtifactType {
        :>> title = "System Requirement"; :>> schemaType = "SystemRequirement"; :>> nature = "real";
    }
    part subsystemReqType : ArtifactType {
        :>> title = "Subsystem Requirement"; :>> schemaType = "SubsystemRequirement"; :>> nature = "real";
    }
    part componentReqType : ArtifactType {
        :>> title = "Component Requirement"; :>> schemaType = "ComponentRequirement"; :>> nature = "real";
    }
    part componentType : ArtifactType {
        :>> title = "Component"; :>> schemaType = "Component"; :>> nature = "real";
    }
    part designElementType : ArtifactType {
        :>> title = "Design Element"; :>> schemaType = "DesignElement"; :>> nature = "real";
    }
    part decisionType : ArtifactType {
        :>> title = "Architecture Decision"; :>> schemaType = "Decision"; :>> nature = "real";
    }
    part icdType : ArtifactType {
        :>> title = "Interface (ICD)"; :>> schemaType = "ICD"; :>> nature = "view";
        :>> derivation = "from Component port defs + connections";
    }
    part allocType : ArtifactType {
        :>> title = "Allocation matrix"; :>> schemaType = "AllocationMatrix"; :>> nature = "view";
        :>> derivation = "from allocate edges";
    }

    // ---- Gate B ----
    part chkSatisfies : GateCheck {
        :>> title = "Technical reqs satisfy Needs"; :>> criterion = "no orphan Need; no orphan requirement";
    }
    part chkAllocated : GateCheck {
        :>> title = "Allocation complete; interfaces resolve"; :>> criterion = "every component req allocated; ICDs resolve";
    }
    part gateB : Gate {
        :>> title = "Design baselined"; :>> kind = "exit"; :>> checks = (chkSatisfies, chkAllocated);
    }

    // ---- Critics at Gate B ----
    part cTrace : CriticBinding { :>> critic = "traceability"; :>> severityThreshold = "medium"; }
    part cPremortem : CriticBinding { :>> critic = "premortem"; :>> severityThreshold = "medium"; }

    // ---- Phases: 3 TOGAF domains + Allocation & Interfaces ----
    part dataArch : Phase {
        :>> title = "Data Architecture"; :>> order = 1;
        :>> produces = (systemReqType, designElementType);
    }
    part appArch : Phase {
        :>> title = "Application Architecture"; :>> order = 2;
        :>> produces = (componentType, componentReqType, designElementType);
    }
    part techArch : Phase {
        :>> title = "Technology Architecture"; :>> order = 3;
        :>> produces = (subsystemReqType, decisionType);
    }
    part allocPhase : Phase {
        :>> title = "Allocation & Interfaces"; :>> order = 4;
        :>> produces = (icdType, allocType);
        :>> exitGate = gateB;
        :>> critics = (cTrace, cPremortem);
    }

    part architecture : Workflow {
        :>> title = "Architecture";
        :>> purpose = "Set the technical direction that satisfies the Needs (Data / Application / Technology).";
        :>> cadence = "once";
        :>> phases = (dataArch, appArch, techArch, allocPhase);
    }
}
