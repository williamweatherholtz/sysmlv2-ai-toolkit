// =====================================================================
// D0071 — query-driven viewpoints: a view is a DECLARED query (seed + filter + traverse),
// executed by ONE generic engine — not per-view hardcoded renderer code (resolves issue018).
// Viewpoints split into two honest KINDS: query (graph walk) and computed (algorithm); the
// reframe applies to the query kind, computed views stay named computations. Refines D0056/D0057;
// drops the Arcadia-flavored framing.
//
// SUPERSEDED by D0075 (2026-06-19): the CARRIER changes from fields-on-the-SysML-Viewpoint-schema
// to concise TOML .view.toml files, and the model is enriched (seed+filter+traverse -> a filter
// over an enriched authored+computed item-table, after a 12-use-case stress test). D0071's intent
// — declared, executable viewpoints that resolve issue018, with a query/computed distinction — is
// RESTATED in D0075 (the distinction collapses to enrichment-functions + filters). status flipped
// to superseded; the planned Viewpoint-schema-field change is cancelled.
// Accepted 2026-06-18 by wweatherholtz (AskUserQuestion: "Accept — full build", incl. the
// frozen-Viewpoint-schema change; prior framing: "Arcadia is a program, not a viewpoint schema";
// "point to starting items, then filter downstream").
// =====================================================================
package Decision0071 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0071 : Decision {
        :>> id = "63940516-2c7d-4e8f-a071-516273849640";
        :>> title = "D0071: query-driven viewpoints — a view = declared seed+filter+traverse, run by one generic engine; two viewpoint kinds (query | computed); resolves issue018, refines D0056/D0057";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::superseded;
        :>> context = "issue018: the declared viewpoints (D0056/D0057) are anchors whose actual rendering is hardcoded report.py/orient code — adding or changing a view means writing code, not declaring a viewpoint, and the 'sources' field is prose. The user rejected Arcadia-style prescribed layers ('Arcadia is a program, not a viewpoint schema') and asked for 'something super clear and concise ... point to starting items, then filter downstream things/tiers.' Reviewing the 12 registered viewpoints showed they are NOT all one shape: ~half are graph walks (traceability, safety/STPA, requirements, orphans, decisions, baselines) and ~half are algorithms (orient, whats-next, suspect, governing-version, reprocess, attestation-coverage) that cannot be expressed as seed+filter+traverse (e.g. git-ancestry staleness).";
        :>> decision = "A view is a DECLARED QUERY executed by ONE generic engine — not per-view code. Viewpoints carry an explicit KIND: (1) QUERY viewpoints declare a structured spec — seedTypes (the starting item types, e.g. Loss / Requirement), traverseEdges (the typed edges to follow: satisfy/verify/:>/dependency/control/feedback/...), direction (downstream|upstream|both), depth (1|closure); the orphans variant declares a required-edge predicate (flag items missing a required edge). A generic `query.py view <name>` reads the spec and computes the induced subgraph — NO per-view renderer. (2) COMPUTED viewpoints are named algorithms (status/git-ancestry/coverage) and keep a `renderer` binding (orient, whats-next, suspect, governing-version, reprocess, attestation-coverage). The Viewpoint schema (EngineComputed, frozen) gains a `vpKind` discriminator + the optional query-spec fields (§2.5 schema change, accepted). The Arcadia-flavored prose is dropped; `sources` stays as human concern-context, the spec carries the executable definition. Lightweight — no Capella/Kitalpha (D0054 friction).";
        :>> rationale = "Forcing ALL views into seed+filter+traverse is the SAME error as Arcadia-forcing — algorithms (suspect by git ancestry, orient status) are not graph walks; honesty about two kinds is the point (the user's 'flexibility = lack of direction' caution). Declaring the query kind makes adding/changing a structural view a DECLARATION edit, not code — the issue018 fix and the D0001/D0005 'text is truth, views are computed' invariant applied to views themselves. ALTERNATIVES: (A) force every view into seed+filter+traverse — rejected: the computed half doesn't fit, it would distort or hardcode them anyway; (B) keep hardcoded per-view renderers (status quo) — rejected: that IS issue018 (views are code, not declarations); (C CHOSEN) two declared kinds + one generic executor for the query kind, named computations for the rest. CRITERIA: declarative-not-coded (issue018) x honest-to-the-data (don't force one shape) x lightweight (D0054) x compute-don't-store (D0001/D0005).";
        :>> consequences = "Execution = the queryDrivenViewpoints backlog task (own sprint): (1) Viewpoint schema gains vpKind {query|computed} + seedTypes/traverseEdges/direction/depth (frozen-schema, accepted §2.5); (2) a generic query.py `view <name>` executor for query viewpoints; (3) migrate the ~6 query-kind viewpoints (traceability, safety, requirements, orphans, decisions, baselines) to declared specs; computed viewpoints set vpKind=computed + keep renderer; (4) doc-sync (usage + the decisionsVP stale 'decisionText' reference, retired by D0061). Refines D0056/D0057 (viewpoints stay first-class + concern-coverage-audited; they become EXECUTABLE declarations). Resolves issue018. Honest limit: the executor needs the edge graph kernel-free — reuse the existing edge readers (satisfy/verify/:>/charter/etc.). Not a process-def change (no .engine/processes|workflows file), so no process-change marker required by the keystone guard.";
    }

    verification d0071Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a071-516273849641";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of decision d0071 (query-driven viewpoints: declared seed+filter+traverse run by one generic engine; two kinds query|computed; frozen Viewpoint schema gains vpKind + query-spec fields; resolves issue018; refines D0056/D0057) via the AskUserQuestion answer 'Accept — full build' on 2026-06-18, building on the prior 'Arcadia is a program not a viewpoint schema' / 'point to starting items, then filter downstream' framing.";
    }
    part d0071AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a071-516273849642";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "fb6af26";
        :>> judgedAt = "2026-06-18";
        :>> judgedBy = "wweatherholtz";
    }
}
