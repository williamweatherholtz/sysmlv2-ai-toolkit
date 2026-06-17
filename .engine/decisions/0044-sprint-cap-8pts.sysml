// =====================================================================
// D0044 — sprint-cap-8pts: raise solo+AI sprint capacity cap from 5 to 8 pts.
// Supersedes D0038 on the cap only; estimation guidelines unchanged.
// Accepted 2026-06-16 by wweatherholtz.
// =====================================================================
package Decision0044 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0044 : Decision {
        :>> id = "a46ed6da-7d16-4deb-a969-12b630de37d8";
        :>> title = "D0044: raise sprint capacity cap to ≤8 pts (from ≤5); Fibonacci scale unchanged";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Solo+AI team velocity has been capped at 5 pts per sprint. Several items (e.g. trackedMetadata, traceabilityView) are 5 pts each, and larger feature requests would benefit from single-sprint delivery without forced splits. The ≤5 cap was conservative for early sprint rhythm establishment (D0038). Consistent delivery suggests headroom to raise the cap.";
        :>> decision = "Raise the solo+AI sprint capacity cap from ≤5 pts to ≤8 pts per sprint. Fibonacci scale unchanged (1/2/3/5/8/13); items estimated at 13+ must still be split before scheduling. Combinations up to 8 pts total are valid (e.g. 5+3, 8, 3+3+2).";
        :>> rationale = "Velocity had been capped at 5 pts per sprint; several items (e.g. trackedMetadata, traceabilityView) are 5 pts each and larger feature requests would benefit from single-sprint delivery without forced splits. The ≤5 cap was conservative for early sprint-rhythm establishment (D0038), and consistent delivery suggests headroom to raise it. D0038's hour-guideline for 8 pts (2-3 days) remains the reference; this supersedes D0038 on the cap only — estimation guidelines unchanged.";
        :>> consequences = "Sprint planning may now accept a single 8-pt item or point combinations summing to ≤8. No schema changes. D0038 sprint skills (sprint-planning, sprint-review) and actualHours discipline remain in effect. D0038 status updated to superseded.";
    }

    verification d0044Accept : Test { :>> id = "a86a24c7-8cec-4f78-9108-44c8c49ee337"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0044 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0044AcceptR1 : TestResult { :>> id = "f4ac5fa1-3c19-4a8f-94cb-83e8af404999"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
