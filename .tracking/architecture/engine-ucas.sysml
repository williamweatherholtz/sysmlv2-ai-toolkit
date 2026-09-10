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

    // ── run 2 (stpa3): the human's authority reaching the model - cmdAccept, cmdReject, cmdOverride, ──
    // ── humanDecidesOnChannel, consoleApprovesWrite, humanDirects; the human->model, human->work and ──
    // ── console->model edges walked in full ─────────────────────────────────────────────────────────

    part ucaAcceptGestureIsTyped : UnsafeControlAction {
        :>> id = "74bd015d-c62a-4c80-956e-92bb63aea6b9";
        :>> title = "UCA-AC1: an acceptance or rejection PROVIDED from an agent session whose note cites a gesture the recorder typed";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "cmdAccept or cmdReject run from an agent-marked session (CLAUDECODE set, no terminal) with delegatedRecording declared, the note carrying the substring console, deck, tty gesture, github comment or github.com/ and no quoted span.";
        :>> statement = "verdict_channel_refusal (keel-cli/src/main.rs) accepts the note because quotes_conversational_words (view/checks.rs) matches the substring and read_back_names (view/mod.rs) defers to it when nothing is quoted; the console's own tag [device <id> HMAC-verified] is text whose signature is not stored, so a typed copy is the same at the gate - the AI has recorded a human judgment, EHZ3.";
    }
    dependency from ucaAcceptGestureIsTyped to ehz3;

    part ucaAcceptThenTextEdited : UnsafeControlAction {
        :>> id = "5360d481-9b7d-4c0b-9ffb-3b34b50fbf31";
        :>> title = "UCA-AC2: an acceptance PROVIDED and the Decision's text edited afterwards";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::wrongTimingOrder;
        :>> context = "cmdAccept recorded at SHA s, then the Decision's decision or rationale field edited at a later commit with no re-binding.";
        :>> statement = "The human's word stands on text they did not see - EHZ3 - and the accepted Decision reads as authority for a clause nobody accepted.";
    }
    dependency from ucaAcceptThenTextEdited to ehz3;

    part ucaOverrideTargetIsWide : UnsafeControlAction {
        :>> id = "5c68a5ae-f7a7-41dd-8b90-af9870293bac";
        :>> title = "UCA-O1: an override PROVIDED with a target that names no single file";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "cmdOverride with a directory, a path fragment or a short string as the target, followed within 900 s by a Write or Edit to any path containing it.";
        :>> statement = "consume_override (keel-cli/src/main.rs) matches written_path.contains(target) || target.contains(written_path), so one unlock covers every path under the fragment and the protected write the agent chooses lands outside the validated channel - EHZ2.";
    }
    dependency from ucaOverrideTargetIsWide to ehz2;

    part ucaDirectionRenderedNotRecorded : UnsafeControlAction {
        :>> id = "9234b22a-36b2-40b2-9b61-6b4668e448b3";
        :>> title = "UCA-H1: a direction PROVIDED in chat and carried into a Decision as the agent's rendering of it";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "humanDirects: the human states a direction in the conversation; the agent records a Decision or a task DoD whose text says the human's words and quotes or paraphrases them, with no Statement recorded and no DerivedFrom edge.";
        :>> statement = "The tree attributes words to the human that only the agent's transcript holds; the computed control structure already marks humanDirects as a channel nothing parses, and nothing downstream can check the rendering against a record - an actor is misattributed, EHZ3.";
    }
    dependency from ucaDirectionRenderedNotRecorded to ehz3;

    part ccGestureIsTheCommandsReceipt : ControllerConstraint {
        :>> id = "4d6ab611-69cf-44a9-acca-a7314e76dd42";
        :>> title = "CC-AC1: a gesture citation is written by the command that observed it and re-verifiable from the tree";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> statement = "A console or deck acceptance stores the signature (or a receipt the paired-device store resolves) beside its tag; a TTY citation is written only by the command at a real terminal; a note that merely contains a gesture word from an agent session is refused with the remedy named. No control does this today - bound to issue426 (resolver dcGestureCitationsAreWrittenByTheCommand).";
    }
    dependency from ccGestureIsTheCommandsReceipt to ucaAcceptGestureIsTyped;
    dependency from ccGestureIsTheCommandsReceipt to issue426;

    part ccAcceptanceBindsToText : ControllerConstraint {
        :>> id = "98bc7619-2b57-4977-b86d-c542573b80b6";
        :>> title = "CC-AC2: an acceptance binds to the text it accepted";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> statement = "An accepted Decision whose signed fields differ from their text at the acceptance's SHA fails the gate until re-bound by a new acceptance against the current text (D0308/D0329); enforced by acceptance-binds-to-text.";
    }
    dependency from ccAcceptanceBindsToText to ucaAcceptThenTextEdited;
    dependency from ccAcceptanceBindsToText to gAcceptanceBindsToText;

    part ccOverrideNamesOneFile : ControllerConstraint {
        :>> id = "8c22c20b-dd24-4eda-9b38-24171e68a428";
        :>> title = "CC-O1: an override unlock covers exactly one file, matched by canonical path";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> statement = "keel override refuses a target that is not one file and consume_override matches by canonical equality; the single-use consumption and the recorded obligation stand as today. No control does this today - bound to issue427 (resolver dcOverrideBindsToOneFile).";
    }
    dependency from ccOverrideNamesOneFile to ucaOverrideTargetIsWide;
    dependency from ccOverrideNamesOneFile to issue427;

    part ccDirectionLinksItsStatement : ControllerConstraint {
        :>> id = "0868826c-8427-4d23-9716-08338d4a0969";
        :>> title = "CC-H1: a Decision or DoD that quotes the human links a Statement holding their words verbatim";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> statement = "A Decision or task DoD that cites the human's words carries a DerivedFrom edge to a Statement (keel record statement, D0216/D0236) whose text contains the span verbatim, forward-only from the guard's date with the existing ones grandfathered and counted. No control does this today - keel hardening reports unsourced items and gates nothing - bound to issue428 (resolver dcDirectionDecisionsLinkTheirStatement).";
    }
    dependency from ccDirectionLinksItsStatement to ucaDirectionRenderedNotRecorded;
    dependency from ccDirectionLinksItsStatement to issue428;

    verification stpaRun2 : Test {
        :>> id = "432b1153-2f8f-4932-9ce0-322accf471b0";
        :>> title = "stpa-self run 2 (2026-09-09) - the human's authority reaching the model: cmdAccept, cmdReject, cmdOverride, humanDecidesOnChannel, consoleApprovesWrite, humanDirects";
        :>> createdAt = "2026-09-09"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "ANALYSED: cmdAccept, cmdReject, cmdOverride, humanDecidesOnChannel, consoleApprovesWrite, humanDirects. FRAME: EHZ1-EHZ9 as recorded in engine-safety.sysml; no hazard added; the tranche is the three edges human->model, human->work and console->model, walked in full. CONSIDERED SAFE: cmdAccept notProvided (a proposed Decision stays proposed and blocks what it charters - the honest default, D0098); cmdAccept provided from the human's own terminal (the TTY is the gesture and the command cites it, D0315 - covered by UCA-AC1 only in its asserted stand-in); cmdAccept provided on a fork without naming the option (read_back_names refuses a note that names neither the id, an option letter nor three words of the text - a control inside the command, exercised by its tests); cmdReject provided or wrongTiming (reject_decision_locked refuses a Decision with no proposed status and writes one fail result under the file lock; the channel gap is UCA-AC1, shared with accept); cmdReject notProvided (a Decision the human wanted refused stays proposed - no hazard); cmdOverride notProvided (the protected write is refused, the honest default); cmdOverride stoppedTooSoonAppliedTooLong (an unlock is single-use, expires after 900 s and its consumption is recorded as an obligation - ctlOverrideObligation); consoleApprovesWrite provided against a changed plan (the approval is bound to the plan hash and refused when the route changed, srServeApproveGateHuman) and provided from an unpaired or altered device (HMAC-SHA256 over a pairing code, refused with nothing written, D0334/D0335); consoleApprovesWrite notProvided (the write waits); humanDecidesOnChannel in all four types (channel_wired is false - the decision channel is disconnected and the structure records the action as dormant; when a channel is wired, the decider list in github-actors.toml is the control to analyse, and that wiring is a process change that re-opens this action); humanDirects notProvided and wrongTimingOrder (an undirected or late-directed agent follows the ranked frontier, D0052 - no hazard). FOUR UCAs recorded, four constraints: one bound to a control (gAcceptanceBindsToText), three bound to Issues naming gaps (issue426 new - a gesture citation is typed text; issue427 new - an override target is a substring; issue428 new - direction is rendered, not recorded). NOT ANALYSED: the remaining computed actions, grouped by edge, are named by the stpa-currency guard until a run covers them.";
    }
    // RAN: keel show control-structure . read 2026-09-09 (48 local actions); the six actions of the human->model, human->work and console->model edges walked through all four UCA types against keel-cli/src/main.rs verdict_channel_refusal, tty_gesture and consume_override, view/checks.rs quotes_conversational_words, view/mod.rs read_back_names, write.rs reject_decision_locked, serve.rs device tag and plan-bound approval, view/control_structure.rs channel_wired; four UCAs, four constraints, every constraint edged to a control or an Issue; keel validate . clean
    part stpaRun2R1 : TestResult { :>> id = "d909e4ab-84e6-4305-b797-ed6d952a580e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e9c5a83f27566cdc0a2a78aa9d9e85c52847019e"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "claudeOpus5"; }

    // ── run 3 (stpa3, first under stpa2bCensus): the agent editing the deliverable - agentEditsDeliverable; ──
    // ── the agent->deliverable edge walked in full. keel mediates none of the edit itself (issue394); ──
    // ── what stands around it is the keystone at commit, the pre-bash hook, drift feedback afterwards. ──

    part ucaDeliverableEditWeakensControl : UnsafeControlAction {
        :>> id = "e0de5571-54c9-4309-9a6d-5f4bca12e976";
        :>> title = "UCA-ED1: agentEditsDeliverable PROVIDED on the enforcement surface with no marked Decision in the same commit";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "agentEditsDeliverable on keel-cli/src/guards.rs, adherence.rs, .githooks/ or .github/workflows/*.yml, committed with no #ProspectiveChange or #SafetyChange Decision staged beside it.";
        :>> statement = "The monitored edits the monitor: a guard's severity, a hook's command or a CI job can be relaxed by the actor it gates, and nothing in the edit itself records that a control changed - EHZ7. The keystone (guards.rs keystone_violations, line 1165) reads the marker from decision files STAGED in the commit, and CI's audit-adherence re-derives guard-set monotonicity from the tree, so the edit is refused at commit and again at push; issue236 is the code-read finding that the surface was modifiable by the monitored before the keystone covered it.";
    }
    dependency from ucaDeliverableEditWeakensControl to ehz7;

    part ucaDeliverableEditRewrittenInTransit : UnsafeControlAction {
        :>> id = "0bd969ef-8b89-456d-9b67-2bce16cc312a";
        :>> title = "UCA-ED2: agentEditsDeliverable PROVIDED through a shell heredoc whose body carries a backslash";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::provided;
        :>> context = "The agent writes source (Rust with escapes, a regex, a Windows path) through a Bash heredoc; the harness collapses a backslash pair before bash runs, even inside a quoted heredoc.";
        :>> statement = "The file that lands is not the file the agent authored and the tool reports success, so a test run against it verifies something nobody wrote - EHZ4. It broke Rust files with literal newlines eight times in two days (issue372) while every retro said the item was already tracked; the pre-bash hook now DENIES the shape (D0309, control heredoc-backslash - the ledger holds its blocks, all on agent acts).";
    }
    dependency from ucaDeliverableEditRewrittenInTransit to ehz4;

    part ucaDeliverableEditLandsPartially : UnsafeControlAction {
        :>> id = "1e236e72-f038-4314-ab91-1668a1aaf146";
        :>> title = "UCA-ED3: agentEditsDeliverable STOPPED TOO SOON - a patch whose anchor misses lands nowhere, or after the serialisation, and exits zero";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> ucaType = UcaType::stoppedTooSoonAppliedTooLong;
        :>> context = "A scripted edit finds its anchor by string; the anchor occurs zero times or more than once; the script falls through to an append or a no-op and exits 0.";
        :>> statement = "The computation is present in the file and never runs, or is absent and nothing said so - the receipt of the edit is the script's exit code, which is clean - EHZ4. Three times in one session (issue398, D0386); scripts/textpatch.py now refuses a zero or ambiguous anchor and writes nothing, but it is a tool the agent chooses, not a control at any boundary: a bare str.replace still lands nowhere silently, and no census control counts it.";
    }
    dependency from ucaDeliverableEditLandsPartially to ehz4;

    part ccEnforcementEditCarriesItsDecision : ControllerConstraint {
        :>> id = "6d506998-e60b-4ad7-9248-3edbb1fbf8a5";
        :>> title = "CC-ED1: an edit to the enforcement surface is co-committed with the marked Decision that governs it, and CI re-derives that no control weakened without one";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "A commit touching guard source, hook config or a CI workflow carries a #ProspectiveChange or #SafetyChange Decision staged in the same commit (guard process-change, D0209 clause 2), and CI fails any push where guard-set or severity monotonicity broke without a signed Decision (ctlAuditAdherence). Bound to both; the census classes process-change evidenced (issue024) and ctlAuditAdherence hypothetical - no block and no incident - which is what a control that has never had to fire looks like from the ledger.";
    }
    dependency from ccEnforcementEditCarriesItsDecision to ucaDeliverableEditWeakensControl;
    dependency from ccEnforcementEditCarriesItsDecision to gProcessChange;
    dependency from ccEnforcementEditCarriesItsDecision to ctlAuditAdherence;
    dependency from ccEnforcementEditCarriesItsDecision to issue236;

    part ccSourceEditGoesThroughTheWriteTool : ControllerConstraint {
        :>> id = "d996de90-3f98-4b72-9811-afd5cb2355aa";
        :>> title = "CC-ED2: a heredoc body carrying a backslash is refused before bash runs; source goes through the Write tool and runs by path";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "hook pre-bash denies a heredoc whose body carries a backslash in every profile (D0309) - the one deny in an otherwise advisory hook. Bound to the hook rule heredoc-backslash, declared in control-map.sysml by this run so the census's ledger row and the control map name one control, and to issue372, the incident that recurred eight times before the deny existed.";
    }
    dependency from ccSourceEditGoesThroughTheWriteTool to ucaDeliverableEditRewrittenInTransit;
    dependency from ccSourceEditGoesThroughTheWriteTool to ctlHeredocBackslash;
    dependency from ccSourceEditGoesThroughTheWriteTool to issue372;

    part ccPatchFailsWhenItsAnchorMisses : ControllerConstraint {
        :>> id = "368e596d-fea7-483a-94ca-813a070fac1d";
        :>> title = "CC-ED3: an anchored edit that finds its anchor zero times or more than once writes nothing and fails";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "scripts/textpatch.py holds the rule (replace_once, insert_after, insert_before; --probe is its known-positive and known-negative case, D0386/D0388), and no boundary enforces the choice of it. Bound to issue398 as the named gap: the control that would make this a constraint rather than a habit - pre-bash seeing a bare str.replace over a source path - does not exist.";
    }
    dependency from ccPatchFailsWhenItsAnchorMisses to ucaDeliverableEditLandsPartially;
    dependency from ccPatchFailsWhenItsAnchorMisses to issue398;

    verification stpaRun3 : Test {
        :>> id = "a2b0c755-e43c-4910-9558-87f3b68f6c09";
        :>> title = "stpa-self run 3 (2026-09-10) - the agent editing the deliverable: agentEditsDeliverable; first run under stpa2bCensus";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "ANALYSED: agentEditsDeliverable. FRAME: EHZ1-EHZ9 as recorded in engine-safety.sysml; no hazard added; the tranche is the one edge agent->deliverable, an action keel mediates at no point (issue394) - the structure has of it only the feedback deliverableDrift. CENSUS READ FIRST (stpa2b, D0426/D0428): 76 controls, 2 friction (accept:delegated-words, tap:unsigned - both dissolved to WARN lines, both binding the human), 64 hypothetical, 10 evidenced; heredoc-backslash was an evidenced hook rule with 4 blocks on agent acts that no constraint could bind because the control map did not declare it - this run declares it (ctlHeredocBackslash). CONSIDERED SAFE: agentEditsDeliverable notProvided (undone work stays ready on the frontier - the honest default, D0098); provided on the generated .claude/ surface instead of .engine/skills (done in this very session on stpa-self/SKILL.md: the claude-surface-drift guard reports the drift at the turn boundary and at commit, and sync-claude regenerates - gClaudeSurfaceDrift, exercised today); provided on source a done task's pass verified (deliverableDrift makes the task suspect in orient and ctlReverify re-stamps at HEAD - feedback, not refusal, and the honest one); wrongTimingOrder as an edit landing AFTER the pass that claims it (the unsafe act is the record, not the edit - ucaResultBoundToTheWrongTree, run 1, observed via issue375); wrongTimingOrder as a rebuild of the binary a running command holds (issue150/issue386: cargo fails loudly, keel suite refuses before it starts, the post-commit hook lands from its own copy, D0422 - no hazard); appliedTooLong as an edit session spanning a context compaction (the tree is the truth the gates read; a patch re-applied from a summary is refused by textpatch on a zero-count anchor). THREE UCAs recorded, three constraints: one bound to two controls (gProcessChange, ctlAuditAdherence) and issue236; one bound to a control declared by this run (ctlHeredocBackslash) and issue372; one bound to an Issue alone naming the gap (issue398 - anchor discipline is a tool, not a boundary). EVIDENCE: 5 observed, 11 code-read (ucaSummary at this tree: observed = ucaLandWithoutSuite/issue360, ucaResultBoundToTheWrongTree/issue375, ucaResultReceiptIsTestimony/issue374, ucaDeliverableEditRewrittenInTransit/issue372, ucaDeliverableEditLandsPartially/issue398 - the last two flagged discoveredInField by this run because each title reports an act that happened eight and three times; the 11 code-read are every other UCA, this run's UCA-ED1 among them). REMOVAL CANDIDATES: (no constraint) accept:delegated-words (friction; refused a correct act in issue397); (no constraint) tap:unsigned (friction; refused a correct act in issue447); ucaAcceptThenTextEdited -> gAcceptanceBindsToText (hypothetical); ucaRecordAcceptedBeforeFinal -> gAcceptanceBindsToText (hypothetical); ucaLandBeforeGate -> ctlPrePushBehind (hypothetical); ucaLandMergedTreeUngated -> ctlPrePushBehind (hypothetical); ucaResultHumanWordRecordedByAi -> gAttestationAuthority, gConfirmationAuthenticity (hypothetical); ucaDeliverableEditWeakensControl -> ctlAuditAdherence (hypothetical). The two friction rows are the human's ask (st115, 2026-09-10: guards that gatekeep the human and have not been abused should be reconsidered or dissolved), and every constraint-level candidate stands on a hypothetical control - a control with no incident and no block, which is not the same as a control that gatekeeps a person. NOT ANALYSED: the remaining 38 computed actions, grouped by edge, named by the stpa-currency guard until a run covers them.";
    }
    // RAN: keel show control-structure . read 2026-09-10 (48 local actions; stpa-currency NEXT TRANCHE: agent->deliverable); keel show control-census . read before any UCA was written (76 controls / 2 friction / 64 hypothetical / 10 evidenced; heredoc-backslash undeclared) and again after the records landed (ucaSummary observed 5, codeRead 11, 9 removal candidates, copied above - ucaResultHumanWordRecordedByAi is two rows, one per control); agentEditsDeliverable walked through all four UCA types against keel-cli/src/guards.rs keystone_violations, keel-cli/src/view/control_structure.rs agentEditsDeliverable, scripts/textpatch.py, the pre-bash heredoc deny (D0309) and the claude-surface-drift guard; three UCAs, three constraints, every constraint edged to a control or an Issue; keel validate . clean
    part stpaRun3R1 : TestResult { :>> id = "6a1c2e8f-9b4d-4f7a-8c3e-2d5b7a9f1c4e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7674e12e56a2886094d0849917d7e29bfc9e8627"; :>> judgedAt = "2026-09-10"; :>> judgedBy = "claudeOpus5"; }
}
