---
name: decision-surfacing
description: Run at the end of substantive work, and whenever a Decision is recorded or accepted. Assesses what actually awaits the human from the computed authority queue, and if that set has CHANGED since the last publish, rebuilds the executive brief and republishes it to the same URL - linking it once. If the set has not changed, it produces nothing at all: no page, no link, no sentence. Use it instead of ever restating outstanding decisions in prose.
---

# decision-surfacing — publish the queue, don't narrate it

Deploys `.engine/processes/decision-surfacing.sysml` (D0269, reshaped by D0359). A decision the
human cannot find is not a decision they have been asked to make — and a decision they have been
told about four times is not one either.

## The two failures this sits between

1. **Buried.** Open decisions reported accurately, in prose, at the *end* of long turns. Their
   words: *"Don't bury it under past messages."*
2. **Repeated.** The turn boundary itself then carried a count of what was outstanding, every green
   turn. It was accurate every time, which is exactly why it stopped being read. Their instruction,
   2026-09-06: *"I don't want stop says text anymore. remove. I'm interested in a post-analysis,
   like an AI skill that assesses if there are outstanding decisions and posts the exec summary if
   so, but not repetitive prose anymore."*

The resolution is that **the page is the channel, and prose is not**. The page is refreshed when the
queue moves; the turn says one line when that happens and nothing when it doesn't.

## Procedure

### 1. Assess — from the lens, never from memory

```
keel show authority-queue .
```

Each `decisionAcceptance` row carries the id, the `shortName` of what it is about, and how long it
has waited. **This is the assessment.** Do not re-derive it, do not estimate it, do not carry it
between turns in your head.

### 2. The state test — this is what makes silence honest

Compare that pending set against `.keel/decision-page.toml` (the ids published last time, the URL,
and when).

| Result | What this process does |
|---|---|
| Same set | **Nothing.** No page, no link, no sentence about decisions. |
| Changed — added, answered, or withdrawn | Continue to step 3. |

Silence is a *computed state*, not forgetfulness: it means the published page already says
everything true. That is the whole point of the change — **never** append a count to a turn.

### 3. Build the brief — the numbers are computed, not typed

```
python scripts/exec_brief/facts.py > facts.json     # every number, each with its own `how`
```

Then author the page against the **executive-brief contract in the `asi-templates` skill** (§2):
answer first, one page, no record id anywhere the reader reads, per ask the two mandatory logic
exhibits, a do-nothing course, a falsifier. The exhibit builders are
`scripts/exec_brief/charts.py` (encodings) and `scripts/exec_brief/logic_exhibits.py`
(`logic_lanes` = how it works today vs. changed; `downstream` = what the change lands on).

A number that is not in `facts.json` does not go on the page. A fact that cannot be computed
honestly is emitted with `"value": null` and a `how` that says why — publish that, not a guess.

### 4. Ground each section so it can be judged independently

Their words on what fails: *"I get some semblance of 'we need Need1 & Need2 judged. WHAT DECIDES IT:
picking which ones are good.'"* — circular. Every section carries, **inline**:

- **the item itself**, quoted — a decision needing another file opened has not been surfaced;
- **what accepting binds** — what it legitimises, what it forecloses;
- **the specific fork** — the contested clause against its alternative, never "good vs bad";
- **the evidence** — what is TRUE in the model vs. what is my assumption, labelled apart;
- **what I do with each answer** — the concrete next action per option.

**Test for "what decides it":** it must name an *observable*, or a value only they hold. If reading
the model answers it, I haven't finished my work — answer it myself.

### 5. Check, publish, record

```
python scripts/check_templates.py --brief <page.html>
```

Publish as an Artifact and **republish the same file path** so the URL stays one queue. Then write
the published id set and URL back to `.keel/decision-page.toml` — that file is what step 2 reads
next time.

### 6. Link it once

The URL and one line naming **what changed** go at the top of that response, before the report of
the work. Once. A reader who has seen the link still has it next turn.

An item leaves the page two ways: they answer it, or **I state that I am withdrawing it**. One that
quietly stops being mentioned was decided by default, by me, invisibly.

## Stated residuals

- **Nothing forces this to run.** No gate reads conversational output (D0151). The honest control is
  that the assessment is one command anyone can re-run against the page.
- **`.keel/decision-page.toml` is machine-local.** A different clone republishes to its own URL and
  forks the queue until the two are reconciled by hand.

## Removal path

Delete this skill + registry + the process file and `scripts/exec_brief/`. Nothing else depends on
them; `keel show authority-queue` is unaffected and remains the computed answer.
