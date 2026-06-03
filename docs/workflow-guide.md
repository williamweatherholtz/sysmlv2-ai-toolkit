# Workflow Guide

This guide explains how to *use* the workflow encoded in the `.sysml` files of
this repository. It is the manual for humans and a reference for AI agents.

## The mental model

Every artifact of the project — work items, decisions, architecture, processes,
AI skills — is a typed element in a SysMLv2 model. Elements are connected by
**typed relationships**: `satisfy`, `verify`, `refine`, `dependency`. The model
lives in `.sysml` text files in this repo. There is no other tracking system.

## Day-to-day actions

### "I have new work to do"

Add a Story (or Epic, if it decomposes into multiple Stories) to a file under
`work/`. Minimum fields:

```sysml
part myNewStory : Story {
    :>> title = "...";
    :>> status = "backlog";
    :>> owner = "<your-handle>";
    :>> priority = "p2";       // p0 highest, p3 lowest
    :>> created = "<YYYY-MM-DD>";
    :>> updated = "<YYYY-MM-DD>";
    :>> estimatedPoints = -1;  // sized at refinement
    :>> acceptanceCriteria = "...";
}
```

Link it to the things it depends on or satisfies:

```sysml
dependency from myNewStory to someUpstreamItem;
satisfy someRequirement by myNewStory;
```

### "I'm ready to work on something"

1. Find a Story with `status = "ready"`.
2. Set its status to `"in_progress"`.
3. If it's > 5 points, add child Tasks first (see Implementation process).
4. Do the work. Write tests.
5. Open a PR.
6. Set status to `"in_review"`.
7. On merge, set status to `"done"` and run the Definition of Done checks.

### "I changed something upstream — what's affected?"

Once the query CLI exists:

```
sysmlv2-ai whats-downstream <qname>
sysmlv2-ai whats-stale-since <git-ref>
```

Before the CLI exists, you grep:

```
grep -r "<qname>" --include="*.sysml"
```

Open every match and decide if it needs revision.

### "I learned something the team should know"

If it changes how you work going forward, write a Decision in `decisions/`:

```sysml
part dXXXX : Decision {
    :>> title = "...";
    :>> status = "accepted";
    :>> date = "<YYYY-MM-DD>";
    :>> context = "...";
    :>> decision = "...";
    :>> consequences = "...";
    :>> supersededBy = "";
}
```

Decisions are immutable once accepted. If a later decision overrides this one,
set `supersededBy` and update the new Decision to reference it back.

## Statuses

```
backlog   → not yet refined
ready     → refined: criteria, size, deps in place
in_progress → being worked
in_review → PR open
done      → merged, tests pass, DoD validated
blocked   → can't proceed; should have a 'dependency' edge to the blocker
```

## Relationship cheatsheet

| You want to say... | Use... |
|---|---|
| "X must be implemented to fulfill requirement R" | `satisfy R by X;` |
| "Test T proves requirement R holds" | `verify R by T;` |
| "Story S realizes architectural component C" | `dependency from S to C;` |
| "Story B can't start until Story A is done" | `dependency from B to A;` |
| "Decision D2 replaces Decision D1" | set `D1.supersededBy = "Decisions::D2"` |

## What the AI agents do (and don't)

Skills with `writePolicy = "direct"` (e.g. staleness-sweep) commit straight to
main. They do bookkeeping only — flagging items as suspect, updating timestamps.

Skills with `writePolicy = "pr-only"` (e.g. implementer, triage) open PRs for
human review. They do judgment-laden work.

Skills cannot self-promote their write policy. The policy is set in
`skills/skills-registry.sysml` and a human change is required to alter it.

## What lives where

| It's a... | It goes in... |
|---|---|
| Type definition (WorkItem, Decision, etc.) | `conventions/` |
| Named workflow procedure | `processes/` |
| Required capability of the toolkit | `requirements/` |
| Toolkit component (parser, indexer, etc.) | `architecture/` |
| Architectural or process decision | `decisions/` |
| Epic / Story / Task | `work/` |
| Registered AI skill or agent | `skills/` |
| Markdown explanation for humans | `docs/` |
| Built tool (binary, library, GUI) | `tools/` (once tools exist) |

## What does NOT go in this repo

- Runtime event logs (workflow runs, builds, deploys) — those live in the
  systems that produce them.
- GitHub Issues / Linear / Jira tickets — there are none. Work tracking is
  here.
- Daily standup notes / ephemeral chat — keep those out of the model.

## When something feels wrong

If the workflow is making a task harder than it should be, that's a signal —
either the conventions need a Decision-tracked change, or you're in a case the
workflow doesn't cover and a new Process or convention is needed. Don't work
around the model silently; surface it.
