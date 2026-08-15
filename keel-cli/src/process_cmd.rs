//! `keel process` — the catalogue: list, search, export, import (srPortCatalogExchange, D0128).
//!
//! # A process is a UNIT, and these commands are what make that true rather than asserted
//!
//! srPortModularProcessUnit says a process must carry its definition, its deploying skill, its
//! declared rules/guards and its metadata, "so it can be moved between projects whole, without
//! carrying the rest of the engine and without leaving its enforcement behind". `Unit` already
//! models that composition and `activate`/`deactivate` already toggle enforcement by it. What was
//! missing is the part that PROVES the unit is self-contained: being able to take one out and put it
//! into another project. An export that produces a bundle another project can import is the only
//! honest demonstration; a diagram of the parts is not.
//!
//! # The palette is the engine's own shipped processes
//!
//! "A curated palette plus bring-your-own sources" needs no registry service: `keel init` already
//! ships `.engine/processes/`, so every project starts holding the curated set, and `import` takes a
//! bundle from any path, which is bring-your-own. A hosted catalogue would be a different product.
//!
//! # What is deliberately NOT built
//!
//! The backlog item's prose also mentions "opt-in popularity". No requirement asks for it —
//! srPortCatalogExchange stops at palette plus BYO — and it would send usage data outward, which is
//! a decision for a human rather than a detail of this command. Left out, and said so.

use std::path::{Path, PathBuf};

/// One catalogue row: a process and the unit it carries.
struct Row {
    name: String,
    active: bool,
    purpose: String,
    skills: Vec<String>,
    rules: Vec<String>,
    guards: Vec<String>,
}

fn rows(root: &Path) -> Vec<Row> {
    let act = crate::activation::Activation::load(root);
    let mut out = Vec::new();
    for name in act.unit_names() {
        let u = act.unit(&name);
        out.push(Row {
            purpose: process_purpose(root, &name),
            active: act.is_process_active(&name),
            skills: u.map(|u| u.skills.clone()).unwrap_or_default(),
            rules: u.map(|u| u.rules.clone()).unwrap_or_default(),
            guards: u.map(|u| u.guards.clone()).unwrap_or_default(),
            name,
        });
    }
    out
}

/// A process's declared `purpose`, read from its definition file.
fn process_purpose(root: &Path, name: &str) -> String {
    let path = root.join(".engine").join("processes").join(format!("{name}.sysml"));
    let Ok(text) = std::fs::read_to_string(path) else { return String::new() };
    text.split(":>> purpose = \"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or_default()
        .chars()
        .take(160)
        .collect()
}

/// The files that MAKE UP a process unit — the thing that moves between projects.
fn unit_files(root: &Path, r: &Row) -> Vec<PathBuf> {
    let e = root.join(".engine");
    let mut files = vec![e.join("processes").join(format!("{}.sysml", r.name))];
    for s in &r.skills {
        files.push(e.join("skills").join(s).join("SKILL.md"));
    }
    // Rules live in shared files, so a rule is carried by NAME in the manifest rather than by
    // copying a file that also holds other processes' rules. Splitting them per process would be a
    // schema-shaped change, not an export detail — recorded rather than done silently.
    files.into_iter().filter(|p| p.exists()).collect()
}

fn print_row(r: &Row, verbose: bool) {
    let mark = if r.active { "active  " } else { "INACTIVE" };
    println!("  [{mark}] {}", r.name);
    if !r.purpose.is_empty() {
        println!("             {}", r.purpose);
    }
    if verbose {
        println!("             skills: {}", if r.skills.is_empty() { "—".to_string() } else { r.skills.join(", ") });
        println!("             rules:  {}", if r.rules.is_empty() { "—".to_string() } else { r.rules.join(", ") });
        println!("             guards: {}", if r.guards.is_empty() { "—".to_string() } else { r.guards.join(", ") });
    }
}


/// `keel process export <name> --out <dir>` — write the unit as a portable bundle.
fn cmd_export(args: &[String], root: &Path) -> i32 {

            let Some(name) = args.get(1) else {
                eprintln!("usage: keel process export <name> --out <dir>");
                return 2;
            };
            let Some(out) = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1)) else {
                eprintln!("usage: keel process export <name> --out <dir>");
                return 2;
            };
            let all = rows(root);
            let Some(r) = all.iter().find(|r| &r.name == name) else {
                eprintln!("error: no process '{name}'.");
                return 2;
            };
            let dst = PathBuf::from(out).join(name);
            let files = unit_files(root, r);
            if files.is_empty() {
                eprintln!("error: '{name}' has no definition file — nothing to export.");
                return 1;
            }
            for f in &files {
                let rel = f.strip_prefix(root.join(".engine")).unwrap_or(f);
                let target = dst.join(rel);
                if let Some(p) = target.parent() {
                    if let Err(e) = std::fs::create_dir_all(p) {
                        eprintln!("error: {e}");
                        return 1;
                    }
                }
                if let Err(e) = std::fs::copy(f, &target) {
                    eprintln!("error copying {}: {e}", f.display());
                    return 1;
                }
            }
            // The manifest is what makes the bundle a UNIT rather than loose files: it names the
            // guards and rules the receiving project must activate for the enforcement to travel
            // with the process. Without it an import would land the skill and leave the teeth behind
            // — precisely the failure srPortModularProcessUnit names.
            let manifest = format!(
                "# keel process unit: {name}\n# Exported from a keel project. Import with `keel process import <this dir>`.\n\
                 #\n# The GUARDS below are the enforcement this process owns. A receiving project must have them\n\
                 # in its binary and activate this process for them to run — importing the files alone lands the\n\
                 # skill and leaves the teeth behind, which is the failure this manifest exists to prevent.\n\
                 process = \"{name}\"\nskills = [{}]\nrules = [{}]\nguards = [{}]\n",
                r.skills.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "),
                r.rules.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "),
                r.guards.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "),
            );
            if let Err(e) = std::fs::write(dst.join("unit.toml"), manifest) {
                eprintln!("error writing manifest: {e}");
                return 1;
            }
            println!("exported '{name}' -> {}", dst.display());
            println!("  {} file(s) + unit.toml naming {} guard(s) the receiving project must activate.", files.len(), r.guards.len());
            0
}

/// `keel process import <dir>` — install a unit, refusing on a name collision.
fn cmd_import(args: &[String], root: &Path) -> i32 {

            let Some(dir) = args.get(1).map(PathBuf::from) else {
                eprintln!("usage: keel process import <dir>");
                return 2;
            };
            let manifest = dir.join("unit.toml");
            let Ok(mtext) = std::fs::read_to_string(&manifest) else {
                eprintln!("error: {} has no unit.toml — that file is what makes a bundle a process UNIT.", dir.display());
                eprintln!("  Export one with `keel process export <name> --out <dir>`.");
                return 2;
            };
            let name = mtext
                .lines()
                .find_map(|l| l.strip_prefix("process = \""))
                .and_then(|s| s.split('"').next())
                .unwrap_or_default()
                .to_owned();
            if name.is_empty() {
                eprintln!("error: unit.toml does not name a process.");
                return 2;
            }
            let target = root.join(".engine").join("processes").join(format!("{name}.sysml"));
            if target.exists() {
                eprintln!("error: this project already has a process '{name}' — refusing to overwrite it.");
                eprintln!("  Two processes of the same name are not a merge, they are a collision (D0108: a non-owner");
                eprintln!("  never overwrites in place). Rename the incoming unit, or supersede the existing one first.");
                return 1;
            }
            let mut copied = 0u32;
            for entry in walk(&dir) {
                let Ok(rel) = entry.strip_prefix(&dir) else { continue };
                if rel == Path::new("unit.toml") {
                    continue;
                }
                let dst = root.join(".engine").join(rel);
                if let Some(p) = dst.parent() {
                    let _ = std::fs::create_dir_all(p);
                }
                if std::fs::copy(&entry, &dst).is_ok() {
                    copied += 1;
                }
            }
            println!("imported '{name}': {copied} file(s) into .engine/");
            let guards = mtext.lines().find(|l| l.starts_with("guards = ")).unwrap_or("guards = []");
            println!("  {guards}");
            println!("  NOT YET ENFORCED. Run `keel activate {name}` to turn its guards on — importing the files");
            println!("  installs the process, activating it is what makes the enforcement travel with it.");
            println!("  Then `keel validate . && keel guard .` before committing.");
            0
}

/// `keel process <list|search|show|export|import>`.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    match args.first().map(String::as_str) {
        Some("list") | None => {
            let all = rows(root);
            println!("processes ({} in this project's palette):", all.len());
            for r in &all {
                print_row(r, args.iter().any(|a| a == "--verbose"));
            }
            println!();
            println!("  `keel process show <name>` for the unit; `keel activate|deactivate <name>` to change what is ENFORCED.");
            0
        }
        Some("search") => {
            let Some(term) = args.get(1).map(|s| s.to_lowercase()) else {
                eprintln!("usage: keel process search <term>");
                return 2;
            };
            let all = rows(root);
            let hits: Vec<&Row> = all
                .iter()
                .filter(|r| {
                    r.name.to_lowercase().contains(&term)
                        || r.purpose.to_lowercase().contains(&term)
                        || r.guards.iter().any(|g| g.to_lowercase().contains(&term))
                        || r.skills.iter().any(|s| s.to_lowercase().contains(&term))
                })
                .collect();
            println!("{} process(es) match '{term}':", hits.len());
            for r in hits {
                print_row(r, true);
            }
            0
        }
        Some("show") => {
            let Some(name) = args.get(1) else {
                eprintln!("usage: keel process show <name>");
                return 2;
            };
            let all = rows(root);
            let Some(r) = all.iter().find(|r| &r.name == name) else {
                eprintln!("error: no process '{name}'. `keel process list` shows the palette.");
                return 2;
            };
            print_row(r, true);
            println!("             files that MOVE with it:");
            for f in unit_files(root, r) {
                println!("               {}", f.strip_prefix(root).unwrap_or(&f).display().to_string().replace('\\', "/"));
            }
            0
        }
        Some("export") => cmd_export(args, root),
        Some("import") => cmd_import(args, root),
        Some(other) => {
            eprintln!("unknown: keel process {other} (expected list | search <term> | show <name> | export <name> --out <dir> | import <dir>)");
            2
        }
    }
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else {
            out.push(p);
        }
    }
    out
}
