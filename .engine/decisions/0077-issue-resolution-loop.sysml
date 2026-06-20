// =====================================================================
// D0077 — Issue Resolution Loop (IRL): a #Resolves typed edge + COMPUTED issue open/resolved
// state (an issue is resolved iff a #Resolves resolver — an action that's done or a Decision
// that's accepted — is complete). Adds an open-issues view + orient surfacing + a guard + a
// process/skill + a backfill of all existing issues. Supersedes the hand-appended prose
// "RESOLVED" convention. Accepted 2026-06-20 by wweatherholtz (issue-resolution design dialogue;
// schema/core §2.5 sign-off for the #Resolves edge given there).
// =====================================================================
package Decision0077 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // D0077 introduces the issue-resolution PROCESS + a schema edge. Like D0075, its own commit
    // edits no .engine/processes|workflows file (the process-def + schema land in implementation
    // sprints), so no #ProspectiveChange marker is required on this Decision (D0070).
    part d0077 : Decision {
        :>> id = "63940516-2c7d-4e8f-a077-516273849665";
        :>> title = "D0077: Issue Resolution Loop — #Resolves edge + COMPUTED open/resolved; open-issues view + orient surfacing + guard + process/skill; supersedes the prose-RESOLVED convention";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The backlog frontier read empty (orient ready=[]) while issue014 + issue025 were open — a structural gap. Three root causes: (1) Issues live in .tracking/issues.sysml OUTSIDE the NextWork action DAG that orient's ready computes, so an open issue never appears in the frontier; (2) there is NO computed open/resolved state — the Issue schema has only description/discoveredInField/relatedTask (no status), and closure was a hand-appended prose 'RESOLVED' clause in the description (not machine-readable); (3) relatedTask (D0029) points at the RELATED/causing task, not the OPEN task that RESOLVES the issue — issue014->reverifyDeliverable and issue025->retireQueryPy are BOTH done yet both issues are unresolved (13 suspects persist; 7 query.py subcommands still dropped), so a naive 'resolved when relatedTask done' rule mis-closes them. D0029 established relatedTask but no resolution LOOP was ever built; issues silently accumulate while the frontier reads empty.";
        :>> decision = "Model issue resolution as a #Resolves TYPED EDGE: `#Resolves dependency from <resolver> to <issueNNN>;` where <resolver> is a backlog ACTION or a DECISION (mirrors the proven #CharteredBy marker-edge). An issue is RESOLVED iff some #Resolves edge points to a resolver that is COMPLETE — an action that is done (orient done-set) or a Decision with status=accepted — else OPEN. Resolution is fully COMPUTED, never stored (D0001/D0018). Add: `sysmlv2 open-issues` (computed view: the OPEN issues + each one's resolver, or 'untriaged' if none); an `open_issues` field in `sysmlv2 orient` so the frontier surfaces them (an open issue with no OPEN resolving action is a triage signal); a `sysmlv2 guard issues` (D0047 — every issue carries a #Resolves edge; flag untriaged issues + prose 'RESOLVED' claims lacking a complete resolver) wired into the pre-commit hook; an issue-resolution PROCESS + deploying SKILL (D0059 — record issue -> link #Resolves to a resolving action, creating one if none, or a wontfix/by-design Decision -> resolution computes when the resolver completes); and a BACKFILL (migration, D0067) of #Resolves onto all existing issues. The prose 'RESOLVED' clauses become informational only; relatedTask is retained as the original related/causing link but is no longer the resolution signal.";
        :>> rationale = "Compute-don't-store keeps resolution from drifting (the exact risk D0018 warns about; §2.1) — there is no authored status to fall out of sync. #Resolves reuses the #CharteredBy marker-edge machinery (one mechanism, already parsed) and expresses BOTH fix-by-work and fix-by-decision (wontfix/by-design). Making the open-issue set a first-class COMPUTED view that orient surfaces means the ready frontier can never again read 'empty' while issues are unresolved. ALTERNATIVES: (A CHOSEN) #Resolves computed edge; (B) re-convention relatedTask to mean 'the open resolving action' — rejected: conflates caused-by vs resolved-by and cannot cleanly express Decision-closure; (C) an authored Issue.status field (open/resolved/wontfix) — rejected: stores computable state (violates §2.1), can drift (D0018), and is a larger frozen-schema change. CRITERIA: computed-not-stored x expressive(work+decision) x surfaces-in-frontier x one-mechanism(reuse #CharteredBy) x frozen-schema-minimal.";
        :>> consequences = "Frozen schema/core gains ONE edge (#Resolves in relationships.sysml; §2.5 sign-off given by wweatherholtz 2026-06-20). Built over 5 chained sprints: issueResolveModel (D0077 + #Resolves schema edge + Rust edge recognition) -> issueOpenView (open-issues view + orient open_issues surfacing) -> issueGuard (`sysmlv2 guard issues` + hook) -> issueBackfill (migration: #Resolves onto all ~25 issues) -> issueResolutionSkill (process def + deploying skill). The BACKFILL will EXPOSE the secretly-open issues (014, 025, and any others whose relatedTask is done-but-unresolved) as needing a fresh resolving action or a wontfix Decision — that surfacing is the point. The prose 'RESOLVED' convention is superseded by computed resolution. Pairs with D0029 (relatedTask origin), D0001/D0018 (compute-don't-store), D0070 (#marker-edge pattern), D0047 (corrections->guards), D0059 (every process has a deploying skill), D0067 (migration process).";
    }

    verification d0077Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a077-516273849666";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0077 (Issue Resolution Loop: #Resolves typed edge + COMPUTED open/resolved resolution; open-issues view + orient surfacing + guard + process/skill + backfill; supersedes the prose-RESOLVED convention) AND gives the frozen schema/core §2.5 sign-off for the #Resolves edge — by selecting '#Resolves typed edge (computed)' then 'Accept — plan the full IRL milestone' in the issue-resolution design dialogue on 2026-06-20.";
    }
    part d0077AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a077-516273849667";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "62aa211";
        :>> judgedAt = "2026-06-20";
        :>> judgedBy = "wweatherholtz";
    }
}
