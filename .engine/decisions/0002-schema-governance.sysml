package Decision0002 {
    private import EngineWork::*;

    part d0002 : Decision {
        :>> id = "7ef57346-417a-4704-bc82-03c0c896837d";
        :>> title = "Schema governance: frozen core, optional safety module, .engine folder";
        :>> status = DecisionStatus::accepted;
        :>> context = "If contributors model 'task' three ways the typed-graph "
            + "value evaporates (convention drift). But the engine must also "
            + "be reusable and must not force safety vocabulary on non-safety "
            + "projects.";
        :>> decisionText = "The engine schema lives in a .engine/ dotfolder as "
            + "infrastructure (like .git). schema/core is always imported and "
            + "is frozen — changes require a Decision. schema/safety (STPA) is "
            + "an OPTIONAL import, via plain SysMLv2 package import; non-safety "
            + "projects simply omit it. New types or modules are gated by a "
            + "Decision.";
        :>> consequences = "Pros: integrity preserved at every level; unbounded "
            + "growth without core churn; clean top-level. Cons: friction to "
            + "add a concept (must go through a Decision) — intentional.";
    }
}
