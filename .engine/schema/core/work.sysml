// =====================================================================
// EngineWork — the "things you do" + the decision/change/release/issue types.
// All :> Element, so id/title/createdAt/authoredBy are inherited (state is computed).
// =====================================================================
package EngineWork {
    doc /* Work-item types (Epic/Story/Task), Decision (immutable ADR; replaced via supersede), ChangeRequest, Issue, Release (baseline is a computed view). */

    private import ScalarValues::*;
    private import EngineElement::*;

    enum def WorkKind { code; research; documentation; refactor; decision; modeling; }
    enum def WorkPriority { p0; p1; p2; p3; }
    enum def DecisionStatus { proposed; accepted; superseded; }

    part def WorkItem :> Element {
        attribute kind : WorkKind;
        attribute priority : WorkPriority;
        attribute owner : String;         // Actor id
    }
    part def Epic :> WorkItem {
        attribute themeArea : String;     // aggregate progress is COMPUTED from children
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
    part def Task :> WorkItem {
        // [0..1] is SysML multiplicity (optional field), NOT a value bound.
        // estimatedHours may hold any positive Real (e.g. 40.0 for a week).
        attribute estimatedHours : Real[0..1];
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

    part def Issue :> Element {
        attribute description : String;
        attribute discoveredInField : Boolean;
        attribute relatedTask : String[0..1];  // backlog task name this issue should be addressed by
        attribute severity : Severity[0..1];    // D0079/D0080: a critique finding carries its severity here (>= Medium needs human disposition)
    }

    // A Release is a declaration + a membership rule. The baseline, release
    // notes, and pack are VIEWS computed from this + commit match (spec §8).
    part def Release :> Element {
        attribute membershipRule : String;   // "items matching X at commit Y"
    }
}
