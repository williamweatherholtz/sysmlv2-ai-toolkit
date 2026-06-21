// =====================================================================
// D0083 — revise the FastStart target (n11/sr11). The original flat "500ms parse AND orient" was
// set when orient was a pure parse; orient now performs git-temporal SUSPICION across all tasks (a
// heavier analysis). Split the metric honestly: parse/validate under 500ms (the 'instant' bar, MET at
// ~110ms); full orient-with-suspicion under 1.5s (MET at ~0.85s after orientPerf). Accepted 2026-06-21.
// =====================================================================
package Decision0083 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0083 : Decision {
        :>> id = "63940516-2c7d-4e8f-a083-5162738496a0";
        :>> title = "D0083: revise the FastStart target — parse/validate under 500ms (the 'instant' bar) + full orient-with-suspicion under 1.5s; supersedes the flat 500ms set pre-suspicion";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The three-tier coverage verification (D0082, Sprint 73) measured `orient` at ~13.6s vs the n11/sr11 FastStart target of 500ms; orientPerf (Sprint 74) cut it to ~0.85s by batching git, but that still misses the literal 500ms. Investigating WHY surfaced that the original requirement conflated two very different paths under one bar: (a) parse/validate — `sysmlv2 validate .` measures ~110ms, comfortably under 500ms (the 'instant' perception threshold the rationale cites); (b) full `orient` — which now performs GIT-TEMPORAL SUSPICION across 200+ tasks (criterion-drift + deliverable-drift + transitive propagation), an analysis that did NOT exist when the 500ms bar was written (orient was then a simple parse, D0045). On Windows each git spawn is ~60ms, so a handful of batched git calls + parsing put full orient near ~0.85s irreducibly. The flat 500ms is therefore an apples-to-oranges target: right for parse, unrealistic for orient-with-suspicion.";
        :>> decision = "Split the FastStart metric to match what each path actually does. (1) PARSE/VALIDATE: `sysmlv2 validate` (full schema + tracking parse, no git) shall complete in under 500ms — the 'instant' bar, retained (measured ~110ms). (2) FULL ORIENT: `sysmlv2 orient` (parse + git-temporal suspicion across all tasks + open-issue computation) shall complete in under 1.5s on a developer laptop with no warm cache (measured ~0.85s after orientPerf's batched git). Amend the statements of need n11FastStart and requirement sr11FastStart to state both bars. This SUPERSEDES the prior flat '500ms parse-and-orient' value. The deeper parse-once refactor (orient parses the repo ~twice) is deferred — not pursued unless full orient regresses toward 1.5s.";
        :>> rationale = "Honest targets beat aspirational ones: a bar that conflates a 110ms parse with a 0.85s temporal analysis can never be met and would sit permanently red, training people to ignore it (the exact anti-pattern D0079 warns against). Splitting reflects reality — the fast interactive path (validate) genuinely is instant; orient's suspicion is a deliberate, valuable analysis whose cost is mostly irreducible git-spawn overhead on Windows. 1.5s keeps orient firmly interactive with headroom over the ~0.85s measurement for variance + modest growth. ALTERNATIVES: (A) keep 500ms + chase the parse-once refactor — REJECTED for now (real risk to a shared core path for ~350ms, diminishing returns; revisit if orient regresses); (B) accept sr11 permanently 'addressed/unmet' — REJECTED (a standing false-red requirement is worse than an honest bar); (C CHOSEN) split + revise to realistic bars, then verify with real timing evidence. This is requirement maintenance via a recorded Decision (not silent edit), consistent with capturing commitment changes as Decisions.";
        :>> consequences = "Amend n11FastStart (business.sysml) + sr11FastStart (requirements.sysml) statements to the two-bar form. Author sr11Verify (.tracking/verification, method=test) citing the live measurements (validate ~110ms under 500ms; orient ~0.85s under 1.5s) -> sr11 reports VERIFIED (D0082), and n11 verifies transitively, closing the last coverage gap with real evidence. D0083 is critiqued (dogfood, D0080/D0081 governed). KNOWN LIMITATION: sr11Verify's timing evidence isn't auto-rechecked on code change (no timing in the suspect manifest) — orient regression would not auto-flag; mitigated by the 1.5s headroom + this rationale. Pairs with D0045 (computed orient), D0050 (suspicion), D0079/D0082 (assurance/tiers), orientPerf (Sprint 74).";
    }

    verification d0083Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a083-5162738496a1";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0083 (revise FastStart: parse/validate under 500ms retained + full orient-with-suspicion under 1.5s; supersede the flat 500ms) by selecting 'Revise target to a realistic bar' in the sr11 close-out dialogue on 2026-06-21.";
    }
    part d0083AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a083-5162738496a2";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "324ab15";
        :>> judgedAt = "2026-06-21";
        :>> judgedBy = "wweatherholtz";
    }
}
