package Decision0007 {
    private import EngineWork::*;

    part d0007 : Decision {
        :>> id = "67a098fc-9431-4a12-b2fd-98b80b6d9cda";
        :>> title = "The project is a reusable work-tracking engine; two models";
        :>> status = DecisionStatus::accepted;
        :>> context = "Scope drifted toward an abstract 'modeling kernel with "
            + "pluggable HR/software domains'. That over-abstracts. The actual "
            + "intent: a work-tracking engine, reused across projects.";
        :>> decision = "This repo IS a reusable work-tracking engine (schema + processes + computed-state contract track WORK BEING DONE for any project); TWO MODELS are never conflated — the engine model tracks the work and the deliverable is a SEPARATE artifact whose domain vocabulary never enters the engine schema; reuse = template (clone, keep .engine, replace instance files); STPA/HARA-style analysis artifacts are FIRST-CLASS tracked items whose verification is computed analysis-completeness.";
        :>> rationale = "Scope had drifted toward an abstract 'modeling kernel with pluggable HR/software domains', which over-abstracts; the actual intent is a grounded work-tracking engine reused across projects, keeping safety analysis live via suspicion — at the cost of discipline to keep deliverable vocabulary out of the engine.";
        :>> consequences = "Pros: simple, grounded, reusable; safety analysis "
            + "stays live via suspicion. Cons: requires discipline to keep "
            + "deliverable vocabulary out of the engine.";
    }
}
