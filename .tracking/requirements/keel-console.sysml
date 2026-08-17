// =====================================================================
// ProjectConsoleRequirements — Architecture-phase SystemRequirements for the merged keel surface
// (D0152: the operator console IS the viewer). Derived from the five console Needs accepted at the
// Business gate 2026-08-16, plus N-17 which the merge makes binding on the console.
//
// EVERY THRESHOLD HERE IS ONE THAT CAN FAIL. A requirement whose test cannot distinguish pass from
// fail is a wish; the sprint-305 critique found exactly that defect in the serve-discipline set
// ("there shall be no code path that..." — a universal negative no test can demonstrate), and these
// are written to avoid repeating it. Where a property cannot be measured here — because this
// repository is the one place the engine and its deliverable coincide — that is stated on the
// requirement rather than papered over with a threshold that looks measurable and is not.
// =====================================================================
package ProjectConsoleRequirements {
    private import EngineElement::*;
    private import EngineRequirements::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    // ── N-C1: what is owed is discoverable without knowing to look for it ──────────────────────
    requirement srConsoleObligationsOnArrival : SystemRequirement {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000s01";
        :>> title = "Outstanding obligations are present on arrival, and their classes are declared";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> statement = "The surface shall present every outstanding obligation requiring human judgment on ARRIVAL, with no navigation action required to reach them. The set of obligation CLASSES shall be derived from declared sources rather than enumerated in surface code, so that a class introduced later appears without the surface being edited.";
        :>> rationale = "N-C1. The navigation threshold is what makes discoverability testable: zero actions, not 'few'. The derived-classes half is what makes it survive — an enumerated list would satisfy the requirement on the day it was written and silently stop being true the first time a class was added, which is precisely how the twelve tabs happened.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::demo;
    }

    // ── N-C2: the surface never misleads ────────────────────────────────────────────────────────
    requirement srConsoleViewStatusExplicit : SystemRequirement {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000s02";
        :>> title = "Every view carries an explicit status: computed-empty, computed, failed, or stale";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every view the surface renders shall carry an explicit status discriminant — COMPUTED-EMPTY, COMPUTED-WITH-CONTENT, FAILED with a reason, or STALE with the moment it was true — and shall render each of the four distinguishably. No render path shall be able to emit content without one.";
        :>> rationale = "N-C2. The discriminant is the mechanism: without it, 'empty' and 'failed' are the same absence of rows and the surface cannot help lying. Making it structurally required — no path emits without a status — is what prevents the property degrading later, and it is directly testable by forcing each of the four states.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    // ── N-C3: which model I am looking at is never in doubt ─────────────────────────────────────
    requirement srConsoleModelScopeResolved : SystemRequirement {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000s03";
        :>> title = "Every displayed item resolves to a model scope; the active scope is always named";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every item the surface displays shall resolve to exactly one model scope — ENGINE, DELIVERABLE, COINCIDENT, or UNSCOPED — and the scope in view shall be named on the surface at all times without interaction. An item whose scope cannot be determined shall display as UNSCOPED and shall never be assigned a scope by default.";
        :>> rationale = "N-C3. The never-default clause is the load-bearing one: a defaulted scope is a confident wrong answer, and this engine has produced that failure repeatedly in its own tooling. UNSCOPED is an admitted unknown, which a reader can act on. VERIFICATION LIMIT, stated rather than hidden: in THIS repository the engine and its deliverable coincide, so the ENGINE/DELIVERABLE distinction cannot be demonstrated here — only COINCIDENT and UNSCOPED can. Full verification requires a downstream project, the same limit issue117 recorded for the arch views.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::demo;
    }

    // ── N-C4: several projects, one surface ─────────────────────────────────────────────────────
    requirement srConsoleProjectRebind : SystemRequirement {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000s04";
        :>> title = "Projects switch in-surface without restart; the active project is always named";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> statement = "The surface shall bind a set of projects and switch the one in view without restarting the process and without editing configuration; the active project shall be visible at all times without interaction. A project whose model fails to parse shall report that and shall NOT fall back to the previously bound project.";
        :>> rationale = "N-C4. The no-silent-fallback clause is what keeps a switch honest: falling back leaves the reader believing they are looking at project B while seeing project A, which is the same class of defect as a defaulted model scope. Testable by pointing the surface at a deliberately broken project.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::should;
        :>> verificationMethod = VerificationMethod::demo;
    }

    // ── N-C5 + N-17: returning cold, and orientation cost that does not grow ────────────────────
    requirement srConsoleNavigationDerived : SystemRequirement {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000s05";
        :>> title = "Navigation is DERIVED from declared Viewpoints; no literal surface list exists";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> statement = "The surface's navigation shall be computed from the declared Viewpoint registry — concern, audience, sources, renderer — and no enumerated list of surfaces, tabs or sections shall exist anywhere in surface code. Declaring a Viewpoint shall make it reachable with no edit to the surface.";
        :>> rationale = "N-17 and N-C5. This is the requirement that condemns the current `const TABS=[...]` array at console.html:41 outright. Stated as a structural property rather than as 'there shall be no hardcoded list anywhere' — the sprint-305 critique showed a universal negative cannot be tested — so verification is: declare a new Viewpoint, observe it appear, and assert the surface source contains no enumerated surface list.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement srConsoleArrivalBounded : SystemRequirement {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000s06";
        :>> title = "Arrival choices are bounded by declared grouping, not by capability count";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> statement = "The number of top-level choices presented on arrival shall be a function of declared GROUPING and shall not increase when a Viewpoint is added. Adding N viewpoints to the registry shall leave the count of top-level choices unchanged.";
        :>> rationale = "N-C5. This is the measurable form of 'orientation cost does not grow as the model grows', and it is deliberately a DELTA rather than a cap: a fixed maximum would be arbitrary and would be gamed by nesting, whereas 'adding capability adds no top-level choice' is exactly the property that failed here — twelve tabs is the integral of one tab per endpoint. Directly testable by adding a viewpoint and counting.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    // ── satisfy: SR -> Need ─────────────────────────────────────────────────────────────────────
    satisfy nConsoleObligationDiscoverable by srConsoleObligationsOnArrival;
    satisfy nConsoleNeverMisleads by srConsoleViewStatusExplicit;
    satisfy nConsoleContextCertain by srConsoleModelScopeResolved;
    satisfy nConsoleMultiProject by srConsoleProjectRebind;
    satisfy nConsoleFitsIntermittentAttention by srConsoleNavigationDerived;
    satisfy nConsoleFitsIntermittentAttention by srConsoleArrivalBounded;
    satisfy nViewerGenerativeUi by srConsoleNavigationDerived;

    // ── #DependsOn: build order. The derived navigation is the foundation — obligations and scope
    //    are surfaces rendered through it, so building them first would hardcode what must be
    //    derived, which is how the twelve tabs were built.
    #DependsOn dependency from srConsoleObligationsOnArrival to srConsoleNavigationDerived;
    #DependsOn dependency from srConsoleArrivalBounded to srConsoleNavigationDerived;
    #DependsOn dependency from srConsoleModelScopeResolved to srConsoleProjectRebind;

    // ── #DerivedFrom: the shape Decision these requirements sit inside ─────────────────────────
    #DerivedFrom dependency from srConsoleNavigationDerived to d0152;
    #DerivedFrom dependency from srConsoleObligationsOnArrival to d0152;
}
