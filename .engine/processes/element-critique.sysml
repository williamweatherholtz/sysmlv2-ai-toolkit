// =====================================================================
// EngineElementCritique — antagonistic critique of TRACKED MODEL ELEMENTS as lens-tagged
// verification (D0080; implements D0079's critique-as-verification).
//
// Distinct from EngineCritiqueProcess (architectural-critique), which adversarially audits the
// ENGINE ITSELF (schema/tooling/invariants) ad-hoc and ends in CHANGE REQUEST proposals. THIS
// process adversarially VERIFIES each tracked Need / SystemRequirement / Decision (+ Architecture)
// through required adversarial LENSES, recording each critique as a `method=critique` Test +
// TestResult `#Verify`-linked to the target, by an INDEPENDENT critic. Findings become Issues
// (the D0077/D0078 #Resolves loop); coverage + staleness reuse the assurance machinery
// (keel coverage, suspect). Deployed by the element-critique skill (D0059).
// =====================================================================
package EngineElementCritique {
    private import EngineElement::*;     // Cadence / Owner / CritiqueLens / Severity / CriticKind
    private import EngineProcess::*;

    part elementCritique : Process {
        :>> id = "63940516-2c7d-4e8f-c080-516273849800";
        :>> title = "Antagonistic Element Critique";
        :>> purpose = "Adversarially verify each tracked Need / SystemRequirement / Decision / Architecture element through required adversarial lenses, by an independent critic, recording each critique as a method=critique verification + result (#Verify-linked to the target); raise findings as severity-carrying Issues (#Resolves loop); let critique coverage + staleness COMPUTE from the assurance substrate.";
        :>> cadence = Cadence::perItem;
    }

    part ecScope : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c080-516273849801";
        :>> title = "Scope — pick targets + required lenses";
        :>> order = 1;
        :>> triggerCondition = "An assurance element lacks a required-lens critique (computed gap), OR a target changed and its critique is suspect, OR critique is requested.";
        :>> actionText = "Pick the target element(s): prioritize the `keel coverage` gap set (uncovered requirements/needs first), then accepted Decisions and Architecture. The GATED required lenses per element type are the DECLARED critique policy (`.engine/contracts/critique-policy.toml`, D0097 — default Core-3; `keel critique-policy` shows the active set); lenses are drawn from the requirement-quality canon (CritiqueLens: completeness, correctness, ambiguity, testability, feasibility, consistency, necessity) and a critique may apply BROADER lenses than the gate requires (D0079). Fix the (element x lens) work-list before critiquing.";
        :>> producedArtifact = "The (element x lens) critique work-list (ephemeral input to ecCritique).";
        :>> owner = Owner::ai;
    }
    part ecCritique : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c080-516273849802";
        :>> title = "Critique — lens-tagged verification by an INDEPENDENT critic";
        :>> order = 2;
        :>> triggerCondition = "An (element x required-lens) pair has no current critique.";
        :>> actionText = "For each pair, the critic tries to BREAK the element through that lens (antagonistic, not affirming). Record a `verification <name> : Test { method = VerificationMethod::critique; lens = CritiqueLens::<lens>; critiquedBy = CriticKind::<aiModel|human|tool>; ... }` `#Verify`-linked to the target, plus a TestResult: outcome=pass (clean — element survives the lens) or fail (a finding). The result's judgedBy (the critic IDENTITY) MUST differ from the target's author (independence) and carries judgedAgainst (the commit critiqued, for staleness).";
        :>> producedArtifact = "A method=critique Test + TestResult #Verify-linked to the target (the recorded critique).";
        :>> owner = Owner::ai;
    }
    part ecFinding : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c080-516273849803";
        :>> title = "Finding — raise as a severity-carrying Issue (#Resolves loop)";
        :>> order = 3;
        :>> triggerCondition = "A critique TestResult is `fail` (the element did not survive the lens).";
        :>> actionText = "Author an `Issue { description; severity = Severity::<Critical|High|Medium|Low>; relatedTask }` for the finding and TRIAGE it via the issue-resolution loop (D0078: `#Resolves` to a resolving action or a mooting Decision). Severity is the assessed impact (D0079 taxonomy). Low findings are surfaced (not gated); the Issue still gets a resolver. Never leave a finding as prose in the critique notes only.";
        :>> producedArtifact = "An Issue (with severity) + a #Resolves edge; the finding enters the computed open-issue frontier.";
        :>> owner = Owner::ai;
    }
    part ecDisposition : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c080-516273849804";
        :>> title = "Disposition — human gate at >= Medium";
        :>> order = 4;
        :>> triggerCondition = "A finding Issue has severity >= Medium (Critical / High / Medium).";
        :>> actionText = "A human dispositions every >= Medium finding with a TYPED verdict (D0092), recorded via `keel apply-review` as a method=confirmation disposition carrying `disposition : DispositionKind` (#Dispositions-linked to the finding Issue): ACT (also add a #Resolves resolver action to do the fix), ACCEPT-RISK (the verdict closes the finding — knowingly accepted), or DISMISS (the verdict moots it). A Critical finding MUST be fixed or explicitly risk-accepted — it cannot be left open. Low findings are AI-dispositioned (surfaced, not gated). The disposition is the human's word (D0016/D0051) — never inferred. State is COMPUTED: `keel dispositions` (dispositioned vs not) + `assured` read the typed verdict, never prose.";
        :>> producedArtifact = "A typed disposition (D0092): a #Dispositions-linked method=confirmation verification carrying the DispositionKind verdict (act/acceptRisk/dismiss) + rationale + provenance; ACT additionally has a #Resolves resolver.";
        :>> owner = Owner::human;
    }
    part ecStaleness : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c080-516273849805";
        :>> title = "Staleness — re-critique on change (reuse suspect)";
        :>> order = 5;
        :>> triggerCondition = "A critiqued target changes materially after its critique commit (the critique is suspect, computed via the same git-temporal rule as verification suspicion, D0050).";
        :>> actionText = "Treat the stale critique like any suspect verification: re-run it at HEAD (a new critique TestResult) — the old pass no longer demonstrably covers the current element. Re-OPEN any accept-risk disposition whose basis changed (the human must re-attest). Do nothing by hand to 'refresh' state: critique coverage + staleness COMPUTE from the critique results + git ancestry.";
        :>> producedArtifact = "A re-critique TestResult at HEAD; re-opened dispositions where the basis changed (computed staleness, never stored).";
        :>> owner = Owner::ai;
    }
}
