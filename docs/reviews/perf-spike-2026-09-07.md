# Performance spike — where keel's time goes, measured (2026-09-07)

**Question (the human's words):** *"Keel has become slow, which is a known tradeoff for a computed state, but
ideally I'd like ways to achieve performance even though our data doesn't follow a strict schema … e.g. a
database acting as cache, some kind of indexing, some auto-refreshing intermediate layer? It can still be pure
compute, maybe we should just have a file index hash that can identify changes quickly? … I feel like we don't
make a ton of changes all at once, so keel shouldn't have to take so long to compute state."*

**Answer:** the computing is 1% of the time. Parsing the whole corpus costs 275 ms and happens once per
process. The rest is repetition — 65 guards run serially and each re-read the corpus from disk, 56 git spawns
at ~79 ms each that the perf line reports as 0 ms, 40 clones of a cached model — and the whole 25 s is paid at
every turn boundary on a tree that has usually not changed. Decision: D0367. Tracked defects: issue409,
issue410. The work: `dcGuardsRunInParallelAndTimed`, `dcGitFactsAreContentAddressed`,
`dcOneCorpusPerProcess`, `dcGateAnswersFromItsReceipt`, then two conditional items.

## Host

Windows 11 Enterprise, 20 logical cores, Windows Defender on-access scanning **with no exclusion for the
repository**. Release binary built from `018315a`. Every number below is warm-cache unless stated. A cold read of
the 11.6 MB corpus by a Python loop took **51 s**; the same read warm took **0.4 s** — Defender scans every
open, so on this host the unit of I/O cost is the *open*, not the byte. `git rev-parse HEAD` costs ~79 ms per
spawn (shell timer, five runs).

## Whole commands (shell timer, `KEEL_PERF=2`)

| command | wall | perf line |
|---|---|---|
| `keel validate .` | 0.46 s | 678 files |
| `keel check-engine .` | ~0.5 s | — |
| `keel gate --fast` | 0.86 s | — |
| `keel whats-next` | ~5 s | same pipeline as orient |
| `keel orient` | **5.1 s** | Model::build x7 (6 cached, 1 parsed), parse 281 ms, git x23 |
| `keel guard` | **25.5 s** | Model::build x41 (40 cached, 1 parsed), parse 273 ms, git x56 "in 100ms" (untimed — issue410) |

## Where `keel guard` goes — per guard, in-process

Measured with a measurement-only patch wrapping `run_all`'s `run_one(n, root)` call in
`crate::perf::phase("guard:<n>", …)` (script: `phase_patch.py`, reverted after the run). Sum of phases 27.1 s
over a 25.5 s wall — phases nest (priority-inversion's orient run is inside its phase).

| guard | ms |
|---|---|
| priority-inversion | 6825 |
| decision-scaffolding | 1842 |
| attribute-vocabulary | 1376 |
| untrusted-taint | 1281 |
| sequence-multiplicity | 1053 |
| edge-endpoints | 1004 |
| claude-surface-drift | 916 |
| acceptance-binds-to-text | 906 |
| release-recorded | 838 |
| parser-coverage | 805 |
| confirmation-authenticity | 643 |
| verification-trace | 619 |
| identity-present | 518 |
| scaffold-placeholder | 482 |
| stale-gate-prose | 454 |
| identity-well-formed | 454 |
| acceptance-events | 416 |
| control-event-coverage | 409 |
| base-first-justification | 401 |
| stpa-currency | 367 |
| claim-ancestry | 355 |
| retro-backlog | 334 |
| control-map-reconciled | 282 |
| type-collision | 280 |
| untrusted-routing | 270 |
| resolver-kind | 266 |
| engine-lint | 264 |
| sprint-closure | 243 |
| activation-manifest | 243 |
| instruments-declared | 225 |
| marker-vocabulary | 194 |
| impossible-evidence-date | 188 |
| decision-requirement-link | 186 |
| sprint-coverage | 180 |
| duplicate-identity | 163 |
| attestation-substance | 144 |
| process-change | 141 |
| evidence-cited | 135 |
| charter | 112 |
| judgment-request-quality | 107 |
| ownership | 106 |
| issues | 106 |
| gating-workflow-history | 105 |
| viewpoint-renderer | 103 |
| attestation-authority | 85 |
| tool-reference | 84 |
| question-coverage | 83 |
| doc-sync | 82 |
| decision-amends-process | 80 |
| ceremony | 74 |
| actors | 68 |
| decision-rationale | 59 |
| critic-independence | 59 |
| requirement-rootedness | 57 |
| manifest-coverage | 46 |
| process-skill | 9 |
| hook-config-integrity | 9 |
| process-applicability | 3 |
| gate-environment-parity | 1 |
| enrollment-binding | 1 |
| doc-guard-count | 1 |
| cli-surface-declared | 1 |
| unit-extras-present | 0 |
| manifest-key-portability | 0 |
| control-defect-registry | 0 |

Also in the same run: `model:cache_clone` **1573 ms** across 40 cache hits — a `Model` clone per hit
(`view/mod.rs` `cached_model`, `.map(|(_, m)| m.clone())`).

Reading it: one guard (priority-inversion) is a quarter of the run because it calls `orient::compute` whole.
About twenty guards sit at 0.4–1.8 s because they scan lines rather than the model and so re-read the corpus
from disk — `guards.rs` has 39 `collect_sysml(` sites and 61 `read_to_string(` sites — and on this host each
open is a Defender scan. The rest are under 300 ms, most of which is the model-cache clone.

## Where `keel orient` goes — per step, in-process

Same method on `orient::compute` (script: `orient_phase_patch.py`, 14 anchors, reverted).

| step | ms |
|---|---|
| criterion_suspects | 1797 |
| deliverable | 955 |
| narrowing | 510 |
| activation | 340 |
| in_progress | 277 |
| burndown | 269 |
| indexer | 224 |
| valid_commits | 171 |
| sync2 | 168 |
| open_issues | 117 |
| pending | 116 |
| sync1 | 88 |
| dod_files | 64 |
| propagate | 0 |

`criterion_suspects` and `deliverable` are git spawns: `cat-file --batch` for DoD blobs at every distinct
verified SHA plus a `git grep` fallback per miss, and one `git diff --name-only <sha>..HEAD` per distinct SHA.
Every one of those answers is a fact about an immutable commit.

## The approaches, ranked, with the number that ranks each

| # | approach | expected effect on this host | cost | verdict |
|---|---|---|---|---|
| 1 | run the guards across a thread pool, order preserved; time every guard and every spawn in-process | guard 25.5 s → bounded by slowest guard (~6.5 s) | one function, no dependency | **do** — dcGuardsRunInParallelAndTimed |
| 2 | content-addressed cache of git facts keyed by full commit id (is-commit, DoD text at sha, paths changed since sha) | orient 5 s → ~2 s; priority-inversion with it | a machine-local file, trivially correct (immutable keys) | **do** — dcGitFactsAreContentAddressed |
| 3 | one corpus per process: stat-validated content cache, memoized walk, `Arc<Model>` on cache hit | removes the per-guard reopen and the 1.57 s of clones | a module + mechanical replacement of read sites | **do** — dcOneCorpusPerProcess |
| 4 | a receipt of the last green run keyed by HEAD + dirty set (with stats) + `.keel/` stats + build id | idle turn boundary 25 s → a stat pass | the key must be argued sound; DoD names the check | **do** — dcGateAnswersFromItsReceipt |
| 5 | in-process git object reads (gix) | removes the residual spawns after 2 | ~60 crates | **conditional** on residual > 500 ms — dcGitReadsStayInProcess |
| 6 | resident daemon serving the hooks | only pays if 4 leaves > 1 s idle | a process lifecycle to get right | **conditional** — dcHookDaemonOnlyIfTheReceiptIsNotEnough |
| 7 | Defender exclusion or Dev Drive for the repository | every open loses the scan (51 s vs 0.4 s cold) | a change to the human's machine | **theirs to make**, not keel's |

Declined with a reason rather than deferred: a database or index (the costly computations are whole-corpus
predicates, which a database scans too; a materialised copy is a second truth, §1/D0018); a faster parser
(275 ms is the smallest number here); a salsa-style incremental framework (pays only inside a daemon and rewrites
every computation as a query; bounded by rank 4's win). The "file index hash" the question proposed already
exists — `fingerprint::of`, ~40 ms, memoized per epoch — and keys the model cache; what was missing is a
receipt keyed on it, which is rank 4.

## Caveats

Every number is from one host with Defender scanning each open. A Linux CI runner will show a different mix —
less I/O, the same serial guard cost. The per-guard standalone times (`keel guard <name>` one at a time,
`perf_per_guard.txt`) include ~180 ms of process start each and are not the in-process table above. The
`parse 18306ms` seen in one run did not reproduce (273–281 ms on every rerun) and is attributed to a cold
Defender scan cache, corroborated by the 51 s Python cold read; it is stated as an attribution, not a finding.

## Measurement patches (reverted; kept in `perf-spike-2026-09-07/` so the numbers can be re-derived)

`phase_patch.py` — one anchor in `guards.rs` `run_all`, wraps `run_one(n, root)` in `crate::perf::phase`.
`orient_phase_patch.py` — fourteen anchors in `orient.rs` `compute_orient`, one phase per step.
`clone_phase_patch.py` — one anchor in `view/mod.rs` `cached_model`, times the clone.
Each fails when its anchor is not found exactly once (sprint 592 retro). After dcGuardsRunInParallelAndTimed
lands the first is permanent and the scripts are unnecessary.
