// =====================================================================
// EngineComputed — the MATERIALIZED-VIEW + derivation layer (CLAUDE.md §2.6).
//
// Views and calculations are DEFINED here (native where the pilot allows) and
// COMPUTED by tooling (query.py) from authored facts + git — never authored as
// truth. These are the §2.6 "marked, regenerable materialized views."
//
// PILOT NATIVE SUPPORT (spiked 2026-06-10):
//   calc def (incl. expressions) + analysis def + view def/viewpoint def  -> PARSE.
//   expose / render                                                       -> DO NOT.
// So view bodies stay tooling-computed; the `view def`s are named anchors for the
// computed views query.py produces. Full native `expose` views await a newer toolchain.
// =====================================================================
package EngineComputed {
    doc /* The materialized-view + derivation layer: native calc/analysis defs and view-def anchors. Computed by tooling from authored facts + git - never authored as truth. */

    private import ScalarValues::*;
    private import EngineElement::*;   // Element base + value types

    // ---- formal viewpoint registry type (D0056/D0057, ISO 42010 / Arcadia / SysML v2) ----
    // A Viewpoint is a DECLARED lens (ISO 42010): it names the stakeholder CONCERN and
    // AUDIENCE it serves, the model elements/edges it draws from (SOURCES), and the
    // tooling RENDERER that computes it. Instances live in .engine/views/viewpoint-registry.sysml.
    // This makes the view layer auditable for concern COVERAGE (which concerns lack a
    // viewpoint) and extensible by DECLARATION, not by writing new code. The bare native
    // `view def`/`viewpoint def` anchors below are superseded by this queryable registry
    // (kept as SysML-native markers; expose/render still await a newer pilot grammar).
    part def Viewpoint :> Element {
        attribute concernText : String; // stakeholder concern this lens frames (e.g. "safety") — 'concern' is a reserved SysML v2 keyword
        attribute audience : String;   // stakeholder/role served (e.g. "safety engineer")
        attribute sources : String;    // model elements/edges it draws from (what it exposes)
        attribute renderer : String;   // tooling binding that computes/renders it (e.g. "report.py:tab_traceability")
    }

    // ---- derivations as native calc / analysis defs (analysisCalc) ----
    // Coverage = satisfied/total*100, expressed natively (not just in Python).
    calc def RequirementCoverage {
        in satisfied : Real;
        in total : Real;
        return pct : Real = satisfied / total * 100;
    }
    analysis def SuspicionSweep;     // git-ancestry staleness over suspicion-carrying edges (query.py)

    // ---- view anchors as native view defs (nativeViews; expose deferred) ----
    view def OutstandingView;        // ready + blocked outstanding tasks  (query.py whats-next)
    view def SuspectView;            // git-ancestry-stale verified items   (query.py suspect)
    view def TraceView;              // satisfy / verify trace              (query.py)
    viewpoint def MaintainerVP;      // the engine maintainer's concern
}
