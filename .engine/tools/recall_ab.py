"""A/B: does injection put the answering element in front of the model, or not?

GROUND TRUTH IS FIXED HERE, BEFORE RUNNING, and none of these eight prompts is among the six the
confidence thresholds were tuned on (that overfitting risk is recorded in sprint 479's retro).

The measure is mechanical, not self-reported: for each question, is the element that ANSWERS it present
in the rows the payload actually SHOWS? Injection ON hands it over; injection OFF is the same prompt
with recall suppressed, where the count is necessarily zero and the model must go and search.
"""
import subprocess
import time

KEEL = "./target/release/keel.exe"
BUDGET = "4000"

CASES = [
    ("why must we never rebase or force-push?", "d0129"),
    ("what stops two contributors allocating the same decision number?", "d0129"),
    ("why is the schema frozen and what does changing it require?", "d0002"),
    ("what is an obligation and what discharges one?", "d0176"),
    ("who is allowed to decide on the github channel?", "d0219"),
    ("can several keel projects share one git repository?", "d0234"),
    ("how does a fresh downstream project number its first decision?", "d0237"),
    ("where is the knowledge store kept and is it removable?", "d0161"),
]


def run(prompt, off=False):
    env = {"PATH": "/usr/bin:/bin", "SYSTEMROOT": "C:\\Windows", "KEEL_ACTOR": "claudeOpus5"}
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
    hits_on = hits_off = 0
    rows = []
    for prompt, truth in CASES:
        on, ms_on = run(prompt)
        off, _ = run(prompt, off=True)
        shown = shown_elements(on)
        hit_on = truth in shown
        hit_off = truth in shown_elements(off)
        hits_on += hit_on
        hits_off += hit_off
        pos = shown.index(truth) + 1 if hit_on else 0
        injected = "yes" if "[keel recall" in on else "NO (silent)"
        rows.append((prompt, truth, injected, len(shown), hit_on, pos, ms_on, hit_off))

    print(f"{'question':52s} {'truth':9s} {'injected':11s} {'rows':4s} {'hit':4s} {'pos':4s} {'ms':5s}")
    print("-" * 96)
    for prompt, truth, injected, n, hit, pos, ms, _ in rows:
        print(f"{prompt[:52]:52s} {truth:9s} {injected:11s} {n:<4} "
              f"{'YES' if hit else 'no':4s} {pos if pos else '-':<4} {ms:<5}")
    print("-" * 96)
    print(f"injection ON : answering element shown in {hits_on}/{len(CASES)}")
    print(f"injection OFF: answering element shown in {hits_off}/{len(CASES)}  (nothing is pushed)")
    silent = sum(1 for r in rows if r[2] != "yes")
    print(f"stayed silent (LOW confidence): {silent}/{len(CASES)}")
    avg = sum(r[6] for r in rows) / len(rows)
    print(f"mean prompt-path cost with recall: {avg:.0f}ms")


main()
