// EngineCli — the CLI surface as AUTHORED FACTS, so the interface can be critiqued and changed
// deliberately rather than accreting (D0271).
//
// WHY THIS IS NOT AN ICD DOCUMENT. The human asked to "cement our CLI ICD so we can critique and
// improve it down the road", and invariant 1 forbids authoring an ICD: it would be a document
// regenerable from other facts, drifting from the binary the moment either moved. So the interface
// itself becomes the authored fact - one `CliCommand` per command - and the ICD is a COMPUTED VIEW
// over those facts. What makes it cemented rather than aspirational is the two-way guard: every
// dispatched command must be declared, and every declared command must be dispatched.
//
// WHAT THIS BUYS THAT A DOCUMENT CANNOT. `family` makes "these are variations of one idea" a
// QUERYABLE property rather than an impression - which is what the human noticed by eye across 90
// commands. `effect` makes "status must not write" checkable. `stability` is what lets a
// consolidation land without breaking the 118 call sites that name these commands today: an alias
// is a declared state, not a comment.
package EngineCli {
    private import ScalarValues::*;
    private import EngineElement::*;

    // WHAT A COMMAND DOES TO THE MODEL. The distinction is load-bearing, not descriptive: a `reads`
    // command may be run to find out where you stand, and one that writes may not - which is
    // precisely the property `keel status` had to be built to hold.
    enum def CliEffect {
        reads;                  // computes and prints; mutates nothing
        writes;                 // authors facts, or changes the tree
        both;                   // reads to decide, then writes (migrate, sync, land)
        // `meta` and `alias` are SysML KEYWORDS and the kernel rejects them as enum members - the
        // same trap ImplicationKind hit with `constraint` and `requirement`, and the same reason
        // validate_schema.py blocks on schema changes: the Rust parser accepted both.
        tooling;                // about the tool itself (version, help, hook plumbing)
    }

    // WHETHER A NAME IS PART OF THE CONTRACT OR ON ITS WAY OUT. An `alias` exists so a rename can
    // land without breaking callers on the day it ships; `deprecated` is an aliased name with a stated end.
    enum def CliStability {
        stable;                 // part of the interface; changing it is a breaking change
        aliased;                // a second name for a stable command, kept for callers
        deprecated;             // named, still working, and scheduled to go
        internal;               // plumbing a human is not expected to invoke (hook, gesture)
    }

    // ONE COMMAND. `name` is the invocation token; `synopsis` is what a user needs in order to run
    // it and nothing else - no decision citations, which is what 55 help rows carried until D0270.
    part def CliCommand :> Element {
        attribute name : String;                    // the token typed after `keel`
        attribute synopsis : String;                // what it does, in a user's terms
        attribute invocation : String;              // the argument shape, e.g. `<name> [ROOT]`
        // THE FAMILY IS THE CRITIQUE HANDLE. Commands sharing a family are candidates for one
        // router; a family with one member is a command that earns its own verb. Authored as a
        // string rather than an enum so a new family needs no schema change to be PROPOSED - the
        // point is to argue about the grouping, and an enum would make each argument a CR.
        attribute family : String;
        attribute effect : CliEffect;
        attribute stability : CliStability;
        // What a stable command promises about its OUTPUT, where it promises anything. Absent means
        // the shape is not contracted and a caller must not parse it.
        attribute outputContract : String [0..1];
    }
}
