# project-migration — moving a project onto an engine it did not ask for

Deploys `.engine/processes/project-migration.sysml` (D0275). **Not** D0067's `migration`, which
changes a project's own data by a transform its author writes. This is the opposite direction: the
engine moved, the project did not choose it, and the project has to land somewhere honest.

## Why this one is different from every other keel discipline

The engine cannot migrate itself. `check_preconditions` refuses any tree holding `keel-cli/Cargo.toml`
as a self-build (`migrate.rs:664-666`), so this is the single surface the self-build never exercises.
The defect density follows exactly: **seven defects in this path — issue301, issue310, issue314,
issue323, issue324, issue326, issue327 — and not one was found by a test.** Three were found by one
downstream session in a single day. Four still have no test.

Read that as an instruction about your own confidence, not as history. When you migrate a project you
are on the least-tested path in the system, so **read state back rather than trusting a command's own
report** — that rule is general in keel and load-bearing here.

## The six steps

1. **Preflight.** Gate on the CURRENT engine, record that SHA, make the tree clean, note existing
   library drift.
2. **Check what is REMOVED.** Additions are safe; renames and removals break whatever named the old
   thing.
3. **Apply.** `keel migrate .` — let it refuse, let it roll back, never hand-repair a partial run.
3b. **Verified or reverted (D0336).** `keel migrate` runs the project's own gate after applying - validate, every enforced guard, check-engine - under the new binary. Green retains and moves the pin; any red reverts .engine/ and .tracking/ to the pre-update commit, prints the gate's output verbatim and records the attempt (`.keel/update-attempts.toml`; `keel status` shows it; a re-run names it). There is no half-updated state to reconcile. `--no-verify` is the deliberate exception and says UNVERIFIED.

4. **Reconcile.** Prove the project's own choices survived: pin, adoption, project-owned contracts, its own `[unit]` sections in `unit-extras.toml` (merged, never overwritten; a colliding section blocks by name, D0317),
   customised files. Then gate.
5. **Prove it remotely.** A local green is one machine and one binary. Read the project's own CI.
6. **Report upstream.** The engine is blind here; your report is the only channel.

## The three traps, each of which has already happened

**The deadlock (issue324).** Migrate refuses a dirty tree. Under engine-version skew the pre-commit
gate refuses too. So a project that is behind its pin *and* holds one uncommitted file can neither
migrate nor commit. The file is usually engine-authored — an obligation record the Stop hook wrote —
which means this fires precisely when the project is already unhealthy.

Escape, bypassing nothing: `git stash push -u -- <path>` → migrate → `git stash pop`.

**Never `--no-verify`.** It disables every other check to get past one, and the one you are getting
past is correct.

**The half-update.** Taking a newer library *unit* while the binary stays pinned is the case no
refusal covers today. It fails silently. Until D0252 clause A's capability declaration ships, move
the engine and the units together or not at all.

**The CI that was never running.** Do not read a silent Actions tab as success. A scaffolded project's
gate workflow has never run — its install step is three `echo` lines (issue327) — and a project that
imported `decision-channel` acquired two workflows that cannot build (issue326). Check that the gate
*executed* before believing what it did not say.

## What "done" means

The tree is either **fully migrated and green**, or **byte-identical to the preflight SHA**. There is
no third outcome, and a report of one is a defect worth filing.

## Removal path

Delete this skill + registry + the process file. `keel migrate` keeps working; only the discipline
leaves the catalogue.
