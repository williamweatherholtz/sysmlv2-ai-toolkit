// D0190 (PROPOSED — NOT YET ACCEPTED) — D0190: one declared version identity, parity-checked before authoring
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0190Accept event on sign-off.
package Decision0190 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - governs the engine-version.toml contract, the parity warning at
    // the gate entry points, and the version-identity reporting surfaces.
    #ProspectiveChange part d0190 : Decision {
        :>> id = "1ab6fa76-f80d-4671-8352-afbeac7d291f";
        :>> title = "D0190: one declared version identity, parity-checked before authoring";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Two verification FAILs recorded in sprint 394 name the same gap from two sides. srDcEngineVersionParity: no project declaration states which engine version the checks are defined against, and nothing compares a contributor's binary to it - the gap that already produced two field incidents (issue089: a v0.2.0 binary against a pre-D0133 on-disk engine, 566 violations, every commit blocked; issue090: absent rules read as six hard guard failures). srRelSemverReconciled: the binary reports 0.2.3, the API/write contract reports 2.0.0, and the claude-surface generator carries its own SURFACE_VERSION - three version identities that move independently with nothing reconciling them and no recorded reason for the divergence.";
        :>> decision = "PROPOSED: (1) keel init and keel migrate write a declared engine version into .engine/contracts/engine-version.toml - the version the on-disk engine's checks are defined against, stamped by the binary that wrote it. (2) Every gate entry point (validate, guard, hook stop) compares its own version against the declaration and WARNS on mismatch - advisory in the guided profile, naming keel migrate as the remedy; never blocking, because a version skew is not dishonest state (D0098). (3) The binary version is the single declared semver: the API contract version and SURFACE_VERSION become derived facts REPORTED alongside it (keel version and /api/version list all three with the binary version as the identity), and a breaking API change is recorded in the release Decision rather than by an independently-versioned constant. (4) The two failing verification records flip only when the machinery lands and is probed.";
        :>> rationale = "The parity check is the control the issue089/090 class has been missing: both incidents were a binary-vs-on-disk-engine skew that nothing detected until a downstream team was blocked at commit time. Declaring the version at init/migrate time makes the skew computable in one comparison; warning at the gate entry points puts the detection where every contributor already is. Folding the semver reconciliation into the same decision is deliberate - both FAILs are the same fact (version identity is nowhere declared), and two decisions would charter the same file twice.";
        :>> consequences = "IF ACCEPTED: one new contract file (engine-version.toml, exempt from the process-definition guard class like unit-ids.toml), a version stamp in init/migrate, a comparison + warning in the three gate entry points, and the version surfaces reporting all three identities. The two sprint-394 FAIL records flip on probe evidence. COST: the declaration is one more file downstream projects carry, and a stale declaration after a manual engine edit would warn falsely until keel migrate re-stamps - acceptable, since a manual engine edit outside migrate is already the unsanctioned path. IF REJECTED: the FAILs stand as the recorded state and the issue089/090 class keeps relying on binary-side tolerance fixes after the fact.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0190Accept : Test { :>> id = "ac6d740e-499d-4fc0-9c3c-8acb6f6c3834"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'approved. keep the momentum going' - said in direct response to the report presenting the four queued Decisions (D0189, D0190, D0192, D0193) and asking for signatures. Covers this Decision as one of that presented queue. D0192 is NOT covered by the same word: accepting it requires choosing between its two costed options, which a bare approval does not do - it stays proposed until they name A or B."; }
    part d0190AcceptR1 : TestResult { :>> id = "4cde1c00-b866-4bd2-abb6-e1dd26c01f59"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ef26b9"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
