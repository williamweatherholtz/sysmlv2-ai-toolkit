---
name: repo-push
description: |
  Enforces THIS engine's repository discipline: main-only direct commits (no
  feature branches), the `CR:` prefix for schema/process changes, a required
  Co-Authored-By trailer, and the pre-commit/post-commit hooks (validators run
  on commit; every commit auto-pushes). Use when asked to "commit," "commit and
  push," "save my work," "ship this change," or for commit-message review. Do
  NOT use for story refinement (backlog-refinement) or for deciding whether work
  is done (the Definition of Done gate / sprint-closeout).
metadata:
  version: 0.2.0
  domain: [git, main-only, commit-craft, hooks, release-engineering, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# repo-push

Runs the engine's commit runbook. **`main` is the canonical branch — work on it
directly** (D0014, CLAUDE.md §4). There are no long-lived feature branches:
accepted work is committed straight to `main` and the `post-commit` hook pushes
every commit. The `pre-commit` hook validates staged `.sysml` layers; a red
validator blocks the commit.

> This OVERRIDES the generic "branch off the default branch first" default. In
> THIS repo, committing to `main` is correct; creating a feature branch for
> normal accepted work is the anti-pattern.

## Expert Vocabulary Payload

**Main-only flow (D0014):** stage → commit to `main` → `post-commit` auto-push.
No `git checkout -b`, no PRs for normal work. The history IS the audit trail.

**Commit grammar (this repo's actual convention):**
- **`CR:` prefix is REQUIRED** for any commit that changes schema or a
  workflow/process/skill/tool/decision (CLAUDE.md §4): `CR: <short rationale>`.
  This is the audit trail the engine cannot yet enforce itself.
- Other commits use a short imperative subject (e.g. `Sprint 16 closeOut …`,
  `Track 5 findings as Issues …`).
- Body explains **why**, not how; wrap ~72 cols.
- **Co-Authored-By trailer is REQUIRED** when authored with AI assistance
  (the exact trailer line is mandated by the harness/CLAUDE.md).

**Acceptance precedes commit (commit-and-acceptance memory):** running git while
implementing *accepted* work needs no extra permission, but green-lighting an
investigation is NOT blanket approval — each CHANGE (schema/process/decision,
§3a) needs human acceptance before its commit.

**Hooks are the gate:** `pre-commit` runs the validators that cover the staged
layers; `post-commit` pushes. Never `--no-verify` / skip signing unless the user
explicitly asks — if a hook fails, fix the cause (CLAUDE.md §6, repo discipline).

**Doc-sync rides every change (CLAUDE.md §4):** if the commit changes an item
type, schema, workflow, process, skill, tool, or template, fix every doc claim it
invalidates IN THE SAME COMMIT.

## Anti-Pattern Watchlist

1. **Creating a feature branch for accepted work** — Detection: `git checkout -b`
   for normal sprint/CHANGE work. Resolution: commit to `main` directly; this
   repo is main-only (D0014). (Branches are only for genuine throwaway
   experiments the user asked to isolate.)
2. **Missing `CR:` on a schema/process change** — Detection: a commit touches
   `.engine/schema`, `workflows/`, `processes/`, `skills/`, `tools/`, or
   `decisions/` without the `CR:` prefix. Resolution: prefix with
   `CR: <rationale>`; the Decision record must also exist (§4).
3. **Missing Co-Authored-By trailer** — Detection: AI-authored commit without the
   required trailer. Resolution: append it.
4. **Bypassing hooks** — Detection: `--no-verify`, `-c commit.gpgsign=false`, or
   ignoring a red validator. Resolution: never skip; fix the failing validation.
5. **Secrets / cruft in the diff** — keys, tokens, `.env`, debug prints, stray
   build artifacts. Resolution: remove before staging; `.gitignore` if needed.
6. **Kitchen-sink commit** — unrelated concerns mixed. Resolution: stage
   selectively; one concern per commit.
7. **Unverified success claim** — reporting "pushed / validators green" without
   running them. Resolution: run; report actual output; evidence before assertion.
8. **Committing unaccepted CHANGE** — Detection: committing a schema/process edit
   that has no recorded human acceptance. Resolution: get acceptance + record the
   Decision first (§3a/§4).

## Behavioral Instructions

1. **Scan for the anti-patterns** — #4 (bypass hooks), #5 (secrets), and #8
   (unaccepted CHANGE) are hard stops.
2. **Confirm you are on `main`.** If not, you are likely mid-experiment — confirm
   with the user before merging back; normal accepted work belongs on `main`.
3. **Stage selectively** — only the files for this one concern.
4. **Compose the message:**
   - If the change touches schema/process/skill/tool/decision → start with
     `CR: <rationale>`. Otherwise a short imperative subject.
   - Blank line, body explaining WHY (~72-col wrap).
   - Append the required Co-Authored-By trailer.
5. **Validate first.** Ensure the relevant validators are green (the `pre-commit`
   hook will run them anyway; don't rely on it as your first check). Capture
   output.
6. **Commit.** The `pre-commit` hook validates; the `post-commit` hook pushes to
   `origin/main`. Do not add a manual `git push` step unless the hook is absent.
7. **Report** the commit SHA + that the push hook fired + validator result. Never
   claim success you did not observe.

## Output Format

```
commit:
  branch: main                  # must be main for accepted work
  subject: "CR: <rationale>"    # CR: prefix iff schema/process/skill/tool/decision
  cr_prefix_required: true|false
  co_authored_by: present
  body: "<why>"
gates:
  acceptance_recorded: true|false   # for CHANGE commits
  validators_green: true|false      # actual
  no_secrets: true
  focused_diff: true
result:
  sha: <printed after commit>
  pushed: true|false                # post-commit hook
```

## Examples

- **GOOD (process change):**
  `CR: D0046 — adopt CAE+GQM+ATAM audit framework; backlog 4 workstreams`
  body explains the findings + accepted direction; Co-Authored-By trailer;
  committed to `main`; post-commit hook pushes.
- **GOOD (non-process):**
  `Track 5 process step-back findings as Issues routed to W1-W4`
- **BAD:** `git checkout -b feat/audit && … && open PR` → feature branch +
  PR in a main-only repo (#1); also likely missing `CR:` (#2).
- **BAD:** `git commit --no-verify -m "wip"` → bypasses validators (#4),
  non-descriptive subject, no trailer (#3).

## Questions This Skill Answers

- "Commit and push this"
- "Write a commit message for these changes"
- "Does this need a `CR:` prefix?"
- "Should I branch for this?" (no — main-only)
- "Save my work to the repo"
- "Why did my commit fail?" (pre-commit validator / missing acceptance)
