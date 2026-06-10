// =====================================================================
// EngineElement — the universal tracked-element base + minimal actor model.
//
// One flat package per file (Engine<Concern>); siblings cross-reference via
// `private import`. See .engine/docs/sysmlv2-syntax-notes.md.
//
// Element is the base every authored, tracked item specializes (:>).
//  - id          immutable UUID identity  -> items NEVER collide on name
//  - title       authored human string    -> may duplicate freely
//  - displayLabel is a computed VIEW (NOT stored here; see the view registry)
// Computed fields (satisfaction, coverage, suspicion) are likewise views.
// =====================================================================
package EngineElement {
    private import ScalarValues::*;

    abstract part def Element {
        attribute id : String;            // UUID — immutable identity
        attribute title : String;         // authored human label (may duplicate)
        attribute createdAt : String;     // ISO 8601
        attribute updatedAt : String;     // ISO 8601
        attribute authoredBy : String;    // actor id (human or AI skill)
        attribute currentState : String;  // name of the current WorkflowState
    }

    // Minimal actor model — only enough to record WHO does tracked work.
    // Rich org modeling is DELIVERABLE subject matter, never the engine.
    part def Actor {
        attribute name : String;
        attribute kind : String;          // "human" | "ai"
    }
    part def Person :> Actor {
        :>> kind = "human";
    }
    part def Role {
        attribute name : String;          // a function filled while doing tracked work
    }
}
