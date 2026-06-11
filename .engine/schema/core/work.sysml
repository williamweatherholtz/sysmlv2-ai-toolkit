// =====================================================================
// EngineWork — the "things you do" + the decision/change/release/issue types.
// All :> Element, so id/title/createdAt/authoredBy are inherited (state is computed).
// =====================================================================
package EngineWork {
    private import ScalarValues::*;
    private import EngineElement::*;

    part def WorkItem :> Element {
        attribute kind : String;          // code|research|documentation|refactor|decision|modeling
        attribute priority : String;      // p0|p1|p2|p3
        attribute owner : String;         // Actor id
    }
    part def Epic :> WorkItem {
        attribute themeArea : String;     // aggregate progress is COMPUTED from children
    }
    part def Story :> WorkItem {
        // Acceptance criteria are NOT a text blob — each is an atomic Test
        // linked by `verify` (EngineVerification + EngineRelationships).
        attribute estimatedPoints : Integer[0..1];
        attribute actualPoints : Integer[0..1];
    }
    part def Task :> WorkItem {
        attribute estimatedHours : Real[0..1];
    }

    // Decision (ADR). Immutable once accepted; replaced via the `supersede`
    // edge, not by editing. Scope decisions supersede Needs (see the spec §5).
    part def Decision :> Element {
        attribute status : String;        // proposed|accepted|superseded
        attribute context : String;
        attribute decisionText : String;
        attribute consequences : String;
    }

    part def ChangeRequest :> Element {
        attribute reasonForChange : String;
        attribute impact : String[0..1];
    }

    part def Issue :> Element {
        attribute description : String;
        attribute discoveredInField : Boolean;
    }

    // A Release is a declaration + a membership rule. The baseline, release
    // notes, and pack are VIEWS computed from this + commit match (spec §8).
    part def Release :> Element {
        attribute membershipRule : String;   // "items matching X at commit Y"
    }
}
