# decision-channel — decisions as GitHub issues: auto-accept the plain, ask well on forks

The operating skill for the `decisionChannel` process (D0205 githubChannel + D0207 autoAccept).
Zero self-hosted components; the human's machine can be off; every automated act is an
inspectable workflow run; the gesture surface and the done-signal are one thread.

## The contract

1. **Non-fork decisions auto-accept** under the human's standing consent the moment their issue is
   raised. The issue is a notification and a forever-open override thread — it closes immediately
   with a receipt that says plainly *not individually reviewed*. The acceptance note carries the
   `AUTO-ACCEPTED` machine token so views can always split auto from judged (never one number).
2. **Forks earn the ask.** A decision enumerating `OPTION X (label)` choices cannot auto-accept.
   Before it may even be proposed, the `judgment-request-quality` guard requires: a strong short
   name leading the title (one word before the colon, ≤ 28 chars); rationale ≥ 200 chars; a
   `RESEARCH` statement (what was looked at before asking — or "none found, and here is where I
   looked"); and a `COST` per option. If you can't fill those, you're not ready to ask.
3. **One issue per decision** — the human's own requirement, to keep the gesture unambiguous.
4. **The gesture** is one letter (`B`), `accept`, or `reject <why>` as an issue comment (GitHub app
   or reply-to-notification-email). Latest comment wins; supersessions are announced.
5. **Receipts always**: 👀 on pickup, a receipt comment quoting exactly what entered git with the
   commit link, auto-close as the push-notified done-signal, failures spoken in-thread, a
   scheduled sweeper for missed gestures.
6. **Override forever**: `reject <why>` on any decision issue — open or closed — reopens it, gets
   acknowledged, and the reversal is recorded as a superseding fact. History is never rewritten.

## Adopting this in another project (inherit right away)

1. `keel activate decision-channel` (the unit brings the `judgment-request-quality` guard).
2. Copy from this repo, unchanged unless noted:
   - `.github/workflows/decision-issue.yml`
   - `.github/workflows/decision-record.yml` — **edit the allowlist login** in the job `if`
   - `.github/scripts/open_decision_issues.py`
   - `.github/scripts/record_decision.sh`
3. Author `.engine/contracts/github-actors.toml` mapping each GitHub login to its keel Person.
   Unmapped logins are refused, never defaulted.
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
