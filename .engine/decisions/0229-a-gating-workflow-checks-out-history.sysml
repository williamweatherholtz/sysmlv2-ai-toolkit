// D0229 (status: the `status` field below is the truth) — gatingWorkflowHistory: the release gate had been red since guard 48 landed, and nobody could have known
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0229Accept event on sign-off.
package Decision0229 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0229 : Decision {
        :>> id = "6759297d-4ac1-4838-b488-826f38726ed6";
        :>> title = "gatingWorkflowHistory: the release gate had been red since guard 48 landed, and nobody could have known";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Tagging v0.3.0, the release workflow FAILED - not on anything in the release, but on a test asserting that no guard reports a zero population. The guard reporting zero was claim-ancestry, which skips loudly on a shallow clone because a depth-dependent verdict is the machine-dependence K15 forbids. `actions/checkout@v4` defaults to depth 1. `ci.yml` carries `fetch-depth: 0` with a comment citing issue229 - the exact fix, for the exact reason. `release.yml` never got it. So the release gate has been red since guard 48 landed weeks ago, and it went undetected because releases are rare enough that the workflow had not run since: v0.2.3 was tagged on 2026-08-14, before that guard existed. Three of four workflows had the fix. This is the same-fix-in-N-places class, which is the single most-repeated finding in this project's retros, and here it hid a red gate rather than breaking a build - the release path is the one place where nothing else would notice.";
        :>> decision = "`release.yml`'s gate job checks out full history, matching the other three workflows. Guard 51, `gating-workflow-history`, refuses any workflow that RUNS the gate - judged by what it invokes, not by its name - while checking out shallow, because such a workflow reports a pass it never verified. The scan-count test records claim-ancestry as legitimately empty on a shallow clone, with the reason rather than just the name. The guard's limitation is stated in its own doc comment and in guards.md: the check is per-FILE, not per-job, because there is no YAML parser here, so a workflow whose non-gating job alone sets the flag would pass - which is the shape release.yml legitimately has.";
        :>> rationale = "A workflow that runs the gate and silently disables part of it is worse than one that does not run the gate at all, because the first reports a pass. That is the honest-state class D0098 permits gating unconditionally: the claim 'this workflow verified the tree' was false, and falsifiable by a literal anyone can grep for. Making it a guard rather than a note is D0047 applied to the case that motivates D0047 most sharply - the fix WAS already known and written down in ci.yml, with the issue number in a comment, and it still did not reach the fourth file. Manual vigilance did not merely fail here; it had already succeeded once and did not generalise. The rarity is the aggravating factor rather than a mitigating one: a gate that runs weekly gets caught in a week, and a gate that runs at release time can stay red across an entire development cycle.";
        :>> consequences = "Adding a workflow that runs the gate now costs one literal, and the guard refuses the commit without it. The per-file limitation means a two-job workflow can still hide a shallow gating job, which is a real hole and is recorded as one rather than left to be discovered; closing it needs a YAML parser and is not worth a dependency today. This also means v0.3.0's tag currently points at a commit whose release gate FAILED - the tag will be re-cut against a commit that passes, since a release is the one artifact whose gate must have actually run. Resolves issue260.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0229Accept : Test { :>> id = "3598239a-f77d-4c5f-95c7-d2f2182ad232"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/27."; }
    part d0229AcceptR1 : TestResult { :>> id = "f4a08df1-ef2b-42a6-abd8-6057ac48ca67"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c444ca5"; :>> judgedAt = "2026-08-25"; :>> judgedBy = "wweatherholtz"; }
}
