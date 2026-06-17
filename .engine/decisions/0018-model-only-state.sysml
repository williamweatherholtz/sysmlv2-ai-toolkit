package Decision0018 {
    private import EngineElement::*;
    private import EngineWork::*;

    part d0018 : Decision {
        :>> id = "7c4a1e9d-8b2f-4e6a-9d3c-5f1e2a7b8c9d";
        :>> title = "Model-only state: no prose status/handoff documents, ever";
        :>> status = DecisionStatus::accepted;
        :>> context = "RESUME.md persisted as a prose handoff doc duplicating the backlog, "
            + "the docs, and computed state. The 2026-06-11 critique flagged it (finding A7: "
            + "shadow tracker); it was thinned, then REGREW in the very handoff commit that "
            + "closed the critique CRs - prose state is structurally attractive and must be "
            + "banned, not trimmed. The user called the split architecture: state must exist "
            + "as downstream work items in the SysML-backed workflow, 100 percent.";
        :>> decision = "RESUME.md is DELETED and the prose status/handoff pattern is banned; where things stand is COMPUTED (query.py orient), outstanding work exists ONLY as backlog work items with DoDs, and durable knowledge lives in CLAUDE.md, .tracking/README + .engine/docs, and .engine/decisions. Anything a resume requires that fits none of those becomes a work item or a Decision.";
        :>> rationale = "RESUME.md duplicated the backlog, the docs, and computed state; the 2026-06-11 critique flagged it (finding A7: shadow tracker), it was thinned, then REGREW in the very handoff commit that closed the critique CRs - prose state is structurally attractive and must be banned, not trimmed. One source of truth means fresh contexts orient by computing, not reading prose. The user called the split architecture (state as downstream work items, 100 percent). Accepted by user 2026-06-11 ('resume.md should be superseded - state should exist as downstream work items... hard shift to the sysmlv2 backed workflow. 100 percent').";
        :>> consequences = "One source of truth; fresh contexts orient by computing, not "
            + "reading prose; cost: orient requires the kernel toolchain until runtimeParser "
            + "lands (mitigated: the backlog text itself is human-readable).";
    }
}
