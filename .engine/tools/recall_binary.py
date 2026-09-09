# not-an-instrument: names the binary the two recall Sensors interrogate (header/footer identity, issue406); it measures nothing about the model.
"""The binary a recall measurement interrogates, named before the first case and checked after the last
(issue406 / dcRecallMeasurementNamesItsBinary).

WHY. recall_ab.py was run while cargo was still relinking target/release/keel.exe and printed 6/8 at
5282 ms; three runs against the settled image read 7/8 at ~1050 ms, and nothing in the 6/8 output said
which binary answered. The number was wrong and looked exactly like a regression of the function that
had just been refactored. This is the issue386 family - an instrument reporting over a file it does not
control - in the measurement harnesses: a number that cannot name the image that produced it is not
attributable, and an unattributable number on a Decision is worse than none.

WHAT. `identify(path)` runs `<path> version` and returns the build line (`keel 0.4.1 build df7a54e+dirty`)
or raises `BinaryUnavailable` naming the reason - the file is absent, the process could not start, it
exited non-zero, or it printed no `build commit:` line. `Run` wraps a harness: `header()` prints the line
before the first case; `footer()` re-reads the binary after the last case and, if the build changed
under the run, says so and makes the process exit non-zero - the mid-relink case is the one the header
alone cannot see. Neither harness is on a gate; this is legibility, not blocking: a stale or mid-relink
measurement is visible in its own output instead of indistinguishable from a settled one.

`KEEL_BENCH_BIN` overrides the path both harnesses interrogate, so a run against a deliberately absent
binary is one environment variable away. `--probe` runs the two cases known before anything is measured
(D0388): the real binary yields a build line; a path that does not exist raises with its reason.
"""
import os
import re
import subprocess
import sys

DEFAULT_KEEL = "./target/release/keel.exe"


class BinaryUnavailable(RuntimeError):
    """The binary could not be interrogated; the message names why."""


def keel_path():
    return os.environ.get("KEEL_BENCH_BIN", DEFAULT_KEEL)


def identify(path):
    """`<path> version` -> 'keel <ver> build <commit[+dirty]>', or BinaryUnavailable naming the reason."""
    if not os.path.exists(path):
        raise BinaryUnavailable(f"binary absent: {path} does not exist")
    try:
        out = subprocess.run([path, "version"], capture_output=True, text=True, timeout=60,
                             env={"PATH": "/usr/bin:/bin", "SYSTEMROOT": "C:\\Windows"})
    except OSError as e:
        raise BinaryUnavailable(f"binary could not be started: {path}: {e}") from e
    except subprocess.TimeoutExpired as e:
        raise BinaryUnavailable(f"binary did not answer `version` within 60 s: {path}") from e
    if out.returncode != 0:
        raise BinaryUnavailable(f"`{path} version` exited {out.returncode}: {(out.stderr or out.stdout).strip()[:200]}")
    ver = re.search(r"^keel\s+(\S+)", out.stdout, re.M)
    build = re.search(r"^build commit:\s*(\S+)", out.stdout, re.M)
    if not build:
        raise BinaryUnavailable(f"`{path} version` printed no `build commit:` line: {out.stdout.strip()[:200]!r}")
    return f"keel {ver.group(1) if ver else '?'} build {build.group(1)}"


class Run:
    """Name the binary before the first case, check it after the last."""

    def __init__(self, path=None):
        self.path = path or keel_path()
        self.before = None

    def header(self):
        """Print the build line, or abort with the reason and exit 2 - never print a score over nothing."""
        try:
            self.before = identify(self.path)
        except BinaryUnavailable as e:
            print(f"ABORT: no measurement - {e}", file=sys.stderr)
            sys.exit(2)
        print(f"binary : {self.path} = {self.before}")
        return self.before

    def footer(self):
        """Re-read the binary. Unchanged: one line saying so. Changed: the numbers above are disowned and
        the process exits 3, because the image that produced them is not the one named in the header."""
        try:
            after = identify(self.path)
        except BinaryUnavailable as e:
            print(f"binary : NOT ATTRIBUTABLE - the binary could not be re-read after the last case ({e}); "
                  f"the numbers above are not tied to an image")
            sys.exit(3)
        if after == self.before:
            print(f"binary : unchanged through the run ({after})")
            return True
        print(f"binary : CHANGED DURING THE RUN - {self.before} before the first case, {after} after the last; "
              f"the numbers above are not attributable to either image. Re-run against a settled binary.")
        sys.exit(3)


def probe():
    checks = []
    try:
        line = identify(DEFAULT_KEEL)
        checks.append(("known-positive: the real binary yields a build line", line.startswith("keel ") and " build " in line, line))
    except BinaryUnavailable as e:
        checks.append(("known-positive: the real binary yields a build line", False, str(e)))
    absent = "./target/release/no-such-keel.exe"
    try:
        identify(absent)
        checks.append(("known-negative: an absent path raises naming the reason", False, "no exception"))
    except BinaryUnavailable as e:
        checks.append(("known-negative: an absent path raises naming the reason", "binary absent" in str(e), str(e)))
    ok = True
    for what, held, detail in checks:
        print(f"[probe] {'PASS' if held else 'FAIL'} {what}\n        -> {detail}")
        ok &= held
    print(f"[probe] {'both cases hold' if ok else 'A CASE FAILED - the build line is not trusted'}")
    return ok


if __name__ == "__main__":
    if "--probe" in sys.argv[1:]:
        sys.exit(0 if probe() else 1)
    print(identify(keel_path()))
