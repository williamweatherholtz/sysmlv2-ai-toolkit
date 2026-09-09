#!/usr/bin/env python3
# ci-probe: --probe
# not-an-instrument: it holds the write discipline the instruments share; it measures nothing.
"""artefact - a run that cannot complete leaves no artefact that reads as a current answer (D0387, issue399).

The defect: the computed-facts script raised partway through a run and exited non-zero, and the PREVIOUS
run's JSON stayed on disk - valid, timestamped, with a tree SHA - indistinguishable from a current answer,
and the file the standing brief is built from. A non-zero exit is not enough: the exit code lives in the
shell and the artefact outlives it. Scenario S-F4 with a timestamp.

Two shapes, chosen by whether the artefact can carry its own outcome:

  * A JSON answer carries it. `begin_json(path)` writes `{"complete": false, "state": "running"}` in place
    of the previous answer BEFORE the run computes anything, and installs an excepthook that rewrites it as
    `{"complete": false, "state": "failed", "error": ...}` if the run raises; `finish_json(path, doc)` writes
    the answer with `"complete": true`. A killed run leaves the running stub; a failed one says it failed.
    Every reader goes through `require_complete(path)`, which refuses anything else - a file that says it
    failed is more useful than no file, PROVIDED every reader checks the field, so the check is the reader's
    only entry point.

  * A page or a drawing cannot. `claim(path)` removes the previous output before the run starts, so a run
    that dies leaves nothing to publish; the writer then writes atomically at the end.

Both write through a temp file and rename, so a reader never sees a half file.

    import sys; sys.path.insert(0, "scripts")
    from artefact import begin_json, finish_json, require_complete, claim

    python scripts/artefact.py --probe     # the known cases: killed, raised, finished, claimed
"""
from __future__ import annotations

import datetime as _dt
import io
import json
import os
import sys
import tempfile
import traceback


def _write_atomic(path: str, text: str) -> None:
    d = os.path.dirname(os.path.abspath(path)) or "."
    os.makedirs(d, exist_ok=True)
    fd, tmp = tempfile.mkstemp(prefix=".artefact-", dir=d)
    try:
        with io.open(fd, "w", encoding="utf-8", newline="\n") as f:
            f.write(text)
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise


def _now() -> str:
    return _dt.datetime.now().replace(microsecond=0).isoformat()


def begin_json(path: str, note: str = "") -> str:
    """Replace the previous answer with a running stub, and make an uncaught exception mark it failed."""
    started = _now()
    stub = {"complete": False, "state": "running", "startedAt": started, "note": note}
    _write_atomic(path, json.dumps(stub, indent=2) + "\n")
    previous_hook = sys.excepthook

    def failed_hook(exc_type, exc, tb):
        doc = {
            "complete": False,
            "state": "failed",
            "startedAt": started,
            "failedAt": _now(),
            "error": "".join(traceback.format_exception_only(exc_type, exc)).strip(),
            "note": note,
        }
        try:
            _write_atomic(path, json.dumps(doc, indent=2) + "\n")
        except OSError:
            pass
        previous_hook(exc_type, exc, tb)

    sys.excepthook = failed_hook
    return path


def finish_json(path: str, doc: dict) -> None:
    """The completed answer. `complete` is set here and nowhere else."""
    doc = dict(doc)
    doc["complete"] = True
    doc.pop("state", None)
    _write_atomic(path, json.dumps(doc, indent=2) + "\n")


def require_complete(path: str) -> dict:
    """The only way a reader gets the answer: refuses a running, failed, or pre-D0387 file."""
    try:
        with io.open(path, encoding="utf-8") as f:
            doc = json.load(f)
    except FileNotFoundError:
        sys.exit(f"artefact: {path} does not exist - the instrument has not run, or its run died and left nothing")
    except ValueError as e:
        sys.exit(f"artefact: {path} is not valid JSON ({e}) - a half-written answer is not an answer")
    if doc.get("complete") is not True:
        state = doc.get("state", "unknown (no `complete` field - written before D0387; re-run the instrument)")
        err = doc.get("error")
        sys.exit(f"artefact: {path} is not a completed answer - state {state!r}"
                 + (f", error: {err}" if err else "") + "; refusing to read it as current")
    return doc


def claim(path: str) -> str:
    """Remove the previous output before the run starts: a run that dies leaves nothing to take as current."""
    try:
        os.remove(path)
    except FileNotFoundError:
        pass
    return path


# ------------------------------------------------------------------------------------------- probe
_CHILD = r"""
import sys, os
sys.path.insert(0, sys.argv[1])
from artefact import begin_json, finish_json
p, mode = sys.argv[2], sys.argv[3]
begin_json(p, "probe child")
if mode == "killed":
    os._exit(9)            # SIGKILL's shape: no cleanup, no hook, nothing after this line runs
if mode == "raised":
    raise RuntimeError("the section after the anchor never ran")
finish_json(p, {"answer": 42})
"""


def probe() -> int:
    import shutil
    import subprocess

    failures = 0
    d = tempfile.mkdtemp(prefix="artefact-probe-")
    here = os.path.dirname(os.path.abspath(__file__))

    def case(name: str, ok: bool) -> None:
        nonlocal failures
        print(f"  {'pass' if ok else 'FAIL'}  {name}")
        if not ok:
            failures += 1

    def child(mode: str) -> tuple[int, dict | None]:
        p = os.path.join(d, f"{mode}.json")
        _write_atomic(p, json.dumps({"complete": True, "answer": "STALE", "generatedAt": "yesterday"}) + "\n")
        r = subprocess.run([sys.executable, "-c", _CHILD, here, p, mode], capture_output=True, text=True)
        try:
            with io.open(p, encoding="utf-8") as f:
                return r.returncode, json.load(f)
        except (OSError, ValueError):
            return r.returncode, None

    def refused(p: str) -> bool:
        try:
            require_complete(p)
        except SystemExit as e:
            return e.code not in (0, None)
        return False

    try:
        rc, doc = child("killed")
        case("a KILLED run leaves the running stub, not yesterday's answer", doc is not None and doc.get("complete") is False and doc.get("state") == "running" and "STALE" not in json.dumps(doc))
        case("...and a reader refuses it", refused(os.path.join(d, "killed.json")))

        rc, doc = child("raised")
        case("a run that RAISES exits non-zero", rc != 0)
        case("...and its artefact says failed, with the error", doc is not None and doc.get("state") == "failed" and "never ran" in doc.get("error", ""))
        case("...and a reader refuses it", refused(os.path.join(d, "raised.json")))

        rc, doc = child("finished")
        case("a FINISHED run writes complete: true and exits zero", rc == 0 and doc is not None and doc.get("complete") is True and doc.get("answer") == 42)
        case("...and a reader accepts it", require_complete(os.path.join(d, "finished.json")).get("answer") == 42)

        pre = os.path.join(d, "pre.json")
        _write_atomic(pre, json.dumps({"answer": 1}) + "\n")
        case("a pre-D0387 file with no `complete` field is refused", refused(pre))
        case("a missing file is refused", refused(os.path.join(d, "absent.json")))

        page = os.path.join(d, "page.html")
        _write_atomic(page, "<h1>yesterday</h1>")
        claim(page)
        case("claim() removes the previous page before the run", not os.path.exists(page))
        claim(page)
        case("claim() on nothing is not an error", True)
    finally:
        shutil.rmtree(d, ignore_errors=True)
    print(f"artefact probe: {failures} failure(s)")
    return failures


if __name__ == "__main__":
    if sys.argv[1:] == ["--probe"]:
        sys.exit(1 if probe() else 0)
    print(__doc__)
