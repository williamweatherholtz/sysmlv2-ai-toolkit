package Decision0033 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0033 : Decision {
        :>> id = "ad388dbe-2f3a-4c4b-def5-6a7b8c9d0e1f";
        :>> title = "S11 review doc-sync: occurrence def + state def body gap in sysmlv2-syntax-notes.md";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 11 review (doc-sync step). Two documentation gaps after Sprint 11: (1) sysmlv2-syntax-notes.md had no entry confirming occurrence def with attributes works in pilot 0.59.0. (2) sysmlv2-syntax-notes.md had no entry documenting that state def body syntax (entry state / then transitions) fails in pilot 0.59.0.";
        :>> decision = "Applied as Sprint 11 review doc-sync to sysmlv2-syntax-notes.md: (1) added an 'occurrence def — confirmed Sprint 11 (D0032)' section (occurrence def :> Element with attributes parses; part instances of occurrence def parse unchanged; TestResult retype rationale noted); (2) added a 'state def body syntax — NOT supported in pilot 0.59.0 (D0031)' section (bare state def X; parses but body syntax fails; transitions stay String[*]; toolchainWatch tracks the revisit).";
        :>> rationale = "Two documentation gaps remained after the Sprint 11 spikes; the syntax notes had to record both the confirmed occurrence def capability and the confirmed state def body gap so authors know what is safe to use and what is deferred. Accepted: wweatherholtz standing instruction (continue through sprints), 2026-06-15.";
        :>> consequences = "sysmlv2-syntax-notes.md accurately reflects the Sprint 11 spike findings. Authors know occurrence def is safe to use and state def body transitions are deferred.";
    }

    verification d0033Accept : Test { :>> id = "ca59c717-b456-4d9f-9aa2-0e34385f82f2"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0033 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0033AcceptR1 : TestResult { :>> id = "424d3739-c8f3-411b-b398-f091442bbf0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
