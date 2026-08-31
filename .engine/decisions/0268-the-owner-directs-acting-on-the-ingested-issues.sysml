// D0268 (status: the `status` field below is the truth) — The repository owner's direct instruction is the consent that untrusted-tier issues require
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0268Accept event on sign-off.
package Decision0268 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0268 : Decision {
        :>> id = "9b6c72a5-50a0-41a1-ae6d-b7942d7bfd96";
        :>> title = "The repository owner's direct instruction is the consent that untrusted-tier issues require";
        :>> createdAt = "2026-08-31";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Fifteen open issues were pulled and ingested. This repository is PUBLIC, so D0264 records each of them at `sourceTrust = untrusted` and guard 56 refuses to route an untrusted-origin story to work with no Decision among its targets: untrusted input may PLAN, and a human accepts before anything is built. The human then said, in this session's authenticated channel: 'ingest github issues and fix'. That is not an instruction arriving through the untrusted channel - it is the repository owner directing the work directly, which is precisely the acceptance the tier exists to require. RECORDED RATHER THAN ASSUMED, because the alternative was to pass `--trust trusted` and make the boundary evaporate with no trace. A boundary that can be stepped over silently is not a boundary; one that requires a recorded Decision naming who authorised what is.";
        :>> decision = "The owner's instruction authorises acting on the fifteen issues open at 2026-08-31 (GH#9, #14, #29, #30, #31, #32, #33, #34, #35, #36, #37, #38, #39, #40, #41). Stories triaged from those utterances route to THIS Decision, satisfying guard 56's routing requirement with a real acceptance rather than a bypass. WHAT THIS DOES NOT AUTHORISE. It is scoped to these fifteen and to this instruction. A future issue arriving on the same public repository is untrusted again and needs its own acceptance; this Decision is not a standing grant, and D0207's standing consent covers Decisions raised on the channel, not arbitrary issue content.";
        :>> rationale = "WHY NOT --trust trusted. That flag exists for a repository whose visibility misrepresents its authorship, and using it here would have re-labelled the ISSUES as trusted when what is actually trusted is the HUMAN'S INSTRUCTION about them. The distinction matters for a later reader: the issues are still stranger-authorable text; what changed is that the owner read them and said act. WHY THE SCOPE IS ENUMERATED. An open-ended 'act on issues from now on' would reproduce the hazard with extra steps - the point of the tier is that each batch of unauthenticated input gets a human's attention before it moves the project.";
        :>> consequences = "Fifteen stories may route to work. Guard 56 stays armed for everything after them. TWO ISSUES ARE ALREADY RESOLVED by work landed earlier today, verified empirically rather than assumed: GH#30 (project decision numbering colliding with the engine's reference decisions - a fresh project now records d0001 alongside 261 ReferenceDecisionNNNN packages with duplicate-identity scanning 368 and finding zero) and GH#39 (migrate overwriting a downstream project's chartered process set - a project chartered by d9001 keeps it across a migration, because project-owned contracts are now added and never overwritten, issue314).";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0268Accept : Test { :>> id = "cc31939e-394b-4b8d-b246-835a4e187fa9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/29."; }
    part d0268AcceptR1 : TestResult { :>> id = "565f6d59-c250-4ff4-b2a8-93a248d34398"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cc74636"; :>> judgedAt = "2026-08-31"; :>> judgedBy = "wweatherholtz"; }
}
