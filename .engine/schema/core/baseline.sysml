// =====================================================================
// EngineBaseline — Release type for computed baseline views (D0042).
//
// A Release is a named project milestone (occurrence def: it IS an event —
// a baseline cut at a specific moment in time, analogous to TestResult D0032).
// The computed baseline view renders the project state at Release.commit:
// items with a passing TestResult judged against that commit or earlier.
// Member refs are NOT stored (that would duplicate git-computable information);
// instead, the commit SHA is the canonical reference — the view engine computes
// "done at release time" from git ancestry.
// =====================================================================
package EngineBaseline {
    doc /* Release: an authored milestone pointer (commit SHA + purpose). The computed baseline view is always re-derived from git — never authored. See D0042. */

    private import ScalarValues::*;
    private import EngineElement::*;

    occurrence def Release :> Element {
        attribute commit : String;        // git SHA at baseline cut
        attribute purpose : String;       // what this release represents
    }
}
