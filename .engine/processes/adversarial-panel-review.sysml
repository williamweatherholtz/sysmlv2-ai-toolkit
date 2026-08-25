// Adversarial Panel Review — a design artifact is hardened by a persona-diverse critic panel
// run iteratively to uncompelled unanimity. Governed by D0187. Precedent for the independence
// honesty note: the 2026-08-14 panel (.tracking/panel-critiques.sysml).
package ProcessAdversarialPanelReview {
    private import EngineElement::*;
    private import EngineProcess::*;

    action adversarialPanelReview : Process {
        :>> id = "8253e201-eda3-4f92-b016-d61ef0389d28";
        :>> title = "Adversarial Panel Review";
        :>> purpose = "Harden a design artifact by convening independent critic personas whose ACCEPT verdicts are conditional on named, verifiable changes, iterating artifact revisions until every panelist independently accepts - uncompelled unanimity. The panel is a rigor framing, never a claim of critic independence (same-model subagents record critiquedBy = aiModel and pass nothing by themselves).";
        :>> cadence = Cadence::adHoc;
        // APPLIES-WHEN: a choice is expensive to reverse - an architecture, a schema, a public interface, a material commitment - and you would rather it were attacked before you are committed than after
    }

    action aprConvene : ProcessStep {
        :>> id = "c7db74f0-0408-4810-bdae-ad7df93e128b";
        :>> title = "Convene the panel";
        :>> triggerCondition = "A design artifact carries enough scope or risk that a single author's judgment should not be the last word - a multi-work-package plan, a schema or process change, a direction Decision - and the human has asked for panel review or accepted its cost.";
        :>> actionText = "Scale the roster to the stakes (full form: 2 pessimists - premise and operations; 2 optimists - steelman/anti-dilution and value/ROI; 3 pedants - factual accuracy, internal consistency, mechanism failure modes; 1 reduction advocate; 1 systems engineer deriving implementation-free kernel invariants). Every prompt carries: the artifact path, the repository root for claim verification, the HONESTY RULE (the persona is a lens, not a script - no manufactured objections, no capitulation), and the output contract (numbered findings, each with severity, evidence verified against the repository, and the specific change that resolves it; a verdict of ACCEPT or OBJECT naming the minimal flip set of findings that would flip the verdict).";
        :>> producedArtifact = "A panelist roster with per-persona prompts.";
        :>> owner = Owner::ai;
    }

    action aprDispatch : ProcessStep {
        :>> id = "6f107866-905e-4a61-a08b-e79c0d21a313";
        :>> title = "Dispatch a round in parallel";
        :>> triggerCondition = "A round begins: round 1 against the draft; later rounds against a revision.";
        :>> actionText = "Dispatch all panelists concurrently as independent contexts. From round 2 onward each panelist receives the revised artifact plus a digest of every panelist's prior positions and verdicts, and is explicitly invited to rebut other panelists' resolutions - critics checking each other is where laundered premises die (round 2 of the founding run: the factual-accuracy pedant refuted the operations pessimist's central premise from the delivery record, and the pessimist verified the refutation before accepting). Panelists verify claims against the repository, never against the artifact's own assertions.";
        :>> producedArtifact = "One findings report per panelist, each ending in a verdict with its minimal flip set.";
        :>> owner = Owner::ai;
    }

    action aprRevise : ProcessStep {
        :>> id = "b1689ec0-ca77-4a15-8913-5053fd18f89f";
        :>> title = "Resolve every flip condition in a revision";
        :>> triggerCondition = "A round returns at least one OBJECT.";
        :>> actionText = "Revise the artifact to resolve every stated flip condition - in substance, not by renaming. Where panelists conflict (a reducer's cut against an optimist's wall), resolve on evidence and record the choice. Maintain a finding-to-resolution map inside the artifact so the next round verifies resolutions instead of re-litigating them. A finding that survives on evidence against the author's text wins; the author's job is synthesis, not defense.";
        :>> producedArtifact = "A revised artifact carrying its finding-to-resolution map.";
        :>> owner = Owner::ai;
    }

    action aprConverge : ProcessStep {
        :>> id = "c76732a5-67cf-46fc-8dfe-7a4ee3010017";
        :>> title = "Iterate to uncompelled unanimity";
        :>> triggerCondition = "A revision exists.";
        :>> actionText = "Repeat dispatch and revise until every panelist independently returns ACCEPT. Unanimity is UNCOMPELLED or it is nothing: a panelist may not be argued into accepting - only shown their flip conditions resolved or refuted with repository evidence. If convergence has not arrived after roughly three rounds, stop and hand the irreducible disagreements to the human rather than grinding the panel into agreement.";
        :>> producedArtifact = "A final artifact plus the round-by-round verdict record.";
        :>> owner = Owner::ai;
    }

    action aprRecord : ProcessStep {
        :>> id = "5d800adc-3ba8-4510-be9a-8b5b4696b561";
        :>> title = "Record outcomes as tracked facts";
        :>> triggerCondition = "Unanimity reached, or the human has adjudicated the residue.";
        :>> actionText = "Everything that should outlive the conversation becomes a tracked fact through the write API: surviving findings that demand work become Issues or backlog tasks; direction outcomes become proposed Decisions awaiting human acceptance; charter-time residuals are carried into the Decisions they belong to. The recorded provenance carries the independence honesty note verbatim: same-model subagent panelists are a rigor framing whose critiques record critiquedBy = aiModel - the record claims convergence, never critic independence.";
        :>> producedArtifact = "Proposed Decisions, Issues, and chartered tasks; the artifact itself referenced from them; and a KNOWLEDGE RECORD (D0196) at .tracking/knowledge/<slug>.sysml - a method=analyze Test carrying the replication block (process, process-def version pin, skill, roster, rounds, artifact paths) with per-round TestResults and plain-dependency edges to the process, the governing Decision, the sprint, and every produced item.";
        :>> owner = Owner::ai;
    }
}
