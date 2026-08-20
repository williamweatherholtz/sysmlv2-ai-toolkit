// =====================================================================
// D0046 — audit-framework-cae-gqm-atam: adopt Claims-Arguments-Evidence +
// GQM + ATAM as the engine's standard audit framework; accept four
// improvement workstreams surfaced by the 2026-06-16 process step-back.
// Accepted 2026-06-16 by wweatherholtz (AskUserQuestion disposition).
// =====================================================================
package Decision0046 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0046 : Decision {
        :>> id = "c5e8a1f2-3b4d-4e6a-9c7f-8d0b1a2c3e4f";
        :>> title = "D0046: adopt CAE+GQM+ATAM audit framework; accept 4 process-improvement workstreams";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "A process step-back (2026-06-16) asked whether we have an antagonistic comprehensive audit process, whether processes are solid, what unknown/known-unknown processes are missing, and whether traceability holds for both engine and deliverable. Empirical diagnostics surfaced: (1) 175 orphaned SysML kernel JVMs (kernel-leak from the JVM validation path); (2) the Rust toolchain (rustS0-S9, marked 'done'/'parity verified') has silently REGRESSED — `sysmlv2 validate .` fails to preload schema (unresolved TestResult), `sysmlv2 orient .` still emits the retired cursor, pollutes the ready-list with raw Test-definition fragments, and reports done=185/outstanding=12 vs the Python authority's 81/2; (3) suspicion watches .sysml authored facts but NOT the deliverable's Rust source tree, so code drift never invalidates the TestResults that claim the code works; (4) the architectural-critique PROCESS exists but has no operationalizing SKILL (violates D0040), records findings as a prose doc rather than tracked Issues/Decisions, and has no objective metrics; (5) delivery skills for build/compile and test/verify are missing (only repo-push exists). User requested a modern, objective-oriented audit framework, explicitly NOT ASPICE/ISO maturity models unless very defensible.";
        :>> decision = "ADOPTED audit framework = Claims-Arguments-Evidence (assurance-case / GSN-style Goal->Strategy->Evidence) as backbone, layered with GQM (Goal-Question-Metric) for objective process-health metrics and ATAM scenario walkthroughs for architecture risk; ACCEPTED four improvement workstreams W1 rustToolchainFix, W2 auditSkill, W3 deliverySkills, W4 deliverableSuspicion (W1/W3 parallel-ready, W2/W4 gate on W1).";
        :>> rationale = "The engine ALREADY IS a CAE machine (Requirement=claim, satisfy/verify edges=argument, TestResult=evidence), so auditing it with CAE dogfoods its own model. ASPICE/ISO-15504 maturity levels REJECTED as doc-theater that measures conformance to a generic model rather than computed evidence. W1 repairs sysmlv2 validate/orient parity with the Python authority + adds a parity gate + makes the no-kernel Rust path the default (kills the JVM-leak at root); W2 creates+registers an architectural-critique skill operationalizing CAE+GQM+ATAM and records findings as tracked Issues/Decisions; W3 adds build/compile + test/verify skills alongside repo-push; W4 extends suspicion so Rust source changes invalidate the parity TestResults.";
        :>> consequences = "Critique process gains a CAE+GQM+ATAM structure and a registered skill; findings become tracked model facts, not prose. The no-kernel Rust path becomes the default once W1 lands, retiring the JVM-leak (CLAUDE.md S5 to be updated in W1). Suspicion extends to the deliverable source tree (W4). Four new backlog work items added under NextWork with DoDs + successions. Each workstream still validates green + commits CR: + (for sprints) closes via confirmation gate. This Decision is the human acceptance for the DIRECTION; per-workstream review/closeout sign-offs are recorded separately.";
    }

    verification d0046Accept : Test { :>> id = "62929618-c87e-4879-9ac6-60775b538e99"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0046 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0046AcceptR1 : TestResult { :>> id = "0366db5d-c6d2-4738-8072-17479d506a39"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
