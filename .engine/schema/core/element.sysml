// =====================================================================
// Engine::Core::Element
// Universal tracked-element concern + minimal actor model.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // Tracked: the cross-cutting concern carried by every trackable
    // element, regardless of its metaclass (part, requirement, etc.).
    //
    // Modeled as metadata because trackable things span SysMLv2
    // metaclasses (a requirement is not a part), so a single part-def
    // supertype cannot cover them all. Every type in the engine schema
    // is annotated as @Tracked.
    //
    // NOTE: `id`, `createdAt`, `updatedAt` are AUTHORED facts (live in
    // text). `currentState` references a WorkflowState by name (see
    // workflow.sysml). Computed fields (satisfaction, coverage,
    // suspicion) are NOT here — they are views, never stored
    // (see contracts/computed-state.md).
    // -----------------------------------------------------------------
    metadata def Tracked {
        attribute id : String;            // stable unique id
        attribute createdAt : String;     // ISO 8601
        attribute updatedAt : String;     // ISO 8601
        attribute authoredBy : String;    // actor id (human or ai skill)
        attribute currentState : String;  // qualified name of a WorkflowState
    }

    // -----------------------------------------------------------------
    // Minimal actor model — ONLY enough to record who does tracked work.
    // Rich org modeling (departments, reporting, comp) is DELIVERABLE
    // subject matter and must NOT be added here. See .engine/README.md.
    // -----------------------------------------------------------------
    part def Actor {
        attribute name : String;
        attribute kind : String;          // "human" | "ai"
    }

    part def Person :> Actor {
        :>> kind = "human";
    }

    part def Role {
        doc /* A function someone fills while doing tracked work
               (e.g. "reviewer", "implementer"). Not an org position. */
        attribute name : String;
    }

}
}
