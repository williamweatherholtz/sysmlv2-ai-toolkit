---
name: repo-push
description: |
  Enforces the engine's repository discipline when committing and pushing:
  Conventional Commits message grammar, a pre-push checklist (branch off main,
  tests green, focused diff, linked work item), and pull-request creation with
  a structured description. Use when asked to "commit and push," "open a PR,"
  "push this branch," "ship this change," "create a pull request," or "save my
  work to the repo." Also triggers on commit-message review and branch-naming
  questions. Do NOT use for story refinement (use agile-refinement) or for
  deciding whether work is functionally done (that is the Definition of Done
  gate). Never commit directly to main/master.
metadata:
  version: 0.1.0
  domain: [git, conventional-commits, pull-requests, CI, release-engineering]
  writePolicy: pr-only
  engine: sysmlv2-ai-toolkit
---

# repo-push

Runs the engine's `repo-push` runbook. Hard gates fire BEFORE any push. Branches
off `main`; never commits to `main`/`master` directly. Consistent with the
engine's PR-based discipline (decision 0003) and the computed-state contract
(CI comments suspicion/coverage deltas on the PR; verdicts are never committed).

## Expert Vocabulary Payload

**Conventional Commits 1.0.0:** type(scope): description; types feat, fix, docs,
style, refactor, perf, test, build, ci, chore, revert; breaking change via `!`
and `BREAKING CHANGE:` footer; SemVer correlation (feat→MINOR, fix→PATCH, !→MAJOR).

**Commit craft (Beams):** imperative mood, ≤50-char subject, blank line, body
explains *why* not *how*, ~72-char wrap, footer tokens (`Refs:`, `Closes #`).

**Branch & PR hygiene:** Conventional Branch (`feat/`, `fix/`, `chore/…`),
trunk-based vs feature branch, small focused diff (<~400 LOC), self-review,
linked work item, mergeable criteria (CI green, reviews approved, no conflicts).

**Engine binding:** WorkItem id in footer, computed-state delta comment on PR,
writePolicy enforcement (a skill cannot self-promote to direct).

## Anti-Pattern Watchlist

1. **Commit to main** — Detection: current branch is `main`/`master`.
   Resolution: HARD STOP. Create a `type/short-description` branch first; never
   commit to the default branch.
2. **Non-conventional subject** — Detection: subject lacks a valid `type:` prefix
   or is non-imperative ("fixed bug", "updates"). Resolution: rewrite to
   `type(scope): imperative description`, ≤50 chars, no trailing period.
3. **Kitchen-sink diff** — Detection: the change mixes unrelated concerns / is
   huge (>~400 LOC). Resolution: split into focused commits/PRs, one concern
   each; large mechanical changes get their own commit.
4. **Secrets / cruft in the diff** — Detection: keys, tokens, `.env`, debug
   prints, commented-out blocks, stray build artifacts. Resolution: remove
   before staging; never push secrets; add to `.gitignore` if needed.
5. **Unlinked work** — Detection: no WorkItem/issue reference. Resolution: add
   the item id to the commit footer (`Refs: <id>`) or branch name; work is
   tracked, so the link must exist.
6. **Bypassing gates** — Detection: temptation to `--no-verify`, skip tests, or
   force-push over review. Resolution: never skip hooks or signing unless the
   user explicitly asks; if a hook fails, fix the cause.
7. **Unverified success claim** — Detection: reporting "pushed / tests pass"
   without having run them. Resolution: run the checks; report actual output;
   evidence before assertion.

## Behavioral Instructions

1. **Scan for the anti-patterns above first** — #1 (commit-to-main) and #4
   (secrets) are hard stops.
2. **Pre-push checklist (all must pass before any push):**
   1. On a correctly named branch (`feat/…`, `fix/…`, etc.), NOT `main`/`master`.
      IF on main: create the branch now.
   2. Diff is focused and reasonably small; self-reviewed; no secrets, debug
      code, or stray files.
   3. Tests + linters pass locally; build is green. Run them; capture output.
   4. Rebased / up to date with the target branch; no conflicts.
   5. A WorkItem id is available to link.
3. **Compose each commit message** in Conventional Commits grammar:
   `type(scope): imperative subject` (≤50). Blank line. Body explaining WHY.
   Footer: `Refs: <work-item-id>` / `Closes #<n>`. Flag breaking changes with
   `!` and a `BREAKING CHANGE:` footer. End the message with the required
   Co-Authored-By trailer if authored with AI assistance.
4. **Commit** the staged, focused change(s). Prefer multiple small commits over
   one large mixed commit.
5. **Push** the branch: `git push -u origin <branch>`.
6. **Open the PR** against the correct base. Title mirrors the commit
   (`type(scope): …`). Body uses the What / Why / How / Testing / Linked-item
   sections (Output Format). Request review; attach output/screenshots for
   behavioral changes.
7. **Note computed-state deltas.** The CI bot comments coverage/suspicion deltas
   (computed-state contract); do not commit those verdicts into the tree.
8. **Respect write policy.** A skill with `pr-only` opens a PR and never merges
   to main itself; `direct` is reserved for mechanical bookkeeping only and is
   set in the registry, not self-assigned. Report the PR URL; let a human merge.

## Output Format

```
Commit:
  type(scope): <imperative subject ≤50>

  <body: why this change, ~72-col wrap>

  Refs: <work-item-id>
  [BREAKING CHANGE: <description>]
  Co-Authored-By: <agent identity>

PR:
  base: main
  head: <branch>
  title: type(scope): <subject>
  body:
    ## What  — <2–3 sentence summary>
    ## Why   — <root cause / motivation>
    ## How   — <notable decisions> (optional)
    ## Testing — <commands run + result>
    ## Linked — Closes #<n> / Refs <work-item-id>
  url: <printed after creation>
gates:
  branch_ok: true            # not main
  tests_green: true|false    # actual result
  diff_focused: true
  linked: true
```

## Examples

### BAD vs GOOD commit subject
- **BAD:** `Fixed the thing and updated tests and some docs` → no type, past
  tense, multiple concerns (#2, #3).
- **GOOD:** `fix(indexer): prevent stale coverage after force-push`
  ```
  Coverage stayed "covered" when a force-push rewrote the commit the
  test result was judged against. Compare via merge-base ancestry, not
  raw SHA equality.

  Refs: STORY-31
  ```

### BAD vs GOOD flow
- **BAD:** on `main`, `git add -A && git commit -m "wip" && git push` → commits
  to main (#1), non-conventional (#2), kitchen-sink (#3).
- **GOOD:** `git checkout -b fix/indexer-stale-coverage` → stage only the
  indexer change → run tests (green) → conventional commit with `Refs:` →
  `git push -u origin fix/indexer-stale-coverage` → open PR with What/Why/
  Testing → report URL → leave merge to a human.

### The representative case — breaking change
Renaming an engine schema attribute used downstream:
`refactor(schema)!: rename WorkflowState.doc to description`
```
'doc' collided with the SysML v2 reserved keyword and broke parsing.

BREAKING CHANGE: WorkflowState.doc is now WorkflowState.description;
update any items or queries referencing the old name.

Refs: STORY-2
```
PR title mirrors it; body lists the migration; computed-state delta comment
will flag items put into `suspect` by the rename.

## Questions This Skill Answers

- "Commit and push this"
- "Open a PR for this branch"
- "Write a commit message for these changes"
- "Is this commit message conventional?"
- "Ship this change"
- "What branch name should I use?"
- "Create a pull request"
- "How do I flag a breaking change?"
- "Push my work to the repo"
- "Is this diff ready to push?"
