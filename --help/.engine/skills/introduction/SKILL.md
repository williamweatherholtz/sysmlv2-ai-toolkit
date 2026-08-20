---
name: introduction
description: |
  Deploys the guided, project-based onboarding process (D0093): take a newcomer on a freshly
  `keel init`-ed project from zero to FIRST VALUE in one session — a 1-minute mental model,
  capture the project's first business Need, derive a first requirement + work item, run the first
  sprint to a real artifact, then show `orient`. Learn by DOING the first real item, not a doc dump.
  Use when a human says "introduce me to the engine", "how do I start", "set up a new project",
  "onboard me", or right after `keel init`. Do NOT use for steady-state work (that's the normal
  loop) or for re-explaining a single concept (just answer).
metadata:
  version: 0.1.0
  domain: [onboarding, spin-up, introduction, first-value, dual-surface, adoption, SysMLv2, D0093]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# introduction (guided, project-based onboarding)

Deploys `.engine/processes/introduction.sysml`. The defining move: **onboard by doing the
newcomer's first real item end-to-end**, not by reciting the engine. One closed item + the ability
to read `orient` = onboarded. Grounded in onboarding research (time-to-first-value; project-based
learning beats docs) and the engine's #1 risk (D0054 adoption friction / revert-to-spreadsheets).

## Expert Vocabulary Payload

**Dual surface (D0093):** the AI drives the **CLI/JSON** (the authority + automation substrate); the
human supervises via **HTML views** + by accepting decisions. One `.tracking/` truth; HTML never a
second store. Don't make the human read JSON or hand-edit `.sysml`.

**Engine vs instance:** `.engine/` + the binary + `CLAUDE.md` = the reusable **engine**; `.tracking/`
= this project's **instance** (what the newcomer authors). The architecture decisions are read-only
**reference**, not the new project's own decisions.

**Time-to-first-value:** the goal is a *completed* first need→requirement→work→verification chain,
fast — so status/trace/coverage visibly **compute** from it. Recording a fact is **one write-API
command** (the friction win, lower than a spreadsheet, D0054).

## Anti-Pattern Watchlist

1. **Doc dump** — Detection: pointing the newcomer at the 90+ decisions before they've done anything.
   Resolution: 1-minute mental model, then their first real item; deeper reading is for later.
2. **Backlog dump** — Detection: capturing many needs/requirements up front. Resolution: ONE smallest
   real need → one requirement → one work item; breadth comes after the loop closes once.
3. **Hand-editing `.sysml` / showing raw JSON to the human** — Detection: the human in a text editor.
   Resolution: AI authors via the write API; human supervises via HTML + acceptance (dual surface).
4. **Skipping the close** — Detection: stopping after authoring, before a sprint closes. Resolution:
   run the first sprint to a recorded passing DoD so the newcomer sees the full loop close on their work.
5. **Re-authoring engine decisions as the project's own** — Detection: copying our decisions into the
   newcomer's instance. Resolution: they're read-only reference; the project authors fresh decisions.

## Behavioral Instructions (the 5 steps)

1. **Orient** — give the 1-minute model: text-is-truth / compute-don't-store; the CHANGE/EXECUTE/
   RECORD/VIEW/ORIENT loop; the dual surface; where to look (`orient`, CLAUDE.md). No decision-log recital.
2. **First Need** — elicit the project's single first business need; author ONE `Need` item (human
   states, AI records).
3. **Refine** — derive ONE `SystemRequirement` (`satisfy` the Need) + one work item; refine to DoR
   (DoD as a verifiable Test) via `keel add-task` + typed edges. Show: recording = one command.
4. **First sprint** — run the agile-workflow ceremony (autonomous, D0049) to the artifact + a recorded
   passing DoD TestResult. The loop closes once, on their own work.
5. **Show value + hand off** — `keel orient` (+ `orient --html` / `report` / `render` review) over
   their one chain; then transition to the steady-state loop (CLAUDE.md §3). Onboarding done = one
   closed item + can read orient.

## Questions This Skill Answers

- "Introduce me to the engine / onboard me / how do I start?"
- "I just ran `keel init` — what now?"
- "Set up / spin up a new project with the engine."
- "Walk me through recording my first need / running my first sprint."
