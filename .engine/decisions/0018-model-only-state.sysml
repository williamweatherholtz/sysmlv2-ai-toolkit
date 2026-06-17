package Decision0018 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

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

    verification d0018Accept : Test { :>> id = "f00d2120-6c5b-459c-9a70-da9d9a93ee41"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0018 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0018AcceptR1 : TestResult { :>> id = "fb662570-0c57-4cd1-9e8a-5d7b0ead18d5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
