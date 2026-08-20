package Decision0038 {
    private import EngineWork::*;

    part d0038 : Decision {
        :>> id = "fc7c2db1-9f0a-4b1c-cde2-3a4b5c6d7e8f";
        :>> title = "sprint-estimation-model: Fibonacci points for estimates; actual hours for spend; sprint-planning + sprint-review skills added";
        :>> status = DecisionStatus::superseded;  // sprint cap superseded by D0044 (≤8 pts); estimation guidelines still apply
        :>> context = "Story.estimatedPoints existed but had no enforced scale or scale guidance. Story.actualPoints existed but was never populated. No sprint-planning or sprint-review skill existed to encode the estimation discipline. The project needed a consistent estimation model that enables velocity, efficiency, and accuracy trending without introducing a separate 'actual size' re-estimation step unless scope genuinely changed.";
        :>> decision = "Adopt a sprint estimation model: estimatedPoints uses Fibonacci scale only (1/2/3/5/8/13); add actualHours : Real[0..1] to Story for wall-clock spend recorded at closeOut; retain actualPoints for scope-change re-estimation only; add sprint-planning and sprint-review skills; update backlog-refinement with the Fibonacci table.";
        :>> rationale = "Story.estimatedPoints had no enforced scale or guidance and actualPoints was never populated; no sprint-planning/sprint-review skill encoded the discipline. The project needed a consistent model enabling velocity/efficiency/accuracy trending without a separate 'actual size' re-estimation step unless scope genuinely changed. Fibonacci hour-guidelines: 1=<2h, 2=3-5h, 3=5-10h, 5=~1day, 8=2-3days, 13+=split. sprint-planning covers orient->select->size->estimatedPoints->DoR; sprint-review covers record actualHours->compute velocity/efficiency/accuracy->surface retro items.";
        :>> consequences = "Story instances gain actualHours field; validate_schema + validate_instances must remain green. Velocity = trailing avg estimatedPoints/sprint. Efficiency = estimatedPoints/actualHours per sprint and trailing avg. Accuracy = qualitative review of actualHours vs Fibonacci scale guideline at retrospective. actualPoints recorded only when sprint scope shifted significantly. Sprint closeOut gate procedure should prompt for actualHours before recording the confirmation TestResult.";
    }
}
