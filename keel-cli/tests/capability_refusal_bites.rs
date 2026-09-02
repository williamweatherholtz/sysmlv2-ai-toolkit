//! D0252 clause A, COMMAND slice — a unit cannot land on an engine that lacks what it invokes.
//!
//! WHY THIS EXISTS BEFORE D0273. The human chose the clean break: the 43-command lens family
//! collapses and the old names are REMOVED with no alias window. D0252 clause A already anticipated
//! exactly that, recording that renaming a command "is now a breaking change for units that declare
//! it... recorded so a future rename is not made casually", and promising an install-time refusal
//! naming the missing capability. That half was never built — `commands` appeared nowhere in
//! `process_cmd.rs` and `unit.toml` carried only skills, rules, guards and extras.
//!
//! So the downstream failure mode of a rename was not a named refusal but a SKILL WHOSE
//! INSTRUCTIONS SILENTLY DO NOT RUN, which is worse than the cost stated on the page the human
//! answered. This suite is what turns it loud.
//!
//! THE INVENTORY CASE IS THE LOAD-BEARING ONE. A capability check is only as good as its list: an
//! inventory that drifts from the dispatch makes the handshake refuse commands the binary HAS, or
//! accept ones it lacks — a control that is confidently wrong. Rust cannot introspect a `match` at
//! runtime, so the const is unavoidable and the test is what makes it trustworthy.

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

/// Every command name `main.rs` actually dispatches, parsed from the one top-level `match`.
fn dispatched_commands() -> Vec<String> {
    let src = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("main.rs"))
        .expect("main.rs is readable from the crate it belongs to");
    let start = src.find("fn main() {").expect("main() exists");
    let block = &src[start..start + src[start..].find("\n    };").expect("the dispatch match closes")];
    let mut names: Vec<String> = Vec::new();
    for line in block.lines().filter(|l| l.trim_start().starts_with("Some(")) {
        let mut rest = line;
        while let Some(q) = rest.find('"') {
            rest = &rest[q + 1..];
            let Some(end) = rest.find('"') else { break };
            let tok = &rest[..end];
            // A leading `-` means a FLAG spelling (`--version`, `-V`), not a command. Units invoke
            // commands; nothing declares a dependency on a flag alias, and including them would make
            // the inventory refuse to match the dispatch forever.
            let is_command_token = tok.starts_with(|c: char| c.is_ascii_lowercase())
                && tok.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            if is_command_token {
                names.push(tok.to_string());
            }
            rest = &rest[end + 1..];
        }
    }
    names.sort();
    names.dedup();
    names
}

#[test]
fn the_command_inventory_matches_the_dispatch() {
    let dispatched = dispatched_commands();
    let declared: Vec<String> = keel_cli::cli_surface::COMMAND_NAMES.iter().map(|s| (*s).to_string()).collect();
    assert!(!dispatched.is_empty(), "the parser must actually find the dispatch, or this test proves nothing");

    let undeclared: Vec<&String> = dispatched.iter().filter(|c| !declared.contains(c)).collect();
    let undispatched: Vec<&String> = declared.iter().filter(|c| !dispatched.contains(c)).collect();
    assert!(
        undeclared.is_empty(),
        "these commands are DISPATCHED but not in COMMAND_NAMES, so a unit invoking one would be \
         refused against a binary that has it: {undeclared:?}"
    );
    assert!(
        undispatched.is_empty(),
        "these are in COMMAND_NAMES but NOT dispatched, so a unit invoking one would be ACCEPTED \
         against a binary that cannot run it — the exact silent failure D0252 clause A exists to \
         prevent: {undispatched:?}"
    );
}

/// Every lens `cmd_show` routes must be in `LENS_NAMES`, and vice versa (D0273).
///
/// The same argument as the command inventory, one level down. The viewpoint-renderer guard now
/// accepts `keel show <lens>` only when the LENS resolves — accepting the verb alone would let
/// `keel show frobnicate` pass, which is the original hole with an extra word in it. So `LENS_NAMES`
/// became load-bearing the moment a guard started reading it, and a list a guard trusts has to be
/// held equal to the thing it describes.
#[test]
fn the_lens_inventory_matches_the_router() {
    let src = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("main.rs"))
        .expect("main.rs is readable");
    let start = src.find("fn cmd_show(args: &[String]) -> i32 {").expect("the router exists");
    let block = &src[start..start + src[start..].find("\n}\n").expect("the router closes")];
    let mut routed: Vec<String> = Vec::new();
    for line in block.lines().filter(|l| l.trim_start().starts_with("Some(\"")) {
        let mut rest = line;
        while let Some(q) = rest.find('"') {
            rest = &rest[q + 1..];
            let Some(end) = rest.find('"') else { break };
            let tok = &rest[..end];
            if tok.starts_with(|c: char| c.is_ascii_lowercase())
                && tok.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                routed.push(tok.to_string());
            }
            rest = &rest[end + 1..];
        }
    }
    routed.sort();
    routed.dedup();
    assert!(!routed.is_empty(), "the parser must find the router's arms, or this proves nothing");

    let declared: Vec<String> = keel_cli::cli_surface::LENS_NAMES.iter().map(|s| (*s).to_string()).collect();
    let unlisted: Vec<&String> = routed.iter().filter(|c| !declared.contains(c)).collect();
    let unrouted: Vec<&String> = declared.iter().filter(|c| !routed.contains(c)).collect();
    assert!(unlisted.is_empty(), "routed by `keel show` but absent from LENS_NAMES: {unlisted:?}");
    assert!(
        unrouted.is_empty(),
        "in LENS_NAMES but NOT routed — a viewpoint naming one would pass the renderer guard and then \
         fail when anyone ran it: {unrouted:?}"
    );
}

/// The old lens spellings are GONE, not aliased (D0273 — the human chose the clean break).
#[test]
fn a_retired_lens_verb_is_no_longer_a_command() {
    let base = std::env::temp_dir().join(format!("keel-retired-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("mkdir");
    for retired in ["orphans", "coverage", "suspect", "indicators"] {
        let (ok, text) = run(&base, &[retired, "."]);
        assert!(!ok, "`keel {retired}` must no longer be a command — no alias window: {text}");
        assert!(
            text.contains("keel <subcommand>"),
            "and an unknown verb falls through to the usage banner rather than a confusing error: {text}"
        );
    }
    // ...while the router reaches every one of them.
    for lens in ["orphans", "coverage", "suspect", "indicators"] {
        let (_, text) = run(&base, &["show", lens, "--help"]);
        assert!(
            !text.contains("unknown lens"),
            "`keel show {lens}` must resolve — the point of the collapse is that the lens survives: {text}"
        );
    }
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn an_exported_unit_declares_the_commands_its_own_text_invokes() {
    let base = std::env::temp_dir().join(format!("keel-cap-export-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let proj = base.join("proj");
    std::fs::create_dir_all(proj.join(".engine").join("processes")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".engine").join("skills").join("capy")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".tracking")).expect("mkdir");
    std::fs::write(proj.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    // The process file is the unit's one shipped file for this minimal shape, so the instruction has
    // to live there for the export to see it.
    std::fs::write(
        proj.join(".engine").join("processes").join("capy.sysml"),
        "// Run `keel audit` and then `keel definitely-not-a-command` to finish.\npackage ProcessCapy {\n}\n",
    )
    .expect("process");
    std::fs::write(proj.join(".engine").join("skills").join("capy").join("SKILL.md"), "# capy\n").expect("skill");
    std::fs::write(
        proj.join(".engine").join("skills").join("capy").join("registry.sysml"),
        "package SkillsRegistryCapy {\n}\n",
    )
    .expect("registry");

    let out = base.join("out");
    let (ok, text) = run(&proj, &["process", "export", "capy", "--out", &out.to_string_lossy()]);
    assert!(ok, "export: {text}");
    let manifest = std::fs::read_to_string(out.join("capy").join("unit.toml")).expect("unit.toml");
    let line = manifest.lines().find(|l| l.starts_with("commands = ")).expect("a commands line exists");
    assert!(line.contains("\"audit\""), "a real command the text invokes is declared: {line}");
    assert!(
        !line.contains("definitely-not-a-command"),
        "and the scan is SELF-LIMITING: a token following `keel ` that this binary does not dispatch \
         is not a capability, so prose cannot invent a dependency: {line}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn importing_a_unit_that_needs_a_missing_command_refuses_and_names_it() {
    let base = std::env::temp_dir().join(format!("keel-cap-import-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let proj = base.join("proj");
    std::fs::create_dir_all(proj.join(".engine")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".tracking")).expect("mkdir");
    std::fs::write(proj.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");

    // A bundle from a FUTURE engine: it invokes a command this binary does not have. Hand-built
    // rather than exported, because the point is a unit built elsewhere — which is the only way this
    // situation ever actually arises.
    let unit = base.join("bundle").join("futureproc");
    std::fs::create_dir_all(unit.join("processes")).expect("mkdir");
    std::fs::write(
        unit.join("unit.toml"),
        "unitId = \"11111111-2222-3333-4444-555555555555\"\nversion = 1\nprocess = \"futureproc\"\n\
         skills = []\nrules = []\nguards = []\nextras = []\ncommands = [\"orphans\", \"audit\"]\n",
    )
    .expect("manifest");
    std::fs::write(unit.join("processes").join("futureproc.sysml"), "package ProcessFuture {\n}\n").expect("proc");

    let (ok, text) = run(&proj, &["process", "import", &unit.to_string_lossy()]);
    assert!(
        !ok,
        "a unit invoking a command this engine does not dispatch must REFUSE — landing it installs a \
         skill whose instructions cannot be followed:\n{text}"
    );
    assert!(
        text.contains("orphans"),
        "and the refusal NAMES the missing command rather than printing two version numbers, which \
         is the whole reason D0252 chose capabilities over a range. `orphans` is a RETIRED lens verb \
         (D0273), so this is precisely the case the clean break creates downstream:\n{text}"
    );
    assert!(
        !text.contains("audit"),
        "only the MISSING one is named — listing capabilities the engine HAS would bury the signal:\n{text}"
    );
    // And nothing was written: the refusal happens before any file lands.
    assert!(
        !proj.join(".engine").join("processes").join("futureproc.sysml").exists(),
        "a refused import must leave NO trace: a half-installed unit is the state the handshake exists \
         to prevent"
    );
    let _ = std::fs::remove_dir_all(&base);
}

/// issue326: the decision-channel unit ships two CI workflows as extras, and both ran
/// `cargo build -p keel-cli` — a crate no consumer has. Every import handed the consumer two
/// permanently red workflows. Both must now build ONLY where the crate exists and download the
/// project's PINNED release everywhere else, producing the binary at the same path either way so
/// nothing downstream in the workflow changes.
#[test]
fn the_shipped_workflows_build_here_and_download_the_pin_downstream() {
    let wf = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").join(".github").join("workflows");
    for name in ["decision-issue.yml", "decision-record.yml"] {
        let t = std::fs::read_to_string(wf.join(name)).expect(name);
        assert!(t.contains("if [ -f keel-cli/Cargo.toml ]"), "{name}: the build must be CONDITIONAL on the crate existing");
        assert!(t.contains(".engine/contracts/engine-version.toml"), "{name}: the download must read the PIN");
        assert!(t.contains("releases/download/v${PIN}/keel-linux-x86_64"), "{name}: and fetch that exact asset");
        assert!(t.contains("target/release/keel"), "{name}: the binary lands where the rest of the workflow already looks");
        // The Rust toolchain steps are pointless downstream and must be gated the same way.
        assert!(
            t.contains("if: hashFiles('keel-cli/Cargo.toml') != ''"),
            "{name}: toolchain/cache steps must be skipped where there is no crate to build"
        );
    }
}

