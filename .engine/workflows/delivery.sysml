// =====================================================================
// DeliveryWorkflow — build and verify, one item at a time (continuous).
// Mirrors the existing agile loop (processes/agile-workflow.sysml); each
// phase's procedure is an Engine Process referenced by name for now
// (procedureRef becomes a typed ref once the core schema parses).
// DoR is the entry gate at Standup; DoD is the exit gate at Done. Spec §7.
// =====================================================================
package DeliveryWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part storyType : ArtifactType { :>> title = "Story"; :>> schemaType = "Story"; :>> nature = "real"; }
    part testType : ArtifactType { :>> title = "Test"; :>> schemaType = "Test"; :>> nature = "real"; }
    part testResultType : ArtifactType { :>> title = "Test Result"; :>> schemaType = "TestResult"; :>> nature = "real"; }

    // ---- DoR entry gate / DoD exit gate ----
    part chkDoR : GateCheck {
        :>> title = "Definition of Ready"; :>> criterion = "acceptance criteria as verify-linked Tests; sized; linked";
    }
    part dorGate : Gate { :>> title = "Definition of Ready"; :>> kind = "entry"; :>> checks = chkDoR; }
    part chkDoD : GateCheck {
        :>> title = "Definition of Done"; :>> criterion = "satisfaction == verified";
    }
    part dodGate : Gate { :>> title = "Definition of Done"; :>> kind = "exit"; :>> checks = chkDoD; }

    // ---- Phases: refine -> standup -> implement -> review -> done ----
    part refine : Phase {
        :>> title = "Refine"; :>> order = 1; :>> produces = (storyType, testType);
        :>> procedureRef = "EngineProcesses::refinement";
    }
    part standup : Phase {
        :>> title = "Standup"; :>> order = 2; :>> entryGate = dorGate;
        :>> procedureRef = "EngineProcesses::standup";
    }
    part implement : Phase {
        :>> title = "Implement"; :>> order = 3; :>> produces = testResultType;
        :>> procedureRef = "EngineProcesses::implementation";
    }
    part review : Phase { :>> title = "Review"; :>> order = 4; }
    part done : Phase { :>> title = "Done"; :>> order = 5; :>> exitGate = dodGate; }

    part delivery : Workflow {
        :>> title = "Delivery";
        :>> purpose = "Build and verify, one item at a time.";
        :>> cadence = "per-item";
        :>> phases = (refine, standup, implement, review, done);
    }
}
