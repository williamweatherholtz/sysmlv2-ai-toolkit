---
name: test-verify
description: |
  The D0425 VERIFIER's procedure: run the deliverable's gate set against the tree
  as it is - keel suite --touched (detached, receipt read only after exit), validate,
  check-engine, guard --no-receipt, sync-claude --check, clippy, the sprint's D0388
  probe pair - and report every discrepancy naming the command. Use when dispatched
  as a verifier subagent, or asked to "verify," "is it green," "run the touched set,"
  or before any deliverable DoD or gate TestResult is recorded. READS and RUNS only:
  writes nothing under .tracking or .engine, records no TestResult (that is the
  recorder's, from this skill's receipt), does NOT build from scratch (use build)
  and does NOT commit (use repo-push).
metadata:
  version: 0.2.0
  domain: [rust, cargo-test, clippy, keel-suite, touched-set, verification, D0425, SysMLv2]
  writePolicy: readOnly
  engine: keel-ai-toolkit
---

# test-verify — the verifier's procedure (D0425 / D0432 / D0438)

A session splits into a PRIMARY that does the substance and a VERIFIER that runs the checks in
its own context (D0425). This skill IS the verifier's procedure. Before it existed the procedure
was retyped into every dispatch prompt (issue469) and a verifier that had it only in prose wrote to
the tree and reported the state from before its own edit (issue471). The primary's dispatch names
this skill, the sprint's probe pair, and nothing else.

## Mandate — read this before any command

| You DO | You do NOT |
|---|---|
| run the commands below, verbatim, against the tree as it is | edit, create or delete ANY file under `.tracking/`, `.engine/`, `keel-cli/`, `.claude/` or `CLAUDE.md` |
| read files, receipts and logs | run `keel record`, `append-result`, `append-gate-result`, `accept`, `new sprint`, `git add/commit/push` |
| report every discrepancy between what was claimed and what a command returned, NAMING the command | "fix" a red by triaging, patching or re-recording — a red is a line in your receipt, and the RECORDER or PRIMARY acts on it |
| say `NONE` under discrepancies only when every check below ran to completion | summarise: the receipt carries the counts and the exact lines, not your reading of them |

A write you believe is owed (an untriaged obligation, a missing `#Resolves` edge, a gate result)
is ONE LINE in your receipt under `OWED WRITES`, addressed to the recorder. issue471 is what
happens otherwise: a verifier hand-wrote `#Resolves d0437 part obligation... : Issue {` — a
marker on a part, not an edge — the parser skipped both Issues, parser-coverage went red, and the
receipt cited a validate run from before the edit.

## Procedure

Run from the project root. `KEEL` below is the binary named in the dispatch (default
`./target/release/keel.exe`); never `target/release/keel.exe` when a build may run — a copy
(`keel-serve.exe`, `keel-land.exe`) survives a relink.

### 1. Launch the touched run DETACHED, record the launch time

`keel suite --touched .` is the set the land will run (D0421/D0432): the integration tests whose
text names a changed `keel-cli/src/<stem>.rs`, plus the lib's own unit tests whenever any
`keel-cli/src` path changed. With the lib it takes 7-12 minutes on this host (434 s, 623 s, 713 s,
727 s measured) and a harness foreground call is capped at 600 s — the cap killed one run
(issue469). So:

```
python -c "import time; print(int(time.time()))" > .keel/metrics/verify-launch.epoch
(nohup KEEL suite --touched . > .keel/metrics/verify-touched.out 2>&1 < /dev/null & disown)
```

or the harness's `run_in_background` on the same command. Do NOT wait on it in a foreground call
with a timeout; do NOT read the receipt yet.

**No `keel-cli/src` change means an empty touched set** - the run takes seconds, its receipt says
`stems = []`, and that IS the answer. When the dispatch also names **`keel suite .`** (the full
suite, D0356: receipt `.keel/metrics/suite-receipt.toml` with `fingerprint`, `head`, `at`,
`passed`, `failed`, `outcome`, `seconds`, `log`; ~11 minutes here), launch it the same way AFTER the
touched run exits - the two would contend for cargo's lock - and read it under the same rules in
step 5, minus the `stems`/`lib` rows. The two receipts use the same two words the same way
(issue472): `at` is when the file was WRITTEN - the end of a done run, the start of a running stub -
and `seconds` is the run's wall clock, so `at` > launch epoch proves the run finished after you
launched it and `at - seconds` is within two seconds of the launch epoch when it is this run's;
`seconds` belongs in the `SUITE RECEIPT:` row as `ran=<n>s`.
`keel suite` REFUSES to run from `target/release/keel.exe` (it cannot relink its own image); run
it from the copy the dispatch names.

### 2. Run the rest of the gate set while it runs

Each line of the receipt is `<command> -> <the verdict line the command printed>; exit=<code>`.

```
KEEL validate .                       # the .tracking semantic authority
KEEL check-engine .                   # .engine instance reference resolution
KEEL guard --no-receipt .             # every enforced forward guard; --no-receipt forces the run
KEEL sync-claude --check .            # the claude-surface-drift check
git rev-parse --short HEAD
git status --short                    # count and list; the recorder needs to know the tree was dirty
```

`guard` exit 1 is a FAIL to report with the failing guard's line — not a thing to explain away.
The dispatch may name guards it EXPECTS red (a proposed Decision's known red); report them as red
and cite the dispatch's expectation beside each.

Note `keel guard` before staging reads NOTHING for the index-reading guards (`process-change` scans
`git diff --cached`; issue464): say in the receipt that the commit tier was not exercised.

### 3. The D0388 probe pair

The dispatch names the sprint's check and its two cases — one known-positive, one known-negative,
chosen before the tree was read. Run exactly what the dispatch names (a `cargo test --release
<name>` filter, a `python scripts/probes/<x>.py --probe`, or a `KEEL <lens>` over a fixture) and
report both cases' outcomes by name. A dispatch that names no pair is reported as `PROBE PAIR: not
named by the dispatch` — never invented.

### 4. Clippy — and what to do when the build lock is held

```
cargo clippy --release --all-targets -- -D warnings
```

The touched run holds cargo's build lock, so clippy may block until it finishes. Run it AFTER
step 5 if it did not complete within the foreground cap, and report `TIMEOUT` with the reason
rather than `pass`; never report a clippy you did not see finish.

### 5. Read the touched receipt — ONLY after the process exits

Wait until the process is gone (`tasklist | grep -i cargo` / `pgrep cargo` is empty and
`verify-touched.out` ends with a `test result:` / `touched:` line). Then read
`.keel/metrics/touched-receipt.toml` and check, in this order, each as its own receipt line:

| Check | Honest when | Why (issue468 / D0387) |
|---|---|---|
| `outcome` | is not `"running"` | the stub written at launch says `running`; a killed run leaves it |
| `at` | > the epoch in `verify-launch.epoch` | otherwise this is the PREVIOUS run's receipt; sprint 661 was recorded on one 46 minutes stale |
| `stems` | == the sorted set of `<stem>` for every changed `keel-cli/src/<stem>.rs` (`git diff --name-only origin/main -- keel-cli/src` plus untracked) | the receipt must be over THIS change set |
| `lib` | `true` whenever any `keel-cli/src` path changed | the lib run is where pass-alone/fail-together tests show (issue459) |
| `passed` / `failed` / `failing` | copied verbatim | the recorder's `--evidence` quotes these |
| `head` | == `git rev-parse --short HEAD` | |

An empty `stems` with no `keel-cli/src` change is a receipt too: report it as such and run
nothing more.

### 6. Write the receipt to the path the dispatch names

Plain text, this shape, in the scratchpad path the dispatch gives (never under the project):

```
VERIFIER RECEIPT  <date>  head=<sha>  tree=<clean|N dirty paths>
KEEL validate . -> <line>; exit=<n>
KEEL check-engine . -> <line>; exit=<n>
KEEL guard --no-receipt . -> PASS <n>; FAIL <m> [<guard>: <line>]; exit=<n>
KEEL sync-claude --check . -> <line>; exit=<n>
cargo clippy ... -> <pass|TIMEOUT|fail: first error>; exit=<n>
PROBE PAIR: <check> -> positive <case>: <outcome>; negative <case>: <outcome>
TOUCHED RECEIPT: outcome=<..> passed=<n> failed=<n> seconds=<n> at=<epoch> launch=<epoch> (<at>launch: ok|STALE)
  stems=<[...]> changed=<[...]> (MATCH|MISMATCH) lib=<bool> head=<sha> log=<path>
  test result line: "<verbatim from verify-touched.out>"
DISCREPANCIES: NONE | <one line each, naming the command>
OWED WRITES: NONE | <one line each, for the recorder>
```

The recorder reads THIS file and nothing else; it never reads the primary's description.

## Anti-patterns — each one has happened

1. **Foreground touched run** (issue469): killed at the cap, eleven minutes lost. Detach.
2. **Reading the receipt while `outcome = "running"`** or with `at` before the launch (issue468,
   sprint 661): the previous run's pass over a different change set.
3. **A filter the primary chose instead of the touched set** (issue459/D0432): six tests passed
   alone, the lib run failed one of them.
4. **Writing to the tree** (issue471): the mandate above. A verifier with a fix in mind writes it
   down for the recorder.
5. **Reporting pre-edit state** (issue471): every receipt line is from a command run in this
   dispatch, after the last write the tree saw.
6. **`validate` green read as SysML-conformant** (issue097): it is the engine's authority, not the
   kernel's.
7. **Reading a CI verdict from an exit code** (D0420/issue434): only `gh run list --json conclusion`.

## Questions This Skill Answers

- "Verify this sprint" / "run the verifier" / "is the tree green?"
- "Run the touched set" / "what will the land run?"
- "Is the receipt honest?" (outcome / at / stems / lib)
- "What evidence backs this gate?" — the receipt file, line by line
