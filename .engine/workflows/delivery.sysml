// =====================================================================
// DeliveryWorkflow — build/verify loop, as a native action behavior.
// refine -> standup -> implement -> review -> done. See spec §7.
// =====================================================================
package DeliveryWorkflow {
    private import EngineWork::*;           // Story
    private import EngineVerification::*;   // TestResult

    action def Delivery {
        action refine { out s : Story; }
        action standup { in s : Story; }
        action implement { in s : Story; out tr : TestResult; }
        action review { in tr : TestResult; }
        action closeOut { in tr : TestResult; }

        first refine then standup;
        first standup then implement;
        first implement then review;
        first review then closeOut;

        flow from refine.s to standup.s;
        flow from refine.s to implement.s;
        flow from implement.tr to review.tr;
        flow from implement.tr to closeOut.tr;
    }
}
