package Decision0037 {
    private import EngineWork::*;

    part d0037 : Decision {
        :>> id = "eb5a1caf-7d8e-4f9a-abc0-1e2f3a4b5c6d";
        :>> title = "authoredByRefs: validated-name approach; claudeSonnet46 added to ProjectActors; legacy names grandfathered";
        :>> status = DecisionStatus::accepted;
        :>> context = "authoredBy and judgedBy fields are String references to ProjectActors parts. Sprint 13 implements the validated-name approach (D0037) rather than typed SysML references, because native ref features to part instances across files do not work reliably in pilot 0.59.0 (reopened package scope issue). The validator (validate_actors.py) reads actors.sysml and cross-checks all authoredBy/judgedBy values. Early tracking entries (2026-06-10/11) used tool names (validate_schema, demo, user, inspect) as judgedBy — these are grandfathered as LEGACY_ACTORS (WARN not ERROR).";
        :>> decisionText = "Adopted: validated-name approach. (1) claudeSonnet46 added to actors.sysml as `part claudeSonnet46 : Actor`. (2) validate_actors.py reads ProjectActors, scans tracking, reports unknown values as ERROR and legacy values as WARN. (3) LEGACY_ACTORS set covers pre-convention names; no migration of old entries. (4) Schema stays unchanged (authoredBy : String). Going forward, all new TestResult/Decision/Story authoredBy and judgedBy values MUST be in ProjectActors. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "validate_actors.py exits 0 (PASS) with 52 legacy warnings. New entries with unknown actor names will fail. actors.sysml is the authoritative actor registry. The typed-ref approach is deferred until pilot cross-file ref scoping is confirmed.";
    }
}
