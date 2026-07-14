# JVM-kernel retirement — feasibility spike (issue066 / jvmRetirementSpike)

**Status:** research spike deliverable — informs the *proposed* Decision D0112 (not accepted).
**Author:** claudeOpus, 2026-07-14. **Charter:** issue066.

## Question

D0048 removed the JVM kernel from the routine `.tracking` commit path (Rust authority) but
**deliberately kept it** for `.engine` schema/workflow/instance validation, because the Rust
validator does not cover the deeper SysML-v2 semantics. Cost: every `.engine` commit still needs
`conda` + a ~20 s JVM start, and inherits an environment-fragile dependency (this session the
PowerShell tool broke and the kernel step could not run locally). **Can the Rust validator cover
enough of the `.engine` SysML semantics to retire the kernel entirely — and is it worth it?**

## What the kernel validators actually assert

All three (`validate_schema.py`, `validate_workflows.py`, `validate_instances.py`) share one shape:
load `schema/core` (+ `_meta`) into the SysML-v2 **pilot/reference kernel**, then load each target
file and **fail iff the kernel emits an error** matching a keyword set. Concretely, from
`validate_instances.py`:

1. **`lint_decision_imports`** — Decision files must contain `import EngineWork`. **Pure Python, no
   kernel.** Trivially a Rust guard.
2. **`warn_missing_ids`** — every tracked instance (`part|verification|requirement … : <Type>`) must
   carry `:>> id`. **Pure Python, no kernel.** Trivially a Rust guard (WARN-level).
3. **kernel load** — fail iff kernel output contains: `error, couldn't, cannot, unexpected,
   mismatched, no viable, unresolved, extraneous, wasn't expected`.

The error set splits into two classes:

| Class | Keywords | Already covered by Rust? |
|---|---|---|
| **Parse / grammar** | `unexpected`, `no viable`, `extraneous`, `mismatched`, `wasn't expected` | **YES** — the Rust parser (`keel validate`) parses every `.sysml`; a syntax error already fails it. |
| **Name / type resolution** | `unresolved`, `cannot`, `couldn't` | **NO** — the true kernel-only delta: a reference to an undefined type/name, an invalid specialization/redefinition/subsetting, a multiplicity/type-conformance violation. |

So the **kernel's unique value on `.engine` is deep SysML-v2 name-and-type resolution** — not parsing
(duplicated by the Rust parser) and not the two mechanical lints (already pure Python).

## What Rust covers today

`keel validate` parses all `.sysml` and, for `.tracking`, resolves **cross-file references** (the D0048
machinery — the kernel *cannot* do this because it loads files in per-file isolation, issue021/024).
So Rust already has a working **cross-file reference-existence resolver** — just not aimed at `.engine`,
and not a full SysML **type-conformance / specialization** checker.

## Options

- **A — Full Rust SysML semantic checker.** Reimplement name/import resolution + the type-conformance /
  specialization / redefinition / multiplicity rules for the `.engine` subset. Retires the kernel
  outright. **Cost: high; risk: high** (a chunk of the reference implementation, plus ongoing
  spec-tracking). Likely disproportionate to how rarely `.engine` changes.
- **B — Status quo.** Keep the kernel as the CI/on-demand `.engine` oracle. **Cost: none**, but keeps
  the conda/JVM dependency + environment fragility on the (infrequent) engine-commit path.
- **C — Targeted port (recommended).** (1) Port the two pure-Python checks (`import EngineWork` lint,
  missing-`id`) to Rust guards — zero kernel needed, immediate. (2) Extend the Rust cross-file
  reference resolver to `.engine` (type/name **existence** across schema+instances, reusing the
  `.tracking` machinery) — catches the common `unresolved` class. (3) **Demote the kernel to an
  optional CI-only deep-semantics backstop** for the residual (full type-conformance/specialization
  validity). **Cost: medium**; removes conda/JVM from the *common local* engine path (the actual pain)
  without reimplementing the whole type system. Full retirement (A) becomes a later step if the
  residual kernel-only checks prove to catch nothing real over time.
- **D — Alternative SysML checker library.** Swap the pilot kernel for a lighter embeddable checker if
  one exists. **Unknown** — the pilot kernel is the reference impl; viable alternatives may not exist.
  Worth a brief ecosystem scan, but not assumed.

## Recommendation (for the proposed Decision D0112)

**Option C.** It attacks the real cost (conda/JVM on the local engine-commit path) incrementally and
low-risk, keeps an honest deep-semantics backstop in CI, and preserves the option of full retirement
once we can *measure* that the kernel-only residual catches nothing. It explicitly rejects Option A's
"boil the ocean" full-semantics reimplementation as premature.

**Honest limits of this spike:** (a) I did not enumerate every semantic rule the pilot kernel enforces
— only the error-class split that the validators key on; a Phase-1 of Option C should instrument real
`.engine` commits to see which kernel errors actually fire in practice. (b) Option D's feasibility is
unassessed. (c) No implementation is proposed here — D0112 is a **direction**, gated on human
acceptance before any build.
