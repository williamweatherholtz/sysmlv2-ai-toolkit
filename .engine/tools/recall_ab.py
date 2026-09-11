"""A/B: does injection put the answering element in front of the model, or not?

GROUND TRUTH IS FIXED HERE, BEFORE RUNNING, and none of these eight prompts is among the six the
confidence thresholds were tuned on (that overfitting risk is recorded in sprint 479's retro).

The measure is mechanical, not self-reported: for each question, is the element that ANSWERS it present
in the rows the payload actually SHOWS? Injection ON hands it over; injection OFF is the same prompt
with recall suppressed, where the count is necessarily zero and the model must go and search.

WHAT A HIT IS (D0408, option A; issue391 / dcRecallHitDefinitionIsStated). A hit is the ground-truth
record named in CASES appearing among the rows the payload shows - the named record, or nothing. A row
that answers the question without being that record is not a hit, and no row is scored by reading it:
the payload's job (D0161, D0365) is to put the GOVERNING record in front of the model, and "a row near
the answer" is what the 50-case bench (recall_bench.py) already measures.

THE BAR IS 7 OF 8, AND THE EIGHTH IS A DECLARED SENTINEL. The d0176 case ("what is an obligation and
what discharges one?") carries the UNREACHABLE marker in CASES, stated here before any run: d0176 is the
record that created the obligation fact - the files under .tracking/obligations/ name it in their first
line - yet its own text says "obligation" twice and "discharge" never, and no shown row is joined to it
by a typed edge, so no lexical or graph mechanism this harness measures can reach it. Its row prints
`unreachable (declared)` in place of `no`. The required bar is therefore 7 of 8: all seven reachable
cases hit. A hit on the sentinel, if a future mechanism produces one, is NEWS - it means something reads
meaning rather than words - and is printed as such rather than folded into the norm. The case stays in
the set for exactly that reason; it is not ground truth edited to fit what came back, and it is not
resolved by an alias I author (the lexicon is human-owned; D0408 option C is theirs to take).

THE BINARY IS NAMED (issue406). A run of this once printed 6/8 while cargo was still relinking the image
it was calling, and nothing in the output said so; the header now carries the build line of the binary
about to be interrogated, the footer re-reads it, and a run whose binary cannot be identified or changed
underneath it prints no score it stands behind (recall_binary.py).
"""
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import recall_binary  # noqa: E402

KEEL = recall_binary.keel_path()
BUDGET = "4000"

def _env():
    """A clean environment plus any KEEL_RECALL_* experiment knob the caller set (the mechanism is
    chosen on the 50-case set with these before it is hardcoded)."""
    import os
    env = {"PATH": "/usr/bin:/bin", "SYSTEMROOT": "C:\\Windows"}
    env.update({k: v for k, v in os.environ.items() if k.startswith("KEEL_RECALL_")})
    return env


# Third field: REACHABLE for a case the measured mechanism can reach; UNREACHABLE for a declared sentinel
# (D0408 option A) - stated here, before any run, never set after reading a result.
REACHABLE = None
UNREACHABLE = "unreachable (declared)"

CASES = [
    ("why must we never rebase or force-push?", "d0129", REACHABLE),
    ("what stops two contributors allocating the same decision number?", "d0129", REACHABLE),
    ("why is the schema frozen and what does changing it require?", "d0002", REACHABLE),
    ("what is an obligation and what discharges one?", "d0176", UNREACHABLE),
    ("who is allowed to decide on the github channel?", "d0219", REACHABLE),
    ("can several keel projects share one git repository?", "d0234", REACHABLE),
    ("how does a fresh downstream project number its first decision?", "d0237", REACHABLE),
    ("where is the knowledge store kept and is it removable?", "d0161", REACHABLE),
]
REQUIRED = sum(1 for _, _, marker in CASES if marker is REACHABLE)  # 7 of 8


def run(prompt, off=False):
    env = _env()
    env["KEEL_ACTOR"] = "claudeOpus5"
    if off:
        env["KEEL_RECALL"] = "off"
    payload = '{"prompt": %s}' % _json_str(prompt)
    t0 = time.time()
    out = subprocess.run([KEEL, "hook", "user-prompt"], input=payload,
                         capture_output=True, text=True, env=env)
    ms = int((time.time() - t0) * 1000)
    return out.stdout, ms


def _json_str(s):
    return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'


def shown_elements(text):
    """Element names on the '- Type name (...)' rows the payload actually printed."""
    names = []
    for line in text.splitlines():
        if line.startswith("- "):
            parts = line[2:].split()
            if len(parts) >= 2:
                names.append(parts[1])
    return names


def main():
    binary = recall_binary.Run(KEEL)
    binary.header()
    hits_on = hits_off = hits_reachable = hits_sentinel = 0
    rows = []
    for prompt, truth, marker in CASES:
        on, ms_on = run(prompt)
        off, _ = run(prompt, off=True)
        shown = shown_elements(on)
        hit_on = truth in shown
        hit_off = truth in shown_elements(off)
        hits_on += hit_on
        hits_off += hit_off
        if marker is REACHABLE:
            hits_reachable += hit_on
        else:
            hits_sentinel += hit_on
        pos = shown.index(truth) + 1 if hit_on else 0
        injected = "yes" if "[keel recall" in on else "NO (silent)"
        rows.append((prompt, truth, injected, len(shown), hit_on, pos, ms_on, hit_off, marker))

    hit_w = len(UNREACHABLE)
    print(f"{'question':52s} {'truth':9s} {'injected':11s} {'rows':4s} {'hit':{hit_w}s} {'pos':4s} {'ms':5s}")
    print("-" * (96 - 4 + hit_w))
    for prompt, truth, injected, n, hit, pos, ms, _, marker in rows:
        if hit:
            verdict = "YES" if marker is REACHABLE else "YES (sentinel hit - news)"
        else:
            verdict = "no" if marker is REACHABLE else marker
        print(f"{prompt[:52]:52s} {truth:9s} {injected:11s} {n:<4} "
              f"{verdict:{hit_w}s} {pos if pos else '-':<4} {ms:<5}")
    print("-" * (96 - 4 + hit_w))
    sentinels = [truth for _, truth, marker in CASES if marker is not REACHABLE]
    print(f"injection ON : answering element shown in {hits_on}/{len(CASES)}")
    print(f"required bar : {REQUIRED} of {len(CASES)} - the {REQUIRED} reachable cases; "
          f"{len(sentinels)} declared unreachable sentinel ({', '.join(sentinels)}) is not counted against it")
    print(f"bar          : {'MET' if hits_reachable >= REQUIRED else 'NOT MET'} "
          f"({hits_reachable}/{REQUIRED} reachable cases hit)")
    if hits_sentinel:
        print(f"NEWS         : {hits_sentinel} declared-unreachable case HIT - a mechanism reached a record no "
              f"lexical or graph path was known to reach; read the row, name the mechanism")
    print(f"injection OFF: answering element shown in {hits_off}/{len(CASES)}  (nothing is pushed)")
    silent = sum(1 for r in rows if r[2] != "yes")
    print(f"stayed silent (LOW confidence): {silent}/{len(CASES)}")
    avg = sum(r[6] for r in rows) / len(rows)
    print(f"mean prompt-path cost with recall: {avg:.0f}ms")
    binary.footer()


main()
