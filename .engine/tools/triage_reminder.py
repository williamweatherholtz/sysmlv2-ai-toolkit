#!/usr/bin/env python3
"""UserPromptSubmit hook (D0064 / triageRouterSkill): fire the engine-triage route-first
checklist on every turn, so routing is structural rather than vigilance. stdout is injected
into the model's context by Claude Code before it answers the user's prompt.

ASCII-only (Windows stdout is not UTF-8 — non-ASCII would inject as mojibake).

Wired in .claude/settings.json:
  "hooks": { "UserPromptSubmit": [ { "hooks": [
    { "type": "command", "command": "python .engine/tools/triage_reminder.py" } ] } ] }
"""
print(
    "[engine-triage -- route FIRST (D0064)] Break the request into parts and route EACH before "
    "acting: CHANGE (sec 3a: workflow/phase/gate/schema) | EXECUTE (sec 3b: tracked artifact, "
    "sprinted) | RECORD (sec 3c: one atomic fact -- decision/test result/issue) | VIEW (sec 3d: "
    "computed answer) | ORIENT (sec 3f: where things stand). Flag anything that does NOT cleanly "
    "map -- ask, don't force-fit. Substantive work goes through a sprint (only trivial one-off "
    "edits are exempt). method=confirmation needs explicit human sign-off. Invoke the "
    "engine-triage skill if unsure."
)
