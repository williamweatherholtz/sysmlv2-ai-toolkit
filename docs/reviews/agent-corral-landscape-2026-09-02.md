# Agent-corral landscape — 2026-09-02

Process: adversarial-panel-review (D0187; def pin 64f2dea), one round, three parallel web-enabled
lenses, converged without a refutation round. Knowledge record: `.tracking/knowledge/agent-corral-landscape-2026-09-02.sysml`.
Trigger: an unauthenticated commenter on the standing-consent override thread (issue #29, 2026-09-02 10:12 UTC,
login `benrrr56-wq`, correctly refused by the recording Action) asked whether keel records consent scope and
rule revision on auto-accepted decisions, and linked agentkey.us. The human asked: what are other projects
in this space cooking, and what can keel learn.

## Answer to the first question

AgentKey is an MCP intermediary — "your agent asks AgentKey before it acts", allow / block / require-approval
per action, hash-chained signed ledger. Its own page says the *in-path proxy* is "on the roadmap": today it is
an MCP server the agent may simply not call. The ledger is real; the enforcement is not yet. The question it
asked is legitimate regardless, and is now issue342.

## The fifteen, in three families

| # | Project | Family | Layer | Enforces | Truth lives | Human authority | Re-derived independently |
|---|---|---|---|---|---|---|---|
| 1 | agentgateway (LF / Solo.io) | in-path | Rust proxy, CEL on MCP calls | permission per tool+identity | cluster config | none | no |
| 2 | ToolHive (Stacklok) | in-path | Go proxy + container, Cedar on tool **arguments** | permission per argument | policy files + OTel | none | no |
| 3 | Snyk Agent Scan (Invariant) | in-path | config-rewriting proxy + scanner | string guardrails, rug-pull detection | logs | per-server launch consent | no |
| 4 | Entra Agent ID (Microsoft) | in-path | identity + entitlement plane | access grants, **time-bound, sponsor re-attests** | tenant directory | named sponsor, approval on extension | no |
| 5 | Arcade.dev | in-path | credential vault + actions runtime | OAuth scope per user+tool | provider grant | mid-loop OAuth consent (standing) | no |
| 6 | Beads (Yegge) | in-repo | Go CLI, Dolt DB, git hooks | graph ops; **gh:run gates** | gitignored Dolt | `bd gate resolve`, unauthenticated; actor defaults to "unknown" | no |
| 7 | GitHub Spec Kit | in-repo | markdown + prerequisite scripts | file existence | markdown, agent-writable | PR review of the constitution | no |
| 8 | OpenSpec (Fission) | in-repo | CLI + markdown, declared schemas | artifact existence ("convention, not enforcement") | markdown | PR review | no |
| 9 | Superpowers (obra) | in-repo | one SessionStart hook | nothing — prompt only | none | chat "go" | no |
| 10 | Kiro (AWS) | in-repo | IDE + PreToolUse hooks + EARS specs | blocking hooks, agent-editable | `.kiro/` markdown | IDE approval gate, unrecorded | no |
| 11 | AgentSpec (ICSE 2026) | formal | DSL + in-process monitor | predicate on pending call | process memory | `user_inspection` callback | no |
| 12 | Progent (Berkeley) | formal | JSON policy + **SMT narrowing/expansion** | predicate on arguments; rule weakening non-silent | process memory | unspecified callback | no |
| 13 | CaMeL (DeepMind) | formal | custom interpreter, capability labels | data-flow (taint) at sinks | interpreter memory | none | no |
| 14 | PAuth (MSR, paper) | formal | server-side operand recomputation | **the result**, by recomputing the operand | signed envelopes | task-scoped grant | at the resource server |
| 15 | halo-record | ledger | PostToolUse hook, hash chain, witness | nothing; detects alteration | JSONL + external witness | none | witness only |

Runners-up with one finding each: CapLease (single-use tokens still replay under agent retry — "semantic
replay"; fix is durable state over the confirmation event), IETF compliance-receipts draft (`policy_digest` and
`expires_at` mandatory), in-toto agent-decision predicate (still an open RFC — no standard exists), Wassette
(Wasm tool sandbox, off-target), OWASP Agentic Top 10 (prose taxonomy), ruflo (advertises a "Truth Verification
System" over a 13-month-open issue: agents "self-report success... when 89% actually fail").

## What converges across the three lenses

1. **Nobody checks substance.** Every in-path product answers *may this call happen*; every in-repo product
   answers *does the artifact exist*; every formal system answers *does the pending call satisfy a predicate*.
   The two exceptions are partial: PAuth recomputes an operand at the server; Beads closes a gate only on a real
   CI run. keel's own weakness (form rigorous, substance weak) is the field's weakness — keel is simply further
   along the form axis than anyone else.
2. **Standing consent is the artifact the field has learned to distrust.** Entra time-bounds and re-attests;
   CapLease proves single-use is not enough without durable confirmation state; the IETF draft mandates a policy
   digest and expiry; even the AgentKey pitch is about single-use binding. keel's D0207 — auto-accept under a
   standing, revision-unbound, non-expiring consent with a forever-thread — is the outlier. Not necessarily
   wrong (the human chose it), but it is the one keel design choice the whole landscape argues against.
3. **keel's three legs are unique together.** Computed state (Beads and OpenSpec have it), blocking guards at
   the harness boundary (Kiro has it, agent-editable), independent re-derivation on infrastructure the agent
   cannot modify (nobody). No project has all three. Beads is closest and its truth is a gitignored database.
4. **The in-repo family is a context family.** 280k + 133k + 52k stars of prompt-only discipline. Superpowers
   ships keel's verify-don't-assert clause word for word and zero hooks that could enforce it — which is also
   the one keel clause with no control behind it (D0151). That parity is the honest measure of the gap.

## What keel takes, ranked (declaration order in the backlog IS this ranking)

| Rank | From | Backlog item | Resolves | Why this rank |
|---|---|---|---|---|
| 1 | mcp-scan rug-pull · PAuth operand binding | `dcAcceptanceBindsToTheText` — acceptance compares the accepted fields at the acceptance SHA with HEAD; drift → suspect | issue341 (High) | Cheap, one guard, closes a consent-forgery-adjacent hole keel already closes for evidence and confirmations |
| 2 | Entra re-attestation · CapLease · IETF receipts | `dcConsentIsScopedAndExpires` — FORK proposal: keep / stamp revision / bind to revision / expire | issue342 (Medium) | The human's decision; the spike supplies the costed options |
| 3 | Beads `gh:run` gates | `dcExternalFactGate` — a TestResult kind CI verifies against a real run id and SHA; the agent cannot assert it | — | First evidence class the agent cannot fabricate |
| 4 | Superpowers test rig | `dcSkillsRouteBehaviourally` — real-model tests that a skill fires; indicator until a threshold is defensible | — | The tractable half of issue336 without touching the frozen schema |
| 5 | CaMeL capability labels | `dcUntrustedTaintClosure` — guard 56 follows `sourceTrust = untrusted` through derivation to privileged sinks | — | Today the label dies after one hop |
| 6 | OpenSpec delta-in-PR | `dcCommitDeltaView` — computed commit-delta view at the top of the channel comment and exec-summary | — | Puts computed state where the human's authority already is |

Fed into an existing fork rather than a new item: **AgentSpec's closed predicate registry** (a `ProcessStep`
carries `check = <name>`, the guard refuses an unregistered name, `advance` runs it — the marker-vocabulary
trick applied to steps) and **Kiro's EARS grammar** (a parseable requirement form; a guard that refuses a
requirement whose text does not parse) are the two concrete mechanisms for `dcProcessStepsBindToChecks` /
issue336, which until today had the problem statement and no mechanism.

## What keel does NOT take, and why

- **Progent's SMT narrowing/expansion over guard predicates.** Correct in principle — `audit-adherence`
  catches deletion and severity downgrade, not a widened regex. But encoding 57 guards' conditions as SMT
  formulae is a rewrite of the guard layer for a defect class not yet observed here. Recorded as knowledge;
  becomes a task if a loosened-predicate defect ever lands.
- **halo-record's capture-completeness accounting.** Narrower than the lens claimed: CI re-derives validate
  and every guard on the tree, so a bypassed hook cannot hide dishonest *state*, only a skipped advisory. The
  hook fire-ledger is machine-local by design (D0180). No action.
- **Wassette, OWASP Agentic Top 10, ruflo, BMAD, Task Master, Vibe Kanban.** Off-target, prose-only, or stale.

## Claims about keel's own state, and the check that established each

| Claim | Check | Verdict |
|---|---|---|
| Acceptance binds to a commit, not the body | `govern.rs:400`; grep `digest\|body_hash` in write path and guards: none | confirmed → issue341 |
| Agent can `keel accept --by <human>` | `main.rs:3969` refuses in agent sessions unless TTY-human/console — best-effort, env-mutable; `write.rs:549` checks the NAME's kind | partly; channel is the strong path |
| No external-fact gate | grep `gh run\|run_id\|workflow_run` in src: only ingest and headless_ask | confirmed → rank 3 |
| Fire-ledger not reconciled in CI | `hooks.jsonl` read by pm.rs/launcher.rs only | confirmed, narrower — no action |
| Auto-accept records no rule revision | `decision-issue.yml:55` step body | confirmed → issue342 |


---

# Lens A: in-path intermediaries / gateways / authorization brokers

## Verification triage first (adversarial)

Before the five: **the trigger itself is thinner than its comment implies.** agentkey.us's own landing page says the in-path proxy does not exist yet — it is an MCP *server* you add with one config line, i.e. **the agent can simply not call it**:

> "an in-path MCP proxy is on the roadmap for actions that must not be skippable"
> — https://agentkey.us/ (© 2026 AgentKey; closed source; free tier 500 decisions/mo → Enterprise custom)

It does have the ledger mechanism it asked keel about:

> "hash-chained, signed, and independently re-verifiable records" … "provable to anyone outside your company without trusting this dashboard" … "Allow, block or require approval per action, with limits and templates"
> — https://agentkey.us/

Also note a **naming collision that is itself a maturity signal**: four distinct live products called AgentKey — agentkey.us (policy/ledger), agentkey.dev (self-hostable credential vault, "you approve once"), agentkeys.io (credential proxy), and chainbase-labs/Agentkey (data marketplace plugin).

**Dropped for lack of verifiable substance in this pass:** Lasso MCP Gateway, MCP Guardian, Zenity, Noma, Descope, Pomerium, Portkey, Composio, Cloudflare AI Gateway — all exist in listicles, none surfaced an inspectable enforcement mechanism quotable at the granularity this spike needs. AgentKey.us is reported above rather than as one of the five, because it has less shipped mechanism than any of them.

---

## 1. agentgateway

- **Name / URL / open-closed / activity**: https://github.com/agentgateway/agentgateway — open source, Apache-2.0, Rust. 4.7k stars, 2,573 commits. v1.5.0 released 2026-08-27. Linux Foundation project since July 2025; Solo.io sells the enterprise distro.
- **Layer**: in-path proxy (Rust, Envoy-lineage, standalone or Kubernetes CRDs).
- **Core mechanism**: CEL expressions evaluated against MCP *method invocations*, not HTTP requests. `jwt.sub == "alice" && mcp.tool.name == "add_issue_comment"`. Policy is a declarative `AgentgatewayPolicy` object — config-as-code, diffable, git-versionable. Per-tool, per-caller-identity. **No approval concept at all** — every decision is synchronous allow/deny.
- **Provenance & attestation**: access logs + observability policy; OpenTelemetry. **No signed or chained ledger.** Protection from the agent is purely network placement.
- **What keel could steal**: evaluate the policy against the *semantic call*, not the transport — guards that match on the typed model operation (`record decision --accept`, edge kind, target item type) rather than on the file bytes an edit produced. Policy as a first-class named object with its own version, so "which rule fired" is answerable by object revision.
- **What it cannot do that keel does**: decides only whether *this caller may invoke this tool*. No notion of whether the work is right, complete, traceable, or whether a human consented to the *goal*. State lives in cluster config, not in the repo.
- **Evidence**: "MCP authorization controls which tools, prompts, and resources a client can reach, by using CEL expressions that evaluate against MCP method invocations rather than against an HTTP request." — https://agentgateway.dev/docs/standalone/latest/mcp/mcp-authz/

## 2. Stacklok ToolHive

- **Name / URL / open-closed / activity**: https://github.com/stacklok/toolhive — open source, Apache-2.0, Go. 2.1k stars, 4,233 commits. v0.46.0 released 2026-08-27 — six releases in August 2026.
- **Layer**: in-path proxy + sandbox — runs each MCP server in a container with declared egress, and proxies calls to it.
- **Core mechanism**: Amazon Cedar, default-deny, forbid-overrides-permit. principal = `Client::<id>`, action = `Action::"call_tool"`, resource = `Tool::"weather"`. **Authorizes on tool ARGUMENTS** (`arg_` prefix): `when { resource.arg_location == "New York" }`. Caches `tools/list` annotations as resource attributes at call time.
- **Provenance & attestation**: OTel/Prometheus audit logging; Sigstore keyless signing for plugins/skills — supply-chain attestation of the *tooling*, not of decisions. No approval workflow, no chained decision ledger, no per-decision policy-version stamp.
- **What keel could steal**: **argument-level policy as data.** A declarative predicate over the *parameters* of a write, not over the resulting file's form. A Cedar-style layer over `record decision --accept --by X --at T` would make "who may accept what kind of decision" an authored fact with a computable diff.
- **What it cannot do that keel does**: no human-in-the-loop anywhere; cannot pause and ask; cannot represent "a named human accepted this specific proposal at this commit." Truth lives outside the repo.
- **Evidence**: "Tool arguments become available in policies with an `arg_` prefix. This lets you create policies based on the specific parameters of requests." — https://docs.stacklok.com/toolhive/concepts/cedar-policies

## 3. Snyk Agent Scan (formerly Invariant Labs mcp-scan)

- **Name / URL / open-closed / activity**: https://github.com/snyk/agent-scan — open source, Apache-2.0, Python. 3.0k stars, 730 commits, v0.6.1. Invariant Labs acquired by Snyk.
- **Layer**: dual — static scanner (CI) and in-path proxy via *config rewriting* (injects a local gateway into MCP configs, removes itself on exit).
- **Core mechanism**: Invariant Guardrails DSL in YAML, scoped client/server/tool/call, inspecting arguments; actions `block`, `log`, `paused`. Detects tool poisoning, **rug pulls** ("unauthorized changes to MCP tool descriptions after initial user approval"), toxic flows. Interactive consent gates each stdio server at launch.
- **Provenance & attestation**: weakest of the five. Consent is per-server, at launch, standing for the session — not per-call, not scoped, not signed. Agent-bypassable by construction.
- **What keel could steal**: **rug-pull detection as a first-class control** — a previously approved thing can change underneath the approval. keel analogue: a hash of the *proposal text* stamped into the Decision at acceptance, so an accepted Decision whose body was later edited computes as **not accepted**. keel does this for evidence via `judgedAgainst` SHA; not for consent.
- **What it cannot do that keel does**: judges *strings* for signs of attack. No model of the work, no traceability edges.
- **Evidence**: "By default, Agent Scan prompts for user consent before starting each stdio MCP server" — https://github.com/snyk/agent-scan ; "MCP Rug Pulls (unauthorized changes to MCP tool descriptions after initial user approval)" — https://invariantlabs.ai/blog/introducing-mcp-scan

## 4. Microsoft Entra Agent ID + Entra ID Governance

- **Name / URL / open-closed / activity**: https://learn.microsoft.com/en-us/entra/id-governance/agent-id-governance-overview — closed, commercial. GA April 2026. Doc updated 2026-06-24.
- **Layer**: identity/credential broker + entitlement control plane — not a call-level proxy.
- **Core mechanism**: agent identity blueprint / principal / identity / agent user. Inherits OAuth2 delegated scopes; more via **access packages** requested by the agent, its **sponsor**, or an admin, routed to approvers. Conditional Access per-access risk. Grants are **time-bound**.
- **Provenance & attestation**: strongest consent-lifecycle model in this set. Named human **sponsor** accountable; sponsorship auto-transfers to the sponsor's manager if they leave; expiry notifies; **extension re-opens a fresh approval cycle**. Record in Entra directory + audit logs: tenant-owned, agent-unwritable, but not hash-chained, not externally verifiable without trusting Microsoft.
- **What keel could steal**: **expiring consent with sponsor re-attestation.** D0207 standing consent is the opposite design — auto-accepts, override thread lives forever. Applied to keel: standing consent should carry an expiry and a rule-revision binding, so when the guard set or auto-accept criteria change, every standing consent granted under the old revision lapses and must be re-granted — single-use override falls out as `expiry = 1 use`.
- **What it cannot do that keel does**: governs *access to resources*, never the content of work. Outside the repo, tenant-locked.
- **Evidence**: "If the sponsor requests an extension, this request can trigger a new approval cycle, where approvers again confirm whether continued access is appropriate. If the sponsor takes no action, the access package assignment automatically expires on its end date" — https://learn.microsoft.com/en-us/entra/id-governance/agent-id-governance-overview

## 5. Arcade.dev

- **Name / URL / open-closed / activity**: https://arcade.dev — closed core (Arcade Engine), open tool SDK. $60M Series A 2026-06-15; $72M total.
- **Layer**: in-path actions runtime + credential vault. The agent never holds tokens.
- **Core mechanism**: authorization binds to (user, tool, OAuth scope). On a call without consent, "Arcade Engine will automatically prompt the user to authorize the tool and coordinate the OAuth 2.0 flow" — a synchronous human consent step mid-agent-loop, then delegated per-user thereafter.
- **Provenance & attestation**: the record of consent is the OAuth grant at the third-party provider — revocable, externally checkable. But **standing, not single-use**: consent to `gmail.send` authorizes every future send; grant records the *scope*, never the *call*.
- **What keel could steal**: **consent as a capability the agent cannot forge, held by a party the agent cannot write to.** keel's nearest gap: the AI runs `keel accept` itself against a table of logins. Suggests an acceptance that is only valid if it carries a token minted by the GitHub decision-channel event, so a forged acceptance is not merely against the rules but unverifiable.
- **What it cannot do that keel does**: authorizes an *effect on an external system*; no view of a repository, a plan, a requirement, or a phase.
- **Evidence**: "When an agent calls a tool, if the user has not granted the required permissions, Arcade Engine will automatically prompt the user to authorize the tool and coordinate the OAuth 2.0 flow with the service provider." — https://docs.arcade.dev/en/get-started/about-arcade

---

## SYNTHESIS (Lens A)

**What the family believes:** an agent is a *caller*, and safety is a property of individual calls. Give it an identity, sit in the path, evaluate each invocation against a declarative predicate (CEL, Cedar, Guardrails DSL), default-deny, and log. Two of the five now authorize on **arguments**, not just tool names — the family's real 2026 advance.

**Where it is blind:** every one answers *may this action happen* and none answers *was this the right work*. Substance-blind in a way that mirrors keel's own weakness, inverted: keel enforces form and can't read process substance; they enforce permission and can't read intent. Second blindness: **their own records are weaker than their rhetoric.** Only AgentKey claims a hash-chained signed ledger, and it hasn't shipped the in-path proxy that would make the ledger complete. None of the five demonstrably stamps *which policy revision* produced a decision.

**The one question keel should answer:** *does an acceptance bind to the rule-set it was granted under, and does it expire?* Entra is the only member with the mechanism; AgentKey is asking because it lacks it. D0207 auto-accept is a standing, revision-unbound, non-expiring consent whose override thread "stays the override thread forever" — precisely the artifact this family has learned to distrust. Bind consent to the guard-set revision and the proposal's hash, and single-use becomes a special case rather than a feature.


---

# Lens B: in-repo / harness-level agent discipline

Verified 2026-09-02 via GitHub REST API (stars/push/release dates are API-exact, not scraped) and raw file reads of the actual enforcement code, not just READMEs.

---

## 1. Beads (`bd`)

- **Name / URL / license / activity**: https://github.com/gastownhall/beads (the `steveyegge/beads` URL 301-redirects here). Open source, MIT, Go. **26,809 stars**, last push **2026-09-01**, releases `v1.2.2` (2026-08-15) and `v1.3.0-rc.1` (2026-08-31), 865 open issues. Docs: https://beads.gascity.com/
- **Layer**: CLI + in-repo config + git hooks + MCP server. Not an orchestrator.
- **Core mechanism**: A dependency graph of issues. `bd ready` computes claimable work — "Show ready work (open issues with no active blockers). Excludes in_progress, blocked, deferred, and hooked issues" (docs/cli-reference/ready.md). Readiness is **computed, never stored**, exactly like keel. **Declared processes**: "formulas" are TOML/JSON workflow templates with `[[steps]]`, `needs = [...]` dependencies and `type = "human"` steps (docs/workflows/formulas.md), and **gates** — "Some workflow steps can't proceed on code alone: a release needs CI to go green, a deploy needs a human sign-off... A gate is an issue that represents that wait" with types `human`, `timer`, `gh:run`, `gh:pr` (docs/workflows/gates.md).
  **Where truth lives is the surprise**: not plain text. An embedded **Dolt** SQL database under `.beads/embeddeddolt/`, which `bd init` **gitignores**; sync over a Dolt ref namespace (`refs/dolt/data`). The JSONL is explicitly demoted.
- **Enforcement strength**: **Mixed, and weaker than it looks.** `bd` refuses invalid graph ops and refuses to open a DB whose schema is ahead of the binary. But script hooks "run after the mutation and fire-and-forget: asynchronous, output discarded, a failure neither blocks nor retries the write" (docs/reference/events-journal.md). Git hooks are thin shims. **Nothing is re-derived in CI.** The agent can `bd hooks uninstall`, `bd init --stealth`, or set `BD_IGNORE_SCHEMA_SKEW=1`.
- **Provenance & human authority**: Best-in-class among these five, and still forgeable. `prepare-commit-msg` "Adds an `Executed-By:` agent identity trailer when an agent (`BD_ACTOR`) makes the commit"; per-issue audit trail (`bd history <id> --events`); append-only `.beads/interactions.jsonl`. **But actor identity never refuses**: `--actor` → `BEADS_ACTOR` → `BD_ACTOR` → `git config user.name` → `$USER` → `"unknown"` (docs/reference/configuration.md#actor-identity-resolution). A `human` gate closes via plain `bd gate resolve <id>` with **no authentication** — the agent can sign off on the human's behalf and the record says the human did it.
- **What keel could steal**: **the `gh:run` / `gh:pr` gate.** A gate that only closes when `bd gate check` finds a real green run is a gate the agent cannot talk its way past. keel's `append-gate-result --evidence` still takes the agent's word for what it ran; a `gh:run`-shaped gate type would make one class of `TestResult` satisfiable **only** by an external fact keel queries. (Second-order: `bd gate check` is a command *the agent runs* — keel should have CI, not the agent, close such a gate.)
- **What keel does that it does not**: plain-text truth reviewable in a PR diff vs a gitignored binary Dolt store. A Beads reviewer cannot see in the PR that the agent `bd gate resolve`d its way through the release gate.

## 2. GitHub Spec Kit

- **Name / URL / license / activity**: https://github.com/github/spec-kit. MIT, Python. **132,997 stars**, last push 2026-09-01, **v1.0.3, 2026-09-01**.
- **Layer**: In-repo files (`.specify/`, `specs/###-feature/`) + agent slash commands + prerequisite scripts. Agent-agnostic across 30+ harnesses.
- **Core mechanism**: constitution → specify → plan → tasks → implement → converge. Two real programs: `check-prerequisites.sh --require-spec/--require-tasks` refuses when the prior phase's artifact is missing; branch/dir mechanics. **Everything else is prose the model grades itself against.** The plan template's gate is a placeholder: "GATE: Must pass before Phase 0 research." / "`[Gates determined based on constitution file]`" (templates/plan-template.md:39–43).
- **Enforcement strength**: **Advisory with a thin file-existence blocking layer.** `/speckit.analyze` is "**STRICTLY READ-ONLY**: Do **not** modify any files." No CI re-derivation. The agent can edit `.specify/memory/constitution.md` — including `**Version**`, `**Ratified**`, `**Last Amended**` — with no co-signature.
- **Provenance & human authority**: The constitution is *asserted*, not evidenced: "Amendments... require a PR with rationale, maintainer approval, and a version bump" / "Unjustified violations block merge" — enforced by **human PR review only**. An agent can fabricate a ratification date and a version bump in one Write call.
- **What keel could steal**: the **`.specify/extensions.yml` mandatory pre-hook contract** — per-command, per-project declared hooks with `optional: false` ("Wait for the result of the hook command before proceeding to the Goal"). A declared, machine-read `before_<step>` obligation list would let a guard check *that the required step actually ran*.
- **What keel does that it does not**: the governing record is non-self-modifiable (keystone lock + `audit-adherence`). Spec-kit's constitution — its entire authority layer — is markdown in the agent's write path.

## 3. OpenSpec

- **Name / URL / license / activity**: https://github.com/Fission-AI/OpenSpec. MIT, TypeScript. **67,016 stars**, last push 2026-09-01, **v1.11.0, 2026-08-26**. Self-hosting under `openspec/`.
- **Layer**: CLI + in-repo markdown truth + slash commands. "OpenSpec doesn't touch git."
- **Core mechanism**: The most keel-shaped state model in the spec-driven family. `openspec status --json` returns per artifact `status: "done"|"skipped"|"ready"|"blocked"` with `requires` and `missingDeps`, "in dependency order, with the schema's `artifacts:` declaration order breaking ties... (never alphabetical), so the first `ready` entry is the artifact to write next" (docs/agent-contract.md §4.4). **Declaration order is priority — keel's D0052, independently reinvented.** Processes are *declared*: `openspec schema init/fork/validate` with `requires` edges; community schema registry. `validate --json` exits 1 on failure.
- **Enforcement strength**: **Advisory, and they say so.** "Everything below is convention, not enforcement." Their docs concede the form-vs-substance gap: "**OpenSpec only checks that artifacts exist, so enforce the gate with your own CI or hook**"; "Neither field is an enforceable check"; `verify` "won't block archive." CI tests the CLI, not the project's own specs.
- **Provenance & human authority**: None, structurally. No actor field, no approval record. Human authority = GitHub PR approval on markdown — real, but outside OpenSpec, not queryable.
- **What keel could steal**: **the review artifact as the PR's first-class content.** "When a PR includes the change's delta spec, the reviewer gets... a plain-language statement of what this change is supposed to do, before they read a single line of code." A `#View` rendered into the PR body — the delta in Needs/Requirements/Decisions this commit makes — would put keel's computed state where the human's authority already lives. Also: `docs/agent-contract.md`, a code-audited spec of every `--json` shape, dated to its audit.
- **What keel does that it does not**: substance checks and receipts (edges resolve, `judgedAgainst` resolves, `// RAN:` receipt refused if absent). OpenSpec has no concept of an unsupported claim.

## 4. Superpowers

- **Name / URL / license / activity**: https://github.com/obra/superpowers. MIT, Shell. **280,663 stars** — largest by an order of magnitude — last push 2026-08-31, **v6.3.0, 2026-08-12**. In Anthropic's and OpenAI's plugin marketplaces.
- **Layer**: Harness plugin delivering ~40 skills. Nothing in-repo per project; methodology, not state.
- **Core mechanism**: **Prompt injection, and nothing else.** `hooks/hooks.json` registers exactly one hook — `SessionStart` — which cats one skill file as `additionalContext` wrapped in `<EXTREMELY_IMPORTANT>`. **No `PreToolUse`, no `Stop`, no `PostToolUse`.** No work state at all.
- **Enforcement strength**: **Purely advisory.** Its strongest control is rhetorical and well-written: "NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE" / "If you haven't run the verification command in this message, you cannot claim it passes" / "Skip any step = lying, not verifying" (skills/verification-before-completion/SKILL.md). That is keel's verify-don't-assert clause with no `Stop` hook behind it — which is precisely the one keel clause that also has no control behind it (D0151).
- **Provenance & human authority**: Nothing recorded. Human approval is a chat "go" — ephemeral, unattributed, trivially claimable in a summary. Review routed through subagents — the model reviewing the model.
- **What keel could steal**: **its skill test harness.** `tests/claude-code/run-skill-tests.sh`, `run-multiturn-test.sh`, `run-haiku-test.sh` — real models against real prompts, asserting the intended skill actually fires. keel has 57 guards over the *model* and zero tests over whether its `ProcessStep` prose and skill descriptions actually route the agent. The tractable near-term answer to "process steps are prose" is not to formalize the prose but to **test it behaviourally**.
- **What keel does that it does not**: everything mechanical. Superpowers, with 280k stars, cannot prevent a single false completion claim; it can only ask very firmly.

## 5. AWS Kiro

- **Name / URL / closed / activity**: https://kiro.dev/. **Closed source.** GA 2026-05-07 (superseding Amazon Q Developer). Included because it is the only entrant where the *harness vendor* owns the blocking layer.
- **Layer**: Agentic IDE — in-repo files (`.kiro/specs/`, `.kiro/steering/`, `.kiro/hooks/`) **plus first-party harness hooks**.
- **Core mechanism**: Specs = `requirements.md`, `design.md`, `tasks.md` with phase **approval gates**; "Quick Spec" is the variant that "auto-generate[s] all three artifacts without approval gates." Requirements use **EARS** (Easy Approach to Requirements Syntax). Steering files = standing-instruction layer with `always`/`conditional`/`manual`/`auto` inclusion. Hooks: "run shell commands or agent prompts automatically when specific events happen... the agent modifies a file, invokes a tool, or completes a task."
- **Enforcement strength**: **The only entrant besides keel with real blocking at the harness boundary** — `PreToolUse`, `UserPromptSubmit`, `PreTaskExec` can refuse. But **wide open to the agent**: hooks and steering are repo files in the agent's write path, no signature, no lock, no re-derivation. An agent can delete its own `PreToolUse` hook or switch to Quick Spec to drop the gates.
- **Provenance & human authority**: Phase approval is an IDE-session interaction; no durable, attributed approval record in the spec files. Audit *trail* without attributed *authority*.
- **What keel could steal**: **EARS as the grammar for the fields keel currently leaves as free prose.** `WHEN <trigger> the <system> SHALL <response>` — a small, parseable requirement grammar with aerospace pedigree. A guard that refuses a `SystemRequirement` whose text does not parse as an EARS pattern converts an unreadable String into a checkable form — no new schema type, only a parser and one guard.
- **What keel does that it does not**: non-self-modifiable, independently recomputed enforcement surface.

---

**Checked and dropped (auditable exclusions):**

- **ruflo** (formerly claude-flow) — 70,197 stars, pushed 2026-09-02. An orchestrator, not in-repo discipline, but the family's most instructive corpse: its wiki markets a "Truth Verification System" with a 0.95 threshold, while its **still-open** issue #640 (filed 2025-08-11) states "Agents self-report 'success' without mandatory verification... Agent claims '✅ All tests working' when 89% actually fail" and "'Principle 0: Truth Above All' exists only as aspiration."
- **BMAD-METHOD** — 52,586 stars, v6.11.0. Pure prompt-and-persona methodology; strictly less enforcement than Superpowers.
- **Task Master AI** — 28,039 stars but last push 2026-04-28; stale.
- **Vibe Kanban** — 27,988 stars, last push 2026-04-24; stale, UI over agent runs.
- **claude-code-hooks-mastery** — 3,906 stars, last push 2026-03-04; teaching repo.
- **agentjail** (OPA policy on every tool call) — genuinely blocking, but 85 stars, security-scoped.

---

## SYNTHESIS (Lens B)

This family believes agent discipline is a **context problem**: give the model a written spec, a constitution, a steering file, an `<EXTREMELY_IMPORTANT>` preamble, and it will comply. Aggregate stars for pure-prompt approaches (Superpowers 280k, spec-kit 133k, BMAD 52k) exceed every mechanism-bearing project combined. The blindness is total and self-documented. spec-kit's constitution gate is the literal string `[Gates determined based on constitution file]`, graded by the model that wrote the constitution, checked by a command that is "STRICTLY READ-ONLY." OpenSpec states it outright — "convention, not enforcement." Superpowers writes keel's verify-don't-assert clause verbatim and ships zero hooks that can enforce it. ruflo advertises truth enforcement over a 13-month-open issue admitting it is "aspiration."

**Beads comes closest**: computed ready frontier, declared formulas, gates including `gh:run` gates closed only by real green CI, an append-only audit log, and an `Executed-By:` actor trailer. It does one thing better — gate satisfaction bound to an external fact keel cannot assert its way past. But its truth is a gitignored Dolt database, its actor defaults to `"unknown"` rather than refusing, `bd gate resolve` on a `human` gate needs no authentication, and nothing is re-derived in CI. **No project in this landscape has keel's third leg** (independent re-derivation on infrastructure the agent cannot modify).


---

# Lens C: runtime formal enforcement, capability sandboxes, tamper-evident audit

Repo metrics from the GitHub API on 2026-09-02, not from prose.

---

## 1. AgentSpec — runtime enforcement DSL for LLM agents

- **Name / URL / open-closed / activity**: paper https://arxiv.org/abs/2503.18666 · code https://github.com/haoyuwang99/AgentSpec · open code, **no LICENSE file** · 51★, last push 2026-01-15 · **peer-reviewed: ICSE 2026**. The only one of the five with a real venue.
- **Layer**: policy DSL + in-process runtime monitor (LangChain interceptor).
- **Core mechanism**: ANTLR4 grammar defines rules `rule @name / trigger <ToolName> / check <predicate> / enforce <action> / end`. The **predicate is an arbitrary Python function** over `(user_input, tool_input, intermediate_steps)`; enforcement ∈ `stop`, `user_inspection`, `invoke_action`, `llm_self_examine`. The checkable unit is a machine-checkable predicate over the *pending* action, not prose. Cannot prove anything about outcomes.
- **Tamper resistance**: **none**. Interpreter, grammar, predicate table live in the same Python process as the agent. Threat model is a *mistaken* agent, not an adversarial one.
- **Substance vs form**: pure form (`is_destructive` regexes `os\.remove`).
- **What keel could steal**: the **two-part predicate binding** — a predicate *name* that is a terminal in the rule grammar, plus a registry mapping name → executable function. Applied: a `ProcessStep` carries `check = <predicateName>`, `keel guard` refuses to load a workflow naming a predicate absent from the registry, and `advance` runs it. Converts "process steps are prose the guards can't read" into a *closed vocabulary* problem — the trick keel already uses for marker vocabulary. Make the registry data-driven, not grammar-driven.
- **What keel does that it does not**: re-derivation on infrastructure the producer cannot touch; monotonicity audit.
- **Evidence**: "rule @check_shell_exec / trigger PythonREPL / check is_destructive / enforce user_inspection / end" — README. Abstract: "a lightweight domain-specific language for specifying and enforcing runtime constraints on LLM agents... structured rules that incorporate triggers, predicates, and enforcement mechanisms" — https://arxiv.org/abs/2503.18666

## 2. Progent — privilege control with SMT-decided monotonic confinement

- **Name / URL / open-closed / activity**: https://arxiv.org/abs/2504.11703 · code https://github.com/sunblaze-ucb/progent · open code, **no LICENSE** · 49★, last push 2026-05-14 · v3 2026-05-14 (Dawn Song's group, UC Berkeley; no venue stated).
- **Layer**: policy DSL (JSON Schema) + deterministic runtime monitor + SMT-checked policy-update controller.
- **Core mechanism**: rule lists — `effect` (allow/forbid), target tool, boolean conditions **over tool arguments**, fallback. The *update* path is the contribution: an LLM proposes policy changes mid-run, and an SMT solver classifies each as a **narrowing** (auto-applied) or an **expansion** (requires explicit approval). "The effective action space can only shrink without approval" as a decided property.
- **Tamper resistance**: architecturally separated but co-resident. Real contribution: **privilege escalation is non-silent**.
- **Substance vs form**: form at argument granularity; but narrowing/expansion is a *meta*-substance check — whether a change to the rules themselves is a weakening.
- **What keel could steal**: replace keel's guard-set/severity **monotonicity heuristic with a decision procedure**. `audit-adherence` catches deletion and downgrade, not a guard whose *predicate* was quietly loosened (regex widened, threshold raised). Encode each guard's condition as an SMT formula; CI asks whether `new ⊨ old` (narrowing, allowed) or not (expansion, requires co-committed `#SafetyChange`). Rust has usable z3 bindings.
- **What keel does that it does not**: durable, human-attributable approval — Progent's "explicit approval" is an unspecified runtime callback with no identity or record.
- **Evidence**: "Each proposed update is determined by an SMT solver to be either a narrowing (applied automatically) or an expansion (requiring explicit approval), ensuring that the agent's effective action space can only shrink without approval (monotonic confinement)." — https://arxiv.org/abs/2504.11703

## 3. CaMeL — capability + data-flow interpreter (Google DeepMind / ETH Zurich)

- **Name / URL / open-closed / activity**: https://arxiv.org/abs/2503.18813 · https://github.com/google-research/camel-prompt-injection · Apache-2.0 · **380★ but last push 2025-06-20** (dormant ~14 months) · no peer-reviewed venue found.
- **Layer**: runtime monitor as a **custom interpreter** — strongest architectural separation in this set.
- **Core mechanism**: privileged LLM translates the trusted query into pseudo-Python expressing *control flow only*; quarantined LLM (no tools) parses untrusted data; custom interpreter executes, propagates **capability labels** as data provenance, evaluates policies at each tool call. Untrusted data is never interpreted as code. Policies are deployer-authored Python predicates — conformance to a policy, never that the policy is right.
- **Tamper resistance**: best in principle — the LLM emits a *plan*, a separate interpreter decides. Repo warns it is a research artifact.
- **Substance vs form**: form, but over the *data-flow graph* (this value's provenance is untrusted, therefore it may not reach this sink).
- **What keel could steal**: the **privileged/quarantined split, applied to authoring**. keel's analogue of prompt injection is a `Statement` ingested from a public GitHub issue (D0264 `untrusted`). CaMeL's generalisation: attach a *capability label to every value derived from an untrusted utterance* and refuse when an untrusted-labelled value reaches a privileged sink (an accepted Decision, a `#SafetyChange`, a guard predicate). Today keel enforces the *routing* (guard 56); CaMeL says enforce the *taint closure* — laundering through three intermediate items stops working.
- **What keel does that it does not**: persistence and audit — CaMeL's labels live in interpreter memory for one session.
- **Evidence**: "the untrusted data retrieved by the LLM can never impact the program flow." — https://arxiv.org/abs/2503.18813 · counterweight: "ten months later, convincing real-world implementations remain limited" — https://neuraltrust.ai/blog/camel-prompt-injection (2026-02-12).

## 4. PAuth — operand-scoped ("task-scoped") authorization via NL slices

- **Name / URL / open-closed / activity**: https://arxiv.org/abs/2603.17170 · **no code released** · v2 2026-08-25, CC-BY-4.0 · Microsoft Research + Ohio State. **Weakest activity signal — included for the mechanism.**
- **Layer**: standard/protocol proposal, enforced server-side.
- **Core mechanism**: OAuth grants over *operators* (`TRANSFER`); PAuth grants over *operations* (operator + operands). A **NL slice** is the symbolic sub-computation determining one server call. Values crossing servers travel in **signed envelopes** pairing the concrete value with its symbolic expression. The receiving server independently derives the slice and accepts only if the concrete argument equals the value recomputed from the symbolic provenance.
- **Tamper resistance**: strong — enforcement at the resource server the agent does not run; provenance signed by the producer of each value.
- **Substance vs form**: **the closest thing to substance in this landscape.** It recomputes the argument and asks whether the value the agent produced *is the value the task implies*. Checking a result, not a call.
- **What keel could steal**: **make `keel accept` an operand-bound approval.** Acceptance records a digest over the exact Decision body + the exact set of typed edges it authorizes; CI recomputes that digest from the tree — an accepted Decision whose text or scope drifts after sign-off becomes an automatic refusal rather than a standing approval. Same for `#SafetyChange`: the Decision names the guard-predicate digest it authorizes changing.
- **What keel does that it does not**: exist as running software with a maintainer.
- **Evidence**: "OAuth scopes govern operators, not operands, and this gap becomes a chasm in agentic workflows." · "The call is accepted only if this computed value equals the concrete amount and the symbolic provenance matches the task-derived Chase slice." — https://arxiv.org/html/2603.17170v2

## 5. halo-record — hash-chained tamper-evident runtime records, with an honest threat model

- **Name / URL / open-closed / activity**: https://github.com/bkuan001/halo-record · PyPI 0.2.38 · Apache-2.0, Python, zero deps · 74★, **last push 2026-09-02**, 98 commits · single maintainer; press 2026-08-31 (helpnetsecurity). Included because it is the only *shipping* artifact in this lens whose documentation correctly states what a ledger cannot prove.
- **Layer**: attestation/ledger. Ships a **`halo hook` Claude Code PostToolUse hook** plus OTel/LangChain/MCP adapters.
- **Core mechanism**: append-only JSONL, each record hashing its predecessor; `halo verify` recomputes; `halo anchor` emits/checks **witness checkpoints** (count + head hash held outside the operator), optionally RFC 3161-timestamped; `halo policy` corroborates against a declarative policy pack. Refuses nothing at runtime.
- **Tamper resistance**: a self-held chain is *not* tamper-evident against its own operator until the head leaves the operator's control. Separation comes from the witness, not the hash.
- **Substance vs form**: neither — it records. Names **capture completeness** as a property of *where the recorder sits*, and tags every record with a `source`.
- **What keel could steal**: **capture-completeness `source` tag plus witness accounting.** keel's guards prove the recorded model is well-formed; nothing proves every action *reached* the record. (a) tag every authored fact with the capture path (`write-api` / `direct-edit` / `ingest` / `unknown`); (b) have the hook fire-ledger (`.keel/metrics/hooks.jsonl`) emit a count + head hash checkpoint into CI each push, so CI can assert the number of post-edit fires is consistent with the number of `.sysml` diffs in the range. Today a `.sysml` edit made with hooks bypassed leaves no trace — the "delete the embarrassing Tuesday" attack in keel's own shape.
- **What keel does that it does not**: keel already *has* the witness — git is a Merkle DAG and the GitHub remote is a party outside the producer. keel computes state from the chain rather than merely sealing it.
- **Evidence**: "Neither the chain nor the witness proves that every real-world action passed through the recorder. That is capture completeness — a property of where the recorder sits in the stack... not of any hash." — README · "Delete the embarrassing Tuesday, re-seal the chain, and the file stays internally consistent." — https://www.helpnetsecurity.com/2026/08/31/halo-record-open-source-ai-agent-audit-trail/

---

## Verified runners-up

- **CapLease** — "Beyond Single-Use Tokens: Durable Authorization State for Replay-Resistant LLM Agent Actions", https://arxiv.org/abs/2608.01710 (2026-08-03, no code). **The single most relevant adversarial finding for keel**: "These behaviors can cause one user authorization to be requested and executed multiple times under freshly issued token identifiers, even when each individual token is single-use. We call this failure semantic replay"; the fix is "transactional Issue-Prepare-Commit transitions" over "monotonic durable state over the authorized action, confirmation event, and remaining execution budget."
- **Microsoft Wassette** — https://github.com/microsoft/wassette, MIT, Rust, 941★, last push 2026-09-01; Wasmtime-sandboxed WebAssembly components over MCP, deny-by-default. "not production ready yet." Off-target: keel's threat is a dishonest *record*, not tool escape.
- **in-toto `agent-decision/v0.1` predicate RFC** — https://github.com/in-toto/attestation/issues/554, open since 2026-05-19, no PR: "runtime agent decisions have no registered predicate."
- **IETF draft-marques-asqav-compliance-receipts-08** — 2026-08-31, individual submission. Signed action receipts with `policy_digest`, `action_ref`, nonce, `expires_at`, mandatory hash chaining: "Verifiers MUST reject a downstream action that replays a decision whose expires_at lies in the past."
- **OWASP Top 10 for Agentic Applications 2026** — ASI01 (Agent Goal Hijack) → ASI10 (Rogue Agents). Prose taxonomy, useful only as a coverage checklist against `keel controls`.
- **Rejected**: *Proof-Carrying Agent Actions* (arXiv 2606.04104) — single author, no code, "protected benchmark" under a "disclosure-bounded evaluation protocol"; unreproducible.

---

## SYNTHESIS (Lens C)

**(a) Binding an approval to one action.** State of the art is *not* a token. PAuth binds authorization to an **operand-level symbolic expression** the resource server recomputes, with signed provenance envelopes; CapLease shows that even a single-use token fails under agent replanning/retry/crash-recovery — "semantic replay" — and that the only fix is **monotonic durable state over (canonical action, confirmation event, remaining budget)**. **This is a live defect in keel:** D0207 standing consent auto-accepts non-fork Decisions, which is structurally an authorization with unbounded execution budget and no canonical action binding. Fix cheaply: make acceptance record a digest over the exact Decision body + authorized edge set, and have CI recompute it (PAuth), plus a consumption record so one acceptance admits one apply (CapLease).

**(b) Prose → predicates.** Nobody has solved it. AgentSpec (closed predicate vocabulary in a grammar + registry) and Progent (SMT-decided narrowing vs expansion) are the two portable mechanisms; both check *form*. PAuth alone checks a *result*, by recomputation.

**Portability to keel:** AgentSpec's predicate registry (high), Progent's SMT monotonicity for guard predicates (high, z3-rs), PAuth's digest-bound acceptance (high), halo-record's capture-completeness accounting (high — keel's git+CI is already the witness), CaMeL taint-closure labels (medium), Wassette/OWASP (low).

Caveat: quotes were retrieved via page fetch and summarization; repo metrics are direct API reads. Re-check any quote before citing verbatim.
