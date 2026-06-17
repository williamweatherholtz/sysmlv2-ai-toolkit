package Decision0016 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0016 : Decision {
        :>> id = "62f55738-0d12-448e-a8f4-277d64c7a812";
        :>> title = "Confirmation results need explicit human sign-off; classification is visible-first";
        :>> status = DecisionStatus::accepted;
        :>> context = "The AI recorded a method=confirmation DoD as verified from an instruction to 'do the sign-offs' - inferring attestation instead of obtaining it. The user caught it.";
        :>> decision = "A confirmation verification IS a recorded human attestation: record it only on the human's explicit confirmation of that specific claim (AskUserQuestion is the clean capture), never inferred. Every recorded fact carries provenance (who/verifiedBy, when/verifiedAt ISO-8601, verifiedAtCommit), and classify-first is visible and mandatory.";
        :>> rationale = "The AI had recorded a method=confirmation DoD as verified from an instruction to 'do the sign-offs' - inferring attestation instead of obtaining it - and the user caught it; the human gate must be meaningful and the audit trail cannot be self-issued by the AI. The attestation time is its own fact. Classify-first is enforced in CLAUDE.md 3 and the engine-triage skill. Accepted by user 2026-06-10 (commit 5a65d94).";
        :>> consequences = "The human gate is meaningful; audit trail cannot be self-issued by the AI.";
    }

    verification d0016Accept : Test { :>> id = "375b74e5-9389-4165-8c9f-7cc4909a1116"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0016 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0016AcceptR1 : TestResult { :>> id = "a7d14bb4-773f-4289-878c-65ee71fe95b0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
