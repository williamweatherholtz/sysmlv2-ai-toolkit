//! Acting-actor identity resolution (D0129 / issue072 / issue073).
//!
//! Provenance is the substrate every honesty guard reads: `critic-independence` needs to know WHO
//! critiqued (D0080), and `confirmation-authenticity` needs to know that an acceptance was judged by
//! a HUMAN (D0106). Before this module, thirteen write paths DEFAULTED the acting actor — seven of
//! them to a named human — so an AI-driven call that merely OMITTED the field recorded a human
//! attestation silently. That did not make the guard fail loudly; it made it stop meaning anything.
//!
//! Resolution order — explicit beats ambient, and nothing is ever invented:
//!   1. an explicit argument from the caller (`--judged-by` / `--author` / `--by`)
//!   2. the `KEEL_ACTOR` environment variable (per-session, e.g. one agent per shell)
//!   3. the machine-local binding file `.keel/actor` (written by `keel actor set`)
//!   4. REFUSE
//!
//! Refusing is the correct outcome, not a failure mode: an unattributable fact is worse than an
//! absent one, because it looks like evidence. Deliberately absent from the chain: git committer
//! identity. On an agent's machine that is merely whatever the machine was configured with, which is
//! the root of issue073 (an enrolling AI being recorded as a human).

use std::path::{Path, PathBuf};

/// Find the repo root by walking up from `hint` looking for `.tracking` or `.git`.
///
/// Write commands are addressed by FILE, not by root, and the binding lives at the repo root — so
/// resolution must not depend on the current working directory (issue013: the Bash and PowerShell
/// tools share one cwd, so a `cd` in either silently changes what relative paths mean).
#[must_use]
pub fn root_for(hint: &Path) -> PathBuf {
    let start = if hint.is_dir() { hint.to_path_buf() } else { hint.parent().unwrap_or_else(|| Path::new(".")).to_path_buf() };
    let mut cur: &Path = &start;
    loop {
        if cur.join(".tracking").is_dir() || cur.join(".git").exists() {
            return cur.to_path_buf();
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => return PathBuf::from("."),
        }
    }
}

/// Machine-local binding file, relative to the repo root. NEVER committed — it is per-machine, and
/// committing it would re-create the shared-default defect it exists to remove.
pub const BINDING_PATH: &str = ".keel/actor";

/// The registered kind of `name`: `"human"`, `"ai"`, or `None` when unregistered.
///
/// A `Person` (or `kind = ActorKind::human`) is human. Reads `.tracking/actors.sysml` — the registry
/// is the authority (D0178: the write layer refuses AI-kind actors on human-judgment records, and
/// that check needs KIND).
#[must_use]
pub fn kind_of(root: &Path, name: &str) -> Option<String> {
    let text = std::fs::read_to_string(root.join(".tracking").join("actors.sysml")).ok()?;
    let needle = format!("part {name} :");
    for line in text.lines() {
        let l = line.trim_start();
        if !l.starts_with(&needle) {
            continue;
        }
        if l.contains(": Person") {
            return Some("human".to_string());
        }
        if l.contains("ActorKind::human") {
            return Some("human".to_string());
        }
        if l.contains("ActorKind::ai") {
            return Some("ai".to_string());
        }
        return Some("unknown".to_string());
    }
    None
}

/// Every registered `Person` name — the set the D0178 Bash carve-out matches `--by`/`--judged-by`/
/// `KEEL_ACTOR=` values against.
#[must_use]
pub fn person_names(root: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(root.join(".tracking").join("actors.sysml")) else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|line| {
            let l = line.trim_start();
            let rest = l.strip_prefix("part ")?;
            let (name, after) = rest.split_once(':')?;
            after.trim_start().starts_with("Person").then(|| name.trim().to_string())
        })
        .collect()
}

/// Resolve the acting actor from explicit sources only.
///
/// # Errors
///
/// Returns a message explaining how to bind an identity when none is stated. Callers must surface it
/// and abort the write rather than substituting any default.
pub fn resolve(root: &Path, explicit: Option<&str>) -> Result<String, String> {
    if let Some(a) = explicit.map(str::trim).filter(|s| !s.is_empty()) {
        return Ok(a.to_owned());
    }
    if let Ok(v) = std::env::var("KEEL_ACTOR") {
        if !v.trim().is_empty() {
            return Ok(v.trim().to_owned());
        }
    }
    if let Ok(v) = std::fs::read_to_string(root.join(BINDING_PATH)) {
        if !v.trim().is_empty() {
            return Ok(v.trim().to_owned());
        }
    }
    Err(unresolved_message())
}

/// The refusal message: states why, and every way to fix it.
#[must_use]
pub fn unresolved_message() -> String {
    [
        "error: no acting actor — refusing to record a fact that cannot be attributed truthfully.",
        "  Provenance is what the honesty guards read: an acceptance must be judged by a real human",
        "  (D0106), and a critique by an independent critic (D0080). A defaulted actor silently",
        "  falsifies both, so this path has no default (issue072/issue073).",
        "  FIX, in order of precedence:",
        "    1. pass it explicitly:   --judged-by <actor>   (or --author / --by)",
        "    2. set it per session:   KEEL_ACTOR=<actor>",
        "    3. bind this machine:    keel actor set <actor>",
        "  The actor must be registered in .tracking/actors.sysml — an AI is an Actor with kind=ai,",
        "  never a Person. Run the actor-enrollment skill to enroll a new contributor.",
    ]
    .join("\n")
}

/// Declared actor ids in the project registry (`part <id> : Person|Actor`).
#[must_use]
pub fn registered(root: &Path) -> Vec<String> {
    let path = root.join(".tracking").join("actors.sysml");
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("part ") else { continue };
        let Some((name, tail)) = rest.split_once(':') else { continue };
        let ty = tail.trim();
        if ty.starts_with("Person") || ty.starts_with("Actor") {
            out.push(name.trim().to_owned());
        }
    }
    out
}

/// `keel actor show | set <id>` — inspect or bind this machine's acting identity.
///
/// `set` validates against the registry first: binding to an unregistered actor would only defer the
/// failure to commit time, where the `actors` guard would reject every fact already written.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    match args.first().map(String::as_str) {
        Some("show") | None => {
            match resolve(root, None) {
                Ok(a) => {
                    let known = registered(root);
                    let mark = if known.contains(&a) { "registered" } else { "NOT in .tracking/actors.sysml" };
                    println!("{a} ({mark})");
                    0
                }
                Err(msg) => {
                    eprintln!("{msg}");
                    1
                }
            }
        }
        Some("set") => {
            let Some(id) = args.get(1).map(String::as_str).map(str::trim).filter(|s| !s.is_empty()) else {
                eprintln!("usage: keel actor set <actorId>");
                return 2;
            };
            let known = registered(root);
            if !known.iter().any(|k| k == id) {
                eprintln!("error: '{id}' is not registered in .tracking/actors.sysml.");
                eprintln!("  Registered: {}", if known.is_empty() { "(none)".to_owned() } else { known.join(", ") });
                eprintln!("  Enroll first (actor-enrollment skill): an AI is `part {id} : Actor {{ :>> name = \"...\"; :>> kind = ActorKind::ai; }}`,");
                eprintln!("  a human is `part {id} : Person {{ :>> name = \"...\"; :>> email = \"...\"; }}`. Kind is asked, never inferred (issue073).");
                return 1;
            }
            let path = root.join(BINDING_PATH);
            if let Some(parent) = path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("error: cannot create {}: {e}", parent.display());
                    return 1;
                }
            }
            if let Err(e) = std::fs::write(&path, format!("{id}\n")) {
                eprintln!("error: cannot write {}: {e}", path.display());
                return 1;
            }
            println!("bound this machine to actor '{id}' ({})", path.display());
            println!("note: {BINDING_PATH} is machine-local and must never be committed.");
            0
        }
        Some(other) => {
            eprintln!("unknown: keel actor {other} (expected 'show' or 'set <actorId>')");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{registered, resolve};
    use std::path::Path;

    #[test]
    fn explicit_argument_wins_and_is_trimmed() {
        let root = Path::new(".");
        assert_eq!(resolve(root, Some("  someActor  ")).unwrap(), "someActor");
    }

    #[test]
    fn empty_explicit_is_not_an_identity() {
        // The old defaults triggered on an OMITTED field; an empty string must be treated the same
        // way — as unstated — rather than recorded as an actor named "".
        let root = Path::new("/nonexistent-keel-root-for-test");
        std::env::remove_var("KEEL_ACTOR");
        let err = resolve(root, Some("   ")).unwrap_err();
        assert!(err.contains("no acting actor"), "empty explicit must refuse: {err}");
    }

    #[test]
    fn refusal_names_every_remedy() {
        let root = Path::new("/nonexistent-keel-root-for-test");
        std::env::remove_var("KEEL_ACTOR");
        let err = resolve(root, None).unwrap_err();
        for expected in ["--judged-by", "KEEL_ACTOR", "keel actor set", "kind=ai"] {
            assert!(err.contains(expected), "refusal must mention {expected}: {err}");
        }
    }

    #[test]
    fn registered_reads_both_person_and_ai_actors() {
        // Kind matters, not just the id: `actor set` must be able to tell a Person from an AI Actor,
        // because that distinction is what protects human-only attestation authority (issue073).
        // Synthetic registry in the exact authored format — no coupling to this repo's own data.
        let dir = std::env::temp_dir().join("keel-actor-test-registry");
        let tracking = dir.join(".tracking");
        std::fs::create_dir_all(&tracking).expect("temp dir");
        std::fs::write(
            tracking.join("actors.sysml"),
            "package ProjectActors {\n\
             \x20   private import EngineElement::*;\n\n\
             \x20   part someHuman : Person { :>> name = \"A Human\"; :>> email = \"a@b.c\"; }\n\
             \x20   part someAi : Actor { :>> name = \"An AI\"; :>> kind = ActorKind::ai; }\n\
             }\n",
        )
        .expect("write registry");

        let known = registered(&dir);
        assert!(known.contains(&"someHuman".to_string()), "human actor read: {known:?}");
        assert!(known.contains(&"someAi".to_string()), "ai actor read: {known:?}");
        assert_eq!(known.len(), 2, "only actor declarations, not imports or the package: {known:?}");

        // An unreadable registry yields no actors rather than inventing one.
        assert!(registered(Path::new("/nonexistent-keel-root-for-test")).is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
