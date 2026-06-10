# RESUME — where this project stands

Handoff doc for picking up in a fresh context. Read this, then
`.engine/README.md`, then `.engine/decisions/`.

## What this is

A reusable, AI-complemented **work-tracking engine** built on SysML v2 text
files (`.engine/`), being built using its own workflow. It tracks the *work of
building anything*; the thing built (software, or a future org model) is a
separate deliverable. Full rationale: `.engine/decisions/0001`–`0010`.

## Git state

- Branch: **`engine-restructure`** (off `main`), pushed to
  `github.com/williamweatherholtz/sysmlv2-ai-toolkit`. Do NOT merge to `main`
  until the schema validates green.
- Prior commit: `9f6d319` (the `.engine/` restructure).

## Status

### Done
- `.engine/` schema authored (10 files) + 10 decision records + computed-state
  contract + agile/DoR/DoD processes + skills registry.
- **4 of 13 skills built** in `.engine/skills/`: `requirement-quality`, `stpa`,
  `agile-refinement` (to be renamed `backlog-refinement`), `repo-push` — each
  with references and following the skill-creator structure.
- SysML v2 validation toolchain stood up (`.engine/tools/validate/`) and the
  syntax fully characterized (`.engine/docs/sysmlv2-syntax-notes.md`).
- Research synthesized (INCOSE GtWR + EARS; STPA Handbook SOP; INVEST/Gherkin/
  Conventional Commits) — captured inside the relevant skills' references.

### Schema now parses GREEN (was the #1 blocker)
- **`.engine/schema/` validates 12/12** after the `engine-restructure` rewrite to
  flat `Engine<Concern>` packages (EngineElement/Needs/Requirements/Verification/
  Work/Architecture/Relationships/State/Process/Skills/Risk + EngineSafety). Run
  `.engine/tools/validate/validate_schema.py`. The process-as-data workflows
  (`.engine/workflows/`, 7/7) parse via `validate_workflows.py`.
- **Remaining migration:** the instance files `processes/*.sysml`,
  `decisions/*.sysml`, and `skills/skills-registry.sysml` still use the old nested
  `Engine::Core` structure and must be rewritten to import the new packages; retire
  the legacy `validate_sysml.py` in favor of `validate_schema.py`/`validate_workflows.py`.

## NEXT TASKS (in order — decided: schema first, then skills)

### 1. Schema rewrite → validate green
Apply, using `.engine/docs/sysmlv2-syntax-notes.md` as ground truth:
- **Restructure to one distinct package per file**, named `Engine<Concern>`
  (`EngineElement`, `EngineRequirements`, `EngineWork`, `EngineVerification`,
  `EngineWorkflow`, `EngineProcess`, `EngineRisk`, `EngineSkills`,
  `EngineSafety`). No nested `Engine::Core` reopened across files (doesn't share
  scope); no qualified package names (don't parse).
- Add `private import ScalarValues::*;` to every package using primitives;
  `private import EngineElement::*;` etc. for cross-references.
- `part def Element` base (id, title, createdAt, updatedAt, authoredBy,
  currentState) in `EngineElement`; part-based types `:> Element`. Requirements
  (native `requirement def`) carry their own tracking attributes (different
  metaclass — cannot specialize a part def). Drop the `metadata def Tracked`
  approach.
- Rename reserved-keyword attributes: `doc`→`description` (workflow),
  `action`→`actionText` (process). Scan for others (`state`, `subject`, …).
- Replace `dependency def Supersede` (invalid) with `metadata def Supersede;`
  applied via `#Supersede` on a `dependency` (and/or a `ref supersedes` feature
  on `Decision`). Update decision 0006 to record this.
- Keep closed sets (`status`, `method`, `kind`, `writePolicy`, `ucaType`) as
  `String` with documented vocab (enum literals can't be reserved keywords like
  `analysis`).
- Apply research-refined requirement attributes (see `requirement-quality`
  references and the item-data model summarized in the conversation).
- Update `.engine/tools/validate/validate_sysml.py` to **concatenate
  dependency-ordered files into one submission** (so imports resolve), then run
  until zero `ERROR:`.

### 2. Express quality gates as real SysML v2 `Test`/`Gate` items
The 31 requirement-quality checks, the STPA per-step completeness gates, and
DoR/DoD — author as `Gate`/`GateCheck`/`Test` items so the paper trail is
first-class, not just prose in skills.

### 3. Build the remaining 9 skills
Decided: **2 test skills** (`test-design` = suites+tests; `test-result` =
logging with git-ancestry). Rename `agile-refinement`→`backlog-refinement`.

Worker skills to add: `test-design`, `test-result`, `traceability-audit`,
`definition-of-done`.
Ceremony orchestrators (agents) to add: `standup`, `sprint-planning`,
`implementation`, `retrospective`, `staleness-sweep` (direct write policy).
Then register all skills in `.engine/skills/skills-registry.sysml`.

### 4. Then: build the actual tools
Tracked via the engine itself (dogfood): parser, indexer (Kùzu), validator,
query CLI (`whats-downstream`, `whats-stale-since`), API, browser GUI. See
`.engine/decisions` and the original architecture sketch.

## Key settled decisions (don't re-litigate)
- Text is truth; computed values (satisfaction/coverage/suspicion) are
  index-only views (D0001, D0005).
- Atomic items, no nested checklists; acceptance criteria ARE `verify`-linked
  Tests (D0004).
- Verification by method (test/analysis/inspection/demonstration); `verify`
  subsumes `validate` via target type (D0006).
- STPA in, HARA/ASIL out (D0008). Hazard is one shared type.
- Modular workflow states as data (D0009). Agile ceremonies kept for solo+AI as
  discipline; event-driven sprints; Standup runs DoR + grill-me (D0010).
- Edge algebra: `:>`, `satisfy`, `verify`, `allocate`, `dependency`, `supersede`.

## How to validate (every schema change)
See `.engine/tools/validate/README.md` and the memory note
"SysMLv2 validation toolchain". TL;DR:
`conda run -n sysml --no-capture-output python .engine\tools\validate\validate_sysml.py`
(sandbox disabled).

## Recommended first action on resume
Do task 1 (schema rewrite) end-to-end and get a green validation, then commit to
`engine-restructure`. Everything else depends on a parseable schema.
