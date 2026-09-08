// D0388 (status: the `status` field below is the truth) — checksAreProbedFirst: a check written to answer a question is run against a known-positive and a known-negative case before its answer is stated anywhere, and a check that is kept carries those cases as its --probe
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0388Accept event on sign-off.
package Decision0388 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: scripts/probes/issue400_reproduction.py, 2026-09-08, re-ran both defective checks from issue400 against cases chosen before the tree was read. The census that scored 64 of 64 on 2026-09-06 reads doc-guard-count - a guard no file under keel-cli/tests/ names - as covered, and scores 67 of 67 on today's tree; the corrected version reads it as uncovered. The anchor comparison from 2026-09-07 reads the human controller, which carries a process model, as anchorless, and reads all 7 controllers so; the corrected comparison reads ctHuman as the human's anchor. Both probes fail the defective check on its first case. Two helpers written today, textpatch.py and artefact.py, each carry a --probe; check_templates.py carries --self-test.
    part d0388 : Decision {
        :>> id = "339bf76e-3712-4803-8e30-b17b47b9c43a";
        :>> title = "checksAreProbedFirst: a check written to answer a question is run against a known-positive and a known-negative case before its answer is stated anywhere, and a check that is kept carries those cases as its --probe";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue400: two ad-hoc checks in one session returned wrong answers on their first run and both were stated - one as a census too good to be true, one as a finding in a response before it was recomputed. Neither had been run against a case whose answer was known in advance. dcAdHocChecksAreProbedFirst asks that the probe become a step rather than a virtue, that the practice be written where a future session reads it, and that the fate of the probe cases - recorded with the check as an instrument under D0361, or discarded with it - be decided and recorded.";
        :>> decision = "NOT A FORK: the item's two fates are for two kinds of check, and each gets the one that fits. A check written to answer a question - in a response, a record, or a published page - is run first against one case known to be positive and one known to be negative, chosen before the real tree is read, and only then is its answer on the tree stated; the statement names the probe. A check that is kept becomes an instrument under D0361 and carries its cases as a --probe entry point that runs them, the way scripts/textpatch.py, scripts/artefact.py and scripts/check_templates.py --self-test do; a check that is discarded takes its cases with it, and the receipt of what it answered names the two cases it was probed on.";
        :>> rationale = "Care produced both defective checks; the missing step is mechanical and takes a minute, so it is written as a step with two named inputs rather than as an exhortation. A kept check inherits the propriety lenses as a Sensor, and its probe is the arming evidence a control is asked for under D0298 - so the cases travel with the mechanism they arm. A discarded check leaves its cases in the receipt because the receipt is the only place the answer survives, and an answer whose probe cannot be named was not probed.";
        :>> consequences = "CLAUDE.md section 4 states the step beside the verify-don't-assert rule; scripts/probes/issue400_reproduction.py reproduces both defects against their known cases and stands as the evidence that the probe catches what it is for; every kept python check in scripts/ exposes --probe or --self-test, which dcScriptProbesRunInCi will run in CI. The response contract's own clause 3 is unchanged - a claim names its check - and this Decision adds what the check itself must have been run against.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0388Accept : Test { :>> id = "1a562bd6-4039-4a08-ae9b-b86543011b9f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0388AcceptR1 : TestResult { :>> id = "decc3736-6e1f-4445-b002-969c5f2c0b41"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "82d826c"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
