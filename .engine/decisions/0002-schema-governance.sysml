package Decision0002 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0002 : Decision {
        :>> id = "7ef57346-417a-4704-bc82-03c0c896837d";
        :>> title = "Schema governance: frozen core, optional safety module, .engine folder";
        :>> status = DecisionStatus::accepted;
        :>> context = "If contributors model 'task' three ways the typed-graph "
            + "value evaporates (convention drift). But the engine must also "
            + "be reusable and must not force safety vocabulary on non-safety "
            + "projects.";
        :>> decision = "The engine schema lives in a .engine/ dotfolder as infrastructure (like .git); schema/core is always imported and is frozen (changes require a Decision); schema/safety (STPA) is an OPTIONAL package import that non-safety projects omit; new types or modules are gated by a Decision.";
        :>> rationale = "If contributors model 'task' three ways the typed-graph value evaporates (convention drift), so core must be frozen; but the engine must stay reusable and not force safety vocabulary on non-safety projects, so safety is an optional import and growth is gated rather than freeform.";
        :>> consequences = "Pros: integrity preserved at every level; unbounded "
            + "growth without core churn; clean top-level. Cons: friction to "
            + "add a concept (must go through a Decision) — intentional.";
    }

    verification d0002Accept : Test { :>> id = "704295ea-6611-4213-b75b-3a3213f97f66"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0002 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0002AcceptR1 : TestResult { :>> id = "7af4cd27-c064-4cb9-ac8a-bfa3c1b28cce"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
