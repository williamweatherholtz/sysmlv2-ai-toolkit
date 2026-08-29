# control-propriety — attack the control, don't inventory it

Deploys `.engine/processes/control-propriety.sysml` (D0257). The panel assesses ENFORCEMENT
MECHANISMS — controls, guards, arming probes, load-bearing conventions — under the five questions
the human named (st032), one `ProprietyFinding` per target × lens, each with a **named defeater**.

## The five lenses, and the practice each one imports

| Lens | The question | Grounding |
|---|---|---|
| `overfit` | Fitted to the motivating example, or the defect class? | Mutation testing: a control must kill VARIANTS, not the specimen. Mutate the original defect and ask whether the variant slips past. |
| `classCoverage` | Prevents the CLASS, or this one instance? | Variant analysis: name the class explicitly, then name what is in the class but outside the mechanism's reach. |
| `changeRobustness` | Survives ordinary evolution, or depends on incidentals? | The change-detector antipattern: list the incidental facts (formats, prefixes, file locations) it silently depends on. A check that scans 0 and passes has died without a sound. |
| `circumventability` | Can an actor or accident route around it? | Control validation / BAS: enumerate the routes (bypass flags, plausible `none` notes, unverified pointers) and what backstops each. |
| `measurability` | Objectively re-derivable, or the assessor's word? | D0232 receipt-vs-testimony: can a third party re-derive the verdict from what is recorded? |

## Procedure (the process's five steps)

1. **Inventory in tranches** sized to be done well. Proposed solutions are assessable by design.
2. **Assess adversarially** — argue against the mechanism BEFORE crediting it, per lens.
3. **Record atomically** — one `ProprietyFinding` per lens: `target`, `lens`, `verdict`
   (`sound`/`conditional`/`unsound`), `rationale`, `defeater`. Eliminative induction: `sound` means
   "credible defeaters were sought and not found", and its `defeater` field names what was TRIED.
   A finding whose defeater cannot be named goes back to step 2.
4. **Route**: `unsound` → an Issue with a resolver (a defect in a control outranks a defect in
   code); actionable `conditional` → work; accepted `conditional` → a disposition (D0092).
   View: `keel view propriety`.
5. **State independence**: where the assessor authored the target, say so. An author may always
   convict their own work; a self-judged `sound` on a Critical control is a testimony and is named
   as one (D0080).

## Guardrails

- **Never assess a copy.** The target is the shipped mechanism — the real guard run, the real hook,
  the real contract text. A verdict on a paraphrase certifies the paraphrase.
- **The defeater is the deliverable.** A round that produced only verdicts produced praise or blame;
  the defeaters are what the next engineer can act on.
- **Findings do not fix.** The panel records; fixes ride their own Issues and sprints, so the
  assessment cannot be biased by the cost of what it finds.

## Removal path

Delete `.engine/schema/propriety/`, this skill and its registry, the process file,
`.engine/views/propriety.view.toml`, and any `.tracking/propriety*.sysml` instance files. Nothing
else reads the vocabulary; the controls themselves are untouched.
