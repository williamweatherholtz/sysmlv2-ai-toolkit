// D0348 (status: the `status` field below is the truth) — deactivatedSkillSaysSo: sync-claude deploys a deactivated process's skill with its inactive state written first, so the agent learns the channel is closed before acting
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0348Accept event on sign-off.
package Decision0348 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0348 : Decision {
        :>> id = "4c182330-8bb3-44eb-9fa9-f8247f69e675";
        :>> title = "deactivatedSkillSaysSo: sync-claude deploys a deactivated process's skill with its inactive state written first, so the agent learns the channel is closed before acting";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#49 (st080, us085, issue355, routed under D0292; dcDeactivatedSkillSaysSo). keel deactivate turned a process's guards off and left its skill deployed unchanged under .claude/skills/, so an agent reading its skill list was still told to run a process the project had switched off. The reporter's own lean, recorded in the task: state the inactive state in the skill rather than remove the file - a removed file reads as drift (GH#46) and a stated state teaches the model.";
        :>> decision = "sync-claude deploys every registry skill as deployed_skill_text: when the skill's process (its directory name under .engine/skills/) is in the project's inactive set, the deployed SKILL.md begins with a one-paragraph INACTIVE banner naming the process, the activation contract, that its guards do not run and the skill is not to be followed, and the keel activate command that re-arms it; the source skill is unchanged below it; the drift check compares against the same deployed text, so an inactive skill is not drift and reactivating clears the banner on the next sync. NOT A FORK: removal was weighed in the task and set aside for the reason above; this records the one shape.";
        :>> rationale = "Why first, not last: the agent reads a skill's opening to decide whether to invoke it; a state written at the end is read after the decision. Why the deployed text and not the source: the source is the engine's content and travels to every project; the inactive state is this project's fact. Arming (D0298): a binary test deactivates render on a scaffold, syncs, asserts the deployed skill starts with the banner and names the process and the re-arm command, asserts sync-claude --check is clean while inactive, reactivates, syncs, and asserts the banner is gone and the check clean.";
        :>> consequences = "A project that switches a process off has its skill say so at the top; this repository deactivates nothing today, so its surface is unchanged. dcDeactivatedSkillSaysSo is done on the test above.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0348Accept : Test { :>> id = "7bb46a4c-0c7b-411d-8daf-97230938f9f0"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0348AcceptR1 : TestResult { :>> id = "a8fba1e1-a4af-4ea4-a82c-1ab128a32b82"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a66324c"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
