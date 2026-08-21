# Design — production-readiness of the process engine (forced · updateable · portable · modular)

> **Status: r4 FINAL (panel-unanimous) — pending human acceptance.** This is a
> Change-Request *statement + rationale* artifact (CLAUDE.md §3a) in the style of the
> 2026-06-30 and 2026-07-06 proposals. A 9-member adversarial review panel (2 pessimists,
> 2 optimists, 3 correctness critics, 1 reduction advocate, 1 systems engineer) reviewed
> it across three rounds under an honesty rule (no manufactured objections, no
> capitulation): round 1 — 2 ACCEPT / 7 OBJECT; round 2 (vs r2) — 5 ACCEPT / 4 OBJECT;
> round 3 (vs r3) — **9 ACCEPT / 0 OBJECT, unanimous and uncompelled**. r4 applies the
> panel's final charter-time line edits (§12). §11 maps findings to resolutions. §9 is the
> mechanical backlog-insertion plan. All file:line citations are pinned to HEAD `74ecefb`
> (2026-08-21).
> **Coordination note:** the working tree carries uncommitted concurrent edits
> (`main.rs`, `guards.rs`, `lib.rs`, new `scaffold.rs`) — working-tree line numbers
> already drift from HEAD; reconcile before chartering P0.

## 1. Problem and goals

Keel's discipline machinery is real but has three production-blocking properties:

1. **The in-loop half doesn't ship.** The hooks, permission denies, and the output style —
   the response contract that exists precisely because CLAUDE.md was "demonstrably
   ignored" (issue084/085) — live only in this repo's `.claude/`; `keel init`
   (`main.rs:1829-1911` @74ecefb) scaffolds none of them. The 35 skills DO ship (embedded
   in the binary, copied under `.engine/skills/`), but land where no harness discovers
   them — shipped-but-undiscoverable. The scaffolded pre-commit passes with only a printed
   notice when the binary is absent (`PRECOMMIT_HOOK`, `main.rs:62`). A binary release
   channel EXISTS and works — v0.1.0–v0.2.3 are tagged, and the release workflow's runs
   are verified in the record (sprint259: three platforms built and attached, checked via
   `gh release view`; sprint264: the v0.2.3 asset downloaded, executed, version-matched) —
   but nothing downstream points at it: the scaffolded remedy text names no install path.
   (`d0128-direction-requirements.sysml:96`'s claim of one stale GitHub release and a
   Deploy workflow "defined but has never run" was true when authored on 2026-08-13 and
   falsified the next day; the repo's own `nReleaseDisciplineCorrectnessCritique` already
   flags the same claim's staleness where it appears in the Need `nReleaseDiscipline`.
   Cited here as dated historical motivation only.)
2. **The processes are mostly prose.** The 116 `ProcessStep`s are free-text strings no
   code reads; only 6/21 process files assert any constraint; "launching" a process hands
   an LLM one paragraph (`serve.rs:1049`); ceremony order is a Rust constant
   (`GATE_ORDER`, `guards.rs:279`) duplicating the workflow topology the parser reads.
3. **The teeth are monolithic while the prose is modular.** `GUARD_NAMES` is compiled
   (`guards.rs:1604`); `keel rules` is wired to no gate; process units export with
   `rules = []` always (`activation.rs:97`) and import refuses collisions
   (`process_cmd.rs:153`) — install-once, no update path.

**Goal:** a downstream `keel init` project gets the *same* enforcement as the self-build,
installable from the existing release channel; processes carry their enforcement with
them when exported and can be updated after install; controls are declared data a project
can extend without forking the binary; routine AI work starts from a constrained launch
boundary; and the effect of all of it is **measured**, not presumed.

**Non-goals:** replacing the CLI (it remains the authority and automation substrate,
D0093); gating completeness (honest-state only, D0098); enforcing judgments — the
`process-enforcement.toml` unguardable set stays *unenforced* by design (its
classification may later become typed data, P3b, but no gate is conjured for a judgment);
a standalone frontend app (rejected D0109; cautionary precedent D0152); an MCP server now
(§5-P2: direction-only, trigger-gated); package managers, auto-update, or install
scripting beyond documentation (D0157-pattern future triggers — the distribution scope is
fenced to the documented install path and remedy text).

## 2. Threat model and the guarantee hierarchy

Stated explicitly because the panel showed the r1 draft over-claimed. The adversary model
is a **cooperative-but-drifting agent**, not a malicious one: the recorded failure mode is
drift and laziness ("oops, I did what I said I wouldn't" — D0106 violated after adoption;
issues 082–085), plus one recorded self-inflicted control bypass under the full live stack
(issue116: `core.hooksPath=/dev/null`). Against an agent with Bash, action-space controls
are *friction*, not proof — enumerable bypass patterns are infinite, and the repo's own
pre-bash hook is deliberately advisory (issue076/081: an over-strict gate trains its actor
to disable it).

The guarantee therefore lives in layers, weakest-to-strongest:

| Layer | Behavior required (implementation-free) | Current binding | Strength |
|---|---|---|---|
| **4 Instruction** | judgment the machine can't check is stated where the agent reliably receives it | output style, skills, CLAUDE.md | advisory |
| **1 Boundary** | enforced work enters as a declared, approved process instance in fresh bounded context | console launch + `claude -p --max-turns` | shapes what starts |
| **2 Action space** | invalid writes are refused with the sanctioned path named; refusals and overrides leave durable records | permission rules + blocking PreToolUse + the typed write API | friction + audit |
| **3 Verification** | no persisted state survives that is untruthful/malformed/untraceable; verdicts are re-derivable from the tree independent of any hook having run | validate + guards + rules at edit/turn/commit/CI + `keel audit-history` | **the guarantee** |

Layer 3 is the only layer the record shows working unconditionally (the issue log is
substantially guard-discovered; `keel audit-history` — built as the D0047 control for
issue116 — re-derives every commit's verdict from the tree, hook-independent). Layers 1–2
exist to make drift rare and visible, not to be the proof. Building any one layer
"instead of" the others repeats the category error D0128 warned about.

## 3. Kernel invariants (what must be true, however implemented)

The panel's systems engineer derived these; the plan adopts them as its requirement
skeleton. Each maps to committed scope (WP in parentheses); none is orphaned.

- **K1** No tracked-model state transition except through a validated channel or a
  recorded, surfaced override — and no such transition survives gating unverified
  (P1, P5; the override residue is K7's audit trail, and layer 3 is the backstop).
- **K2** Every enforcement point fails loud — absent, erroring, or timed-out enforcers
  never *silently* pass; the full enforcement-point inventory with its
  absent/error/timeout behavior is enumerated and reported (P0.7's hardening extension;
  hook-timeout expiry resolves to allow at the harness layer, so it is logged to the PM
  ledger and surfaced as a recorded residual, never left invisible).
- **K3** Enforcement parity: downstream projects and spawned runs get the self-build's
  enforcement, and inheritance is *proven* (behavioral acceptance + fire-ledger evidence),
  not assumed (P0, P5).
- **K4** Enforced work enters as a declared, human-approved process instance; undeclared
  actions are not offered at the boundary (P5; residual: interactive sessions remain, §4).
- **K5** Work is not recorded complete while its declared obligations are unverified;
  completion is computed from evidence (P5 post-run gate; form-only — substance stays
  human, D0094/D0096).
- **K6** Records of human judgment (acceptance, confirmation) can originate only from a
  channel the agent cannot invoke: the write layer refuses AI-kind actors, the harness
  layer treats the human-judgment CLI subcommands and actor-identity mutation as
  protected (never exempt), and acceptance channels are human-held (P1.2/P1.3/P1.4).
- **K7** Weakening a control (deactivation, exemption, hook/config edit, hooksPath,
  actor rebinding) is either human-approved at the moment of action or leaves a durable,
  surfaced record — and always both a record and visibility (P1.4; PM). Override unlocks
  are single-use, target-path-bound, and session-bound, and the recorded fact names the
  path actually written.
- **K8** A process definition and its enforcement are one distributable unit; import
  verifies the consumer can enforce it and refuses or loudly degrades (P4a/P4b).
- **K9** A process definition is updateable by supersession without identity/history loss,
  through the same governed channel as any other process change (P4a).
- **K10** A definition change propagates re-evaluation obligations to work done under the
  prior definition (P4a: generalized `governing-version` + `reprocess-candidates`).
- **K11** Controls are declared data; a consumer adopts/extends/retires them without
  modifying the engine, and adoption changes are governed (P1.5, P3a, K7).
- **K12** Every enforcement verdict is traceable to the definition version and tree it was
  judged against (existing doctrine; P5 run records carry the run-start fingerprint and
  `judgedAgainst`).
- **K13** Every refusal names an available sanctioned path — deny-and-provide — or agents
  route around it (P1 refusal texts; the CLI is the current typed binding; MCP is a future
  binding of the same need, §5-P2).
- **K14** The effect of enforcement is measured: fires, blocks, overrides, and adherence
  defects are counted, and promotion of any advisory control to blocking cites that
  evidence (PM; D0128's own "prove the in-loop gate (measure)" step, recorded as
  undelivered by D0130).
- **K15** Every enforcement verdict is re-derivable from committed truth alone,
  hook-independent (validate/guards over the tree + `keel audit-history`, P1.6) — this is
  §2's layer-3 guarantee stated as an invariant so no future cut can orphan its carrier.

## 4. Direction ruling — "mostly web-based"?

**Yes for the interaction boundary; no for the truth and automation substrate.**

- Where work *starts* and is *reviewed* becomes web-first. This is D0109's accepted
  direction — action-space constraints realized as, in the words of the levers proposal
  it chartered, "the maturation of `keel serve` into a process-launcher +
  generation-time output-validator" (`2026-07-06-ai-discipline-levers.md:60-61`) — and
  F6's finding that the interaction boundary is the one place the parse-first residual
  (issue059) becomes structurally enforceable: input → visible route → approve → execute,
  with un-routed actions not offered.
- Where truth lives and what automation calls stays CLI/JSON (D0093, unamended). Single
  binary, embedded console (D0152). All CLI functionality retained.
- What the launcher buys: fresh narrow context per task (lever #2); a closed action space
  at the start of work (lever #1; `view::is_launchable` already enforces the declared
  set); run-exit verification (lever #3 at run granularity, P5).
- What it does not buy: inside a spawned run the agent is a full harness with Bash and
  Edit — `claude -p` runs the same agent and system prompt (levers doc F2). Mid-run
  forcing comes from layers 2–3 *as far as P0/P1 build them*; the launcher shrinks the
  freeform aperture, it does not replace gates. Interactive sessions remain for
  CHANGE/design work; the launcher owns routine EXECUTE and RECORD paths (VIEW needs no
  launch — reads are free). "The ratio shifts as launchable coverage grows" is a
  **hypothesis**, measured by PM (fraction of EXECUTE/RECORD work launcher-initiated,
  per sprint), not a claim.

## 5. Work packages

Binding authority note: WPs are bound by CLAUDE.md invariant 5 and the CHANGE route —
every WP records its named Decision(s) with explicit human acceptance. The D0070 keystone
lock additionally fires only on commits staging `.engine/processes/`/`.engine/workflows/`
files (extended by P3a); it is cited below only where it actually applies.

### P0 — Ship the enforcement (init parity + install path) · ~1 sprint · prerequisite of everything

*Decisions to record:* **D-P0a** "the `.claude/` surface is engine-derived,
keel-scope-owned, and ships with init — and owns the protected-path deny list";
**D-P0b** "the scaffold points at the existing release channel; distribution scope is
fenced to the documented install path" (no package managers, no auto-update, no install
scripts — D0157 triggers).

1. `keel init` scaffolds `.claude/settings.json` (five hook events: UserPromptSubmit,
   PostToolUse, Stop, PreToolUse, SubagentStop), the keel output style, the keel-owned
   permission rules **and the protected-path deny list (owned by D-P0a; P1's D-P1a owns
   only the ask/override/Bash-matcher semantics layered on top)** — the self-build's
   current deny block is personal, not keel content — and one
   `.claude/skills/<name>/SKILL.md` per `skills-registry.sysml` entry (counts asserted
   equal). Also scaffolds an optional CI workflow template
   (validate + guard + rules + `audit-history`) so downstream layer 3 has a
   hook-independent home; projects with their own CI get a documented one-line wiring.
   (Single-vendor template only; the `rules` step's blocking behavior activates with
   P1.5.) **The fire-ledger *emit* plumbing — hooks writing fire/block/override events to
   `.keel/metrics/` — ships here in P0** so P0.8's acceptance is self-contained; PM owns
   the schema freeze, red-yield/duration capture, analysis, and promotion evidence.
2. **Ownership model (not byte-equality):** `settings.json` is mixed-ownership by
   construction — downstream users add their own hooks/permissions. `keel sync-claude`
   deep-merges, owning only keel-identified entries (hook commands invoking `keel hook`,
   the keel output style, keel-named permission rules); the drift check compares only the
   keel-owned subset and stamps the generator version, reporting version skew as a
   "regenerate" obligation rather than a violation. The drift check IS
   `keel sync-claude --check`, registered as one guard — one implementation, one surface.
3. **Fail-loud without the binary (K2):** the scaffolded PreToolUse path test for
   protected paths is a pure shell test needing no binary (deny-by-default with a loud
   message when `keel` is unresolvable); all scaffolded hooks resolve the binary via
   `KEEL_BIN` (absolute, injected) then PATH — never a cwd-relative `target/` probe; a
   missing binary emits a visible warning, never `|| true` silence. The scaffolded
   pre-commit fails loud (exit 1) with a remedy line pointing at the release channel's
   documented install path. `init` sets `core.hooksPath` when `.git` exists; `orient`
   warns loudly when unset.
4. **Adoption profiles — declared, never inferred** (field-data constraint,
   issue089/issue129 — two downstream lockouts in one week, both existing-repo
   adoptions): `keel init --profile strict|guided`, explicit, printed, and recorded as a
   declared fact in a contract file so the profile is model-visible. Default only for a
   provably empty directory (strict — a fresh scaffold starts green, `init_smoke` proves
   it); any directory with existing content requires the flag. Guided = advisory-first;
   one command promotes to blocking, promotion cites PM evidence (K14).
5. **Install path:** document the downstream install (the release channel already ships
   verified multi-platform binaries, §1.1); the scaffolded remedy texts point at it;
   state the release-cadence/latest-version expectation. Fenced per D-P0b.
6. **SubagentStop** gates only when the tree changed during that subagent's lifetime —
   baseline captured at the subagent's first PreToolUse fire (keyed by session id),
   degrading to `systemMessage` when no baseline exists; read-only subagents get
   `systemMessage`, never block.
7. **Enforcement-point inventory (K2):** extend `keel hardening` to enumerate every
   enforcement point (hooks, pre-commit, CI steps, guards, rules) with its
   absent/error/timeout behavior; timeout-resolves-to-allow points are listed as recorded
   residuals with their PM-ledger visibility.
8. *Acceptance (behavioral, not presence):* on Windows and Linux, in a fresh scaffold —
   a direct write to a protected `.tracking` path is refused with the sanctioned command
   named (tests D-P0a's deny list); a commit with the binary absent blocks with the
   remedy text; skills are discoverable in a live Claude Code session;
   `sync-claude --check` green; hooks demonstrably fire (fire-ledger evidence) in a
   spawned `claude -p` session; the hardening inventory lists every enforcement point;
   the profile fact is recorded and printed.

### P1 — Pave the write path (friction, human channels, rules wired) · ~1 sprint

*Decisions to record:* **D-P1a** "API-owned facts are written only through the write API;
override unlocks reach every tier and are recorded"; **D-P1b** "declared rules are
gate-evaluated" (its own Decision — this is the modularity keystone and must not ride
along); **D-P1c** "writes recording human judgment are human-channel-only (K6)";
**D-P1d** "control-plane writes — including actor-identity mutation — are approval-gated
and recorded (K7)".

1. **Coverage-scoped blocking, not a blanket deny** (the write API covers ~9 fact
   operations; CLAUDE.md §4 sanctions direct editing for the rest — a blanket block would
   deadlock against the Stop hook's "fix it" demand and train the Bash bypass):
   - **hard deny — absent a recorded override —** on the API-owned fact surfaces (test
     results, gate results, issues, decisions, backlog task entries), refusal naming the
     exact command. The override reaches this tier too: repairing a corrupted API-owned
     file (merge markers, a torn block) is not one of the 9 operations, and a tier the
     unlock cannot reach recreates the deadlock;
   - **`permissionDecision: "ask"`** for other `.tracking/**` writes — the human approves
     in the harness prompt (itself a human channel). **Headless mapping:** in spawned
     runs there is no prompt, so the launcher maps "ask" to a console proxy (live
     approval on the approve queue) when the console is up, else auto-deny + a recorded
     obligation on the queue; per-process pre-approved write scopes narrow how often it
     fires. D-P5b names this mapping; acceptance includes one launched run exercising it;
   - **recorded override** for everything the API doesn't cover: a sanctioned unlock
     (reason required), **single-use, target-path-bound, session-bound**; the
     auto-recorded obligation fact names the path actually written (K7). Never a silent
     env-var;
   - path matching normalizes Windows separators (as `guards.rs:480` already does).
2. **Bash matcher tokenizes argv** — no raw-string regex (a commit message *describing*
   `--no-verify` must not match). It exempts commands whose argv[0] resolves to keel,
   **except the human-judgment and actor-identity subcommands, which are never exempt**:
   `keel accept`, confirmation-method result writes, `keel actor set`, `keel enroll` of a
   Person, and invocations carrying `KEEL_ACTOR=`/`--by`/`--judged-by` naming a Person
   route through the "ask" tier + a recorded fact (closes the K6 seam: without this
   carve-out, an agent on the human's machine — where `.keel/actor` binds a Person —
   could record the human's sign-off via Bash). Blocks only unambiguous operator-level
   patterns: redirection/`tee`/`sed -i` targeting `.tracking` **or `.claude/`**,
   `git commit --no-verify`, `SKIP_VALIDATE=1`, `SKIP_KEEL=1`,
   `git config core.hooksPath` (the issue116 vector). Ambiguous → advisory, promotion
   evidence-gated per K14 (honoring the repo's own accepted conditionality: blocking
   tiers "deliberately UNDELIVERED until the turn-boundary gate has produced evidence",
   `d0128-direction-requirements.sysml:52` — P0's strict profile covers the *unambiguous*
   set only).
3. **Human-judgment channel (K6):** `write::accept_decision` and `method=confirmation`
   results refuse when the resolved actor is `ActorKind::ai`; additionally, `accept` in a
   session bearing agent-environment markers requires a console-issued or TTY-interactive
   confirmation, so the check does not rest on the actor binding alone (the binding is
   agent-mutable state — hence the P1.2 carve-out and P1.4 coverage). Acceptance flows
   through channels the human holds (their terminal, the console approve queue). No
   agent-callable surface ever exposes `accept`.
4. **Control-plane protection (K7):** `Write|Edit` on `.claude/settings.json`,
   `.githooks/**`, the output style, and Bash `git config core.hooksPath` /
   `keel deactivate` / exemption writes / **actor-identity mutation (`keel actor set`,
   Person-enrollment, `KEEL_ACTOR`)** → "ask" + an auto-recorded, orient-visible fact.
5. **Wire `keel rules` into `hook stop`, pre-commit, and CI** (blocking rules gate;
   warning rules report). Fix `hook_stop` bypassing the activation filter
   (`guards.rs:2270-2288`).
6. **Tree-derived audit in CI (K15):** add `keel audit-history` over the pushed range to
   CI (hook-independent verification, the issue116 control); soften CLAUDE.md:136's
   "enforced by remote config and hooks" to what is actually enforced — no new pre-push
   hook (one honesty edit beats permanent surface).
7. **Red-yield:** the second red pass still yields (loop-avoidance stands) but records a
   tracked obligation fact visible in `orient` (console renders it when up — the console
   is chronically down during rebuilds, issue150, so the fact must not live only there).

### PM — Measure the enforcement (K14) · small, parallel with P1

*Decision to record:* **D-PM** "enforcement effect is measured; advisory→blocking
promotions cite the measurements; the sprint window N is fixed here (default 3)".

**Entirely machine-local** (`.keel/metrics/*.jsonl`, gitignored class — resolved from
r2's open fork B: no tracked summaries until a consumer for them is named, D0144): hook
fires/blocks/overrides/red-yields per session **keyed by session id and event type — this
fire-ledger is also the single instrumentation path the P0.8/P5.3 "hooks actually fired"
checks read** (one write path, two consumers; no separate heartbeat mechanism; the emit
plumbing itself ships in P0.1, so PM here is schema, capture breadth, and analysis);
launcher-run durations/turn counts. Adherence trend uses the already-tracked
`#ProcessDefect` issues. §7 gains a measured criterion. This is D0128's
recorded-but-undelivered step 1, delivered.

### P5 — Launcher maturation (boundary layer) · ~2 sprints, demand-driven increments

*Decisions to record:* **D-P5a** "the console is the launch boundary for routine
EXECUTE/RECORD work" (recommendation contingent on human review; extends D0109/D0152,
leaves D0093 unamended); **D-P5b** "run records, their gate results, the headless-ask
mapping, and single-writer-per-tree during a run". (The run-branch amendment to
CLAUDE.md's "commit directly to main" is NOT recorded now — it belongs to the worktree
trigger Decision, recorded if/when P5.6's DoR is met.)

1. **Launch catalog, thin:** the closed launch set already exists (`is_launchable`,
   `serve.rs:1062`); add per-process input forms generated from the model (`/api/schema`,
   `/api/surfaces` — D0117 substrate). Breadth is demand-driven; depth (items 2–4) ships
   first if scope pressure hits.
2. **Run records, hybrid residence:** per-run local log (`.keel/runs/<id>.jsonl`:
   transcript path, duration, turn count, exit status — machine-local by declared schema,
   exempt from existence checks) + **one tracked summary fact per run with a non-empty
   diff, one file per run** (empty-diff runs stay local-only — nothing to review), written
   by the launcher *after* the post-run gate **under the run's stamped actor, not the
   machine binding**, carrying the run-start fingerprint and `judgedAgainst` (K12).
   Summary files get a roll-up/archival policy under the governed migration process
   before they can grow turn-gate latency without bound. Consumers, named per D0144: the
   console run view, PM, and the human's review queue.
3. **Post-run gate (K5, form-only):** launch **refuses on a dirty tree** and snapshots
   the tree fingerprint at spawn; on agent exit the launcher runs
   validate + guards + rules over the run's diff — computed **run-start snapshot →
   working tree** (not the index, which is vacuous for uncommitted runs; not the mainline
   fork point, which smears prior residue) — and evaluates ownership against the run's
   stamped actor. Writes not attributable to the run's session (fire-ledger session
   keying) are flagged, not silently folded in. Red ⇒ run recorded failed; the run's diff
   is routed to human review **unconditionally, green or red** — the gate verifies form;
   substance verification is the human diff review (D0094/D0096; hollow-pass history:
   D0050, issue092, issue099). The gate also checks the PM fire-ledger for the run's
   session — evidence against *accidental* enforcement loss (missing KEEL_BIN, unloaded
   settings), not tamper-proof, and conditional on the transcript actually containing
   matched tool calls.
4. **Minimal completion check:** a launched run must have recorded its outputs through the
   write API (at least the process's declared result kind exists for this run), with
   bounded re-prompt on failure. Full artifact contracts wait until `producedArtifact` is
   typed (P3b trigger) — no retry machinery on a free-text field.
5. **Spawn hardening:** absolute `KEEL_BIN` injection into the spawned environment;
   explicit permission mode with the headless-ask mapping (P1.1/D-P5b); per-run
   wall-clock timeout (kill + "timed-out" status — `--max-turns` bounds turns, not
   stalls); duration/turns recorded (quota burn visible, D0094).
6. **Isolation:** in-place execution with the post-run gate is the committed mode; it
   pins launcher concurrency to 1 **and requires single-writer-per-tree during a run**
   (D-P5b) — a concurrent interactive session writing the same tree would be attributed
   to the run. Stated consequence: consecutive launches serialize behind human
   review/commit of the prior run — a new launch requires the prior run's tree committed
   or reverted. Worktree-per-run is the target state for concurrency > 1, **deferred**
   behind a named DoR: `KEEL_BIN` injection proven (P0.3), per-worktree actor stamping,
   snapshot-based gating (item 3), a run-branch policy Decision amending CLAUDE.md's git
   section, Windows worktree smoke test. Recorded as direction-with-trigger,
   D0157-pattern.

### P4a — Update and exchange (after P1; independent of P3) · ~1 sprint

*Decision to record:* **D-P4a** "process units are versioned, updateable by supersession,
and imports handshake enforcement" (P4b executes under D-P4a jointly with D-P3a).

Panel-corrected design constraints (stem-keying breaks everything otherwise):
- **Unit identity is a versioned id in the manifest, not the file stem.**
- **Import writes an install record** (versioned unit id, version, per-file content
  hashes) as a tracked fact — the three-way base `--update` needs; pre-P4a installs
  bootstrap by treating local as base with explicit human confirmation.
- `import --update`: three-way comparison (recorded base / upstream-new / local) — local
  `assert constraint` additions are never silently clobbered; divergence refuses with a
  report. The superseded definition's supersede edge lands with the mandatory
  `#ProspectiveChange`/`#SafetyChange` marker so `reprocess-candidates` fires (K9/K10).
- Import rewrites `activation.toml` atomically (old unit out, successor in); `guard_state`
  treats a guard as active if *any* active unit asserts it (fixes first-match-wins,
  `activation.rs:223-234`).
- **Version handshake, guard-names slice:** import diffs the unit's required guards
  against the binary's inventory (`keel version` prints it); missing teeth ⇒ refuse or
  loudly degrade with a recorded Issue (K8). Guards travel by name today; rules travel
  with P4b.
- Generalize `governing-version` off the hardcoded `delivery.sysml` path (`govern.rs:17`)
  to any process/workflow def, resolved per item from its charter edges (K10).

### P3a — Modular teeth, minimum slice (enables P4b) · ~1 sprint

*Decision to record:* **D-P3a** "rules are attributable to process units; the keystone
lock covers skills and contracts".

- **Per-process rule attribution** — the declared-rule → owning-unit association that
  `unit.rules = []` (`activation.rs:97`) lacks; this, not the guard migration, is P4b's
  actual prerequisite.
- **Keystone-lock extension** to `.engine/skills/` and `.engine/contracts/` (cheap; a
  skill body or activation policy is process definition).

### P4b — Rules travel in units · small, after P3a and P4a

Extends the P4a unit format: declared rules export/import with the unit and are verified
by the handshake. (Completes K8.) Executes under D-P4a + D-P3a jointly (no separate
Decision).

### P3b — Declarative-controls completion · DEFERRED, trigger-gated

*Recorded as a direction Decision (D0157 pattern). Triggers:* the first downstream
project that needs a built-in control changed; **or P5 requires typed artifact
contracts**; or the human reraises. Scope when triggered: guard→rule parity migration
behind the D0105 parity gate (guards without a rule expression remain compiled — the bar
is *declared rules gate* and *migrated guards match parity*, never 100% migration);
`GATE_ORDER` derived from workflow topology; the succession authoring pass; **typed
`producedArtifact`** and typed `evaluability` + retirement of `process-enforcement.toml`
and its namespace mismatch. Rationale for deferral: P1.5 already delivers the modularity
goal ("extend without forking"); migrating 39 compiled guards changes no downstream
behavior; D0054 names framework upkeep the #1 risk.

### P2 — MCP surface · DEFERRED, direction-only Decision

*Decision to record:* **D-P2** — direction, not scope (D0157 pattern): MCP is a future
*binding* of K13 (deny-and-provide: refusals must name a path that works — currently
bound by the CLI's refusal paragraphs) and of harness portability (`.claude/` enforcement
is Claude-Code-specific; a `.mcp.json` travels to any MCP harness — the one real argument
for building it). **Triggers:** a second harness in actual downstream use, or a named
consumer (D0144). To make the trigger fire-able rather than passive, the downstream docs
state the harness-support matrix (on a non-Claude harness you get the CLI, commit/CI
gates, and `audit-history`; in-loop enforcement is Claude-Code-bound; MCP is a recorded
direction — ask if you need it). This resolves rather than papers over the D0128
sequencing conflict: MCP stays fourth-or-later in its own recorded queue, and the failing
independent critique (`critiques.sysml:2140` — "the clause was never the requirement") is
answered, not overridden: the Issue is recorded with **D-P2 as its `#Resolves` target**
(RECORD route — owned here, not by hygiene). The write-path extraction (`accept`,
`apply-review`, `record issue` preflights, `enroll`, `actor set` out of `main.rs`) is
good refactoring that rides whenever those handlers are next touched; `accept` is
additionally bound by K6 and never becomes agent-callable on any future surface.

### Hygiene (ride-along)

- TRIVIAL (doc lines): `guards.md` 35/26 → 39/30; ghost `process-units.toml` reference in
  `ACTIVATION_HEADER`; CLAUDE.md:136 wording (P1.6); README harness-support matrix line
  (P2's trigger enablement) and downstream-install pointer (P0.5).
- RECORD (atomic facts, normal route): the `nAiGovernanceInLoop` critique Issue — recorded
  under D-P2 (above), listed here only for completeness.

## 6. Sequencing

Hard dependencies (arrows) vs recommendations (annotations):

```
P0 ──► P1 ──┬──► P5           (recommended next: boundary value, user priority)
            ├──► P4a ──► P4b  (P4a parallel-safe with P5)
            └──► PM           (parallel with P1 itself)
P3a ──► P4b                   (P3a independent; schedule at will)
P3b, P2: trigger-gated direction Decisions — no scheduled slot
```

P0→P1 is the only prerequisite chain everything shares (a launched run without shipped
hooks is a freeform agent with a nicer start page). P5 and P4a are independent after P1;
P5-first is the recommendation (the human's stated priority and D0109's letter), not a
dependency. P4b requires both P4a (unit format) and P3a (rule attribution). **P0+P1 alone
constitute a shippable increment; every later WP is independently abortable without
stranding the earlier ones.**

## 7. What "production-ready" means, testably

- A fresh `keel init` project, on Windows and Linux, with no manual steps: blocking
  in-loop gates that provably fire (fire-ledger), discoverable skills, the output style,
  a loud-fail commit gate whose remedy text points at the documented install path, a
  recorded adoption profile, and a hardening inventory of every enforcement point. (P0)
- A direct edit to an API-owned `.tracking` fact surface is refused with the sanctioned
  command named (deny list owned and shipped by P0/D-P0a); other `.tracking` writes ask;
  a used override leaves an orient-visible record naming the written path. (P1)
- `keel accept` invoked under an AI-bound actor refuses — and invoked via an agent
  session under the human's binding, it is intercepted before the write layer. (P1/K6)
- A project adds a blocking control by declaring a rule — no Rust, no fork — and the rule
  gates turns, commits, and CI. (P1.5)
- A process unit exported from project A imports into project B with its guard
  requirements verified against B's binary, and a later upstream revision lands by
  `import --update` supersession without clobbering B's local constraint additions (and
  rule additions, after P4b); affected downstream work appears in `reprocess-candidates`;
  **the imported process appears as a launchable action in B's console.** (P4a/P4b + P5.1)
- A launched run executes in fresh bounded context and is never *recorded* complete
  without a green post-run gate result; its diff reaches human review unconditionally;
  and one launched run exercises the headless-ask mapping (console proxy / auto-deny +
  recorded obligation). (P5/P1 — the recorded-status wording and the worktree deferral
  are a matched pair: do not re-strengthen one without delivering the other.)
- **Measured:** hook/override/adherence metrics exist for ≥ N sprints (N fixed in D-PM,
  default 3) and at least one advisory→blocking promotion (or non-promotion) decision
  cites them. (PM/K14)

## 8. Resolved forks and what remains open

Resolved by the panel (rounds 1–2) — the human can overrule any at review:
1. **Isolation:** in-place + post-run gate now (concurrency 1, single-writer-per-tree);
   worktrees deferred behind a named DoR.
2. **P5 vs P2 order:** dissolved — P2 is direction-only.
3. **Interactive end-state:** launcher-first for EXECUTE/RECORD is the recommendation,
   recorded in D-P5a as contingent; interactive remains for CHANGE/design; VIEW needs no
   launcher.
4. **Run-ledger residence:** hybrid — local `.keel/runs/` + one tracked summary per
   non-empty-diff run, post-gate, under the run's actor.
5. **Red-yield:** recorded tracked fact + orient visibility; no hard block.
6. **PM residence (was open fork B):** entirely machine-local until a consumer for
   tracked summaries is named (D0144).

Genuinely open for the human:
- **A.** Strict-vs-guided default for *provably empty* directories (plan says strict; any
  directory with existing content requires an explicit `--profile` — the issue089/129
  lockouts were existing-project adoptions, now impossible to hit by inference since the
  profile is declared, never inferred).

## 9. Backlog insertion plan (post-review, mechanical)

For each accepted WP, under the normal discipline:
1. CHANGE route: record the WP's named Decision(s) (§5) in `.engine/decisions/`; human
   acceptance via `keel accept` (human-only, K6). Commits staging process/workflow files
   additionally carry the D0070 keystone marker.
2. Charter Stories per WP scope item (`keel add-task`, `#CharteredBy`; declaration order =
   priority, D0052).
3. RECORD route for atomic facts (the critique Issue, under D-P2).
4. TRIVIAL for the doc-line hygiene items.
Nothing enters `.tracking/` until the human directs it.

## 10. Panel provenance

Nine independent agents critiqued this design across two rounds: PESSIMIST-1 (premise),
PESSIMIST-2 (operations), OPTIMIST-1 (steelman/anti-dilution), OPTIMIST-2 (ROI),
PEDANT-1 (factual accuracy — every citation re-verified at `74ecefb`), PEDANT-2
(internal consistency/governance), PEDANT-3 (mechanism failure modes), REDUCER (scope),
SE (kernel invariants). Round 1: 2 ACCEPT / 7 OBJECT. Round 2 (against r2): 5 ACCEPT /
4 OBJECT, every remaining objection a bounded spec fix. r3 applies all of them; §11 maps
the findings.

## 11. Flip-condition → resolution map

**Round 1 (applied in r2):**

| Panelist · finding | Resolution |
|---|---|
| PESS-1·1 control plane unprotected (issue116) | §2 threat model; P1.4; P1.6 audit-history in CI |
| PESS-1·2 zero measurement (D0128 step 1) | PM; K14; §7 measured criterion |
| PESS-1·3 "close the ring" over-claim | P1 reframed; §2 names layer 3 the guarantee |
| PESS-1·4 downstream lockout risk (issue089/129) | P0.4 adoption profiles |
| PESS-1·5 hollow completion + in-place claim | P5.3 form-only + unconditional review; §7 recorded-status |
| PESS-2·1 distribution | P0.5 + D-P0b (re-scoped in r3, see PED-1·r2) |
| PESS-2·2 blanket block recreates issue081 | P1.1 tiering; evidence-gated promotion |
| PESS-2·3 hooks silently evaporate | P0.3 KEEL_BIN + binary-less test + fire-ledger |
| OPT-1 walls / OPT-2 splits | intact / P4a-P4b, P3a-P3b |
| PED-1·1 skills-location premise | §1.1 shipped-but-undiscoverable |
| PED-2·1/2/3/5/7 governance coherence | per-WP Decisions; binding note; resolved forks; §6 diagram; P2 direction-only |
| PED-3·1/5/6/7/8 mechanism failures | P1.1 tiering; worktree DoR; snapshot gating; hybrid ledger; P4a constraints |
| RED·1/2/3/4/5 scope | P2 direction-only; P4a independent; P3b trigger-gated; ledger local; worktrees deferred |
| SE·F1/F2/F3 kernel gaps | K6+P1.3; K7+P1.4; K2 inventory |

**Round 2 (applied in r3):**

| Panelist · finding | Resolution |
|---|---|
| PED-1·1 [flip] stale "no release channel" claim | §1.1 corrected (channel exists, v0.1.0–v0.2.3 verified); P0.5/D-P0b re-scoped to documented install path + remedy text; line 96 cited as dated |
| PED-2·1 [flip] K2 inventory orphaned | P0.7 scope item + acceptance; timeout behavior stated |
| PED-2·2 [flip] P4b unnamed Decision | rides D-P4a + D-P3a jointly, stated |
| PED-2·3 [flip] deny-list ownership split | D-P0a owns the deny list (P0 ships and tests it); D-P1a owns ask/override/matcher semantics; §7 attribution fixed |
| PED-3·1 [flip] deadlock on hard-deny tier | "deny absent a recorded override" — the unlock reaches every tier (D-P1a) |
| PED-3·2 [flip] headless ask inverts | P1.1/P5.5 headless mapping: console proxy, else auto-deny + recorded obligation; per-process pre-approved scopes; in D-P5b |
| PED-3·3 / SE·F8 / PESS-1·A / PESS-2·R1 [flip] keel-exemption defeats K6 | P1.2 carve-out (accept, confirmation writes, actor set, Person-enrollment, KEEL_ACTOR/--by); actor rebinding added to P1.4; channel check in P1.3 |
| PED-3·4 [flip] concurrent-edit attribution | P5.3 dirty-tree refusal + run-start snapshot + session-keyed write flagging; single-writer-per-tree in D-P5b |
| PED-3·5 no three-way base | P4a install record (unit id, version, file hashes) + bootstrap path |
| PED-3·6 profile inference | P0.4 declared `--profile`, never inferred; recorded fact |
| PED-3·7/8/9/10/11 | single-use path-bound overrides (K7); fire-ledger scoped to accidental loss; summary roll-up policy; summary under run actor; SubagentStop baseline mechanism (P0.6) |
| PESS-1·B `.claude/` redirection | added to P1.2 block targets |
| PESS-2·R2/R3 | headless mapping (above); diff base = run-start snapshot |
| OPT-1·1/2 | downstream CI template in P0.1; P3b trigger includes P5's typed-contract need |
| OPT-2·1/2 | harness-support matrix doc line (P2/hygiene); §7 composed launchable bullet |
| RED·1/2/3/4 | fire-ledger unified with heartbeat (PM); fork B resolved local-only; empty-diff runs local-only; distribution fence in D-P0b/§1 non-goals |
| SE·F9/F10/F11 | K2 carrier (P0.7); K15 added; P4b Decision statement |
| PED-2·4–11 | fork B closed; P3b scope includes producedArtifact typing; §6 P4a→P4b arrow; N bound in D-PM; constraint/rule wording; "with P4b"; git amendment moved to worktree trigger Decision; K1 reworded |

**Round 3 (applied in r4):** PED-1 quotation precision (§1.1); PED-2·1 fire-ledger emit
plumbing owned by P0, analysis by PM; PED-2·2 headless-ask exercise added to §7; PED-2·3
launch-serialization consequence stated (P5.6); PED-2·4 CI-template rules step activates
with P1.5.

## 12. Charter-time notes (panel residuals, none blocking)

Carried into the WP Decisions when chartered — all raised and explicitly not objected on:

1. **Override-recording resilience (PED-3):** obligation facts use one-file-per-fact; if
   the tracked write fails (e.g. the corrupted file IS the obligation target), the unlock
   proceeds on a local ledger entry with a sync obligation. (D-P1a)
2. **Console-proxy timing (PED-3):** write the "ask-pending" ledger entry *before*
   blocking; bound the wait at hook-deadline-minus-margin; expiry ⇒ deny + obligation —
   never let the harness timeout decide. (D-P5b)
3. **Dirty-tree friction valve (PED-3, PESS-2):** console accepts land uncommitted by
   design, so "accept then launch" hits the dirty-tree refusal — provide a console commit
   action (the human clicking is correct attribution) or a guided commit step at launch;
   PM's launcher-fraction metric is the tripwire if refusal friction materializes. (P5)
4. **Derive, don't enumerate, the K6 carve-out (PED-3):** the protected-subcommand set
   should come from a declared attribute on write-API operations (the schema knows which
   record human judgment — `apply-review` is the case the hand list misses); P1.3's
   channel check backstops `accept` only. (D-P1c)
5. **Pre-approved write scopes are control-plane data (SE):** widening one is a governed
   change under K7/P1.4 or the P3a keystone extension, never a quiet config edit. (D-P5b)
6. **P1.3's agent-environment-marker heuristic stays best-effort (REDUCER):** no reactive
   marker arms race; the P1.2 carve-out and K15's tree-derived audit are the real
   controls.
7. **PM baseline review** should examine the dirty-tree-refusal rate alongside the
   launcher-fraction hypothesis (PESS-2's watch-item).
