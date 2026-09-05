//! `keel adoption-check` — gate a FOREIGN tree, because that is where the defects actually are.
//!
//! WHY THIS EXISTS (issue264/D0231). Measured over this project's own defect record: 39 of 51 guards
//! read only the authored model, 11 cross some boundary, and **zero** read another project's tree —
//! while 11 of the last 20 issues were discovered in the field, clustering at exactly that boundary.
//! Verification reports 99% examined and 91% exercised, and 5,120 recorded results carry a 3.9% fail
//! rate. The measures are neither weak nor mis-thresholded. They point INWARD, at the artifact this
//! project authors, and nothing breaks there any more.
//!
//! WHAT IT CATCHES, tested rather than claimed. `tests/adoption_check_can_fail.rs` reproduces the
//! largest historical class and asserts the foreign gate goes RED: a unit whose skill→process
//! BINDING stays home (issue252/issue253 — 23 of 24 units would have landed red).
//!
//! WHAT THIS CANNOT CATCH, and it is most of what actually happened. The fixture is a CURRENT keel
//! scaffold, so it is keel adopting keel — a weaker foreign-ness than a real adopter:
//!
//! - **issue263** (a unit asserting a constraint it does not carry) is INVISIBLE here, because
//!   `keel init` ships `.engine/rules/guard-constraints.sysml`, so the symbol resolves in the
//!   fixture. penumbra lacked it only because penumbra is on an older VINTAGE.
//! - **issue259** (a control firing on a project that never adopted it) is INVISIBLE for the same
//!   reason: every process in the fixture already carries the current conventions, so a
//!   convention-based guard has nothing to fire on.
//!
//! I first wrote that this check would have caught five defects. Testing the claim showed two.
//! Representing an older vintage needs a second fixture pinned to a real prior release, which this
//! is not — recorded as its own gap rather than left implied.
//!
//! TWO DIRECTIONS PER UNIT, because the failures run both ways:
//!
//! 1. **WITHOUT** the unit — a project that never adopted it must still gate clean. This is the
//!    issue259 direction: a control that fires on projects which never opted in.
//! 2. **WITH** the unit, freshly imported — it must land clean. This is the issue251/252/263
//!    direction: a unit that does not carry everything it references.
//!
//! It drives the REAL CLI by re-invoking this binary, not internal functions: an adopter runs
//! commands, so the commands are what must work. `restore_dst` is shared with `import` rather than
//! reimplemented — a fixture that resolved paths differently would test something nobody runs.
use std::path::{Path, PathBuf};
use std::process::Command;

/// One unit's two verdicts.
struct UnitVerdict {
    unit: String,
    /// Did the fixture gate clean with the unit REMOVED?
    without: Result<(), String>,
    /// Did it gate clean after importing the unit back?
    with: Result<(), String>,
}

fn self_exe() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("keel"))
}

/// Run `keel <args>` and return Err(tail of output) on a non-zero exit.
fn keel(args: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    keel_with(&self_exe(), args, cwd)
}

/// Run a keel binary - THIS one, or a prior release's (D0302): the vintage fixture is scaffolded,
/// imported into and gated by the OLD binary, because that is what an adopter on that vintage runs.
fn keel_with(exe: &Path, args: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    let mut c = Command::new(exe);
    c.args(args);
    if let Some(d) = cwd {
        c.current_dir(d);
    }
    let out = match c.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("could not run `keel {}`: {e}", args.join(" "))),
    };
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    if out.status.success() {
        Ok(text)
    } else {
        // The FAILING lines are the reason, not the last four: `guard all` prints every guard's verdict
        // and the last four are almost always PASS lines from guards that came after the failure - which
        // is what the first vintage run reported, a "failure" made of passes (GH#45, issue350).
        let failing = failing_lines(&text);
        if !failing.is_empty() {
            return Err(failing.join(" | "));
        }
        let mut tail: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).rev().take(4).collect();
        tail.reverse();
        Err(tail.join(" | "))
    }
}

/// The lines of a gate's output that ARE the failure, by the shape of the line and never by substring
/// (issue350 / GH#45; the issue329 class): a guard verdict line `[guard:<name>] <VERDICT> — ...` whose
/// verdict token is FAIL, and a detail line whose first token is `ERROR` (a violation, or a
/// validate/check-engine error). A PASS or WARN verdict is never a reason, whatever its text says -
/// a warning that mentions a FAILED sprint gate is a passing guard talking. The `[guard] FAILED` total
/// is a count, not a reason, and is left out. At most four, in output order.
fn failing_lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| line_is_failure(l)).take(4).collect()
}

fn line_is_failure(line: &str) -> bool {
    let t = line.trim_start();
    if let Some(rest) = t.strip_prefix("[guard:") {
        // `[guard:name] VERDICT — ...` - the verdict is the first token after the bracket.
        return rest.split_once("] ").is_some_and(|(_, after)| after.split_whitespace().next() == Some("FAIL"));
    }
    if t.starts_with("[guard]") {
        return false;
    }
    let first = t.split(|c: char| c.is_whitespace() || c == ':').next().unwrap_or("");
    first == "ERROR"
}

#[cfg(test)]
mod verdict_tests {
    use super::{failing_lines, line_is_failure};

    /// GH#45: a PASS line is never the reason, even when its warning text says FAIL; a FAIL verdict
    /// and an ERROR detail are; the total line is a count.
    #[test]
    fn a_pass_line_mentioning_fail_is_not_a_failure_and_a_fail_verdict_is() {
        assert!(!line_is_failure("[guard:sprint-closure] PASS — 12 scanned, 1 warning(s), 0 violation(s)"));
        assert!(!line_is_failure("  WARN  sprint gate FAILED for storyX - unstamped (D0260)"));
        assert!(!line_is_failure("[guard] FAILED — 3 warning(s) across 2 guard(s), NOT violations"));
        assert!(line_is_failure("[guard:claude-surface-drift] FAIL — 47 scanned, 0 warning(s), 1 violation(s)"));
        assert!(line_is_failure("  ERROR 1 skill(s) missing or stale under .claude/skills/"));
        assert!(line_is_failure("ERROR: .tracking/x.sysml:4 — unresolved type reference `Person`"));
        assert!(!line_is_failure("everything is fine, no errors here"));
    }

    /// The reason reported is the failing lines in order, capped, and never a pass.
    #[test]
    fn the_reason_is_the_failing_lines_in_order() {
        let text = "[guard:a] PASS — 1 scanned, 0 warning(s), 0 violation(s)\n  ERROR x is wrong\n[guard:b] FAIL — 1 scanned, 0 warning(s), 1 violation(s)\n[guard:c] PASS — 2 scanned, 1 warning(s), 0 violation(s)\n[guard] FAILED — 1 warning(s) across 1 guard(s)";
        assert_eq!(failing_lines(text), vec!["  ERROR x is wrong", "[guard:b] FAIL — 1 scanned, 0 warning(s), 1 violation(s)"]);
        assert!(failing_lines("[guard:a] PASS — 1 scanned, 0 warning(s), 0 violation(s)\n[guard] ALL PASS").is_empty());
    }
}

/// The full gate an adopting project would actually run.
fn gate(fixture: &Path) -> Result<(), String> {
    gate_with(&self_exe(), fixture)
}

fn gate_with(exe: &Path, fixture: &Path) -> Result<(), String> {
    let f = fixture.to_string_lossy().to_string();
    keel_with(exe, &["validate", &f], None).map_err(|e| format!("validate: {e}"))?;
    keel_with(exe, &["guard", "all", &f], None).map_err(|e| format!("guard: {e}"))?;
    keel_with(exe, &["check-engine", &f], None).map_err(|e| format!("check-engine: {e}"))?;
    Ok(())
}

/// The release asset name for this platform, mirroring `release.yml` and `keelw`.
const fn release_asset() -> &'static str {
    if cfg!(windows) {
        "keel-windows-x86_64.exe"
    } else if cfg!(target_os = "macos") {
        "keel-macos-aarch64"
    } else {
        "keel-linux-x86_64"
    }
}

/// A prior RELEASE's binary, cached or fetched - obtained, never synthesised (D0302, issue265).
///
/// From keelw's machine-local cache (`.keel/bin/<version>/`) or fetched from the release origin.
/// `KEELW_BASE_URL` overrides the origin as it does for keelw; `KEEL_OFFLINE` refuses to fetch and
/// says so. A missing binary is an ERROR the caller reports, never a silent fall-through to the
/// current one - a "vintage" verdict computed by this binary would be the current fixture twice.
///
/// # Errors
/// Offline with an empty cache, an unreachable origin, a binary that does not run, or one that reports
/// a different version than asked for.
pub fn vintage_binary(root: &Path, version: &str) -> Result<PathBuf, String> {
    let dir = root.join(".keel").join("bin").join(version);
    let exe = dir.join(if cfg!(windows) { "keel.exe" } else { "keel" });
    if exe.is_file() {
        return Ok(exe);
    }
    if std::env::var_os("KEEL_OFFLINE").is_some() {
        return Err(format!("v{version} is not cached at {} and KEEL_OFFLINE is set - the vintage fixture cannot be built offline", dir.display()));
    }
    let base = std::env::var("KEELW_BASE_URL").unwrap_or_else(|_| "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases/download".to_string());
    let url = format!("{base}/v{version}/{}", release_asset());
    std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    let out = Command::new("curl").args(["-fsSL", "-o", &exe.to_string_lossy(), &url]).output().map_err(|e| format!("curl is not available ({e}); fetch {url} into {} by hand", exe.display()))?;
    if !out.status.success() {
        let _ = std::fs::remove_file(&exe);
        return Err(format!("could not fetch {url}: {}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&exe, std::fs::Permissions::from_mode(0o755));
    }
    // Prove it runs and IS that version before trusting a single verdict from it.
    let v = keel_with(&exe, &["version"], None).map_err(|e| format!("the fetched binary does not run: {e}"))?;
    if !v.contains(version) {
        return Err(format!("the fetched binary reports `{}`, not v{version}", v.lines().next().unwrap_or_default()));
    }
    Ok(exe)
}

/// THE VINTAGE FIXTURE (D0302): the same two directions, on a project scaffolded by the OLD binary and
/// gated by it, with the unit exported from THIS engine. issue263 (a unit asserting a constraint the
/// old rules file does not define) and issue259 (a convention-based guard firing on old-style
/// processes) are visible only here.
fn vintage_pass(v: &str, exe: &Path, work: &Path, bundles: &Path, units: &[String]) -> Result<Vec<UnitVerdict>, String> {
    let vfixture = work.join(format!("fixture-v{v}"));
    eprintln!("  vintage v{v}: scaffolding with {}", exe.display());
    keel_with(exe, &["init", &vfixture.to_string_lossy()], None).map_err(|e| format!("the v{v} scaffold itself failed: {e}"))?;
    gate_with(exe, &vfixture).map_err(|e| format!("a FRESH v{v} scaffold does not gate clean under its own binary, so no vintage verdict could mean anything: {e}"))?;
    let total = units.len();
    let mut out = Vec::new();
    for (i, unit) in units.iter().enumerate() {
        eprintln!("  [v{v} {}/{total}] {unit}", i + 1);
        let bundle = bundles.join(unit).join(unit);
        if !bundle.is_dir() {
            out.push(UnitVerdict { unit: unit.clone(), without: Err("no bundle (export failed above)".into()), with: Err("not attempted".into()) });
            continue;
        }
        let removed = strip_unit(&vfixture, &bundle);
        let without = if removed.is_empty() {
            Err("nothing was removed - the vintage fixture never carried this unit's files (it may not have existed at that vintage), so the WITHOUT direction was not tested".into())
        } else {
            gate_with(exe, &vfixture)
        };
        let with = match keel_with(exe, &["process", "import", &bundle.to_string_lossy()], Some(&vfixture)) {
            Ok(_) => {
                let _ = keel_with(exe, &["sync-claude"], Some(&vfixture));
                gate_with(exe, &vfixture)
            }
            Err(e) => Err(format!("import failed: {e}")),
        };
        out.push(UnitVerdict { unit: unit.clone(), without, with });
    }
    Ok(out)
}

/// Print both fixtures' tables - CURRENT first, VINTAGE second, never merged - and count failures.
fn report(verdicts: &[UnitVerdict], vintage: Option<&str>, vintage_verdicts: &[UnitVerdict]) -> usize {
    let f = |r: &Result<(), String>| match r {
        Ok(()) => "clean".to_string(),
        Err(e) => {
            let mut s = e.clone();
            s.truncate(120);
            format!("FAIL: {s}")
        }
    };
    let mut failed = 0usize;
    println!("fixture: CURRENT ({})", env!("CARGO_PKG_VERSION"));
    println!("{:<28} {:<38} WITH it (freshly imported)", "UNIT", "WITHOUT it (never adopted)");
    for v in verdicts {
        if v.without.is_err() || v.with.is_err() {
            failed += 1;
        }
        println!("{:<28} {:<38} {}", v.unit, f(&v.without), f(&v.with));
    }
    println!();
    if let Some(v) = vintage {
        println!("fixture: VINTAGE v{v} (release asset, scaffolded and gated by that binary)");
        println!("{:<28} {:<38} WITH it (freshly imported)", "UNIT", "WITHOUT it (never adopted)");
        for vv in vintage_verdicts {
            if vv.without.is_err() || vv.with.is_err() {
                failed += 1;
            }
            println!("{:<28} {:<38} {}", vv.unit, f(&vv.without), f(&vv.with));
        }
        println!();
    }
    failed
}

/// Bundle-relative paths a unit carries, read from the exported bundle itself rather than recomputed.
fn bundle_files(bundle: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, base: &Path, out: &mut Vec<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, base, out);
            } else if p.file_name().is_some_and(|n| n != "unit.toml") {
                if let Ok(rel) = p.strip_prefix(base) {
                    out.push(rel.to_path_buf());
                }
            }
        }
    }
    let mut out = Vec::new();
    walk(bundle, bundle, &mut out);
    out.sort();
    out
}

/// Remove a unit's files from the fixture, so the fixture becomes a project that never adopted it.
/// Returns what was removed, so the report can say when a unit had nothing to remove.
fn strip_unit(fixture: &Path, bundle: &Path) -> Vec<PathBuf> {
    let mut gone = Vec::new();
    for rel in bundle_files(bundle) {
        let dst = crate::process_cmd::restore_dst(fixture, &rel);
        if dst.is_file() && std::fs::remove_file(&dst).is_ok() {
            gone.push(rel);
        }
    }
    // A skill directory left empty is not "absent" to a directory-walking reader, so clear it out.
    let skills = fixture.join(".engine").join("skills");
    if let Ok(rd) = std::fs::read_dir(&skills) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() && std::fs::read_dir(&p).is_ok_and(|mut r| r.next().is_none()) {
                let _ = std::fs::remove_dir(&p);
            }
        }
    }
    gone
}

/// `keel adoption-check [ROOT] [--unit NAME] [--keep]`.
fn usage() {
    println!("usage: keel adoption-check [ROOT] [--unit NAME] [--keep] [--vintage VERSION]");
    println!("  Gates a FOREIGN tree: scaffolds a fresh project, then per unit checks BOTH");
    println!("  directions - that the project gates clean WITHOUT the unit (a control must not");
    println!("  fire on a project that never adopted it), and that the unit lands clean when");
    println!("  freshly imported (it must carry everything it references). issue264/D0231.");
    println!("  --unit NAME  check one unit    --keep  leave the fixture on disk for inspection");
    println!("  --vintage VERSION  ALSO gate every unit against a fixture scaffolded, imported into and");
    println!("                     gated by that prior RELEASE's binary (fetched, cached under .keel/bin/) -");
    println!("                     the adopter on an older vintage that the current fixture cannot stand in");
    println!("                     for (issue265/D0302). Verdicts are reported per fixture, never merged.");
}

fn summary(failed: usize, units: usize) -> i32 {
    if failed == 0 {
        println!("adoption-check: {units} unit(s) — every one gates clean both without it and freshly imported.");
        0
    } else {
        println!("adoption-check: {failed} of {units} unit(s) FAILED. A unit that cannot land in a project that lacks");
        println!("it is not transferable, and a control that fires on a project which never adopted it is the");
        println!("D0164 failure. Neither is visible from inside this repository (issue264).");
        1
    }
}

pub fn cmd(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return 0;
    }
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    let only = args.iter().position(|a| a == "--unit").and_then(|i| args.get(i + 1)).cloned();
    let keep = args.iter().any(|a| a == "--keep");
    let vintage = args.iter().position(|a| a == "--vintage").and_then(|i| args.get(i + 1)).cloned();

    let work = std::env::temp_dir().join(format!("keel-adoption-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    let fixture = work.join("fixture");
    let bundles = work.join("bundles");
    if let Err(e) = std::fs::create_dir_all(&bundles) {
        eprintln!("error: cannot create the work directory: {e}");
        return 1;
    }

    // The vintage binary is resolved FIRST: a requested vintage that cannot be obtained is a failed
    // check, not a check that quietly ran on the current fixture alone.
    let vintage_exe = match &vintage {
        Some(v) => match vintage_binary(&root, v) {
            Ok(exe) => Some((v.clone(), exe)),
            Err(e) => {
                eprintln!("error: vintage v{v} unavailable - {e}");
                return 1;
            }
        },
        None => None,
    };
    println!("adoption-check: scaffolding a FOREIGN project (nothing here has adopted anything)");
    println!("  SCOPE: this verifies what EXPORT produces, not what any real target received (issue290); a");
    println!("  target's own `keel guard unit-extras-present` is what checks its mechanism files are there.");
    if let Err(e) = keel(&["init", &fixture.to_string_lossy()], None) {
        eprintln!("error: the scaffold itself failed: {e}");
        return 1;
    }
    // A fixture that starts dirty makes every later verdict meaningless.
    if let Err(e) = gate(&fixture) {
        eprintln!("error: a FRESH scaffold does not gate clean, so no unit verdict below could mean");
        eprintln!("       anything: {e}");
        return 1;
    }
    println!("adoption-check: baseline clean — a fresh scaffold gates green\n");

    let units: Vec<String> =
        only.as_ref().map_or_else(|| crate::activation::declared_processes(&root), |u| vec![u.clone()]);
    if units.is_empty() {
        eprintln!("error: no declared processes found under {} — refusing to report a pass over an", root.display());
        eprintln!("       empty population (the vacuous-pass class this project has shipped twice)");
        return 1;
    }

    let mut verdicts = Vec::new();
    let total = units.len();
    for (i, unit) in units.iter().enumerate() {
        // Progress goes to stderr as it happens. A check that prints nothing for minutes and then a
        // table is indistinguishable from a hung one - which is exactly how this looked on its first
        // run, and a CI job nobody can read the progress of is a CI job nobody trusts.
        eprintln!("  [{}/{total}] {unit}", i + 1);
        let bundle_root = bundles.join(unit);
        if let Err(e) = keel(&["process", "export", unit, "--out", &bundle_root.to_string_lossy()], Some(&root)) {
            verdicts.push(UnitVerdict {
                unit: unit.clone(),
                without: Err(format!("export failed: {e}")),
                with: Err("not attempted".into()),
            });
            continue;
        }
        let bundle = bundle_root.join(unit);
        let removed = strip_unit(&fixture, &bundle);
        let without = if removed.is_empty() {
            Err("nothing was removed — the fixture never carried this unit's files, so the WITHOUT direction was not actually tested".into())
        } else {
            gate(&fixture)
        };
        // Re-sync the generated surface BEFORE gating: import changes the registry, and `sync-claude`
        // is the adopter's documented next command, not a defect in the unit. Found by the vintage
        // fixture: a v0.3.0 import does not sync on its own and that binary's drift guard failed every
        // unit for it - a verdict about the engine's vintage, which this check is not asking.
        let with = match keel(&["process", "import", &bundle.to_string_lossy()], Some(&fixture)) {
            Ok(_) => {
                let _ = keel(&["sync-claude"], Some(&fixture));
                gate(&fixture)
            }
            Err(e) => Err(format!("import failed: {e}")),
        };
        verdicts.push(UnitVerdict { unit: unit.clone(), without, with });
    }

    let vintage_verdicts: Vec<UnitVerdict> = match &vintage_exe {
        Some((v, exe)) => match vintage_pass(v, exe, &work, &bundles, &units) {
            Ok(vv) => vv,
            Err(e) => {
                eprintln!("error: {e}");
                return 1;
            }
        },
        None => Vec::new(),
    };

    let failed = report(&verdicts, vintage_exe.as_ref().map(|(v, _)| v.as_str()), &vintage_verdicts);
    if keep {
        println!("fixture kept at {}", fixture.display());
    } else {
        let _ = std::fs::remove_dir_all(&work);
    }
    summary(failed, verdicts.len())
}

#[cfg(test)]
mod tests {
    use super::{bundle_files, strip_unit, vintage_binary};

    /// D0302: a vintage binary is OBTAINED, never synthesised and never silently the current one -
    /// offline with an empty cache is a named refusal; a cached binary is used without a fetch; a
    /// cached file that is not that version is refused too (so a swapped asset cannot pose as a vintage).
    #[test]
    fn vintage_binary_refuses_offline_and_uses_the_cache() {
        let root = std::env::temp_dir().join(format!("keel-vintage-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir");
        std::env::set_var("KEEL_OFFLINE", "1");
        let err = vintage_binary(&root, "0.0.1").expect_err("offline + empty cache refuses");
        assert!(err.contains("KEEL_OFFLINE") && err.contains("0.0.1"), "{err}");
        // a cached binary is used as-is (the version check runs only on a fresh fetch)
        let dir = root.join(".keel").join("bin").join("0.0.1");
        std::fs::create_dir_all(&dir).expect("cache dir");
        let exe = dir.join(if cfg!(windows) { "keel.exe" } else { "keel" });
        std::fs::write(&exe, b"not really a binary").expect("cached");
        assert_eq!(vintage_binary(&root, "0.0.1").expect("cached path wins"), exe);
        std::env::remove_var("KEEL_OFFLINE");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_bundles_own_file_list_drives_the_strip_and_excludes_the_manifest() {
        // The file list comes from the BUNDLE, not from a second computation over the source tree -
        // two lists would drift, and the fixture would then strip something the importer never
        // restores (or miss something it does), which is the same-fact-in-two-places class.
        let root = std::env::temp_dir().join("keel-adoption-unit-test");
        let _ = std::fs::remove_dir_all(&root);
        let b = root.join("bundle");
        std::fs::create_dir_all(b.join("processes")).unwrap();
        std::fs::create_dir_all(b.join("skills").join("s")).unwrap();
        std::fs::create_dir_all(b.join(".github").join("workflows")).unwrap();
        std::fs::write(b.join("unit.toml"), "x").unwrap();
        std::fs::write(b.join("processes").join("u.sysml"), "x").unwrap();
        std::fs::write(b.join("skills").join("s").join("SKILL.md"), "x").unwrap();
        std::fs::write(b.join(".github").join("workflows").join("w.yml"), "x").unwrap();

        let files = bundle_files(&b);
        assert_eq!(files.len(), 3, "unit.toml is the manifest, not a carried file: {files:?}");

        // Engine files land under .engine/; an EXTRA keeps its repo-relative path. Strip must follow
        // the importer exactly, so build a fixture the way import would have left it.
        let fx = root.join("fixture");
        for rel in &files {
            let dst = crate::process_cmd::restore_dst(&fx, rel);
            std::fs::create_dir_all(dst.parent().unwrap()).unwrap();
            std::fs::write(&dst, "x").unwrap();
        }
        assert!(fx.join(".engine").join("processes").join("u.sysml").is_file());
        assert!(fx.join(".github").join("workflows").join("w.yml").is_file(), "an extra stays repo-relative");

        let gone = strip_unit(&fx, &b);
        assert_eq!(gone.len(), 3, "every carried file is removed, extras included: {gone:?}");
        assert!(!fx.join(".engine").join("skills").join("s").exists(), "an emptied skill dir must not linger");
        let _ = std::fs::remove_dir_all(&root);
    }
}
