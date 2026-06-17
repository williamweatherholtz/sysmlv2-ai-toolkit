package Decision0010 {
    private import EngineWork::*;

    part d0010 : Decision {
        :>> id = "95a5f2f6-9489-4d7d-8597-1c24500e0ef8";
        :>> title = "Keep agile ceremonies for solo+AI as discipline, not ritual";
        :>> status = DecisionStatus::accepted;
        :>> context = "Solo + AI development. Argued whether sprint ceremonies "
            + "are theater. They are not: AI drifts without strong short-term "
            + "clarity, and ceremonies are problem-solving processes that put a "
            + "human gate on medium/long-term direction.";
        :>> decision = "Keep the agile ceremonies (Refinement, Planning, Standup, Implementation, Retrospective, Staleness Sweep) for solo+AI as discipline; sprints are EVENT-DRIVEN, sized by a points budget (initial target 30-50, tuned from velocity).";
        :>> rationale = "AI drifts without strong short-term clarity, and ceremonies are problem-solving processes that put a human gate on medium/long-term direction. Standup is event-driven (per ~5 points), runs the Definition of Ready gate, and invokes grill-me on gaps; Retrospective promotes workflow learnings into Decisions.";
        :>> decisionText = "Keep Refinement, Planning, Standup, Implementation, "
            + "Retrospective, Staleness Sweep. Sprints are EVENT-DRIVEN, sized "
            + "by a points budget (initial target 30-50, tuned from velocity). "
            + "Standup is event-driven (per ~5 points), runs the Definition of "
            + "Ready gate, and invokes grill-me on gaps. Retrospective promotes "
            + "workflow learnings into Decisions.";
        :>> consequences = "Pros: strong short-term clarity for AI; user-gated "
            + "direction; self-improving workflow. Cons: ceremony cost — "
            + "accepted as less than the cost of drift/indecision.";
    }
}
