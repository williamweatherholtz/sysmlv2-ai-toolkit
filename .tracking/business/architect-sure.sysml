// =====================================================================
// ProjectArchitectSure — interaction USE CASES + new SystemRequirements for "Architect Sure" (the keel
// viewer), fleshed out from the human's 2026-07-16 direction: right-click focus, undo/redo, search-first
// discovery with single-type filter, connection settings, and a RICHER authoring surface (create / edit /
// connect / critique — not just "new decision"). UseCases capture intent; the new SRs satisfy the existing
// viewer Needs (nViewerExplorableGraph navigation; nViewerInProgramEdit authoring). Governed by D0126.
// =====================================================================
package ProjectArchitectSure {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    // ── interaction use cases ────────────────────────────────────────
    use case ucAsFocusFromGraph : UseCase {
        :>> id = "0a5c0001-0114-4c8b-9d2e-000000000c01";
        :>> title = "Focus an element from the graph (right-click)";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "While reading the graph, the user right-clicks a node and picks Focus to re-anchor the view on it (its slice becomes the new focus) — without leaving the canvas for the register.";
        :>> mainSuccess = "Right-click a node -> context menu -> Focus -> the graph re-loads the slice around that element and the detail sheet updates.";
        :>> extensions = "The same context menu offers the other per-element actions (expand, critique, edit, connect, supersede) gated on what keel's write contract advertises.";
    }
    use case ucAsNavHistory : UseCase {
        :>> id = "0a5c0002-0114-4c8b-9d2e-000000000c02";
        :>> title = "Undo / redo navigation";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user moves through several focuses while exploring and wants to step back to a prior view (and forward again) without re-searching.";
        :>> mainSuccess = "Undo restores the previous focus/graph state; redo re-applies it; the controls disable at the ends of the history.";
        :>> extensions = "Keyboard shortcuts (Ctrl+Z / Ctrl+Shift+Z) mirror the buttons.";
    }
    use case ucAsFindElement : UseCase {
        :>> id = "0a5c0003-0114-4c8b-9d2e-000000000c03";
        :>> title = "Find an element by search (not a full list)";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user types part of an element's name to find a starting point. With thousands of items and a graph as the primary surface, the finder returns MATCHES to a query — it does not dump the whole model into the pane.";
        :>> mainSuccess = "Type in Find -> a short ranked list of matching elements appears -> pick one to focus. Empty query shows guidance, not the entire index.";
        :>> extensions = "Results are bounded; refine the query to narrow. A type constraint (below) further scopes results.";
    }
    use case ucAsFilterOneType : UseCase {
        :>> id = "0a5c0004-0114-4c8b-9d2e-000000000c04";
        :>> title = "Constrain the finder to a single element type";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user wants to see ONLY one type (e.g. only Decisions) rather than toggling every other type off one by one.";
        :>> mainSuccess = "Right-click a type chip -> 'Only this type' -> the finder is constrained to that type; a clear/all control restores the full set.";
        :>> extensions = "Left-click still toggles a single type on/off; right-click is the fast 'solo' path.";
    }
    use case ucAsSettings : UseCase {
        :>> id = "0a5c0005-0114-4c8b-9d2e-000000000c05";
        :>> title = "Configure the keel connection under Settings";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The keel endpoint address + connect controls are configuration, not primary chrome; the user opens Settings to change them, keeping the top bar focused on the model.";
        :>> mainSuccess = "Open Settings -> edit the keel API address -> connect -> status reflects the connection; the setting persists for the session.";
        :>> extensions = "Connection status stays visible (a small indicator) even when Settings is closed.";
    }
    use case ucAsCreateItem : UseCase {
        :>> id = "0a5c0006-0114-4c8b-9d2e-000000000c06";
        :>> title = "Create a new item of a chosen type";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user creates a new element (a Need, Issue, etc.) of a declared type, through keel's guarded write path — not only Decisions.";
        :>> mainSuccess = "Pick a type -> fill its declared attributes -> keel records the item via the write API; inline guard feedback confirms it is well-formed; never auto-committed.";
        :>> extensions = "If a type has no create endpoint yet, the action is not offered (generative — gated on the write contract).";
    }
    use case ucAsEditItem : UseCase {
        :>> id = "0a5c0007-0114-4c8b-9d2e-000000000c07";
        :>> title = "Edit an item's attributes (through process)";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user changes an element's content through the disciplined path (a superseding Decision, or the item's owner-of-record process), never a raw field overwrite.";
        :>> mainSuccess = "Choose Edit -> the change is recorded through the appropriate keel process with provenance; inline guards run; the graph/sheet reflect the change.";
        :>> extensions = "For a frozen/owned item, the surface guides the user to the correct process (supersede) rather than allowing an in-place overwrite.";
    }
    use case ucAsConnectItems : UseCase {
        :>> id = "0a5c0008-0114-4c8b-9d2e-000000000c08";
        :>> title = "Connect two items with a typed edge";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user draws a typed relationship (satisfy, verify, dependency, derive, supersede, …) between two elements through keel's guarded edge-write API.";
        :>> mainSuccess = "Pick a source + target + edge kind (from the model's edge vocabulary) -> keel appends the edge via /api/edge; inline guards run; the graph shows the new edge.";
        :>> extensions = "Only edge kinds keel permits are offered; an invalid pairing is refused inline.";
    }
    use case ucAsCritiqueElement : UseCase {
        :>> id = "0a5c0009-0114-4c8b-9d2e-000000000c09";
        :>> title = "Critique an element with AI";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "From an element (or the graph), the user runs an antagonistic AI critique along a chosen lens; findings land as tracked critiques/Issues.";
        :>> mainSuccess = "Choose Critique on an element -> keel's agent bridge runs the lens -> findings stream in and record as tracked artifacts (no second store).";
        :>> extensions = "If the agent bridge is off, the action explains how to enable it and degrades gracefully.";
    }
    use case ucAsDispositionFinding : UseCase {
        :>> id = "0a5c000a-0114-4c8b-9d2e-000000000c0a";
        :>> title = "Disposition a critique finding";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> description = "The user records a typed verdict (act / accept-risk / dismiss) on a finding, with rationale, as a recorded judgment.";
        :>> mainSuccess = "Open a finding -> choose a disposition + rationale -> keel records it via the write API; an ACT verdict also attaches a resolver.";
        :>> extensions = "A >=Medium finding without a disposition stays visibly open.";
    }

    // ── new SystemRequirements (satisfy existing viewer Needs) ───────
    requirement srViewerContextActions : SystemRequirement {
        :>> id = "0a5b0001-0114-4c8b-9d2e-000000000b01";
        :>> title = "Per-element context actions (right-click) — generated from the write contract";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> statement = "The graph (and register rows) shall expose an element's actions via a right-click context menu — Focus and Expand always; Critique, Edit, Connect, Supersede, Create when keel advertises the corresponding endpoint. The action set is GENERATED from the version/write contract, not hardcoded.";
        :>> rationale = "The human directed (2026-07-16): interact with elements directly ('right-click ... Focus'); surface the many actions, not just 'new decision'. A context menu keeps actions where the element is.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerNavHistory : SystemRequirement {
        :>> id = "0a5b0002-0114-4c8b-9d2e-000000000b02";
        :>> title = "Reversible navigation (undo / redo of focus + view state)";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> statement = "The viewer shall maintain a navigation history and provide undo/redo (buttons + Ctrl+Z / Ctrl+Shift+Z) over focus/graph state, disabling each control at its end of the history. (Model WRITES are reversed through keel processes — supersede / git — not this client history.)";
        :>> rationale = "The human directed (2026-07-16): 'an undo/redo option in the UI'. Reversible exploration lowers the cost of a wrong turn — a core usability floor.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerSearchDiscovery : SystemRequirement {
        :>> id = "0a5b0003-0114-4c8b-9d2e-000000000b03";
        :>> title = "Search-first discovery (type-ahead, single-type constrainable, not a full dump)";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> statement = "Discovery shall be SEARCH-FIRST: a query over displayLabel + id returns a bounded, ranked match list; the pane shall NOT list the entire model by default (the graph is the primary surface). The user shall be able to constrain results to a SINGLE type in one action (solo), in addition to toggling types.";
        :>> rationale = "The human directed (2026-07-16): 'don't really want all elements viewed on the left pane ... no need for the list when we have a graph'; and single-type select instead of disabling types one by one.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerAuthoringSurface : SystemRequirement {
        :>> id = "0a5b0004-0114-4c8b-9d2e-000000000b04";
        :>> title = "Full authoring surface (create / edit / connect / critique / disposition) — beyond decisions";
        :>> createdAt = "2026-07-16"; :>> createdBy = "wweatherholtz";
        :>> statement = "The viewer shall surface keel's full disciplined authoring set — create an item of a declared type, edit attributes through process, author a typed edge, critique, disposition — not only 'propose a Decision'. Every action goes through the guarded write API with inline guard feedback, is generative (offered iff the endpoint is advertised), and never auto-commits.";
        :>> rationale = "The human directed (2026-07-16): 'i want to be able to edit, make connections, make new items, critique with AI, not just new decision'. The write surface must span the real use cases, all process-bound (D0106).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    // ── scope-aware authoring + system-bound analysis (human direction 2026-07-17) ──
    use case ucAsCreateWithLinkage : UseCase {
        :>> id = "0a5c000b-0114-4c8b-9d2e-000000000c0b";
        :>> title = "Create an item together with its traceability edges";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> description = "Creating an item is only useful if it is CONNECTED. The user creates a new element and, in the same flow, links it upstream and/or downstream to existing items via typed edges — so it lands traceable, not orphaned.";
        :>> mainSuccess = "New-item flow offers 'connect to' targets (upstream + downstream) with the edge kinds valid for the pair; on create, keel authors the item + the edges via /api/item + /api/edge; guards run inline.";
        :>> extensions = "If a required edge is missing, /api/check flags the orphan/rootedness obligation inline before commit.";
    }
    use case ucAsScopedCreate : UseCase {
        :>> id = "0a5c000c-0114-4c8b-9d2e-000000000c0c";
        :>> title = "Scope- and process-aware creation (contextual to the active filter)";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> description = "Creation is contextual to what is in view. When the scope is filtered (e.g. to safety artifacts), the user can create items IN scope and the items those directly interface with at either end (upstream/downstream), and NOT arbitrary out-of-scope types. Example (a safety/STPA scope): create Loss Scenarios (in scope), downstream Safety/System Requirements, or upstream Losses — never a bare Need. The offered types + connectors come from a DECLARED relationship grammar (generic, read from the connected project), so this holds for any domain, not just safety.";
        :>> mainSuccess = "Given the active scope + the declared grammar, the New-item picker offers only in-scope types + their valid upstream/downstream connectors; out-of-scope creation is not offered.";
        :>> extensions = "With no scope filter, the full declared grammar is available. The grammar is declared once (metamodel) and drives every domain generatively.";
    }
    use case ucAsSystemBoundAnalysis : UseCase {
        :>> id = "0a5c000d-0114-4c8b-9d2e-000000000c0d";
        :>> title = "Sibling / children / ancestry analysis (view-modify-critique)";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> description = "The user runs system-bound analyses as DYNAMIC views with a shifting focus: SIBLINGS (same type sharing a parent), CHILDREN (the downstream of a focus), and ANCESTRY (the upstream traceability chain of a focus) — and can view, modify, or critique the items in that set. These are computed directional slices of the typed graph, exploiting that elements compose linearly along typed edges (superposition).";
        :>> mainSuccess = "From a focused element, choose Siblings / Children / Ancestry -> the graph + a working set re-anchor to that computed slice; VMC actions apply to the set (e.g. critique all siblings for consistency).";
        :>> extensions = "The focus shifts as the user drills; each analysis is a pure computed view (no stored state).";
    }

    requirement srViewerCreateLinkage : SystemRequirement {
        :>> id = "0a5b0005-0114-4c8b-9d2e-000000000b05";
        :>> title = "Create authors traceability edges (not orphans)";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> statement = "The create flow shall let the user attach typed upstream/downstream edges to new (and existing) items in the same action, via the guarded edge-write API, so items land connected. Edge kinds offered are the closed algebra keel permits for the pair.";
        :>> rationale = "The human (2026-07-17): 'I don't see a way to create linkage/traceability to other items.' An unconnected item is invisible to trace/coverage; linkage is the point of the model.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerRelationshipGrammar : SystemRequirement {
        :>> id = "0a5b0006-0114-4c8b-9d2e-000000000b06";
        :>> title = "Declared relationship grammar (type x edge x type), exposed via the API";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel shall expose a DECLARED relationship grammar — which (sourceType, edgeKind, targetType) triples are valid — built on the existing closed edge algebra + EdgeRule/ElementRule layer, so a client can offer only valid connections/creations generically for ANY project's schema (not hardcoded per domain). The grammar is metamodel, declared once; the deliverable's domain vocabulary stays in the downstream project (CLAUDE.md §1).";
        :>> rationale = "Enables scope/process-aware creation (srViewerScopedAuthoring) + valid-edge offers (srViewerCreateLinkage) generatively. Governed by a Decision (metamodel change).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerScopedAuthoring : SystemRequirement {
        :>> id = "0a5b0007-0114-4c8b-9d2e-000000000b07";
        :>> title = "Scope- and process-aware creation constrained to the active filter";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> statement = "Given an active scope filter, the viewer shall offer creation only of in-scope types + the types they directly interface with upstream/downstream (per the declared grammar), and shall not offer out-of-scope creation. With no filter, the full grammar is available.";
        :>> rationale = "The human (2026-07-17): focus the scope, focus the actionable context to that scope, allow modification at that scope (+ the adjacent interfacing scope). Keeps authoring disciplined + on-topic per the current lens.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srViewerSystemBoundViews : SystemRequirement {
        :>> id = "0a5b0008-0114-4c8b-9d2e-000000000b08";
        :>> title = "Sibling / children / ancestry dynamic views with a shifting focus";
        :>> createdAt = "2026-07-17"; :>> createdBy = "wweatherholtz";
        :>> statement = "The viewer shall compute, from a focused element, SIBLING (same type + shared parent), CHILDREN (downstream closure), and ANCESTRY (upstream trace) slices as dynamic views with a shifting focus, and allow view/modify/critique over each set. Pure computed views over the typed graph (no stored state).";
        :>> rationale = "The human (2026-07-17): system-bound analyses (sibling/children/ancestry VMC) are 'dynamic views of the data with a shifting focused element' — valuable because elements superpose + compose linearly along typed edges.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }

    // ── satisfy edges: SR satisfies the upstream Need ────────────────
    satisfy nViewerExplorableGraph by srViewerContextActions;
    satisfy nViewerExplorableGraph by srViewerNavHistory;
    satisfy nViewerExplorableGraph by srViewerSearchDiscovery;
    satisfy nViewerInProgramEdit by srViewerAuthoringSurface;
    satisfy nViewerInProgramEdit by srViewerCreateLinkage;
    satisfy nViewerInProgramEdit by srViewerRelationshipGrammar;
    satisfy nViewerInProgramEdit by srViewerScopedAuthoring;
    satisfy nViewerExplorableGraph by srViewerSystemBoundViews;
}
