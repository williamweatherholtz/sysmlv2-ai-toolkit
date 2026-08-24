# Brief for panel review: adoption, portability, elicitation — and why reports drift from truth

**Status:** DRAFT for adversarial panel review (D0187). Nothing here is accepted.
**Author:** claudeFable5, 2026-08-24. **Repo root:** the keel self-build.
**Panelists: verify every claim below against the REPOSITORY, never against this document.**

The human asked six questions and requested a panel. The questions, verbatim:

1. Do we have a portable means of eliciting feedback?
2. Of transferring processes?
3. If I start a new keel project, is there an onboarding stage where I am guided to pick which
   processes should apply?
4. Or an AI skill process that helps the author identify which processes should apply?
5. Is all user input being recorded as statements, then translated into user stories? *"Being able
   to pull latent needs out of a user is quite critical for the average user."*
6. How discoverable is keel's process or tools to projects?

And a seventh, which is the one that matters most:

7. *"It feels like we're still regularly out of sync with what's being reported and what the truth
   is. Why?"*

---

## Part 1 — Verified ground truth (measured this session, pre-panel)

Each row was measured by running the tool named. Panelists should re-run and challenge.

| # | Question | What actually exists | Measurement |
|---|---|---|---|
| G1 | Feedback elicitation | `keel render <view> --mode review` renders any declared view for human markup; `keel apply-review --batch F` writes the markup back as linked critiques. The GitHub decision-channel (D0205/D0207) carries proposed Decisions out and letter-gestures back. A deck exists. | `keel` help lists both; the channel has live issues #1–#6 |
| G2 | Process transfer | `keel process list\|search\|show\|export --out <dir>\|import <dir>`. A unit carries its definition, deploying skill, and the rules/guards a receiver must activate. | `keel process --help` |
| G3 | **Transfer is HALF-COVERAGE** | **12 of 23 process definitions are NOT in the exportable palette** — including `intake`, `definition-of-done`, `definition-of-ready`, `adversarial-panel-review`, `architectural-critique`, `review-critique`, `migration`, `report`, `indicator`, `diagram`, `introduction`, `obligation-review`. | `ls .engine/processes/*.sysml` = 23; `keel process list` = 11 |
| G4 | **Root cause of G3** | Palette membership = `activation::Activation::unit_names()`; a unit is formed from the activation model, which keys on a process declaring a **guard** (`constraint def` named as the camelCase of a guard name). A process that declares NO guard forms no unit, so it cannot be exported or imported **by construction** — even though it has a definition file AND a deploying skill. | `process_cmd.rs` `fn rows()` → `act.unit_names()`; `activation.rs` `units_from_model()` |
| G5 | Onboarding exists | `keel init DIR [--profile strict\|guided]` scaffolds. An `introduction` process + skill takes a newcomer "from zero to FIRST VALUE in one guided session: mental model, first Need, first requirement + work item, first sprint, then `orient`." | `keel init` usage; `.engine/processes/introduction.sysml` purpose |
| G6 | **No process-SELECTION onboarding** | Neither `init` nor `introduction` guides *which processes should apply*. `introduction` teaches the loop on whatever is active; `--profile strict\|guided` sets enforcement severity, not process choice. Selection is `keel activate\|deactivate <name>` — a command you must already know to run, against a palette that is missing 12 of 23. | `introduction.sysml` purpose text; `adoption_profile()` in `main.rs` reads only `strict\|guided\|undeclared` |
| G7 | Intake chain exists | `Statement` → `UserStory` → downstream, computed by `keel intake` (D0166): unparsed / unrouted / unsourced. | `keel intake` |
| G8 | **Intake covers ~3% of the work** | 17 statements, 20 user stories, **599 of 619 downstream items UNSOURCED**. So: no, user input is not systematically recorded as statements and translated to stories. 97% of tracked work traces to no recorded human utterance. | `keel intake` |
| G9 | Discoverability surface | 30 skills in `.engine/skills/`; 23 process definitions; 48 guards; 32 viewpoints; `keel activation` reports adoption. | directory counts |
| G10 | **Two catalogues disagree** | `keel activation` reports 23 processes as adopted/active. `keel process list` reports "11 in this project's palette." Both are shipped, computed, first-class surfaces answering "what processes does this project have," and they disagree by 12. | run both |

### What G1–G10 say about question 7 (report-vs-truth drift)

Four drift mechanisms are already evidenced *in this session alone*:

- **D1 — Two computed views of the same question, no reconciliation control.** G10. Nothing compares
  `activation`'s process set against the palette's, so a 12-item disagreement between two shipped
  answers to one question persisted invisibly.
- **D2 — A coverage metric whose denominator excludes what it cannot see.** G4/G3: the portability
  layer reports on 11 units and is *correct* about those 11; the 12 it structurally cannot represent
  are absent, not flagged. Absence reads as "nothing there," never as "not covered."
- **D3 — Assertions in prose that no gate can check.** This session: I told the human D0209 was
  "overridable on its GitHub issue" when no issue existed (issue238). CLAUDE.md records the class
  (D0151): §3's verify-before-asserting clause "binds what you SAY and has no control behind it —
  no gate can read conversational output."
- **D4 — Stale artifact read as fresh evidence.** This session: a background release build exited 0
  but had failed to relink (`os error 5`), so a demo against the stale binary produced a false
  SILENT that looked like working code. Exit-0 was read as proof of relink.

---

## Part 2 — Candidate improvements (UNACCEPTED; the panel's job is to kill or sharpen these)

Each is stated so it can be refuted. Panelists: attack the premise, the mechanism, and the ROI order.

- **C1 — Decouple process-unit identity from guard declaration.** A process becomes a transferable
  unit if it has a definition + a deploying skill; guards become an *optional* attribute of the unit
  rather than its identity condition. Closes G3/G4 (12 → 0 non-transferable).
- **C2 — One catalogue, one answer.** Reconcile `activation` and `process list` into one computation
  with a two-way diff control (the shape D0195 already uses for hazard/control), so a disagreement
  fails a gate instead of sitting in two outputs. Closes G10/D1.
- **C3 — A process-selection onboarding stage.** Extend `introduction` (or add a `keel adopt`) that
  walks the palette, asks what the project is building, and proposes an activation set — recording
  the choice as a Decision so the adoption is auditable rather than default-everything.
- **C4 — An elicitation skill that pulls latent needs.** A guided interview process whose OUTPUT is
  authored `Statement`s (verbatim human utterances) plus proposed `UserStory` routings for
  confirmation — targeting G8's 97% unsourced. The average user cannot author a Need; they can
  answer questions about their pain.
- **C5 — Sourcedness as a reported indicator with a stated denominator.** `keel intake` already
  computes unsourced=599; make the *ratio* a first-class indicator so the 3% is visible in `orient`
  rather than only on demand. (Deliberately an indicator, not a gate — D0088/Goodhart: back-filling
  fake statements to move a gated number would be worse than the gap.)
- **C6 — A drift-class control for D1: "two views, one question."** Any two shipped surfaces that
  answer the same question must be reconciled by a computed diff, and the diff must be gated.
- **C7 — Bound D3 structurally instead of asking for vigilance.** D0151 concedes no gate can read
  conversational output. Candidate: make the *claim* a recorded artifact when it is load-bearing —
  e.g. any assertion of the form "you can do X at URL Y" must cite a computed view, and the surfaces
  the AI quotes from must be generated, not typed.

---

## Part 3 — What the panel is asked to decide

1. Is the **root-cause diagnosis of question 7** (D1–D4) right, and is it complete? Name any drift
   mechanism these four miss.
2. Which of C1–C7 are **load-bearing**, which are **redundant with existing machinery**, and which
   are **actively harmful** (metric-gaming, friction, or false assurance)?
3. What is the correct **payback order**, given that adoption friction is the recorded #1 risk
   (D0054) and that gating a completeness metric is forbidden (D0098)?
4. For question 5 specifically: is a guided-elicitation process (C4) the right mechanism to pull
   latent needs from an average user, or does it manufacture Needs the user does not have — the
   N-8 defect D0157 already paid for ("written WIDER THAN THE DEMAND")?
5. **Is the 97%-unsourced number (G8) a defect at all?** Steelman the other side: this is a
   self-build whose work was chartered by Decisions (legitimate under D0064). Distinguish
   "unsourced because nobody asked for it" from "unsourced because the asking was never recorded."

## Standing caveat on this panel's own validity

Every panelist is the same model lineage. Per the deferred `dcHeterogeneousCritics` (parked by
D0210 pending second-model access), this panel can honestly claim **convergence, not independence** —
same-model panels carry correlated blind spots. Findings should be read with that discount.
