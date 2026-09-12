// D0456 (status: the `status` field below is the truth) — channelFoldProcessAndSkillRewrites: the github-intake process and skill are rewritten to the routed spellings in the D0453 fold commit
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0456Accept event on sign-off.
package Decision0456 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: Measured 2026-09-12 by the committed transform's dry run (.engine/tools/migrations/2026-09-12-channel-family-fold.py --dry-run) at ecd447e: 22 call sites across 8 files after the hand list, conservation reconciled per verb per file, re-plan 0 after --apply. On the locked surface the transform rewrote .engine/processes/github-intake.sysml lines 25 and 34 (the actionText of the pull and ingest steps), .engine/skills/github-intake/SKILL.md lines 9 and 40 and .engine/skills/github-intake/registry.sysml line 9; by hand SKILL.md line 34 (the removal path names the ingest arm of keel github). keel gate guard process-change on the staged tree read those three files as locked with no co-committed marked Decision - this Decision is that record. hardening reads dispatched 35, namedInHelp 35.
    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0456 : Decision {
        :>> id = "32e2b3b3-ba4a-4965-b6ab-d42259d89476";
        :>> title = "channelFoldProcessAndSkillRewrites: the github-intake process and skill are rewritten to the routed spellings in the D0453 fold commit";
        :>> createdAt = "2026-09-12";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0453 folds the five github verbs under keel github and states that every call site under .engine, .claude, the root docs, the Rust source and tests is rewritten in the ONE commit that removes the names. Two of those sites are the actionText of the github-intake process's steps and three are the github-intake skill and its registry, and the keystone lock (D0209 clause 2, D0070) reads a process definition and a skill as locked files that change only with a co-committed marked Decision. The human accepted D0453 by their own word on 2026-09-12, and that acceptance landed in 6821b5c, before the commit that carries out the rewrite, so the fold commit needs a marker Decision of its own naming the step it carries out.";
        :>> decision = "The fold commit rewrites the github-intake process's two actionText lines and the github-intake skill's three invocation lines from keel github-pull and keel github-ingest to keel github pull and keel github ingest, by the committed transform where the line matches keel followed by the verb and by hand for the skill's removal path, which named the ingest dispatch arm bare. Nothing else in the process or the skill changes: no step is added, removed or reordered, no gate moves, no trust rule changes, and the skill's procedure keeps every instruction with the new spelling in place of the old. NOT A FORK: D0453 chose the course; this Decision names the one step of it that the keystone lock requires to be signed in its own commit.";
        :>> rationale = "A process step whose actionText names a verb the binary lacks instructs the reader to run a command that is refused, and the github-intake skill is the path by which an issue from outside the repository becomes a fact inside it - the surface D0453 says the fold must not move anything on but the spelling. Recording this as a plan-covered step rather than re-asking the human keeps D0396's shape: they signed the plan, the plan enumerates the rewrite of every call site under .engine, and the marker still lands on the commit so the lock has a Decision to read. D0453's own rationale said the plan cover was not used for D0453 itself because d0399 reserved the channel family for the human's word; that word has now been given, and this Decision covers a step of the accepted plan, not the plan.";
        :>> consequences = "The fold commit carries this Decision beside the rewritten process and skill and the keystone lock reads it. keel sync-claude regenerates .claude/skills/github-intake from the rewritten source in the same commit. Every downstream project holding a skill that names a github verb must update engine and skills together, as D0453 states. The computed control structure derives one action per sub-verb from the github fact, so cmdGithubPull and cmdGithubIngest keep their names and the three read and tooling sub-verbs appear as write actions until the sub-verb effect is declared (issue501, dcSubVerbEffectIsDeclared).";
    }

    // D0303 layering: the .engine call-site step of the fold D0453 fixed
    #DependsOn dependency from d0456 to d0453;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0456Accept : Test { :>> id = "6c64140b-34d2-4a7c-b4d0-066b07b074cf"; :>> method = VerificationMethod::confirmation; :>> procedureText = "PLAN-COVERED under d0453 step 'Every call site under .engine, .claude, the root docs, the Rust source and tests is rewritten by the committed transform or by hand in the ONE commit that removes the names': the human signed the plan itself and this enumerated step does not re-ask (D0375 option C / D0396). The plan's judge: wweatherholtz."; }
    part d0456AcceptR1 : TestResult { :>> id = "bbd09cbf-f8a3-4862-93c7-36998aa9f9a1"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ecd447e"; :>> judgedAt = "2026-09-12"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeFable5"; }
}
