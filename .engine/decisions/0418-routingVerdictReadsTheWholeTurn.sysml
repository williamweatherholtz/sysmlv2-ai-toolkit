// D0418 (status: the `status` field below is the truth) — routingVerdictReadsTheWholeTurn: the routing rig scores every Skill call of a turn - intake before the intended skill is a declared prefix, a different skill anywhere is wrong, and the scorer is probed before a model is paid for
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0418Accept event on sign-off.
package Decision0418 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0418 : Decision {
        :>> id = "33408f21-f120-48eb-812f-b55ab894ea0f";
        :>> title = "routingVerdictReadsTheWholeTurn: the routing rig scores every Skill call of a turn - intake before the intended skill is a declared prefix, a different skill anywhere is wrong, and the scorer is probed before a model is paid for";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The rig's first investigation (sprint609, 2026-09-08) read the release case as 1 routed / 2 wrong, and both wrong samples had invoked intake first. Intake's deployed description declares that position (.claude/skills/intake/SKILL.md:3: use at the START of any turn where the human states direction) and every case in routing_prompts.toml is a person stating direction, so the first-call criterion was scoring the process as written as a wrong route (issue415). The same reading found the rig unlinking its last receipt before the estimate read it, so every estimate quoted the 2026-09-06 constant and every dry run erased the previous investigation (issue431).";
        :>> decision = "NOT A FORK: the verdict rule follows from intake's own declaration. routing_probe.py scores a sample through a pure function over every Skill call in order: routed when a skill in the case's accept list appears and every call before it is a declared prefix (recorded as `prefix`, printed beside the verdict); wrong when a skill that is neither accepted nor a declared prefix appears anywhere, whether or not the intended skill follows; none when the intended skill never appears. The declared prefix set is intake alone, because no other skill declares a start-of-turn position. `--probe` runs the scorer on its known transcripts (D0388) and an investigation refuses to start when one fails. The previous receipt is removed only once a paid run begins, after the estimate has read it and after a dry run has returned.";
        :>> rationale = "An agent that runs intake and then the intended skill is following the process exactly as written; a rig that scores that wrong is measuring intake, not the skill under investigation, and any before/after reading of a skill rewrite would have been reading intake's behaviour. A different skill first stays wrong because nothing declares it may run first. The scorer is pure and probed so the rule is checked on the four transcripts it must read correctly before any model call is spent on it, which is the D0388 discipline applied to a check that costs money to run.";
        :>> consequences = "The release re-run under this rule (3 samples, 3 turns, $3.71) read 3/3 none with prefix intake and tool calls to the last turn in every sample: the samples agree on a reason they show - the declared prefix spends the budget - which is recorded as issue432 with its resolver, and no rate is stated. Sample records carry `prefix`; the printed line shows it. The dry-run estimate now quotes the last paid run ($1.24 / 47 s on 2026-09-09) instead of the constant. No guard, hook, skill or process changes.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0418Accept : Test { :>> id = "44f27d78-1ee7-443c-b6fa-62a9ecf98a9c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0418AcceptR1 : TestResult { :>> id = "a6998f3c-f1e2-4f90-993e-d00c91e42506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f848923"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
