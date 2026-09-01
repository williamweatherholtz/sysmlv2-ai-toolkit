//! The control for issue323 — a unit's identity belongs to the UNIT, not to whoever exports it.
//!
//! WHAT WENT WRONG, and why a test is the only honest fix. This project imported `exec-summary` from
//! the library, improved its SKILL.md, and re-published. The content shipped. The identity did not
//! survive: export resolved the unit id from `.engine/contracts/unit-ids.toml` — the registry of ids
//! this project MINTED — found nothing, minted a fresh one, and wrote it into the library's
//! `unit.toml`, replacing the id every consumer had installed. `next_unit_version` then looked the new
//! id up in the install record, missed, took its `None => 1` arm, and restarted the version at 1.
//!
//! So the published unit CHANGED CONTENT, CHANGED IDENTITY, and KEPT ITS VERSION NUMBER. That is worse
//! than a numbering bug: a consumer comparing versions to decide whether it is behind — exactly what
//! `keel status`'s drift section does against `srDriftIsReportedPerUnit` — reads "v1 installed, v1
//! available" and is told it is CURRENT. The report does not miss the update; it affirms there is none.
//!
//! WHY THESE CASES. `identity_survives_a_republish_from_an_importing_project` is the round trip
//! issue323 named as the test that "would have failed today", and it is the whole defect in one run.
//! The other two cover the arms that made it silent rather than loud: a forked identity must refuse
//! rather than pick, and a version lookup that misses on a process the project demonstrably HAS must
//! refuse rather than restart — because `None` conflated "new unit" with "I could not find it", and a
//! wrong version is not recoverable once consumers have compared against it (D0253: a control never
//! observed to fail is a claim, so each of these asserts the refusal, not just the happy path).

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

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn keel_home(home: &Path, cwd: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .env("USERPROFILE", home)
        .env("HOME", home)
        .current_dir(cwd)
        .output()
        .expect("keel runs");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

/// A minimal keel project holding one exportable, guard-less process.
fn project(at: &Path, process: &str) {
    std::fs::create_dir_all(at.join(".engine").join("processes")).expect("mkdir");
    std::fs::create_dir_all(at.join(".engine").join("skills").join(process)).expect("mkdir");
    std::fs::create_dir_all(at.join(".tracking")).expect("mkdir");
    std::fs::write(at.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::write(
        at.join(".engine").join("processes").join(format!("{process}.sysml")),
        "// tiny\npackage ProcessTiny {\n}\n",
    )
    .expect("process");
    std::fs::write(at.join(".engine").join("skills").join(process).join("SKILL.md"), "# tiny\nv-one\n").expect("skill");
    std::fs::write(
        at.join(".engine").join("skills").join(process).join("registry.sysml"),
        "package SkillsRegistryTiny {\n}\n",
    )
    .expect("registry");
}

/// An isolated HOME with an initialised library, plus a PRODUCER project that has published
/// `tiny-process` into it. Returns `(base, home, producer, clone)`.
fn published(tag: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("keel-ident-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    std::fs::create_dir_all(&home).expect("mkdir");
    let bare = base.join("library.git");
    std::fs::create_dir_all(&bare).expect("mkdir");
    assert!(Command::new("git").args(["init", "-q", "--bare"]).arg(&bare).output().expect("bare").status.success());
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    let clone = home.join(".keel").join("library");
    git(&clone, &["config", "user.email", "ident@example.invalid"]);
    git(&clone, &["config", "user.name", "ident"]);

    let producer = base.join("producer");
    project(&producer, "tiny-process");
    let (ok, text) = keel_home(&home, &producer, &["process", "publish", "tiny-process"]);
    assert!(ok, "the producer must be able to publish v1: {text}");
    (base, home, producer, clone)
}

/// The unit's `unitId` and `version` as the library currently holds them.
fn library_identity(clone: &Path) -> (String, String) {
    let manifest = std::fs::read_to_string(clone.join("tiny-process").join("unit.toml")).expect("unit.toml");
    let field = |key: &str| -> String {
        manifest
            .lines()
            .find_map(|l| l.trim().strip_prefix(&format!("{key} = "))?.trim().trim_matches('"').to_string().into())
            .unwrap_or_else(|| panic!("unit.toml has no {key}:\n{manifest}"))
    };
    (field("unitId"), field("version"))
}

#[test]
fn identity_survives_a_republish_from_an_importing_project() {
    let (base, home, _producer, clone) = published("roundtrip");
    let (original_id, original_version) = library_identity(&clone);
    assert_eq!(original_version, "1", "the producer's first publish is v1");

    // A DIFFERENT project imports the unit, improves one file, and publishes the improvement.
    let consumer = base.join("consumer");
    std::fs::create_dir_all(consumer.join(".tracking")).expect("mkdir");
    std::fs::write(consumer.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::create_dir_all(consumer.join(".engine")).expect("mkdir");
    let (ok, text) = keel_home(&home, &consumer, &["process", "import", "--from-library", "tiny-process"]);
    assert!(ok, "the consumer must be able to import the unit: {text}");

    // The improvement goes into the file the unit actually SHIPS. This minimal process declares no
    // skill, so its one unit file is the process definition — editing anything else would move bytes
    // the export never looks at, and the version would then correctly refuse to advance.
    let definition = consumer.join(".engine").join("processes").join("tiny-process.sysml");
    std::fs::write(&definition, "// tiny\n// AND THE IMPROVEMENT THE CONSUMER MADE\npackage ProcessTiny {\n}\n")
        .expect("improve");

    let (ok, text) = keel_home(&home, &consumer, &["process", "publish", "tiny-process"]);
    assert!(ok, "publishing an improvement to an IMPORTED unit must succeed: {text}");

    let (id_after, version_after) = library_identity(&clone);
    assert_eq!(
        id_after, original_id,
        "THE WHOLE DEFECT (issue323): a unit's identity belongs to the unit, not to the project \
         exporting it. Re-minting on republish orphans every install — consumers match on this id, \
         so a changed one means the unit they installed no longer exists."
    );
    assert_eq!(
        version_after, "2",
        "the content moved, so the version MUST advance. Restarting at v1 is what made the failure \
         silent: a consumer's drift check then AFFIRMS 'nothing behind' while the content differs."
    );
    // And the improvement really did travel — a version bump over unchanged bytes would be its own lie.
    let published = std::fs::read_to_string(clone.join("tiny-process").join("processes").join("tiny-process.sysml"))
        .expect("published definition");
    assert!(
        published.contains("AND THE IMPROVEMENT THE CONSUMER MADE"),
        "the content the version claims must be the content that shipped:\n{published}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_process_recorded_under_two_unit_ids_refuses_to_export() {
    let (base, home, producer, _clone) = published("forked");
    // The corrupted state issue323 left behind: ONE process name filed under TWO ids. Reproduced by
    // appending a second section rather than by re-running the bug, so the refusal is tested against
    // the state itself and stays valid after the bug is gone.
    let record = producer.join(".engine").join("contracts").join("installed-units.toml");
    let mut text = std::fs::read_to_string(&record).expect("install record");
    text.push_str("\n[deadbeef-0000-0000-0000-000000000000]\nprocess = \"tiny-process\"\nversion = 1\n");
    std::fs::write(&record, text).expect("fork the identity");

    let (ok, out) = keel_home(&home, &producer, &["process", "export", "tiny-process", "--out", &base.join("out").to_string_lossy()]);
    assert!(!ok, "a forked identity must REFUSE — picking one silently is how the wrong id ships: {out}");
    assert!(
        out.contains("deadbeef-0000-0000-0000-000000000000"),
        "the refusal NAMES both candidate ids, because the human resolving this has to know which \
         two are in conflict: {out}"
    );
    assert!(out.contains("installed-units.toml"), "and names the file to fix: {out}");
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_version_lookup_that_misses_on_an_installed_process_refuses() {
    let (base, home, producer, _clone) = published("missed");
    // The exact arm that restarted the version: the process IS installed, but under an id the
    // minted registry does not name. Before the fix, `None` here meant "new unit, start at 1".
    let ids = producer.join(".engine").join("contracts").join("unit-ids.toml");
    let text = std::fs::read_to_string(&ids).expect("unit-ids");
    let rewritten: String = text
        .lines()
        .map(|l| {
            if l.trim_start().starts_with("tiny-process = ") {
                "tiny-process = \"facade00-0000-0000-0000-000000000000\"".to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&ids, format!("{rewritten}\n")).expect("desync the registries");

    // The install record still names the real id, so identity resolution REPAIRS the minted registry
    // and the export proceeds at the right identity — the disagreement is fixed, not merely reported,
    // because unit-ids.toml is what the NEXT export reads.
    let (ok, out) = keel_home(&home, &producer, &["process", "export", "tiny-process", "--out", &base.join("out").to_string_lossy()]);
    assert!(ok, "a repairable disagreement is repaired, not refused: {out}");
    let repaired = std::fs::read_to_string(&ids).expect("unit-ids");
    assert!(
        !repaired.contains("facade00-0000-0000-0000-000000000000"),
        "the minted registry is REPAIRED in place — leaving it wrong reopens the defect on the next \
         publish, which is precisely how this survived one fix already:\n{repaired}"
    );
    let _ = std::fs::remove_dir_all(&base);
}
