// =====================================================================
// ArchitectureWorkflow — technical "how", as a native action behavior.
// Three TOGAF domains run in PARALLEL (no succession between them) and converge
// into Allocation & Interfaces (three successions into it). See spec §6.
// =====================================================================
package ArchitectureWorkflow {
    private import EngineRequirements::*;   // SystemRequirement, SubsystemRequirement
    private import EngineArchitecture::*;   // Component
    private import EngineArtifacts::*;      // ICD (#View)

    action def Architecture {
        action dataArch { out sysReq : SystemRequirement; }
        action appArch { out comp : Component; }
        action techArch { out subReq : SubsystemRequirement; }
        action allocIface {
            in sr : SystemRequirement;
            in c : Component;
            in br : SubsystemRequirement;
            out icd : ICD;
        }

        // dataArch | appArch | techArch are independent (parallel); all precede allocIface
        first dataArch then allocIface;
        first appArch then allocIface;
        first techArch then allocIface;

        flow from dataArch.sysReq to allocIface.sr;
        flow from appArch.comp to allocIface.c;
        flow from techArch.subReq to allocIface.br;
    }
}
