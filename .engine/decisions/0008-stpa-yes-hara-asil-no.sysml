package Decision0008 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0008 : Decision {
        :>> id = "0b7a7f53-e22f-43ed-846b-fba8b1802a8a";
        :>> title = "STPA first-class; HARA and ASIL out of scope";
        :>> status = DecisionStatus::accepted;
        :>> context = "Both STPA and HARA were researched and integrated, "
            + "converging on a shared Hazard and a shared SafetyRequirement "
            + "output. The user scoped HARA (and ASIL) out.";
        :>> decision = "Keep STPA in full (Losses, Hazards, control structure incl. Sensors/Actuators/ProcessModels, UCAs, ControllerConstraints, LossScenarios) as the optional schema/safety module; DROP HARA (operational situations, hazardous events, S/E/C, safety goals) and ASIL (no asil attribute, no rolledUpAsil view); with HARA gone, Hazard is a single unambiguous STPA concept with no STPA/HARA type split.";
        :>> rationale = "Both STPA and HARA were researched and integrated (converging on a shared Hazard and SafetyRequirement), but the user scoped HARA and ASIL out, yielding a simpler safety model with one Hazard type; no ISO-26262 ASIL classification, which is re-addable later as its own optional module at zero core cost if a standard requires it.";
        :>> consequences = "Pros: simpler safety model; one Hazard type. Cons: "
            + "no ISO-26262 ASIL classification — re-addable later as its own "
            + "optional module if a standard requires it (architecture supports "
            + "it at zero core cost).";
    }

    verification d0008Accept : Test { :>> id = "964df1ba-f52d-4990-8303-36ffa23e841a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0008 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0008AcceptR1 : TestResult { :>> id = "daffeacd-1600-41fe-a3e7-b4080f6d3ec6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
