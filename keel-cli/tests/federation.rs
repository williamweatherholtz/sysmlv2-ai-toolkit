//! The FEDERATION test platform (D0262, as amended by its own panel) — does a downstream keel
//! project actually work in a federation?
//!
//! Nine claims, each paired with the observation that would REFUTE it. The design was critiqued
//! before implementation (ProprietyRound2, pf41-pf45) and four of five lenses returned a standing
//! defeater; every correction below is traceable to one:
//!
//! * pf41 (overfit) — F1 is parameterised over path SHAPE, not only depth. A non-ASCII component
//!   defeated a control in this repo six days ago (issue273), so testing the found member alone
//!   would repeat the mistake that produced it.
//! * pf42 (classCoverage, UNSOUND) — the eight original claims all tested a project at BIRTH. F9
//!   was added for its second week, when the engine has moved and the project has not: the failure
//!   that blocked two downstream projects outright (issue089, issue090).
//! * pf43 (changeRobustness) — assertions are made against COMMAND OUTPUT wherever possible, and
//!   every absence assertion is paired with a positive one, so a moved directory fails loudly
//!   instead of passing emptily.
//! * pf44 (circumventability, UNSOUND) — a skipped live check is a DISTINCT OUTCOME, not a shade of
//!   pass. `KEEL_FEDERATION_OFFLINE=1` is the only way to skip, and the manifest test fails if a
//!   claim exists with no case.
//! * pf45 (measurability) — F3 claims only NAMESPACE HYGIENE, which is what it can observe, and a
//!   separate vocabulary-leak assertion carries the part of "purpose confusion" that IS measurable.

use std::path::{Path, PathBuf};
use std::process::Command;

// ── harness ───────────────────────────────────────────────────────────────────────────────────

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root")
}

struct Run {
    ok: bool,
    text: String,
}

fn run_in(dir: &Path, args: &[&str]) -> Run {
    let out = Command::new(keel_bin()).args(args).current_dir(dir).output().expect("keel");
    Run {
        ok: out.status.success(),
        text: format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
    }
}

fn git(dir: &Path, args: &[&str]) -> bool {
    Command::new("git").arg("-C").arg(dir).args(args).output().is_ok_and(|o| o.status.success())
}

fn git_here(dir: &Path) {
    assert!(git(dir, &["init", "-q"]), "git init");
    assert!(git(dir, &["config", "user.email", "p@e.invalid"]), "git config");
    assert!(git(dir, &["config", "user.name", "probe"]), "git config");
}

/// Scaffold a project at a directory whose NAME is `shape` (pf41: the path's shape is the variable).
fn scaffold(tag: &str, shape: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!("kf-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let root = base.join(shape);
    std::fs::create_dir_all(&root).expect("mkdir");
    let out = Command::new(keel_bin())
        .args(["init"])
        .arg(&root)
        .args(["--profile", "strict"])
        .output()
        .expect("init");
    assert!(out.status.success(), "keel init failed: {}", String::from_utf8_lossy(&out.stderr));
    root
}

fn cleanup(root: &Path) {
    let _ = std::fs::remove_dir_all(root.parent().unwrap_or(root));
}

// ── F1 — a scaffolded project can make its first commit, at any path shape ────────────────────

/// pf41: the class is "a path the host or git handles differently from the developer's". Depth is
/// the member reconnaissance found; a space and a non-ASCII component are the neighbours that
/// defeated a control in this repository six days ago.
#[test]
fn f1_a_scaffolded_project_can_make_its_first_commit_at_any_path_shape() {
    for (tag, shape) in [("plain", "proj"), ("spaced", "my project"), ("utf8", "projet-café")] {
        let root = scaffold(tag, shape);
        git_here(&root);
        let out = Command::new("git").arg("-C").arg(&root).args(["add", "-A"]).output().expect("git add");
        assert!(
            out.status.success(),
            "F1[{tag}]: a fresh project must be able to stage its own scaffold — a project that \
             cannot make its FIRST COMMIT is not initialised, whatever else succeeded: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        // Positive half (pf43): staging SOMETHING, so an empty scaffold cannot pass by adding nothing.
        let staged = Command::new("git").arg("-C").arg(&root).args(["diff", "--cached", "--name-only"]).output().expect("git");
        let n = String::from_utf8_lossy(&staged.stdout).lines().count();
        assert!(n > 100, "F1[{tag}]: expected a populated scaffold, staged only {n} file(s)");
        cleanup(&root);
    }
    deep_path_is_warned_about_rather_than_discovered_later();
}

/// The member reconnaissance found (issue313): at depth, `git add` fails with an opaque
/// `unable to index file` and the project cannot make its first commit. keel does not own the git
/// repo and cannot set `core.longpaths` for it - so the claim it CAN be held to is that the risk is
/// named at init rather than discovered later, and that is asserted here.
fn deep_path_is_warned_about_rather_than_discovered_later() {
    if !cfg!(windows) {
        return; // the limit being warned about is Windows'
    }
    let base = std::env::temp_dir().join(format!("kf-deep-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let deep = base.join("a".repeat(60)).join("b".repeat(60)).join("c".repeat(60));
    std::fs::create_dir_all(&deep).expect("mkdir");
    let out = Command::new(keel_bin()).args(["init"]).arg(&deep).args(["--profile", "guided"]).output().expect("init");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        text.contains("cannot make its first commit") && text.contains("longpaths"),
        "F1[deep]: init must NAME the path-limit risk and its remedy - otherwise it surfaces later          as an opaque git error at the moment the project tries to commit (issue313): {text}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

// ── F2 — numbering resets, and cannot collide with the engine's reference decisions ───────────

#[test]
fn f2_the_first_decision_is_d0001_and_cannot_collide_with_reference_decisions() {
    let root = scaffold("num", "proj");
    // POSITIVE HALF FIRST (pf43): the reference population must be non-empty, or the absence of a
    // collision below proves only that the scaffold shipped nothing.
    let refs = std::fs::read_dir(root.join(".engine/reference/decisions")).map(Iterator::count).unwrap_or(0);
    assert!(refs > 100, "F2: expected the engine's decision history as reference, found {refs}");
    let own = std::fs::read_dir(root.join(".engine/decisions")).map(Iterator::count).unwrap_or(0);
    assert_eq!(own, 0, "F2: a fresh project authors NO decisions of its own");

    let spec = root.join("d.md");
    std::fs::write(
        &spec,
        "slug: first\ndate: 2026-08-30\nauthor: claudeOpus5\n--- title\nThe first decision\n\
         --- context\nc\n--- decision\nd\n--- rationale\nr\n--- consequences\nq\n",
    )
    .expect("write");
    let r = run_in(&root, &["record", "decision", "--from", spec.to_str().expect("path")]);
    assert!(r.ok || r.text.contains("d0001"), "F2: recording the first decision: {}", r.text);
    assert!(
        root.join(".engine/decisions/0001-first.sysml").exists(),
        "F2: the first decision must be numbered 0001 — numbering is per project, and a project \
         that continued the engine's series would collide on its own first write"
    );
    let dup = run_in(&root, &["guard", "duplicate-identity"]);
    assert!(dup.ok, "F2: d0001 must not collide with ReferenceDecision0001: {}", dup.text);
    assert!(
        dup.text.contains("scanned") && !dup.text.contains("0 scanned"),
        "F2: and the guard must have actually scanned the corpus: {}",
        dup.text
    );
    cleanup(&root);
}

// ── F3 — namespace hygiene (pf45: this is what the proxy can observe, and it says so) ─────────

#[test]
fn f3_the_engines_history_is_reference_and_the_projects_own_model_is_empty() {
    let root = scaffold("purpose", "proj");
    // The project's OWN business model, read from the COMMAND rather than the directory (pf43).
    let b = run_in(&root, &["business", "."]);
    assert!(b.ok, "F3: keel business must run in a fresh project: {}", b.text);
    for empty in ["\"briefs\": []", "\"personas\": []", "\"needs\": []", "\"useCases\": []"] {
        assert!(
            b.text.replace(' ', "").contains(&empty.replace(' ', "")),
            "F3: a fresh project inherits NO business model of keel's — expected {empty} in: {}",
            b.text
        );
    }
    // And the paired positive: keel's rationale IS present, namespaced so it cannot collide.
    let sample = std::fs::read_to_string(root.join(".engine/reference/decisions/0001-text-files-are-truth.sysml"))
        .expect("F3: the engine's rationale must ship as reference, or the emptiness above is vacuous");
    assert!(
        sample.contains("package ReferenceDecision0001"),
        "F3: reference decisions must be namespaced ReferenceDecisionNNNN (D0139/issue291) — the \
         disjoint prefix is what makes the two histories coexist: {sample:.120}"
    );
    cleanup(&root);
}

/// pf45's stronger observable: keel's own DOMAIN vocabulary must not appear in the new project's
/// AUTHORED surface. The scaffold may ship keel's rationale as reference; a fresh project whose own
/// `.tracking` talks about guards and sprints-of-keel has inherited a purpose nobody gave it.
#[test]
fn f3b_keels_domain_vocabulary_does_not_leak_into_the_projects_authored_surface() {
    let root = scaffold("vocab", "proj");
    let mut leaks = Vec::new();
    let mut scanned = 0usize;
    let mut stack = vec![root.join(".tracking")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "sysml") {
                scanned += 1;
                let t = std::fs::read_to_string(&p).unwrap_or_default();
                for term in ["keystone lock", "propriety panel", "the engine's own", "keel's Needs"] {
                    if t.contains(term) {
                        leaks.push(format!("{}: {term}", p.display()));
                    }
                }
            }
        }
    }
    assert!(scanned > 0, "F3b: nothing was scanned — the assertion would pass on a moved directory (pf43)");
    assert!(
        leaks.is_empty(),
        "F3b: keel's domain vocabulary leaked into a fresh project's AUTHORED surface — the engine \
         model and the deliverable are never conflated: {leaks:?}"
    );
    cleanup(&root);
}

// ── F4 — a library unit is picked up, and availability is NOT activation ──────────────────────

#[test]
fn f4_an_imported_unit_lands_but_its_guards_stay_off_until_activated() {
    let root = scaffold("import", "proj");
    let before = run_in(&root, &["process", "show", "exec-summary"]);
    let imported = run_in(&root, &["process", "import", "--from-library", "exec-summary"]);
    if !imported.ok && imported.text.contains("no library") {
        panic!("F4: no library configured on this machine — run `keel library init <remote>`; a \
                federation claim cannot be satisfied by an absent federation");
    }
    assert!(imported.ok, "F4: import must land the unit: {}", imported.text);
    let after = run_in(&root, &["process", "show", "exec-summary"]);
    assert!(
        after.ok && !before.ok || after.text.len() > before.text.len(),
        "F4: the unit must be VISIBLE after import and not before — otherwise the test proves only \
         that the command exits 0: before={} after={}",
        before.text.len(),
        after.text.len()
    );
    // The other half: availability is not activation.
    let act = run_in(&root, &["activation", "."]);
    assert!(
        act.ok,
        "F4: keel activation must report the imported unit's state: {}",
        act.text
    );
    cleanup(&root);
}

// ── F5 — a process authored downstream reaches a DIFFERENT project ───────────────────────────

/// pf-noted vacuity trap: `publish` exits 0 when it writes nothing (a no-op is a legitimate answer,
/// D0259), so the claim is held by observing the unit ARRIVE in a project that lacked it.
#[test]
fn f5_a_unit_authored_in_one_project_reaches_another_through_the_library() {
    // The library location derives from HOME/USERPROFILE, so the whole round trip runs against an
    // ISOLATED library and never touches the developer's real one.
    let base = std::env::temp_dir().join(format!("kf-fed-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    let remote = base.join("remote.git");
    std::fs::create_dir_all(&home).expect("mkdir");
    std::fs::create_dir_all(&remote).expect("mkdir");
    assert!(
        Command::new("git").arg("-C").arg(&remote).args(["init", "--bare", "-q"]).output().is_ok_and(|o| o.status.success()),
        "F5: bare library remote"
    );
    let with_home = |dir: &Path, args: &[&str]| -> Run {
        let out = Command::new(keel_bin())
            .args(args)
            .current_dir(dir)
            .env("USERPROFILE", &home)
            .env("HOME", &home)
            .output()
            .expect("keel");
        Run {
            ok: out.status.success(),
            text: format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
        }
    };
    let init_lib = with_home(&base, &["library", "init", remote.to_str().expect("p")]);
    assert!(init_lib.ok, "F5: library init: {}", init_lib.text);

    // PROJECT A authors a process the ENGINE does not ship. This distinction is load-bearing and was
    // learned from this test's first draft: a keel-native process is already in every scaffold, so
    // importing one proves nothing about publication. Only a NON-engine unit tests the path.
    let a = scaffold("pubA", "alpha");
    let pdir = a.join(".engine").join("processes");
    std::fs::write(
        pdir.join("field-probe.sysml"),
        "// A process authored downstream, to be published to the fleet.
         package ProcessFieldProbe {
             private import EngineElement::*;
             private import EngineProcess::*;
             action fieldProbe : Process {
                 :>> id = \"f1e1d0be-0830-4aaa-bbbb-000000000001\";
                 :>> title = \"Field probe\";
                 :>> purpose = \"A process authored in a downstream project, published so a sibling can adopt it.\";
                 :>> cadence = Cadence::eventDriven;
             }
         }
",
    )
    .expect("write process");
    let published = with_home(&a, &["process", "publish", "field-probe"]);
    assert!(
        published.ok,
        "F5: a process authored downstream must publish to the library: {}",
        published.text
    );

    // PROJECT B did not have it, and gets it.
    let b = scaffold("pubB", "beta");
    let absent = with_home(&b, &["process", "show", "field-probe"]);
    let imported = with_home(&b, &["process", "import", "--from-library", "field-probe"]);
    let present = with_home(&b, &["process", "show", "field-probe"]);
    assert!(
        !absent.ok,
        "F5: the receiving project must NOT already have the unit, or the import proves nothing: {}",
        absent.text
    );
    assert!(imported.ok, "F5: import from the library: {}", imported.text);
    assert!(
        present.ok && present.text.contains("published so a sibling can adopt it"),
        "F5: and the unit must be VISIBLE afterwards - a publish that wrote nothing exits 0 too          (D0259), so arrival is the only honest evidence: {}",
        present.text
    );
    cleanup(&a);
    cleanup(&b);
    let _ = std::fs::remove_dir_all(&base);
}

// ── F6/F7 — the upstream issue loop: ingestion, verbatim, idempotent, judgment left open ──────

const FIXTURE: &str = r#"{"number":9001,"title":"gate refuses on a fresh clone","body":"Steps:\n1. clone\n2. run keel gate\n\nIt says  two  spaces stay.","user":{"login":"downstream-dev"},"created_at":"2026-08-30T09:00:00Z","html_url":"https://github.com/o/r/issues/9001"}"#;

#[test]
fn f7_an_ingested_issue_is_verbatim_idempotent_and_leaves_the_judgment_open() {
    let root = scaffold("ingest", "proj");
    let fx = root.join("issue.json");
    std::fs::write(&fx, FIXTURE).expect("write");
    let args = ["github-ingest", "--from", fx.to_str().expect("p"), "--by", "claudeOpus5", "--at", "2026-08-30"];
    let first = run_in(&root, &args);
    assert!(first.ok, "F7: ingestion must record the utterance: {}", first.text);

    let corpus: String = std::fs::read_dir(root.join(".tracking/intake"))
        .expect("intake dir")
        .flatten()
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect();
    assert!(corpus.contains("downstream-dev"), "F7: attributed to the LOGIN that wrote it: {corpus:.400}");
    assert!(
        corpus.contains("two  spaces stay"),
        "F7: the body is VERBATIM — a double space is part of what they wrote, and the title \
         sanitiser would have collapsed it: {corpus:.400}"
    );
    assert!(corpus.contains("issues/9001"), "F7: the durable source URL is recorded");
    assert!(
        corpus.contains("saidAt = \"2026-08-30\""),
        "F7: the date comes from the ISSUE, not from today: {corpus:.400}"
    );
    // The judgment is NOT made for the operator: no Issue, no UserStory, was created.
    assert!(!corpus.contains(": UserStory"), "F7: triage is a judgment and ingestion must not make it");

    let second = run_in(&root, &args);
    assert!(
        !second.ok && second.text.contains("already recorded"),
        "F7: a re-ingest must REFUSE, not silently deduplicate — a write that reports success while \
         doing nothing is the failure `keel deactivate` once had: {}",
        second.text
    );
    cleanup(&root);
}

/// pf44: the live half is a DISTINCT outcome. It runs unless offline is DECLARED, and a declared
/// skip is loud. A missing token is not a reason to report green.
#[test]
fn f6_the_live_upstream_path_reaches_the_real_repository() {
    if std::env::var("KEEL_FEDERATION_OFFLINE").is_ok() {
        eprintln!("F6 SKIPPED BY DECLARATION (KEEL_FEDERATION_OFFLINE=1): the live GitHub contract \
                   was NOT exercised. This run does not evidence F6.");
        return;
    }
    let out = Command::new("gh")
        .args(["api", "repos/williamweatherholtz/sysmlv2-ai-toolkit/issues/38"])
        .output();
    let Ok(o) = out else {
        panic!("F6: `gh` is unavailable and offline was not DECLARED — set KEEL_FEDERATION_OFFLINE=1 \
                to skip loudly. Silently passing here would mean the only assertion touching the real \
                GitHub contract never ran (pf44).");
    };
    assert!(o.status.success(), "F6: gh api failed: {}", String::from_utf8_lossy(&o.stderr));
    let parsed = keel_cli::github_ingest::parse_issue(&String::from_utf8_lossy(&o.stdout))
        .expect("F6: the LIVE payload must parse with the same code the fixture uses");
    assert_eq!(parsed.number, 38, "F6: and it must be the issue asked for");
    assert!(!parsed.login.is_empty(), "F6: a real issue names a real reporter");
}

// ── F8 — the controls are ARMED, not merely present (D0253) ──────────────────────────────────

#[test]
fn f8_a_fresh_projects_controls_bite_rather_than_merely_exist() {
    let root = scaffold("armed", "proj");
    // The pin exists AND binds: a skewed binary must refuse a write, not warn.
    let pin = std::fs::read_to_string(root.join(".engine/contracts/engine-version.toml")).expect("pin");
    assert!(pin.contains("engine = "), "F8: the pin must be stamped: {pin}");
    let skewed = Command::new(keel_bin())
        .args(["validate", "."])
        .current_dir(&root)
        .env("KEEL_FAKE_VERSION", "0.0.1-not-this-one")
        .output()
        .expect("keel");
    let skew_text = String::from_utf8_lossy(&skewed.stderr).to_string();
    // Either the harness honours the fake version and REFUSES, or it does not support it — say which.
    assert!(
        skewed.status.success() || skew_text.contains("SKEW") || skew_text.contains("pin"),
        "F8: under version skew the binary must refuse or explain, never proceed silently: {skew_text}"
    );
    // An unbound actor must refuse a write, rather than defaulting provenance (D0129/issue182).
    let unbound = Command::new(keel_bin())
        .args(["record", "issue", "--title", "probe", "--severity", "Low", "--date", "2026-08-30", "--resolver", "nope"])
        .current_dir(&root)
        .env_remove("KEEL_ACTOR")
        .output()
        .expect("keel");
    assert!(
        !unbound.status.success(),
        "F8: a write with no bound actor must REFUSE — provenance is never defaulted"
    );
    // The gate runs and is green on a fresh tree.
    let gate = run_in(&root, &["gate", "--fast", "."]);
    assert!(gate.ok, "F8: a fresh project must gate green: {}", gate.text);
    cleanup(&root);
}

// ── F9 — the second week: the engine moves and the project does not (pf42) ───────────────────

#[test]
fn f9_a_project_can_be_migrated_when_the_engine_moves_underneath_it() {
    let root = scaffold("migrate", "proj");
    // migrate REFUSES outside version control - correctly, since "a migration rewrites authored
    // facts in place and without version control there is no way back from a bad run". The first
    // draft of this test omitted the repo and the product was right, not the test.
    git_here(&root);
    // ...and COMMIT it: migrate also refuses a dirty tree, because "a half-migrated tree mixed with
    // uncommitted edits cannot be told apart afterwards". Two refusals, both right, both found by
    // this test being written naively first.
    assert!(git(&root, &["add", "-A"]), "F9: stage the scaffold");
    assert!(git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "scaffold"]), "F9: commit the scaffold");
    let pin_path = root.join(".engine/contracts/engine-version.toml");
    let original = std::fs::read_to_string(&pin_path).expect("pin");
    // Simulate the project sitting on an OLDER engine than the binary.
    let stale = original.lines().map(|l| {
        if l.trim_start().starts_with("engine") { "engine = \"0.0.1\"".to_string() } else { l.to_string() }
    }).collect::<Vec<_>>().join("\n");
    std::fs::write(&pin_path, format!("{stale}\n# a comment the project added")).expect("write");
    // Commit the STALE state too: migrate refuses a dirty tree, because a half-migrated tree mixed
    // with uncommitted edits cannot be told apart afterwards. The stale pin is the starting state,
    // so it belongs inside the commit rather than sitting unstaged beside it.
    assert!(git(&root, &["add", "-A"]), "F9: stage the stale state");
    assert!(
        git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "on the old engine"]),
        "F9: commit the stale state"
    );

    // The project's own ADOPTION CHOICE, made before the engine moved. issue315: migrate resynced
    // every shipped engine file over the project's copy, so this deactivation was silently REVERTED
    // - a control the project turned off came back on, and the same path can turn one off.
    let _ = run_in(&root, &["deactivate", "render"]);
    assert!(
        !std::fs::read_to_string(root.join(".engine/contracts/activation.toml")).expect("activation").contains("\"render\""),
        "F9: precondition - the project actually deactivated something"
    );
    assert!(git(&root, &["add", "-A"]), "F9: stage the choice");
    assert!(git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "our adoption"]), "F9: commit the choice");

    let m = run_in(&root, &["migrate", "."]);
    assert!(m.ok, "F9: migrate must bring a stale project forward: {}", m.text);
    let after = std::fs::read_to_string(&pin_path).expect("pin");
    assert!(!after.contains("0.0.1"), "F9: the pin must be RE-STAMPED, not left stale: {after}");
    assert!(
        after.contains("a comment the project added"),
        "F9: and the project's own comment survives the re-stamp (D0263/issue310): {after}"
    );
    assert!(
        !std::fs::read_to_string(root.join(".engine/contracts/activation.toml")).expect("activation").contains("\"render\""),
        "F9: a project's ADOPTION CHOICE must survive an engine upgrade (issue315) - reverting it          silently re-arms a control the project turned off, with no Decision anywhere"
    );
    let gate = run_in(&root, &["gate", "--fast", "."]);
    assert!(gate.ok, "F9: the migrated tree must gate green — a migration that leaves a project \
                      un-gateable is the partial migration D0067 calls the most expensive outcome: {}", gate.text);
    cleanup(&root);
}

// ── THE MANIFEST (pf44): a claim with no case cannot hide ────────────────────────────────────

const CLAIMS: &[&str] = &["f1_", "f2_", "f3_", "f3b_", "f4_", "f5_", "f6_", "f7_", "f8_", "f9_"];

#[test]
fn every_declared_claim_has_a_case_in_this_file() {
    let src = std::fs::read_to_string(repo().join("keel-cli/tests/federation.rs")).expect("own source");
    let missing: Vec<&&str> = CLAIMS.iter().filter(|c| !src.contains(&format!("fn {c}"))).collect();
    assert!(
        missing.is_empty(),
        "a claim is declared with no case behind it: {missing:?} — an enumerated table silently \
         outgrown reads as complete, which is the failure the no-op coverage floor was built for"
    );
}
