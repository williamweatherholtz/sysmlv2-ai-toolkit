// =====================================================================
// D0082 — coverage reports THREE TIERS, not one flat "covered": verified (reproducible evidence) >
// attested (human confirmation, where judgment isn't testable) > addressed (work/trace only — a
// claim). Reserves "verified" for objective evidence; stops conflating satisfy/charter-dod with it.
// Accepted 2026-06-21.
// =====================================================================
package Decision0082 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0082 : Decision {
        :>> id = "63940516-2c7d-4e8f-a082-516273849690";
        :>> title = "D0082: three-tier coverage — verified (objective evidence) > attested (human confirmation) > addressed (claim); 'verified' reserved for reproducible evidence";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The `sysmlv2 coverage` view (D0079 C) collapsed four different-strength bases into one flat 'covered' verdict: explicit-test (a passing verify-edge Test), accept-event (a Decision's human acceptance), charter-dod (a work item chartered to the element is done), and satisfy (a need traced to a covered requirement). The human observed that this CONFLATES two genuinely different things: 'satisfied' (a subjective claim that the element is addressed — by derivation, intent, or work done) vs 'verified' (defensible, objective, reproducible evidence that the element actually holds). This is exactly the SysML satisfy(intent) vs verify(evidence) distinction, already latent in computed-state.md (satisfaction==verified requires a covered verify-chain, never a satisfy-trace). Treating charter-dod (work happened) or a satisfy-trace as 'covered' overstates assurance — a done DoD says the work finished, not that the requirement is demonstrated.";
        :>> decision = "`sysmlv2 coverage` reports a TIER per element, strongest-available: (1) VERIFIED — a reproducible verify-edge Test passed non-stale (method test/analyze/inspect/demo); a Need is verified TRANSITIVELY when its satisfy'd requirement is verified (contract transitive satisfaction); method=critique verify-edges are EXCLUDED (they feed critique-coverage, an adversarial lens, not objective coverage). (2) ATTESTED — a human acceptance event (Decisions; defensible where judgment isn't testable, e.g. 'we chose Rust'). (3) ADDRESSED — only charter-dod (work done) or a satisfy-trace to a non-verified requirement: a CLAIM, not evidence. (4) SUSPECT — the only complete evidence is a stale verify-Test. (5) UNCOVERED — nothing. The GATE (coverage gap feeding `guard assured`/readiness) treats verified|attested as covered; addressed/suspect/uncovered are gaps. A verify-edge Test's completeness is read from its own TestResult (`<name>R1` outcome), not the action-task `done` set (verification Tests aren't action DoDs).";
        :>> rationale = "Reserving 'verified' for reproducible evidence makes the strongest assurance word MEAN something — it can't be earned by intent or by merely doing work. Attestation is kept as its own tier because forcing 'verified' onto Decisions would be wrong (they aren't unit-testable; their evidence IS the human's word, D0016). 'Addressed' names the honest middle: traced/worked-on but not demonstrated. This realigns the view with computed-state.md's own satisfy/verify split (the implementation had drifted from the contract). Transitive verification through satisfy is legitimate ONLY because the requirement carries real downstream evidence — it propagates verification, not intent. ALTERNATIVES: (A) keep one flat 'covered' — REJECTED (conflates evidence with claims, the defect the human caught); (B) two tiers verified/not — REJECTED (would mislabel validly-attested Decisions as unverified). CRITERIA: verified-means-evidence x attestation-where-untestable x honest-middle-tier x realign-to-contract.";
        :>> consequences = "view.rs: tier_of replaces state_of (verified/attested/addressed/suspect/uncovered); tier_for_kind maps verifier kinds to tiers; explicit-test completeness read from the Test's own R1 result + method!=critique exclusion; needs verified transitively iff their requirement is verified; coverage view emits `tier` + a per-tier summary; is_covered_tier (verified|attested) feeds the gate. Pairs with D0079 (C), the satisfy/verify edge semantics, D0016 (confirmation evidence), computed-state.md (coverageState/satisfaction). FOLLOW-ON (same sprint): objective verification authored for 13/14 requirements (real evidence) -> they + their needs report VERIFIED; sr11FastStart is a measured GAP (orient ~13.6s vs <500ms) tracked as an Issue, not faked. KNOWN LIMITATION (future): requirement-verifications aren't in the deliverable-suspicion manifest, so they don't auto-stale on code drift (mitigated: the whole-suite checks they cite re-run every commit via the hook)."; }

    verification d0082Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a082-516273849691";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0082 (three-tier coverage: verified > attested > addressed; 'verified' reserved for reproducible objective evidence, attestation kept for non-testable judgment, satisfy/charter-dod stop counting as verified) by selecting 'Accept the three tiers' in the coverage-semantics dialogue on 2026-06-21.";
    }
    part d0082AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a082-516273849692";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "0941e97";
        :>> judgedAt = "2026-06-21";
        :>> judgedBy = "wweatherholtz";
    }
}
