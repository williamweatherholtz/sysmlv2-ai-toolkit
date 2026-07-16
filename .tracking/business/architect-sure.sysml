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

    // ── satisfy edges: SR satisfies the upstream Need ────────────────
    satisfy nViewerExplorableGraph by srViewerContextActions;
    satisfy nViewerExplorableGraph by srViewerNavHistory;
    satisfy nViewerExplorableGraph by srViewerSearchDiscovery;
    satisfy nViewerInProgramEdit by srViewerAuthoringSurface;
}
