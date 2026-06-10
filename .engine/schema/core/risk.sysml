// =====================================================================
// EngineRisk — enterprise risk register concepts (NOT safety hazards; those
// are STPA in schema/safety). A Risk is reduced by a Mitigation; a
// materialized Risk becomes an EngineWork::Issue.
// =====================================================================
package EngineRisk {
    private import ScalarValues::*;
    private import EngineElement::*;

    part def Risk :> Element {
        attribute statement : String;
        attribute likelihood : String;    // high | medium | low
        attribute impact : String;        // high | medium | low
        attribute residual : String;      // remaining after mitigation
        attribute status : String;        // open | mitigated | accepted | materialized
    }

    part def Mitigation :> Element {
        attribute statement : String;     // an action/requirement that reduces a Risk
    }
}
