---
name: build
description: |
  Compiles the deliverable (the Rust keel workspace) and confirms the model
  still validates. Use when asked to "build," "compile," "does it build,"
  "cargo build," "rebuild the binary," or before any change that claims the
  toolchain works. Runs the release build, surfaces warnings-as-errors, and
  reports the binary location. Does NOT run tests (use test-verify) and does NOT
  commit (use repo-push).
metadata:
  version: 0.1.0
  domain: [rust, cargo, build, compilation, deliverable, SysMLv2]
  writePolicy: readOnly
  engine: keel-ai-toolkit
---

# build

Covers the compile step of Delivery for the **deliverable** (the Rust `keel`
parser + CLI workspace). Output: a green release build of `keel.exe` and a
statement of whether the engine model still validates. Build is a *prerequisite*
for `test-verify`, not a substitute for it.

## Expert Vocabulary Payload

**Workspace layout:** Cargo workspace at repo root; crates `keel-parser` (lib)
and `keel-cli` (bin → `target/release/keel.exe`). Both compile under
`#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]` — a warning
**is** a build failure here, by design (rustS0 DoD).

**Release vs debug:** ship/verify against `--release` (the binary downstream tools
and CLAUDE.md §-paths invoke). Debug builds are for fast iteration only.

**build.rs spec pin (rustS4):** the build fetches the SysML v2 grammar manifest
and verifies its SHA256; a mismatch **fails the build** with an actionable
message. `SYSML_V2_SPEC_OFFLINE=1` skips the remote fetch (offline / CI without
network). If a build fails on the spec check, that is a *real signal* (grammar
drift), not noise — route it to `specVersionRuntimeCheck` / `toolchainWatch`.

**Two artifacts, one build discipline:** the *deliverable* is the Rust binary;
the *engine model* is the `.sysml` tree. A build is not "done" until both are
green — the binary compiles AND the model still validates (else the binary may
no longer match the schema it parses).

## Behavioral Instructions

1. **Build release, warnings-as-errors:**
   `cargo build --release` from repo root. The deny-attributes make warnings
   fatal; do not pass `--allow` to silence them — fix the cause.
2. **On spec-check failure** (build.rs SHA mismatch): do not blindly set
   `SYSML_V2_SPEC_OFFLINE=1` to make it pass. Report the drift; only use offline
   mode when network access is genuinely unavailable, and say so.
3. **Confirm the binary exists:** `target/release/keel.exe`. Report its path.
4. **Validate the model still parses** with the no-kernel path:
   `./target/release/keel.exe validate .` (fast, no JVM). If the Rust
   validator is not yet trustworthy (see issue005 / `rustToolchainFix`), fall
   back to the kernel validators (CLAUDE.md §5) and SAY which path you used.
5. **Report actual output** — compile result + binary path + validation verdict.
   Never claim "builds clean" without having run it (evidence before assertion).
6. **Read-only:** this skill builds and reports; it does not edit source, commit,
   or push. Hand off to `test-verify` for tests, `repo-push` for commits.

## Anti-Patterns

1. **Silencing warnings** — `#[allow(...)]` or `--cap-lints` to force a green
   build. The deny-attributes are the gate; fix the warning.
2. **Debug-only build** — verifying against `target/debug` then claiming the
   shipped `--release` binary works.
3. **Offline-mode papering** — setting `SYSML_V2_SPEC_OFFLINE=1` to dodge a real
   grammar-drift failure instead of reporting it.
4. **"It builds" without running cargo** — asserting success from reading code.
5. **Conflating build with verify** — a compile is not a pass; tests still run.

## Output Format

```
build:
  cargo_build_release: pass|fail        # actual
  warnings: 0                            # deny => must be 0
  spec_check: ok|offline|MISMATCH
  binary: target/release/keel.exe
model_validation:
  path: rust|kernel                      # which validator ran
  result: pass|fail
verdict: GREEN | BROKEN
```

## Questions This Skill Answers

- "Build it" / "Compile the toolchain" / "Does it build?"
- "Rebuild the keel binary"
- "Is the release build clean?"
- "Why is the build failing?" (spec check, warnings)
- "Did my change break compilation?"
