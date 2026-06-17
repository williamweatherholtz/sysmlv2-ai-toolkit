package Decision0037 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0037 : Decision {
        :>> id = "eb5a1caf-7d8e-4f9a-abc0-1e2f3a4b5c6d";
        :>> title = "authoredByRefs: validated-name approach; claudeSonnet46 added to ProjectActors; legacy names grandfathered";
        :>> status = DecisionStatus::accepted;
        :>> context = "authoredBy and judgedBy fields are String references to ProjectActors parts. Sprint 13 implements the validated-name approach (D0037) rather than typed SysML references, because native ref features to part instances across files do not work reliably in pilot 0.59.0 (reopened package scope issue). The validator (validate_actors.py) reads actors.sysml and cross-checks all authoredBy/judgedBy values. Early tracking entries (2026-06-10/11) used tool names (validate_schema, demo, user, inspect) as judgedBy — these are grandfathered as LEGACY_ACTORS (WARN not ERROR).";
        :>> decision = "Adopt the validated-name approach for authoredBy/judgedBy: add claudeSonnet46 to actors.sysml; validate_actors.py cross-checks all values against ProjectActors; legacy tool-names grandfathered as WARN; schema unchanged (authoredBy : String).";
        :>> rationale = "Native ref features to part instances across files do not work reliably in pilot 0.59.0 (reopened package scope issue), so typed SysML references are deferred until cross-file ref scoping is confirmed. validate_actors.py reads ProjectActors, reports unknown values as ERROR and legacy values as WARN; LEGACY_ACTORS set covers pre-convention names (validate_schema, demo, user, inspect) so no migration of old entries is needed. Going forward all new TestResult/Decision/Story authoredBy and judgedBy values MUST be in ProjectActors.";
        :>> consequences = "validate_actors.py exits 0 (PASS) with 52 legacy warnings. New entries with unknown actor names will fail. actors.sysml is the authoritative actor registry. The typed-ref approach is deferred until pilot cross-file ref scoping is confirmed.";
    }

    verification d0037Accept : Test { :>> id = "48c6e3c6-ae13-4c47-8c45-740cb884ae92"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0037 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0037AcceptR1 : TestResult { :>> id = "64666b54-fd89-4d16-a814-13fa499fc368"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
