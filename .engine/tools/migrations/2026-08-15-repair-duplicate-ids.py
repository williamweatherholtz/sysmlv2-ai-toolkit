"""D0067 migration: give every duplicated element id a distinct identity.

RULES, from the item and from D0067:
  * The FIRST occurrence in a deterministic (file, line) order KEEPS its id — so any external
    reference to that id stays valid and the change is minimal.
  * Every LATER occurrence is re-identified with a fresh UUIDv4.
  * Provenance is untouched: only the `id` attribute changes, never createdAt / createdBy / judgedAt.
  * Control totals must reconcile exactly: the count of id-bearing records is identical before and
    after, and the count of DISTINCT ids rises by exactly the number of records re-identified.
  * Never fabricate historical data — nothing is invented except the new identifiers themselves.

Run with --dry-run first; it prints the plan and the reconciliation without writing.
"""
import argparse
import collections
import glob
import io
import re
import uuid

ID_RE = re.compile(r':>> id = "([0-9a-fA-F-]{36})"')


def scan():
    locs = collections.defaultdict(list)
    for f in sorted(glob.glob('.tracking/**/*.sysml', recursive=True) + glob.glob('.engine/**/*.sysml', recursive=True)):
        rel = f.replace('\\', '/')
        for i, line in enumerate(io.open(f, encoding='utf-8').read().split('\n'), 1):
            m = ID_RE.search(line)
            if m:
                locs[m.group(1)].append((rel, i))
    return locs


def referenced_outside_declaration(dup_ids):
    """Is any duplicated id used as a REFERENCE anywhere (not as its own `:>> id =` declaration)?

    If so, re-identifying the later occurrence would break that reference and the migration would
    need a repointing step. Checked BEFORE transforming rather than discovered afterwards.
    """
    hits = []
    for f in sorted(glob.glob('.tracking/**/*.sysml', recursive=True) + glob.glob('.engine/**/*.sysml', recursive=True)
                    + glob.glob('.engine/**/*.toml', recursive=True) + glob.glob('.engine/**/*.md', recursive=True)):
        rel = f.replace('\\', '/')
        for i, line in enumerate(io.open(f, encoding='utf-8').read().split('\n'), 1):
            for d in dup_ids:
                if d not in line or ID_RE.search(line):
                    continue
                # PROSE, not a reference: an id quoted inside an attribute VALUE (a description or
                # procedureText DESCRIBING the defect) names a fact, it does not point at one. This
                # corpus contains prose about itself — the same trap that inflated the marker census
                # in issue099 — and it is safe here for a specific reason: every ORIGINAL id survives
                # on its first occurrence, so prose naming one stays accurate after the migration.
                is_prose = re.search(r'= "[^"]*' + re.escape(d), line) is not None
                hits.append((rel, i, d, line.strip()[:80], is_prose))
    return hits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--dry-run', action='store_true')
    args = ap.parse_args()

    before = scan()
    total_before = sum(len(v) for v in before.values())
    distinct_before = len(before)
    dups = {k: v for k, v in before.items() if len(v) > 1}
    to_reidentify = sum(len(v) - 1 for v in dups.values())

    print('BEFORE  records:', total_before, ' distinct ids:', distinct_before)
    print('        duplicate ids:', len(dups), ' records to re-identify:', to_reidentify)

    hits = referenced_outside_declaration(set(dups))
    prose = [h for h in hits if h[4]]
    real = [h for h in hits if not h[4]]
    print('        id mentions outside a declaration: ', len(hits), f'({len(prose)} prose, {len(real)} real references)')
    for r in prose[:5]:
        print('          PROSE ', r[0] + ':' + str(r[1]), '|', r[3])
    for r in real[:10]:
        print('          REF   ', r[0] + ':' + str(r[1]), '|', r[3])
    if real:
        print('        -> a repointing step is REQUIRED; refusing to transform.')
        return 1

    # plan: per file, the (line, old, new) rewrites
    plan = collections.defaultdict(list)
    for old, occurrences in sorted(dups.items()):
        for rel, line in occurrences[1:]:  # first keeps its id
            plan[rel].append((line, old, str(uuid.uuid4())))

    for rel in sorted(plan):
        for line, old, new in sorted(plan[rel]):
            print('  ', rel + ':' + str(line), old, '->', new)

    if args.dry_run:
        print('DRY RUN — nothing written.')
        return 0

    for rel, rewrites in plan.items():
        lines = io.open(rel, encoding='utf-8').read().split('\n')
        for line_no, old, new in rewrites:
            idx = line_no - 1
            assert old in lines[idx], f'{rel}:{line_no} no longer contains {old}'
            lines[idx] = lines[idx].replace(f'"{old}"', f'"{new}"', 1)
        io.open(rel, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))

    after = scan()
    total_after = sum(len(v) for v in after.values())
    distinct_after = len(after)
    remaining = {k: v for k, v in after.items() if len(v) > 1}
    print('AFTER   records:', total_after, ' distinct ids:', distinct_after)
    print('        duplicate ids remaining:', len(remaining))
    ok = (total_after == total_before
          and distinct_after == distinct_before + to_reidentify
          and not remaining)
    print('RECONCILES:', ok,
          f'(records {total_before}=={total_after}; distinct {distinct_before}+{to_reidentify}=={distinct_after}; duplicates 0)')
    return 0 if ok else 1


raise SystemExit(main())
