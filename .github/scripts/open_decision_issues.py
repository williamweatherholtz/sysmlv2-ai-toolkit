#!/usr/bin/env python3
"""open_decision_issues.py (D0205 githubChannel + D0207 autoAccept).

One open GitHub issue per proposed keel decision. Reads `keel decision-card --proposed` JSON from
cards.json; the issue body is the decision's OWN deciding context (one parser, never a second
extraction that can drift); the title leads with the short name (D0203); labels are exactly the
board filter (D0204). Idempotent on the embedded `keel-decision: <name>` marker.

D0207 standing consent: a NON-FORK decision auto-accepts — new issues are queued to auto_queue.txt
(name<TAB>number<TAB>url) for the workflow's recorder step, and an already-open non-fork issue
sweeps into the same queue (the standing consent arriving after the issue did). Forks stay open
awaiting the human's letter.

D0227 (issue258), OPTION A, chosen by the human on a fork: the split is by ATTENTION, not by
decision. A NON-FORK is a COMMENT on ONE standing override thread that assigns nobody — it needs no
human by definition, so it must not spend anyone's attention to exist. A FORK opens its own issue and
is assigned, because it genuinely needs them, and assignment notifies through a mute of the standing
thread. Measured cause: 20 issues opened in a single day, each assigned, none of which needed anyone.
This reverses the earlier "one issue per decision" requirement, which is why it was asked rather than
fixed — and the cost is that a gesture must now NAME the decision it reverses (`reject dNNNN why`).
"""
import json
import re
import subprocess


def gh(args, check=False):
    return subprocess.run(["gh", *args], capture_output=True, text=True, check=check)


def decider_logins():
    """The declared deciders, from the committed table via the binary (D0219).

    REPO_OWNER was WRONG, not merely inflexible: `github.repository_owner` is not a person. For an
    org-owned repo like asirobots/penumbra it is the ORG, and `gh issue create --assignee <org>`
    fails outright - the mechanism would break on the first adoption rather than degrade. Assignment
    is a convenience; the `decision` label is what makes an issue findable, so with no declared
    decider we assign nobody and still open the issue.
    """
    out = subprocess.run(["./target/release/keel", "github-decider"], capture_output=True, text=True, check=False)
    return [l.split("	")[0] for l in out.stdout.splitlines() if l.strip()]


def assignee_args():
    return [a for login in decider_logins() for a in ("--assignee", login)]


def queue_auto(name: str, number: str, url: str) -> None:
    with open("auto_queue.txt", "a", encoding="utf-8") as f:
        f.write(name + "\t" + number + "\t" + url + "\n")


# D0205's number: decisions from here on are governed by the GitHub channel and must each have an
# override-thread issue, however they were accepted (issue238: a local-console/API accept bypassed
# the channel and left D0207/D0209 with no override surface). The guarantee is enforced here.
CHANNEL_FROM = 205


def ensure_override_threads() -> None:
    """issue238 control: every ACCEPTED decision numbered >= CHANNEL_FROM has a GitHub override
    thread, regardless of acceptance path. A local/API accept that skipped the channel gets its
    thread opened-and-closed here, so the override surface always exists (D0205/D0207)."""
    allcards = json.loads(_all_cards() or '{"cards":[]}').get("cards", [])
    for c in allcards:
        # A TRUE decision card is named d + exactly four digits (d0205); acceptance/defer record
        # cards carry a suffix (d0205AcceptR1) and are NOT decisions - skip them.
        m = re.fullmatch(r"d(\d{4})", c["name"])
        if not m:
            continue
        num = int(m.group(1))
        if c["status"] != "accepted" or num < CHANNEL_FROM:
            continue
        if has_override_surface(c["name"]):
            continue  # already has a surface: its own issue, or a comment on the standing thread
        body = (
            f"### {c['title']}\n\n"
            f"<!-- keel-decision: {c['name']} -->\n\n"
            f"**Why it came up:** {c['context']}\n\n"
            f"**What is being decided:** {c['decision']}\n\n---\n"
            "**Status: ACCEPTED** before this override thread existed (a channel-bypass, issue238). "
            "This issue is your override surface: reply `reject <why>` here anytime to reverse."
        )
        # D0227: the backfill posts to the standing thread too. It used to OPEN AND CLOSE one issue
        # per bypassed decision - two notifications each, for a surface nobody had asked for.
        if post_to_standing(c, body):
            print(f"{c['name']} override surface backfilled onto the standing thread")


STANDING_MARK = "<!-- keel-standing-thread -->"
STANDING_TITLE = "keel: decisions auto-accepted under standing consent"
STANDING_LABEL = "keel-standing"
_STANDING_CACHE = {}


def repo_url() -> str:
    return gh(["repo", "view", "--json", "url", "-q", ".url"]).stdout.strip()


def standing_thread() -> str:
    """The one open standing thread's number, creating it if this repo has none yet.

    FOUND BY LABEL, NOT BY SEARCH (issue262). The first live run of D0227 opened the thread TWICE,
    #28 and #29, because two lookups in one run each missed the other's creation: `--search` reads
    the GitHub SEARCH INDEX, which lags creation by seconds. A label filter is served from the API
    directly and is immediate. The in-process cache closes the same race for the remaining window.
    A stubbed dry run could not have caught this - it returned an existing thread instantly, so the
    create-then-look-again path never ran. Live verification is what found it.
    """
    if "n" in _STANDING_CACHE:
        return _STANDING_CACHE["n"]
    found = json.loads(gh(["issue", "list", "--state", "open", "--label", STANDING_LABEL,
                           "--json", "number"]).stdout or "[]")
    if found:
        n = str(min(int(i["number"]) for i in found))  # oldest wins if a race ever leaves two
        _STANDING_CACHE["n"] = n
        return n
    body = (
        STANDING_MARK + "\n\n"
        "Every Decision auto-accepted under your standing consent (D0207) is posted here as a comment "
        "rather than as its own issue, so this thread is the override surface for all of them and "
        "**nothing here needs you**.\n\n"
        "To reverse one, reply `reject dNNNN <why>` naming the decision.\n\n"
        "Nobody is assigned, so you can mute this thread. A decision that genuinely needs you is a "
        "FORK: it opens its own issue and assigns you, which notifies through a mute."
    )
    with open("body.md", "w", encoding="utf-8") as f:
        f.write(body)
    created = gh(["issue", "create", "--title", STANDING_TITLE, "--body-file", "body.md",
                  "--label", "decision", "--label", STANDING_LABEL], check=True)
    url = created.stdout.strip().splitlines()[-1] if created.stdout.strip() else ""
    print("opened the standing override thread: " + url)
    number = url.rsplit("/", 1)[-1] if url else ""
    if number:
        _STANDING_CACHE["n"] = number
    return number


def standing_has(name: str) -> bool:
    """Is this decision already on the standing thread? Idempotency for a re-run or the sweeper."""
    number = standing_thread()
    if not number:
        return False
    bodies = gh(["issue", "view", number, "--json", "comments", "-q", ".comments[].body"]).stdout
    return ("keel-decision: " + name) in bodies


def post_to_standing(card, body: str) -> str:
    """Append one auto-accepted decision to the standing thread. Returns its issue number."""
    number = standing_thread()
    if not number:
        return ""
    with open("body.md", "w", encoding="utf-8") as f:
        f.write(body)
    gh(["issue", "comment", number, "--body-file", "body.md"], check=True)
    print("posted " + card["name"] + " to the standing thread #" + number)
    return number


def has_override_surface(name: str) -> bool:
    """Does this decision have an override surface anywhere — its own issue, or the standing thread?"""
    if json.loads(gh(["issue", "list", "--state", "all", "--label", "decision",
                      "--search", "keel-decision-" + name + " in:body",
                      "--json", "number"]).stdout or "[]"):
        return True
    return standing_has(name)


def _all_cards() -> str:
    return subprocess.run(["./target/release/keel", "decision-card"],
                          capture_output=True, text=True, check=False).stdout


def main() -> None:
    cards = json.load(open("cards.json", encoding="utf-8"))["cards"]
    print(f"proposed decisions: {len(cards)}")
    for c in cards:
        search = f"keel-decision-{c['name']} in:body"
        q = gh(["issue", "list", "--state", "open", "--label", "decision",
                "--search", search, "--json", "number,url"])
        existing = json.loads(q.stdout or "[]")
        if not c["options"] and standing_has(c["name"]):
            print(f"{c['name']} already on the standing thread")
            continue
        if existing and c["options"]:
            print(f"{c['name']} already has an open issue (fork - awaiting their letter)")
            continue
        if existing:
            row = existing[0]
            queue_auto(c["name"], str(row["number"]), row["url"])
            print(f"{c['name']} queued for auto-accept (existing issue #{row['number']})")
            continue
        research = c.get("research", "").strip()
        body = (
            f"<!-- keel-decision: {c['name']} -->\n\n"
            f"**Why it came up:** {c['context']}\n\n"
            f"**What is being decided:** {c['decision']}\n\n"
            + (f"**Research:** {research}\n\n" if research else "")
            + "---\n"
        )
        if c["options"]:
            body += (
                "**Reply with one letter to decide:**\n\n"
                + "\n".join(f"- **{o['key']}** - {o['label']}" for o in c["options"])
                + "\n\nOr reply `reject <why>`."
            )
        else:
            body = f"### {c['title']}\n\n" + body + (
                "**Auto-accepted under your standing consent (D0207)** - nothing needs you. "
                f"Reply `reject {c['name']} <why>` on this thread anytime to reverse it. "
                "Naming the decision is what lets one thread carry them all (D0227)."
            )
        if not c["options"]:
            # D0227 OPTION A: a comment on the shared standing thread, assigning NOBODY. The title
            # rides in the comment body because a comment has none of its own.
            number = post_to_standing(c, body)
            if number:
                queue_auto(c["name"], number, f"{repo_url()}/issues/{number}")
                print(f"{c['name']} queued for auto-accept (standing thread #{number})")
            continue
        with open("body.md", "w", encoding="utf-8") as f:
            f.write(body)
        created = gh(["issue", "create", "--title", c["title"], "--body-file", "body.md",
                      "--label", "blocks-work", "--label", "decision",
                      *assignee_args()], check=True)
        url = created.stdout.strip().splitlines()[-1] if created.stdout.strip() else ""
        print(f"opened FORK issue for {c['name']}: {url}")
    ensure_override_threads()  # issue238: guarantee an override thread for every post-channel accepted decision


if __name__ == "__main__":
    main()
