package Engine { package Decisions {
    private import Engine::Core::*;

    part d0008 : Decision {
        :>> title = "STPA first-class; HARA and ASIL out of scope";
        :>> status = "accepted";
        :>> context = "Both STPA and HARA were researched and integrated, "
            + "converging on a shared Hazard and a shared SafetyRequirement "
            + "output. The user scoped HARA (and ASIL) out.";
        :>> decision = "Keep STPA in full (Losses, Hazards, control structure "
            + "incl. Sensors/Actuators/ProcessModels, UCAs, ControllerConstraints, "
            + "LossScenarios) as the optional schema/safety module. DROP HARA "
            + "(operational situations, hazardous events, S/E/C, safety goals) "
            + "and ASIL (no asil attribute, no rolledUpAsil view). With HARA "
            + "gone, Hazard is a single unambiguous STPA concept (no STPA/HARA "
            + "type split).";
        :>> consequences = "Pros: simpler safety model; one Hazard type. Cons: "
            + "no ISO-26262 ASIL classification — re-addable later as its own "
            + "optional module if a standard requires it (architecture supports "
            + "it at zero core cost).";
    }
}}
