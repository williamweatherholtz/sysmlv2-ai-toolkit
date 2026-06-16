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
  engine: sysmlv2-ai-toolkit
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
| Parity           | `sysmlv2 orient .` ≡ `query.py orient`    | identical done/outstanding/ready |

**Parity is the anti-drift gate (issue006).** The Rust CLI and the Python
authority must agree. The 2026-06-16 regression (rustToolchainFix / issue005)
happened because nothing compared them; once the parity check exists it is a
first-class layer here. Until `rustToolchainFix` lands, the two diverge — when
that is the case, report the divergence explicitly rather than masking it.

**Evidence before assertion (D0016 spirit):** a DoD `method=test` result is only
as good as the run behind it. Capture the actual command output; never record a
pass you did not observe. `confirmation` results are the human's to give — this
skill never fabricates one.

**No-kernel default (post-rustToolchainFix):** verification should run on the
Rust path (fast, no JVM); the kernel validators are the fallback and the parity
oracle, not the routine path. Say which path produced each verdict.

## Behavioral Instructions

1. **Run the layers in order**, capturing real output:
   1. `cargo test` (repo root) — unit + integration + BDD.
   2. `cargo clippy -- -D warnings` (or the workspace deny-config) — zero warnings.
   3. Parity: run `./target/release/sysmlv2.exe orient .` and
      `conda run ... python .engine/tools/query.py orient`; diff
      done/outstanding/ready. **Do not pipe `conda run` into a live cmdlet**
      (CLAUDE.md §6 — the JVM holds the pipe and the shell hangs); run plain,
      capture, compare.
2. **A red layer stops the verdict.** Report which layer failed with its output;
   do not proceed to "GREEN."
3. **On parity divergence:** report the exact delta (counts + ready-set diff) and
   route to `rustToolchainFix` (issue005). A known-divergent state is a *fail* of
   the parity layer, reported honestly — not a skip.
4. **Hand off the result** to `test-result` to record the appended TestResult
   (outcome + judgedAgainst commit + judgedAt + judgedBy). This skill does not
   write TestResults itself.
5. **Read-only:** runs tests + reports; never edits source or commits.

## Anti-Patterns

1. **Green-without-running** — claiming tests pass from reading code/CI memory.
2. **Skipping the parity layer** — "cargo test passed" while `orient` silently
   disagrees with `query.py` is exactly the hole that hid issue005.
3. **Masking divergence** — reporting GREEN when parity fails because "it's a
   known issue." Known ≠ passing; report it as a fail routed to its task.
4. **Hanging the shell** — piping `conda run` output into `Select-String`/
   `Out-Null`/redirects (CLAUDE.md §6). Run plain.
5. **Fabricating a confirmation** — verification evidence for `method=test` is the
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
