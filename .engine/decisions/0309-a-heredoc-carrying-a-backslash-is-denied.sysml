// D0309 (status: the `status` field below is the truth) — A Bash heredoc whose body carries a backslash is DENIED by the pre-bash hook in every profile - the one blocking pre-bash verdict - because this harness rewrites backslash pairs before bash runs
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0309Accept event on sign-off.
package Decision0309 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0309 : Decision {
        :>> id = "4a4320e3-83e4-4689-a4e3-3e7989720c93";
        :>> title = "A Bash heredoc whose body carries a backslash is DENIED by the pre-bash hook in every profile - the one blocking pre-bash verdict - because this harness rewrites backslash pairs before bash runs";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human, 2026-09-05: fix heredoc so this stops happening. I can't believe it's still occuring. are we not flagging issues in sprint retros? The defect: source written through a heredoc arrives rewritten - measured with cat <<'EOF': a backslash pair collapses to one even inside a quoted heredoc, so Python meant to emit a Rust escape sequence emits the character instead and Rust files gain literal newlines inside string literals. It was named as an Issue and a backlog item on 2026-09-03 (sprint 531, dcPreBashSeesHeredocSource), a memory said the same, and it recurred eight or more times in two days - each caught by a wrong test result or an unparsable file, each excused in a retro as already tracked. The pre-bash hook was advisory by design (issue094: advise, never block) and the advisory had never been built.";
        :>> decision = "keel hook pre-bash refuses (PreToolUse deny, every profile) a Bash command containing a heredoc whose body has a backslash, naming the heredoc tag and the offending line and directing the writer to the Write tool plus running the file by path. Detection is shellcheck::heredoc_with_backslash - the <<TAG, <<-TAG and quoted-tag forms, body up to the closing tag; a backslash outside a heredoc and a prose heredoc (a commit message) pass untouched. The deny is recorded in the fire-ledger.";
        :>> rationale = "WHY DENY AND NOT ADVISE: an advisory is a reminder, a memory is a reminder, an Issue was a reminder, and D0047 says a reminder is not a control; this one recurred eight times past all three, on the same author, in two days. WHY THIS SHAPE EXACTLY: the harness collapses backslash pairs - measured, not assumed - so a backslash in a heredoc body is the one thing that cannot be written correctly this way, while prose heredocs are the sanctioned way to keep governance text out of shell arguments (issue255). WHY EVERY PROFILE: the failure is the harness's, not the project's adoption tier; guided projects hit it as surely as strict ones. THE RECEIPT: the control fired on its author within one minute of being built, on a census script with regex backslashes, and the script was written by path instead.";
        :>> consequences = "dcPreBashSeesHeredocSource is delivered. The pre-bash hook now has one blocking verdict beside its advisories, and the issue094 rule reads: advisory for shell adaptation, deny for the one shape the harness itself corrupts. CLAUDE.md section 6 names the rule. The recurrence that let a tracked item sit while its defect fired eight times is issue372 and D0310, not this Decision.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0309Accept : Test { :>> id = "b7cd8d67-eb4c-4bc1-8296-5f0487dd4737"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0309AcceptR1 : TestResult { :>> id = "90770464-93a9-4587-9af2-f288ff0a1534"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6702c38"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
