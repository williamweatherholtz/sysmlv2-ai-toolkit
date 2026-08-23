// D0198 (status: the `status` field below is the truth) — D0198: extend the D0192 recording delegation to confirmation flips - companion record, or an attestation field on TestResult
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0198Accept event on sign-off.
package Decision0198 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    // APPLIED sprint 430 (2026-08-23), OPTION A per their deck tap: [confirmationRecord] policy
    // class, delegatedConfirmationSubstanceRule + quote-receipt predicate, guard sourcing, docs.
    #ProspectiveChange part d0198 : Decision {
        :>> id = "3eeb62cd-cc0b-4ba2-b0a6-537603bed20a";
        :>> title = "D0198: extend the D0192 recording delegation to confirmation flips - companion record, or an attestation field on TestResult";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue224 (srK06a completeness critique, sprint 421): D0192 OPTION A declared and substance-checks the recording delegation for decisionAcceptance, but the identical practice flips method=confirmation DoD results on the human's chat words - sprint 420's dcHumanChannelDelegationRuleDoD flip is the live example - and no [confirmationRecord] policy class or quote demand exists there. The evidence shape differs: an acceptance event is a Test whose procedureText IS the attestation, while a DoD flip appends a TestResult that carries only outcome/judgedAgainst/judgedAt/judgedBy - there is nowhere for the quote to live today, so extending the substance check is a design question, not a copy.";
        :>> decision = "PROPOSED, two options costed - accepting means choosing ONE. OPTION A (companion attestation record, no schema change): a delegated confirmation flip requires a sibling Test named <test>Attest<N> with method=confirmation whose procedureText quotes the human's words, plus its own passing result; the substance rule extends to demand the companion for human-judged confirmation results after the cutoff. COST: two records per flip, a naming convention the guard must key on, and the attestation lives beside rather than on the result it evidences. OPTION B (schema: TestResult gains an OPTIONAL attestationText attribute): the quote lives on the result itself; keel append-result gains --note; the substance rule reads the field directly. COST: a frozen schema/core amendment (invariant 5 - this Decision is that CR), though migration is nil because the field is optional and forward-only. DRAFTER RECOMMENDS B: one home per fact (D0105), less per-use friction (D0054 - one flag on the existing command versus a second authored record), and the attestation rides the exact result it attests.";
        :>> rationale = "Either option ends the asymmetry issue224 names; they differ in where the fact lives and what it costs to record. The D0192 precedent applies unchanged: the check is text-shape, not authentication, and the stated protections are the human's own queue and the audit trail.";
        :>> consequences = "OPTION A: rules + guard extension only; validate untouched. OPTION B: one optional attribute in schema/core/verification (explicit human sign-off on the schema edit per invariant 5), append-result --note, guard extension; existing TestResults are untouched and grandfathered by the cutoff. EITHER WAY: attestation-policy.toml gains [confirmationRecord] with delegatedRecording, so the policy contract names every practiced delegation class; issue224 resolves.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0198Accept : Test { :>> id = "54b8a449-2f07-4375-89ea-ceb009968efc"; :>> method = VerificationMethod::confirmation; :>> procedureText = "OPTION A chosen (companion attestation record, no schema change). Deck tap: uid 3eeb62cd, verdict accept, note OPTION A, At 2026-08-23T01:15:18Z, deck HEAD 55d1381 - a single deliberate option tap, superseding their earlier maybe (22:12). The fork-option pipeline carrying the choice is the sprint-419 mechanism."; }
    part d0198AcceptR1 : TestResult { :>> id = "d5ec40b6-0cbf-4438-8cfa-4ad5892c9ea6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71f6453"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
