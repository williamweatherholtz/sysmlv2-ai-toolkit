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
- Metadata application: `#MarkerName <element>` (prefix) and
  `<element> { @MarkerName; }` (member) both work, including a `#Marker` on a
  `dependency`.
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

```
& "C:\Users\WilliamWeatherholtz\miniforge3\Scripts\conda.exe" run -n sysml \
    --no-capture-output python <repo>\.engine\tools\validate\validate_sysml.py
```
The kernelspec calls bare `java`, so it MUST run through `conda run -n sysml`
(running the env python directly fails with WinError 2). Needs sandbox disabled
(subprocess + the kernel). A cell FAILS iff the kernel emits a line containing
`ERROR:`.
