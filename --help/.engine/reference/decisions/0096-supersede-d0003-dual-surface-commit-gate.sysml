// =====================================================================
// D0096 — supersede d0003 (single identical API + API-enforced writePolicy) with the REALIZED
// dual-surface + commit-gate model (D0093 + D0094). Resolves issue045: the critique campaign found
// d0003's "writePolicy ENFORCED by the API, no self-promotion" was never implemented (one CLI write
// path, no per-skill authz), AND its "same API with identical capability for GUI + AI" was itself
// superseded by D0093's dual-surface. This Decision records what is actually true.
// Accepted 2026-06-23 by wweatherholtz (ACT disposition on issue045: "supersede d0003 to the
// commit-gate model").
// =====================================================================
package Decision0096 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0096 : Decision {
        :>> id = "96a6b7c8-1d6e-4f70-a829-516273849696";
        :>> title = "D0096: supersede d0003 — authorization is the human commit-gate (D0094), surfaces are dual not identical (D0093)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Critique-coverage campaign (D0080) finding issue045 (Medium): d0003 (single-api-write-policy) asserts (1) the GUI and AI agents use the SAME API with identical capability, and (2) authorization differs via a per-skill writePolicy (read-only|pr-only|direct) ENFORCED by the API, with no self-promotion. Reviewing the code + later decisions: (2) is UNREALIZED — write.rs has no per-skill authz, there is one CLI write path callable regardless of any skill's declared policy, and nothing reads a skill's policy or blocks self-promotion; (1) was SUPERSEDED by D0093's dual-surface (the CLI/JSON and the HTML are NOT identical-capability surfaces). d0003's model never matched reality and has been overtaken without a recorded supersede.";
        :>> decision = "Supersede d0003. Its model (one API with identical capability for GUI + AI; authorization via per-skill API-ENFORCED writePolicy; no self-promotion) is REPLACED by the realized DUAL-SURFACE + COMMIT-GATE model: (a) the CLI/JSON is the AUTHORITY + automation substrate (the AI agent's full-capability surface — every fact authored via the write API, every state computed by the toolchain); the HTML is the human's ergonomic OVERSIGHT lens (renders computed #Views + wraps the write API for review/decide), NEVER a second store or a second authority — the two surfaces are DIFFERENTIATED, not identical (D0093). (b) AUTHORIZATION is the human COMMIT-GATE: the agent runs under the existing discipline (skills + write API + guards + >=Medium human-disposition) and NEVER auto-commits, so the human's commit/acceptance IS the authorization boundary; there is NO per-skill API-level writePolicy enforcement and NO separate permission policy (D0094). The skill `writePolicy` frontmatter remains a DECLARED convention the agent follows, not an API-enforced control. d0003 -> status superseded.";
        :>> rationale = "ALTERNATIVES: (i) implement per-skill writePolicy enforcement in the write path (read the caller's policy, gate read-only|pr-only|direct, block self-promotion) — REJECTED as unnecessary complexity for the single-user / AI-drives-CLI / human-supervises design, where the commit-gate already provides the authorization boundary; (ii) leave d0003 as-is — REJECTED (it states an enforcement that does not exist + a same-API claim D0093 already superseded, so it misleads any reader/newcomer); (iii) CHOSEN — record the realized model (D0093 dual-surface + D0094 commit-gate) as d0003's successor, so the architecture decisions match the code. CRITERIA: accuracy-of-the-record x least-complexity x consistency-with-D0093/D0094. The human ACT-dispositioned issue045 to this effect.";
        :>> consequences = "d0003 is marked superseded (its single-API + API-enforced-writePolicy model retired); D0093 (dual-surface) + D0094 (commit-gate authz, agent-never-auto-commits) + this D0096 are the in-force authorization + surface model. No code change: write.rs already has no per-skill authz, and the commit-gate is already how the agent operates (it never auto-commits; the human commits). The skill `writePolicy` field stays a followed convention. issue045 resolved (this Decision is its #Resolves resolver). Newcomers reading the decisions now see the actual model, not the unrealized one.";
    }

    verification d0096Accept : Test { :>> id = "96a6b7c8-1d6e-4f70-b829-516273849696"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of d0096 (supersede d0003 with the dual-surface + commit-gate authorization model) via the issue045 ACT disposition on 2026-06-23 ('ACT - supersede d0003'), with the recorded rationale to supersede d0003's enforcement clause to the commit-gate model."; }
    part d0096AcceptR1 : TestResult { :>> id = "96a6b7c8-1d6e-4f70-c829-516273849696"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "64d2135"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "wweatherholtz"; }

    // Decision-supersedes-decision: d0003.status -> superseded (edited in 0003-single-api-write-policy.sysml);
    // the supersede back-reference is computed (relationships.sysml). This Decision is issue045's #Resolves resolver.
}
