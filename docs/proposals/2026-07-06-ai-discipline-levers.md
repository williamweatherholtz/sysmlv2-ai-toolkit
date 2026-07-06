# Research spike — levers for AI-agent discipline (do agents need a frontend, a better skill, or something else?)

> **Status: RESEARCH SPIKE artifact (analysis).** Produced by sprint194, chartered to D0109 (proposed).
> Dogfoods the research-spike routing (issue055) — this research is TRACKED, not freeform chat.
> Authored 2026-07-06 by the analysis thread.

## Question

The human is tired of AI agents that "stray off, ignore processes, don't document, subvert discipline"
("oops, I did what I said I wouldn't"). Candidate fixes floated: (a) a frontend app that launches AI
tasks + validates output format, (b) `claude -p` to cut prompt overhead, (c) a more rigorous
input-interpretation skill. Which actually buys discipline?

## Findings

### F1 — The problem is real (evidenced, not imagined).
This very self-build shows it: D0106 (strict process-boundedness) was VIOLATED after adoption (the
dual-truth prose, issue058); a resolver was prematurely marked "close as covered" against a DoD that
asked for a process. Agents drift. So the motivation is sound.

### F2 — `claude -p` is the wrong lever for "less overhead prose."
`claude -p` (headless/print mode) runs the SAME agent + SAME system prompt; it is scriptable + bills
against the subscription (why `keel serve` uses it) — it does NOT strip Anthropic's injected instructions.
The only lever that gives a minimal system prompt is a CUSTOM harness on the Agent SDK/API (you own the
prompt). Enterprise caveat: third-party "lite agent" harnesses typically need an API key; Claude
Enterprise/subscription auth generally can't be delegated to arbitrary third-party tools — only the
official CLI/SDK. So "custom lite harness + enterprise billing" is largely exclusive unless built on the SDK.

### F3 — Discipline lives in the ACTION SPACE, not in instructions (the strong-typing insight).
The human's TS-vs-stringly-typing analogy is exact. A SKILL is a LINTER — soft, advisory, ignorable
(engine-triage + D0106's `Parsed:` block were still violated). TYPED TOOLS / a process-launcher /
schema-validated I/O are the COMPILER — they make invalid actions UNREPRESENTABLE. "Just trust the agent"
is `any`. The fix for drift is making the wrong move impossible, not asking the agent to be careful.

### F4 — Lever hierarchy (hard → soft).
1. **Constrain the action space** — tools/UI where an un-routed/invalid action can't be expressed (the
   process-launcher: each affordance IS a defined process). Hardest, most effective.
2. **Short, single-purpose context** — the human's own insight, well-founded: agents drift with long,
   diffuse context. Decompose into small tasks with fresh/narrow context (why subagents/workflows exist).
3. **Validated structured I/O + repair loop** — schema in, schema out, reject-and-retry on malformed
   (generation-time, earlier than keel's commit-time validate/guard).
4. **Approve-before-execute gates** at the interaction boundary (surface the parse/route; human confirms).
5. **A rigorous skill/instruction** (engine-triage). Necessary but LEAST sufficient — the linter.

### F5 — A better input-interpretation skill is necessary-not-sufficient.
"Is all I need a more rigorous skill?" — No. engine-triage ALREADY is the input interpreter, and D0106
already makes it emit a visible `Parsed:` decomposition — yet D0106 was still violated. A more rigorous
skill is incremental (lever #5). The step-change is levers #1–#4, which live in TOOLING.

### F6 — What a frontend uniquely CAN enforce (ties to issue059).
D0106's conversational "parse-first / no ad-hoc action" part is UN-gatable at commit time (a hook can't
see the agent interpreting a prompt) — we accepted that residual as reminder-only (issue059). A frontend
at the INTERACTION boundary is the one place it becomes STRUCTURALLY enforceable: input → visible parse/
route → approve → execute, with un-routed actions simply not offered. That is the real, unique value of
the frontend — not "launching tasks" (ergonomics) but closing the issue059 residual. It will NOT enforce
quality/judgment ("did a shallow job") — no boundary tool can.

### F7 — Build-vs-scope: it's keel's boundary, not a new AI IDE.
A general "launch AI tasks + manage output" app competes with Claude Code/Cursor (huge, off-goal). keel
already has the interaction surface (`keel serve`, D0093 dual-surface). The disciplined version is the
maturation of `keel serve` into a process-launcher + generation-time output-validator over the existing
declared model — reusing the source of structure, not creating a second one. Friction (D0054) is
de-risked here because the tool is single-user (the human said so).

## Recommended direction (-> D0109, proposed)

Pursue discipline through the ACTION-SPACE levers, realized as keel's interaction boundary, in priority:
1. **Process-launcher** — a `keel serve` mode where you invoke DEFINED processes/skills, not freeform
   prompts; each launch shows its parse/route and requires approve-before-execute (closes issue059's
   residual structurally).
2. **Generation-time schema-validated I/O + repair loop** — reject malformed AI output at the boundary,
   auto-retry, before it ever reaches a commit.
3. **Short-context task decomposition** — launches are small, single-purpose (fresh context per task).
4. Treat the input-interpretation skill (engine-triage) as the soft backstop, not the primary control.
Explicitly NOT: `claude -p` for overhead (wrong lever); a general freeform AI IDE (scope); relying on a
"more rigorous skill" as the main fix (soft).

## Open forks for the human

- Build order: process-launcher first (F6, highest unique value) vs output-validator first (F3, quickest win)?
- Harness: extend `keel serve` (reuses model + subscription billing) vs a separate SDK app (minimal prompt,
  but enterprise-auth friction)?
- Is this its own project now, or the next `keel serve` increments?
