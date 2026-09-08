// D0386 (status: the `status` field below is the truth) — patchesRefuseAMiss: a text patch is made through scripts/textpatch.py, which refuses an absent or ambiguous anchor and makes an append a named operation
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0386Accept event on sign-off.
package Decision0386 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: issue398, 2026-09-07: a patch adding a computation section to facts.py fell through to an end-of-file append when its anchor did not match, landing after the serialisation, so the script exited zero and omitted every fact the section computed. The same shape three times in one session. On 2026-09-08 the scratch patches for the brief and for guards 66/67 used `assert s.count(anchor) == 1` by hand in three scripts and a bare `s.replace` in a fourth - the convention held where it was remembered and lapsed where it was not, in one day.
    part d0386 : Decision {
        :>> id = "f0ca4910-f5df-46c5-ab27-04e59321821b";
        :>> title = "patchesRefuseAMiss: a text patch is made through scripts/textpatch.py, which refuses an absent or ambiguous anchor and makes an append a named operation";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "dcPatchesFailWhenTheyMiss asks whether the fix is a shared helper the scratch scripts import or a documented rule with an example, and requires the choice recorded with its reason. A rule nobody imports decays: the 2026-09-08 patches show the hand-written assertion present in three scripts and absent in the fourth. A helper nobody is told about is not found: scratch scripts are written fresh each session from what CLAUDE.md and the skills say.";
        :>> decision = "NOT A FORK: the item posed helper-or-rule, and each fails alone in a way the other covers, so the answer is the helper named by the rule. A text patch to a file in this repository is made through scripts/textpatch.py - replace_once, insert_after, insert_before, or the separately named append - which refuses (exit 1, nothing written, the target untouched) when the anchor occurs zero times or more than once; CLAUDE.md section 6 names it as the shape, and `python scripts/textpatch.py --probe` is the helper's own known-positive and known-negative cases, run before it is trusted.";
        :>> rationale = "An append reached by a failed search is a plausible-looking success; a refusal names the problem in one line at the moment it occurs. Making the refusal the only shape the helper offers removes the decision from each scratch script, where it was made correctly three times in four. The rule in CLAUDE.md is what makes the helper findable by a fresh session; the probe is what makes the helper itself trusted rather than assumed, per dcAdHocChecksAreProbedFirst.";
        :>> consequences = "scripts/textpatch.py exists with its probe; CLAUDE.md section 6 names it beside the Write-tool rule; the pre-bash hook is unchanged - a bare str.replace in a heredoc is still possible and this Decision does not pretend otherwise. The probe runs by hand today because no python check runs in CI; that gap is a retro item of sprint 612, not a claim this Decision makes.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0386Accept : Test { :>> id = "6ca0e4f0-1725-4eb1-96b7-064869677495"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0386AcceptR1 : TestResult { :>> id = "2cbda66b-1ea5-4610-908c-4a7500d007dd"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4b85cde"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
