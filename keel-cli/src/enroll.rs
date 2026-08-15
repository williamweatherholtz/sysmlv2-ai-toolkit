//! `keel enroll` — bring a contributor from "unknown machine" to "can author an attributed fact,
//! gated identically to everyone else" in one command (D0129, `.engine/processes/actor-enrollment`).
//!
//! Every remote contributor starts unenrolled, and enrollment is the gate through which attribution,
//! acceptance authority and gate uniformity are all established. Distinct from `introduction`, which
//! onboards a fresh PROJECT and never establishes WHO is acting.
//!
//! # Everything here refuses rather than defaults
//!
//! Before `actor::resolve`, thirteen write paths defaulted the acting actor — seven to a named human
//! — so an AI-driven call that merely OMITTED the field recorded a human attestation silently
//! (issue072/issue073). This command is the front door to that substrate, so it inherits the rule
//! absolutely: identity, kind, and a running gate are each stated or the enrollment REFUSES.
//!
//! GIT IDENTITY IS NOT CONSULTED, deliberately, and this is the one place someone would reach for
//! it. On an agent's machine the git committer is merely whatever that machine was configured with,
//! which is exactly how an enrolling AI gets recorded as a human — the root of issue073.
//!
//! # Kind is asked, never inferred
//!
//! `kind` decides what the actor may ATTEST: accepting a Decision, dispositioning a finding at or
//! above the threshold, and adjudicating a contributor conflict are human-only, and nothing else
//! protects them. So registering an AI as a `Person` must be impossible here rather than
//! discouraged — there is no code path in this module that writes `Person` without `--kind human`.

use std::path::Path;

/// What the caller stated. Every field is required; none has a default.
pub struct Enrollment<'a> {
    pub actor: &'a str,
    pub name: &'a str,
    pub kind: Kind,
    pub email: Option<&'a str>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Human,
    Ai,
}

impl Kind {
    /// Parse `--kind`. Anything else is refused rather than guessed.
    fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "human" | "person" => Some(Self::Human),
            "ai" | "agent" => Some(Self::Ai),
            _ => None,
        }
    }
}

/// The `part` declaration for a newly enrolled actor.
///
/// A human is a `Person` (whose schema pins `kind = ActorKind::human`); an AI is an `Actor` with
/// `kind = ActorKind::ai`, NEVER a `Person`. The two branches are separate on purpose — a single
/// parameterised writer would make "AI registered as Person" a one-character bug.
#[must_use]
pub fn declaration(e: &Enrollment) -> String {
    let email = e.email.map_or_else(String::new, |m| format!(" :>> email = \"{m}\";"));
    match e.kind {
        Kind::Human => format!("    part {} : Person {{ :>> name = \"{}\";{email} }}\n", e.actor, e.name),
        Kind::Ai => format!("    part {} : Actor {{ :>> name = \"{}\";{email} :>> kind = ActorKind::ai; }}\n", e.actor, e.name),
    }
}

/// Insert a declaration before the registry package's closing brace, preserving everything else.
///
/// Returns `None` if the registry has no closing brace to insert before — a malformed registry is
/// reported, never repaired by appending and hoping.
#[must_use]
pub fn insert_declaration(registry: &str, decl: &str) -> Option<String> {
    let close = registry.rfind('}')?;
    let mut out = String::with_capacity(registry.len() + decl.len());
    out.push_str(registry[..close].trim_end());
    out.push('\n');
    out.push_str(decl);
    out.push_str(&registry[close..]);
    Some(out)
}

/// Why an enrollment cannot proceed. Each carries the remedy, because a refusal without one just
/// moves the guesswork to the contributor.
pub struct Refusal {
    pub what: String,
    pub remedy: String,
}

/// Verify the local gate EXECUTES rather than silently skipping.
///
/// The issue076 failure is a contributor whose gate does nothing: they are then held to a different
/// standard than everyone else, which breaks the uniform-gate property for the whole team rather
/// than only for them. So "the binary exists" is not the check — "the checks actually ran" is.
fn gate_report(root: &Path) -> Result<String, Refusal> {
    if !root.join(".tracking").is_dir() {
        return Err(Refusal {
            what: format!("{} is not a keel project (no .tracking/)", root.display()),
            remedy: "run `keel init <dir>` to scaffold one, or enroll from inside an existing project.".to_string(),
        });
    }
    let report = crate::validate_root(root);
    if !report.is_clean() {
        return Err(Refusal {
            what: format!("the local gate RUNS but the project does not pass it ({} problem(s))", report.errors.len() + report.diagnostics.len()),
            remedy: "run `keel validate .` and fix what it reports. Enrolling onto a red gate would record you as ready when you are not.".to_string(),
        });
    }
    Ok(format!("validate executed over {} file(s), clean", report.validated))
}

/// Parse args and run the enrollment. Returns the process exit code.
///
/// NON-INTERACTIVE by design. The enrollment process allows an interactive answer, but an agent
/// shell has no usable stdin — and a prompt that reads EOF and proceeds would default exactly the
/// fields that must never be defaulted. Stating them as arguments is the same explicitness by a
/// different route.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let get = |flag: &str| -> Option<&str> {
        args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).map(String::as_str).map(str::trim).filter(|s| !s.is_empty())
    };
    let (actor, name, kind_raw) = (get("--actor"), get("--name"), get("--kind"));
    let email = get("--email");

    if actor.is_none() || name.is_none() || kind_raw.is_none() {
        eprintln!("usage: keel enroll --actor <id> --name \"<display name>\" --kind human|ai [--email <addr>] [ROOT]");
        eprintln!();
        eprintln!("Every field is REQUIRED and none is defaulted. In particular `--kind` is asked, never inferred:");
        eprintln!("  it decides what you may ATTEST — accepting a Decision, dispositioning a finding, adjudicating a");
        eprintln!("  conflict are human-only, and nothing else protects them. An AI enrolls as `--kind ai`, which");
        eprintln!("  registers an Actor, never a Person (issue073).");
        eprintln!();
        eprintln!("Git committer identity is deliberately NOT consulted: on an agent's machine it is whatever that");
        eprintln!("machine was configured with, which is precisely how an AI gets recorded as a human.");
        return 2;
    }
    let (Some(actor), Some(name), Some(kind_raw)) = (actor, name, kind_raw) else { return 2 };
    let Some(kind) = Kind::parse(kind_raw) else {
        eprintln!("error: --kind must be `human` or `ai` (got '{kind_raw}'). It is never guessed: see `keel enroll` with no arguments.");
        return 2;
    };
    if kind == Kind::Ai && email.is_some() {
        // Not fatal, but worth saying: an email on an AI actor invites a reader to treat it as a person.
        println!("note: --email on an AI actor is unusual; it will be recorded, but `kind = ActorKind::ai` is what governs authority.");
    }

    let e = Enrollment { actor, name, kind, email };
    let registry_path = root.join(".tracking").join("actors.sysml");

    // Step 5 FIRST: prove the gate runs before writing anything. Registering onto a gate that does
    // not execute would record a contributor as ready while holding them to no standard at all.
    let gate = match gate_report(root) {
        Ok(g) => g,
        Err(r) => {
            eprintln!("keel enroll: NOT READY — {}", r.what);
            eprintln!("  remedy: {}", r.remedy);
            return 1;
        }
    };

    // Step 3: register without duplicating.
    let already = crate::actor::registered(root).iter().any(|k| k == actor);
    if already {
        println!("actor '{actor}' is already registered — re-binding without creating a second entry.");
    } else {
        let Ok(registry) = std::fs::read_to_string(&registry_path) else {
            eprintln!("keel enroll: cannot read {} — a project registry must exist before enrolling.", registry_path.display());
            eprintln!("  remedy: `keel init` scaffolds one; otherwise create .tracking/actors.sysml with `package ProjectActors {{ ... }}`.");
            return 1;
        };
        let Some(updated) = insert_declaration(&registry, &declaration(&e)) else {
            eprintln!("keel enroll: {} has no closing brace — refusing to repair a malformed registry by appending.", registry_path.display());
            return 1;
        };
        if let Err(err) = std::fs::write(&registry_path, updated) {
            eprintln!("keel enroll: cannot write {}: {err}", registry_path.display());
            return 1;
        }
        println!("registered '{actor}' as {} in {}", if kind == Kind::Ai { "an AI Actor (kind = ActorKind::ai)" } else { "a Person" }, registry_path.display());
    }

    // Step 4: bind the machine, then PROVE the binding by reading back what the write path resolves.
    let binding = root.join(crate::actor::BINDING_PATH);
    if let Some(parent) = binding.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            eprintln!("keel enroll: cannot create {}: {err}", parent.display());
            return 1;
        }
    }
    if let Err(err) = std::fs::write(&binding, format!("{actor}\n")) {
        eprintln!("keel enroll: cannot write {}: {err}", binding.display());
        return 1;
    }
    // The read-back is the point: issue072 was a capture step that printed an actor id which nothing
    // CONSUMED, so the write paths kept defaulting. Asking the resolver is the only proof that the
    // binding actually governs attribution.
    match crate::actor::resolve(root, None) {
        Ok(resolved) if resolved == actor => {
            println!("bound this machine to '{actor}' ({}) — the write path resolves to it.", binding.display());
        }
        Ok(other) => {
            eprintln!("keel enroll: NOT READY — bound '{actor}' but the write path resolves to '{other}'.");
            eprintln!("  remedy: KEEL_ACTOR is set in this shell and overrides the file binding. Unset it, or enroll with the identity it names.");
            return 1;
        }
        Err(msg) => {
            eprintln!("keel enroll: NOT READY — wrote the binding but the write path still cannot resolve an actor.\n{msg}");
            return 1;
        }
    }

    println!("gate: {gate}");
    println!();
    println!("READY. '{actor}' can author facts that are attributed to it and gated like everyone else's.");
    match kind {
        Kind::Human => println!("  You MAY record human acceptance: accepting a Decision, dispositioning a finding, adjudicating a conflict."),
        Kind::Ai => {
            println!("  You may NOT record human acceptance. Accepting a Decision, dispositioning a >= Medium finding and");
            println!("  adjudicating a contributor conflict all require a human actor — an AI cannot supply them, and");
            println!("  recording one anyway is a fabricated attestation, not a shortcut.");
        }
    }
    println!("  ROLE is not recorded: `Actor` has no `role` attribute yet. See the proposed Decision in");
    println!("  `keel orient` -> pendingAcceptances; role stays unrecorded until that is signed off, rather");
    println!("  than being written somewhere it cannot be queried.");
    println!("  Next: `keel orient .` to see where things stand, then the `distributed-collaboration` skill.");
    println!("  note: {} is machine-local and must never be committed.", crate::actor::BINDING_PATH);
    0
}

#[cfg(test)]
mod tests {
    use super::{declaration, insert_declaration, Enrollment, Kind};

    /// The single most important property in this module: an AI is never written as a `Person`.
    #[test]
    fn an_ai_is_registered_as_an_actor_never_a_person() {
        let ai = declaration(&Enrollment { actor: "botX", name: "Bot X", kind: Kind::Ai, email: None });
        assert!(ai.contains("part botX : Actor"), "{ai}");
        assert!(ai.contains("kind = ActorKind::ai"), "{ai}");
        assert!(!ai.contains("Person"), "an AI must never be a Person (issue073): {ai}");

        let human = declaration(&Enrollment { actor: "jo", name: "Jo", kind: Kind::Human, email: Some("jo@x.com") });
        assert!(human.contains("part jo : Person"), "{human}");
        assert!(human.contains("email = \"jo@x.com\""), "{human}");
    }

    #[test]
    fn kind_is_parsed_or_refused_never_guessed() {
        assert_eq!(Kind::parse("human"), Some(Kind::Human));
        assert_eq!(Kind::parse("AI"), Some(Kind::Ai));
        assert_eq!(Kind::parse("agent"), Some(Kind::Ai));
        // Anything else refuses. "yes", "true" and an empty string are all indeterminate, and
        // indeterminate identity is the condition confirmation-authenticity cannot survive.
        for bad in ["", "yes", "true", "robot", "wweatherholtz"] {
            assert_eq!(Kind::parse(bad), None, "must not guess a kind from '{bad}'");
        }
    }

    #[test]
    fn declaration_lands_inside_the_registry_package() {
        let registry = "package ProjectActors {\n    private import EngineElement::*;\n\n    part a : Person { :>> name = \"A\"; }\n}\n";
        let out = insert_declaration(registry, "    part b : Actor { :>> name = \"B\"; :>> kind = ActorKind::ai; }\n").unwrap();
        assert!(out.contains("part a : Person"), "the existing entry survives: {out}");
        assert!(out.contains("part b : Actor"), "{out}");
        // The new entry is INSIDE the package, i.e. before the final brace.
        let (b, close) = (out.find("part b").unwrap(), out.rfind('}').unwrap());
        assert!(b < close, "declaration must land inside the package: {out}");
        // A malformed registry is reported, not repaired by appending.
        assert!(insert_declaration("package Broken {", "x").is_none());
    }
}
