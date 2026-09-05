// D0320 (status: the `status` field below is the truth) — surfaceDriftIgnoresLineEndings: the claude-surface-drift comparison normalises line endings, so a Windows checkout of an LF-written surface is not drift
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0320Accept event on sign-off.
package Decision0320 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0320 : Decision {
        :>> id = "420c0470-7fa7-4e2b-bdc3-6e313c67d6d8";
        :>> title = "surfaceDriftIgnoresLineEndings: the claude-surface-drift comparison normalises line endings, so a Windows checkout of an LF-written surface is not drift";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#46 (st083, issue351): every fresh clone on Windows failed guard claude-surface-drift on arrival. sync-claude writes LF; a checkout with core.autocrlf reads the same files back as CRLF; the output-style and per-skill comparisons were byte-equal tests, so the guard was red before the project's first commit - and a control that is always red is routed around (D0214). The plugin rendering already normalised; the other two comparisons did not.";
        :>> decision = "One comparison helper, claude_surface::same_text, compares the deployed text to the generated text with CRLF folded to LF on both sides and nothing else normalised, and every surface comparison in sync_claude - the output style, each registry skill, the plugin rendering - uses it; a changed word or a changed line count is still drift.";
        :>> rationale = "Why only line endings: they are the checkout's property, not the content's - git rewrites them per machine and the file's meaning is unchanged; anything wider (whitespace, case) would let real edits hide. Why one helper: three comparisons had already diverged (one normalised, two did not), which is how the defect existed. Arming (D0298): a unit test on same_text (CRLF either side equal, a changed word unequal, a changed line count unequal) and a check-run test on a written surface re-read as CRLF (no drift) and then with one word changed (drift).";
        :>> consequences = "A Windows clone gates green on arrival; sync-claude's write path is unchanged and still writes LF. issue351 is resolved by dcSurfaceDriftIgnoresLineEndings.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0320Accept : Test { :>> id = "3a7fce4a-1f0a-47f7-944a-1f99f8525268"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0320AcceptR1 : TestResult { :>> id = "d77b5494-56fd-4ac3-8739-e24a116cb707"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9e250e"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
