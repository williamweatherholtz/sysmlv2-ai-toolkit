// D0329 (status: the `status` field below is the truth) — rebindLandsWithItsText: acceptance-binds-to-text reads a re-binding against the commit that first carried it, so a correction and its re-binding can land in one gated commit
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0329Accept event on sign-off.
package Decision0329 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0329 : Decision {
        :>> id = "d657034f-ddde-4850-ac1d-0e40a6fce939";
        :>> title = "rebindLandsWithItsText: acceptance-binds-to-text reads a re-binding against the commit that first carried it, so a correction and its re-binding can land in one gated commit";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Found while correcting D0234's record (issue283, dcPanelRecordCorrections): guard 61 compares an accepted Decision's signed fields with their text at the LATEST acceptance's judgedAgainst. keel accept --rebind records against HEAD, and in this repository the commit gate refuses the corrected text without a re-binding and refuses the re-binding's SHA as soon as the text differs - so an editorial correction could not land at all: uncommitted, the guard read HEAD's old text against the new text; committed together, the re-binding's judgedAgainst named the tree BEFORE the edit. The only test of the flow committed the edit in a hook-less fixture first, which is the one thing the live gate forbids. The introducing-commit fallback D0308 already made for a FIRST acceptance recorded in the same commit as its file is the same shape one step later.";
        :>> decision = "When the signed fields drift from the text at the latest acceptance's judgedAgainst, the guard also reads the commit that FIRST CARRIED that acceptance result (`git log -S 'part <dec>AcceptR<n> '`): if the fields at that commit equal HEAD's, the binding holds - the re-binding landed with the text it vouches for; if the result is not committed yet, the guard WARNS that the binding is pending and is checked at the commit that lands it, rather than failing. A result that landed in a commit whose text also differs from HEAD still fails as before.";
        :>> rationale = "Why the landing commit is the right base: an acceptance recorded at the human's word and committed together with the text is the human accepting THAT text - the same reading D0308 gave the first acceptance when it shares a commit with its file, and the one the live gate forces for every later correction. Why warn rather than pass silently on an uncommitted re-binding: the guard cannot know yet which tree will carry it, and saying so keeps the pending state visible (D0136). Why a violation still stands when the landing commit's text differs too: that is the case the guard exists for - text moved after the acceptance that vouched for it. Arming (D0298): acceptance_binds_to_text.rs gains the third phase - edit plus re-binding uncommitted reads pending, committed together reads green. D0234 itself is NOT the first live case: D0108 forbids a non-owner editing its text, so its corrections are D0330 beside it, and this rule serves an owner correcting their own record.";
        :>> consequences = "A record correction and its re-binding land in one commit through the gate, for the record's owner. STATED: one extra `git log -S` per DRIFTED Decision, which is rare; the guard's happy path is unchanged.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0329Accept : Test { :>> id = "513e56d5-7273-4ef8-9bbd-77d2afa9b76e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0329AcceptR1 : TestResult { :>> id = "af684c97-e551-4813-8e22-41cc05e27d82"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f84ff68"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
