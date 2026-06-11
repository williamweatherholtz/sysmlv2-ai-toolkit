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

    // Native metaclasses (requirement / use case / verification def) CANNOT
    // specialize `part def Element` (different metaclass), so each gets a
    // per-metaclass abstract tracked base carrying the SAME engine tracking
    // metadata. Identity stays the authored `id`: native `elementId` is
    // parse-ephemeral (regenerated every parse), so it can't be durable identity.
    // (schemaAudit 2026-06-10 — supersedes the prior part-based-everything choice.)
    abstract requirement def TrackedRequirement {
        attribute id : String;
        attribute title : String;
        attribute createdAt : String;
        attribute updatedAt : String;
        attribute authoredBy : String;
        attribute currentState : String;
    }
    abstract use case def TrackedUseCase {
        attribute id : String;
        attribute title : String;
        attribute createdAt : String;
        attribute updatedAt : String;
        attribute authoredBy : String;
        attribute currentState : String;
    }
    abstract verification def TrackedVerification {
        attribute id : String;
        attribute title : String;
        attribute createdAt : String;
        attribute updatedAt : String;
        attribute authoredBy : String;
        attribute currentState : String;
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
