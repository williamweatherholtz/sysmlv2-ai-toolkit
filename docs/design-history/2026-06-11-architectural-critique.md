# Antagonistic Architectural Critique — 2026-06-11

Executed per `EngineCritiqueProcess` (`.engine/processes/architectural-critique.sysml`),
steps 1–7. Three parallel adversarial audits (SysML-v2 misuse/unused-features; top-10
use-case walkthroughs; anti-patterns/process-layer), synthesized into the CR proposals
in §4. **Nothing here is applied** — step 8 (human disposition) gates every change.

Finding IDs: M=misuse, U=unused-feature, S=self-consistency, UC=use-case, A=anti-pattern,
P=process-layer.

---

## 1. The headline pattern

**The engine is strongest where it is newest (native-metaclass schema, the query layer's
graph reading) and weakest at its own center of gravity.** Every defining mechanism —
identity, decision capture, suspicion, computed state, the edge algebra, gates, the
skills registry — currently exists as a *contract that the shipped artifacts contradict*.
Most violations are self-admitted in comments ("slated to become a view," "intermediate,"
"parked"), which is the most dangerous form: documented violations read as sanctioned.

Use-case scoreboard: **1 of 10 use cases partially works today** (the backlog half of
"what's next?"). Everything that works runs on the `_meta.sysml` action-def +
one-line-DoD dialect; everything written in the real `schema/core` is currently
unreachable (no instance of any schema/core type exists, and none can pass mandatory
validation — see UC-gap-1).

## 2. Key findings (deduped, with evidence)

### SysML v2 misuse
- **M1 (HIGH) Two parallel type universes.** `_meta.sysml` declares 15 vacuous shadow
  `item def`s (Brief/Need/UseCase/Decision/TestResult/...) duplicating real
  `schema/core` types. All workflows + the entire backlog are typed against the
  *shadows*; the frozen deliverable schema has **zero instances**. Textbook
  two-sources-of-truth (§2.1) inside the engine model itself (= A6, UC-gap-2).
- **M2 (HIGH) The backlog inverts the engine's own verification model.**
  `AcceptanceCriterion` folds a *mutable* verdict (`verifiedAtCommit[0..1]`) into a
  requirement def — `verification.sysml:4-9` explicitly forbids exactly this
  ("RESULT is a SEPARATE record... appended never overwritten"). Re-verification
  overwrites history. `satisfy DoD by task` is a fake `verify` (= A2). The
  backlog's "OUTSTANDING" comment header now sits above entries that are all done
  (S7) — an authored status view, lying.
- **M3/M4 (MED) Stringly-typed structure where pilot-confirmed natives exist.**
  `WorkflowDefinition.transitions : String[*]` ("from->to" in strings),
  `Sprint.items : String[*]`, `Agent.subagents : String[*]`,
  `authoredBy/owner : String` — `ref x : Type[*]` is pilot-confirmed and unused.
- **M5 (MED) The STPA control structure has no structure** — seven disconnected
  part defs; UCAs can't reference the ControlAction they make unsafe, though
  `connect`/`interface def` are proven in this repo.
- **S2 (HIGH) The actor registry's data lives in comments.** `ActorId` enum literals
  carry name/email/kind as `//` comments; the schema's `Actor`/`Person` types have
  zero instances.
- **S3 (HIGH) Two verification-method vocabularies.** `VerificationMethod` enum says
  `demo`; six backlog instances say `"demonstration"`. Any normalizing tool silently
  misses them.
- **S4 (MED) strongTyping's DoD statement is false.** Recorded green as "adopted
  across schema/core," but work/risk/process/skills/stpa retain ~10 String
  comment-vocab fields (incl. a second priority system `p0..p3` vs MoSCoW).
  A verification-integrity failure, not just typing debt.
- **S5 (MED) The edge algebra beyond `satisfy`/`:>` is vaporware.** All five custom
  markers (#Allocate/#DependsOn/#Supersede/#OrderingOnly/#View) have **zero
  applications**; scope-by-supersession (§2.4) has never been exercised.
- **S6 (MED) Zero modeled gates.** "Gate = verify-linked Tests" has no instance for
  any of the six workflows' phases; change-request.sysml claims one exists.

### Unused native features (pilot-checked)
- **U1 (HIGH) `doc` clauses** — pilot-confirmed, used zero times; load-bearing vocab
  and rationale live in `//` comments the model can't see.
- **U2 (HIGH) Attributed `metadata def`** could collapse the 4× duplicated Tracked*
  bases into one cross-metaclass annotation (needs spike).
- **U3 (MED) Native `dependency` + #OrderingOnly never authored** — suspicion
  semantics are currently encoded *by omitting edges* (see the deleted
  schemaAudit→testModel ordering edge: authored truth destroyed to silence a false
  SUSPECT).
- **U4 (MED) `require`/`assume` constraints** — bought (spiked, documented, DoD'd
  green), adopted nowhere.
- Deferred/low: `alias` (M1 helper), std-lib Time/ISQ, `variation`, `occurrence def`
  retyping of TestResult/Decision (M6), `state def` spike (M3).

### Use-case walkthroughs (top 10, end to end)
| UC | Verdict | Point of failure |
|---|---|---|
| 1 Onboard project / author Brief | **BROKEN** | Two Brief types, no layout convention, instance can't pass validate_tracking (preloads only `_meta`), `.tracking` "gitignored" claim false |
| 2 Business needs→gate | **BROKEN** | No phase has any verify-linked Test; no authoring form vs an action; query.py excludes workflows |
| 3 Requirements + trace from Need | authoring FRICTION; **query BROKEN** | No tool parses satisfy edges (backlog's own `satisfy` lines are decorative — binding is by `<task>DoD` naming) |
| 4 Story through Delivery | **BROKEN** | Story has zero instances + can't validate; states read by nothing; DoR/DoD prose-only |
| 5 Decision supersedes Need | **BROKEN** | #Supersede parses but zero examples; usage.md teaches invalid old syntax; no tool reads it; superseded Need shows nowhere |
| 6 "What next?" unified | **FRICTION** | Split-brain (query.py vs whats_next.py); the §3b "state cursor" does not exist anywhere; JVM-per-query too heavy for ORIENT |
| 7 Upstream change→suspicion | **BROKEN** | query.py inverts D0005: succession-only (the excluded edge class), one hop, no material-change detection; broken-Assumption scenario fires nothing |
| 8 Change Request e2e | **FRICTION** | Zero CR instances ever (engine's own ~15 CR: commits bypassed the CR workflow); acceptance has no recorded artifact |
| 9 Release/baseline | **BROKEN** | membershipRule unevaluable free text; baseline computed by nothing |
| 10 Field Issue→re-verify | **BROKEN** | No path from Operate to instance authoring; Issue can't validate; trace/suspicion gaps compound |

Cross-cutting: (1) **instance write path broken** (validate_tracking preloads only
`_meta`) — hits UC1/2/3/4/5/8/9/10; (2) two type universes; (3) no state cursor;
(4) **edge algebra is write-only** (no production tool parses satisfy/verify/:>/
markers); (5) suspicion inverts D0005; (6) gates mechanically undefined; (7) identity
dead-on-arrival (zero `:>> id` anywhere; tools key by bare name and silently merge
collisions); (8) hidden textual contracts (one-line DoD regex, `<task>DoD` naming,
non-recursive globs — undocumented); (9) documentation drift (usage.md invalid
syntax, CLAUDE.md stale banner + gitignore claim, RESUME as shadow tracker);
(10) read path too heavy/leaky (JVM per query; **75 orphaned kernel JVMs found and
killed during this critique** — interrupted runs never reach teardown, so the
validatorFix "zero orphaned JVMs" DoD covered only the clean path).

### Anti-patterns / process layer
- **A1+A4 (HIGH) Authored state the invariants forbid:** `Element.currentState`
  (+`updatedAt`) authored on every Tracked base; the agile process *instructs* "set
  currentState = ready/in_review/done" — process layer and invariant layer give
  opposite instructions.
- **A3 (HIGH) Identity fictional in practice** (zero ids; name-keyed tooling;
  silent merge on collision).
- **A5 (HIGH) Decision capture lapsed when it mattered:** decisions stop at 0010;
  ≥11 CR: commits since (incl. native-metaclass supersession, temporalModel,
  main-only policy, verification option A) have **no Decision record** — they live
  in commit messages/comments/RESUME/memory.
- **A7 (HIGH) Truth-docs contradict git reality:** `.tracking/` is tracked, docs say
  gitignored; CLAUDE.md banner still says "schema does not parse yet"; RESUME
  duplicates the backlog (shadow tracker).
- **A8 (HIGH) Suspicion implementation contradicts every rule of the D0005 contract
  it cites** (see UC7) — and the predicted failure already happened (edge deleted to
  silence a false flag).
- **A9 (MED) "Done" = presence of an unvalidated string.** Nothing checks
  `verifiedAtCommit` resolves, is an ancestor of HEAD, or contains the change; a
  typo'd SHA silently disables suspicion.
- **P1 (HIGH) Skills registry registers four phantoms** (triage/staleness-sweep/
  implementer/grill-me — none exist) **and omits all five real skills**; engine-triage's
  own SKILL.md said "register during instanceMigration" — that DoD passed with the
  obligation skipped (A9 made flesh). writePolicy is "enforced by the API" — no API.
- **P2 (MED) The agile process governs a world that doesn't exist** — Sprints/points/
  Stories with zero instances; DoD requires PRs/CI that main-only policy now forbids.
- **P3 (MED) DoR/DoD claim engine evaluation** ("computed satisfaction == verified")
  that no machinery performs — borrowed authority.
- **P4 (MED) CR acceptance has no recorded artifact** (who accepted what, when —
  nothing distinguishes an accepted CR from an AI-recorded one).
- **P5 (MED) BOOTSTRAP is a hole in the triage fence:** changing query.py's
  done/suspect semantics is routed as BOOTSTRAP (no Decision, no acceptance) though
  it changes process behavior as surely as editing a workflow.
- **P6 (MED) Validation unenforced + auto-push amplifies:** no validate-all, no
  pre-commit check, post-commit pushes unvalidated commits to canonical main in
  seconds.
- **P7 (LOW) The dogfood trigger lapsed:** §4's first-real-dogfood conditions have
  been true since 2026-06-10; neither started nor deferred-by-Decision.

## 3. What is genuinely sound (keep)

Native-metaclass schema + strong typing (post-audit); the succession-graph reading +
Kahn waves; git-ancestry as the engine clock (temporalModel); the four-layer
validators' clean-path teardown; the request-triage discipline + confirmation
sign-off rule; the CR: commit convention (followed in practice); process-as-data
(this critique itself was authored as a process and executed).

## 4. Change-request proposals (step 7 output — awaiting human disposition)

Ranked by leverage. Effort: S/M/L.

| CR | Proposal | Resolves | Effort |
|---|---|---|---|
| **CR-1** | **Fix the instance write path**: validate_tracking preloads schema/core (mirror validate_instances); recursive `.tracking/**` globs; a `.tracking/README` + template showing the sanctioned instance idiom & layout | UC-gap-1 (unblocks UC1/2/4 + every instance authoring path) | **S** |
| **CR-2** | **One type universe**: declare `schema/core` canonical; retire `_meta`'s shadow item defs (workflows import schema types; spike action-pin typing / `alias` fallback); migrate backlog.sysml | M1, A6, UC-gap-2 | M |
| **CR-3** | **One verification model**: DoDs become `verification def`-based criteria; results become **appended `TestResult` records** (immutable, git-SHA'd) instead of mutable verdict fields; `method : VerificationMethod` (fix `"demonstration"`→`demo`); retire/absorb `AcceptanceCriterion` | M2, A2, S3 | M-L |
| **CR-4** | **Make suspicion honest (D0005)**: material-change trigger (file-granularity `git log` per element file at minimum), suspicion-carrying edges (satisfy + `:>`), honor `#OrderingOnly`, transitive closure; restore the deleted schemaAudit→testModel edge tagged ordering-only; add evidence validation (A9: `git cat-file` every verifiedAtCommit; `INVALID-EVIDENCE` bucket) | A8, UC7, U3, A9 | M |
| **CR-5** | **Make identity real**: populate `:>> id` (UUIDs) on all ~30 existing instances; tooling keys by package-qualified name; validator warns on missing id; actor registry becomes `Person` part instances (capture_user emits them), enum derived | S1, A3, S2, UC-gap-7 | M |
| **CR-6** | **State cursor + unified orient**: define `.tracking/state.sysml` (active workflow + phase, one part); `query.py orient` merges cursor + ready set + suspect; whats_next.py folds in or retires | UC-gap-3, UC6, P7 | S-M |
| **CR-7** | **Kill authored state**: delete `currentState`/`updatedAt` from Element + 3 Tracked bases (before instances populate them); rewrite the agile process's three "set currentState" steps to "append the fact; state is computed" | A1, A4 | S |
| **CR-8** | **Resume decision capture + CR acceptance artifacts**: backfill Decisions 0011–0016 (native-metaclass supersession; verification option A; temporalModel git-only; main-only branch policy; AcceptanceCriterion interim model; §2.6 materialized views); henceforth every CR: commit pairs with a recorded acceptance (who/when/commit — the DoD confirmation pattern applied to CRs) | A5, P4, §4 rule 1 | S-M |
| **CR-9** | **Truth reconciliation (docs)**: decide `.tracking` tracked-vs-gitignored (recommend: tracked for the self-build, exemption recorded); fix CLAUDE.md stale banner; rewrite usage.md (teaches invalid syntax); gut RESUME.md to a pointer at the query tool; delete lying comments (backlog OUTSTANDING header, verification.sysml:12, syntax-notes closed-set guidance) | A7, S7, UC-gap-9 | S |
| **CR-10** | **Registry + process-layer honesty**: register the 5 real skills, drop the 4 phantoms (or mark planned); rebase agile process on the actual work-item dialect and strike PR/CI language (main-only); label every DoR/DoD step computed-now / computed-later / human-judgment | P1, P2, P3 | S-M |
| **CR-11** | **Validation enforcement + JVM hygiene**: `validate_all.py` (one kernel, all four layers); pre-commit hook running the layer matching staged paths (post-commit push stays); kernel teardown hardening for interrupted runs (atexit + a `kill-stale-kernels` sweep utility) | P6, the 75-JVM finding | M |
| **CR-12** | **Cheap native wins batch**: `doc` clauses for semantics-bearing comments (U1); typed `ref`s replace name-strings (M4); finish strongTyping enums honestly + supersede its false DoD statement (S4); spike attributed `metadata def Tracked` (U2); STPA control-loop refs/connections (M5); one worked example per marker incl. `#Supersede` Decision→Need (S5, UC5) | U1-U4, M3-M5, S4-S6 | M |
| **CR-13** | **Close the BOOTSTRAP hole**: one sentence in §3e — a tooling change that alters the *meaning* of a computed view (done/ready/suspect/satisfaction) is CHANGE, not BOOTSTRAP | P5 | S |

Suggested order: CR-1 → CR-7 → CR-9 → CR-13 (small, stop active bleeding) →
CR-8 (restore the audit spine) → CR-2 → CR-3 → CR-4 → CR-5 → CR-6 (the structural
core) → CR-10 → CR-11 → CR-12.

## 5. Disposition (step 8 — human)

Each CR above awaits explicit accept / reject / defer. Accepted CRs enter the normal
CHANGE path (§3a) as backlog work items; rejections are recorded Decisions.
