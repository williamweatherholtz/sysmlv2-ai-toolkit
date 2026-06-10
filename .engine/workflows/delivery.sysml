// =====================================================================
// DeliveryWorkflow — build and verify, one item at a time (continuous).
// Mirrors the agile loop; phase procedure referenced by name (procedureRef).
// DoR/DoD are verify-linked Tests (pre/post conditions), not authored gates.
// See spec §7.
// =====================================================================
package DeliveryWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part storyType : ArtifactType { :>> title = "Story"; :>> schemaType = "Story"; :>> nature = "real"; }
    part testType : ArtifactType { :>> title = "Test"; :>> schemaType = "Test"; :>> nature = "real"; }
    part testResultType : ArtifactType { :>> title = "Test Result"; :>> schemaType = "TestResult"; :>> nature = "real"; }

    part refine : Phase {
        :>> title = "Refine";
        :>> produces = (storyType, testType);
        :>> procedureRef = "EngineProcess::refinement";
    }
    part standup : Phase {
        :>> title = "Standup";
        :>> consumes = storyType;
        :>> procedureRef = "EngineProcess::standup";
    }
    part implement : Phase {
        :>> title = "Implement";
        :>> consumes = storyType;
        :>> produces = testResultType;
        :>> procedureRef = "EngineProcess::implementation";
    }
    part review : Phase {
        :>> title = "Review";
        :>> consumes = testResultType;
    }
    part done : Phase {
        :>> title = "Done";
        :>> consumes = testResultType;
    }

    part delivery : Workflow {
        :>> title = "Delivery";
        :>> purpose = "Build and verify, one item at a time.";
        :>> cadence = "per-item";
        :>> phases = (refine, standup, implement, review, done);
    }
}
