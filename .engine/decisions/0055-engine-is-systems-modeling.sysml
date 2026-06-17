// =====================================================================
// D0055 — engine-is-systems-modeling: correct D0054's framing. The engine IS
// systems modeling (of the work/project/process as system-of-interest); the real
// boundary is the engine<->deliverable two-model separation, not "not a system".
// Reclassifies the three "deliverable-domain" patterns as engine-relevant; clarifies
// issue017. Accepted 2026-06-16 by wweatherholtz (challenge: "defend that what we're
// making isn't modeling a system, just tracking work").
// =====================================================================
package Decision0055 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0055 : Decision {
        :>> id = "4f506172-8394-4a41-376f-1324a5b6c7e9";
        :>> title = "D0055: the engine IS systems modeling (of the work/project); correct D0054's framing + clarify issue017";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0054 dismissed variation/MoE/viewpoint patterns as 'deliverable-domain, not engine-core' on the premise that the engine 'just tracks work' rather than modeling a system. The human challenged this. On reflection the premise is wrong: needs -> requirements -> architecture -> V&V -> decisions -> baselines with typed traceability IS the systems-engineering spine, and the engine applies it to the engineering EFFORT/PROJECT itself (and the engine is also a software system being built). It is doing systems modeling.";
        :>> decision = "CORRECTED framing: the engine IS systems modeling; its system-of-interest is the work/project/process (and recursively the engine software). The defensible architectural boundary is NOT 'not a system' but the TWO-MODEL separation (CLAUDE.md §1): the engine models the work/process while the DELIVERABLE's internal domain vocabulary stays in the deliverable's own model.";
        :>> rationale = "The two-model separation keeps the engine domain-reusable. RECLASSIFIED the three patterns D0054 dismissed as engine-RELEVANT (at the work/process layer), to revisit: (1) viewpoints — we already have informal views (orient, whats-next, traceability tab, dashboard); formal viewpoint typing is a candidate; (2) measures of effectiveness/parametrics — our GQM metrics (gate-coverage, parity, friction) ARE process MoE; (3) variants — our Decisions-with-alternatives (issue016) + chosen variants (kernel vs Rust, D0048) are the option mechanism. CLARIFIED issue017: the metadata-for-cross-cutting insight splits into (a) AUTHORED attributes (id/title/createdAt/authoredBy) = the @Tracked migration already deferred to trackedMetadataReplan (D0053), and (b) COMPUTED concerns (status, suspicion) = REJECTED as metadata — they must stay computed views (§2.1); storing them would violate the core invariant. So issue017 nets to 'subsumed by D0053 + reject the anti-invariant half'. The only standalone cheap enrichment from D0054 is issue016 (structured decision alternatives/criteria).";
        :>> consequences = "D0054's 'deliverable-domain dismissal' clause is superseded by this reclassification. The three patterns are tracked as engine-relevant-to-revisit (not filed as active backlog yet — lower priority than current frontier). issue017 reframed to subsumed-by-D0053 + rejected-computed-half. The two-model separation (CLAUDE.md §1) is reaffirmed as the real boundary. No change to the computed-views invariant — it is, in fact, reinforced (status/suspicion stay computed).";
    }

    verification d0055Accept : Test { :>> id = "a7bc0e12-5703-43ff-97d3-86f563def6a9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0055 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0055AcceptR1 : TestResult { :>> id = "05e62321-4cf2-42b0-a876-6ec6fb259cef"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
