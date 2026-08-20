// =====================================================================
// EngineCodeAudit — a registry of the code a project is responsible for.
// OPTIONAL MODULE (D0148) — import only for projects that audit their own code.
//
// A MODULE, not an addition to `core`, by the precedent `safety` sets: these are a coherent
// optional concern, and putting them in frozen core would make every project carry them to serve
// the projects that want them.
//
// WHAT THIS IS NOT: it is not a parse of your source. keel never reads the deliverable's code to
// decide what exists — a `CodeElement` is an AUTHORED fact about a unit someone took responsibility
// for, and the `keel arch` views compute over those facts plus the Needs graph. The one place source
// is touched is `arch drift`, which compares the recorded `codeHash` against the live `@audit-hash`
// marker; that is a staleness check on the record, not a discovery mechanism. An element nobody
// authored is invisible here, and `arch coverage` exists to say so out loud rather than let the
// registry's silence read as "nothing there".
// =====================================================================
package EngineCodeAudit {
    doc /* Code-audit element types (optional module, D0148): an authored registry of code units with their risk class, invariant safety, STPA role and design pattern, plus the #Controls/#Feedback edges that capture a control structure for a later STPA analysis. */

    private import ScalarValues::*;
    private import EngineElement::*;

    // ---- What is at stake if this unit is wrong -------------------------
    // Ordered worst-first, and `arch criticality` ranks on that order, so the sequence is
    // load-bearing rather than cosmetic. The distinction that matters is CONSEQUENCE, not
    // likelihood: `dataLoss` outranks `security` because a corrupted write is unrecoverable while a
    // disclosure, however bad, leaves the record intact.
    enum def RiskClass { dataLoss; security; correctness; availability; performance; cosmetic; }

    // ---- How the unit's invariants are actually held --------------------
    // `assumed` and `unguarded` are deliberately separate: an assumed invariant has a stated
    // precondition someone is relying on, an unguarded one has nobody holding it at all, and
    // collapsing them would hide the difference between a documented contract and a hope.
    enum def InvariantSafety { enforced; checked; assumed; unguarded; }

    // ---- Where the unit sits in a control loop --------------------------
    // Captured here, ANALYSED nowhere in this module — `arch stpa-inputs` hands the recorded
    // structure to an STPA process (EngineSafety) and stops. Recording a role is cheap and honest;
    // inferring unsafe control actions from it would be an analysis pretending to be a view.
    enum def StpaRole { controller; actuator; sensor; controlledProcess; }

    // ---- The shape the unit is built in ---------------------------------
    // A small closed set on purpose. This exists so `arch coupling` can say "these twelve adapters
    // all sit on the volatile side" — not to catalogue every pattern a codebase might use.
    enum def DesignPattern { adapter; facade; strategy; repository; factory; observer; stateMachine; pipeline; valueObject; service; }

    part def CodeElement :> Element {
        attribute kind : String;                    // module | type | function | trait | command
        attribute filePath : String;                // repo-relative, forward slashes
        // The content hash recorded when this element was last audited, matching the `@audit-hash`
        // marker in the source. Optional: an element can be catalogued before it is audited, and
        // `arch drift` reports that as UNAUDITED rather than as drift, because "never checked" and
        // "checked and since changed" call for different work.
        attribute codeHash : String[0..1];
        attribute riskClass : RiskClass;
        // How much of this unit is interface rather than implementation, "0.0".."1.0". AUTHORED,
        // because it cannot be computed from anything this engine holds — the classic ratio counts
        // abstract types within a package, and this registry records no containment. `arch coupling`
        // needs it for distance-from-main-sequence and reports that column as `-` where it is absent,
        // rather than deriving a number from `kind` and presenting a guess as a measurement.
        //
        // String, not Real, following `Measurement.value` (indicator.sysml) — this engine's settled
        // convention for a numeric datapoint. A second convention would be the worse cost: the
        // authoring surface is quoted strings everywhere else, and one Real attribute would make
        // `0.2` legal in exactly one place.
        attribute abstractness : String[0..1];
        attribute invariantSafety : InvariantSafety[0..1];
        attribute stpaRole : StpaRole[0..1];
        attribute designPattern : DesignPattern[0..1];
        // Control actions this unit issues, when it is a controller. Strings rather than typed refs:
        // the target of a control action is usually a runtime process that has no element in this
        // registry, and inventing one to hold the reference would be authoring fiction.
        attribute controlActions : String[0..*];
    }

    // ---- Markers ---------------------------------------------------------
    // Declared HERE rather than in frozen core (D0136): a module ships its own vocabulary, and the
    // marker-vocabulary guard reads `metadata def` from any engine file.

    // #Controls: a controller CodeElement -> the CodeElement it issues control actions to.
    metadata def Controls;
    // #Feedback: a CodeElement -> the controller it reports state back to. The RETURN path of the
    // loop, and separate from #Controls on purpose — a loop missing its feedback edge is the classic
    // STPA finding, and it is only visible if the two directions are distinct edges.
    metadata def Feedback;
    // #Restructure: marks a CodeElement whose shape is a known problem, so `arch coupling` can
    // separate "coupled and known" from "coupled and unnoticed". It records a judgment already made;
    // it does not schedule the work, which is a WorkItem's job.
    metadata def Restructure;
}
