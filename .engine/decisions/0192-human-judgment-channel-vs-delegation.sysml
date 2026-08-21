// D0192 (PROPOSED — NOT YET ACCEPTED) — D0192: reconcile srK06's letter with the D0165 delegation - amend the requirement, or close the delegation path
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0192Accept event on sign-off.
package Decision0192 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0192 : Decision {
        :>> id = "5b1d2b0e-3d57-4d93-bd2d-c0f84bf546ea";
        :>> title = "D0192: reconcile srK06's letter with the D0165 delegation - amend the requirement, or close the delegation path";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "Critique pass 1 (issue204, High) found srK06HumanChannel's letter and this project's sanctioned practice in direct contradiction. The letter: records of human judgment shall originate only from a channel the agent cannot invoke. The practice: under D0165 the AI records the human's verbatim chat words through the agent-invokable HTTP acceptance endpoint with judged_by=<human> - used for D0191 this very week - because the human works by telling the AI their verdicts in conversation, and forcing every acceptance through a browser gesture would tax exactly the workflow they chose. No predicate can distinguish the human typing a name from an agent typing it (authentication, not text-checking); the attestation-policy already records the DISPOSITION delegation in its contract, while the ACCEPTANCE delegation exists only as practice. One of the two must move, and which is the human's call.";
        :>> decision = "PROPOSED, two options costed - accepting this Decision means choosing ONE: OPTION A (amend the requirement): supersede srK06 with a delegation clause - an agent may record a human's judgment when the record quotes their words verbatim, names them as judge, and cites the delegation decision; the clause is declared in attestation-policy.toml per class (mirroring the findingDisposition entry) so the guard checks a policy the project chose, and confirmation-authenticity gains a substance check that a delegated record actually quotes conversational words. COST: an agent that fabricates a quote defeats it - the protection is the human reading their own queue and the audit trail, stated plainly. OPTION B (close the path): acceptances become human-gesture-only - the deck, the console, or a TTY-checked CLI; the HTTP acceptance endpoint gains the same agent-marker channel refusal cmd_accept has; the AI queues acceptances and never records them. COST: every acceptance requires the human at a surface, and the accept-by-chat workflow they use today stops working; their 'accepted all' would become fifteen deck taps.";
        :>> rationale = "The contradiction is real and recorded, and resolving it by silently continuing the practice would leave a requirement that fails every honest examination forever - exactly the dual truth D0018 forbids. Option A matches how this human demonstrably works (verbatim-words acceptances all month) and makes the practice policy-visible like the disposition delegation already is; option B matches the requirement's original intent and the panel's K6 severity. The drafter deliberately recommends NEITHER: the trade is throughput-with-audit-trail versus channel-purity, and it is a supervision preference, not an engineering fact.";
        :>> consequences = "OPTION A: srK06 superseded (identity and history kept), one policy entry, one guard substance-check; issue204 resolves; the acceptance queue keeps working by chat. OPTION B: one endpoint refusal plus a queue surface for pending acceptances; issue204 resolves; chat acceptances end. EITHER WAY the asymmetry ends: the policy contract records ALL delegations or none. REJECTED-AS-WRITTEN is also informative: it means the contradiction stands recorded and srK06's examinations keep failing honestly.";
    }
}
