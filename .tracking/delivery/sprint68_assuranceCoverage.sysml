// =====================================================================
// ProjectDeliveryS68 — Sprint 68: assuranceCoverage (D0079 C — computed assurance coverage).
// Story: `sysmlv2 coverage` — for each Need / SystemRequirement / accepted Decision, compute
//        whether it has COMPLETE + PASSING + NON-STALE evidence (a verifier), from authored facts
//        + git, never stored. Implements the computed-state.md coverageState/satisfaction/gaps
//        contract through the D0079 lens (charter-dod + accept-event + satisfy + explicit-test).
// HEADLINE FINDING (live): Decisions 76/76 covered (attestation); all 28 requirements+needs
//        uncovered — the requirement layer has zero verification linkage (0 verify edges, 0
//        charter-to-requirement). The gap the assurance doctrine exists to expose.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS68 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from assuranceCoverageStory to d0079;

    part assuranceCoverageStory : Story {
        :>> id = "63940516-2c7d-4e8f-b068-51627384a501";
        :>> title = "Sprint 68: assuranceCoverage (5 pts) — `sysmlv2 coverage`, computed assurance coverage (D0079 C)";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS68 {
        action storyAssuranceCoverage;
        verification storyAssuranceCoverageDoD : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a502"; :>> method = VerificationMethod::test; :>> procedureText = "assuranceCoverageDoD: `sysmlv2 coverage` computes per-element coverage for Need/SystemRequirement/accepted-Decision from verifiers (strongest-first: explicit-test verify edge -> accept-event Decision attestation -> charter-dod done+non-stale work via the orient done/suspect authorities + Story->action name bridge -> satisfy transitive for Needs). State covered/suspect/uncovered + basis; per-type summary + flat gap set; honest by construction (never fabricates). Superseded/rejected Decisions excluded (accepted-only, mirrors the attestation guard). VERIFY (live): Decisions 76/76 covered (accept-event); SystemRequirements 14/14 + Needs 14/14 uncovered (28 gaps) = the real verification-linkage gap. clippy --workspace -D warnings + cargo test (32 lib) + rust validate (78 clean) + guard all green."; }
        part storyAssuranceCoverageDoDR1 : TestResult { :>> id = "44d5b80b-395d-41c2-b9e0-d7759419a9a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification assuranceCoverageRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a503"; :>> title = "Sprint 68 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0079 (C), the one defensible coverage gap (computed-state.md written-but-unimplemented). Refine INVESTIGATION of the live data (the key DoR work): 14 needs satisfy 14 requirements; ZERO verify edges; ALL #CharteredBy edges target Decisions (none target a requirement/need). So a literal 'requirement assured iff chartered work' rule yields 28/28 uncovered — which is the honest finding, not a bug to paper over. Design: verifiers strongest-first (explicit-test / accept-event for Decisions / charter-dod / satisfy-transitive for Needs); accepted-Decisions only. Story->done bridge (two name forms) verified to resolve all 31 charters. estimatedPoints=5. #CharteredBy d0079. DoD accepted."; }
    part assuranceCoverageRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b068-51627384a504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification assuranceCoverageStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a505"; :>> title = "Sprint 68 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: add compute_coverage + coverage(root) to view.rs (mirrors open_issues: build Model, pull orient::done_names + orient::compute().suspect, emit Json); direct_verifiers(explicit-test edge / accept-event for Decisions / charter-dod via charter_forms bridge); Needs add a transitive satisfy verifier over pass-1 requirement states; wire `sysmlv2 coverage` in main.rs. Add 'verify' to KNOWN_EDGES. Unit tests for states/basis + the charter bridge. No blockers."; }
    part assuranceCoverageStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b068-51627384a506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification assuranceCoverageImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a507"; :>> title = "Sprint 68 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented view.rs compute_coverage/coverage + main.rs cmd_coverage + 'verify' in KNOWN_EDGES. Two correctness refinements caught DURING implement by reading the live output (not in the original plan): (1) a Decision's canonical assurance is its accept-event attestation (D0066), not chartered work — added accept-event verifier, else 69 attested Decisions falsely read uncovered; (2) superseded/rejected Decisions are not active commitments — scoped targets to accepted Decisions only (mirrors the attestation guard), removing the false d0038 gap. VERIFY (live): Decisions 76/76 covered; SR 14/14 + Need 14/14 uncovered = 28 gaps; covered-by-basis all accept-event. clippy --workspace --all-targets -D warnings green; cargo test 32 lib pass (coverage_states_and_basis + charter_forms_bridges_story_names added); rust validate 78 clean; guard all PASS."; }
    part assuranceCoverageImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b068-51627384a508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification assuranceCoverageReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a509"; :>> title = "Sprint 68 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. coverage() is a pure view (build Model + orient authorities -> Json), never stores a verdict (D0001/contract). Honest-by-construction: uncovered when no complete verifier — the 28 requirement/need gaps are surfaced, not hidden. Reuses the SINGLE done/suspect authority (orient) — no parallel staleness logic. The two implement-time refinements (accept-event; accepted-only) make the Decision dimension correct (integrates the attestation contract) rather than red-noise. Known scope edges, deliberately deferred (downstream sprints): explicit-test basis is 0 until verify edges are authored/parsed; attestation-staleness not yet modeled; the 28 requirement/need gaps are the input to critiqueMechanism + a future 'wire requirements to evidence' decision (assuranceReadiness will need a coverage-scope policy, not a blind 28-gap block). doc-sync: coverage added to main.rs subcommand doc; no other doc surface claims coverage was implemented (computed-state.md already marked it as TARGET/unimplemented — now true)."; }
    part assuranceCoverageReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b068-51627384a50a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification assuranceCoverageCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a50b"; :>> title = "Sprint 68 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyAssuranceCoverage + backlog assuranceCoverage DoDs recorded pass; `sysmlv2 coverage` live (Decisions 76/76 covered, 28 requirement/need gaps); cargo test 32 + clippy + rust validate + guard all green. Self-evidencing (D0051). Computed-state.md coverageState/satisfaction/coverageGaps now implemented. Unblocks critiqueMechanism (D0080)."; }
    part assuranceCoverageCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b068-51627384a50c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification assuranceCoverageRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b068-51627384a50d"; :>> title = "Sprint 68 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: refining against the LIVE data (not the assumed model) again paid off — it set realistic expectations (28 gaps is the truth, not a failure) and the Story->done name bridge was empirically validated across all 31 charters before coding. Reading the live output mid-implement caught two correctness defects (accept-event; accepted-only) that a spec-only build would have shipped as false gaps. CAUGHT (avoidable-issue candidate): the original DoD criterion carried a fabricated '57 verify / 46 satisfy edges' estimate from the draft — corrected to the real numbers before recording (a reminder that draft DoD criteria must be re-grounded at implement, not trusted). No tracked Issue: it was a draft never recorded, fixed in-sprint. Estimate 5 pts held. SURFACED for the user/next sprints: the 28 requirement/need coverage gap is structural (no verify edges, no requirement charters) — assuranceReadiness must adopt a coverage-SCOPE policy rather than block on all 28, and critiqueMechanism should target the uncovered set first. Next: critiqueMechanism (D0080)."; }
    part assuranceCoverageRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b068-51627384a50e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c813eae"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
