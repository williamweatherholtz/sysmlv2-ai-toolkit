// D0323 (status: the `status` field below is the truth) — externalFactGate: a ci-run receipt is a TestResult class CI verifies against the run itself - the one pass an agent cannot talk past
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0323Accept event on sign-off.
package Decision0323 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0323 : Decision {
        :>> id = "7d31f5e7-68a9-47ee-81c8-241e47232627";
        :>> title = "externalFactGate: a ci-run receipt is a TestResult class CI verifies against the run itself - the one pass an agent cannot talk past";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "A // RAN: receipt (D0232, guard 52) proves a sentence was typed, not that a run happened: on 2026-09-04 a suite count was written into a receipt while the suite was still running and only the author's read-back caught it (stpa-self run 1, UCA-A1, issue374). The landscape spike (D0282, Beads' gh:run gates) found the shape that ends this: a gate that closes only when an EXTERNAL fact exists, checked by a party the agent cannot write to. keel's append-result still records what the agent SAYS it ran; Beads' gate check is a command the agent runs; keel's check has to run in CI.";
        :>> decision = "A receipt of the form `// RAN: ci-run id=<GitHub Actions run id> workflow=<name>` is the external-fact class of TestResult, and `keel audit-ci-runs [ROOT] [--repo OWNER/NAME]` verifies every such receipt against the run it names - it exists in this repository, concluded success, ran on the result's judgedAgainst SHA, and is the named workflow - failing with the result named on a fabricated or mistyped id, a run from another repository, a run on a different tree, a non-success conclusion, or another workflow; the check runs as a gate step in ci.yml (GH_TOKEN, --repo from the workflow context) under the keystone lock, and locally as the same command when gh is available; a tree with no ci-run receipt reports zero and passes, stated.";
        :>> rationale = "Why CI is the judge: the agent's machine can write any receipt; the run record on GitHub it cannot, and a check that runs where the agent has no write is the only one whose green means something more than the agent's word - the point D0282 took from Beads. Why the verdict is a pure function: every branch (missing run, other repository, other tree, failed, other workflow, green) is unit-tested against fixture JSON without a network, so the network fetch is the only untested line and it is a gh api call. Why the receipt grammar is strict (numeric id, named workflow): a loose parse would let an ordinary receipt be read as a claim about a run it never made. Why this is the first half of D0312's option B and not its whole: it makes ONE result class externally verified; what a passing verification means by authority across the methods is the fork the human holds. Arming (D0298): ci_runs unit tests on the DoD's three cases and the three further refusals; the live first receipt lands on this Decision's own sprint result, naming CI run 33978282844 for 50175b6, so the CI step has a real receipt to check on its first run.";
        :>> consequences = "Every future TestResult may cite a run instead of a sentence; the attestation census will count them as a receipt kind. CLI facts gain audit-ci-runs (commands.sysml, CLI_FACTS, COMMAND_NAMES); ci.yml gains the step. issue374 is resolved by dcExternalFactGate; CC-A1 (ccResultReceiptIsExternal) binds to this control at the next stpa-self run. STATED RESIDUAL: a run that is green for the wrong reason (a test that passes vacuously) still vouches; the check proves the run happened on that tree and concluded success, not that the suite was sufficient.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0323Accept : Test { :>> id = "daf68c71-954e-41ac-a1b7-cf73f9b9a717"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0323AcceptR1 : TestResult { :>> id = "bd34f49a-b84e-447a-a619-1c0bcba8281f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "50175b6"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
