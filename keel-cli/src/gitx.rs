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
//! WHAT THIS DOES NOT COVER, stated so the numbers stay honest: `GIT_NANOS` (wall time) and `GIT_ARGV`
//! (per-argv tally at `KEEL_PERF=2`) are still recorded only by the two rich helpers, so the TIME
//! attribution remains partial while the COUNT is now total. Count-don't-time is this session's rule;
//! the count is the number that settles arguments.

/// A `git` command, counted. The only permitted constructor — a static test in this module fails the
/// build on any `Command::new("git")` elsewhere in the crate.
#[must_use]
pub fn git() -> std::process::Command {
    crate::perf::add(&crate::perf::GIT_CALLS, 1);
    std::process::Command::new("git")
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
}
