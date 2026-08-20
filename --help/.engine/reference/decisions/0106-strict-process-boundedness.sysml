// =====================================================================
// D0106 — strict process-boundedness. Field feedback from keel used in
// other projects: the AI must never propose or take an action that is not tied to a DEFINED process.
// The AI's role is (1) PARSE/interpret input into an enumerated, VISIBLE decomposition classifying each
// part; (2) route each part to a defined process, DEFINING a new process when none fits; (3) EXECUTE the
// process set (in parallel where possible), leveraging skills. Only STRICTLY-TRIVIAL edits use a
// fast-path — and even those are labelled TRIVIAL in the parse. HUMAN SIGN-OFF is an EXPLICIT process
// STEP (a gate), realized as a method=confirmation verification whose passing TestResult IS the
// attestation (D0016/D0066) — never ad-hoc, never inferred. Reframes D0064 (triage-first). status=proposed:
// acceptance is itself an explicit sign-off gate (dogfood). Companion to D0105 (declarative controls).
// =====================================================================
package Decision0106 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0106 : Decision {
        :>> id = "06a1b0c1-4106-42a3-ab5c-516273849706";
        :>> title = "D0106: strict process-boundedness — parse -> route/define process -> execute; sign-off is an explicit gate";
        :>> createdAt = "2026-07-02";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Big field feedback from keel used in OTHER projects (2026-07-02): AI agents were proposing free-form actions loosely 'routed' by triage but not strictly TIED to a defined process — the recurring failure of this whole session (design-in-chat: issue054/issue055; untracked pending work; a decision drifting from its proposal). D0064 makes triage-first VISIBLE but still permits 'flag anything that does not cleanly map — ask' and free-form action after routing. The human wants the AI's role narrowed to a process compiler/executor: interpret input, produce the process set to follow, and carry it out — nothing outside a process.";
        :>> decision = "The AI operates strictly process-bound: (1) PARSE every input into an enumerated, VISIBLE decomposition at the START of the response — each part labelled by kind (TRIVIAL / CHANGE / EXECUTE / RECORD / VIEW / ORIENT) with a one-line statement, e.g. 'Parsed: 1. TRIVIAL - rename process X to Y; 2. CHANGE - add test A to block ii of process Y'. (2) Route each non-trivial part to a DEFINED process; when no process fits, DEFINE a new process (a process definition is the AI's creative output, not an ad-hoc action) — that definition itself runs through the discipline. (3) EXECUTE the resulting process set, in PARALLEL where the dependency DAG allows, leveraging skills where practical. (4) HUMAN SIGN-OFF is an EXPLICIT process STEP (a declared gate), realized as a method=confirmation verification whose passing TestResult carries the attestation (D0016/D0066) — never inferred from a general instruction. (5) ONLY strictly-trivial one-off edits (a typo, a single rename, one doc line) use the fast-path, and are still LABELLED 'TRIVIAL' in the parse so the exemption is visible, not silent. No action is proposed or taken that is not tied to a process.";
        :>> rationale = "ALTERNATIVES: (i) status quo (D0064 triage routes but permits free-form action + 'ask when unmapped') — REJECTED (the escape hatch is exactly where untracked actions slip in, proven repeatedly this session). (ii) strict-everything, no trivial fast-path — REJECTED (real ceremony overhead on typos; the human chose a LABELLED fast-path). (iii) CHOSEN — parse-and-emit + strict process adherence + define-when-unmapped + sign-off-as-gate + labelled-trivial-fast-path: makes the AI a process compiler/executor, makes the exemption visible, and makes sign-off a first-class verified gate rather than an ad-hoc attestation. It reuses the existing method=confirmation TestResult mechanism (no new sign-off construct) and composes with D0105 (declarative controls) — processes/gates become declared, the AI executes them. CRITERIA: no-untracked-action x visible-classification x sign-off-as-verified-gate x preserves-momentum-on-trivia x reuses-existing-mechanisms.";
        :>> consequences = "REFRAMES D0064: triage-first becomes PARSE-and-EMIT (a visible enumerated decomposition every turn) + strict process adherence; the 'flag & ask when unmapped' step becomes 'DEFINE a process'. Sign-off = a declared method=confirmation gate step (D0016/D0066) — no new mechanism. UPON ACCEPTANCE (operationalize via strictProcessOperationalization): update CLAUDE.md Sec 3 (the parsed-output requirement + strict adherence + define-when-unmapped + sign-off-as-gate) and the engine-triage skill (emit the 'Parsed:' block; label TRIVIAL; route/define; sign-off gate); doc-sync. Composes with D0105 (declarative controls: processes/gates/rules declared, AI executes). Trivial fast-path retained but must be LABELLED. Acceptance of THIS decision is itself an explicit sign-off gate (dogfood). Relates to issue054/issue055 (design-in-chat), issue057 (downstream framing), D0059 (skills deploy processes).";
    }

    verification d0106Accept : Test { :>> id = "06a1b0c1-4106-42a3-bb5c-516273849706"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of D0106 (strict process-boundedness — parse every input into a visible enumerated decomposition labelled by kind; route each non-trivial part to a defined process, defining one when none fits; execute in parallel where the DAG allows; only strictly-trivial edits fast-path, and LABELLED; human sign-off is an explicit method=confirmation gate) via the explicit 'yes for d0106' on 2026-07-02."; }
    part d0106AcceptR1 : TestResult { :>> id = "06a1b0c1-4106-42a3-cb5c-516273849706"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4873cd3"; :>> judgedAt = "2026-07-02"; :>> judgedBy = "wweatherholtz"; }
}
