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
    let mut c = Command::new(self_exe());
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
        // The last non-empty lines are where the reason is; a whole guard run is far too much.
        let mut tail: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).rev().take(4).collect();
        tail.reverse();
        Err(tail.join(" | "))
    }
}

/// The full gate an adopting project would actually run.
fn gate(fixture: &Path) -> Result<(), String> {
    let f = fixture.to_string_lossy().to_string();
    keel(&["validate", &f], None).map_err(|e| format!("validate: {e}"))?;
    keel(&["guard", "all", &f], None).map_err(|e| format!("guard: {e}"))?;
    keel(&["check-engine", &f], None).map_err(|e| format!("check-engine: {e}"))?;
    Ok(())
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
pub fn cmd(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("usage: keel adoption-check [ROOT] [--unit NAME] [--keep]");
        println!("  Gates a FOREIGN tree: scaffolds a fresh project, then per unit checks BOTH");
        println!("  directions - that the project gates clean WITHOUT the unit (a control must not");
        println!("  fire on a project that never adopted it), and that the unit lands clean when");
        println!("  freshly imported (it must carry everything it references). issue264/D0231.");
        println!("  --unit NAME  check one unit    --keep  leave the fixture on disk for inspection");
        return 0;
    }
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    let only = args.iter().position(|a| a == "--unit").and_then(|i| args.get(i + 1)).cloned();
    let keep = args.iter().any(|a| a == "--keep");

    let work = std::env::temp_dir().join(format!("keel-adoption-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    let fixture = work.join("fixture");
    let bundles = work.join("bundles");
    if let Err(e) = std::fs::create_dir_all(&bundles) {
        eprintln!("error: cannot create the work directory: {e}");
        return 1;
    }

    println!("adoption-check: scaffolding a FOREIGN project (nothing here has adopted anything)");
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
        let with = match keel(&["process", "import", &bundle.to_string_lossy()], Some(&fixture)) {
            Ok(_) => gate(&fixture),
            Err(e) => Err(format!("import failed: {e}")),
        };
        // Re-sync the generated surface: import changes the registry, and drift here is the
        // adopter's next command, not a defect in the unit.
        let _ = keel(&["sync-claude"], Some(&fixture));
        verdicts.push(UnitVerdict { unit: unit.clone(), without, with });
    }

    let mut failed = 0usize;
    println!("{:<28} {:<38} WITH it (freshly imported)", "UNIT", "WITHOUT it (never adopted)");
    for v in &verdicts {
        let f = |r: &Result<(), String>| match r {
            Ok(()) => "clean".to_string(),
            Err(e) => format!("FAIL {}", e.chars().take(70).collect::<String>()),
        };
        if v.without.is_err() || v.with.is_err() {
            failed += 1;
        }
        println!("{:<28} {:<38} {}", v.unit, f(&v.without), f(&v.with));
    }
    println!();
    if keep {
        println!("fixture kept at {}", fixture.display());
    } else {
        let _ = std::fs::remove_dir_all(&work);
    }
    if failed == 0 {
        println!("adoption-check: {} unit(s) — every one gates clean both without it and freshly imported.", verdicts.len());
        0
    } else {
        println!("adoption-check: {failed} of {} unit(s) FAILED. A unit that cannot land in a project that lacks", verdicts.len());
        println!("it is not transferable, and a control that fires on a project which never adopted it is the");
        println!("D0164 failure. Neither is visible from inside this repository (issue264).");
        1
    }
}

#[cfg(test)]
mod tests {
    use super::{bundle_files, strip_unit};

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
