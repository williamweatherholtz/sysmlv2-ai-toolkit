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
  # WORKSPACE (D0234): the channel id may be QUALIFIED - `alpha/d0001` - because `dNNNN` is unique
  # only within a project and this repo may hold several. Split it in the BINARY, not in shell: the
  # project half decides which TREE the human's acceptance is written into, and getting that wrong
  # records their judgment against the wrong project.
  split=$("$KEEL" github-decision-id "$AUTO_DECISION")
  proj=$(printf '%s' "$split" | cut -f1)
  AUTO_DECISION=$(printf '%s' "$split" | cut -f2)
  if [ "$proj" != "." ] && [ -d "$proj" ]; then
    echo "recorder: decision belongs to project '$proj' — recording there"
    cd "$proj" || { echo "recorder: cannot enter '$proj'"; exit 1; }
    KEEL="$(cd "$OLDPWD" && pwd)/${KEEL#./}"
  fi
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
      gh issue comment "$ISSUE_NUMBER" --body "**Auto-accepted** under your standing consent (D0207) - not individually reviewed. Commit ${sha}. Reply \`reject ${AUTO_DECISION} <why>\` here anytime to reverse; this thread stays the override surface."
      # NEVER CLOSE THE STANDING THREAD (issue262). Under D0227 a non-fork's receipt lands on the ONE
      # shared thread, and closing it destroyed the forever-open override surface for every decision
      # at once - which the first live run did, to the thread it had just created. A per-decision
      # issue is still closed, because that issue is about that decision and nothing else.
      if gh issue view "$ISSUE_NUMBER" --json labels --jq '.labels[].name' | grep -qx 'keel-standing'; then
        echo "standing thread #$ISSUE_NUMBER left OPEN - it is the override surface for every auto-accepted decision"
      else
        gh issue close "$ISSUE_NUMBER" --reason completed --comment "In the tree. Nothing needs you unless you disagree."
      fi
      exit 0
    fi
    git fetch origin main && git merge --no-edit origin/main && "$KEEL" validate . && "$KEEL" guard
  done
  gh issue comment "$ISSUE_NUMBER" --body "Auto-accept FAILED after 3 push attempts; the sweeper or next session will complete it."
  exit 1
fi

# -- who / which decision / what they said / already done? ALL FROM THE BINARY (D0221) ----------
# This block used to be four sections of shell: a re-grep of the login table the binary already
# owns (two implementations of one rule - how the gate and the recorder drifted apart, D0219), a
# `grep -oE` for the decision marker, a `case` with a locale-dependent `tr` parsing the gesture, and
# a receipt scan. All of it is deterministic text work, so it belongs where it can be unit-tested.
# The comment body still arrives ONLY by env and is never interpolated into a shell command.
# The decider is resolved AFTER the decision is known (issue279) - see below. It cannot be resolved
# here, because which table authorises this login depends on which PROJECT the decision belongs to,
# and that is not known until the gesture is parsed and its id split.

issue_body=$(gh issue view "$ISSUE_NUMBER" --json body --jq .body)
comment_bodies=$(gh issue view "$ISSUE_NUMBER" --json comments --jq '.comments[].body')
gesture=$(COMMENT_BODY="$COMMENT_BODY" ISSUE_BODY="$issue_body" COMMENT_ID="$COMMENT_ID"           COMMENT_BODIES="$comment_bodies" "$KEEL" github-gesture) || {
  first_line=$(printf '%s' "$COMMENT_BODY" | head -1 | tr -d '
')
  gh issue comment "$ISSUE_NUMBER" --body "Didn't parse \`$first_line\` - reply with just the option letter (e.g. \`B\`), or \`accept\`, or \`reject <why>\`."
  exit 0
}
decision=$(printf '%s' "$gesture" | jq -r .decision)
verdict=$(printf '%s' "$gesture" | jq -r .verdict)
option=$(printf '%s' "$gesture" | jq -r .option)
reason=$(printf '%s' "$gesture" | jq -r .reason)
first_line=$(printf '%s' "$COMMENT_BODY" | head -1 | tr -d '
')

if [ -z "$decision" ]; then
  gh issue comment "$ISSUE_NUMBER" --body "This issue carries no keel-decision marker - nothing recorded."
  exit 0
fi

# WORKSPACE (D0234/issue279): split the channel id EXACTLY as the AUTO branch does. This branch did
# not, so a qualified id reached `keel accept` whole and was refused - or, once the gesture parser
# dropped the qualifier, was recorded against the ROOT project's tree instead. That is a HUMAN's
# judgment written against the wrong project, which is the one outcome this whole split exists to
# prevent, and it was the fork branch - the single decision class that genuinely needs a human.
split=$("$KEEL" github-decision-id "$decision")
proj=$(printf '%s' "$split" | cut -f1)
decision=$(printf '%s' "$split" | cut -f2)
if [ "$proj" != "." ]; then
  echo "recorder: decision belongs to project '$proj' - recording there"
  cd "$proj" || { echo "recorder: cannot enter '$proj'"; exit 1; }
  KEEL="$(cd "$OLDPWD" && pwd)/${KEEL#./}"
fi

# -- who may decide, resolved IN THE PROJECT the decision belongs to (issue279) -----------------
# The table lives in that project's .engine/contracts/github-actors.toml. Asking at the repository
# root in a workspace found no table and therefore authorised nobody; asking the ROOT project's table
# about another project's decision would be the mirror error - authorising against the wrong tree.
actor=$("$KEEL" github-decider "$COMMENT_USER" 2>/dev/null || true)
if [ -z "$actor" ]; then
  gh issue comment "$ISSUE_NUMBER" --body "GitHub login \`$COMMENT_USER\` is not a declared decider in ${proj}/.engine/contracts/github-actors.toml - nothing recorded (provenance is never defaulted)."
  exit 0
fi
if [ "$(printf '%s' "$gesture" | jq -r .alreadyReceipted)" = "true" ]; then
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
