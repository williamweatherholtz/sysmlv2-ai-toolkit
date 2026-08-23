// D0199 (status: the `status` field below is the truth) — D0199: the distributed-collaboration unit asserts the ownership guard it owns
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0199Accept event on sign-off.
package Decision0199 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0199 : Decision {
        :>> id = "9f7e0803-2ce5-4fec-b068-6e103b844a23";
        :>> title = "D0199: the distributed-collaboration unit asserts the ownership guard it owns";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue226: process-enforcement.toml claimed D0108's ownership rule was unchecked while the ownership guard has existed since sprint 293 - the guard was CORE (named in no unit) so the D0139(D) enforcement map, which computes a unit's guards from its own assert-constraint statements, honestly read zero for the unit whose discipline the guard enforces, and the stale prose beside it went unswept. Sprint 421 chartered work to build a guard that already existed, on the strength of that stale claim.";
        :>> decision = "The distributed-collaboration process file asserts the constraint 'enforcesOwnership : ownership' (the D0191 unit-ownership shape), and the stale [distributedCollaboration] UNDER-GUARDED entry is deleted from process-enforcement.toml. THE ONE SEMANTIC DELTA, stated plainly: a unit-owned guard deactivates WITH its unit, so a project that deactivates distributed-collaboration drops the ownership check - which is correct, because a single-contributor project has no non-owner to guard against, and every other guard is untouched. APPLIED PROVISIONALLY under D0139(D)'s accepted unit-ownership architecture; REJECTING this decision reverts one assert line and restores one toml entry.";
        :>> rationale = "The enforcement map exists so that what is enforced is COMPUTED from the model, never restated in prose - the stale entry is exactly the dual truth D0018 forbids, and binding the guard to its unit removes the prose claim's job entirely. The alternative (leave the guard core and only fix the prose) keeps a hand-maintained claim beside a computable fact, which is how it went stale the first time.";
        :>> consequences = "hardening's processEnforcement reads ENFORCED 14 with distributedCollaboration carrying guards=1 (read back live); the ownership guard still runs by default everywhere (the unit is active); deactivating the unit now visibly drops it, reported by keel activation; issue226 resolves via dcOwnershipDiffGuard.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0199Accept : Test { :>> id = "21916288-9632-4e10-8a84-88b3d4198e0c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Accepted via their deck taps (uid 9f7e0803, At 2026-08-22T22:12:24Z and 2026-08-23T01:15:24Z, deck HEADs 2663337/55d1381). The provisional application (unit asserts the ownership guard; stale contract claim deleted) is ratified as applied."; }
    part d0199AcceptR1 : TestResult { :>> id = "78eab463-453a-473c-9370-5cadbbeddb79"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71f6453"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
