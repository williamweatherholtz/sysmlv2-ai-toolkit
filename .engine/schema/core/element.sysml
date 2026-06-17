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
    doc /* Universal tracked-element base (Element + per-metaclass Tracked bases), shared enums and value types, and the minimal actor model. Identity = authored id (native elementId is parse-ephemeral). NOTE: attributed metadata (@Tracked with values) SPIKED GREEN 2026-06-11 - the `trackedMetadata` backlog task will collapse the Tracked* bases into one annotation. */

    private import ScalarValues::*;

    // ---- shared value types + enumerations (strongTyping 2026-06-10) ----
    // Named scalar value types (intent + future validation) and closed-set
    // enums replace bare String where the domain is fixed. CROSS-FILE vocab
    // lives here (EngineElement is imported everywhere); domain-only vocab
    // (e.g. NeedSource) lives in its own domain file.
    attribute def Uuid :> String;          // immutable identity
    attribute def Timestamp :> String;     // ISO-8601 date/time

    enum def ActorKind { human; ai; }
    enum def VerdictKind { pass; fail; inconclusive; error; }                    // native verification verdict
    enum def VerificationMethod { inspect; analyze; demo; test; confirmation; }  // native set + engine confirmation
    enum def Priority { must; should; could; wont; }                            // MoSCoW
    enum def RequirementKind { functional; nonFunctional; safety; security; interfaceReq; }
    enum def RiskLevel { high; medium; low; }
    enum def Owner { human; ai; either; }
    enum def Cadence { continuous; eventDriven; perItem; perSprint; perStory; adHoc; }
    enum def WritePolicy { readOnly; prOnly; direct; }

    abstract part def Element {
        attribute id : Uuid;              // immutable identity
        attribute title : String;         // authored human label (may duplicate)
        attribute createdAt : Timestamp;
        attribute authoredBy : String;    // DEPRECATED (D0063 expand): replaced by createdBy at contract (attrModelContract)
        attribute createdBy : String[0..1]; // canonical author/creator (D0063 expand→migrate→contract; becomes [1..1], authoredBy dropped, at contract)
    }

    // Native metaclasses (requirement / use case / verification def) CANNOT
    // specialize `part def Element` (different metaclass), so each gets a
    // per-metaclass abstract tracked base carrying the SAME engine tracking
    // metadata. Identity stays the authored `id`: native `elementId` is
    // parse-ephemeral (regenerated every parse), so it can't be durable identity.
    // (schemaAudit 2026-06-10 — supersedes the prior part-based-everything choice.)
    abstract requirement def TrackedRequirement {
        attribute id : Uuid;
        attribute title : String;
        attribute createdAt : Timestamp;
        attribute authoredBy : String;       // DEPRECATED (D0063 expand): replaced by createdBy at contract
        attribute createdBy : String[0..1];  // canonical (D0063 expand)
    }
    abstract use case def TrackedUseCase {
        attribute id : Uuid;
        attribute title : String;
        attribute createdAt : Timestamp;
        attribute authoredBy : String;       // DEPRECATED (D0063 expand): replaced by createdBy at contract
        attribute createdBy : String[0..1];  // canonical (D0063 expand)
    }
    abstract verification def TrackedVerification {
        attribute id : Uuid;
        attribute title : String;
        attribute createdAt : Timestamp;
        attribute authoredBy : String;       // DEPRECATED (D0063 expand): replaced by createdBy at contract
        attribute createdBy : String[0..1];  // canonical (D0063 expand)
    }

    // Minimal actor model — only enough to record WHO does tracked work.
    // Rich org modeling is DELIVERABLE subject matter, never the engine.
    part def Actor {
        attribute name : String;
        attribute email : String[0..1];
        attribute kind : ActorKind;
    }
    part def Person :> Actor {
        :>> kind = ActorKind::human;
    }
    part def Role {
        attribute name : String;          // a function filled while doing tracked work
    }
}
