// =====================================================================
// EngineElement — the universal tracked-element base + minimal actor model.
//
// One flat package per file (Engine<Concern>); siblings cross-reference via
// `private import`. See .engine/docs/keel-syntax-notes.md.
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
    enum def VerificationMethod { inspect; analyze; demo; test; confirmation; critique; }  // native set + engine confirmation + antagonistic critique (D0080)
    // Antagonistic critique (D0080): critique IS a lens-tagged verification.
    enum def Severity { Critical; High; Medium; Low; }                          // finding/issue severity (D0079); human-disposition gate at >= Medium
    enum def CriticKind { aiModel; human; tool; }                               // who performed the critique (identity stays in judgedBy)
    enum def CritiqueLens { completeness; correctness; ambiguity; testability; feasibility; consistency; necessity; }  // requirement-quality canon
    enum def DispositionKind { act; acceptRisk; dismiss; }                      // D0092: a human's recorded verdict on a >= Medium finding (act=resolve / acceptRisk=knowingly accept / dismiss=moot)
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
        attribute createdBy : String[0..1]; // actor id of the creator (a ProjectActors part name); [0..1] — some Element subtypes (e.g. TestResult) use judgedBy instead
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
        attribute createdBy : String[0..1];  // actor id of the creator (a ProjectActors part name)
    }
    abstract use case def TrackedUseCase {
        attribute id : Uuid;
        attribute title : String;
        attribute createdAt : Timestamp;
        attribute createdBy : String[0..1];  // actor id of the creator (a ProjectActors part name)
    }
    abstract verification def TrackedVerification {
        attribute id : Uuid;
        attribute title : String;
        attribute createdAt : Timestamp;
        attribute createdBy : String[0..1];  // actor id of the creator (a ProjectActors part name)
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
