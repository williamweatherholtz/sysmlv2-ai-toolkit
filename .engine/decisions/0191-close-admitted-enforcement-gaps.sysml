// D0191 (PROPOSED — NOT YET ACCEPTED) — D0191: the deploy and actor-enrollment admitted gaps get their guards
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0191Accept event on sign-off.
// APPLICATION NOTES (the enforcement-gap family this Decision opened):
//   - sprint 409: the deploy unit's release pipeline gained its publish gate (issue211).
//   - sprint 410: the pre-push behind-check joined .githooks, the enforcementPoints inventory, and
//     the control-events declaration (issue209) - the [pre-push-behind] row in control-events.toml
//     is this Decision's enforcement family extending the D0193 contract by its own rules.
package Decision0191 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - governs guards 43/44, their unit-ownership declarations in
    // deploy.sysml + actor-enrollment.sysml, and the two satisfied entries leaving process-enforcement.toml.
    #ProspectiveChange part d0191 : Decision {
        :>> id = "14f71df6-3520-4782-8b4a-af3658b1eec4";
        :>> title = "D0191: the deploy and actor-enrollment admitted gaps get their guards";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The hardening lens reports two processes whose enforcement contracts ADMIT a guard could exist and does not. [deploy]: a release's claims - a version was recorded, the reconciled version matches the tag - are text properties over authored items, and today a tag could ship with no Release item and nothing would fire. [actorEnrollment]: an enrolled actor is registered, has a kind, and is bound on the machine; the actors guard covers registration but is CORE, so the unit reports zero, and NOTHING validates the machine binding file at all - a .keel/actor naming an unregistered or kindless actor is currently invisible until a write refuses somewhere downstream. Presented to the human on 2026-08-21 as the two most pressing items ahead of the critique passes; the other two admitted gaps (distributedCollaboration, migration) were presented as part of the same lens read but not named in the plan and stay recorded as honest gaps.";
        :>> decision = "Two new guards, both WARNING tier per the guided profile's advisory-first rule (D0174), each OWNED by its process unit via the D0139(D) assert-constraint declaration so a project that has not adopted the process is not gated by it. (1) release-recorded, owned by deploy: every local version tag (v-prefixed) has a Release item whose title names it and whose recorded commit matches the tag's commit; zero tags scans zero and passes, so a downstream project without releases is untouched. (2) enrollment-binding, owned by actor-enrollment: when a machine binding (.keel/actor) exists, its name resolves to a registered Person or to an Actor carrying a kind; an absent binding scans zero. The actors guard STAYS core - registration integrity holds whether or not a project adopts the enrollment process, and moving it into a deactivatable unit would let deactivation switch off identity checking. The stale UNDER-GUARDED entries for these two processes leave process-enforcement.toml in the same commit (one truth, no tombstones).";
        :>> rationale = "D0047: a gap a guard could close stays closed by the guard, not by vigilance - and both entries have said UNDER-GUARDED in a committed contract while nothing tracked closing them. Warning tier is not timidity: the guided profile promotes on cited fire-ledger evidence, and a brand-new predicate blocking commits on day one is how issue089 happened. Unit ownership over core placement is deliberate for BOTH new guards: each checks a claim that only exists where its process is adopted.";
        :>> consequences = "IF ACCEPTED: GUARD_NAMES 42 -> 44; deploy.sysml and actor-enrollment.sysml each declare their guard; two contract entries deleted as satisfied; the hardening lens's undeclared bucket drops from six entries to four (the two remaining admitted gaps plus two declared-false entries); doc surface updated in the same commit. COST: two more predicates to keep truthful, and the release-recorded guard shells to git tag, so a repo with exotic tag names could scan noise - the v-prefix filter is the containment. REMOVAL PATH: delete the guard functions, the two assert-constraint lines, and the GUARD_NAMES entries; nothing else reads them.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0191Accept : Test { :>> id = "e5f951bc-9406-4e9b-ad8f-26d69564e13b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'agreed. proceed' - said in direct response to the presented ranking whose item 2 named exactly these two admitted enforcement gaps (deploy release-claims guard; actor-enrollment guard ownership) and the recommendation 'I take the two enforcement gaps now'. The acceptance covers those two gaps; the decision text flags that distributedCollaboration and migration were visible in the same lens read and are NOT covered by this sign-off."; }
    part d0191AcceptR1 : TestResult { :>> id = "43c94662-f195-43ef-8862-afe6904cc6dd"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1d84acd"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
}
