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
    /// False for a process that asserts no guard: it is DECLARED and enactable, but activation
    /// switches guards, so there is nothing about it to switch. Kept distinct from `active` because
    /// reporting a guard-less process as INACTIVE would assert this project had turned it off
    /// (issue241/issue149) — a false claim about the control inventory.
    switchable: bool,
    purpose: String,
    skills: Vec<String>,
    rules: Vec<String>,
    guards: Vec<String>,
}

/// Every DECLARED process, not only the guard-bearing subset (issue241).
///
/// `rows()` used to iterate `unit_names()`, which is the set of processes asserting at least one
/// guard. That made `keel process list|search|show|export` deny the 12 guard-less processes this
/// repository declares — `keel process show intake` answered "no process 'intake'" while
/// `.engine/processes/intake.sysml` sat on disk — and it broke `srPortModularProcessUnit`, the
/// accepted `must` that a unit travel whole between projects. Carrying enforcement is not a
/// precondition of travelling. This is the un-swept half of issue149: that fix taught `activation`
/// and `activate`/`deactivate` to report the whole set, and never reached this catalogue.
fn rows(root: &Path) -> Vec<Row> {
    let act = crate::activation::Activation::load(root);
    let mut out = Vec::new();
    for name in crate::activation::declared_processes(root) {
        let u = act.unit(&name);
        let switchable = u.is_some_and(|u| !u.guards.is_empty());
        out.push(Row {
            purpose: process_purpose(root, &name),
            // A guard-less process is never "inactive": there is no guard to switch off.
            active: !switchable || act.is_process_active(&name),
            switchable,
            // Resolve the deploying skill from the registry when no unit exists, so a guard-less
            // process still EXPORTS WHOLE (definition + skill) — issue241/srPortModularProcessUnit.
            skills: u.map(|u| u.skills.clone()).filter(|s| !s.is_empty())
                .unwrap_or_else(|| crate::activation::deploying_skills(root, &name)),
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
/// Extract `rule`'s full declaration block from the shared rules files (P4b): rules live in files
/// holding MANY processes' rules, so a unit export carries the owned declarations in a unit-local
/// file instead of copying a shared file that would clobber the destination's other rules.
fn extract_rule_block(root: &Path, rule: &str) -> Option<String> {
    for f in crate::collect_sysml(&root.join(".engine").join("rules")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let needle = format!("part {rule} :");
        let Some(start) = text.find(&needle) else { continue };
        // walk braces from the first `{` after the declaration head
        let open = text[start..].find('{')? + start;
        let mut depth = 0usize;
        for (i, c) in text[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(text[start..=open + i].to_string());
                    }
                }
                _ => {}
            }
        }
    }
    None
}

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
    // Three states, never two: a guard-bearing process is active or INACTIVE; a guard-less one is
    // `always` — declared and enactable, with no guard to switch (issue241). Mirrors the wording
    // `keel activation` already uses, so the two catalogues read the same.
    let mark = if !r.switchable {
        "always  "
    } else if r.active {
        "active  "
    } else {
        "INACTIVE"
    };
    println!("  [{mark}] {}", r.name);
    if !r.switchable {
        println!("             (asserts no guard — nothing to switch off; transferable as a unit)");
    }
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
/// Stable per-unit identity (D0183/K9): a unit's id survives re-export, so `--update` can match
/// upstream revisions to installs. First export mints and records it in
/// `.engine/contracts/unit-ids.toml` (committed — identity is shared truth, not machine state).
fn unit_id_for(root: &Path, process: &str) -> String {
    let reg = root.join(".engine").join("contracts").join("unit-ids.toml");
    if let Ok(text) = std::fs::read_to_string(&reg) {
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix(&format!("{process} = \"")) {
                if let Some(id) = rest.split('"').next() {
                    return id.to_string();
                }
            }
        }
    }
    let id = crate::write::gen_uuid();
    let mut text = std::fs::read_to_string(&reg).unwrap_or_else(|_| {
        "# Process-unit identity registry (D0183): a unit's id is its EXCHANGE identity - stable\n# across exports, matched by import --update. Never rewritten.\n".to_string()
    });
    {
        use std::fmt::Write as _;
        let _ = writeln!(text, "{process} = \"{id}\"");
    }
    let _ = std::fs::create_dir_all(reg.parent().unwrap_or(root));
    let _ = crate::write::write_atomic(&reg, text);
    id
}

/// Content hash for the three-way base (D0183): the arch module's stable hash, one per file.
fn file_hash(path: &Path) -> String {
    std::fs::read_to_string(path).map_or_else(|_| "unreadable".to_string(), |t| crate::arch::stable_hash(&t))
}

/// The install record for `unit_id`, from `.engine/contracts/installed-units.toml`:
/// `(version, Vec<(rel_path, hash)>)`.
fn install_record(root: &Path, unit_id: &str) -> Option<(u32, Vec<(String, String)>)> {
    let text = std::fs::read_to_string(root.join(".engine").join("contracts").join("installed-units.toml")).ok()?;
    let mut in_section = false;
    let mut version = 0u32;
    let mut files = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('[') {
            in_section = l == format!("[{unit_id}]");
            continue;
        }
        if !in_section {
            continue;
        }
        if let Some(v) = l.strip_prefix("version = ") {
            version = v.trim().parse().unwrap_or(0);
        }
        if let Some(rest) = l.strip_prefix("file.") {
            if let Some((path, hash)) = rest.split_once(" = ") {
                files.push((path.trim().replace("__SL__", "/"), hash.trim().trim_matches('"').to_string()));
            }
        }
    }
    (version > 0).then_some((version, files))
}

/// Write/replace the install record (the three-way BASE for the next `--update`).
fn write_install_record(root: &Path, unit_id: &str, process: &str, version: u32, files: &[(String, String)]) -> std::io::Result<()> {
    let reg = root.join(".engine").join("contracts").join("installed-units.toml");
    let mut text = std::fs::read_to_string(&reg).unwrap_or_else(|_| {
        "# Installed process units (D0183): the versioned identity + per-file content hashes an\n# import records - the three-way base `import --update` merges against. Committed (a tracked fact).\n".to_string()
    });
    // drop any existing section for this unit id
    let mut out = String::new();
    let mut skipping = false;
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('[') {
            skipping = l == format!("[{unit_id}]");
        }
        if !skipping {
            out.push_str(line);
            out.push('\n');
        }
    }
    text = out;
    {
        use std::fmt::Write as _;
        let _ = write!(text, "[{unit_id}]\nprocess = \"{process}\"\nversion = {version}\n");
    }
    for (path, hash) in files {
        use std::fmt::Write as _;
        let _ = writeln!(text, "file.{} = \"{hash}\"", path.replace('/', "__SL__"));
    }
    std::fs::create_dir_all(reg.parent().unwrap_or(root))?;
    crate::write::write_atomic(&reg, text).map_err(std::io::Error::other)?;
    Ok(())
}

/// The version handshake, guard-names slice (D0183/K8): the unit's required guards diffed against
/// THIS binary's inventory. Missing teeth refuse — or, with `--degrade`, proceed loudly with a
/// recorded Issue (never prose-lands-enforcement-silently-missing). Rules travel with P4b.
fn handshake(root: &Path, manifest: &str, bundle_dir: Option<&Path>, degrade: bool) -> Result<(), i32> {
    let declared: Vec<String> = manifest
        .lines()
        .find(|l| l.starts_with("guards = "))
        .map(|l| l.trim_start_matches("guards = [").trim_end_matches(']').split(',').map(|s| s.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();
    let missing: Vec<String> = declared.iter().filter(|g| !crate::guards::GUARD_NAMES.contains(&g.as_str())).cloned().collect();
    // P4b slice: the unit's declared RULES must be evaluable at the destination - carried by the
    // bundle (a rules/*.sysml declaring them) or already present in the destination's rules dir.
    let rule_names: Vec<String> = manifest
        .lines()
        .find(|l| l.starts_with("rules = "))
        .map(|l| l.trim_start_matches("rules = [").trim_end_matches(']').split(',').map(|s| s.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();
    let mut missing_rules: Vec<String> = Vec::new();
    for rule in &rule_names {
        let needle = format!("part {rule} :");
        let in_bundle = bundle_dir.is_some_and(|d| {
            walk(d).iter().any(|f| {
                f.extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml"))
                    && std::fs::read_to_string(f).is_ok_and(|t| t.contains(&needle))
            })
        });
        let in_dest = crate::collect_sysml(&root.join(".engine").join("rules"))
            .iter()
            .any(|f| std::fs::read_to_string(f).is_ok_and(|t| t.contains(&needle)));
        if !in_bundle && !in_dest {
            missing_rules.push(rule.clone());
        }
    }
    if !missing_rules.is_empty() {
        if degrade {
            eprintln!("DEGRADED IMPORT (K8/P4b): {} declared rule(s) are neither in the bundle nor the destination: {}", missing_rules.len(), missing_rules.join(", "));
            if let Ok(actor) = crate::actor::resolve(root, None) {
                let _ = crate::write::record_obligation(
                    root,
                    "degraded-import",
                    &format!("degraded unit import: {} rule(s) missing", missing_rules.len()),
                    &format!("A process unit was imported with --degrade lacking declared rules: {}. The prose landed WITHOUT those teeth (K8/P4b). Discharge: land the rules and triage with a #Resolves edge.", missing_rules.join(", ")),
                    &actor,
                );
            }
        } else {
            eprintln!("error: the unit declares {} rule(s) this import cannot land: {} (K8/P4b - the teeth cannot travel).", missing_rules.len(), missing_rules.join(", "));
            return Err(1);
        }
    }
    if missing.is_empty() && missing_rules.is_empty() {
        return Ok(());
    }
    if missing.is_empty() {
        return Ok(()); // rules handled above (degraded path recorded below with the guards)
    }
    if !degrade {
        eprintln!("error: this binary lacks {} guard(s) the unit requires: {} (K8 - the enforcement cannot land).", missing.len(), missing.join(", "));
        eprintln!("  Upgrade keel (the version handshake reads `keel version`'s guard inventory), or import with --degrade");
        eprintln!("  to land the prose WITHOUT those teeth - loudly, with a recorded Issue.");
        return Err(1);
    }
    eprintln!("DEGRADED IMPORT (K8): {} required guard(s) are not in this binary: {}", missing.len(), missing.join(", "));
    if let Ok(actor) = crate::actor::resolve(root, None) {
        let _ = crate::write::record_obligation(
            root,
            "degraded-import",
            &format!("degraded unit import: {} guard(s) missing", missing.len()),
            &format!("A process unit was imported with --degrade; this binary lacks required guards: {}. The process prose is installed WITHOUT those teeth (K8). Discharge: upgrade the binary and re-verify, then triage with a #Resolves edge.", missing.join(", ")),
            &actor,
        );
    }
    Ok(())
}

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
            // P4b: the unit's declared rules travel as a UNIT-LOCAL rules file (extracted blocks),
            // landing as rules/<name>-unit-rules.sysml so an import never clobbers shared files.
            if !r.rules.is_empty() {
                let mut blocks = Vec::new();
                let mut missing_rules = Vec::new();
                for rule in &r.rules {
                    match extract_rule_block(root, rule) {
                        Some(b) => blocks.push(b),
                        None => missing_rules.push(rule.clone()),
                    }
                }
                if !missing_rules.is_empty() {
                    eprintln!("error: rule-owners.toml attributes {} rule(s) to '{name}' that no rules file declares: {}", missing_rules.len(), missing_rules.join(", "));
                    return 1;
                }
                let bundle_rules = dst.join("rules").join(format!("{name}-unit-rules.sysml"));
                let _ = std::fs::create_dir_all(bundle_rules.parent().unwrap_or(&dst));
                let body = format!(
                    "// Unit-local rules for process `{name}` (P4b/D0183+D0184): extracted from the origin's\n// shared rules files at export, so the teeth travel WITH the unit.\npackage {}UnitRules {{\n    private import EngineElement::*;\n    private import EngineRules::*;\n\n{}\n}}\n",
                    {
                        let mut cap = name.replace('-', " ");
                        let mut out = String::new();
                        for w in cap.split_whitespace() {
                            let mut cs = w.chars();
                            if let Some(f) = cs.next() {
                                out.push(f.to_ascii_uppercase());
                                out.push_str(cs.as_str());
                            }
                        }
                        cap = out;
                        cap
                    },
                    blocks.iter().map(|b| format!("    {b}")).collect::<Vec<_>>().join("\n\n")
                );
                if let Err(e) = std::fs::write(&bundle_rules, body) {
                    eprintln!("error writing unit rules: {e}");
                    return 1;
                }
            }
            let unit_id = unit_id_for(root, name);
            let version = install_record(root, &unit_id).map_or(1, |(v, _)| v + 1);
            let manifest = format!(
                "# keel process unit: {name}\n# Exported from a keel project. Import with `keel process import <this dir>`.\n\
                 #\n# The GUARDS below are the enforcement this process owns. A receiving project must have them\n\
                 # in its binary and activate this process for them to run — importing the files alone lands the\n\
                 # skill and leaves the teeth behind, which is the failure this manifest exists to prevent.\n\
                 unitId = \"{unit_id}\"\nversion = {version}\nprocess = \"{name}\"\nskills = [{}]\nrules = [{}]\nguards = [{}]\n",
                r.skills.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "),
                r.rules.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "),
                r.guards.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", "),
            );
            if let Err(e) = std::fs::write(dst.join("unit.toml"), manifest) {
                eprintln!("error writing manifest: {e}");
                return 1;
            }
            // The ORIGIN records its own exports too (found live: without this the origin re-exports
            // v1 forever and downstream --update can never see a version advance).
            let exported_hashes: Vec<(String, String)> = files
                .iter()
                .map(|f| {
                    let rel = f.strip_prefix(root.join(".engine")).unwrap_or(f).to_string_lossy().replace('\\', "/");
                    (rel, file_hash(f))
                })
                .collect();
            if let Err(e) = write_install_record(root, &unit_id, name, version, &exported_hashes) {
                eprintln!("warning: export registry not updated: {e}");
            }
            println!("exported '{name}' v{version} -> {}", dst.display());
            println!("  {} file(s) + unit.toml naming {} guard(s) the receiving project must activate.", files.len(), r.guards.len());
            0
}

/// `keel process import <dir> [--update] [--degrade] [--assume-local-base]` (D0183/K8/K9).
///
/// FRESH IMPORT: refuses a name collision; runs the guard handshake; writes the INSTALL RECORD
/// (versioned unit id + per-file content hashes) — the three-way base the next `--update` merges
/// against; rewrites `activation.toml` guidance stays with `keel activate`.
///
/// `--update`: three-way per file (recorded base / upstream-new / local): local-at-base takes
/// upstream; upstream-at-base keeps local edits (a local `assert constraint` addition is never
/// silently clobbered); three-way divergence REFUSES with a per-file report. Pre-P4a installs have
/// no base: `--assume-local-base` bootstraps by treating local as base — an explicit human
/// confirmation, never inferred. The update lands under the D0070 keystone (the commit staging
/// `.engine/processes/` must carry a marked Decision), which IS the supersession record K9 wants;
/// `governing-version`/`reprocess-candidates` read the definition history per item (K10).
#[allow(clippy::too_many_lines)]
fn cmd_import(args: &[String], root: &Path) -> i32 {
    let Some(dir) = args.get(1).map(PathBuf::from) else {
        eprintln!("usage: keel process import <dir> [--update] [--degrade] [--assume-local-base]");
        return 2;
    };
    let update = args.iter().any(|a| a == "--update");
    let degrade = args.iter().any(|a| a == "--degrade");
    let assume_base = args.iter().any(|a| a == "--assume-local-base");
    let manifest_path = dir.join("unit.toml");
    let Ok(mtext) = std::fs::read_to_string(&manifest_path) else {
        eprintln!("error: {} has no unit.toml — that file is what makes a bundle a process UNIT.", dir.display());
        eprintln!("  Export one with `keel process export <name> --out <dir>`.");
        return 2;
    };
    let field = |key: &str| -> String {
        mtext
            .lines()
            .find_map(|l| l.strip_prefix(&format!("{key} = \"")))
            .and_then(|s| s.split('"').next())
            .unwrap_or_default()
            .to_owned()
    };
    let name = field("process");
    if name.is_empty() {
        eprintln!("error: unit.toml does not name a process.");
        return 2;
    }
    let unit_id = field("unitId");
    let version: u32 = mtext
        .lines()
        .find_map(|l| l.strip_prefix("version = "))
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(1);
    // K8: the enforcement handshake runs BEFORE any file lands.
    if let Err(code) = handshake(root, &mtext, Some(&dir), degrade) {
        return code;
    }
    let target = root.join(".engine").join("processes").join(format!("{name}.sysml"));

    if !update {
        if target.exists() {
            eprintln!("error: this project already has a process '{name}' — refusing to overwrite it.");
            eprintln!("  A same-name landing is an UPDATE: `keel process import <dir> --update` performs the");
            eprintln!("  three-way supersession (D0183/K9). A different process wants a rename, not a merge.");
            return 1;
        }
        let mut copied = 0u32;
        let mut hashes: Vec<(String, String)> = Vec::new();
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
                hashes.push((rel.to_string_lossy().replace('\\', "/"), file_hash(&dst)));
            }
        }
        if unit_id.is_empty() {
            eprintln!("note: pre-D0183 unit (no unitId) — no install record; a future --update will need --assume-local-base");
        } else if let Err(e) = write_install_record(root, &unit_id, &name, version, &hashes) {
            eprintln!("warning: install record not written ({e}) — the next --update will need --assume-local-base");
        }
        println!("imported '{name}' v{version}: {copied} file(s) into .engine/ (install record written)");
        println!("  NOT YET ENFORCED. Run `keel activate {name}` to turn its guards on, then `keel validate . && keel guard .`.");
        return 0;
    }

    // ── --update: the three-way supersession ────────────────────────────────────────────────
    if unit_id.is_empty() {
        eprintln!("error: --update needs a unitId in unit.toml (D0183 identity) — this bundle predates it.");
        return 1;
    }
    let base = match install_record(root, &unit_id) {
        Some((_, files)) => Some(files),
        None if assume_base => None, // bootstrap: local IS the base, human said so explicitly
        None => {
            eprintln!("error: no install record for unit {unit_id} — this install predates D0183.");
            eprintln!("  Bootstrap the base EXPLICITLY: re-run with --assume-local-base (treats your local files");
            eprintln!("  as the base, so upstream changes land and any local-vs-upstream conflict refuses).");
            return 1;
        }
    };
    // assume-local-base: an absent record means base == local by explicit declaration
    let base_hash = |rel: &str| -> Option<String> {
        base.as_ref().and_then(|files| files.iter().find(|(p, _)| p == rel).map(|(_, h)| h.clone()))
    };
    let mut updated = 0u32;
    let mut kept_local = 0u32;
    let mut conflicts: Vec<String> = Vec::new();
    let mut hashes: Vec<(String, String)> = Vec::new();
    for entry in walk(&dir) {
        let Ok(rel) = entry.strip_prefix(&dir) else { continue };
        if rel == Path::new("unit.toml") {
            continue;
        }
        let rel_s = rel.to_string_lossy().replace('\\', "/");
        let dst = root.join(".engine").join(rel);
        let upstream = file_hash(&entry);
        let local = dst.exists().then(|| file_hash(&dst));
        let base_h = base_hash(&rel_s).or_else(|| local.clone()); // assume-local-base fallback
        match (local, base_h) {
            (None, _) => {
                // new upstream file — lands
                if let Some(p) = dst.parent() {
                    let _ = std::fs::create_dir_all(p);
                }
                let _ = std::fs::copy(&entry, &dst);
                updated += 1;
                hashes.push((rel_s, upstream));
            }
            (Some(l), Some(b)) if l == b => {
                // local untouched since base — take upstream
                let _ = std::fs::copy(&entry, &dst);
                updated += 1;
                hashes.push((rel_s, upstream));
            }
            (Some(l), _) if l == upstream => {
                // already identical — nothing to do
                hashes.push((rel_s, upstream));
            }
            (Some(_), Some(b)) if b == upstream => {
                // upstream unchanged since base; LOCAL additions survive (the assert-constraint case)
                kept_local += 1;
                hashes.push((rel_s.clone(), file_hash(&dst)));
            }
            (Some(_), _) => {
                conflicts.push(rel_s);
            }
        }
    }
    if !conflicts.is_empty() {
        eprintln!("error: three-way DIVERGENCE on {} file(s) — refusing (a silent pick would clobber someone):", conflicts.len());
        for c in &conflicts {
            eprintln!("  {c}: local, base, and upstream all differ — reconcile by hand, then re-run");
        }
        return 1;
    }
    if let Err(e) = write_install_record(root, &unit_id, &name, version, &hashes) {
        eprintln!("warning: install record not refreshed: {e}");
    }
    println!("updated '{name}' to v{version}: {updated} file(s) from upstream, {kept_local} kept local additions.");
    println!("  SUPERSESSION RECORD: commit this under the D0070 keystone (a marked Decision) — that commit is");
    println!("  what governing-version/reprocess-candidates resolve prior work against (K9/K10).");
    println!("  Then `keel validate . && keel guard .` and re-check `keel reprocess-candidates`.");
    0
}

/// `keel process <list|search|show|export|import>`.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    match args.first().map(String::as_str) {
        Some("list") | None => {
            let all = rows(root);
            // State the population and its split (issue241/issue239): a bare total invited the
            // reader to think the switchable subset WAS the process set.
            let switchable = all.iter().filter(|r| r.switchable).count();
            println!(
                "processes ({} declared; {switchable} assert guards and are switchable, {} assert none):",
                all.len(),
                all.len() - switchable
            );
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// THE CONTROL for issue241: the catalogue reports every DECLARED process, not the
    /// guard-bearing subset. `rows()` once iterated `unit_names()`, so `keel process show intake`
    /// answered "no process 'intake'" while the file existed — denying 12 of 23 and breaking
    /// `srPortModularProcessUnit` (a unit must travel whole; carrying enforcement is not a
    /// precondition of travelling).
    #[test]
    fn catalogue_reports_every_declared_process_not_only_guard_bearing_units() {
        let root = Path::new("..");
        let all = super::rows(root);
        let declared = crate::activation::declared_processes(root);
        assert_eq!(all.len(), declared.len(), "the catalogue must cover every declared process");
        for d in &declared {
            assert!(all.iter().any(|r| &r.name == d), "declared process `{d}` missing from the catalogue");
        }
        // And the guard-less ones are present AND not reported as switched-off.
        let guardless: Vec<&super::Row> = all.iter().filter(|r| !r.switchable).collect();
        assert!(!guardless.is_empty(), "this repo declares guard-less processes - the split must be observable");
        for r in &guardless {
            assert!(r.active, "a guard-less process has no guard to switch, so it must never read INACTIVE ({})", r.name);
        }
    }

    /// A guard-less process must export WHOLE — definition AND deploying skill. `skills` came only
    /// from the unit, which is `None` without a guard, so `export intake` wrote a bundle with the
    /// process file and no SKILL.md: srPortModularProcessUnit half-met in the other direction.
    #[test]
    fn a_guardless_process_still_carries_its_deploying_skill() {
        let root = Path::new("..");
        let all = super::rows(root);
        let intake = all.iter().find(|r| r.name == "intake").expect("intake is declared");
        assert!(!intake.switchable, "intake asserts no guard - re-point this test if that changed");
        assert!(
            !intake.skills.is_empty(),
            "a guard-less process must still resolve its deploying skill, or its export leaves the skill behind"
        );
        assert!(
            super::unit_files(root, intake).iter().any(|p| p.ends_with("SKILL.md")),
            "the files that MOVE with a guard-less unit must include its SKILL.md"
        );
    }
}
