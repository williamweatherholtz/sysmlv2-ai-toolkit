// =====================================================================
// Sitting 04 review — the per-sitting human gate (D0049/D0073), and the FIRST review taken against the
// live queue D0155 created: sprints 318 and 319 are the two sittings that postdate its grandfather line,
// so they are the first two items the queue has ever asked for. Confirmation TestResult written ONLY on
// the human's explicit attestation (D0016/D0051 — never inferred) and substantive per issue083.
//
// #Covers edges are what make this review VISIBLE to `keel sitting-coverage`. The sitting01-03 records
// carry none, which is why the coverage view has only ever seen two ad-hoc reviews in delivery files;
// those sittings are grandfathered under D0155 and the gap is noted here rather than backfilled.
// =====================================================================
package ProjectSitting04 {
    private import EngineElement::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    verification sitting04Review : Test {
        :>> id = "5c9e1a83-7b04-4d62-9f18-e35a0c7b6d41";
        :>> title = "Sitting 04 review — sprints 318-319 (the sitting-review grandfather line + the resolver-kind guard and flag sweep) (2026-08-17)";
        :>> createdAt = "2026-08-17";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Per-sitting sprint review (D0049 single human gate; D0073 mechanism; batched per D0019). wweatherholtz reviewed and accepts the content of sprints 318 and 319. WHAT WAS IN FRONT OF THEM, stated exactly, because a confirmation is only worth what the human actually saw: a computed table of the frontier (ready empty, suspect empty, invalidEvidence empty, open_issues issue132 only, pendingAcceptances d0156, done 635 / outstanding 4, sitting-coverage due 2 grandfathered 313), a per-sprint summary of what each landed, and two named items requiring them. SPRINT 318 applied D0155, the grandfather line they had accepted earlier the same day: `sitting-coverage` gained due / grandfathered_unreviewed / due_sprints / grandfatherBasis while `uncovered` kept its meaning, and the Decision was committed SEPARATELY from the sprint record so the sprint doing the work could not grandfather itself. SPRINT 319 added guard `resolver-kind` (the 34th, hard-blocking) after finding that issue133's #Resolves edge pointed at a viewer SystemRequirement pasted from the line above it, so `open-issues` reported it triaged while nothing was on the hook for it; and finished the sweep issue133 had asked for, routing 28 subcommand parsers plus 4 skip-flag variants through one `root_arg` helper so a mistyped flag errors instead of becoming a ROOT path. Both sprints' own defects were reported to them rather than smoothed over — a trailing comma that made every refusal response unparseable, and my discarding uncommitted work with `git checkout` while reverting an injected test defect. Explicit attestation: '#2 accepted' (2026-08-17), given in direct reply to a message whose item 2 was 'The sitting review of these two sittings — the first live items of the queue you just authorized.' This attests the human's ACCEPTANCE OF THE SITTING and is not a claim that they read every line of the diff.";
    }
    part sitting04ReviewR1 : TestResult {
        :>> id = "f10b47d9-6c25-4e83-a7b1-0d94c58e2f63";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "2938d94";
        :>> judgedAt = "2026-08-17";
        :>> judgedBy = "wweatherholtz";
    }

    #Covers dependency from sitting04Review to sittingScopeStory;
    #Covers dependency from sitting04Review to triageFlagsStory;
}
