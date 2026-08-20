// =====================================================================
// D0115 — keel-viewer SCOPE REVISION (human feedback 2026-07-14). Single-user + UNAUTHENTICATED (drop
// N-7's auth/multi-user; saveable viewpoints via N-8 suffice); DEFER N-11 (notifications) + N-14
// (audit/provenance) out of current scope; REFOCUS N-9 from a coverage DASHBOARD to AI CRITIQUE (the
// real intent). Supersedes the four Needs (#Supersede edges in .tracking); their now-moot SRs + backlog
// are removed. Product-scope decision (not a process/schema change) — no #ProspectiveChange. Accepted.
// =====================================================================
package Decision0115 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0115 : Decision {
        :>> id = "10a1b0c1-4115-42a3-ab5c-516273849715";
        :>> title = "D0115: keel-viewer scope revision — single-user unauthenticated; defer notifications + audit-provenance; refocus coverage-gap onto AI critique";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Reviewing the 14 keel-viewer Needs, the human gave scope feedback (2026-07-14): (a) N-7 (multi-stakeholder deployment + auth) is NOT wanted — the viewer is single-user, unauthenticated, for the human's own use ('observe and critique different systems, processes, architecture'); the only surviving part is SAVEABLE viewpoints, already N-8. (b) N-11 (subscribe/notify) and N-14 (audit-trail/provenance) are not needed now — DEFER. (c) N-9 was authored as a coverage/gap DASHBOARD, but the real intent is AI CRITIQUE over a constrained set: e.g. critique a process + its downstream for best practices, or critique whether a requirement's downstream items semantically satisfy it — plus modifying + critiquing elements in-program.";
        :>> decision = "REVISE scope: (1) supersede N-7 (nViewerMultiStakeholder) — the viewer is single-user + unauthenticated; saveable viewpoints are covered by N-8 (drop auth/multi-user/per-user). (2) supersede N-11 (nViewerNotifications) + N-14 (nViewerAuditProvenance) — DEFERRED out of the current scope (may revive later; re-derive then). (3) supersede N-9 (nViewerCoverageGap) — REFOCUSED: replaced by new Needs for iterative AI critique over a query (axis-iterated) + in-program modify-and-critique (authored next, pending the human's acceptance gate). Remove the now-moot SystemRequirements (srViewerCoverageGap/Notifications/AuditProvenance) + their satisfy edges + backlog actions. Superseding is via #Supersede edges (in .tracking, computed back-reference).";
        :>> rationale = "ALTERNATIVES: (i) keep N-7/N-11/N-14 as low-priority — REJECTED: the human explicitly does not want them now; carrying unwanted Needs inflates the burndown + misdirects work (honest scope, §2.4: 'won't do X' is a first-class Decision that supersedes the Need). (ii) mutate the accepted Needs in place — REJECTED: an accepted Need is not overwritten; a superseding Decision is the audit-honest mechanism. (iii) keep N-9 as a dashboard AND add critique — REJECTED: the human reframed N-9's intent onto critique; the requirement-comprehensiveness example IS coverage delivered via critique, so critique subsumes it. CHOSEN — supersede the 4 Needs + refocus onto AI critique, which also leans on substrate keel already has (element-critique, the agent bridge, apply-review write-back). CRITERIA: honest-scope x no-in-place-mutation x reuse-existing-substrate x human-directed.";
        :>> consequences = "ACCEPTED by the human 2026-07-14 (discrete d0115Accept TestResult, human-judged — the scope feedback message). N-7/N-9/N-11/N-14 become superseded (out of current scope) via #Supersede edges from d0115; their 3 SRs + satisfy edges + 3 backlog actions are removed. New Needs (iterative AI critique + in-program modify/critique) are authored and await a human acceptance gate before their architecture. Net current Needs: 10 live (N-1..N-6, N-8, N-10, N-12, N-13) + the 2 new (pending). Reduces scope + refocuses on the human's actual goal: AI-assisted critique of constrained systems."; }

    verification d0115Accept : Test { :>> id = "10a1b0c1-4115-42a3-bb5c-516273849715"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests the keel-viewer scope revision D0115 (single-user unauthenticated; defer N-11 + N-14; refocus N-9 from coverage-dashboard to AI critique) via the explicit scope feedback on 2026-07-14: 'we don't need auth at all... just for me, unauthenticated'; 'I don't think we need N11 - defer it. same for N14'; 'n9: I want to be able to critique constrained systems using AI tools'."; }
    part d0115AcceptR1 : TestResult { :>> id = "10a1b0c1-4115-42a3-cb5c-516273849715"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "wweatherholtz"; }
}
