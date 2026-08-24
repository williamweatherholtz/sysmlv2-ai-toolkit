// D0220 (status: the `status` field below is the truth) — skillRegistersBesideItself: a skill may declare its deployment next to itself, so an adopted unit does not land with a red gate
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0220Accept event on sign-off.
package Decision0220 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0220 : Decision {
        :>> id = "ce2300a9-00ef-4a79-aba8-c7fb67d284e0";
        :>> title = "skillRegistersBesideItself: a skill may declare its deployment next to itself, so an adopted unit does not land with a red gate";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Found by ACTUALLY adopting the decision-channel unit on asirobots/penumbra rather than testing the export in this repository. The bundle carried the process definition, the SKILL.md and the mechanism, but the entry that binds skill -> process lives in .engine/skills/skills-registry.sysml - a single shared file holding all 37 skills - and stayed home. So FAILED in penumbra on the very first run: 'process .engine/processes/decision-channel.sysml has NO deploying skill (inert process)'. A new project's first experience of an adopted unit was a red gate it could only clear by hand-editing a shared file, which is both a fork of the mechanism and precisely the friction D0054 names as the top adoption risk. The rules layer had already met this problem and solved it by carrying rules BY NAME instead of copying a shared file; skills had no equivalent.";
        :>> decision = "A skill may declare its deployment BESIDE ITSELF, in any under , and both readers of that declaration now scan the directory rather than the single central file: the guard, and . A unit therefore ships its own registration as a FILE and nothing has to be text-merged into a registry shared with three dozen other skills. decisionChannelSkill is MOVED - not copied - out of the central registry into .engine/skills/decision-channel/registry.sysml, and carries when present. The central registry remains valid and is still where a skill with no unit of its own belongs; this widens what satisfies the rule without weakening it.";
        :>> rationale = "The alternative was to have import APPEND the extracted declaration into the receiving project's shared registry - a text merge into a file that project also edits, which is the clobber hazard the rules layer deliberately avoided. Reading the directory instead makes the unit self-describing, which is the property srPortModularProcessUnit actually asks for, and it is strictly more honest: a skill's registration sitting next to the skill is at least as discoverable as one buried in a 37-entry file. It also means a downstream project can add its OWN skills as units without touching an engine-shipped file.";
        :>> consequences = "Verified end to end by re-adopting on penumbra: export now carries 7 files (definition, SKILL.md, registry.sysml, two workflows, two scripts) and after import penumbra reports validate 73 clean, process-skill 22 scanned / 0 violations, and [guard] ALL PASS - where before the same adoption left it FAILING. This repository stays green with the entry moved (process-skill 24/0). TWO defects were caught in the doing rather than assumed away: duplicate-identity refused the copy on the first edit, forcing a MOVE (the guard doing its job on its author); and moving the entry silently dropped the export from 6 files to 5, losing the skill entirely, because deploying_skills also read only the central file - found by re-checking the export rather than trusting it. Resolves issue252.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0220Accept : Test { :>> id = "6f954f3a-a75a-41b6-ac42-3f4ee7bd9480"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/18."; }
    part d0220AcceptR1 : TestResult { :>> id = "0c998b4c-9ecc-4f5c-971b-404b2b8f433d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "232c0cd"; :>> judgedAt = "2026-08-24"; :>> judgedBy = "wweatherholtz"; }
}
