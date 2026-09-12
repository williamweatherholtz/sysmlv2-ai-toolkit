//! The ONE way to spawn git (dcInstrumentAllGitSites, closing the other half of issue147/issue148).
//!
//! WHY. The git spawn counter covered exactly two helpers — `view::git_out` and `govern::git_lines` —
//! out of 25 raw `Command::new("git")` sites across 11 files. Every reported total therefore
//! UNDERSTATED, and `orient` could report `git x0` while spawning git five times. Three separate
//! performance investigations this session mis-attributed cost because the count could not be trusted;
//! the 483-spawn finding in `assured` was only found because those two helpers happened to be on the
//! hot path. A counter that covers 8% of the sites is worse than no counter, because it gets believed.
//!
//! THE SHAPE: a drop-in constructor rather than a run-wrapper, deliberately. The 25 sites carry eleven
//! different builder chains (`.arg("-C")`, bundled `args(["-C", ..])`, `.output()`, `.ok()?`,
//! `map_err`, ignored results), and rewriting each to a common run signature is exactly the class of
//! mechanical multi-site edit that has failed three times this session (sprint 373's retro, occurrence
//! three). `gitx::git()` returns the same `Command` the old expression did, so every chain compiles
//! unchanged and the count happens at construction — which equals spawns, because every site runs the
//! command it builds.
//!
//! TIME as well as COUNT (dcGuardsRunInParallelAndTimed, issue410). For a year the constructor counted
//! and only the two rich helpers timed, so `keel gate guard` reported `git x56 in 100ms` over 4.4 s of spawns
//! and a reader trusting the line concluded git was free. The constructor now returns [`Git`], a thin
//! wrapper whose `output()`, `status()` and `spawn()` add their wall time to `GIT_NANOS` and note the argv
//! shape at `KEEL_PERF=2` - so every site that builds a command through here is timed by construction,
//! and the builder chains compile unchanged because the wrapper exposes the same methods. A `spawn()`
//! times only the spawn; the site that waits on the child times the wait itself (orient's two batch
//! reads do). The static test below still forbids a raw `Command::new("git")` anywhere else.

/// A `git` command, counted at construction and TIMED when it runs. The only permitted constructor — a
/// static test in this module fails the build on any `Command::new("git")` elsewhere in the crate.
#[must_use]
pub fn git() -> Git {
    crate::perf::add(&crate::perf::GIT_CALLS, 1);
    Git(std::process::Command::new("git"))
}

/// Time `f` into `perf::GIT_NANOS` whether or not instrumentation is on: one `Instant` read is nothing
/// beside a process spawn (~79 ms on the spike's host), and an always-on clock is what lets a test pin
/// that the counter moves without racing the `KEEL_PERF` lazy read.
fn clocked<T>(f: impl FnOnce() -> T) -> T {
    let t0 = std::time::Instant::now();
    let out = f();
    crate::perf::GIT_NANOS.fetch_add(u64::try_from(t0.elapsed().as_nanos()).unwrap_or(u64::MAX), std::sync::atomic::Ordering::Relaxed);
    out
}

/// A `git` [`std::process::Command`] whose runs are timed into `perf::GIT_NANOS`.
///
/// Exposes exactly the builder methods the crate's sites use; a site needing another one adds it here
/// rather than reaching the inner `Command`, so no run can slip out of the timing.
pub struct Git(std::process::Command);

impl Git {
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, a: S) -> &mut Self {
        self.0.arg(a);
        self
    }

    pub fn args<I, S>(&mut self, a: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.0.args(a);
        self
    }

    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, d: P) -> &mut Self {
        self.0.current_dir(d);
        self
    }

    pub fn env<K: AsRef<std::ffi::OsStr>, V: AsRef<std::ffi::OsStr>>(&mut self, k: K, v: V) -> &mut Self {
        self.0.env(k, v);
        self
    }

    pub fn stdin<T: Into<std::process::Stdio>>(&mut self, s: T) -> &mut Self {
        self.0.stdin(s);
        self
    }

    pub fn stdout<T: Into<std::process::Stdio>>(&mut self, s: T) -> &mut Self {
        self.0.stdout(s);
        self
    }

    pub fn stderr<T: Into<std::process::Stdio>>(&mut self, s: T) -> &mut Self {
        self.0.stderr(s);
        self
    }

    /// The argv shape for the `KEEL_PERF=2` tally: the first two tokens after any `-C <dir>` pair, so
    /// `-C <repo> cat-file --batch` tallies as `cat-file --batch` like the rich helpers always did.
    fn note(&self) {
        if !crate::perf::verbose() {
            return;
        }
        let all: Vec<String> = self.0.get_args().map(|a| a.to_string_lossy().into_owned()).collect();
        let mut rest: Vec<&str> = Vec::new();
        let mut skip = false;
        for a in &all {
            if skip {
                skip = false;
                continue;
            }
            if a == "-C" {
                skip = true;
                continue;
            }
            rest.push(a.as_str());
        }
        crate::perf::note_git(&rest);
    }

    /// Run to completion, timed.
    ///
    /// # Errors
    /// Whatever [`std::process::Command::output`] returns.
    pub fn output(&mut self) -> std::io::Result<std::process::Output> {
        self.note();
        clocked(|| self.0.output())
    }

    /// Run to completion inheriting stdio, timed.
    ///
    /// # Errors
    /// Whatever [`std::process::Command::status`] returns.
    pub fn status(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.note();
        clocked(|| self.0.status())
    }

    /// Spawn, timing the spawn only - the caller owns the wait and times it with
    /// `perf::timed(&perf::GIT_NANOS, ..)` around `wait_with_output`.
    ///
    /// # Errors
    /// Whatever [`std::process::Command::spawn`] returns.
    pub fn spawn(&mut self) -> std::io::Result<std::process::Child> {
        self.note();
        clocked(|| self.0.spawn())
    }
}

/// Is the local commit gate ARMED — i.e. can git actually reach `.githooks/pre-commit`?
///
/// issue240: wiredness used to be "`git config core.hooksPath` returned a non-empty value", a PROXY
/// for reachability rather than reachability. With `core.hooksPath = nul` that proxy answers TRUE
/// while git resolves hooks to a directory that does not exist, so no hook runs at all — and three
/// shipped surfaces (`orient`, `hardening`, `controls`) reported the gate armed and fail-closed for
/// an unknown period, during which the AI told the human "the hook passed" on commits no hook saw.
/// A control reported as present must be reported on its REACHABILITY, never on the presence of a
/// setting that points at it.
///
/// Returns `Ok(dir)` with the effective hooks directory when armed, or `Err(reason)` naming why not,
/// so callers can say WHICH of "not configured", "configured but missing" or "no pre-commit in it"
/// holds instead of collapsing three states into one boolean (the N-C2 honest-surface invariant).
///
/// # Errors
/// Returns the reason the gate is not armed: no `pre-commit` in the tree, git unrunnable, an
/// unresolvable or empty hooks path, a hooks path that is not a directory, or a hooks directory
/// with no `pre-commit` in it.
pub fn commit_gate_armed(root: &std::path::Path) -> Result<std::path::PathBuf, String> {
    if !root.join(".githooks").join("pre-commit").exists() {
        return Err("no .githooks/pre-commit in this tree - nothing to arm".to_string());
    }
    // `rev-parse --git-path hooks` is the effective path git itself will use: it honours
    // core.hooksPath when set and falls back to .git/hooks when not. Asking git beats re-deriving
    // the precedence rules here and getting them subtly wrong.
    let out = git().arg("-C").arg(root).args(["rev-parse", "--git-path", "hooks"]).output();
    let Ok(out) = out else { return Err("git could not be run to resolve the hooks path".to_string()) };
    if !out.status.success() {
        return Err("git could not resolve the hooks path".to_string());
    }
    let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if raw.is_empty() {
        return Err("git resolved an empty hooks path".to_string());
    }
    // The path git reports may be relative to the repo root.
    let p = std::path::Path::new(&raw);
    let dir = if p.is_absolute() { p.to_path_buf() } else { root.join(p) };
    if !dir.is_dir() {
        return Err(format!("core.hooksPath resolves to `{raw}`, which is not a directory - the gate CANNOT run"));
    }
    if !dir.join("pre-commit").exists() {
        return Err(format!("hooks path `{raw}` has no pre-commit - the gate CANNOT run"));
    }
    Ok(dir)
}

#[cfg(test)]
mod tests {
    /// THE CONTROL: no file but this one may construct a git Command directly. The defect was not any
    /// single site — it was 23 individually reasonable sites that added up to an 8%-coverage counter
    /// everyone believed. Same shape as the positional-arg bypass (issue179) and the non-atomic writes
    /// (issue184): the property to pin is "nothing bypasses the helper".
    #[test]
    fn no_git_spawn_bypasses_the_counted_constructor() {
        let needle = format!("Command::new({}git{})", '"', '"');
        let dir = std::path::Path::new("src");
        let mut offenders = Vec::new();
        for entry in std::fs::read_dir(dir).expect("src is readable") {
            let path = entry.expect("entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if name == "gitx.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for (i, line) in text.lines().enumerate() {
                if line.contains(&needle) && !line.trim_start().starts_with("//") {
                    offenders.push(format!("{name}:{}", i + 1));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "raw git spawn(s) outside gitx::git() - the count will understate again: {offenders:?}"
        );
    }

    /// issue410: the perf line reported `git x56 in 100ms` over 4.4 s of spawns because only two
    /// helpers timed. Every run through `Git` now adds to `GIT_NANOS` - unconditionally, which is why this
    /// holds without `KEEL_PERF`; the counter is process-global, so the assertion is that it moved, not by
    /// how much.
    #[test]
    fn a_git_run_through_the_constructor_advances_git_nanos() {
        let before = crate::perf::GIT_NANOS.load(std::sync::atomic::Ordering::Relaxed);
        let _ = super::git().args(["--version"]).output();
        let after = crate::perf::GIT_NANOS.load(std::sync::atomic::Ordering::Relaxed);
        assert!(after > before, "GIT_NANOS did not advance across a spawn");
    }

    /// THE CONTROL for issue240: an unreachable hooks path must read NOT ARMED. The old predicate
    /// ("core.hooksPath is non-empty") answered ARMED for `core.hooksPath = nul`, so the commit gate
    /// was dead while orient/hardening/controls all reported it fail-closed.
    #[test]
    fn an_unreachable_hooks_path_is_not_armed() {
        let tmp = std::env::temp_dir().join(format!("keel-gitx-armed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".githooks")).expect("tmp tree");
        std::fs::write(tmp.join(".githooks").join("pre-commit"), "#!/bin/sh
exit 0
").expect("hook");
        // Not a git repo at all -> git cannot resolve a hooks path -> NOT armed, with a reason.
        let err = super::commit_gate_armed(&tmp).expect_err("a non-repo cannot have an armed gate");
        assert!(!err.is_empty(), "the refusal must name a reason");

        // A tree with no pre-commit at all is also not armed, and says so distinctly.
        let bare = tmp.join("bare");
        std::fs::create_dir_all(&bare).expect("bare");
        let err2 = super::commit_gate_armed(&bare).expect_err("no hook -> not armed");
        assert!(err2.contains("nothing to arm"), "{err2}");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// A CONFIGURED `core.hooksPath` must be REACHABLE — the regression that let issue240 run silently.
    ///
    /// The first version of this test asserted that THIS repo's gate is armed, and my own sprint447
    /// review gate flagged the risk ("it couples a unit test to machine-local git config") and I
    /// overrode it. It was wrong and CI proved it within the hour: a CI checkout configures no
    /// hooksPath and correctly does not need one — CI is the hook-INDEPENDENT layer (K15/D0179) — so
    /// the test failed on every push. Local hook arming is a property of a MACHINE, not of the
    /// repository, and a test can only assert the latter.
    ///
    /// The real invariant is conditional and still catches issue240 exactly: if a hooksPath is
    /// configured at all, it must resolve to a directory holding `pre-commit`. `core.hooksPath = nul`
    /// is configured-but-unreachable and fails; an unconfigured CI checkout is not a violation.
    #[test]
    fn a_configured_hooks_path_must_be_reachable() {
        let root = std::path::Path::new("..");
        let configured = super::git()
            .arg("-C")
            .arg(root)
            .args(["config", "--get", "core.hooksPath"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty());
        let Some(path) = configured else {
            // No hooksPath: nothing claims a local gate, so nothing is broken. CI lands here.
            return;
        };
        match super::commit_gate_armed(root) {
            Ok(dir) => assert!(dir.join("pre-commit").exists(), "armed must mean the hook is there"),
            Err(why) => panic!(
                "core.hooksPath is set to `{path}` but the gate is NOT ARMED: {why} - fix it or unset it; \
                 a configured-but-unreachable hooks path is the issue240 state"
            ),
        }
    }
}
