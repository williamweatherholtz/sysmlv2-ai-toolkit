---
name: test-verify
description: |
  Runs the deliverable's test + verification gate: cargo test, clippy
  warnings-as-errors, BDD scenarios, and the Python<->Rust orient parity check.
  Use when asked to "test," "run the tests," "cargo test," "verify," "is it
  green," "check parity," or before recording any deliverable DoD TestResult.
  Reports actual pass/fail per layer. Does NOT compile from scratch (use build)
  and does NOT commit (use repo-push).
metadata:
  version: 0.1.0
  domain: [rust, cargo-test, clippy, BDD, cucumber, parity, verification, SysMLv2]
  writePolicy: readOnly
  engine: keel-ai-toolkit
---

# test-verify

Covers the verify step of Delivery for the **deliverable**. Output: a per-layer
green/red verdict that is the evidence behind a `method=test` DoD TestResult.
This skill produces *evidence*; recording the TestResult itself is `test-result`.

## Expert Vocabulary Payload

**Test layers (all must pass):**
| Layer            | Command                                   | Gate |
|------------------|-------------------------------------------|------|
| Unit + integration | `cargo test`                            | all green |
| Lint             | `cargo clippy -- -D warnings`             | zero warnings |
| BDD scenarios    | cucumber-rs scenarios (run via `cargo test`) | all green |
| Self-consistency | `keel orient .` total == structural `action` count | matches |

**Rust is the sole authority (D0048; query.py + parity_check retired at M4/D0074).** There is
no second implementation to diff against any more — verification is `cargo test` + `cargo clippy`
+ `keel` self-consistency (the orient total equals the distinct `action` declarations). When
porting query.py logic earlier (M2.2/M3), parity vs query.py WAS the gate; post-M4 the python
reference is gone, so a port's evidence is its tests + the deletion's green commit.

**Evidence before assertion (D0016 spirit):** a DoD `method=test` result is only
as good as the run behind it. Capture the actual command output; never record a
pass you did not observe. `confirmation` results are the human's to give — this
skill never fabricates one.

**No-kernel default:** verification runs on the Rust path (fast, no JVM); the kernel
validators remain only for deep `.engine` SysML semantics, not the routine path. Say which
path produced each verdict.

## Behavioral Instructions

1. **Run the layers in order**, capturing real output:
   1. `cargo test` (repo root) — unit + integration + BDD.
   2. `cargo clippy -- -D warnings` (or the workspace deny-config) — zero warnings.
   3. Self-consistency: run `./target/release/keel.exe orient .`; confirm the
      total task count equals the distinct `action <name>;` declarations in `.tracking`.
2. **A red layer stops the verdict.** Report which layer failed with its output;
   do not proceed to "GREEN."
3. **Hand off the result** to `test-result` to record the appended TestResult
   (outcome + judgedAgainst commit + judgedAt + judgedBy). This skill does not
   write TestResults itself.
4. **Read-only:** runs tests + reports; never edits source or commits.

## Anti-Patterns

1. **Green-without-running** — claiming tests pass from reading code/CI memory.
2. **Skipping self-consistency** — "cargo test passed" while `keel orient`'s task
   total silently disagrees with the structural `action` count.
3. **Hanging the shell** — piping `conda run` output into `Select-String`/
   `Out-Null`/redirects (CLAUDE.md §6). Run plain.
4. **Fabricating a confirmation** — verification evidence for `method=test` is the
   run; for `confirmation` it is the human's word. Never infer the latter.

## Output Format

```
test_verify:
  cargo_test:   pass|fail   (N passed, M failed)
  clippy:       pass|fail   (warnings: 0)
  bdd:          pass|fail   (N scenarios)
  parity:       pass|fail   (rust done/out vs py done/out; ready-diff)
  path:         rust|kernel
verdict: GREEN | RED
failing_layer: <name + captured output>   # if RED
handoff: test-result  # to record the TestResult
```

## Questions This Skill Answers

- "Run the tests" / "cargo test" / "Is it green?"
- "Verify the toolchain" / "Check the build passes"
- "Do Rust and Python orient agree?" (parity)
- "Is clippy clean?"
- "What evidence backs this DoD?"
