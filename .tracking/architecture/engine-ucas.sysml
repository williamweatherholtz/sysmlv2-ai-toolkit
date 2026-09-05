// engine-ucas — STPA step 3/4 for keel's OWN workflow (stpa-self, D0313). First run 2026-09-05.
//
// The control actions analysed are the COMPUTED ones (`keel show control-structure`): this run walks
// `cmdRecord` - the write command that carries standing-consent auto-acceptance (D0207/D0291) -
// `cmdLand` - the integration push - and `cmdAppendResult` - the AI recording a verdict - because the landscape spike (D0282) and the human's own words
// (st065) point there. Each UCA is edged to the hazard it can lead to; each constraint is edged to the
// control that enforces it, or to the Issue that names the gap. Considered-and-safe types are listed
// in the run record (stpaRun1), so the reader can tell considered from not-considered.
package EngineUcas {
    private import EngineElement::*;
    private import EngineSafety::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // ── cmdRecord: standing-consent auto-acceptance of a NON-FORK Decision at record time ────────

    part ucaRecordForkDisguised : UnsafeControlAction {
        :>> id = "1a7c3e5f-9b2d-4f6a-8c0e-2d4f6a8c0e1b";
        :>> title = "UCA-R1: auto-accept PROVIDED to a Decision that is a fork in substance but carries no OPTION markers";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "cmdRecord (record decision) under standing consent, when the Decision's text weighs alternatives in prose without the `OPTION X (label)` shape that deck::fork_options reads.";
        :>> statement = "A direction the human owns is recorded as accepted by an AI on the spot - EHZ3, the AI recording a human judgment - because fork detection is lexical and the author controls the lexicon.";
    }
    dependency from ucaRecordForkDisguised to ehz3;

    part ucaRecordAcceptedBeforeFinal : UnsafeControlAction {
        :>> id = "2b8d4f6a-0c3e-4a7b-9d1f-3e5a7b9d1f2c";
        :>> title = "UCA-R2: auto-accept PROVIDED at record time, then the text is edited";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::wrongTimingOrder;
        :>> context = "cmdRecord accepts against HEAD before the Decision's own commit; the author may edit the signed fields afterwards.";
        :>> statement = "The text the acceptance binds to is not the text the tree carries - EHZ7 (a control weakened without a record) and EHZ3.";
    }
    dependency from ucaRecordAcceptedBeforeFinal to ehz7;

    part ucaRecordConsentNeverExpires : UnsafeControlAction {
        :>> id = "3c9e5a7b-1d4f-4b8c-ae2a-4f6b8cae2a3d";
        :>> title = "UCA-R3: standing consent APPLIED TOO LONG - after the human's context has changed";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::stoppedTooSoonAppliedTooLong;
        :>> context = "cmdRecord reads standingConsent from attestation-policy.toml; the consent carries no scope, no expiry and no re-attestation, and the decisions it covers grew from issues to every non-fork.";
        :>> statement = "Consent given for one class keeps accepting a wider class indefinitely - EHZ3; the landscape spike named standing consent as the artifact the field distrusts (D0282).";
    }
    dependency from ucaRecordConsentNeverExpires to ehz3;

    part ccRecordForkShape : ControllerConstraint {
        :>> id = "4d0f6b8c-2e5a-4c9d-bf3b-5a7c9dbf3b4e";
        :>> title = "CC-R1: a Decision that weighs alternatives must be recorded as a fork, and the recorder must be able to tell";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "record decision must not auto-accept a Decision whose text weighs alternatives; today only the OPTION marker is read, so the constraint is bound to the Issue naming the gap.";
    }
    dependency from ccRecordForkShape to ucaRecordForkDisguised;
    dependency from ccRecordForkShape to issue373;

    part ccRecordBindsText : ControllerConstraint {
        :>> id = "5e1a7c9d-3f6b-4dae-8c4c-6b8dae8c4c5f";
        :>> title = "CC-R2: an acceptance binds to the text it signed";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "An accepted Decision whose signed fields differ from their text at the binding SHA fails the gate; a later edit is re-bound, never silent (D0308).";
    }
    dependency from ccRecordBindsText to ucaRecordAcceptedBeforeFinal;
    dependency from ccRecordBindsText to gAcceptanceBindsToText;

    part ccRecordConsentScoped : ControllerConstraint {
        :>> id = "6f2b8dae-4a7c-4ebf-9d5d-7c9ebf9d5d6a";
        :>> title = "CC-R3: standing consent carries its scope and its expiry";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "Standing consent must name the class it covers and when it lapses or is re-attested; no control enforces this today - bound to issue342, whose resolver is the fork dcConsentIsScopedAndExpires.";
    }
    dependency from ccRecordConsentScoped to ucaRecordConsentNeverExpires;
    dependency from ccRecordConsentScoped to issue342;

    // ── cmdLand: the integration push ─────────────────────────────────────────────────────────────

    part ucaLandWithoutSuite : UnsafeControlAction {
        :>> id = "7a3c9ebf-5b8d-4fca-ae6e-8daefcae6e7b";
        :>> title = "UCA-L1: land PROVIDED over a tree whose test suite was not run";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "cmdLand gates validate + guards + rules (form) and pushes; the Rust suite is not part of the gate, only of CI after the push.";
        :>> statement = "A broken engine ships to every clone and CI goes red after the fact - EHZ9, an ungated artifact ships outward (issue360: CI red seven hours).";
    }
    dependency from ucaLandWithoutSuite to ehz9;

    part ucaLandBeforeGate : UnsafeControlAction {
        :>> id = "8b4daefc-6c9e-4adb-bf7f-9ebfadbf7f8c";
        :>> title = "UCA-L2: push PROVIDED before the gate runs (wrong order)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::wrongTimingOrder;
        :>> context = "A push that reaches the remote before validate/guards judge the tree - a plain `git push`, or land skipping its gate.";
        :>> statement = "Dishonest state reaches the shared ref - EHZ9 and EHZ2 (a write lands outside a validated channel).";
    }
    dependency from ucaLandBeforeGate to ehz9;

    part ucaLandMergedTreeUngated : UnsafeControlAction {
        :>> id = "9c5ebfad-7dae-4bec-8a8a-afcabec8a8a9";
        :>> title = "UCA-L3: after a rejected push, land STOPS TOO SOON - pushes the merge without re-gating the merged tree";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::stoppedTooSoonAppliedTooLong;
        :>> context = "cmdLand on rejection fetches, merges and retries; two contributions that pass alone can fail together (D0129).";
        :>> statement = "A merged tree neither side gated ships - EHZ9.";
    }
    dependency from ucaLandMergedTreeUngated to ehz9;

    part ccLandSuiteReceipt : ControllerConstraint {
        :>> id = "ad6fcabe-8ebf-4cfd-9b9b-bfdbcfd9b9ba";
        :>> title = "CC-L1: land carries a suite receipt for the engine it ships";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "A push of engine source must carry evidence the suite ran green on that tree; no control enforces it today - bound to issue360, whose resolver is dcLandRequiresASuiteReceipt.";
    }
    dependency from ccLandSuiteReceipt to ucaLandWithoutSuite;
    dependency from ccLandSuiteReceipt to issue360;

    part ccLandGatesFirst : ControllerConstraint {
        :>> id = "be7adbcf-9fca-4dae-8cac-cadedae8cacb";
        :>> title = "CC-L2: nothing reaches the remote before the tree is gated";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "The pre-push hook refuses a push whose tree has not passed the gate, and land gates before its first push (D0129); enforced by the pre-push control.";
    }
    dependency from ccLandGatesFirst to ucaLandBeforeGate;
    dependency from ccLandGatesFirst to ctlPrePushBehind;

    part ccLandGatesTheMerge : ControllerConstraint {
        :>> id = "cf8bedaf-afdb-4ebf-9dbd-dbefebf9dbdc";
        :>> title = "CC-L3: a merged tree is gated before it is pushed";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "land gates the MERGED tree before retrying the push (D0129); the pre-push hook is the control that refuses otherwise.";
    }
    dependency from ccLandGatesTheMerge to ucaLandMergedTreeUngated;
    dependency from ccLandGatesTheMerge to ctlPrePushBehind;

    // ── cmdAppendResult: the AI records a verification result ─────────────────────────────────────

    part ucaResultReceiptIsTestimony : UnsafeControlAction {
        :>> id = "e1a3c5b7-9d2f-4b4a-8c6e-0f2a4c6e8b1d";
        :>> title = "UCA-A1: a pass PROVIDED with a receipt the run never earned";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "cmdAppendResult (append-result / append-gate-result) with method test by an AI; guard 52 requires a `// RAN:` receipt and accepts whatever follows it.";
        :>> statement = "Completion is computed from evidence that does not exist - EHZ4 - because the receipt proves a sentence was typed, not that a run happened or finished (it did, 2026-09-04: a suite count written before the suite exited).";
    }
    dependency from ucaResultReceiptIsTestimony to ehz4;

    part ucaResultBoundToTheWrongTree : UnsafeControlAction {
        :>> id = "f2b4d6c8-0e3a-4c5b-9d7f-1a3b5d7f9c2e";
        :>> title = "UCA-A2: a pass PROVIDED against HEAD before the work it judges is committed (wrong order)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::wrongTimingOrder;
        :>> context = "cmdAppendResult --sha HEAD on a dirty working tree: the ordinary shape of a DoD pass, recorded before the commit that carries the work.";
        :>> statement = "judgedAgainst names a tree that does not contain what was verified; drift is measured from the wrong baseline and the claim reads false at its own SHA - EHZ4.";
    }
    dependency from ucaResultBoundToTheWrongTree to ehz4;

    part ucaResultHumanWordRecordedByAi : UnsafeControlAction {
        :>> id = "a3c5e7d9-1f4b-4d6c-8e8a-2b4c6e8a0d3f";
        :>> title = "UCA-A3: a confirmation PROVIDED by an AI actor as if it were the human's word";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "cmdAppendResult with method confirmation, judgedBy an AI, or a confirmation flip without the verbatim quote receipt (D0198).";
        :>> statement = "The AI records a human judgment - EHZ3 - the founding hazard of this engine's provenance rules.";
    }
    dependency from ucaResultHumanWordRecordedByAi to ehz3;

    part ccResultReceiptIsExternal : ControllerConstraint {
        :>> id = "b4d6f8ea-2a5c-4e7d-9f9b-3c5d7f9b1e4a";
        :>> title = "CC-A1: a machine-method pass closes on a fact keel can query, not on the agent's sentence";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "An AI-judged test result must be verifiable by something other than its author - a run id CI checks against the judged SHA; today the receipt is testimony, so the constraint is bound to issue374 (resolver dcExternalFactGate; the authority model is the fork D0312).";
    }
    dependency from ccResultReceiptIsExternal to ucaResultReceiptIsTestimony;
    dependency from ccResultReceiptIsExternal to issue374;

    part ccResultBindsToLandingTree : ControllerConstraint {
        :>> id = "c5e7a9fb-3b6d-4f8e-8a0c-4d6e8a0c2f5b";
        :>> title = "CC-A2: a result binds to the tree that carries the work it judges";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "When judgedAgainst precedes the commit that introduced the result, readers bind the result to its landing commit (the D0308 fallback, result-side); no control does this today - bound to issue375.";
    }
    dependency from ccResultBindsToLandingTree to ucaResultBoundToTheWrongTree;
    dependency from ccResultBindsToLandingTree to issue375;

    part ccResultHumanWordIsQuoted : ControllerConstraint {
        :>> id = "d6f8b0ac-4c7e-4a9f-9b1d-5e7f9b1d3a6c";
        :>> title = "CC-A3: a confirmation is a human's, and a chat-recorded flip carries their verbatim words";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> statement = "A method=confirmation result judged by an AI actor is refused, and a flip recorded from chat carries an Attest quote receipt (D0198); enforced by confirmation-authenticity and attestation-authority.";
    }
    dependency from ccResultHumanWordIsQuoted to ucaResultHumanWordRecordedByAi;
    dependency from ccResultHumanWordIsQuoted to gConfirmationAuthenticity;
    dependency from ccResultHumanWordIsQuoted to gAttestationAuthority;

    // ── the run record (stpa5): what was analysed, and what was considered safe ───────────────────

    verification stpaRun1 : Test {
        :>> id = "d09cfeb0-b0ec-4fca-aece-ecf0fcaaece1";
        :>> title = "stpa-self run 1 (2026-09-05) - cmdRecord, cmdLand and cmdAppendResult";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "ANALYSED: cmdRecord, cmdLand, cmdAppendResult. FRAME: EHZ1-EHZ9 as recorded in engine-safety.sysml; no hazard added. CONSIDERED SAFE: cmdRecord notProvided (a non-fork that stays proposed because the decider count is not one stalls work but leads to no hazard - the command says so and names the remedy); cmdLand notProvided (an unlanded tree leaves evidence local, which is a liveness cost and not a listed hazard); cmdAppendResult notProvided (an unrecorded result leaves the item not-done, the honest default - D0098); cmdAppendResult stoppedTooSoon (a result is one atomic write under the write lock; a partial write is refused by write_atomic parsing before rename). NINE UCAs recorded, nine constraints: four bound to controls (gAcceptanceBindsToText, ctlPrePushBehind x2, gConfirmationAuthenticity + gAttestationAuthority), five bound to Issues naming gaps (issue373 new - disguised fork; issue374 new - receipt is testimony; issue375 new - result bound to the wrong tree; issue342 - consent scope; issue360 - suite receipt). NOT ANALYSED THIS RUN: the other computed actions (39 in the local structure the guard reads; the remote branch-protection row is fetched live) - the stpa-currency guard names them until a run covers them.";
    }
    // RAN: keel show control-structure . read 2026-09-05 (42 local actions); cmdRecord, cmdLand, cmdAppendResult walked through all four UCA types; nine UCAs, nine constraints, every constraint edged to a control or an Issue; keel validate . clean
    part stpaRun1R1 : TestResult { :>> id = "8d009eac-d493-457b-b942-7040d7d5bad4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "494d293340d569d911ff8a3ff7b07abe9da29526"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
}
