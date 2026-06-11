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
