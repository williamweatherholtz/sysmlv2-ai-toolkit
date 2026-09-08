// D0379 (status: the `status` field below is the truth) — skillReferencesDeploy: sync-claude deploys a skill's references/ directory beside its SKILL.md, and claude-surface-drift fails a deployed skill whose instructions name a relative reference that does not resolve
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0379Accept event on sign-off.
package Decision0379 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0379 : Decision {
        :>> id = "db9dd139-d2a5-4df9-8ebf-dce85d608828";
        :>> title = "skillReferencesDeploy: sync-claude deploys a skill's references/ directory beside its SKILL.md, and claude-surface-drift fails a deployed skill whose instructions name a relative reference that does not resolve";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "A deployed skill is a SKILL.md the agent reads where it stands, in .claude/skills/<name>/. The stpa skill's opening instruction is to read references/sop.md before running a step - the 121-line SOP carrying the four steps, the per-element data fields and the per-step completeness gates - and .claude/skills/stpa/ held only SKILL.md, because sync-claude copies exactly one file per registry entry. Following the deployed instruction produced a missing path, and an agent at that point either invents the procedure or skips the gates (issue393). The requirement-quality skill names references/ruleset.md and references/ears.md the same way. claude-surface-drift already owns whether the deployed surface matches the generator; it never asked whether the surface's own instructions are followable. NOT A FORK: the resolver's criterion names the mechanism and the home; this records the generator's contract so the change is governed, not a choice between designs.";
        :>> decision = "sync-claude deploys every file under a skill's source references/ directory to .claude/skills/<name>/references/, byte for byte, and its check reports a missing or differing reference file as drift. The same check reads every deployed SKILL.md for a relative reference token - references/<path> not preceded by a path separator - and reports each one that does not resolve under the deployed skill directory, naming the skill and the path; the count of affected skills is computed by that scan, never hand-written. A skill that names no reference is not scanned for one.";
        :>> rationale = "The deployed surface is a contract with the agent: an instruction it cannot follow is worse than no instruction, because the agent proceeds as if it had. Deploying the reference directory closes the case at hand; the resolution check closes the class, so a future skill that grows a references/ file its author forgot to deploy - or renames one - is caught by the same guard that catches a stale SKILL.md, in the same run. The check lives with claude-surface-drift rather than as a new guard because it is the same question about the same surface (one control per surface, D0105), and the generator source is not keystone-locked, so this is a generator contract change under standing consent, not an enforcement-surface change.";
        :>> consequences = "keel-cli/src/claude_surface.rs gains the reference deployment in sync_claude and an unresolved-reference scan whose findings join the drift list; a unit test constructs a skill naming a reference that is not deployed and asserts the check fails, and a skill naming none and asserts it does not fire. After sync-claude, .claude/skills/stpa/references/sop.md and .claude/skills/requirement-quality/references/{ruleset,ears}.md exist and are committed. The deployed-surface count in the drift report grows by the reference files. Resolves issue393 through dcSkillReferencesAreDeployed.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0379Accept : Test { :>> id = "9d7ad2c1-3a49-4526-b1e0-f8db79bce3cc"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0379AcceptR1 : TestResult { :>> id = "a4b40272-d18c-4891-84e8-3bbaa74a5987"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "748746a"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
