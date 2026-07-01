# Design direction — declarative, model-driven controls (issue054)

> **Status: DESIGN PROPOSAL — analysis only, NOT applied.** This is a Change-Request
> *statement + rationale* (CLAUDE.md §4) handed to the implementing thread. It authors no
> schema, no guard, no Decision. Adopting it is a `schema/core` change → **human sign-off +
> a sprint** before any code. Authored 2026-06-30 by the analysis thread; redirects the
> resolution approach of `issue054` (`needsFirstOrderingGuard`).

## 1. Problem

The engine enforces invariants with **bespoke, hand-compiled Rust guards** — one per defect
class. The set has grown 12 → 13 → 14 (D0102 `decision-requirement-link`, D0103
`decision-rationale`), and `issue054` proposes a 15th (`guard ordering`). D0047 ("every
correction becomes a permanent guard") *institutionalizes* this reactive growth. Symptoms:

- **Guard sprawl** — the control surface grows linearly with discovered defects; CLAUDE.md §5
  carries a sprawling 14-guard enumeration.
- **CLI-verb proliferation** — `add-task`, `append-result`, `append-gate-result`,
  `apply-review`, and now proposed `add-decision`/`add-need`: one verb per authored type. The
  CLI is coupled to the schema and grows with it.
- **Duplicated truth.** `business.sysml` already declares the workflow order
  (`first needs then useCases`). A hand-written `guard ordering` would be a *second* copy of
  that truth in Rust — the exact anti-pattern §2.1 forbids ("everything derivable is a view").
- **Inert workflows (the real root of issue054 F1).** The Business and Architecture workflows
  are defined but have **no deploying skill** (contra D0059); the `process-skill` guard that
  enforces "no inert process" only scans `.engine/processes/`, not `.engine/workflows/`. With
  no keel-native design skill, a keel-blind external brainstorming skill filled the vacuum and
  produced prose + uncaptured decisions in the wrong order.
- **Downstream baggage.** keel `init`s onto new projects (D0093). Today every project inherits
  guards encoding *this repo's* defect history, with no opt-out short of recompiling.

## 2. The core finding (the proof)

Sorting the 14 guards by what they *actually assert*, **~8 of 14 are one assertion in
disguise**: *a downstream instance must carry its declared upstream edge to an existing
element.*

| Guard | Reduces to | Class |
|---|---|---|
| `ceremony` | sprint phases in declared succession | ordering (workflow) |
| `sprint-coverage` | `Story` → `#CharteredBy` → Need | edge-conformance |
| `requirement-rootedness` | `#Capability` → `#DerivedFrom` → Need | edge-conformance |
| `issues` | `Issue` ← `#Resolves` ← Action/Decision | edge-conformance |
| `process-skill` | `Process` → deployedBy → Skill | edge-conformance |
| `charter` | charter-source edges balance | edge-conformance |
| `process-change` | process edit → governing Decision | edge-conformance |
| `decision-requirement-link` | Decision → Requirement (warn) | edge-conformance |
| `actors` | actor name follows convention | element-constraint |
| `decision-rationale` | `rationale` non-blank, substantive | element-constraint |
| `critic-independence` | critic ≠ author | element-constraint |
| `manifest-coverage` | no dead manifest entries | element-constraint |
| `viewpoint-renderer` | renderer names a real command | element-constraint |

We wrote the same guard eight times because each defect surfaced separately. The remaining
five are genuine per-element integrity predicates — they don't vanish, but they also need not
be hand-compiled.

## 3. The principle

> **Controls are *declared in the model* and evaluated *generically* — not coded.** A new
> defect class becomes a new **declared rule** (no code). Code grows *only* when a genuinely
> new *kind* of predicate appears — which saturates fast. Rules → ∞, code → flat.

This is strictly more faithful to the engine's own invariants (§2.1 text-is-truth, §2.6
"reference procedure; don't embed it," D0088 "requirements-as-evaluable-constraints") than the
status quo, where procedure is embedded in `guards.rs`.

## 4. Design

Two declared rule shapes + one existing source. Each rule is a first-class element (UUID
identity, critique-able, suspect-able, supersede-able) carrying `justifiedBy → Decision` —
so every control records *why it exists* and *when to re-evaluate it*.

### 4.1 `EdgeRule` — subsumes the ~8

*An instance of type X (optional marker filter, optional governing scope) must carry edge E to
an existing instance of type Y.*

```
rule capabilityRootedness : EdgeRule {
    :>> id          = "<uuid>";
    :>> title       = "A user-facing capability must derive from a Need";
    :>> subject     = #Capability;                 // type / marker
    :>> requiredEdge= EdgeKind::derivedFrom;        // from the CLOSED edge algebra
    :>> direction   = Direction::outgoing;
    :>> object      = Need;
    :>> cardinality = Cardinality::atLeastOne;
    :>> appliesWhen = "governedSince(d0098)";        // scope predicate
    :>> severity    = Severity::blocking;            // blocking | warning
    :>> onViolation = ViolationKind::issue;          // issue | block | suspect
    justifiedBy dependency from capabilityRootedness to d0099;   // the WHY
}
```

The needs→decisions ordering issue054 wants becomes **one declaration, not a guard**:

```
rule architectureRootsInNeed : EdgeRule {
    :>> subject = #Architecture;  :>> requiredEdge = EdgeKind::derivedFrom;  :>> object = Need;
    :>> severity = Severity::blocking;
    justifiedBy dependency from architectureRootsInNeed to <new-decision>;
}
```

### 4.2 `ElementRule` — subsumes the ~5

*Every instance of type X satisfies predicate P over its own fields*, where P composes a
**closed vocabulary of primitives** the evaluator implements once.

```
rule decisionRationaleSubstantive : ElementRule {
    :>> subject   = Decision;
    :>> predicate = "nonBlank(context) and minLength(rationale, 20)";
    :>> severity  = Severity::blocking;
    justifiedBy dependency from decisionRationaleSubstantive to d0103;
}
rule critiqueIndependence : ElementRule {
    :>> subject   = Test[method=critique];
    :>> predicate = "distinct(critiquedBy, subject.author)";
    justifiedBy dependency from critiqueIndependence to d0080;
}
```

Starter vocabulary (~8 primitives covers all five element-guards): `nonBlank`, `minLength`,
`matchesPattern`, `inRegistry`, `distinct(a,b)`, `referentExists(ref)`,
`resolvesToCommand(s)`, `countCompare`.

### 4.3 Ordering — no new schema

`ceremony` and Business→Architecture ordering are **already declared** in the workflow
succession (`business.sysml: first needs then useCases`). The evaluator consumes the existing
succession graph and checks it at instance level (a later-phase artifact's provenance /
`judgedAt` may not precede its declared upstream). Zero new authored rules.

### 4.4 The closed verb axis (RMWX) + triage

Triage decomposes into two axes:
- **verb** — a *closed* set: `read` / `record` / `modify` / `execute`. Never grows.
- **process** — an *open, declared* set (business, architecture, delivery, …). Grows in the
  model, not the code.

Triage = `(verb × targetType) → process`; the process declares its required edges + gates.
CLAUDE.md §3's prose routing table becomes a **declared `(verb × type) → process` map** the
engine reads. The CLI collapses to:
- **`keel check`** — evaluates whatever the model declares (replaces `guard <name>`, forever-growing).
- **closed write verbs** — `record` / `modify` / `read` / `execute`, each generic over the
  model (replaces `add-task`/`add-decision`/`add-need`/…). The generic `record` verb is also
  what makes authoring a `Rule` as low-friction as a config edit.

## 5. Fork #1 — resolved: SysML rules, reconcile TOML

**Declared rules live as first-class SysML `Rule` elements (single source of truth); the TOML
contracts are reconciled, not extended.**

- SysML wins on: one store / no drift (§2.1); native `justifiedBy` provenance edge; rules
  become critique-able / suspect-able / renderable / coverage-counted; UUID identity; matches
  the `viewpoint-registry.sysml` precedent for declared governance.
- The **friction** objection (the only reason TOML appealed) is killed by the generic `record`
  verb in §4.4 — authoring a `Rule` becomes as cheap as a TOML line.
- **Reconcile the existing contracts:** `critique-policy.toml`'s "required lenses per type" is
  a *rule with a why* → becomes `Rule` elements. `reverify.toml`'s `commands = [...]` is an
  *execution parameter* → may remain declared config but **owned/referenced by a reverify
  `Rule`**, not free-floating. Downstream override is served by `supersede`, better than a
  separate file.
- **Cost (acknowledged):** a `Rule` construct is a `schema/core` change (frozen) → human
  sign-off. No circularity — a `Rule`'s own well-formedness is checked by the frozen schema
  layer; rule *instances* govern everything else.

## 6. Remaining forks (decide in the CR)

2. **Predicate vocabulary v1** — exact primitive set; where the data/code line sits.
3. **Violation semantics** — map `severity × onViolation` onto the existing
   hard-gate / warning / suspect trichotomy (do not invent a fourth state).
4. **schema/core freeze** — `Rule`/`EdgeRule`/`ElementRule` additions need human sign-off.
5. **Migration (D0067, expand/migrate/contract)** — build the evaluator alongside the 14
   guards; express each guard as a rule; **prove parity** (`keel check` output == the 14
   guards' output on the current repo — precedent: the retired python/rust `parity_check`);
   *then* retire the Rust predicates. The parity gate is what makes the cutover safe.

## 7. Relationship to issue054

This **redirects** `needsFirstOrderingGuard`:
- **C2 `keel guard ordering` → dropped.** Ordering is a declared `EdgeRule` /
  workflow-succession check the generic evaluator already runs (§4.1, §4.3).
- **C1 `keel add-decision` → generalized** into the closed `record` verb (§4.4), not a
  type-specific command.
- **The real root of F1 — inert Business/Architecture workflows — is filled** by authoring
  the missing keel-native design/business skill that deploys those workflows (records each
  decision at point-of-decision, Needs before architecture Decisions), and by extending the
  `process-skill` guard (itself soon a declared rule) to cover `.engine/workflows/`.
- **F2 (decision-capture friction)** is answered by the generic `record` verb, not a bespoke
  `add-decision`.

## 8. Non-goals / what is preserved

- **Structural integrity rules do not disappear** — they move from hardcoded Rust to declared
  `ElementRule`s. "Enforce the process" covers ordering/conformance; it does *not* by itself
  cover per-element integrity. Both are declared; both are evaluated generically.
- **Honest-state vs completeness (D0098)** — unchanged. Rules with `severity=blocking` are
  honest-state gates (truthful/well-formed/traceable); completeness stays a non-blocking
  burndown. A rule must never block on incompleteness.
- **The edge algebra stays closed** (`:>`, satisfy, verify, allocate, dependency, supersede +
  markers). `EdgeRule` references it; it does not extend it.

## 9. Acceptance checklist (for the CR)

- [ ] Decision recorded (context + rationale + consequences) accepting the declarative-controls
      architecture; supersedes/reframes D0047 ("corrections become declared *rules*, not new code").
- [ ] `schema/core` additions (`Rule`/`EdgeRule`/`ElementRule`, predicate vocabulary) — human sign-off.
- [ ] Fork #1 (SysML + reconcile TOML) accepted; migration plan for the two existing contracts.
- [ ] Forks 2–3 (vocabulary v1, violation semantics) decided.
- [ ] Migration via expand/migrate/contract with a parity gate (§6.5).
- [ ] Missing Business/Architecture deploying skill authored; `process-skill` extended to workflows.
- [ ] doc-sync: CLAUDE.md §3 (triage axes) and §5 (guard enumeration → `keel check`).

## 10. De-risk spike (2026-07-01) — does the 8-of-14 collapse survive the messy guards?

The §2 proof sorted guards by their *headline* assertion. This spike stress-tested that by
actually expressing the **three hardest** guards (git-temporal / grandfathered / keyword) as
declared rules, reading their real implementations (`guards.rs`, `algo.rs`).

| Guard | Declared form | New capability required |
|---|---|---|
| `charter` | `EdgeRule` (Story → `#CharteredBy`, ≥1) | scope `newlyAdded` (git-diff, forward-only) + numeric cutoff `sprintNum ≥ 38` |
| `sprint-coverage` | `EdgeRule` (done Action ← covered-by ← Story) — **cleaner** than today's text-contains, *if* coverage becomes a typed edge | scope `where status=done` (computed) + exemption set |
| `ceremony` | (a) gate-ordering = **succession-conformance** (reads the workflow `first…then…` at instance level); (b) retro-scan = `ElementRule` `matchesPattern(retro.text, {avoidable,…})` | a **third rule kind** (ordering) + exemption set |

**Verdict — the collapse HOLDS, but my scope was understated.** The *predicates* stay small
(edge-existence + `matchesPattern` cover the messy cases; `sprint-coverage` even improves).
The complexity does not vanish — it **migrates into three bounded, reusable additions**:

1. **A scope sub-language** (~4 predicates: `governedSince(decision)`, `newlyAdded` git-temporal,
   `where status=<computed>`, numeric cutoff). **Scope — not predicates — is the load-bearing
   complexity.** This is the real inner-platform danger zone: if scope predicates keep growing,
   we've built a query language. MITIGATION: cap the scope vocabulary; a new scope predicate
   requires a justifying meta-rule.
2. **A third rule kind — `OrderingRule` / succession-conformance** — ceremony's gate-ordering is
   neither an `EdgeRule` nor an `ElementRule`. It was implicit in §4.3 ("ordering from
   succession") but is first-class and the most complex evaluator.
3. **A declared exemption mechanism** (grandfather sets with recorded basis) — pervasive across
   all three; one mechanism, reused, and honest (matches "grandfather-with-recorded-basis").

**Answer to critique C4:** the vocabulary does **not** balloon into a general programming
language — the additions are bounded and shared. But the honest minimal surface is **3 rule
kinds + a ~4-predicate scope sub-language + exemptions**, NOT "2 shapes + 8 predicates."
**D0105's direction survives; its SCOPE must be corrected upward**, and the "flat code" claim
re-validated against that larger (still-bounded) surface before the full migration. The parity
gate still governs cutover. (Recorded via the research spike `declarativeControlsCollapseSpike`,
chartered to D0105 — dogfooding issue055.)
