# The Interaction-Channel Panel — four lenses on replacing the decision channel

**Date:** 2026-08-23 · **Process:** adversarial-panel-review (D0187, version pin ccc5269) · **Convener:** claudeFable5 · **Trigger, their words:** *"I've accepted these multiple times and it doesn't seem they're saving. we need a better way to interact... the Claude approach isn't consistent and even where it is, it isn't quite introspectable... I'd like them recorded directly from a deck, but recording in GitHub is fine too... how to interact automatically with this terminal client, that isn't required to be on the machine, without requiring us to host something that's accessible everywhere? let's dig into this. make a panel and collaborate."*

**Ground truth established before the panel ran:** every one of their taps HAD landed (20 unprocessed inbox rows including a clean tap-to-sign OPTION B and two blind-spot-fence accepts, all recorded by the convener). The channel's defect is the missing feedback loop — a static artifact page cannot change in front of the person tapping it — compounded by three mutually opaque trust domains.

**Panelists:** GitHub-native architect · red team / failure analyst · terminal-session interaction researcher · feedback-loop UX designer. One round; convergence without contradiction; no refutation round (stated, per the process-value-panel precedent). Same-model panels claim convergence, never independence.

---

## SYNTHESIS — the converged design (proposed as D0205, githubChannel)

1. **Decision → GitHub Issue.** A proposed decision landing on main auto-opens an issue: short-name title, body = the decision's own plain-language context/options/costs (D0203), label `blocks-work`.
2. **Gesture = a one-letter comment** (`B`, `accept`, `reject <why>`) from the GitHub mobile app — or by replying to the notification email, which posts the comment for free. Latest comment wins; every supersession is announced in-thread; reactions (no event trigger) and checklists (weak attribution, two-boxes ambiguity) were evaluated and rejected.
3. **Recorder = a hardened GitHub Action**: allowlisted-login gate as the first condition (`comment.user.login`, never display name — the repo is public); comment body only via `env:` with a strict regex (script/prompt-injection killed); `permissions: {}` top-level; **the keel binary records, never an agent**; merge-only integration with the gate run on the merged tree; bounded retries; failures posted back onto the issue.
4. **Receipt ladder**, every rung on the phone: 👀 reaction ≤ 60 s → receipt comment quoting exactly what entered git + commit link ≤ 3 min → issue auto-closes (push-notified done-signal). A 10-minute sweeper re-drives missed events with an apology comment. Silence within budget is itself a reported failure.
5. **The board is the issues list** filtered `is:open label:blocks-work` — native state, not a regenerated copy (the static-deck failure class); empty list = nothing needs you (D0204).
6. **The terminal session is sync-only** for human judgment: acceptances recorded in Actions while the machine is off arrive via `keel sync`; the session also sweeps unprocessed comments idempotently, so Actions is *a* recorder, never a dependency. Optional later: `@claude` tag-mode (claude-code-action) for full agent reactions in Actions, and cron-scheduled autonomous runs — keel's CI gates make them safe-by-construction.
7. **tap-to-sign (D0201 B) amended:** GitHub's 2FA-backed comment identity subsumes the device HMAC for this channel; the receipt is the re-fetchable comment id/URL; the guard verifies the actor id against the live comment; the periodic human-signed ratification stays as the offline anchor. Residuals stated: account compromise (same class as key theft, different custodian) and a compromised workflow fabricating as the bot (closed by live-comment verification).
8. **Retire** the Smartsheet inbox and the artifact-deck save path after one end-to-end dry run; `keel deck` survives as a computed view.

**Constraint scorecard (unanimous):** recorded in GitHub — native; no hosting — GitHub hosts everything; machine off — durable compute fires on the event; introspectable — comment, workflow file, run log, commit, receipt are all plain artifacts, replacing three opaque trust domains with one.

**Status-quo autopsy (red team):** three root causes — no receiver-side acknowledgment designed in (the issue194 class), three mutually opaque trust domains (structural, unfixable), and a dead-drop store only a live session can watch. Salvageable: `keel deck` as a view, the idempotent record-then-verify loop, the label-ladder discipline.

*The four full reports follow, verbatim.*

---

## PANELIST 1 — GITHUB-NATIVE ARCHITECT

### DESIGN

**Repo facts checked:** public repo, `main` protected (enforce_admins on, force-push blocked — matches D0129), CI already builds `keel` and runs the full gate. `keel accept` exists as a CLI and its agent-marker refusal keys on `CLAUDECODE*` env vars — absent in Actions runners, so CI can invoke it. `keel deck` already emits the HTML.

**(a) Decision surface — Issue + comment-command.** Workflow `decision-issue.yml`, trigger `push` to `main` filtered on `.engine/decisions/*.sysml`. Parses any decision with `status = proposed`, opens an Issue labeled `decision`, body = the file's own context/decision/consequences rendered as markdown, plus one fenced how-to-vote block (`/accept B` · `/accept` · `/reject <why>`). Comment-command, not reactions or labels: reactions are invisible to `issue_comment` triggers and don't push-notify; labels are fat-finger-prone; neither carries a note. A comment is first-class in the mobile app, carries the human's verbatim words (which `keel accept --note` requires — D0106), and is timestamped + identity-bound. Assign the issue so mobile push notifications fire.

**(b) Recording path — `decision-accept.yml`:** on `issue_comment created`, gated on the `decision` label + `startsWith(body, '/accept')` + `comment.user.login == 'williamweatherholtz'` (hardcoded allowlist). Steps: checkout fetch-depth 0 with an App token; restore/build keel (~1 min warm with rust-cache); map github.actor → keel actor via a committed `.engine/contracts/github-actors.toml`; run `keel accept <d> --note "<verbatim comment + option>" --by wweatherholtz --date <utc>`; run the honest-state gate BEFORE push; commit, `git pull --no-rebase`, push (merge-only retry, D0129 land semantics); close the issue with a receipt: "Recorded: d0201Accept judgedBy=wweatherholtz against <sha>. Commit <link>." The closing receipt IS the feedback loop — tap, then a push notification when the issue closes. Auth: GITHUB_TOKEN can't reliably push through protection with enforce_admins, and GITHUB_TOKEN-pushed commits don't trigger workflows (would silently skip the CI gate) — use a fine-grained GitHub App token on the ruleset bypass list.

**(c) The deck — GitHub Pages, regenerated per push.** Issues = interaction surface, Pages = state surface: `keel deck . --out site/index.html` on every push, always computed-at-HEAD. Caveat: public repo → Pages world-readable (the decision files already are). No tap buttons on Pages (static, no auth); the deck links each pending decision to its Issue.

**(d) Actor mapping + D0201.** `github-actors.toml`: `williamweatherholtz = "wweatherholtz"`; unmapped logins → refuse, never default (issue182). GitHub identity SUBSUMES D0201 OPTION B for this channel: a `/accept` comment is authenticated by GitHub login (2FA-backed), timestamped server-side, immutably event-logged — a stronger, externally-operated version of the device-HMAC tap. Record an amending Decision; the HMAC machinery stays only if the localhost console channel keeps it.

**(e) Terminal session: sync-only.** Machine off: comment → Actions builds keel, records, gates, merges, closes — done and green with zero local involvement. Next session start: `keel sync` picks it up. The session never records human judgment again — it only proposes.

### CONSTRAINT SCORECARD
"recorded from a deck / GitHub is fine" — MET, 9/10 · "not on the machine, no hosting" — MET, 10/10 · "consistent, introspectable" — MET, 10/10 (comment, run log, commit, receipt — all inspectable; deterministic YAML, not an artifact runtime).

### WHAT COULD BITE
1. enforce_admins + GITHUB_TOKEN: must use an App token on the bypass list — the one real setup cost. 2. Concurrent push race: merge-only retry + a `concurrency:` group. 3. `keel accept`'s interactivity guard: keyed to CLAUDECODE markers today; if it ever tightens to no-TTY, CI breaks — a `KEEL_CI_ATTESTATION_CHANNEL` escape guarded by the comment-URL citation belongs in the amending Decision. 4. Public repo: decision context world-readable; revisit if private (Actions minutes then metered; the ~1-2 min warm build is trivial either way). 5. Comment parsing: first-line-only strict form, one open issue per decision, latest-uncontradicted-wins (the D0201 cluster lesson).

### MIGRATION STEPS
1. `github-actors.toml` + three workflows, under one Decision superseding the Smartsheet path and amending D0201-B. 2. End-to-end dry run on a synthetic proposed decision. 3. Retire the inbox poller; deck tap-buttons become issue links; obligation-review skill re-pointed. 4. Drain unprocessed inbox rows first; cut over in one commit.

Sources: IssueOps (github.blog), github/command, peter-evans/slash-command-dispatch, protected-branch push discussion (community#25305), App-token push how-to, mobile notification limits (community#168685), Actions pricing changelog 2025-12, free-tier limits.

---

## PANELIST 2 — RED TEAM / FAILURE ANALYST

### STATUS-QUO AUTOPSY (root causes)

Four independent failures, three root causes: (1) **No receiver-side acknowledgment was designed in** — the retired live-doc path (issue194) and "no save feedback" are the same defect: a transport chosen for availability, not ack semantics. (2) **Three mutually opaque trust domains** — the artifact runtime (CSP sandbox, per-session capability contract), the Smartsheet connector (OAuth expiry, per-artifact consent, five error branches), and keel. A failure in either foreign domain is uninstrumentable from keel — that IS the "isn't quite introspectable" complaint, and it is structural. (3) **The store is a dead drop** — only a live AI session holds connector access. Salvageable: `keel deck` (computed view), the idempotent record-then-verify loop with latest-At dedup, the label-ladder discipline. Kill the Smartsheet middle and the artifact save path.

### ATTACKS PER CANDIDATE

**GitHub issue-ops — survives, hardened.** Verified: the repo is PUBLIC; `main` has no required status checks and no push restrictions (only enforce_admins + no force-push). Attacks: **who can trigger** — `issue_comment` workflows run in default-branch context and are NOT covered by first-time-contributor approval; any account can comment today; the hard actor gate (login + `user.type == 'User'`, never display name or author_association) is the mandatory first line. **Injection** — interpolating the comment body into `run:` is the classic hole (Wiz; Trivy's post-incident audit); pass via `env:`, strict regex; and the 2025-26 twist: a comment is also a prompt-injection vector, so the Action must run the keel binary, NEVER an agent (CSA "Comment and Control"). **Token scope** — `permissions: {}` top-level, per-job contents/issues write; no pwn-request surface while checking out main, never a PR head. **Replay/duplicates** — key by comment id + created_at; ignore `edited` for recording; keel's latest-At dedup absorbs the rest. **Divergence** — the Action does exactly D0129: fetch, merge (never rebase), re-gate the merged tree, push, bounded retries, failure commented back. **Spoofing** — confirmed win: `comment.user` is server-authenticated. Caveat: the committed record is authored by the bot, so the identity claim lives in the transcribed payload — see D0201. **Outage/latency** — the comment is the durable signed gesture; the Action is merely one recorder; the local session also sweeps unprocessed comments idempotently. Residuals stated: world-readable decisions (already public in-tree), cosmetic comment spam the gate ignores.

**Discussions** (weaker events, no close semantics): kill. **PR-review approvals** (binary, per-decision branches violate main-only): kill. **Issue forms**: keep as the CREATION format (structured options); comments stay the verdict channel. **Email-to-action bridges**: hosting; kill — but replying to the GitHub notification email posts the comment, so candidate 2 already IS email-to-action, free. **Telegram/Slack bots**: hosting; kill. **Tailscale→local serve**: machine-on only; keep as the local rung. **Codespaces**: session-must-be-alive again; kill.

### THE D0201-B QUESTION

GitHub makes the device-HMAC largely redundant: 2FA-backed gesture→identity binding, stronger provisioning than a hand-carried key, and the comment (id, actor id, timestamp) is an independently re-fetchable receipt. Honest residuals to record as an amendment: (1) account compromise substitutes for key theft — same class, different custodian; (2) the token in the Action can fabricate (`github-actions[bot]` authors the commit) — close by having the guard verify the LIVE comment via API (fact stores the comment URL/id; verification re-fetches the actor id), and keep the periodic human-signed ratification as the offline anchor — GitHub receipts die if the issue/repo/account does; an HMAC verifies offline forever.

### HARDENING CHECKLIST
1. Job-level actor gate (login + type). 2. `permissions: {}` top-level; scoped per job; third-party actions pinned by SHA. 3. Body via `env:`, strict regex, no agent in the loop. 4. Idempotency key = comment id; ignore edits for recording; latest-At dedup stays. 5. Merge-only divergence loop; failures onto the issue. 6. Local-session sweep so Actions is a recorder, not a dependency. 7. Guard verifies human-judged facts against the live comment; periodic signed ratification. 8. Issue forms for creation; `/accept <OPTION>` for verdicts; optional thread-lock.

Sources: Wiz GitHub Actions security guide; Trivy discussion #10402; Praetorian pwn requests; StepSecurity; CSA Comment and Control; GitHub changelogs and docs cited in-line.

---

## PANELIST 3 — TERMINAL-SESSION INTERACTION RESEARCHER

### SESSION-OPTIONAL BASELINE

It goes almost all the way. Keel's own invariants make the terminal session structurally optional for the decision channel: state is computed from text in git, CI already builds keel and runs the kernel-free gates on every push plus `audit-history` re-deriving verdicts from the tree, and D0129 makes `orient` state the tree it computed against. So: phone → GitHub comment → Action runs `keel accept` with explicit `--by`/`--at` (provenance cannot be defaulted; the write refuses) → commit → CI gates it. The local session, whenever next alive, runs `keel sync` and the frontier unblocks. Nothing is lost except reaction latency — the only thing a wake mechanism would buy — and the reaction can run in Actions too. The multi-writer worry (local session + Action bot both writing main) is already solved doctrine: D0108/D0129 ownership + merge-only + gate-on-merged-tree; enroll the bot as an Actor kind=ai via actor-enrollment. Verdict: adopt session-optional as the architecture; treat everything else as latency optimization.

### WHAT EXISTS TODAY (verified, cited)

- **claude-code-action (tag mode)** — real, official, GA: `@claude` in an issue/PR comment triggers a full Claude Code agent run inside an Actions runner with the thread in context; can edit, push, open PRs; reports progress in one updated comment. Auth: ANTHROPIC_API_KEY, or CLAUDE_CODE_OAUTH_TOKEN from `claude setup-token` (subscription-billed), or OIDC with no stored secret. Can replace the terminal for REACTIONS — provided the Action records only what the comment explicitly states (confirmation-authenticity still applies).
- **claude.ai/code web sessions** — real; cloud sessions persist with the browser closed, phone-startable; `claude --teleport` continues one in the terminal later.
- **Claude mobile app** — real; starts/monitors cloud sessions; `/remote-control` drives a LOCALLY running session from the phone (machine-on only).
- **Scheduled prompts** — documented: `on: schedule` cron workflows running claude-code-action in agent mode.

### WAKE/SCHEDULE OPTIONS RANKED

1. **Don't wake it — react in Actions** (tag mode). Zero hosting, machine off, every run an inspectable log. Winner.
2. **Cron-scheduled Action**: agent run does `keel sync && keel orient`, processes pending, commits. Caveats: 5-30 min scheduling jitter, schedules auto-disable after 60 days of repo inactivity (non-issue here); keep `workflow_dispatch` as manual override. Safety largely by-construction (honest-state gates + tree-derived audit) plus action-level rails (tool allowlist, scoped prompt, timeout, concurrency group mirroring the write-lock).
3. **Local-session polling when the machine is on**: `gh api` with ETag conditional requests (304s are rate-limit-free; obey X-Poll-Interval). A latency supplement, never the backbone.
4. **`/remote-control`** — ad-hoc phone-drives-terminal when the session is live.
5. **Push-shaped without hosting: does not exist.** Webhooks need a receiver; tunnels are hosting-in-disguise. GitHub's push notifications reach the HUMAN's phone, which is the only party needing push.

### RECOMMENDED INTERACTION TOPOLOGY

phone (GitHub app / notification email) → GitHub (issues + workflow files = the introspectable channel) → Actions runner (keel binary records; optionally claude-code-action reacts) → git main ← CI gate on every push → local terminal session syncs whenever next active; optional fetch-poll while on. One truth (git), two reactors (Actions when the machine is off, terminal when on), zero hosted components.

Sources: anthropics/claude-code-action; code.claude.com docs (github-actions, web, remote-control, scheduled tasks); GitHub REST best practices; scheduled-workflow limitations (dev.to/ksivamuthu).

---

## PANELIST 4 — FEEDBACK-LOOP / UX DESIGNER

Decisive technical fact: GitHub fires Action events for COMMENTS (`issue_comment` created/edited) but NOT for reactions — a reaction gesture could only be polled, recreating the dead air that caused eleven re-taps. That settles the gesture.

### THE GESTURE

**A one-word comment naming the option — `B` (or `accept B`), parsed leniently; editing your comment is how you change your mind.** Reactions: no trigger — disqualified. Checklist taps: edit the issue body — weak attribution, nothing stops checking two boxes (the all-highlight bug rebuilt as a data model) — disqualified. Close-with-label: ambiguous for forks and steals the close gesture, which this design reserves as the done-signal the machine sends, never a gesture the human performs. A comment is attributable (GitHub auth — delivering D0201's signed-attestation intent for free), timestamped, push-notification-generating, one thumb. The issue body enumerates options as bold single letters with what-changes/cost/what-rejection-means (D0203), so the reply space is closed; a non-parsing reply gets an immediate "didn't parse — reply exactly A or B." **Contradictions self-resolve by construction:** latest comment (or edit) is the answer; identical re-comment is idempotent ("already recorded at <SHA>"); a different one supersedes and the bot says what it replaced. No tap cluster can be ambiguous the way the 13:05Z D0201 cluster was.

### THE RECEIPT LADDER

| Step | Surface | Budget | On failure (never silent) |
|---|---|---|---|
| 1 Gesture posted | your comment renders — native, offline-queued by the app | instant | the app shows unsent state |
| 2 Picked up | 👀 reaction ON your comment | ≤60 s | no 👀 in 2 min → 10-min sweeper re-drives + "picked up late — your choice was not lost" |
| 3 Recorded | bot comment quoting EXACTLY what entered git: short name, option, judged_by, judgedAt, linked commit SHA — "recorded" claimed only from receiver-side read-back (D0173) | ≤3 min | error verbatim as a comment + record-failed label; issue stays open; the run's ❌ visible |
| 4 Auto-close | issue closes (completed) — the unmissable done-signal, push-notified even with the app closed | +seconds | receipt comment stands: "recorded but not closed — the comment is truth" |
| 5 Board moves | the filtered list count drops — same datum, not a copy | instant | n/a |

Gesture → closed issue in under 4 minutes, two intermediate proofs of life, every failure speaking into the thread the gesture was made in.

### THE BOARD

**The issues list itself, filtered `is:open label:blocks-work`** — not Pages, not a pinned tracker. Both are regenerated COPIES of state, and a copy can lag: the static-deck failure class in new clothes. The filtered list IS the state; closing removes natively; the empty list is "nothing needs you" (D0204). At most a pinned issue holding the filter link — a door, never a mirror.

### DESIGN RULES (each traceable to a past failure)

1. Act where the state lives — gesture surface and done-signal are one thread. (Eleven re-taps on a page that could not change.)
2. Every confirmation is receiver-side — 👀, quoted record, commit SHA; never the sender's own render. (issue193/194; D0173's row-id receipt.)
3. Latest comment wins; every supersession announced. (The D0201 cluster needed forensic reconstruction.)
4. Silence is a failure state — a missed rung is reported in-thread. ("it doesn't seem they're saving.")
5. Open = blocks work; closed = done; nothing else exists on the surface. (D0204.)
6. The card carries its own why — short name, context, costs, rejection meaning; no IDs. (D0203.)

Sources: GitHub Mobile discussions and changelog; GitHub reactions REST API (no webhook events for reactions).
