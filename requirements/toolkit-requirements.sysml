package ToolkitRequirements {

    // ---------------------------------------------------------------
    // What the sysmlv2-ai-toolkit must do.
    // Each requirement gets satisfied by one or more Stories and
    // verified by one or more TestCases. Use 'satisfy' and 'verify'
    // relationships from work/ and tests/ files.
    // ---------------------------------------------------------------

    requirement def TextFilesAreTruth {
        doc /* The repository's .sysml text files are the sole source
               of truth for project state. All other representations
               (indexes, caches, UI state) are derived. */
    }
    requirement textFilesAreTruth : TextFilesAreTruth;

    requirement def Parseable {
        doc /* All .sysml files in this repo parse successfully under
               the SysMLv2 pilot implementation (or the toolkit's own
               parser once written). CI gates on this. */
    }
    requirement parseable : Parseable;

    requirement def QueryDownstream {
        doc /* Given a model element by qualified name, the toolkit
               can list all elements that depend on it via any typed
               relationship (satisfy, verify, refine, dependency,
               decomposition), transitively. */
    }
    requirement queryDownstream : QueryDownstream;

    requirement def StalenessDetection {
        doc /* Given a commit that modifies one or more model
               elements, the toolkit can produce the set of downstream
               elements that may now be stale and should be reviewed. */
    }
    requirement stalenessDetection : StalenessDetection;

    requirement def Validation {
        doc /* The toolkit validates that every relationship
               (satisfy, verify, dependency, etc.) refers to an
               element that actually exists in the model. No
               dangling references. */
    }
    requirement validation : Validation;

    requirement def ReadOnlyAPI {
        doc /* The toolkit exposes a local API that returns model
               state: list elements by type, get element with
               relationships, run canned queries. Read-only in v1. */
    }
    requirement readOnlyAPI : ReadOnlyAPI;

    requirement def WriteAPI {
        doc /* The toolkit accepts write requests that create or
               modify model elements. Writes round-trip cleanly to
               text files without destroying human formatting. */
    }
    requirement writeAPI : WriteAPI;

    requirement def BrowserGUI {
        doc /* A browser GUI lets a human add, edit, and browse model
               elements through schema-driven forms. The GUI uses
               only the public API — no private channel. */
    }
    requirement browserGUI : BrowserGUI;

    requirement def AIClientParity {
        doc /* AI agents use the same API as the GUI. Any operation
               the GUI can do, an AI can do. Authorization (direct
               commit vs PR) is policy, not capability. */
    }
    requirement aiClientParity : AIClientParity;

    requirement def AISkillWritePolicy {
        doc /* Each registered AISkill carries a writePolicy attribute
               ('direct' | 'pr-only' | 'read-only'). The API enforces
               it. Skills cannot self-promote their policy. */
    }
    requirement aiSkillWritePolicy : AISkillWritePolicy;

    requirement def SchemaDrivenForms {
        doc /* The GUI generates forms by introspecting part def
               attributes from the Conventions package. Adding an
               attribute to a part def surfaces it in the GUI on
               next reload, with no GUI code change. */
    }
    requirement schemaDrivenForms : SchemaDrivenForms;

    requirement def ConcurrencySafe {
        doc /* Concurrent writes (human in editor + AI via API)
               do not silently lose updates. At minimum: optimistic
               locking with conflict detection. */
    }
    requirement concurrencySafe : ConcurrencySafe;

    requirement def Portable {
        doc /* The toolkit's query layer is portable across SysMLv2
               implementations via the standardized REST API. The
               specific parser backend (pilot impl, Tree-sitter,
               etc.) is replaceable. */
    }
    requirement portable : Portable;
}
