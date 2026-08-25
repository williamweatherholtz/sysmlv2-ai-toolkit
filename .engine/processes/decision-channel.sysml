// decisionChannel (D0205/D0207) — decisions travel to the human as GitHub issues; non-forks
// auto-accept under standing consent; forks earn the ask; every gesture gets a receipt.
// PORTABLE BY DESIGN: adopt with `keel activate decision-channel` + the SKILL.md install steps
// (workflow files, actor mapping, labels). The unit owns the judgment-request-quality guard.
package EngineDecisionChannel {
    private import EngineElement::*;
    private import EngineProcess::*;
    private import EngineGuardConstraints::*;

    action decisionChannel : Process {
        :>> id = "c7c8c5c3-6598-4160-a479-9cc5ecfd28f9";
        :>> title = "Decision channel (issue, auto-accept or ask, receipt, override)";
        :>> purpose = "Carry every proposed Decision to the human with zero self-hosted components and receipts at every step, while conscripting none of their attention (D0204): a non-fork decision auto-accepts under their standing consent (D0207) with the issue as notification and a forever-open override thread; a fork - where accepting means choosing - reaches out for one letter, and must first pass the judgment-request-quality bar (strong short name, rationale, alternatives with implications, a RESEARCH statement). The gesture surface and the done-signal are one thread; silence within budget is itself reported.";
        :>> cadence = Cadence::perItem;
        // APPLIES-WHEN: the person who decides is not the person doing the work, or is not present when the decisions are made, so a choice would otherwise wait on catching them
        // D0207 clause 3: the unit owns the guard that prices the ask.
        assert constraint enforcesJudgmentRequestQuality : judgmentRequestQuality;
    }

    action dchPropose : ProcessStep {
        :>> id = "70217e70-fa2a-4f37-9f00-443a0269f2fd";
        :>> title = "Propose - and pay the quality bar before any fork may ask";
        :>> triggerCondition = "A direction needs recording (CHANGE route) or a choice genuinely belongs to the human.";
        :>> actionText = "Record the Decision with `keel record decision` (marked if it governs process/schema). A NON-FORK needs a short-name-led title and honest context - it will auto-accept, and the record is what the human reads if they ever open the thread. A FORK must carry, before it can even be proposed (guard-enforced): a strong short name leading the title; substantive rationale; every OPTION with its COST (alternatives with implications); and a RESEARCH statement - what was looked at before asking, or 'none found, and here is where I looked'. If you cannot fill those, you are not ready to ask.";
        :>> producedArtifact = "A proposed Decision file that passes judgment-request-quality.";
        :>> owner = Owner::ai;
    }
    action dchPublish : ProcessStep {
        :>> id = "f64a38b8-cf9f-4e8a-ace1-593e80664970";
        :>> title = "Publish - one issue per decision, carrying its own why";
        :>> triggerCondition = "The proposed Decision lands on the default branch.";
        :>> actionText = "The decision-issue workflow opens ONE GitHub issue per proposed decision (idempotent on the embedded keel-decision marker): short-name title, body rendered from the decision's own context/decision/options via `keel decision-card` (one parser, two consumers), blocks-work label (the board IS the filtered issues list - never a regenerated copy), assigned so mobile push fires. One issue per decision keeps the gesture unambiguous and the human focused - their own requirement.";
        :>> producedArtifact = "An open GitHub issue per proposed decision.";
        :>> owner = Owner::ai;
    }
    action dchResolve : ProcessStep {
        :>> id = "9a9f80a8-e145-4b6d-8a65-cc5da6def036";
        :>> title = "Resolve - auto-accept the plain, await one letter on the fork";
        :>> triggerCondition = "The issue exists.";
        :>> actionText = "NON-FORK: the same workflow immediately records the acceptance under standing consent (D0207) - judgedBy the human, the note carrying the AUTO-ACCEPTED machine token, quoting the standing words, linking the issue - then posts the receipt and closes. FORK: the issue stays open; the human replies one letter (or `accept` / `reject <why>`), from the app or by replying to the notification email; latest comment wins and every supersession is announced. The recorder is the keel BINARY in a hardened Action (allowlisted login first, body via env, strict parse, never an agent), gating the merged tree before pushing (merge-only, never rebase).";
        :>> producedArtifact = "A recorded acceptance (auto or judged) committed to the default branch.";
        :>> owner = Owner::either;
    }
    action dchReceipt : ProcessStep {
        :>> id = "2b023f1c-93fd-471b-b7d4-7c2ded409a64";
        :>> title = "Receipt - the done-signal in the same thread as the gesture";
        :>> triggerCondition = "The recording commit pushed (or failed).";
        :>> actionText = "The recorder posts a receipt comment quoting EXACTLY what entered git (decision, option, judged_by, date, commit link, receipt-for-comment idempotency key) and closes the issue - the push-notified done-signal. Every failure speaks in-thread (parse miss, gate refusal, push contention); a scheduled sweeper re-drives missed gestures. Silence within budget is a reported failure, never ambiguity.";
        :>> producedArtifact = "A receipt comment + closed issue; or an in-thread failure report.";
        :>> owner = Owner::ai;
    }
    action dchOverride : ProcessStep {
        :>> id = "eff67f11-1f1e-4742-8aa3-ad111bd61421";
        :>> title = "Override - the thread stays open to disagreement forever";
        :>> triggerCondition = "The human replies `reject <why>` or `change: ...` on any decision issue, open or closed.";
        :>> actionText = "The recorder reopens a closed issue, acknowledges in-thread, and labels needs-reversal; the superseding reversal is then recorded as a NEW fact (accepted history is never rewritten in place - supersession, D0018/D0129). Auto-acceptances are exactly as reversible as everything else; that reversibility is what makes standing consent honest.";
        :>> producedArtifact = "An acknowledged override + a recorded superseding reversal.";
        :>> owner = Owner::either;
    }

    // step order, native succession
    first dchPropose then dchPublish;
    first dchPublish then dchResolve;
    first dchResolve then dchReceipt;
    first dchReceipt then dchOverride;
}
