//! A migration fixture that looks like a REAL project, not a fresh scaffold (D0275).
//!
//! `F9` in `federation.rs` already proves a fresh scaffold migrates: refusals fire, the pin is
//! re-stamped, the project's pin comment survives, adoption survives, the gate is green after. That
//! is the EASY case — and it is how seven defects got past it (issue301, issue310, issue314,
//! issue323, issue324, issue326, issue327, none of which was found by a test).
//!
//! A real project differs in four ways, each of which produced one of those defects:
//!   1. it CUSTOMISED an engine-shipped file;
//!   2. it authored its OWN skill naming an engine command;
//!   3. it holds an untracked obligation record while behind its pin;
//!   4. it has its own CI workflow.
//!
//! Cases 1-3 are here. Case 4 is deliberately NOT: issue326 and issue327 are still open, so a test
//! for it would have to assert the defect (locking it in) or be ignored (a test that does not run is
//! a claim, D0253). It lands with the fix, in step 5 of the D0275 plan.
//!
//! SHORT PATHS ARE LOAD-BEARING HERE. The first attempt at this fixture died on Windows MAX_PATH
//! mid-`git add`, which left the tree uncommitted, which made migrate refuse — a failure that looked
//! like a migration bug and was a path-length bug (the issue313 class). The fixture roots itself
//! shallowly on purpose.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn run(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(dir).output().expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn git(dir: &Path, args: &[&str]) -> bool {
    Command::new("git").arg("-C").arg(dir).args(args).output().is_ok_and(|o| o.status.success())
}

/// A deliberately SHALLOW root — see the module note on MAX_PATH.
fn shallow_root(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("r{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir shallow root");
    root
}

/// A project that has been LIVED IN: behind its pin, with local content of its own.
/// Returns `(root, the engine file it customised)`.
fn realistic_project(tag: &str) -> (PathBuf, PathBuf) {
    let root = shallow_root(tag);
    assert!(run(&root, &["init", "."]).0, "scaffold");
    assert!(git(&root, &["init", "-q"]), "git init");
    assert!(git(&root, &["config", "user.email", "r@example.invalid"]), "email");
    assert!(git(&root, &["config", "user.name", "r"]), "name");

    // Behind the pin — the precondition for a migration existing at all.
    let pin = root.join(".engine/contracts/engine-version.toml");
    let text = std::fs::read_to_string(&pin).expect("pin");
    let stale: String = text
        .lines()
        .map(|l| if l.trim_start().starts_with("engine") { "engine = \"0.0.1\"".to_string() } else { l.to_string() })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&pin, format!("{stale}\n")).expect("stale pin");

    // (1) A CUSTOMISED engine-shipped file.
    let customised = root.join(".engine/skills/actor-enrollment/SKILL.md");
    let original = std::fs::read_to_string(&customised).expect("an engine skill ships");
    std::fs::write(&customised, format!("{original}\n<!-- OUR LOCAL CUSTOMISATION -->\n")).expect("customise");

    // (2) The project's OWN skill, naming an engine command by name — the D0273 case.
    let own = root.join(".engine/skills/ours");
    std::fs::create_dir_all(&own).expect("mkdir own skill");
    std::fs::write(own.join("SKILL.md"), "# ours\nRun `keel orphans` to find loose ends.\n").expect("own skill");

    assert!(git(&root, &["add", "-A"]), "stage the lived-in state");
    assert!(
        git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "a project people have used"]),
        "commit the lived-in state"
    );
    (root, customised)
}

fn cleanup(root: &Path) {
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn a_resync_that_replaces_a_customised_engine_file_says_which_files_it_wrote() {
    let (root, customised) = realistic_project("cust");
    assert!(
        std::fs::read_to_string(&customised).expect("read").contains("OUR LOCAL CUSTOMISATION"),
        "precondition: the customisation is there before the migration"
    );

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(ok, "a realistic project must still migrate: {text}");

    // The POLICY is that engine files belong to the engine, so replacement is allowed. What is not
    // allowed is silence: before issue328 the entire output was "wrote 2 file(s)".
    assert!(
        text.contains(".engine/skills/actor-enrollment/SKILL.md"),
        "migrate must NAME the engine files it wrote, so a project can see that its local edit is \
         gone and `git diff` it. A count cannot be acted on:\n{text}"
    );
    assert!(
        text.to_uppercase().contains("REVIEW"),
        "and say plainly that these want reviewing before the commit:\n{text}"
    );
    cleanup(&root);
}

#[test]
fn a_projects_own_skill_naming_an_engine_command_survives_the_migration() {
    let (root, _) = realistic_project("own");
    let own = root.join(".engine/skills/ours/SKILL.md");

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(ok, "migrate: {text}");
    let after = std::fs::read_to_string(&own).expect("the project's own skill still exists");
    assert!(
        after.contains("keel orphans"),
        "a skill the PROJECT authored is not engine content and a resync must not touch it — losing \
         it would be data loss, not a policy call:\n{after}"
    );
    // And it must still gate: a migration leaving a project un-gateable is the partial migration
    // D0067 calls the most expensive outcome.
    let (gate_ok, gate) = run(&root, &["gate", "--fast", "."]);
    assert!(gate_ok, "the migrated realistic project gates green: {gate}");
    cleanup(&root);
}

#[test]
fn a_lived_in_project_holding_an_obligation_record_still_migrates() {
    let (root, _) = realistic_project("oblig");
    // The issue324 deadlock, now in a project with customisations and its own content rather than a
    // bare one — the combination nothing covered.
    let dir = root.join(".tracking").join("obligations");
    std::fs::create_dir_all(&dir).expect("mkdir");
    std::fs::write(dir.join("red-yield-abc12345.sysml"), "// OBLIGATION\npackage ObligationAbc {\n}\n")
        .expect("obligation");

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(ok, "the issue324 escape must hold on a REALISTIC project, not only a bare one: {text}");
    assert!(text.contains("red-yield-abc12345"), "and still name what it tolerated: {text}");
    cleanup(&root);
}
