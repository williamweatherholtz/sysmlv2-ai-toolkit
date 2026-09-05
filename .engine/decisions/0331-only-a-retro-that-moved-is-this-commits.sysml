// D0331 (status: the `status` field below is the truth) — retroUnchangedIsNotJudged: retro-backlog judges a staged sprint record only when its retro text differs from HEAD's
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0331Accept event on sign-off.
package Decision0331 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0331 : Decision {
        :>> id = "5a7a04c7-5ee0-4fb6-9f19-0edf0700e79e";
        :>> title = "retroUnchangedIsNotJudged: retro-backlog judges a staged sprint record only when its retro text differs from HEAD's";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Found while correcting sprint469's RAN receipts (issue283): re-staging a 2026-08-29 sprint file made retro-backlog judge its retro by the D0293 rule written afterwards - the retro records findings but this commit ADDS no tracked item - although nothing in that retro had moved. The same shape decision-amends-process hit on the first re-binding of seventeen Decisions (D0297: a staged Decision whose prose is unchanged amends nothing), and was fixed there by comparing the staged text with HEAD.";
        :>> decision = "retro-backlog skips a staged sprint record whose retro texts (retro_texts of the staged file) equal the retro texts of the file at HEAD; a sprint record new in the commit, or one whose retro moved, is judged as before.";
        :>> rationale = "A retro is this commit's to answer for only when this commit wrote it; a receipt correction or a re-binding in an old sprint file is not a retro. Comparing retro texts rather than whole files keeps the judgment on the field the guard reads. NOT A FORK: one course, the same fix D0297 made for Decisions.";
        :>> consequences = "Old sprint records can carry receipt corrections without re-litigating their retros under later rules; a retro that is edited is still judged. The first live case is sprint469 in this commit.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0331Accept : Test { :>> id = "242a1189-098f-4913-b45c-38c309329d3f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0331AcceptR1 : TestResult { :>> id = "1e120c5d-456d-4543-86cf-c6af8bc437d1"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f84ff68"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
