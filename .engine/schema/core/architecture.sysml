// =====================================================================
// EngineArchitecture — solution-structure item types (the "how").
// Component (with interface ports), DesignElement. See the design spec §6.
//
// ICDs and the allocation matrix are VIEWS (computed from ports + connections
// and from allocate edges) and are therefore NOT authored types here.
// =====================================================================
package EngineArchitecture {
    private import ScalarValues::*;
    private import EngineElement::*;

    port def InterfacePort {
        attribute name : String;
    }

    part def Component :> Element {
        attribute description : String;
        port ifaces : InterfacePort[*];      // ICDs computed from these + their connections
    }

    part def DesignElement :> Element {
        attribute description : String;
    }
}
