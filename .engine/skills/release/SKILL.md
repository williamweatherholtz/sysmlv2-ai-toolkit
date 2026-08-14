---
name: release
description: |
  Deploys the release process (D0135): reconcile the version, run the full gate on the exact tree
  being tagged, push a vX.Y.Z tag so CI builds and publishes per-platform binaries, then record a
  Release item. Use when a coherent capability step is complete on a green trunk, or when asked to
  cut, publish, or tag a release. Do NOT use per-sprint — the cadence is per-milestone, and a tag per
  sprint makes the tag meaningless.
metadata:
  version: 0.1.0
  domain: [release, deploy, semver, tag, publish, distribution, D0135]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# release (cut, publish, record)

Deploys `.engine/processes/deploy.sysml`. The engine had **defined** the Deploy workflow and never run
it: one stale release, no recorded `Release` item, version discipline left to memory.

## The four steps

**1. Reconcile the version.** Set the crate version in root `Cargo.toml`. *Reconcile* means each surface
is deliberate, **not** that they match:

| Surface | Tracks | Bumps when |
|---|---|---|
| crate `version` | the shipped binary | any release |
| `KEEL_API_VERSION` (`serve.rs`) | the committed `/api/*` read contract a viewer pins | a read shape breaks |

Name any change that breaks a caller. Example: in 0.2.0 write paths **refuse** when the acting actor is
unstated (D0129) — that breaks scripts relying on a defaulted actor.

**2. Gate the exact tree you will tag.**

```
keel validate . && keel guard . && keel check-engine .
cargo clippy --workspace --all-targets -- -D warnings
cargo test --release
gh run list --limit 1          # CI green at THIS commit
```

A release is the one artifact you cannot quietly re-verify later — downstream users hold the binary.
Never tag a tree whose gate you have not run at that commit, and never tag with a bypass in force.

**3. Tag; CI publishes.**

```
git tag -a vX.Y.Z -m "<one line>" && git push origin vX.Y.Z
```

`.github/workflows/release.yml` builds natively for linux-x86_64, macOS-arm64 and windows-x86_64 and
attaches all three. **Verify the run succeeded and the assets are attached** — a tag whose build failed
is worse than no tag, because the version exists but is unobtainable.

This is what makes the portable-gate guarantee real for a distributed team (D0129): a contributor gets a
matching gate by download, not by installing a Rust toolchain.

**4. Record the `Release`.** Author it in `.tracking/baselines.sysml` with version, commit and purpose.
A git tag is not a tracked fact — it cannot be traced to, queried, or asked as-of. Leave the release
**contents** computed from git ancestry (§2.1); a hand-written contents list drifts the moment anything
is amended.

## Anti-patterns

| Don't | Why |
|---|---|
| Tag before running the gate at that commit | The one artifact that can't be re-verified after the fact |
| Tag with `SKIP_VALIDATE=1` in force | Ships a binary no layer actually checked |
| Hand-list what the release contained | Derivable from git — a stored copy drifts (§2.1) |
| Release per sprint | Cadence is per-milestone; a tag per sprint means nothing |
| Assume the workflow succeeded | Check the run and the attached assets |
| Make the crate and API versions equal | They are different contracts with different consumers |
