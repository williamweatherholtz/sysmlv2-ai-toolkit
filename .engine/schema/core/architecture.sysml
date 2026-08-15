// =====================================================================
// EngineArchitecture — solution-structure item types (the "how").
// Component. See the design spec §6.
//
// ICDs and the allocation matrix are VIEWS (computed from ports + connections
// and from allocate edges) and are therefore NOT authored types here.
// =====================================================================
package EngineArchitecture {
    doc /* Solution structure: Component (native allocate). ICDs + allocation matrix are computed views. */

    private import ScalarValues::*;
    private import EngineElement::*;


    part def Component :> Element {
        attribute description : String;
    }


    // Allocation (nativeAlloc 2026-06-10): function->component uses the NATIVE
    // `allocate <fn> to <comp>` statement (+ this `allocation def` for typed
    // allocations); connections use native `connect` / `interface def` (>=2 ends).
    // All validated in the pilot. The allocation matrix + ICDs remain COMPUTED
    // views over these native ports / connections / allocate edges.
}
