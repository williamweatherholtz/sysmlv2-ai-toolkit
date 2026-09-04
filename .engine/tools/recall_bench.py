"""50-case retrieval benchmark: recall (KG traversal) against a naive keyword search, on queries the
target did NOT write (issue296 / D0161).

WHY THE CONTROL IS GREP, NOT SILENCE. Scoring "without KG" as zero would be rigging the comparison: a
model without recall does not sit in the dark, it searches. So the baseline is the thing it would
actually do - take the most distinctive word in the request and look for it - and the question is
whether traversal beats that.

WHY THE CONTROL SEARCHES BODIES. The first harness matched name and title only while its queries were
drawn from BODY text, so the control was structurally denied the one field the queries came from - a
strawman (issue296 defect 1). Both arms now search the same fields.

WHY THE QUERIES ARE A NEIGHBOUR'S WORDS, NOT THE TARGET'S. A query built from the target's own body is
leakage: once the retriever indexes bodies, any body-searching arm scores ~100% (measured: 50/50) and
the harness cannot discriminate. The realistic case is someone describing a problem in THEIR words and
the store having to find the record that answers it. The nearest mechanical stand-in is the text of a
record LINKED to the target by a typed edge - the Issue a task resolves, the Decision that chartered a
sprint, the story a need derives from - written by a different record for a different purpose. Words
naming either record are removed so the test is never a string match on the thing searched for.
Sampling is seeded, so the set is reproducible. `--self` runs the old leakage set, labelled as such.

WHAT IS MEASURED. hit: the target is among the rows the payload actually SHOWS (budget 4000). precision:
of the rows shown, the share that are the target or its one-hop neighbours - the only mechanical
relevance available without judging each row by hand; it is a floor, since a shown row can be relevant
without being adjacent. The 2x2 and McNemar's exact test are printed because a difference in hit counts
is not a result until it is one (issue296 defect 3). The corpus asymmetry is printed because the two
arms do not search the same number of things.
"""
import argparse
import math
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
MIN_BODY = 120
BODY_FIELDS = ("decision", "description", "rationale", "actionText", "consequences", "procedureText")
SCAN_DIRS = (".engine/decisions", ".tracking")
EDGE = re.compile(r"(?:#\w+\s+)?dependency\s+from\s+(\w+)\s+to\s+(\w+)\s*;")
EDGE2 = re.compile(r"\b(?:satisfy|verify|allocate)\s+(\w+)\s+by\s+(\w+)\s*;")


def block_of(text, start):
    """The text of the block whose opening brace is at/after `start`, up to and including its matching
    close - string-aware, so a brace inside a quoted field is text, not structure (the D0295 lesson: a
    fixed 6000-char window handed a one-line TestResult the NEXT element's title and body, so both arms
    of the first harness were measured against the wrong text)."""
    depth, in_str, i = 0, False, start
    while i < len(text):
        c = text[i]
        if in_str:
            if c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return text[start:i + 1]
        i += 1
    return text[start:]


def load_elements():
    """name -> (title, body-or-empty, file, type) for every element with a title; and the edge list."""
    out, edges = {}, []
    for d in SCAN_DIRS:
        for p in Path(d).rglob("*.sysml"):
            text = p.read_text(encoding="utf-8", errors="replace")
            for m in EDGE.finditer(text):
                edges.append((m.group(1), m.group(2)))
            for m in EDGE2.finditer(text):
                edges.append((m.group(1), m.group(2)))
            for m in re.finditer(r"\b(?:part|verification|item|requirement|action)\s+(\w+)\s*:\s*(\w+)\s*\{", text):
                name, typ = m.group(1), m.group(2)
                chunk = block_of(text, m.end() - 1)
                title = re.search(r':>>\s*title\s*=\s*"(.*?)"', chunk, re.S)
                body = ""
                for f in BODY_FIELDS:
                    b = re.search(r':>>\s*' + f + r'\s*=\s*"(.*?)"', chunk, re.S)
                    if b and len(b.group(1)) > len(body):
                        body = b.group(1)
                if title and name not in out:
                    out[name] = (title.group(1), body, str(p).replace("\\", "/"), typ)
    return out, edges


def name_words(name):
    parts = re.findall(r"[A-Z]?[a-z0-9]+", name)
    return {p.lower() for p in parts}


STOP = set("""the a an and or of to in for on with that this it is are was were be been being as by
from at into not no any all can could should would must may might will shall do does did done have
has had having if then than so such but which who whom whose what when where why how there here
their them they we you your our its his her about above after again against already also although
always another because before below between both cannot during each either every further just made
make making many more most much only other some thing things stuff""".split())


def build_query(source_body, exclude):
    words = re.findall(r"[A-Za-z][A-Za-z-]{2,}", source_body)
    picked = []
    for w in words:
        lw = w.lower()
        if lw in STOP or lw in exclude or lw in {p.lower() for p in picked}:
            continue
        picked.append(w)
        if len(picked) >= QUERY_WORDS:
            break
    return " ".join(picked)


def neighbours(edges):
    adj = {}
    for a, b in edges:
        adj.setdefault(a, set()).add(b)
        adj.setdefault(b, set()).add(a)
    return adj


def recall(query):
    t0 = time.time()
    r = subprocess.run([KEEL, "recall", "--prompt", "-", "--budget", "4000"],
                       input=query, capture_output=True, text=True,
                       env={"PATH": "/usr/bin:/bin", "SYSTEMROOT": "C:\\Windows"})
    ms = int((time.time() - t0) * 1000)
    shown, corpus = [], None
    for line in r.stdout.splitlines():
        if line.startswith("- "):
            parts = line[2:].split()
            if len(parts) >= 2:
                shown.append(parts[1])
        m = re.search(r"corpus (\d+) items", line)
        if m:
            corpus = int(m.group(1))
    return shown, ms, corpus


def baseline(query, elements):
    """NAIVE KEYWORD SEARCH, the control: the query's rarest word, scanned over NAME, TITLE AND BODY -
    the same fields the retriever reads - first BASELINE_K hits by name."""
    toks = [w.lower() for w in query.split() if len(w) >= 5]
    if not toks:
        return []

    def hay(n, ti, b):
        return (n + " " + ti + " " + b).lower()

    freq = {t: sum(1 for (n, (ti, b, _f, _ty)) in elements.items() if t in hay(n, ti, b)) for t in toks}
    ranked = sorted((f, t) for t, f in freq.items() if f > 0)
    if not ranked:
        return []
    best = ranked[0][1]
    return [n for (n, (ti, b, _f, _ty)) in sorted(elements.items()) if best in hay(n, ti, b)][:BASELINE_K]


def mcnemar_exact(b, c):
    """Two-sided exact McNemar on the discordant pairs: KG-only b, control-only c."""
    n = b + c
    if n == 0:
        return 1.0
    k = min(b, c)
    tail = sum(math.comb(n, i) for i in range(0, k + 1)) / 2 ** n
    return min(1.0, 2 * tail)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--self", action="store_true", help="the old LEAKAGE set: queries from the target's own body")
    ap.add_argument("--cases", type=int, default=N_CASES)
    args = ap.parse_args()

    elements, edges = load_elements()
    adj = neighbours(edges)
    with_body = {n for n, (_t, b, _f, _ty) in elements.items() if len(b) >= MIN_BODY}
    # a case: a target with a body (so the leakage set is comparable) and, for the neighbour set, at
    # least one linked record with a body of its own
    candidates = sorted(n for n in with_body if args.self or any(m in with_body for m in adj.get(n, ())))
    random.Random(SEED).shuffle(candidates)
    cases = candidates[:args.cases]
    label = "SELF (LEAKAGE - the target's own body words)" if args.self else "NEIGHBOUR (a linked record's body words)"
    print(f"corpus: {len(elements)} elements with a title ({len(with_body)} with a body >= {MIN_BODY} chars); "
          f"{len(edges)} typed edges; {len(candidates)} eligible targets; sampling {len(cases)}")
    print(f"query  = {QUERY_WORDS} content words, set {label}; words naming the target (and the source) removed")
    print(f"control= naive keyword search on the query's rarest word over name+title+body, first {BASELINE_K} hits;")
    print("         second control = those hits plus their one-hop neighbours\n")

    kg_hit = base_hit = base1_hit = 0
    base1_sizes, shown_sizes = [], []
    both = kg_only = base_only = neither = 0
    kg_pos, ms_all, precisions = [], [], []
    kg_corpus = None
    rng = random.Random(SEED + 1)
    for i, name in enumerate(cases, 1):
        title, body, _f, typ = elements[name]
        if args.self:
            src, q = name, build_query(body, name_words(name))
        else:
            src = sorted(m for m in adj[name] if m in with_body)[0]
            q = build_query(elements[src][1], name_words(name) | name_words(src))
        shown, ms, corpus = recall(q)
        kg_corpus = kg_corpus or corpus
        b = baseline(q, elements)
        # the second control: the same grep hits PLUS their one-hop neighbours - what a searcher who
        # opened each hit and followed its edges would see. If this scores as well as the KG, the
        # graph's traversal adds nothing beyond "find a linked record, list its links".
        b1 = set(b)
        for hit in b:
            b1 |= adj.get(hit, set())
        h_kg, h_b = name in shown, name in b
        base1_hit += name in b1
        base1_sizes.append(len(b1))
        shown_sizes.append(len(shown))
        kg_hit += h_kg
        base_hit += h_b
        both += h_kg and h_b
        kg_only += h_kg and not h_b
        base_only += h_b and not h_kg
        neither += not h_kg and not h_b
        ms_all.append(ms)
        relevant = {name} | adj.get(name, set())
        if shown:
            precisions.append(sum(1 for s in shown if s in relevant) / len(shown))
        if h_kg:
            kg_pos.append(shown.index(name) + 1)
        print(f"{i:3d} {name[:30]:30s} <- {src[:22]:22s} kg={'Y' if h_kg else '.'} grep={'Y' if h_b else '.'} "
              f"rows={len(shown):<3} p={precisions[-1] if shown else 0:.2f} {ms}ms")

    n = len(cases)
    print("\n" + "=" * 78)
    print(f"query set                  : {label}")
    print(f"KG hit (target shown)      : {kg_hit}/{n} = {100 * kg_hit / n:.0f}%")
    print(f"control hit (rarest word)  : {base_hit}/{n} = {100 * base_hit / n:.0f}%")
    print(f"control + 1-hop neighbours : {base1_hit}/{n} = {100 * base1_hit / n:.0f}%  in a candidate set of mean {sum(base1_sizes) // n} "
          f"(UNRANKED; the KG shows mean {sum(shown_sizes) // n} ranked rows) - the size the reader would have to read")
    if precisions:
        precisions.sort()
        print(f"KG precision (1-hop floor) : mean {sum(precisions) / len(precisions):.2f}, "
              f"median {precisions[len(precisions) // 2]:.2f} over {len(precisions)} pushes")
    if kg_pos:
        kg_pos.sort()
        print(f"KG hit position            : median {kg_pos[len(kg_pos) // 2]}, top-3 in {sum(1 for p in kg_pos if p <= 3)}/{kg_hit}")
    print(f"2x2 (KG hit / control hit) : both {both}, KG-only {kg_only}, control-only {base_only}, neither {neither}")
    p = mcnemar_exact(kg_only, base_only)
    verdict = "significant at 0.05" if p < 0.05 else "NOT significant - parity not excluded"
    print(f"McNemar exact, two-sided   : p = {p:.3f} on {kg_only + base_only} discordant pairs ({verdict})")
    print(f"corpus asymmetry           : KG searches {kg_corpus} model items; the control scans {len(elements)} titled elements")
    print(f"latency                    : mean {sum(ms_all) // len(ms_all)}ms, max {max(ms_all)}ms")


main()
