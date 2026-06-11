# RESUME — handoff (updated 2026-06-11, post-critique, all 13 CRs complete)

Fresh-context pickup doc. **The backlog is the tracker; this file is the map.**

## Orient (do this first)

```
conda run -n sysml --no-capture-output python .engine/tools/query.py orient
```

Returns the state cursor + ready/suspect/invalidEvidence + counts. Subcommands:
`whats-next | outstanding | suspect | item <n> | downstream <n> | trace <n>`.
(PowerShell: NEVER pipe conda-run output into a cmdlet — it hangs. Run plain.)

## Where things stand

- **EngineBuild backlog: COMPLETE (34/34).** Bootstrap construction + the 2026-06-11
  architectural critique's 13 accepted CRs are all done and verified
  (`docs/design-history/2026-06-11-architectural-critique.md` = findings + CR list;
  Decision 0017 = the acceptance record).
- **`NextWork` backlog is QUEUED** in `.tracking/backlog.sysml` (17 tasks). The ready
  frontier and ordering are computed — run orient, don't trust prose.
- **Recommended next action: `dogfoodBusiness`** — the first REAL dogfood (CLAUDE.md §4
  says it's due): move the cursor (`.tracking/state.sysml`) to
  `BusinessWorkflow::Business` / `brief`, author a real Brief→Personas→Needs→UseCases
  in `.tracking/` using `.engine/docs/tracking-template.sysml` idioms, author the
  phase-gate verification instances, get user acceptance (confirmation DoD).
- All validation green at `ec550ac`: `validate_all` 47/47 (schema 13, workflows 7,
  instances 24, tracking 3).

## The NextWork queue (summary — the backlog is authoritative)

| Chain | Tasks |
|---|---|
| Dogfood (headline) | `dogfoodBusiness` → `dogfoodArchitecture` → `dogfoodDelivery` |
| Query gaps (critique UC3/9/10) | `traceNeeds` → `baselineView`; `issueLoop` |
| Native leverage (spiked/design-ready) | `trackedMetadata` (@Tracked spiked GREEN), `stateDefSpike`, `occurrenceRetype`, `authoredByRefs` |
| Parked design questions | `safetyCaseViews`, `needLayer`, `crVersioning`, `skillsRefresh` |
| Runtime (after dogfood) | `runtimeParser` → `writeApi`; `toolchainWatch` |

## Engine mechanics a fresh context must know

- **Work dialect (v2):** backlog = `action def`; tasks = `action`s; deps =
  `first A then B` (`#OrderingOnly` = non-suspicion ordering); criterion =
  `verification <task>DoD : Test` (ONE line); results = APPENDED immutable
  `part <task>R<n> : TestResult` (re-verify = append R2, never edit). Done = latest
  result is a pass. `method=confirmation` needs the human's EXPLICIT sign-off.
- **Suspicion (D0005):** material-change only (criterion text at `judgedAgainst` vs
  now), semantic edges only, transitive; bogus SHAs → `invalidEvidence`. Mere
  re-attestation must NOT re-flag (it oscillates — learned the hard way).
- **Process discipline:** classify every request (CLAUDE.md §3, engine-triage skill);
  every schema/process change = Decision file + recorded acceptance + `CR:` commit
  (§4); doc-sync rides every change (EngineDocSync — fix invalidated doc claims in
  the SAME commit); computed-view semantics changes are CHANGE, not BOOTSTRAP (§3e).
- **Validation:** pre-commit hook auto-runs the layer matching staged `.sysml`
  (post-commit auto-pushes to canonical `main` — main-only policy, Decision 0014).
  `validate_all.py` = everything on one kernel. Orphaned kernel JVMs:
  `python .engine/tools/kill_stale_kernels.py`.

## Where things are

| What | Where |
|---|---|
| Interaction contract | `CLAUDE.md` (triage §3, bootstrap rules §4, validation §5) |
| Schema (canonical vocabulary, doc'd) | `.engine/schema/core/` + `safety/` |
| Workflows (six action defs, schema-typed) | `.engine/workflows/` |
| Processes (agile, DoR, DoD, critique, doc-sync) | `.engine/processes/` |
| Decisions 0001–0017 (ADRs incl. acceptances) | `.engine/decisions/` |
| Skills (5 real, registered) | `.engine/skills/` |
| Tools (query, capture_user, sweep, validators) | `.engine/tools/` |
| Instance data (backlogs, actors, cursor) | `.tracking/` (committed for self-build) |
| Authoring idioms + marker examples | `.engine/docs/tracking-template.sysml`, `usage.md` |
| Pilot syntax do's/don'ts | `.engine/docs/sysmlv2-syntax-notes.md` |
| Critiques + CR history | `docs/design-history/` |

## Pilot-kernel constraints (0.59.0 — verified by spikes in `.engine/tools/validate/_spike_*.py`)

- `%show` renders structure reliably, requirement/verification-usage attribute VALUES
  never (D0006) → tools read scalars from text (one-line dialect contract).
- Parses: `satisfy x by y`, verification `objective`, `allocate x to y`, `connect`,
  `enum def`, `require constraint {}`, `doc /* */`, attributed `metadata def` +
  valued `@Application`, part/requirement-typed action pins + flows.
- Does NOT parse: `expose`/`render`, `verify X by Y`, `derive`/`refine`/`trace`
  (v1-only). Native `elementId` regenerates per parse → authored `id` is identity.
- Reserved-word traps incl. `action` as a FEATURE name (see syntax-notes).

## Known stale / accepted debt

- `docs/superpowers/specs/2026-06-04-process-model-design.md` is the ORIGINAL design
  spec — historically valuable, but superseded in places by the critique CRs (native
  action-def workflows, no Gate type, dialect v2). Read decisions + CLAUDE.md first.
- `WorkflowDefinition.transitions` is still strings (pending `stateDefSpike`).
- `authoredBy`/`judgedBy` are name-strings pending `authoredByRefs`.
