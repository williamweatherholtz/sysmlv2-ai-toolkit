// ProjectSitting06 — the last item in the human's queue. They accepted sprint 354's record in the
// obligations deck; this covers it. The 313 pre-D0155 sittings stay REPORTED as unreviewed, which is
// what D0155 chose over hiding them, and their acceptance of the aggregate row does not change that.
package ProjectSitting06 {
    private import EngineElement::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    verification sitting06Review : Test {
        :>> id = "bcbf2c15-80eb-5687-92cb-2392c6b8afa5";
        :>> title = "Sitting 06 review - sprint 354 (deck review intake and the D0165 delegation) (2026-08-20)";
        :>> createdAt = "2026-08-20";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Per-sitting sprint review (D0049 single human gate; D0073 mechanism). wweatherholtz accepted `deckIntakeStory` in the obligations deck of 2026-08-20, verdict ACCEPT, no per-item note - the sprint in which their own deck review was taken in and finding disposition was delegated under D0165. SCOPE, STATED SO IT IS NOT READ WIDER: this covers ONE sprint. In the same export they also marked the aggregate row '274 sprint(s) awaiting a sitting review' ACCEPT, for the second time, and that row is a SUMMARY rather than an item - it names no sprint and there is nothing to attest against it. The 313 sittings predating D0155's grandfather line remain reported as accepted-unreviewed under that Decision, which deliberately chose to REPORT them rather than delete them; covering them with a blanket review would turn a number D0155 kept honest into a zero, which is the Goodhart move invariant 7 warns about and would be a worse outcome than the noise. If they want those 313 formally reviewed rather than grandfathered, that is a different and much larger piece of work and should be asked for in those terms.";
    }
    part sitting06ReviewR1 : TestResult {
        :>> id = "a3e0fea9-9a47-508c-bbe7-905b62a33753";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "9354c4a";
        :>> judgedAt = "2026-08-20";
        :>> judgedBy = "wweatherholtz";
    }

    #Covers dependency from sitting06Review to deckIntakeStory;
}
