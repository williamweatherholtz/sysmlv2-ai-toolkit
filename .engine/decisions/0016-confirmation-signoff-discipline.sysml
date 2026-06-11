package Decision0016 {
    private import EngineWork::*;

    part d0016 : Decision {
        :>> title = "Confirmation results need explicit human sign-off; classification is visible-first";
        :>> status = "accepted";
        :>> context = "The AI recorded a method=confirmation DoD as verified from an instruction to 'do the sign-offs' - inferring attestation instead of obtaining it. The user caught it.";
        :>> decisionText = "A confirmation verification IS a recorded human attestation: record it only on the human's explicit confirmation of that specific claim (AskUserQuestion is the clean capture); never inferred from the work being done or a general instruction. Every recorded fact carries provenance: who (verifiedBy), when (verifiedAt, ISO-8601 - the attestation time is its own fact), and verifiedAtCommit. Classify-first is visible and mandatory (CLAUDE.md 3, engine-triage skill). Accepted by user 2026-06-10 (commit 5a65d94).";
        :>> consequences = "The human gate is meaningful; audit trail cannot be self-issued by the AI.";
    }
}
