package Decision0022 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0022 : Decision {
        :>> id = "d0e1f2a3-b4c5-4d6e-7f8a-9b0c1d2e3f4b";
        :>> title = "S6 review docSync: orient Rust alternative + VerdictKind naming + DoDR naming convention";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 6 review phase identified three process improvements: (1) CLAUDE.md and .engine/docs/usage.md referenced only query.py orient; the new sysmlv2 orient [ROOT] Rust-native command was undocumented. (2) usage.md described TestResult naming as <task>R<n> — ambiguous; the DoD-linked naming is <task>DoDR<n> (the DoDR infix is required by orient_root for done-detection). (3) sysmlv2-syntax-notes.md had no entry for VerdictKind::pass (the correct outcome enum); an agent artifact (orient_bdd.rs) initially used TestOutcome::pass (wrong), caught only at test-write time — a documented convention would prevent recurrence.";
        :>> decision = "Apply Sprint 6 review doc-sync: (1) update CLAUDE.md §1 and §4 to add 'or Rust-native: sysmlv2 orient [ROOT]' alongside the query.py command; (2) update usage.md day-to-day table — 'See what's next' mentions sysmlv2 orient, 'Mark done' clarified to use <task>DoDR<n> naming and VerdictKind::pass, plus a new 'Record a standalone result' row for the phase-gate R<n> pattern; (3) add a 'TestResult naming and enum conventions' section to sysmlv2-syntax-notes.md documenting the DoDR<n> infix rule, VerdictKind enum name, and required fields.";
        :>> rationale = "The Rust-native orient command was undocumented, TestResult naming in usage.md was ambiguous, and the correct VerdictKind::pass enum had no documented convention — an agent artifact had used the wrong TestOutcome::pass, caught only at test-write time, so a documented convention prevents recurrence. sysmlv2-syntax-notes.md is the source of ground truth for authoring .sysml. Accepted: wweatherholtz standing instruction (fix internal processes in each sprint review), 2026-06-15.";
        :>> consequences = "Any future agent authoring TestResults has the correct VerdictKind::pass enum and DoDR<n> naming documented in sysmlv2-syntax-notes.md — the source of ground truth for authoring .sysml. orient_root done-detection and usage.md conventions are now consistent. CLAUDE.md mentions both orient paths so agents can use the Rust binary without the Jupyter kernel.";
    }

    verification d0022Accept : Test { :>> id = "cfcffca5-826b-4e68-90ec-1e5e918add5b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0022 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0022AcceptR1 : TestResult { :>> id = "8c889021-2189-407c-a494-51a15b2f803f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
