# SysML v2 Syntax Notes (confirmed against the pilot implementation)

Validated empirically against `jupyter-sysml-kernel` 0.59.0 (OMG pilot
implementation, OpenJDK 25) via the harness in `.engine/tools/validate/`.
These supersede guesses; treat them as ground truth for authoring `.sysml`.

## Works ✅

- **Primitives need an import WITH a visibility keyword:**
  `private import ScalarValues::*;` (or `public import …`). A **bare**
  `import ScalarValues::*;` FAILS. Put it inside the package body. Brings
  `String`, `Boolean`, `Integer`, `Real`, etc. into scope.
- Fully-qualified type reference also works with no import:
  `attribute a : ScalarValues::String;`.
- `part def`, `requirement def`, `metadata def`, `enum def`.
- `:>` specialization — used for BOTH type hierarchy and requirement derivation
  (the idiomatic v2 replacement for v1 `deriveReqt`/`refine`/`trace`).
- `:>>` redefinition; `[*]` and `[0..1]` multiplicity.
- `ref name : Type[0..1];` — reference (non-compositional) features.
- Metadata application: the **prefix** form `#MarkerName <element>` (including a `#Marker` on a
  `dependency`, a `first..then` succession, or a `part`) is portable — it validates in BOTH the
  rust authority (D0048) and the kernel. The **member** form `<element> { @MarkerName; }` parses
  in the KERNEL but the **rust parser REJECTS it** ('unexpected character @') — so prefer the
  prefix form for anything rust validates (i.e. anything in the repo). (Found 2026-06-18 applying
  a process-change marker to a Decision part; rust `validate .` caught the member form.)
- **`#Marker first X then Y;`** succession prefix: `#OrderingOnly first A then B;`
  marks a succession edge as ordering-only (confirmed Sprint 8, 2026-06-15). Works both
  inside `action def` bodies and at package level. The Rust parser preserves the marker
  in `Succession.is_ordering_only`; the indexer excludes ordering-only edges from semantic
  dependency computation (they gate ordering but do not block `ready` or propagate
  suspicion — see D0025).
- `doc /* ... */` documentation clauses.
- **Distinct packages + `private import Sibling::*;`** to cross-reference
  between files (the standard-library idiom).
- Reopening a nested package within ONE submission *adds* members.

### Confirmed by the workflow meta-model (2026-06-09; `spike_metamodel.py`, `validate_workflows.py`)

- `:>` specialization from an **`abstract part def`** base (e.g. `part def Workflow :> MetaElement`).
- **Ordered multiplicity** feature: `ref phases : Phase[*] ordered;`.
- **Instance population of a `[*]` feature with a sequence:** `:>> phases = (a, b, c);`
  (and a single value also works: `:>> exitGate = gateA;`).
- Instances via `part x : T { :>> attr = v; :>> ref = other; }` (the `:>>` redefines
  inherited features; `ref` features take element references).
- Closed sets are `enum def` types (pilot-confirmed 2026-06-10: `enum def X { a; b; }`
  + `:>> attr = X::a` parse and render). The old keep-as-String guidance is superseded; the
  remaining caveat is real: enum literals can't be reserved keywords — then fall back to
  vocab — avoids reserved-keyword enum-literal failures.
- `Boolean` attributes parse (via `private import ScalarValues::*;`).
- **Part/usage NAMES also collide with reserved keywords** (not just attribute names):
  `allocation`/`allocate`, `decide`, and `interface` all FAIL as a `part` name
  ("no viable alternative at input ..."). Rename (e.g. `allocPhase`, `decidePhase`).

## Fails / avoid ❌

- **Bare `import X::*;`** (no `private`/`public`).
- **Root-level import** in a cell (must be inside a package).
- **Qualified package names in a declaration:** `package Engine::Core::X { }`
  → "no viable alternative at input '::'". Use nesting `package A { package B {…} }`
  within a single file, OR distinct flat package names across files.
- **Reopening a nested package across files** and expecting shared scope: a
  reopened block does NOT see the earlier block's members or its imports. So
  granular files CANNOT all reopen `Engine::Core` — they must be distinct
  packages that `private import` each other.
- **`dependency def`** — there is no such construct. Model a custom edge as a
  `metadata def` marker applied to a `dependency`, or a `ref` feature.
- **enum literals that are reserved keywords** (e.g. `analysis`) →
  "no viable alternative". Pick non-keyword literals or use `String`.
- **Attribute/feature names that are reserved keywords:** confirmed breakers
  `doc`, `action`. Also avoid `state`, `item`, `part`, `port`, `connection`,
  `subject`, `actor`, `value`, `in`, `out`, etc.

## Structure decision for the schema rewrite

Because qualified names fail and cross-file reopening doesn't share scope, the
schema is restructured as **one distinct top-level package per file**, named
`Engine<Concern>` (e.g. `EngineElement`, `EngineRequirements`, `EngineWork`,
`EngineVerification`, `EngineWorkflow`, `EngineProcess`, `EngineRisk`,
`EngineSkills`, `EngineSafety`). Each file:
- starts with `private import ScalarValues::*;` if it uses primitives, and
- `private import EngineElement::*;` (etc.) for any sibling types it references.

Validate by concatenating dependency-ordered files into one submission (so
imports resolve) — see `.engine/tools/validate/`.

## How to validate

Use the four-layer validators (retired legacy `validate_sysml.py` 2026-06-11):

```powershell
$conda = "C:\Users\WilliamWeatherholtz\miniforge3\Scripts\conda.exe"
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_schema.py
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_workflows.py
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_instances.py
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_tracking.py
```

The kernelspec calls bare `java`, so it MUST run through `conda run -n sysml`
(running the env python directly fails with WinError 2). Needs sandbox disabled
(subprocess + the kernel). A cell FAILS iff the kernel emits a line containing
`ERROR:`. NEVER pipe `conda run` output into another cmdlet — the JVM holds the
pipe and the shell hangs.

## TestResult naming and enum conventions (updated Sprint 7, 2026-06-15)

- **DoD TestResult naming**: `part <task>DoDR<n> : TestResult` — canonical suffix is `DoDR<n>`.
  Standalone phase-gate results use `<gate>R<n>`.
- **Legacy naming**: `part <task>R<n> : TestResult` — accepted by the Rust orient engine
  (`orient::compute` in sysmlv2-cli) as a fallback. Sprint 7 extended done-detection to accept
  both `{task}DoDR{n}` (primary) and `{task}R{n}` (legacy fallback). New work should use the
  `DoDR` canonical form; existing legacy files are tolerated without migration.
- **Outcome enum**: `outcome = VerdictKind::pass` (or `::fail`). The enum is `VerdictKind`,
  **not** `TestOutcome`. Using the wrong name silently produces a non-pass result.
- **Required TestResult fields**: `id` (UUID), `outcome` (VerdictKind), `judgedAgainst`
  (short git SHA), `judgedAt` (ISO-8601 date), `judgedBy` (actor name string).

## `occurrence def` — confirmed Sprint 11 (D0032)

`occurrence def X :> Element { attribute a : String; }` parses correctly. Key rules:
- Avoid reserved attribute names (`doc`, `action`, `actor`, `state`, `item`, etc.) — see Fails section.
- `part <name> : OccurrenceDef { :>> a = "v"; }` instances continue to parse unchanged after retype.
- `TestResult` was retyped from `part def` to `occurrence def` in `EngineVerification` (Sprint 11, D0032) — test results are events (performances of verifications at a point in time). Existing `part <n> : TestResult` instances parse without migration.

## `state def` body syntax — NOT supported in pilot 0.59.0 (D0031, D0041)

Bare `state def X;` parses. But the body transition syntax (`entry state`, `then` keyword) fails with "no viable alternative at input 'entry'". `WorkflowDefinition.transitions : String[*]` stays as string-encoded values (e.g. `'brief->brief-review'`). toolchainWatch sprint (D0041) evaluated this: still deferred — pilot 0.59.0 unchanged; re-evaluate when a new pilot release ships.

## toolchainWatch verdict — Sprint 14 (D0041)

Three feature areas evaluated and deferred:
- `expose`/`render` in view/viewpoint — spec-level feature requiring SysIDE live kernel; incompatible with text-file-first approach. Defer.
- `derive`/`refine`/`trace` — SysML v1 keywords; v2 uses `:>` idiomatically. Defer (consistent with crNativeWins).
- `verify-by` — not a standard SysML v2 keyword; native `verification def` + `verify` target is equivalent. Defer.

Current stack (Rust parser + pilot 0.59.0 + query.py) is stable. Re-evaluate when a new pilot release ships.

## Decision file authoring (`.engine/decisions/`)

Decision files are standalone SysML v2 packages. Common mistakes caught by the lint check in `validate_instances.py`:

- **Imports**: `EngineWork::*` (for `Decision`), `EngineElement::*` (for `VerificationMethod`/`VerdictKind`), and `EngineVerification::*` (for the acceptance event's `Test`/`TestResult`). `Decision` lives in `EngineWork`, so that import is required; `validate_instances.py` lint-checks it.
- **Fields**: `id`, `title`, `createdAt`, `createdBy` (inherited from `Element`) + `status : DecisionStatus`, `context : String` (the forces/situation), `decision : String` (the choice), `rationale : String` (why — incl. alternatives + criteria), `consequences : String`. Acceptance is NOT a field — it is a confirmation event (below).
- **Template**: always copy a recent file (e.g. `0065-attribution-contract.sysml`) — do not author from scratch.
- **Acceptance is a confirmation event (D0066), not a field.** A new accepted Decision `dNNNN` carries `verification dNNNNAccept : Test { :>> method = VerificationMethod::confirmation; ... }` (verifies `dNNNN` by naming) + `part dNNNNAcceptR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedBy = <accepting human>; :>> judgedAt; :>> judgedAgainst; }`. `status = accepted` is the structured fact; the event carries who/when/commit. Tooling reads acceptance from the event.
- **Decision Analysis convention (D0058; ISO 42010 / NPR 7123.1 / ADR).** When a Decision chose
  between real options, capture the *trade* in `rationale`:
  `ALTERNATIVES: (A) <opt> — rejected: <why>; (B) <opt> — rejected: <why>; (C) <chosen>.
  CRITERIA: <the axes the choice was made on>.` Skip for record-only / no-alternative decisions
  (test: *was a real option rejected?*). Records the alternatives-not-chosen (ISO 42010).
- **No cross-package references (issue021).** `validate_instances.py` loads each `.engine` file
  in kernel isolation with only `schema/core` preloaded — it does NOT co-load other decisions,
  processes, or workflows. So a Decision file CANNOT reference an element in another package
  (e.g. `#ProspectiveChange dependency from d0049 to Delivery` with `import DeliveryWorkflow`
  fails: both the namespace and the target are unresolvable; a package is also never a valid
  `dependency` endpoint). Two ways to live within this:
  - **Don't reference — mark + compute.** Prefer a self-contained **prefix marker** on the part
    (`#ProspectiveChange part dNNNN : Decision { ... }`, D0070) and derive the relation from git,
    over storing a cross-package edge. This is how process-change Decisions record (the marker is
    in-package, valid in both rust and the kernel — use the prefix form, not `{ @Marker; }`).
  - **If a genuine cross-package edge is irreducible** (e.g. `#CharteredBy` work→origin, which git
    cannot derive), author it in **`.tracking`**, where rust validation is generic (name refs, no
    cross-resolution) — never in the kernel-validated `.engine` Decision file.

## `%show` read-path limits (pilot 0.59.0 — D0006)

- `%show <FQN>` is a reliable STRUCTURE dumper, an unreliable VALUE dumper.
- Renders attribute values (FeatureValue children) for `PartUsage` and
  `ActionDefinition` — both redefined (`:>> a = "x"`) and direct defaults.
- Renders `RequirementUsage` / verification usages as BARE LEAVES — attribute
  values are NOT surfaced even when `%show`-ing the usage directly. Tooling
  therefore reads scalar values from the `.sysml` TEXT (the one-line dialect).
- `%export` is a silent no-op; multi-valued sequences render as opaque
  `OperatorExpression`; `succession` renders readably (earlier/laterOccurrence).
- Native `elementId` REGENERATES every parse — never use it as durable identity.

