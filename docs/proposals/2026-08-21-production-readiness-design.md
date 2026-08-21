# Design — production-readiness of the process engine (forced · updateable · portable · modular)

> **Status: DRAFT — pending human review. NOT applied, NOT in the backlog.** This is a
> Change-Request *statement + rationale* artifact (CLAUDE.md §3a) in the style of the
> 2026-06-30 and 2026-07-06 proposals. After human review, §7 is the mechanical plan for
> inserting the accepted parts into `.tracking/` under the normal discipline (each work
> package = a recorded Decision + chartered Stories). Authored 2026-08-21 from the
> full-repo process-rigor review (external critique artifact: "Keel Process Rigor").

## 1. Problem and goals

Keel's discipline machinery is real but has three production-blocking properties:

1. **It doesn't ship.** The entire in-loop stack (4 hooks, output style, permission
   denies, 35 skills) lives only in this repo's `.claude/`; `keel init` scaffolds none of
   it (`main.rs:1829-1911`). Downstream projects get prose + a commit gate whose
   `command -v keel || exit 0` silently passes.
2. **The processes are mostly prose.** ~130 `ProcessStep`s are free-text strings no code
   reads; only 6/21 process files assert any constraint; "launch" = one prompt paragraph
   (`serve.rs:1049`); ceremony order is a Rust constant duplicating the workflow topology.
3. **The teeth are monolithic while the prose is modular.** `GUARD_NAMES` is compiled;
   `keel rules` is wired to no gate; process units export with `rules = []` always
   (`activation.rs:97`) and import refuses collisions — install-once, no update path.

**Goal:** a downstream `keel init` project gets the *same* enforcement as the self-build;
processes carry their enforcement with them when exported; controls are declared data a
project can extend without forking the binary; process definitions have a real update path;
and routine AI work starts from a constrained launch boundary instead of freeform chat.

**Non-goals:** replacing the CLI (it remains the authority and automation substrate,
D0093); gating completeness (honest-state only, D0098); guarding judgments
(`process-enforcement.toml`'s unguardable set stays prose by design); a standalone frontend
app (rejected D0109, cautionary precedent D0152).

## 2. Architecture — the funnel stack

Every work package below serves one four-layer model. Nothing in it is new doctrine; it is
D0109's lever hierarchy assembled into one structure:

| Layer | Mechanism | Forces what | Status |
|---|---|---|---|
| **1 Boundary** | web launch surface: work starts as a declared process launch with approve-before-execute, fresh short context per run (`claude -p`, bounded turns) | *what work starts, with what context* — un-routed actions are not offered | partial (`serve.rs` bridge: 2 actions, no run ledger, no post-run gate) |
| **2 Action space** | permission denies + blocking `PreToolUse` hooks + typed write surface (MCP + CLI over `write.rs`) | *what the running agent can do* — invalid writes unrepresentable | unbuilt (the top lever of D0109, F4 of the levers doc) |
| **3 Verification** | validate + guards + declared rules at edit/turn/commit/CI boundaries | *what state can persist* | built for the self-build; rules unwired; doesn't ship |
| **4 Instruction** | output style, skills, CLAUDE.md | *judgment no gate can see* (DoR/DoD/critique quality) | built; doesn't ship |

The layers are complementary, not alternatives: the boundary shrinks the freeform aperture,
the action space constrains mid-run behavior, gates verify the residue, prose covers
judgment. Building any one layer "instead of" the others repeats the category error D0128
warned about for MCP.

## 3. Direction ruling — "mostly web-based"?

**Yes for the interaction boundary; no for the truth and automation substrate.** The
precise form that survives contact with the decision record:

- **Where work *starts* and is *reviewed* becomes web-first.** This is D0109's accepted
  direction verbatim ("the maturation of `keel serve` into a process-launcher +
  generation-time output-validator") and F6's finding that the interaction boundary is the
  one place the parse-first residual (issue059) becomes *structurally* enforceable: input →
  visible parse/route → approve → execute, with un-routed actions simply not offered.
- **Where truth lives and what automation calls stays CLI/JSON** (D0093's dual-surface
  invariant, unamended). The console launches and oversees; the binary computes and writes.
  All CLI functionality is retained — the launcher *is* the CLI's prompt-builders and write
  API behind HTTP, single binary, embedded console (D0152: no second app).
- **What the launcher buys against the freeform/prose problem** — the user's instinct is
  correct on all three counts:
  - *Fresh, narrow context per task* (lever #2). Each launch is a new `claude -p` with a
    directed prompt and `--max-turns`; conversational drift cannot accumulate across tasks.
  - *Closed action space at the start of work* (lever #1). Only declared
    `Process`/`AISkill` targets are launchable (`view::is_launchable` already enforces
    this); freeform chat is not an affordance.
  - *Output validation at the boundary* (lever #3). The launcher can refuse to close a run
    whose declared artifacts don't exist or whose tree doesn't gate green — a repair loop
    at run granularity, earlier than commit-time.
- **What it does not buy — stated to prevent over-claiming:** inside the spawned run the
  agent is still a full harness with Bash and Edit. `claude -p` runs the same agent and the
  same system prompt (F2 of the levers doc). The mid-run forcing comes from layers 2–3
  (P0/P1 below), which the spawned session inherits from the scaffolded `.claude/` — this
  is why P0 is a hard prerequisite of the launcher, not a parallel track. And interactive
  sessions do not disappear: CHANGE/design work is conversational by nature; the launcher
  owns the routine EXECUTE/RECORD/VIEW paths first, and the ratio shifts as launchable
  coverage grows.

## 4. Work packages

Labels continue the review's numbering. Each WP names its Decision(s)-to-record; per the
keystone lock, none applies without them.

### P0 — Ship the enforcement (init parity) · days · prerequisite of everything

- `keel init` scaffolds `.claude/settings.json` (4 hooks + permission-deny block +
  `outputStyle`), `.claude/output-styles/keel.md`, and `.claude/skills/<name>/SKILL.md`
  generated from `skills-registry.sysml`.
- Treat the scaffolded `.claude/` tree as a **computed view of `.engine/`**: a
  `keel sync-claude` command regenerates it; a new guard checks drift (marker `#View`
  semantics — regenerable, never hand-edited). This keeps text-is-truth intact.
- Scaffolded pre-commit: missing binary ⇒ **loud exit 1** (parity with the self-build hook).
- Add `SubagentStop` to the hook set (same gate as `Stop`).
- **Decision:** "the `.claude/` surface is engine-derived and ships with init."
- **Acceptance:** `init_smoke` extended — a fresh scaffold contains the hook config, the
  output style, N skills; `keel sync-claude --check` green; scaffolded hook fails loudly
  without a binary.

### P1 — Close the bypass ring (action-space layer) · ~1 sprint

- `PreToolUse` on `Write|Edit` for `.tracking/**`: **block**, message names the sanctioned
  command ("instance data — use `keel record issue …`"). Escape hatch for what the API
  doesn't cover (existing rule).
- `PreToolUse` on `Bash`: **block only unambiguous** patterns (redirection/`sed -i`/`tee`
  into `.tracking`, `git commit --no-verify`, `SKIP_VALIDATE=1`); everything ambiguous
  stays advisory (respects the issue076/081 over-strict-gate dynamic).
- **Wire `keel rules` into `hook stop`, pre-commit, and CI** — declared
  `EdgeRule`/`ElementRule` become enforced; downstream projects gain a no-fork extension
  point. (Severity mapping: blocking rules gate; warning rules report.)
- Fix `hook_stop` bypassing the activation filter (route through `run_all`'s filter —
  latent until any project writes an `activation.toml`).
- Add `pre-push` (no-rebase/no-force backing for CLAUDE.md:136's claim) or soften the claim.
- Stop-hook red-yield: instead of passing the second red pass silently, surface it as an
  obligation on the console queue.
- **Decisions:** "the write API is the only representable write path for `.tracking`";
  "declared rules are gate-evaluated."

### P5 — Launcher maturation (boundary layer; the "web-based" direction) · 2–3 sprints

The maturation of `serve` that D0109 named. Builds on the existing bridge
(approve-before-execute at `serve.rs:1189`, closed launch set, `claude -p` spawn,
concurrency caps, subscription billing per D0094).

1. **Full launch catalog.** Expand actions from {critique, launch} to every declared
   `Process`/`AISkill`, with per-process input forms generated from the model — the D0117
   generative substrate (`/api/schema`, `/api/surfaces`, `launchables`) already exists.
   Prompt construction stays pure-function per process (the `build_*_prompt` pattern).
2. **Run ledger as tracked facts.** Today the bridge records nothing. Author a run record
   per launch (process, launcher, prompt hash, transcript path, exit status,
   `judgedAgainst` SHA); append the post-run gate result. Transcripts stay on disk,
   referenced — not embedded (atomic items, not blobs).
3. **Post-run gate.** On agent exit: `validate` + guards + rules over the touched tree.
   Red ⇒ run marked failed, writes surfaced for review — never silently merged into the
   human's queue as if green.
4. **Isolation option (fork for review, §8):** each run executes in a **git worktree**;
   keel gates the worktree tree; the human's approval merges it (merge-never-rebase,
   D0129). This makes the human commit-gate (D0094/D0096) *physical*: an ungated run
   cannot touch the mainline tree at all. Strongest form of the boundary; costs
   worktree plumbing + Windows path handling.
5. **Structured completion contract.** A launched run must end by declaring its produced
   artifacts (the process's `producedArtifact` made checkable); the launcher verifies
   existence via the model and re-prompts on mismatch, bounded retries (lever #3 at run
   granularity).
6. **Spawn hardening.** Pass explicit permission mode/allowed tools; assert the spawned
   session sees the scaffolded hooks (P0); keep `--max-turns`/concurrency caps.
- **Decisions:** "the console is the launch boundary for routine EXECUTE/RECORD work"
  (extends D0109/D0152, leaves D0093 unamended); isolation-model decision per §8.

### P2 — `keel mcp`: typed tool surface (action-space layer) · 1–2 sprints

Unchanged from the review, now explicitly positioned *inside* launched runs and interactive
sessions alike — the paved road on top of P1's denials:

- stdio JSON-RPC in the same binary (`keel mcp`), hand-rolled over `serde_json` (house
  style, no new heavy dep); `keel init` writes `.mcp.json`.
- Reads wrap `view.rs` (nearly free). Writes require the extraction sprint: `accept`,
  `apply-review`, `record issue` preflights, `enroll`, `actor set` move from `main.rs`
  into `write.rs` so CLI/HTTP/MCP share one implementation. Refusal paragraphs travel into
  MCP error payloads verbatim — the refusal text is the affordance.
- `tools/list` computed from schema + launchables + `activation.toml` (deactivated process
  ⇒ tools absent). Declaration-parity test mirroring `hardening.rs:519`.
- **Governance first:** record the untriaged Issue on the failing `nAiGovernanceInLoop`
  critique (`critiques.sysml:2140`); record a Decision classifying MCP into D0093's
  dual-surface model (same authority slot as CLI/JSON — a third *transport*, not a third
  *surface of truth*) and reaffirming D0128's surface-not-governance framing.

### P3 — Declarative controls, finished (verification layer) · multi-sprint, parity-gated

- Execute D0105 as spiked: 3 rule kinds (Edge/Element/Ordering) + ~4-predicate scope
  sub-language + declared exemptions; migrate guards to rules behind the parity gate
  (`keel check` == 39-guard output before any Rust predicate retires).
- Derive ceremony ordering from workflow `first…then` topology; delete `GATE_ORDER`.
- Author the missing successions in process files (human-approved authoring pass — the
  migration correctly refused to synthesize them).
- Promote `[judgment]`/`[computed:]` to a typed `evaluability` attribute; `keel hardening`
  computes the enforcement map from the model; retire the hand-kept
  `process-enforcement.toml` and its namespace mismatch with `activation.toml`.
- Extend the keystone lock to `.engine/skills/` and `.engine/contracts/`.

### P4 — Process update path · ~1 sprint, after P3

- `keel process import --update`: collision ⇒ import as a **superseding** definition
  (`supersede` edge + mandatory `#ProspectiveChange`/`#SafetyChange`), so
  `reprocess-candidates` fires downstream. Install-once becomes upgradeable with the
  retroactivity semantics already defined.
- Generalize `governing-version` off the hardcoded `delivery.sysml` path (resolve per item
  from its charter edges).
- Version-handshake imports: diff the unit's required guards/rules against the binary's
  inventory (`keel version` already prints it); refuse or loudly degrade with a recorded
  Issue. Never "prose lands, teeth silently missing." (Requires P3 so rules can travel in
  units at all — fixes `unit.rules = []`.)

### Hygiene (ride-along, no Decision needed beyond doc-sync)

`guards.md` 35/26 → 39/30; ghost `process-units.toml` reference in the generated
`activation.toml` header; record the `nAiGovernanceInLoop` critique Issue.

## 5. Sequencing

```
P0 ──► P1 ──► P5 (launcher) ──► P2 (MCP) ──► P3 (declarative controls) ──► P4 (update path)
 └──────────────── hygiene items ride any adjacent commit ────────────────┘
```

P0→P1 are prerequisites of everything (a launched run without shipped hooks is a freeform
agent with a nicer start page). P5-before-P2 follows the user's stated priority and
D0109's letter; P2-before-P5 is defensible if the extraction sprint should land while the
launcher is being designed — fork for review. P3/P4 are the long arc and match D0128's
recorded queue (in-loop gate → modular unit → catalog/exchange → MCP already partially
inverted by doing MCP at P2; the reconciling Decision in P2 covers this).

## 6. What "production-ready" means, testably

- A `keel init` project, with no manual steps beyond `git config core.hooksPath`, has:
  blocking in-loop gates, the output style, discoverable skills, and a loud-fail commit
  gate. (P0 acceptance test.)
- A direct edit to `.tracking/**` in a Claude session is refused with the sanctioned
  command named. (P1.)
- A project adds a blocking control by *declaring a rule* — no Rust, no fork — and the
  rule gates turns, commits, and CI. (P1+P3.)
- A process exported from project A lands in project B with its rules and guard
  requirements verified against B's binary, and can later be *updated* by re-import with
  supersession. (P3+P4.)
- Routine work is launched from the console against a declared process, runs in bounded
  fresh context, and cannot be marked complete while its tree gates red. (P5.)

## 7. Backlog insertion plan (post-review, mechanical)

For each accepted WP, under the normal discipline (no bulk shortcut):

1. `CHANGE` route: record the WP's Decision(s) in `.engine/decisions/` with
   `#ProspectiveChange`, human acceptance via `keel accept`.
2. Charter Stories per WP scope item (`keel add-task`, `#CharteredBy` the Decision;
   declaration order = priority, D0052).
3. P0's new guard + P1's rule-wiring are themselves process-def changes → keystone
   Decisions co-committed.
4. Hygiene items: `TRIVIAL`-labeled or folded into the nearest WP commit with doc-sync.

Nothing in this document enters `.tracking/` until the human directs it.

## 8. Open forks for the human

1. **Run isolation (P5.4):** worktree-per-run with gated merge (strongest, most plumbing)
   vs in-place writes + post-run gate (cheaper, weaker)?
2. **Order:** P5 before P2 (boundary first, user's lean) vs P2 before P5 (typed surface
   first, smaller)?
3. **Interactive sessions' end-state:** launcher-first for EXECUTE/RECORD with interactive
   reserved for CHANGE/design — or coequal indefinitely?
4. **Run ledger residence:** tracked facts in `.tracking/` (text-is-truth, but noisy) vs a
   local `.keel/runs/` referenced by occasional tracked summaries?
5. **Stop-hook red-yield replacement:** console obligation (proposed) vs hard block with
   human override token?
