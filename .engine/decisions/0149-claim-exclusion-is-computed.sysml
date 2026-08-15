// D0149 — ACCEPTED (wweatherholtz, 2026-08-15): correct D0129's claim-exclusion clause. A push
// rejection gives first-to-land-wins for INTEGRATION; it does not give exclusion over a claimed ITEM.
// Process-definition change (§3a) — sign-off is the d0149Accept event below.
package Decision0149 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0068/D0070): PROSPECTIVE, not safety. Work chartered under the old text —
    // sprint 300 above all — stays valid: the mechanism it described was wrong, but what it BUILT is
    // what this Decision now describes, so nothing downstream needs reprocessing. Marked because the
    // process-change guard hard-locks any process-def edit to a marked Decision, and it caught the
    // omission on this very commit.
    #ProspectiveChange part d0149 : Decision {
        :>> id = "5c2a7f10-84be-4d3e-9a61-2f7c05b8e934";
        :>> title = "D0149: claim exclusion is COMPUTED; the push does not provide it";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0129, the distributed-collaboration process, and the dcWorkClaim DoD all state that two contributors claiming the same item are excluded because the remote accepts exactly one of two concurrent claims, a git ref update being a compare-and-swap. Sprint 300 built the claim mechanism and tested it with TWO REAL CLONES of one origin, and the test DISPROVED the exclusion clause. What actually happens: both contributors claim, B's push is rejected, B merges and retries and lands — and B's claim is then just as present and just as valid as A's. BOTH CLAIMS LAND. The reason is a deliberate design choice made elsewhere in the same process: srDcPerActorWriteTargets routes each contributor's records to a per-actor file precisely SO THAT concurrent appends do not collide. That removes the very contention the exclusion argument depends on. The two requirements are individually correct and were never checked against each other.";
        :>> decision = "ACCEPTED: narrow the claim, do not discard it. RETAINED as true: a git ref update IS a compare-and-swap; a rejected push DOES mean someone else landed first; the loser MUST integrate before retrying; and for INTEGRATION that genuinely is first-to-land-wins with no lock server. CORRECTED: none of that yields mutual exclusion over a claimed ITEM, because after the mandatory merge both claims coexist in the tree. Exclusion over an item is therefore COMPUTED from the authored claims — the holder is the earliest un-expired `claimedAt`, tie-broken by the lower claim UUID — and every clone computes the same holder without coordinating, which is the property that actually matters. Two corollaries belong in the corrected text because each cost a wrong implementation: (i) 'whoever landed first' is NOT RECOVERABLE from merged history — two claims committed in parallel are siblings of equal ancestry depth, and their commit timestamps tie whenever the work happens in the same second, so every rule that tried to read git order silently degraded to an alphabetical tie-break; (ii) expiry must remove a claim from CONTENTION rather than only from the top of the ranking, or an aged-out claim keeps the holder slot and marks a live claim superseded, leaving the item held by nobody.";
        :>> rationale = "WHY THIS NEEDS A DECISION RATHER THAN AN EDIT. D0129 is an accepted record and sprint 300 is historical evidence; neither may be rewritten in place, so the correction arrives as a superseding fact. And the change is not cosmetic: it moves exclusion from a property of git — something a reader may assume holds everywhere, in any repo, for any concurrent write — to a property of one computed rule in this engine. A downstream project reading the current text would reasonably build its own coordination on the same false premise. WHY NOT THE OTHER DIRECTION: making claims contend for a single shared file would restore real push-level exclusion, and is REJECTED — it re-introduces exactly the textual conflict srDcPerActorWriteTargets exists to remove, and a conflict needs a human to resolve, which defeats the requirement that the loser re-selects with no human involved. That requirement holds today and is verified.";
        :>> consequences = "If ACCEPTED: correct the exclusion clause in the distributed-collaboration process, its SKILL.md, and the srDcWorkClaim requirement text; D0129's clause is superseded by this record rather than edited; `claim.rs` already states the corrected mechanism and needs no change. If REJECTED: the process text stands and `claim.rs` must be changed to match it, which means abandoning per-actor claim files and accepting a merge conflict as the exclusion mechanism — a real option, at the cost of the no-human-involved property. Either way the engine stops teaching a mechanism its own test disproved. Tracked as issue115.";
    }

    // Supersedes only the exclusion CLAUSE of D0129 — the rest of that Decision stands, which is why
    // this is a typed edge to the Decision rather than a rewrite of it.
    #Supersede dependency from d0149 to d0129;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0149Accept : Test { :>> id = "af88c9c4-48db-4dbb-9a44-df990c152d19"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz, 2026-08-15, verbatim: 'confirmed'. Given in direct reply to a message that asked about D0149 SPECIFICALLY and separately, after their earlier 'approved' had been recorded against D0150 alone rather than assumed to cover both. The message stated what D0149 corrects (the claim-exclusion clause sprint 300's two-clone test disproved), that it NARROWS rather than discards the claim - a rejected push is genuinely first-to-land-wins for integration, but yields no exclusion over a claimed item because after the mandatory merge both claims coexist - and what accepting permits: correcting the distributed-collaboration process text, its SKILL.md, and the srDcWorkClaim requirement statement, which still contains the disproved clause and is why that requirement was left unlinked by #Verify."; }
    part d0149AcceptR1 : TestResult { :>> id = "ecd48898-826b-4a16-90e6-a1d36f3a2585"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a08cf6a"; :>> judgedAt = "2026-08-15"; :>> judgedBy = "wweatherholtz"; }
}
