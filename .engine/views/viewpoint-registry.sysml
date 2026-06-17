// =====================================================================
// EngineViewpoints — the formal VIEWPOINT registry (D0056/D0057; ISO 42010 / Arcadia
// / SysML v2). Each Viewpoint is a DECLARED lens: concern + audience + sources +
// renderer. Views are the computed application of a viewpoint (#View, regenerable) —
// the rendering stays in tooling (report.py / orient / query.py), now DRIVEN BY these
// declarations. Adding a view = declaring a Viewpoint here, not writing new code.
// Increment 1 (formalViewpointsSchema): declare the registry. Increment 2
// (formalViewpoints): rewire the renderers to read it + compute concern-coverage.
// =====================================================================
package EngineViewpoints {
    private import EngineElement::*;     // Element identity
    private import EngineComputed::*;    // Viewpoint type

    part orientVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-1619-405162738410";
        :>> title = "orient";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Where do things stand and what is ready to work next?";
        :>> audience = "maintainer / lead";
        :>> sources = "backlog action defs + latest TestResults + succession DAG + delivery-file ceremony gates";
        :>> renderer = "sysmlv2 orient / query.py orient";
    }
    part whatsNextVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-2619-405162738411";
        :>> title = "whats-next";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which outstanding items are unblocked, in priority order?";
        :>> audience = "maintainer / AI executor";
        :>> sources = "ready frontier (outstanding tasks with all deps done), ranked by backlog declaration order (D0052)";
        :>> renderer = "sysmlv2 whats-next / query.py";
    }
    part traceabilityVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-3619-405162738412";
        :>> title = "traceability";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Are needs->requirements->verification linked, and where are the gaps / subversions?";
        :>> audience = "reviewer / auditor";
        :>> sources = "satisfy + verify edges; sprint ceremony gate coverage; issue->decision links; process-evolution timeline";
        :>> renderer = "report.py:tab_traceability / query.py trace";
    }
    part safetyVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-4619-405162738413";
        :>> title = "safety";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Are safety-relevant losses/hazards/UCAs identified and constrained, and is each constraint verified?";
        :>> audience = "safety engineer";
        :>> sources = "STPA losses/hazards/UCAs/constraints (EngineSafety); safety-kind requirements; verify edges to their Tests";
        :>> renderer = "(increment 2) report.py safety tab from STPA + safety reqs + verify coverage";
    }
    part requirementsVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-5619-405162738414";
        :>> title = "requirements";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "What does the system need to do, are requirements well-formed, and is each satisfied + verified?";
        :>> audience = "systems engineer / requirements owner";
        :>> sources = "Need / Requirement items; satisfy edges (to components) + verify edges (to Tests); requirement-quality (INCOSE/EARS)";
        :>> renderer = "(increment 2) report.py requirements tab from requirements + satisfy/verify coverage";
    }
    part decisionsVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-6619-405162738415";
        :>> title = "decisions";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "What was decided, by whom, when, and why (incl. alternatives)?";
        :>> audience = "lead / auditor / future maintainer";
        :>> sources = ".engine/decisions/*.sysml Decision items (context/decisionText/consequences; alternatives via decisionAnalysis/issue016)";
        :>> renderer = "report.py:tab_decisions";
    }
    part baselinesVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-7619-405162738416";
        :>> title = "baselines";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "What was the state of the model at a given release/commit?";
        :>> audience = "release manager / auditor";
        :>> sources = "Release (commit SHA + member refs); baseline view computed from git + authored facts";
        :>> renderer = "report.py:tab_baselines";
    }
    part suspectVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-8619-405162738417";
        :>> title = "suspect";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which done items have stale evidence (model OR deliverable-source drift) and need re-verification?";
        :>> audience = "maintainer / QA";
        :>> sources = "git-ancestry of judgedAgainst vs upstream change over satisfy/:> edges; deliverable-manifest source drift (D0050)";
        :>> renderer = "sysmlv2 orient (suspect) / query.py suspect";
    }
    part orphansVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-9619-405162738418";
        :>> title = "orphans";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which model elements are orphaned / dangling — a Need with no satisfying Requirement, a Requirement with no satisfy or verify edge, a task with no DoD, an Issue with no relatedTask, a Decision referenced by nothing — i.e. broken or missing required traceability?";
        :>> audience = "maintainer / auditor / reviewer";
        :>> sources = "all items + typed edges; flag any element missing a REQUIRED incoming/outgoing edge for its kind (Need->Requirement satisfy; Requirement->Test verify; task->DoD; Issue->relatedTask)";
        :>> renderer = "(increment 2) report.py orphans view / query.py orphans";
    }
}
