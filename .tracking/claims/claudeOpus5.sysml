// Claims authored by claudeOpus5 (D0147). PER-ACTOR FILE so concurrent claims by different
// contributors merge cleanly — the only contention left is the git ref update that decides
// who got the item (srDcPerActorWriteTargets).
//
// Liveness is NOT stored here. A claim is live if it is the newest on its item and under the
// expiry window; `keel claim --list` computes that. Nothing needs to be released.
package ProjectClaimsclaudeOpus5 {
    private import EngineElement::*;
    private import EngineWork::*;

    part claimclaudeOpus5001 : Claim {
        :>> id = "f3503f05-1275-4c3f-8c0a-5fc28f34a568";
        :>> title = "claim on keelArchViews";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> claimedItem = "keelArchViews";
        :>> claimedBy = "claudeOpus5";
        :>> claimedAt = "2026-08-15";
        :>> claimedAgainst = "0897093";
    }
}
