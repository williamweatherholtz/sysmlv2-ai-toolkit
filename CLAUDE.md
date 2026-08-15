# CLAUDE.md — how to work in this repo

**keel** is a work-tracking engine whose truth is plain-text SysML v2 files in git and whose state is
**computed**, never stored. It tracks the work of building things, and is built using its own discipline.

Two models, never conflated: the **engine model** tracks the work; the **deliverable** is what the work
produces. The deliverable's domain vocabulary never enters the engine.

- **`.engine/`** — the reusable engine: schema, workflows, processes, skills, rules, decisions. Infrastructure
  (like `.git/`) and this project's deliverable. Committed.
- **`.tracking/`** — instance data from running the process on *this* project (needs, requirements, work
  items, decisions, test results). Committed here because the self-build's history is its evidence;
  downstream projects choose their own policy. See `.tracking/README.md`.

**Your response contract is not in this file.** It lives in the output style
`.claude/output-styles/keel.md` (system prompt, D0130) — parse-first routing, no prose state,
verify-don't-assert, never fabricate an attestation, correct-at-the-root. Enforcement is the `Stop` hook.
Don't restate those rules here: one canonical home per fact (D0105).

---

## 1. Invariants

1. **Text is truth; everything derivable is a view.** Author only *irreducible* facts — atomic items,
   typed edges, test results, recorded judgments. **Never author a document, matrix, baseline, ICD, BOM,
   or report.** Test: *can it be regenerated from other authored facts + git?* Yes → it's a view.
   Materialized views are allowed if marked `#View` and regenerable.
2. **Atomic items, typed edges only.** Edge algebra: `:>` (specialize/derive), `satisfy`, `verify`,
   `allocate`, `dependency`, `supersede`. No checklist blobs inside items.
3. **Identity is an immutable UUID `id`.** Items never collide on name. `title` is a human string and may
   duplicate; `displayLabel` is computed.
4. **Capture decisions even when they cause no action.** "We won't do X" is a first-class `Decision` that
   `supersede`s the need. Scope = superseding Decisions, not a separate type.
5. **`schema/core` is frozen.** Schema and process-definition changes go through Change Request (§3) and
   need explicit human sign-off.
6. **Reference procedure; don't embed it.** Record what *is* — facts, conditions, typed edges. Anything
   naming an action, verdict, or sequence (`ready`, `blocked`, `done`, execution order) is computed or a
   reference, never an authored field. A phase's gate = its `verify`-linked Tests passing.
7. **Requirement vs constraint vs indicator (D0088).** A **constraint** is an executable predicate — the
   guards *are* the constraint layer. A **requirement** is a constraint elevated to a verified stakeholder
   contract (Need/SystemRequirement + satisfy/verify). An **indicator** is monitored with no enforced
   threshold (`keel indicators`). When a "good enough" boundary can't be defensibly set, it stays an
   indicator — promote only when a justified boundary emerges (avoid Goodhart).
8. **Dual surface, one truth (D0093).** CLI/JSON is the authority and automation substrate; HTML is the
   human's oversight lens. HTML never stores truth — it renders `#View`s and wraps the write API.

---

## 2. Orient — never read state from prose

```
keel orient [ROOT]        # in-progress sprints + ready/suspect frontier + non-blocking burndown
keel whats-next [ROOT]    # the ready list, in PRIORITY order (declaration order IS priority, D0052)
```

The AI **auto-follows** the ranked frontier (D0052). Do not ask which ready item to work. Pause only for
a content gate (frozen schema, a direction Decision) or an empty frontier.

Other computed lenses: `suspect` (drift), `orphans`, `view <name>`, `audit`, `coverage`,
`tier-satisfaction`, `rootedness`, `dispositions`, `sitting-coverage`, `concern-coverage`,
`governing-version`, `open-issues`, `indicators`. Human-facing scorecards: `keel report
<assurance|traceability|quality-debt|flow|governance|friction> [--html] [--trend]`. Any declared view
renders interactively via `keel render <view> --mode graph|table|review`, and a human review round-trips
back as linked critiques via `keel apply-review`.

---

## 3. Route every request

Classify by **what it changes**, then follow that route:

| Route | When | What to do |
|---|---|---|
| `CHANGE` §3a | workflow / phase / gate / schema / rule / the *meaning* of a computed view | Change Request: state change + rationale, get **explicit human acceptance**, apply, validate, record a `Decision`, commit `CR:` |
| `EXECUTE` §3b | produces the active phase's typed artifact | orient → act within the phase → record items + edges + judgment → exit when the gate passes |
| `RECORD` §3c | one atomic fact (Decision / TestResult / Issue) | author it + provenance. Never a document blob |
| `VIEW` §3d | asks for a computed answer | compute and present. Never store, never mutate |
| `ORIENT` §3f | where things stand / what's next | `keel orient` |
| `TRIVIAL` | a typo, one rename, one doc line | do it — but label it so the exemption is visible |

Split a multi-part request and route each part. If a non-trivial part maps to no existing process,
**define the process** — that is the creative output, not an ad-hoc action.

**Recurring-or-one-time (D0040, before EXECUTE or VIEW).** Will this recur? Yes and no skill exists →
treat as CHANGE: create the skill first, then execute using it. Clearly one-time → execute. Ambiguous →
ask. Every recurring task done without a skill leaks process knowledge into conversation history, where
it cannot be enforced, reviewed, or improved.

An **`Issue` must be triaged**: give it a `#Resolves` edge from a resolving action or a mooting Decision.
Resolution is then computed, never a prose "RESOLVED" note.

Six workflows: **Business** (needs/what-why) → **Architecture** (how) → **Delivery** (build/verify) →
**Deploy** (release) → **Operate** (field feedback); **Change Request** is cross-cutting and is itself
frozen (modify it only by out-of-band Decision).

---

## 4. Working rules

- **The write API is the sanctioned write path.** `keel append-result`, `append-gate-result`, `add-task`,
  `record decision`, `apply-review`, `actor set`. Direct file editing is for what the API doesn't cover.
- **Every schema/process change must** (a) be recorded as a `Decision` file in `.engine/decisions/`,
  (b) carry its recorded acceptance (who, when, what commit), and (c) validate green before commit.
  Commit messages and memory are **not** decision records.
- **Commit convention:** prefix process/schema commits `CR: <rationale>`.
- **Doc-sync rides every change.** Change an item type, schema, workflow, process, skill, tool, or
  convention → grep the doc surface and fix every claim it invalidates **in the same commit**.
- **Corrections become permanent guards (D0047).** A defect revealing a recurrable gap must become (a) a
  tracked `Issue` and (b) an automated control. Manual vigilance is not a control.
- **Bulk migrations follow the migration process (D0067).** Gated expand/migrate/contract, a committed
  transform, a dry run reconciling control totals, green at every step. Never fabricate historical data.
- **Authoring friction is the #1 risk (D0054).** The dominant MBSE failure mode is adoption friction, not
  bad architecture. If recording a fact is harder than a spreadsheet edit, fix that first.
- **`main` is canonical; commit directly to it.** No long-lived branches.
- **NEVER rebase, squash, or force-push (D0129/issue071).** A passing `TestResult` counts as done only
  while its `judgedAgainst` SHA resolves, so rewriting history orphans evidence and makes `orient`
  **machine-dependent** — green on one clone, not-done on every other. Enforced by remote config and
  hooks. Integrate by merge.
- **Provenance is never defaulted (D0129).** `keel actor set <id>` binds this machine, `KEEL_ACTOR` sets
  it per session, or pass `--judged-by`/`--author`/`--by`. Otherwise the write **refuses**. Actor KIND is
  asked, never inferred: an AI is `Actor` with `kind = ActorKind::ai`, never a `Person`.
- **Multi-contributor work (D0108/D0129).** Each item is owned by its `createdBy`; only the owner edits
  its fields. A non-owner may ADD items and typed edges, or SUPERSEDE — never overwrite in place.
  `git fetch` before a shared-region edit. Conflicting conclusions → record an `Issue`; the human
  adjudicates. Run a multi-contributor session through the **`distributed-collaboration`** skill; enroll a
  contributor with **`actor-enrollment`**.
- **Confirmation results need explicit human sign-off.** A `method=confirmation` verification *is* a human
  attestation — record it only on their explicit confirmation of that specific claim, never inferred from
  an instruction or from the work being done. **Confirm only what tests can't (D0051):** never ask a human
  to confirm a green test. Sprint closeOut and retro are AI-recorded and autonomous (D0049); the single
  human gate is the per-**sitting** review.
- **There is no prose state document (D0018).** Where things stand is computed; what's next is the ranked
  frontier; how to work here is this file. Never author a status, worklist, or handoff doc — if resuming
  requires knowledge, it belongs in the model.

---

## 5. Validation — mandatory for every `.sysml` change

```
keel validate .        # .tracking semantic validation — the AUTHORITY (no kernel)
keel check-engine .    # .engine instance reference resolution (kernel-free) — the ENFORCED instance gate
keel guard             # all 29 forward guards — see .engine/docs/guards.md
keel gate --fast       # the per-edit tier: validate + duplicate-identity + marker-vocabulary (~0.35s)
keel reverify --all-drift   # re-run the declared gate at HEAD; stamp fresh TestResults on green (D0101)
```

**Honest-state gates, not self-assurance gates (D0098).** A commit gate enforces only that the recorded
model is truthful, well-formed, and traceable — **never** that the work is complete. Completeness is a
non-blocking burndown surfaced in `orient`. Don't fake a pass; don't block recording true state.

**What the Rust authority does NOT check (issue097).** `keel validate`/`check` are the ENGINE's semantic
authority — reference resolution, identity, provenance, edge algebra. They are **not** a SysML v2
conformance check, so *"validate is green"* never means *"this is valid SysML v2"*. The Rust parser
accepts `verify X by Y` at package level; the kernel rejects it. That gap is how a non-conformant
construct reached an accepted Decision as a migration target (D0139 clause E). **Never adopt a new base
construct on the Rust parser's acceptance alone** — kernel-check it first:

```
conda run -n sysml --no-capture-output python .engine/tools/validate/conformance_lane.py --construct snippet.sysml > out.txt 2>&1
```

The same tool with no arguments is the **conformance lane**: it sweeps every instance file, reports the
constructs the kernel rejects, and never blocks. Its number is tracked as `conformanceIndicator`
(`keel indicators`) rather than gated, because a rejection may be the pilot kernel's gap rather than
ours — and gating on it would repeat the D0132/issue081 all-or-nothing bypass.

In-loop gating (D0128/D0130/D0134): `keel hook post-edit` runs the fast tier after each `.sysml` edit;
`keel hook stop` runs validate + all guards at the turn boundary and blocks while the model is dishonest.
Both live in the binary — no extra runtime.

The JVM **kernel** validators are the deeper SysML oracle for the type-conformance residual, and are
**opt-in** (`KEEL_KERNEL_VALIDATE=1`, D0132/issue081) — the per-file instance validator was demoted
because it fails correct files, forcing an all-or-nothing bypass that disabled every other layer.
`validate_schema.py` / `validate_workflows.py` still block on schema/workflow changes:

```
& "C:\Users\WilliamWeatherholtz\miniforge3\Scripts\conda.exe" run -n sysml --no-capture-output python .engine\tools\validate\validate_schema.py
```

See `.engine/docs/sysmlv2-syntax-notes.md` before authoring SysML.

---

## 6. Environment

- **Adapt commands to the host OS/shell — the #1 avoidable-friction class (issue065).** Detect which shell
  is active and what the target program expects. Path separators, env-var syntax (`$VAR` vs `$env:VAR`),
  null device, quoting, and backtick behaviour are all shell-specific. If a shell tool errors or hangs,
  switch tools rather than re-issuing the same form. This host: **Windows + PowerShell + git-bash**.
- **`conda` is not on `PATH`** in Claude Code shells. Use the full miniforge3 path (above). Installation
  root: `C:\Users\WilliamWeatherholtz\miniforge3` (miniforge3, not miniconda3).
- **Never pipe a command whose output a JVM holds** — `conda run`, and **`git commit`** when its hooks
  invoke the kernel. The JVM holds the pipe and the shell hangs (cost: a 5-minute stall). Redirect to a
  file and read the file: `git commit -F msg > log 2>&1`. Sweep afterwards with
  `python .engine/tools/kill_stale_kernels.py`.
- **Use absolute paths; don't rely on cwd (issue013).** The Bash and PowerShell tools share one working
  directory, so a `cd` in one changes what the other sees.
- **Validation-path tools must be kernel-free where possible (D0048).** Anything gating a commit should
  not start the JVM.
