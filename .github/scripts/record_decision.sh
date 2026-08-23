#!/usr/bin/env bash
# record_decision.sh (D0205 githubChannel): transcribe the human's authenticated issue-comment
# gesture into the tree via the keel binary, then close the loop with a receipt.
#
# HARDENING (the red-team checklist, dossier 2026-08-23): the comment body arrives ONLY via env
# (never interpolated into shell by the workflow), is parsed by strict pattern here, and nothing in
# this path invokes an agent. The caller has already gated on the allowlisted login; this script
# re-checks against the committed mapping table anyway (two opinions, issue182: never default).
#
# Inputs (env): ISSUE_NUMBER, COMMENT_BODY, COMMENT_ID, COMMENT_URL, COMMENT_USER, GH_TOKEN.
set -euo pipefail

KEEL=./target/release/keel

# ── AUTO mode (D0207 standing consent): a non-fork proposed decision auto-accepts at issue
# creation. The issue is notification + the forever-open override thread; the receipt says plainly
# that nothing was individually reviewed. Inputs: AUTO_DECISION, ISSUE_NUMBER, ISSUE_URL, GH_TOKEN.
if [ -n "${AUTO_DECISION:-}" ]; then
  today=$(date -u +%F)
  note="AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on ${ISSUE_URL}."
  "$KEEL" accept "$AUTO_DECISION" --note "$note" --by wweatherholtz --date "$today"
  "$KEEL" validate .
  "$KEEL" guard
  git config user.name "keel-recorder"
  git config user.email "keel-recorder@users.noreply.github.com"
  git add .engine .tracking   # scoped: never the workflow's own scratch files (cards.json, auto_queue.txt)
  git commit -m "Auto-accept ${AUTO_DECISION} under standing consent (D0207) - override thread: ${ISSUE_URL}

Gate: run in-workflow (GITHUB_TOKEN pushes do not retrigger ci.yml); audit-history re-derives.

Co-Authored-By: keel githubChannel recorder <keel-recorder@users.noreply.github.com>"
  for attempt in 1 2 3; do
    if git push origin main; then
      sha=$(git rev-parse --short HEAD)
      gh issue comment "$ISSUE_NUMBER" --body "**Auto-accepted** under your standing consent (D0207) - not individually reviewed. Commit ${sha}. Reply \`reject <why>\` here anytime to reverse; this thread stays the override surface."
      gh issue close "$ISSUE_NUMBER" --reason completed --comment "In the tree. Nothing needs you unless you disagree."
      exit 0
    fi
    git fetch origin main && git merge --no-edit origin/main && "$KEEL" validate . && "$KEEL" guard
  done
  gh issue comment "$ISSUE_NUMBER" --body "Auto-accept FAILED after 3 push attempts; the sweeper or next session will complete it."
  exit 1
fi

# ── who ────────────────────────────────────────────────────────────────────────────────────────
actor=$(grep -E "^${COMMENT_USER} *= *\"" .engine/contracts/github-actors.toml | sed 's/.*= *"\(.*\)"/\1/' || true)
if [ -z "$actor" ]; then
  gh issue comment "$ISSUE_NUMBER" --body "GitHub login \`$COMMENT_USER\` is not in .engine/contracts/github-actors.toml - nothing recorded (provenance is never defaulted)."
  exit 0
fi

# ── which decision ─────────────────────────────────────────────────────────────────────────────
decision=$(gh issue view "$ISSUE_NUMBER" --json body --jq .body | grep -oE 'keel-decision: d[0-9]+' | head -1 | cut -d' ' -f2 || true)
if [ -z "$decision" ]; then
  gh issue comment "$ISSUE_NUMBER" --body "This issue carries no keel-decision marker - nothing recorded."
  exit 0
fi

# ── what they said (strict parse; first line only) ─────────────────────────────────────────────
first_line=$(printf '%s' "$COMMENT_BODY" | head -1 | tr -d '\r' | sed 's/^ *//;s/ *$//')
verdict=""
option=""
case "$first_line" in
  [A-Za-z])                    verdict=accept; option=$(printf '%s' "$first_line" | tr '[:lower:]' '[:upper:]');;
  accept|/accept|Accept)       verdict=accept;;
  accept\ [A-Za-z]|/accept\ [A-Za-z]|Accept\ [A-Za-z])
                               verdict=accept; option=$(printf '%s' "$first_line" | awk '{print toupper($2)}');;
  reject*|/reject*|Reject*)    verdict=reject;;
  *)
    gh issue comment "$ISSUE_NUMBER" --body "Didn't parse \`$first_line\` - reply with just the option letter (e.g. \`B\`), or \`accept\`, or \`reject <why>\`."
    exit 0;;
esac

# ── already recorded? (idempotency: the receipt names the comment id) ──────────────────────────
if gh issue view "$ISSUE_NUMBER" --json comments --jq '.comments[].body' | grep -q "receipt-for-comment: $COMMENT_ID"; then
  echo "comment $COMMENT_ID already has a receipt"
  exit 0
fi

if [ "$verdict" = "reject" ]; then
  # Rejection / post-fact override (D0205 v1 scope + D0207 clause 2): acknowledged in-thread,
  # reopened if the issue was closed (an auto-acceptance being overridden), labeled for the
  # session to record the superseding reversal - accepted history is never rewritten in place.
  state=$(gh issue view "$ISSUE_NUMBER" --json state --jq .state)
  if [ "$state" = "CLOSED" ]; then
    gh issue reopen "$ISSUE_NUMBER"
  fi
  gh label create needs-reversal --force --color D93F0B --description "override of an auto-acceptance awaiting the superseding record" >/dev/null 2>&1 || true
  gh issue edit "$ISSUE_NUMBER" --add-label needs-reversal
  gh issue comment "$ISSUE_NUMBER" --body "Override noted (receipt-for-comment: $COMMENT_ID). The superseding reversal will be recorded - this issue stays open until it is in the tree."
  exit 0
fi

# ── record ─────────────────────────────────────────────────────────────────────────────────────
today=$(date -u +%F)
note="${option:+OPTION $option - }their words, verbatim: '$first_line' (GitHub comment $COMMENT_URL, id $COMMENT_ID, authenticated login $COMMENT_USER)"
"$KEEL" accept "$decision" --note "$note" --by "$actor" --date "$today"

# gate the tree BEFORE anything is pushed - the same honest-state bar every writer meets. Note:
# commits pushed with GITHUB_TOKEN do not re-trigger ci.yml, so THIS gate run is the gate of
# record for this commit; audit-history re-derives it independently on the next human push.
"$KEEL" validate .
"$KEEL" guard

git config user.name "keel-recorder"
git config user.email "keel-recorder@users.noreply.github.com"
git add .engine .tracking   # scoped: never the workflow's own scratch files (cards.json, auto_queue.txt)
git commit -m "Record ${decision} acceptance via githubChannel (comment ${COMMENT_ID} by ${COMMENT_USER})

Gesture: ${COMMENT_URL}
Gate: run in-workflow (GITHUB_TOKEN pushes do not retrigger ci.yml); audit-history re-derives.

Co-Authored-By: keel githubChannel recorder <keel-recorder@users.noreply.github.com>"

# merge-only push loop (D0129: never rebase; bounded retries; failure speaks in-thread)
for attempt in 1 2 3; do
  if git push origin main; then
    sha=$(git rev-parse --short HEAD)
    gh issue comment "$ISSUE_NUMBER" --body "**Recorded.** ${decision} accepted${option:+ - OPTION $option} · judged_by \`$actor\` · $today · commit ${sha} (receipt-for-comment: $COMMENT_ID)
${GITHUB_SERVER_URL:-https://github.com}/${GITHUB_REPOSITORY}/commit/${sha}"
    gh issue close "$ISSUE_NUMBER" --reason completed --comment "Done - this decision is in the tree."
    exit 0
  fi
  git fetch origin main
  git merge --no-edit origin/main
  "$KEEL" validate . && "$KEEL" guard
done

gh issue comment "$ISSUE_NUMBER" --body "Recording FAILED after 3 push attempts (contention). Your gesture is safe in this thread; the sweeper or the next session will record it. (receipt-pending-for-comment: $COMMENT_ID)"
exit 1
