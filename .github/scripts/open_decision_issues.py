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
"""
import json
import os
import subprocess


def gh(args, check=False):
    return subprocess.run(["gh", *args], capture_output=True, text=True, check=check)


def queue_auto(name: str, number: str, url: str) -> None:
    with open("auto_queue.txt", "a", encoding="utf-8") as f:
        f.write(name + "\t" + number + "\t" + url + "\n")


def main() -> None:
    cards = json.load(open("cards.json", encoding="utf-8"))["cards"]
    print(f"proposed decisions: {len(cards)}")
    for c in cards:
        search = f"keel-decision-{c['name']} in:body"
        q = gh(["issue", "list", "--state", "open", "--label", "decision",
                "--search", search, "--json", "number,url"])
        existing = json.loads(q.stdout or "[]")
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
            body += (
                "**Auto-accepted under your standing consent (D0207)** - nothing needs you. "
                "Reply `reject <why>` here anytime to reverse; this thread stays the override surface."
            )
        with open("body.md", "w", encoding="utf-8") as f:
            f.write(body)
        created = gh(["issue", "create", "--title", c["title"], "--body-file", "body.md",
                      "--label", "blocks-work", "--label", "decision",
                      "--assignee", os.environ["REPO_OWNER"]], check=True)
        url = created.stdout.strip().splitlines()[-1] if created.stdout.strip() else ""
        number = url.rsplit("/", 1)[-1] if url else ""
        print(f"opened issue for {c['name']}: {url}")
        if not c["options"] and number:
            queue_auto(c["name"], number, url)
            print(f"{c['name']} queued for auto-accept (new issue)")


if __name__ == "__main__":
    main()
