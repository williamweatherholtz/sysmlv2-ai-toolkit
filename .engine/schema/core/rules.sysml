// =====================================================================
// EngineRules — declarative controls (D0105). The `keel check` evaluator reads DECLARED Rule elements
// instead of bespoke Rust guards. Two rule kinds (EdgeRule + ElementRule) + a bounded SCOPE sub-language + an exemption
// mechanism (per the sprint172 spike: predicates stay small, complexity lives in SCOPE). This is the
// EXPAND phase of the D0067 guards->rules migration — the 14 guards still run until `keel check`
// reaches PARITY on each (guardsToRulesMigration); nothing is retired by adding this schema.
//
// Each Rule carries its WHY as a typed edge (#JustifiedBy rule -> Decision), authored separately —
// provenance is a link, never prose (D0103/issue058). subject/object/predicate/appliesWhen are text the
// evaluator interprets (keel's evaluation is text-based; see algo.rs).
// =====================================================================
package EngineRules {
    private import ScalarValues::*;     // String
    private import EngineElement::*;   // Element base + shared scalar/enum vocab

    // The CLOSED edge algebra + # markers an EdgeRule can require. This NAMES the existing closed set
    // (relationships.sysml typed edges + the hashtag markers) — it introduces NO new edge kinds.
    // NOTE: SysML v2 reserves satisfy/verify/allocate/dependency/supersede as relationship keywords, so
    // the typed-edge literals are `Edge`-suffixed to stay valid identifiers. The # markers are plain camelCase.
    enum def EdgeKind {
        specializeEdge;                                                        // :>
        satisfyEdge; verifyEdge; allocateEdge; dependencyEdge; supersedeEdge;  // typed edges
        derivedFrom; charteredBy; resolves; dispositions; dependsOn; covers;   // # markers
    }
    enum def EdgeDirection { outgoing; incoming; }         // subject -> object, or object -> subject
    enum def RuleCardinality { atLeastOne; exactlyOne; }   // required multiplicity of the edge
    enum def RuleSeverity { blocking; warning; }           // honest-state gate vs visible warning (D0098/D0102)
    enum def ViolationKind { block; warn; issue; suspect; } // what a violation produces

    // Base for every declared control. `justifiedBy` is a typed edge (#JustifiedBy Rule -> Decision),
    // authored separately, so each rule records WHY it exists (provenance, not prose).
    part def Rule :> Element {
        attribute severity : RuleSeverity;
        attribute onViolation : ViolationKind;
        // SCOPE sub-language (the load-bearing complexity, per the spike). A bounded predicate set:
        //   all | governedSince(dNNNN) | newlyAdded | whereStatus(<value>) | cutoff(<field>,>=,<n>)
        attribute appliesWhen : String;
        // Grandfather/exemption set with a recorded basis (NEVER fabricated); empty = none.
        attribute exemptions : String;
    }

    // CONFORMANCE: an instance of `subject` must carry `requiredEdge` (in `direction`) to an existing
    // instance of `object`, at `cardinality`. Subsumes the ~9 conformance guards (charter,
    // sprint-coverage, requirement-rootedness, issues, process-skill, process-change,
    // decision-requirement-link, acceptance-events, + the needs-first EdgeRule).
    part def EdgeRule :> Rule {
        attribute subjectType : String;    // element type or # marker the rule binds (e.g. "Story", "#Capability")
        attribute requiredEdge : EdgeKind;
        attribute edgeDirection : EdgeDirection;
        attribute objectType : String;     // required target element type (e.g. "Need")
        attribute cardinality : RuleCardinality;
    }

    // PER-ELEMENT INTEGRITY: every instance of `subject` satisfies `predicate`, composed from the CLOSED
    // vocabulary: nonBlank / minLength / matchesPattern / inRegistry / distinct / referentExists /
    // resolvesToCommand / countCompare. Subsumes the ~5 structural guards (decision-rationale,
    // critic-independence, manifest-coverage, viewpoint-renderer, actors).
    part def ElementRule :> Rule {
        attribute subjectType : String;
        attribute predicate : String;      // e.g. "nonBlank(context) and minLength(rationale,20)"
    }

    // NOTE (issue060 rescope, 2026-07-04): a third `OrderingRule` kind (succession-conformance for
    // ceremony) was defined here but NEVER instantiated — ceremony is a bad-fit for a declarative rule
    // (per-file gate-order + retro-keyword + grandfather) and stays a Rust guard (D0107). It was removed
    // as dead schema. If a genuinely rule-fit ordering case ever appears, re-add it THEN (with an instance).
}
