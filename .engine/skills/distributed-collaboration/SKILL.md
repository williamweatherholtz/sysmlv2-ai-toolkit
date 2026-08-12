---
name: distributed-collaboration
description: |
  Deploys the distributed collaboration session loop (D0129) for an asynchronous remote mostly-AI
  team where the git remote is the only coordination medium: sync (fetch, state divergence, integrate
  ancestry-preservingly, check gate parity, orient), claim (take work visibly before starting;
  first-to-land wins), work (sprint ceremony under the ownership contract), land by lane (facts to
  trunk via the gate-on-merged-tree loop; definitional change via reviewed proposal), escalate
  (textual conflicts re-gated; semantic conflicts to human adjudication), close (release the claim).
  Use at the start of any work session on a project with more than one contributor, when a push is
  rejected, when a conflict or contention arises, or when landing work. Do NOT use to enroll a
  contributor (that is `actor-enrollment`, which runs first).
metadata:
  version: 0.1.0
  domain: [distributed, async, remote, collaboration, git, claim, merge, integration, lanes, D0129]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# distributed-collaboration (many contributors, one model, no shared runtime)

Deploys `.engine/processes/distributed-collaboration.sysml`. Runs **once per work session**, around
the existing per-item sprint ceremony:

```
actor-enrollment            once per actor + machine
   └─ distributed-collaboration   once per session   ← THIS
        └─ agile-workflow sprint ceremony   once per work item
```

## The two rules everything else follows from

**1. Never rebase, never squash, never force a shared ref.** A passing `TestResult` counts as done
only while its `judgedAgainst` SHA still resolves. Rebase and squash mint new SHAs, so the rewriting
machine still resolves them locally and reports **green** while every other clone reports
`invalidEvidence`, re-lists finished work as ready, and a second contributor redoes it (issue071).
This is a consequence of the evidence model, not a style preference: **rebase and squash are unsafe
operations in this repo** in a way they are not in an ordinary codebase.

**2. Gate the tree that will actually land.** Two contributions that each pass the gate alone can
fail together. Run the full gate *after* integrating the trunk, *before* pushing.

A third fact makes the whole thing work: **a rejected push is a compare-and-swap failure, not an
error.** Git's atomic ref update is the one coordination primitive a remote gives you — so
first-to-land-wins gives real mutual exclusion with no lock server.

## The loop

### 1. Sync — never orient from a stale tree

```
git fetch
git status -sb                 # divergence: ahead / behind / diverged
git merge --ff-only origin/main    # or a merge commit if you have local work — NEVER rebase
keel guard . && keel validate .    # gate parity: does my gate actually run?
keel orient .
```

Then read the orientation *knowing how stale it was*. Evidence whose anchor is absent from this clone
is **unsynchronized**, not invalid — never count it as work-not-done.

> Shell note: adapt the command *form* to the host (CLAUDE.md §6). PowerShell uses `$env:VAR` /
> `$null`; POSIX uses `$VAR` / `/dev/null`. If one shell tool errors or hangs, switch tools rather
> than re-issuing the same form.

### 2. Claim — before you start, not after

Pick from the ready frontier **minus what others hold**. Record the claim (actor, item, time, commit
claimed against) and land it immediately so it's visible remotely *before* you work.

If the claim is rejected: **that's the mechanism working.** Re-sync, read the winning claim, pick
something else. Never take an item another contributor holds live. A claim held past expiry without
progress is computed stale and is fair to take.

### 3. Work — under the ownership contract (D0108)

- Edit fields **only** on items this actor owns (`createdBy`).
- For anything else: **add** items and typed edges, or **supersede**. Never overwrite in place.
- **Re-sync before writing to a shared region** — the model may have moved since you oriented.
- Every fact carries true provenance: the actor that actually produced it (never a default, never a
  human's name from an AI's session), the attestation time, the commit.
- An AI actor **cannot** record human acceptance, regardless of instruction.

### 4. Land — route by lane, not by preference

**Facts lane** — RECORD / EXECUTE: verification results, claims, ceremony gates, new items and edges.

```
loop:
  git fetch
  git merge origin/main          # ancestry-preserving; never rebase
  keel validate . && keel guard . && keel check-engine .   # on the MERGED tree
  git push                        # rejected? loop again (bounded retries, then back off)
```

**Change lane** — schema, process, workflow, gate, rule, the *meaning* of a computed view; and
deliverable source. Propose as a separately reviewable unit → CI gates it → reviewed by an actor
**other than the author** → carries the recorded human acceptance the change path already requires →
merge commit.

Lane is determined by **what the contribution changes**, not by contributor discretion. It maps onto
the routing you already do (CLAUDE.md §3): RECORD/EXECUTE → facts lane, CHANGE → change lane.

Never exhaust retries into a force-push.

### 5. Escalate — two kinds, handled differently

| Kind | What it looks like | What to do |
|---|---|---|
| **Textual** | Both contributors appended to the same region | Keep **both** facts, then run the **full gate** on the resolution — a textually clean merge can still produce duplicate identity or a broken reference, and merge commits are exactly where that slips through |
| **Semantic** | Two conclusions that can't both be true; two Decisions governing one subject; contention for one item | **Not yours to settle.** Record a tracked Issue naming both positions and their bases; it enters the human-authority queue; the human adjudicates (D0108) |

Never resolve a semantic conflict in favour of your own conclusion — and never concede silently
either. Both lose the disagreement, which is usually information about an upstream ambiguity.

If conflicts recur on the same file, **record that as an observation**: it means the write targets
should be separated, not that friction is normal.

### 6. Close — leave nothing in conversation

Release the claim (or let it expire — that releases the *claim*, never the recorded facts; partial
work stays evidence). Confirm everything a successor needs is **in the model**: no prose handoff, no
status document, no worklist (D0018). The next contributor may be a different actor on a different
machine hours later, and they orient from computed state.

Anything a successor would need that isn't recorded is a **defect** — record it as a fact, a
Decision, or an Issue. Not as a note.

Human-authority items accumulate in the batched queue for the asynchronous supervisor (D0049). Don't
block on them, and don't pre-empt them by recording a judgment the human didn't make.

## Anti-patterns

| Don't | Why |
|---|---|
| `git pull --rebase` to clear a rejected push | Rewrites SHAs → orphans evidence anchors → machine-dependent `orient` (issue071) |
| Squash- or rebase-merge a proposal | Same defect, at integration time |
| Gate your work, then merge, then push | The merged tree is what lands and it was never gated |
| Start work before the claim has landed | Two contributors, one item, discovered at integration |
| Take an item because *you* think the holder is idle | Expiry is computed, not judged by a competitor |
| Settle a contradiction yourself | Precedence isn't adjudication; record it and let the human decide |
| Write a handoff note for the next session | The model is the only tracker (D0018) |
