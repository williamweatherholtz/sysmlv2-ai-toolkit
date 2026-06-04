# SysMLv2 as a Work + Architecture Tracking Spine

**Proposal (refined).** Use SysMLv2 as text files in the repo to track work items,
architectural choices, dependencies, tests, AI skills/agents, and the typed
relationships between them. This replaces a kanban board + ADR folder + loose
traceability conventions with a single typed model that humans review via PRs
and AI agents consume directly.

**Explicitly out of scope:** mirroring runtime events (workflow runs, individual
pushes, image digests, telemetry). Those stay in their native tools. SysML
references them by name/URI when relevant; it does not ingest their firehose.

**Short answer.** Sound idea. The earlier critique attacked the wrong use case.

---

## Why the multi-artifact tie is the core feature

SysMLv2 makes typed relationships first-class:

- `Dependency`, `Allocation`, `Satisfy`, `Verify`, `Refine`, `Trace` — declared,
  not drawn.
- `Specialization`, `Subsetting`, `Redefinition` — extend base types without
  copy-paste.
- `Package` + `import` + qualified names — work spans files cleanly.
- `Requirement`, `VerificationCase`, `AnalysisCase` — language constructs, not
  conventions.

The payoff is **structural staleness detection**. When a requirement changes,
every `satisfy` / `refine` / `verify` edge pointing at it is a typed,
machine-findable link. An AI agent answers "what is now suspect?" with a graph
traversal, not vibes-based grep. Markdown + tickets + wiki links cannot do this
cleanly.

---

## What this looks like in practice

```
package Conventions {
    part def WorkItem {
        attribute status : String;       // todo | doing | review | done
        attribute owner : String;
    }

    part def AISkill {
        attribute name : String;
        attribute prompt : String;
        attribute location : String;     // path on disk
        attribute trigger : String;
    }

    part def Agent :> AISkill {
        attribute subagents : Agent[*];
    }
}

package Project {
    import Conventions::*;

    requirement authReq {
        doc /* Users authenticate via OIDC, sessions expire in 1h. */
    }

    part implementAuth : WorkItem {
        :>> status = "doing";
        :>> owner = "wweath";
    }

    part authTests : WorkItem {
        :>> status = "todo";
    }

    satisfy authReq by implementAuth;
    verify  authReq by authTests;

    part reviewSkill : AISkill {
        :>> name = "code-review";
        :>> location = "skills/review/SKILL.md";
    }

    dependency authTests :> implementAuth;
}
```

PR review on the project file *is* the work-state review. AI agents reading the
repo see the typed graph natively.

---

## Defense of the four points raised against me

### "It collapses if you ever hand-author low-level events"

That claim only applies if you try to log runtime events (workflow runs,
pushes, builds) into the model. For work items and architectural decisions, hand
authoring is *correct* — these are authored deliberately, change on the order of
days, and are the same cadence as kanban cards or ADRs. No collapse.

**Concrete failure example (what to NOT do):** wiring CI to write
`part dockerBuild_a3f9c1 { :>> digest = "sha256:..."; }` on every push. Inside a
month: 40,000 elements, unreadable git history, slow queries, work signal
buried. The boundary is firm: SysML tracks *what you decided to build and how
items relate*; native tools track runtime events. Reference them by URI when a
work item needs to point at a workflow; do not ingest the events.

### "Racing an API"

Applies only to mirroring runtime events whose source of truth lives elsewhere.
A work item like "redesign caching layer" has no upstream API to race — it *is*
the source of truth for its own state.

### "Tooling immaturity means fragility"

For text-files-in-repo with AI-first consumption, you need:

1. A parser — exists (pilot implementation).
2. A validator — exists.
3. A small query layer over the parsed AST — write once, ~hundreds of lines.

What you *don't* get is a polished GUI for non-technical stakeholders. That gap
matters only if you have non-technical stakeholders who need to glance at the
board. For an engineering team where AI agents are first-class readers, the gap
is effectively zero.

### "No convention for AI skills/agents"

Four lines of SysML defines the type. Done. No community standard required for a
four-attribute part definition.

---

## Honest remaining risks (smaller than I claimed, but real)

1. **Convention drift.** Without a frozen `package Conventions` defining
   `WorkItem`, `Decision`, `AISkill`, etc., three contributors model "task"
   three different ways. Cheap to prevent — define base types up front, treat
   changes to them as architectural decisions in their own right.
2. **Onboarding friction for humans editing by hand.** Editing
   `status = "doing"` in a text file is higher-friction than dragging a card.
   Mitigated if AI agents do routine state updates and humans review via PR.
3. **Query tooling does have to exist.** "What's stale because requirement R
   changed?" needs a real implementation, even if small. Plan for it.
4. **No off-the-shelf burndown/velocity chart.** If you need PM-style reporting,
   you write the projection. Not a blocker, but not free.
5. **Failure of nerve.** If half the work lives in SysML and half in GitHub
   Issues "because it was faster today," the typed graph rots. The discipline
   has to be: if it's a work item or an architectural choice, it lives here.
   Period.

---

## Where the boundary sits

| Concern | Tracked in SysMLv2 | Tracked elsewhere |
|---|---|---|
| Work items, statuses, ownership | yes | — |
| Architectural decisions (ADR-equivalent) | yes | — |
| Requirements + verification edges | yes | — |
| AI skills, agents, prompts, locations | yes | — |
| Dependencies between items | yes | — |
| References to repos, workflows, images | by URI | actual data in GitHub/OCI |
| Workflow run history | no | GitHub Actions |
| Image digests, build logs | no | OCI registry, CI logs |
| Telemetry, traces, metrics | no | OpenTelemetry, etc. |
| Code itself | no | the code |

---

## Recommendation

**Yes, do it,** scoped as above. The earlier conditional yes was right in shape
but wrong in framing. Concrete next steps:

1. Define `package Conventions` with `WorkItem`, `Decision`, `AISkill`,
   `Agent`, and the small set of relationship intents you care about. Freeze it
   early; treat changes as architectural moves.
2. Write a thin query layer (parser → AST → "downstream of X" / "what verifies
   X" / "what's stale since R changed"). Aim for a single afternoon's work, not
   a platform.
3. Establish the discipline boundary explicitly: SysML for work and
   architecture state; native tools for runtime events; references between them
   are by URI only.
4. Make AI agents the primary consumer from day one. If a query is hard for
   the AI to run, fix the query layer, not the model.
5. Revisit at one month. If the graph is being queried for real decisions
   ("can I change auth without breaking X?"), it's earning its keep. If not,
   shrink scope or stop.

The framing that makes this work: **SysMLv2 is the typed state of the work, not
a log of the work happening.** Keep that line and the proposal is sound.
