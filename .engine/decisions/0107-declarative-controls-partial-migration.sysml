// =====================================================================
// D0107 — accept the guard->rule migration as PARTIAL (refines D0105). The sprint172 spike + the actual
// migration (sprints 178-183) showed the clean-fit set is ~5 guards, not the ~9-14 D0105 projected. The
// remaining guards are bad-fit for the declarative vocabulary (relational set-ops, external files,
// text-blob scans, git co-commit, prose heuristics, per-file gate-order) and STAY Rust per D0105's own
// rollback criterion. This CROSSED D0105's "majority of conformance guards fail parity -> reconsider"
// threshold, so it was surfaced to the human, who accepted partial migration + CONTRACT of the 5.
// Accepted 2026-07-03 by wweatherholtz (explicit choice "Accept partial migration + CONTRACT the 5").
// =====================================================================
package Decision0107 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0107 : Decision {
        :>> id = "07a1b0c1-4107-42a3-ab5c-516273849707";
        :>> title = "D0107: accept the guard->rule migration as PARTIAL — 5 rules + mechanism; the rest stay Rust";
        :>> createdAt = "2026-07-03";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0105 projected ~9 of 14 conformance guards would collapse into one EdgeRule check. The actual migration (sprints 178-183) cleanly migrated 5 (capabilityRootednessRule/issuesTriagedRule/decisionRationaleRule/acceptanceEventRule/charterRule), proving both rule kinds + the full scope sub-language (all/whereStatus/newlyAdded). Assessing the rest: sprint-coverage (text-blob+done-set), process-change (git co-commit of two files), critic-independence (relational set-op), manifest-coverage (external deliverable-manifest.txt), process-skill (cross-file text reference), decision-requirement-link (prose governance heuristic), ceremony (per-file gate-order+retro-keyword+grandfather), actors (legacy-name text scan) are all BAD-FIT for EdgeRule/ElementRule/OrderingRule; only viewpoint-renderer is borderline. A MAJORITY of conformance guards not reaching clean parity is exactly D0105's flagged 'reconsider the direction' threshold, so this was surfaced to the human.";
        :>> decision = "Accept the migration as PARTIAL (D0105 rollback criterion). (1) The declarative-controls MECHANISM stays — it is proven + valuable, and its primary payoff is that DOWNSTREAM projects can declare their own rules. (2) The 5 clean-fit guards are migrated to declared rules; CONTRACT retires their bespoke Rust predicates so each rule is the single gate source. (3) The ~8 bad-fit guards STAY Rust predicates indefinitely (not a failure — the honest outcome of the rollback criterion); they are re-evaluated for rule-migration only if a fitting rule kind emerges. (4) D0105's '~9 of 14 collapse' headline is corrected to '~5 cleanly migrate; the rest are bad-fit and stay Rust'.";
        :>> rationale = "ALTERNATIVES: (i) force the bad-fit guards into rules (new predicate kinds for relational/external/text/git/prose) — REJECTED (large, risky, drives toward the inner-platform problem D0105 explicitly warned against; low return). (ii) stop + keep rules AND all 14 guards side-by-side, no CONTRACT — REJECTED (leaves the 5 as duplicated gate logic, the exact drift D0105 set out to remove). (iii) CHOSEN — accept partial + CONTRACT the 5: banks the real payoff (mechanism for downstream + 5 single-source gates), stops forcing bad fits, and is honest about scope. CRITERIA: honesty (report the real fit) x payoff (downstream mechanism + retire duplication) x bounded-risk (no inner-platform push) x D0105-rollback-compliant.";
        :>> consequences = "REFINES D0105 (its collapse headline is corrected; its rollback criterion is exercised). CONTRACT: the 5 guards (requirement-rootedness/issues/decision-rationale/acceptance-events/charter) are rewired to source their violations from their declared rule (view::rule_violations), and the bespoke charter predicate fns are removed; the guard NAMES + report format + commit-gating are unchanged (parity proven in sprints 178-183). The ~8 bad-fit guards are unchanged. The `keel check`->`keel rules` name reconciliation + viewpoint-renderer (borderline) remain tracked follow-ups. doc-sync: CLAUDE.md Sec 5 (note which guards are rule-sourced vs Rust). Acceptance verdict/provenance are the discrete d0107Accept TestResult (not restated as prose, D0058/issue058).";
    }

    verification d0107Accept : Test { :>> id = "07a1b0c1-4107-42a3-bb5c-516273849707"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of D0107 (accept the guard->rule migration as PARTIAL: keep the declarative mechanism + the 5 migrated rules, CONTRACT-retire their Rust predicates, leave the ~8 bad-fit guards as Rust per D0105's rollback criterion) via the explicit choice 'Accept partial migration + CONTRACT the 5' on 2026-07-03, after being shown the per-guard fit assessment + that a majority of conformance guards are bad-fit (D0105's reconsider threshold)."; }
    part d0107AcceptR1 : TestResult { :>> id = "07a1b0c1-4107-42a3-cb5c-516273849707"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b1c81e9"; :>> judgedAt = "2026-07-03"; :>> judgedBy = "wweatherholtz"; }
}
