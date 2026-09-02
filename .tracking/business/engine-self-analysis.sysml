// =====================================================================
// EngineSelfAnalysisNeeds — keel is a defined system, and its own control gaps are found by analysis.
//
// SOURCE. st065 (revamp verification: automatic tests self-verify, everything else defaults to human
// judgment on AI-proposed evidence in sets, by method kind - and a strong STPA on the keel workflow if
// we are going to be that cavalier with AI) and st066 (a defined system for keel itself is the first
// step: what authorities are present, how they interact, what data is sent), translated as us069,
// us070 and us071. Elicited direction: each Need carries #DerivedFrom to the story that implicated it.
// =====================================================================
package EngineSelfAnalysisNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    part nKeelIsADefinedSystem : Need {
        :>> id = "f6a7d8e9-1b2c-4d3e-8f90-a1b2c3d4e5f6";
        :>> title = "keel's own control structure is a defined, computed system - authorities, actions, data, feedback";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "The authorities that act on a keel project - the human, the agent, the hook boundary, the commit gate, CI, the decision channel, the remote, the console - and what each may issue on which process, carrying what data, with what feedback returning, are a DEFINED system: computed from the facts that already wire them (hook config, git hooks, workflow files, the CLI's command facts, the guard set) wherever a fact exists, and authored only where no fact can yield it (which processes are controlled, what each authority believes). MEASURABLE: the safety viewpoint renders the structure; every controller has computed actions or is reported inert; removing a hook event or a workflow step changes the view on the next run without anyone editing a model file. WHAT THIS EXCLUDES: it does not say the structure is SAFE - that is the analysis the next Need asks for; a defined system is its precondition.";
    }

    part nControlGapsFoundByAnalysis : Need {
        :>> id = "0718293a-4b5c-4d6e-9f01-b2c3d4e5f607";
        :>> title = "Control gaps in keel's workflow are found by a repeatable STPA, before an incident names them";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::should;
        :>> statement = "For every control action in the defined structure, the unsafe variants (provided, not provided, wrong timing or order, stopped too soon or applied too long) are considered and recorded, each traced to a standing control or to an Issue naming the gap; the analysis re-runs when the structure changes. MEASURABLE: a UCA exists for each control action or a recorded reason why none applies; every UCA has a controller constraint bound to a control-map entry or an open Issue; the run records the structure version it analysed. WHAT THIS EXCLUDES: it is not a safety case for the deliverable - the deliverable's domain never enters the engine; this is the engine analysing the process that builds it.";
    }

    part nVerificationByAuthorityAndMethod : Need {
        :>> id = "1829304b-5c6d-4e7f-a012-c3d4e5f60718";
        :>> title = "A passing verification means one of three things, by who judged and how - and never AI testimony alone";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "A pass is (a) a machine's: an automatic test ran and a party the agent cannot write to can re-derive it, and it satisfies its parent without asking anyone; (b) a human's judgment on evidence the AI proposed - a demonstration, an analysis, an inspection - presented in SETS sized for a person, each item still recorded individually; or (c) a human's own confirmation. The method kinds already in the vocabulary carry the distinction. MEASURABLE: the attestation census shows no AI-judged pass counted as verification unless it carries a receipt a third party re-derived; the human's per-sprint judgment load is reported in sets and in items; the 4954 standing AI-judged results are grandfathered by date, never re-judged. WHAT THIS EXCLUDES: it does not forbid the AI from RECORDING what it observed - a proposed result is a fact about what the agent saw; it forbids that record from COUNTING as verified until the tier says it may.";
    }

    #DerivedFrom dependency from nKeelIsADefinedSystem to us069;
    #DerivedFrom dependency from nControlGapsFoundByAnalysis to us071;
    #DerivedFrom dependency from nVerificationByAuthorityAndMethod to us070;
}
