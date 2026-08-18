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
    private import EngineVerification::*;   // Test / TestResult: the human confirmation below

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

    // UC-C7. The DESTINATION is derived from the same declaration as the COUNT, so this cannot drift into
    // a hardcoded map of class-to-page - the failure nServeReuseModel forbids and VIEW_SUBCOMMANDS already
    // demonstrates in a neighbouring control.
    requirement srConsoleObligationActionable : SystemRequirement {
        :>> id = "d3103651-b3fc-44d8-89e9-52e903a74c07";
        :>> title = "An obligation count is a link to the view that produced it";
        :>> createdAt = "2026-08-17"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every obligation class the surface reports shall carry the IDENTITY of the viewpoint whose renderer computed its count, and selecting the class shall open that viewpoint - selecting its surface as well, so the navigation agrees with what is rendered. A class with no resolvable destination shall carry no affordance rather than an inert one. A destination whose renderer names a view the server cannot compute shall report that, naming the command to run, and shall not render as an empty result. The binding shall be by identity carried on the element, never by the position of a card in a rendered list.";
        :>> rationale = "Two things forced the last clause. The count and the destination must come from ONE declaration or they drift: the count is already read from the view the renderer names, so the destination is free. And the previous arrival view bound clicks by DOM index over every element of class `.c`, while the obligation bar prepends four such cards into the same container - so every viewpoint card was off by four and an obligation card selected an arbitrary viewpoint (issue141). Identity binding cannot go off-by-N, and delegation means a render path added later inherits the behaviour instead of silently producing dead cards.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::demo;
    }
    satisfy nConsoleObligationDiscoverable by srConsoleObligationActionable;

    // ── Human confirmation that the click-through lands where the work is (D0016/D0066: a
    //    method=confirmation result records a HUMAN's word and is never inferred). This one is
    //    NARROW on purpose: it attests the behaviour they exercised, not the requirement's every clause.
    verification srConsoleObligationActionableConfirm : Test {
        :>> id = "e4b8207c-93a1-4d56-8f02-71cae35b9d68";
        :>> title = "Obligation cards take the reader to the work - confirmed in the live console";
        :>> createdAt = "2026-08-17";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz, 2026-08-17, verbatim: 'confirmed otherwise'. Given in a message whose first half challenged a separate claim of mine (that no automated check could read a browser - it could, and that is corrected in issue141 and sprint 339), with 'otherwise' confirming the rest: that clicking the cards now takes them to the right place to act. Exercised against the running console at 127.0.0.1:7777 on the build landed at 844b4ca, where all four act cards carry a resolvable viewpoint identity and their views compute. SCOPE OF THE ATTESTATION, stated so it is not read wider than it is: it confirms the OBSERVED behaviour - a card click lands on the view its renderer names. It does NOT attest the requirement's every clause; specifically the no-affordance-without-a-target case and the uncomputable-destination case were verified by me by construction and API probe, not by them, and the generic panel remains READ-ONLY for authority-queue and sitting-coverage, so for those two classes they can see the work and still discharge it elsewhere.";
    }
    part srConsoleObligationActionableConfirmR1 : TestResult {
        :>> id = "b1f79d3a-6c84-4e07-95d2-380ae6c14b7f";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "PENDING";
        :>> judgedAt = "2026-08-17";
        :>> judgedBy = "wweatherholtz";
    }
    #Verify dependency from srConsoleObligationActionableConfirm to srConsoleObligationActionable;

    // ── THE CONSOLE HAS NO BEHAVIOURAL COVERAGE, and D0160 obliges saying so here rather than leaving it
    //    implicit. This is a stated LIMIT on every console requirement above, not a requirement itself.
    //    One structural test guards the card binding (console_binds_card_targets_by_identity_not_position,
    //    a TEXT assertion over the embedded asset); nothing exercises the console's behaviour. So a wrong
    //    destination, a dead control or a broken render reaches the human before it reaches a gate — which
    //    is how both of this week's console defects were found. Accepted risk, trigger named in D0160:
    //    a third such defect, or a DOM-capable option that needs no Node toolchain.
    // ── #DerivedFrom: the shape Decision these requirements sit inside ─────────────────────────
    #DerivedFrom dependency from srConsoleNavigationDerived to d0152;
    #DerivedFrom dependency from srConsoleObligationsOnArrival to d0152;
    #DerivedFrom dependency from srConsoleObligationActionable to d0152;
}
