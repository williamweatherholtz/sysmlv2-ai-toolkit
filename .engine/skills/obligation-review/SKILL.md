---
name: obligation-review
description: Use when the human wants to review what is waiting on THEM, away from the terminal — "what needs my sign-off", "what's blocking on me". The board is the GitHub issues list (decision-channel, D0205/D0207); the deck survives as a local read view. Decisions record via GitHub comments with receipts, never via a connector inbox (retired, D0206).
---

# Obligation Review — the board is the issues list

**The human's board is GitHub**: `is:open label:blocks-work` on the project repo. An open issue is a
decision blocking work or a critical finding; the empty list means nothing needs them (D0204). Each
decision issue carries its own deciding context (rendered from `keel decision-card` — one parser,
never a second extraction) and its verdict channel is a one-letter comment; receipts and auto-close
come from the `decision-channel` process (see `.engine/skills/decision-channel/SKILL.md` for the
gesture grammar, the receipt ladder, auto-accept under standing consent, and the override path).

Point the human at:

```
https://github.com/<owner>/<repo>/issues?q=is%3Aopen+label%3Ablocks-work
```

One saved filter / mobile-app favorite. Nothing here is a queue they owe; it is what blocks work.

## The deck (`keel deck`) — a local read view, nothing more

`keel deck . --out <path>` still renders the computed blockers view (decision cards with their
why-panels, critical findings). Serving it via `keel serve` gives working tap-buttons on localhost
(the tested `local` transport). **The remote tap path is RETIRED** (D0206, accepted 2026-08-23 via
its own GitHub issue — the channel proved itself on its own cutover): `.engine/contracts/
deck-inbox.toml` is deleted, the page emits `INBOX=null`, and its transport line honestly says
`NONE - nothing tapped here reaches keel` outside localhost. Do not publish the deck as the decision
surface; publish nothing — the issues list IS the surface.

## Recording verdicts

- **GitHub comment** (the normal path): handled entirely by the Actions recorder — receipt, close,
  override thread. The session only ever `keel sync`s the result.
- **Chat words**: record via the write API with verbatim-quote provenance (D0192) and the companion
  quote receipt for confirmation flips (D0198).
- **Localhost console/deck** while the machine is on: the serve endpoints, as always.

The Smartsheet inbox sheet remains readable as history; nothing writes to it and no session sweeps
it. If a stray row ever matters, record it manually through the write API with row-level provenance.
