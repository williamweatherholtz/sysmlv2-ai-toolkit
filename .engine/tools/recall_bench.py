"""50-case retrieval benchmark: recall (KG) vs naive keyword search (what a model does instead).

WHY THE CONTROL IS GREP, NOT SILENCE. Scoring "without KG" as zero would be rigging the comparison: a
model without recall does not sit in the dark, it searches. So the baseline is the thing it would
actually do — take the most distinctive word in the request and look for it — and the question is
whether traversal beats that.

WHY THE QUESTIONS ARE DERIVED, NOT WRITTEN. Fifty hand-written questions would let the author bias the
set toward what recall happens to find. Each query here is built MECHANICALLY from the target element's
BODY text (its decision/description/rationale), with any word that appears in the element's own NAME
removed. That simulates the real case: someone describes a problem in their own words and the store has
to find the record. Sampling is seeded, so the set is reproducible.

The measure is mechanical: is the target element present in the rows the payload actually SHOWS?
"""
import random
import re
import subprocess
import time
from pathlib import Path

KEEL = "./target/release/keel.exe"
SEED = 20260828
N_CASES = 50
QUERY_WORDS = 14
BASELINE_K = 20  # a naive searcher scans this many hits before giving up

BODY_FIELDS = ("decision", "description", "rationale", "actionText", "consequences")
SCAN_DIRS = (".engine/decisions", ".tracking")


def load_elements():
    """(name, title, body, file) for every element carrying a title and a substantive body field."""
    out = {}
    for d in SCAN_DIRS:
        for p in Path(d).rglob("*.sysml"):
            text = p.read_text(encoding="utf-8", errors="replace")
            # One element per `part <name> : <Type> {`, taking the fields that follow it.
            for m in re.finditer(r"part\s+([A-Za-z][A-Za-z0-9_]*)\s*:\s*([A-Za-z][A-Za-z0-9_]*)\s*\{", text):
                name, typ = m.group(1), m.group(2)
                chunk = text[m.end(): m.end() + 6000]
                title = re.search(r':>>\s*title\s*=\s*"(.*?)"', chunk, re.S)
                body = None
                for f in BODY_FIELDS:
                    b = re.search(r':>>\s*' + f + r'\s*=\s*"(.*?)"', chunk, re.S)
                    if b and len(b.group(1)) > 200:
                        body = b.group(1)
                        break
                if title and body and name not in out:
                    out[name] = (title.group(1), body, str(p).replace("\\", "/"), typ)
    return out


def name_words(name):
    """Lowercase segments of a camelCase / dNNNN element name — removed from the query so the test is
    not a trivial string match against the thing being searched for."""
    parts = re.findall(r"[A-Z]?[a-z0-9]+", name)
    return {p.lower() for p in parts}


STOP = set("""the a an and or of to in for on with that this it is are was were be been being as by
from at into not no any all can could should would must may might will shall do does did done have
has had having if then than so such but which who whom whose what when where why how there here
their them they we you your our its his her about above after again against already also although
always another because before below between both cannot during each either every further just made
make making many more most much only other some thing things stuff""".split())


def build_query(name, body):
    words = [w for w in re.findall(r"[A-Za-z][A-Za-z-]{2,}", body)]
    skip = name_words(name)
    picked = []
    for w in words:
        lw = w.lower()
        if lw in STOP or lw in skip or lw in {p.lower() for p in picked}:
            continue
        picked.append(w)
        if len(picked) >= QUERY_WORDS:
            break
    return " ".join(picked)


def recall(query):
    t0 = time.time()
    r = subprocess.run([KEEL, "recall", "--prompt", "-", "--budget", "4000"],
                       input=query, capture_output=True, text=True,
                       env={"PATH": "/usr/bin:/bin", "SYSTEMROOT": "C:\\Windows"})
    ms = int((time.time() - t0) * 1000)
    shown = []
    for line in r.stdout.splitlines():
        if line.startswith("- "):
            parts = line[2:].split()
            if len(parts) >= 2:
                shown.append(parts[1])
    pushed = "recall: no informative term" not in r.stdout and bool(shown)
    return shown, ms, pushed


def baseline(query, elements):
    """NAIVE KEYWORD SEARCH, the control: take the query's rarest word (the one a searcher would pick
    as most distinctive) and scan the first BASELINE_K elements whose name or title contains it."""
    toks = [w.lower() for w in query.split() if len(w) >= 5]
    if not toks:
        return []
    freq = {}
    for t in toks:
        freq[t] = sum(1 for (n, (ti, _b, _f, _ty)) in elements.items()
                      if t in n.lower() or t in ti.lower())
    ranked = sorted((f, t) for t, f in freq.items() if f > 0)
    if not ranked:
        return []
    best = ranked[0][1]
    hits = [n for (n, (ti, _b, _f, _ty)) in sorted(elements.items())
            if best in n.lower() or best in ti.lower()]
    return hits[:BASELINE_K]


def main():
    elements = load_elements()
    names = sorted(elements)
    random.Random(SEED).shuffle(names)
    cases = names[:N_CASES]
    print(f"corpus: {len(elements)} elements with a title and a substantive body; sampling {len(cases)}")
    print(f"query  = {QUERY_WORDS} content words from the element's BODY, its own name-words removed")
    print(f"control= naive keyword search on the query's rarest word, first {BASELINE_K} hits\n")

    kg_hit = base_hit = pushed_n = 0
    kg_pos, ms_all = [], []
    misses = []
    for i, name in enumerate(cases, 1):
        title, body, _f, typ = elements[name]
        q = build_query(name, body)
        shown, ms, pushed = recall(q)
        b = baseline(q, elements)
        h_kg = name in shown
        h_b = name in b
        kg_hit += h_kg
        base_hit += h_b
        pushed_n += pushed
        ms_all.append(ms)
        if h_kg:
            kg_pos.append(shown.index(name) + 1)
        else:
            misses.append((name, typ, len(shown), h_b))
        print(f"{i:3d} {name[:34]:34s} {typ[:11]:11s} kg={'Y' if h_kg else '.'} "
              f"grep={'Y' if h_b else '.'} rows={len(shown):<3} {ms}ms")

    n = len(cases)
    print("\n" + "=" * 74)
    print(f"KG recall (traversal)      : {kg_hit}/{n} = {100*kg_hit/n:.0f}%")
    print(f"naive keyword search       : {base_hit}/{n} = {100*base_hit/n:.0f}%")
    print(f"pushed at all (informative): {pushed_n}/{n}")
    if kg_pos:
        kg_pos.sort()
        top3 = sum(1 for p in kg_pos if p <= 3)
        print(f"KG hit position            : median {kg_pos[len(kg_pos)//2]}, top-3 in {top3}/{kg_hit}")
    print(f"latency                    : mean {sum(ms_all)//len(ms_all)}ms, max {max(ms_all)}ms")
    both = sum(1 for name in cases if name in misses)
    print(f"KG missed but grep found   : {sum(1 for m in misses if m[3])}")
    print(f"neither found              : {sum(1 for m in misses if not m[3])}")


main()
