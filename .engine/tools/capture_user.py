"""Capture the acting USER and register them in the project actor registry.

Source order (git is REQUIRED by this workflow, so it's canonical):
  1. `git config user.name` / `user.email`   (piggyback)
  2. a .env file at the repo root             (ENGINE_USER_NAME / ENGINE_USER_EMAIL)
  3. interactive prompt                        (only if a TTY is attached)
  -> else: exit non-zero with instructions.

Derives a stable slug id (first-initial + last name, lowercased), ensures it is
present in .tracking/actors.sysml's `ActorId` enum (appends if missing), and prints
the id to stdout so callers can use it for authoredBy / verifiedBy.

Usage:  python .engine/tools/capture_user.py
"""
import os
import re
import sys
import subprocess

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REGISTRY = os.path.join(REPO, ".tracking", "actors.sysml")


def git_identity():
    # `git var GIT_COMMITTER_IDENT` returns the EFFECTIVE identity git will use,
    # INCLUDING OS auto-detection when user.name/email aren't explicitly configured
    # (the common case — `git config user.name` is often empty). Format:
    #   "William Weatherholtz <0612@asirobots.com> 1718000000 -0600"
    try:
        r = subprocess.run(["git", "-C", REPO, "var", "GIT_COMMITTER_IDENT"],
                           capture_output=True, text=True)
        m = re.match(r"\s*(.*?)\s*<([^>]*)>", r.stdout)
        if m and m.group(1).strip():
            return m.group(1).strip(), m.group(2).strip()
    except Exception:
        pass
    # fallback: explicit config (rare, but honor it if set)
    try:
        name = subprocess.run(["git", "-C", REPO, "config", "user.name"],
                              capture_output=True, text=True).stdout.strip()
        email = subprocess.run(["git", "-C", REPO, "config", "user.email"],
                               capture_output=True, text=True).stdout.strip()
        if name:
            return name, email
    except Exception:
        pass
    return None, None


def env_identity():
    name = os.environ.get("ENGINE_USER_NAME")
    email = os.environ.get("ENGINE_USER_EMAIL", "")
    if not name:  # try a .env file at the repo root
        envf = os.path.join(REPO, ".env")
        if os.path.exists(envf):
            kv = {}
            with open(envf, encoding="utf-8") as fh:
                for line in fh:
                    m = re.match(r"\s*([A-Z_]+)\s*=\s*(.+)", line)
                    if m:
                        kv[m.group(1)] = m.group(2).strip().strip('"').strip("'")
            name = kv.get("ENGINE_USER_NAME")
            email = kv.get("ENGINE_USER_EMAIL", "")
    return (name, email) if name else (None, None)


def prompt_identity():
    if not sys.stdin.isatty():
        return None, None
    name = input("Acting user name: ").strip()
    email = input("Acting user email: ").strip()
    return (name, email) if name else (None, None)


def slug(name, email):
    parts = re.sub(r"[^A-Za-z ]", "", name or "").split()
    if len(parts) >= 2:
        s = (parts[0][0] + parts[-1]).lower()
    elif parts:
        s = parts[0].lower()
    else:
        s = re.sub(r"[^a-z0-9]", "", (email or "").split("@")[0].lower())
    return s or "unknown"


def registry_ids(text):
    body = re.search(r"enum def ActorId\s*\{(.*?)\}", text, re.DOTALL)
    if not body:
        return set()
    return set(re.findall(r"^\s*([A-Za-z]\w*)\s*;", body.group(1), re.MULTILINE))


def append_id(text, actor_id, name, email):
    line = f"        {actor_id};   // {name} <{email}> — human (captured from git)\n"
    # insert before the closing brace of the ActorId enum
    return re.sub(r"(\n)(\s*\}\s*\n\})", r"\1" + line + r"\2", text, count=1)


def main():
    for src, fn in (("git", git_identity), (".env", env_identity), ("prompt", prompt_identity)):
        name, email = fn()
        if name:
            break
    else:
        sys.stderr.write(
            "ERROR: could not determine the acting user.\n"
            "  Set git identity:  git config user.name / user.email\n"
            "  or a .env at repo root with ENGINE_USER_NAME / ENGINE_USER_EMAIL\n")
        sys.exit(1)

    actor_id = slug(name, email)
    with open(REGISTRY, encoding="utf-8") as fh:
        text = fh.read()
    ids = registry_ids(text)
    if actor_id in ids:
        sys.stderr.write(f"[capture_user] {actor_id} ({name}) already registered (via {src}).\n")
    else:
        new = append_id(text, actor_id, name, email)
        if new == text:
            sys.stderr.write("ERROR: could not locate the ActorId enum to append to.\n")
            sys.exit(2)
        with open(REGISTRY, "w", encoding="utf-8") as fh:
            fh.write(new)
        sys.stderr.write(f"[capture_user] registered new actor {actor_id} ({name}) via {src}.\n")

    print(actor_id)  # stdout: the captured actor id, for authoredBy / verifiedBy


if __name__ == "__main__":
    main()
