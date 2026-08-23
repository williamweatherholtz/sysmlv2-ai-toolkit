#!/usr/bin/env python3
"""open_decision_issues.py (D0205 githubChannel): one open GitHub issue per proposed keel decision.

Reads `keel decision-card --proposed` JSON from cards.json; the issue body is the decision's OWN
deciding context (one parser, never a second extraction that can drift); the title leads with the
short name (D0203); labels are exactly the board filter (D0204). Idempotent: the embedded
`keel-decision: <name>` marker is the identity, never the title.
"""
import json
import os
import subprocess


def main() -> None:
    cards = json.load(open("cards.json", encoding="utf-8"))["cards"]
    print(f"proposed decisions: {len(cards)}")
    for c in cards:
        q = subprocess.run(
            ["gh", "issue", "list", "--state", "open", "--label", "decision",
             "--search", f"keel-decision-{c['name']} in:body",
             "--json", "number", "--jq", "length"],
            capture_output=True, text=True, check=False,
        )
        if q.stdout.strip() not in ("", "0"):
            print(f"{c['name']} already has an open issue")
            continue
        body = (
            f"<!-- keel-decision: {c['name']} -->\n\n"
            f"**Why it came up:** {c['context']}\n\n"
            f"**What is being decided:** {c['decision']}\n\n---\n"
        )
        if c["options"]:
            body += (
                "**Reply with one letter to decide:**\n\n"
                + "\n".join(f"- **{o['key']}** - {o['label']}" for o in c["options"])
                + "\n\nOr reply `reject <why>`."
            )
        else:
            body += "**Reply `accept` to sign, or `reject <why>`.**"
        with open("body.md", "w", encoding="utf-8") as f:
            f.write(body)
        subprocess.run(
            ["gh", "issue", "create", "--title", c["title"], "--body-file", "body.md",
             "--label", "blocks-work", "--label", "decision",
             "--assignee", os.environ["REPO_OWNER"]],
            check=True,
        )
        print(f"opened issue for {c['name']}")


if __name__ == "__main__":
    main()
