"""Does dispatch ORDER move the guard set's wall clock? (dcGuardPoolDispatchesLongestFirst, issue455, D0388)

The guard pool (keel-cli/src/guards.rs run_in_parallel) is greedy list scheduling: W workers, each takes the
next guard in GUARD_NAMES order when it frees. The question is whether handing them out longest-first (LPT)
would shorten the makespan on the REAL durations. This script answers it by simulation, and per D0388 it
states its two known cases before it reads the real set:

  known-positive: six 1-unit guards then one 4-unit guard, two workers. Declaration order finishes the
                  short ones first and starts the long one at t=3 -> makespan 7; LPT starts it at t=0 and the other worker
                  absorbs the six short ones around it -> 5, the lower bound (sum 10 over 2 workers).
  known-negative: every guard 1 unit. Order cannot matter -> both makespans equal.

    python scripts/probes/guard_dispatch_order.py --probe
    python scripts/probes/guard_dispatch_order.py --real perf_run_1.txt [perf_run_2.txt ...] [--workers 20]

`--real` takes the output of `KEEL_PERF=2 keel gate guard --no-receipt .` (its `phase guard:<name> <n>ms` lines);
several files are averaged per guard. Declaration order is read from GUARD_NAMES in guards.rs so the
simulation dispatches exactly what the binary dispatches.

WHAT THE SIMULATION ASSUMES, AND WHAT MEASUREMENT FOUND (2026-09-10, sprint 657): the simulation treats
each guard's duration as fixed whatever runs beside it. On this set it said declaration order 1849 ms vs
longest-first 1314 ms ("ORDER MATTERS"). The binary, built with an env-gated longest-first dispatch and
timed in six INTERLEAVED pairs on one host state, measured wall medians 2567 vs 2544 ms - one percent,
inside the spread - because the longest guard (acceptance-binds-to-text, 1.6 s) stretched to 2.2 s when
dispatched first beside the other git-spawning, tree-walking guards, and `git x40` rose 5.2 -> 7.3 s. The
heavy guards contend for git and the filesystem, so the floor is that contention (dcGitReadsStayInProcess,
dcBatchGitReads), not the schedule. The pool stays in declaration order. Two earlier timings taken in
SEPARATE windows read 2.2 s and 2.9 s for identical code - host drift - which is why `--ab` below exists:
a scheduling claim about this pool is judged by interleaved pairs, never by two windows. Before any timed run,
sweep the host: `python .engine/tools/kill_stale_kernels.py` (13 stale kernel JVMs were alive during the two
windows above) and read the top CPU consumer - a run made under a scan is a run made on a different machine.

    python scripts/probes/guard_dispatch_order.py --ab ab_decl_*.txt --against ab_lpt_*.txt

`--ab` summarises two sets of timed runs (each file ends with a `run N wall <ms> ms` line, as the sprint 657
loop wrote them): wall median / min / max, guard-sum median, `git xN` median, and the critical path's range.
"""
import argparse
import heapq
import os
import re
import sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
GUARDS_RS = os.path.join(ROOT, "keel-cli", "src", "guards.rs")


def makespan(durations, workers):
    """Greedy list scheduling: each free worker takes the next task in list order."""
    if not durations:
        return 0
    heap = [0] * min(workers, len(durations))
    heapq.heapify(heap)
    for d in durations:
        t = heapq.heappop(heap)
        heapq.heappush(heap, t + d)
    return max(heap)


def compare(named, workers):
    """named: list of (name, ms) in declaration order. Returns (declared_ms, lpt_ms, lower_bound_ms)."""
    declared = makespan([d for _, d in named], workers)
    lpt = makespan(sorted((d for _, d in named), reverse=True), workers)
    bound = max(max(d for _, d in named), sum(d for _, d in named) / workers)
    return declared, lpt, bound


def probe():
    pos = [(f"short{i}", 1) for i in range(6)] + [("long", 4)]
    d, l, _ = compare(pos, 2)
    assert (d, l) == (7, 5), f"known-positive: expected declared 7 / LPT 5, got {d} / {l}"
    neg = [(f"g{i}", 1) for i in range(7)]
    d, l, _ = compare(neg, 2)
    assert d == l, f"known-negative: expected a tie, got {d} / {l}"
    print("probe: known-positive declared 7 -> LPT 5 (ordering matters); known-negative 4 == 4 (it cannot). Both hold.")
    # --ab's refusal: alternating mtimes are an A/B, two blocks are two windows.
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        def stamp(name, t):
            p = os.path.join(td, name)
            open(p, "w").write("run 1 wall 1 ms\n")
            os.utime(p, (t, t))
            return p
        alt_a = [stamp("a1", 100), stamp("a2", 300)]
        alt_b = [stamp("b1", 200), stamp("b2", 400)]
        assert interleaved(alt_a, alt_b), "known-positive: alternating mtimes must read as interleaved"
        blk_a = [stamp("c1", 100), stamp("c2", 200)]
        blk_b = [stamp("d1", 300), stamp("d2", 400)]
        assert not interleaved(blk_a, blk_b), "known-negative: two blocks must read as two windows"
    print("probe: --ab accepts alternating mtimes and refuses two blocks. Both hold.")


def guard_names():
    src = open(GUARDS_RS, encoding="utf-8").read()
    m = re.search(r"pub const GUARD_NAMES: \[&str; (\d+)\] =\s*\[(.*?)\];", src, re.S)
    if not m:
        sys.exit("GUARD_NAMES not found in guards.rs")
    names = re.findall(r'"([^"]+)"', m.group(2))
    assert len(names) == int(m.group(1)), (len(names), m.group(1))
    return names


def real(files, workers):
    seen = defaultdict(list)
    for f in files:
        for line in open(f, encoding="utf-8", errors="replace"):
            m = re.match(r"\s*phase guard:(\S+) (\d+)ms", line)
            if m:
                seen[m.group(1)].append(int(m.group(2)))
    names = guard_names()
    named = [(n, sum(seen[n]) / len(seen[n])) for n in names if seen[n]]
    missing = [n for n in names if not seen[n]]
    d, l, bound = compare(named, workers)
    print(f"real set: {len(named)} guards timed over {len(files)} run(s), {workers} workers"
          + (f"; {len(missing)} in GUARD_NAMES not timed (inactive or unphased): {', '.join(missing)}" if missing else ""))
    print(f"  sum {sum(x for _, x in named):.0f} ms, longest {max(named, key=lambda p: p[1])[0]} {max(x for _, x in named):.0f} ms, "
          f"lower bound {bound:.0f} ms")
    print(f"  simulated makespan: declaration order {d:.0f} ms, longest-first {l:.0f} ms, difference {d - l:.0f} ms")
    verdict = "ORDER MATTERS on this set" if d - l >= 0.05 * d else "ORDER DOES NOT MATTER on this set (within 5%)"
    print(f"  simulation says: {verdict} - IF durations were independent of what runs beside them.")
    print("  They were not on 2026-09-10 (see the module docstring): confirm with `--ab` on interleaved runs before acting.")
    return d, l


def summarise(files):
    import statistics
    walls, gits, sums, crit_names, crit_ms = [], [], [], set(), []
    for f in files:
        txt = open(f, encoding="utf-8", errors="replace").read()
        w = re.search(r"^run \d+ wall (\d+) ms", txt, re.M)
        if not w:
            sys.exit(f"{f}: no `run N wall <ms> ms` line")
        walls.append(int(w.group(1)))
        g = re.search(r"git x\d+ in (\d+)ms", txt)
        gits.append(int(g.group(1)) if g else 0)
        sums.append(sum(int(m) for m in re.findall(r"phase guard:\S+ (\d+)ms", txt)))
        c = re.search(r"critical path: (\S+) (\d+) ms", txt)
        if c:
            crit_names.add(c.group(1))
            crit_ms.append(int(c.group(2)))
    return {
        "n": len(walls), "wall_median": statistics.median(walls), "wall_min": min(walls), "wall_max": max(walls),
        "sum_median": statistics.median(sums), "git_median": statistics.median(gits),
        "crit": (sorted(crit_names), min(crit_ms) if crit_ms else 0, max(crit_ms) if crit_ms else 0),
    }


def interleaved(a_files, b_files):
    """True when, ordered by mtime, no two files of the same set are adjacent - the shape that shares one host
    state between the sets. Two blocks (all A, then all B) are two windows, and two windows measured 2.2 s and
    2.9 s for identical code on 2026-09-10."""
    order = sorted([(os.path.getmtime(f), "A") for f in a_files] + [(os.path.getmtime(f), "B") for f in b_files])
    return all(x[1] != y[1] for x, y in zip(order, order[1:]))


def ab(a_files, b_files):
    if not interleaved(a_files, b_files):
        sys.exit("REFUSED: the two sets do not interleave in time (by file mtime) - that is two windows, not an A/B; "
                 "alternate the runs and try again")
    for label, files in (("A", a_files), ("B", b_files)):
        s = summarise(files)
        print(f"{label}: n={s['n']} wall median {s['wall_median']:.0f} ms (min {s['wall_min']}, max {s['wall_max']}) | "
              f"guard sum median {s['sum_median']:.0f} ms | git median {s['git_median']:.0f} ms | "
              f"critical path {s['crit'][0]} {s['crit'][1]}-{s['crit'][2]} ms")
    a, b = summarise(a_files), summarise(b_files)
    spread = max(a["wall_max"] - a["wall_min"], b["wall_max"] - b["wall_min"])
    diff = a["wall_median"] - b["wall_median"]
    if abs(diff) <= spread:
        print(f"  wall medians differ by {diff:.0f} ms, inside the larger run-to-run spread ({spread} ms): NO MEASURED DIFFERENCE")
    else:
        print(f"  wall medians differ by {diff:.0f} ms, outside the run-to-run spread ({spread} ms): {'B' if diff > 0 else 'A'} is faster")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--probe", action="store_true", help="run the two known cases and exit")
    ap.add_argument("--real", nargs="+", metavar="PERF_TXT", help="KEEL_PERF=2 guard output file(s)")
    ap.add_argument("--workers", type=int, default=os.cpu_count() or 4)
    ap.add_argument("--ab", nargs="+", metavar="TIMED_TXT", help="set A of timed runs (declaration order)")
    ap.add_argument("--against", nargs="+", metavar="TIMED_TXT", help="set B of timed runs (the alternative)")
    a = ap.parse_args()
    if a.probe:
        probe()
        return
    if a.ab:
        if not a.against:
            sys.exit("--ab needs --against: two interleaved sets, or there is nothing to compare")
        ab(a.ab, a.against)
        return
    if a.real:
        probe()
        real(a.real, a.workers)
        return
    ap.print_help()


if __name__ == "__main__":
    main()
