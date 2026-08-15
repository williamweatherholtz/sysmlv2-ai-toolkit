// =====================================================================
// EngineWork — the "things you do" + the decision/change/release/issue types.
// All :> Element, so id/title/createdAt/authoredBy are inherited (state is computed).
// =====================================================================
package EngineWork {
    doc /* Work-item type (Story), Decision (immutable ADR; replaced via supersede), ChangeRequest, Issue, Release (baseline is a computed view). */

    private import ScalarValues::*;
    private import EngineElement::*;

    enum def WorkKind { code; research; documentation; refactor; decision; modeling; }
    enum def WorkPriority { p0; p1; p2; p3; }
    enum def DecisionStatus { proposed; accepted; rejected; superseded; }

    part def WorkItem :> Element {
        attribute kind : WorkKind;
        attribute priority : WorkPriority;
        attribute owner : String;         // Actor id
    }
    part def Story :> WorkItem {
        // Acceptance criteria are NOT a text blob — each is an atomic Test
        // linked by `verify` (EngineVerification + EngineRelationships).
        // estimatedPoints: pre-sprint Fibonacci integer (1/2/3/5/8/13).
        // actualPoints: revised Fibonacci size if scope changed mid-sprint.
        // actualHours: wall-clock hours spent — recorded at sprint closeOut (D0038).
        attribute estimatedPoints : Integer[0..1];
        attribute actualPoints : Integer[0..1];
        attribute actualHours : Real[0..1];
    }

    // Decision (ADR). Immutable once accepted; replaced via the `supersede`
    // edge, not by editing. Scope decisions supersede Needs (see the spec §5).
    part def Decision :> Element {
        attribute status : DecisionStatus;
        attribute context : String;       // forces/situation present — the basis (Nygard ADR 'Context')
        attribute decision : String;      // the choice statement (ADR 'Decision')
        attribute rationale : String;     // justification for the choice (ISO 42010 'Rationale'); alternatives + criteria
        attribute consequences : String;
    }

    part def ChangeRequest :> Element {
        attribute reasonForChange : String;
        attribute impact : String[0..1];
    }

    // A CLAIM: one contributor's intent to work one item (D0147, accepted 2026-08-15).
    //
    // IRREDUCIBLE, so authored rather than computed (§1.1): it records an intention that exists
    // nowhere else and cannot be regenerated from other facts plus git. The distributed-collaboration
    // process names it directly — "the only things authored are irreducible facts, including the
    // claim" — and had no type to record it in (issue111).
    //
    // NO `status`, NO `expiresAt`, NO `released` FLAG, and that absence is the design. Liveness is
    // COMPUTED: a claim is live if it is the newest claim on its item and its age against git-derived
    // time is under the expiry window. A stored status is a verdict that can disagree with the facts
    // it summarises, which §1.6 forbids — and here it would disagree constantly, because expiry is a
    // function of the clock rather than of anything a writer knows.
    //
    // MUTUAL EXCLUSION NEEDS NO FIELD AND NO LOCK SERVER: the remote accepts exactly one of two
    // concurrent claims because a git ref update is a compare-and-swap. A rejected push IS the
    // exclusion working, not an error to handle.
    part def Claim :> Element {
        attribute claimedItem : String;      // the item name this claim is on
        attribute claimedBy : String;        // actor id — same String form as all other provenance
        attribute claimedAt : Timestamp;     // when, for the computed expiry
        attribute claimedAgainst : String;   // the commit the claim was made against
    }

    part def Issue :> Element {
        attribute description : String;
        attribute discoveredInField : Boolean;
        attribute relatedTask : String[0..1];  // backlog task name this issue should be addressed by
        attribute severity : Severity[0..1];    // D0079/D0080: a critique finding carries its severity here (>= Medium needs human disposition)
    }

    // A Release is a declaration + a membership rule. The baseline, release
    // notes, and pack are VIEWS computed from this + commit match (spec §8).
}
