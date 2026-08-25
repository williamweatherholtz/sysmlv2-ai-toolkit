# decision-channel — decisions as GitHub issues: auto-accept the plain, ask well on forks

The operating skill for the `decisionChannel` process (D0205 githubChannel + D0207 autoAccept).
Zero self-hosted components; the human's machine can be off; every automated act is an
inspectable workflow run; the gesture surface and the done-signal are one thread.

## The contract

1. **Non-fork decisions auto-accept** under the human's standing consent the moment they are posted.
   They are a COMMENT on ONE standing override thread that assigns nobody (D0227) — a decision that
   needs no human must not spend anyone's attention to exist — with a receipt that says plainly
   *not individually reviewed*. That thread is the override surface for all of them, forever. The acceptance note carries the
   `AUTO-ACCEPTED` machine token so views can always split auto from judged (never one number).
2. **Forks earn the ask.** A decision enumerating `OPTION X (label)` choices cannot auto-accept.
   Before it may even be proposed, the `judgment-request-quality` guard requires: a strong short
   name leading the title (one word before the colon, ≤ 28 chars); rationale ≥ 200 chars; a
   `RESEARCH` statement (what was looked at before asking — or "none found, and here is where I
   looked"); and a `COST` per option. If you can't fill those, you're not ready to ask.
3. **Split by attention, not by decision** (D0227). A fork gets its own assigned issue; a non-fork
   gets a comment. This REVERSED the human's earlier "one issue per decision" requirement, and only
   after asking them on a fork — it had opened 20 assigned issues in one day for decisions that
   needed nobody (issue258). The cost they accepted: a gesture must NAME the decision it reverses,
   `reject dNNNN <why>`, because one thread now carries many. A gesture on a fork's own issue still
   inherits the decision from the issue body.
4. **The gesture** is one letter (`B`), `accept`, or `reject <why>` as an issue comment (GitHub app
   or reply-to-notification-email). Latest comment wins; supersessions are announced.
5. **Receipts always**: 👀 on pickup, a receipt comment quoting exactly what entered git with the
   commit link, auto-close as the push-notified done-signal, failures spoken in-thread, a
   scheduled sweeper for missed gestures.
6. **Override forever**: `reject <why>` on any decision issue — open or closed — reopens it, gets
   acknowledged, and the reversal is recorded as a superseding fact. History is never rewritten.

## Adopting this in another project (inherit right away)

Since D0219 the unit **moves whole** — no hand-copying, and **no file needs editing**, because
nothing in the mechanism names a person or an owner.

1. In the SOURCE project: `keel process export decision-channel --out <dir>`. The bundle carries the
   definition, the skill, both workflows and both scripts (6 files + `unit.toml`).
2. In the TARGET project: `keel process import <dir>/decision-channel --update --assume-local-base`
   (`--update` because any `keel init` project already holds the definition; `--assume-local-base`
   bootstraps the three-way base for a pre-D0183 install). Engine files land under `.engine/`,
   the workflows and scripts at the project root. Then `keel activate decision-channel` for the
   `judgment-request-quality` guard.
3. Declare **YOUR** deciders in `.engine/contracts/github-actors.toml` under `[logins]` as
   `<githubLogin> = "<keelActor>"`. This table IS the authorization set — a mapped login may decide,
   an unmapped one is refused and never defaulted (issue182). It starts EMPTY in a fresh project on
   purpose (D0219): inheriting another project's decider would let their login record acceptances in
   your tree. Check with `keel github-decider <login>`.
   **The repo owner is not automatically a decider, and an ORG can never be one** — an org is not a
   person and cannot hold judgment. Assignment goes to the declared deciders, or to nobody; the
   `decision` label is what makes an issue findable.
   `keel process show decision-channel` lists the files that move AND the prerequisites, so the
   receiving project is told rather than discovering it when nothing records.
4. Ensure the repo's Actions token can push to the default branch (no required status checks /
   push restrictions blocking `GITHUB_TOKEN`; otherwise install a fine-grained GitHub App token on
   the bypass list and swap it into both workflows' checkout).
5. The standing consent is per-project and per-human: record YOUR project's equivalent of D0207
   quoting YOUR human's words, and set `standingConsent` in `attestation-policy.toml
   [decisionAcceptance]`. Without that declaration, every decision stays an individually-judged
   ask — auto-accept is opt-in by recorded consent, never a default keel ships.

## Hardening (do not relax; the red-team dossier is the why)

- Allowlisted login + `user.type == 'User'` is the FIRST workflow condition (public repos: anyone
  can comment; only the mapped human can record).
- Comment bodies travel via `env:` only; strict first-line parse; the keel binary records — never
  an agent (comment text is a prompt-injection vector).
- `permissions: {}` at workflow top; scoped per job; third-party actions pinned by SHA.
- The recorder's `git add` is SCOPED to `.engine .tracking` — never the workflow's scratch files.
- The in-workflow gate is the gate of record for recorder commits (GITHUB_TOKEN pushes don't
  retrigger CI); `keel audit-history` re-derives independently on the next push.

## Known limits, stated

- Rejection/override records the acknowledgment in-thread; the superseding reversal is recorded by
  the session (v1 scope).
- GitHub is the single external trust domain: account compromise and workflow fabrication are the
  stated residuals (dossier: docs/reviews/interaction-channel-panel-2026-08-23.md); live-comment
  verification by the guard is the chartered hardening.
