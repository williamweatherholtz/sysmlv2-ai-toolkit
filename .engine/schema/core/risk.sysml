// =====================================================================
// EngineRisk — enterprise risk register concepts (NOT safety hazards; those
// are STPA in schema/safety). A Risk is reduced by a Mitigation; a
// materialized Risk becomes an EngineWork::Issue.
// =====================================================================
package EngineRisk {
    doc /* Enterprise risk register (NOT safety hazards - those are STPA). A materialized Risk becomes an Issue. */

    private import ScalarValues::*;
    private import EngineElement::*;

    enum def RiskStatus { open; mitigated; accepted; materialized; }

    part def Risk :> Element {
        attribute statement : String;
        attribute likelihood : RiskLevel;
        attribute impact : RiskLevel;
        attribute residual : RiskLevel;        // remaining after mitigation
        attribute status : RiskStatus;
    }

    part def Mitigation :> Element {
        attribute statement : String;     // an action/requirement that reduces a Risk
    }
}
