package Decision0033 {
    private import EngineWork::*;

    part d0033 : Decision {
        :>> id = "ad388dbe-2f3a-4c4b-def5-6a7b8c9d0e1f";
        :>> title = "S11 review doc-sync: occurrence def + state def body gap in sysmlv2-syntax-notes.md";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 11 review (doc-sync step). Two documentation gaps after Sprint 11: (1) sysmlv2-syntax-notes.md had no entry confirming occurrence def with attributes works in pilot 0.59.0. (2) sysmlv2-syntax-notes.md had no entry documenting that state def body syntax (entry state / then transitions) fails in pilot 0.59.0.";
        :>> decisionText = "Applied as Sprint 11 review doc-sync. (1) Added 'occurrence def — confirmed Sprint 11 (D0032)' section: occurrence def :> Element with attributes parses; part instances of occurrence def parse unchanged; TestResult retype rationale noted. (2) Added 'state def body syntax — NOT supported in pilot 0.59.0 (D0031)' section: bare state def X; parses but body syntax fails; transitions stay String[*]; toolchainWatch tracks the revisit. Accepted: wweatherholtz standing instruction (continue through sprints), 2026-06-15.";
        :>> consequences = "sysmlv2-syntax-notes.md accurately reflects the Sprint 11 spike findings. Authors know occurrence def is safe to use and state def body transitions are deferred.";
    }
}
