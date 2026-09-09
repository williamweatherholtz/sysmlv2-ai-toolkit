// D0395 (status: the `status` field below is the truth) — mootTransientRedYield: red-yield obligation3b91c51a is discharged - its three problems were transient states of one in-progress build, and the tree is green
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0395Accept event on sign-off.
package Decision0395 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0395 : Decision {
        :>> id = "76c76c15-c439-43d6-bbe7-c2a34f7aa6ea";
        :>> title = "mootTransientRedYield: red-yield obligation3b91c51a is discharged - its three problems were transient states of one in-progress build, and the tree is green";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "obligation3b91c51a was auto-recorded when the Stop hook's second consecutive red pass yielded (D0176/K7/D0350) during a long build in this session. Its three named problems were all transient states of work in flight: sprint617's storySuiteVerdictIsShellIndependent was unstamped because the suite proving it was still running, and `reject` was declared in .engine/cli/commands.sysml but not yet in the compiled cli_facts.rs and dispatch because the binary predated the edit.";
        :>> decision = "The obligation is mooted. sprint617 stamped storySuiteVerdictIsShellIndependent and dcSuiteReceiptDoesNotTurnOnPath, closing the sprint-closure line; sprint618 added `reject` to cli_facts.rs, COMMAND_NAMES and the dispatch, closing both cli-surface-declared lines. keel guard is clean at the commit carrying this Decision and its #Resolves edge to obligation3b91c51a. No correction beyond finishing the sprints was needed, which is what makes this a moot rather than a fix.";
        :>> rationale = "A red-yield obligation records that a turn boundary was crossed while the tree was red; it is a real control event and must be triaged, not deleted. Here the redness was an honest in-progress state of a multi-step build across turn boundaries, not a defect, so the triage is a moot naming the obligation rather than a new task - the sprints that finished the work are its evidence.";
        :>> consequences = "The #Resolves edge from this Decision to obligation3b91c51a discharges it; open-issues no longer lists it. The pattern - a long build spanning turn boundaries yields red and auto-records an obligation - is inherent to D0350 and correct; nothing changes in the control.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0395Accept : Test { :>> id = "ded5d3fd-0918-437d-94d7-a51d53fce947"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0395AcceptR1 : TestResult { :>> id = "1214d695-ce6d-4761-b4f5-b2ad72854ca3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4b75101"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
