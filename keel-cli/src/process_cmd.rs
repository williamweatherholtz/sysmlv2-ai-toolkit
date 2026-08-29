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

/// Retire a central skills-registry declaration that an incoming per-skill file SUPERSEDES (D0222).
///
/// FOUND BY ADOPTING, not by reading. A project inited before the per-skill migration still declares
/// the skill in the shared `.engine/skills/skills-registry.sysml`. Importing a unit that carries its
/// own `registry.sysml` then puts the SAME id in two files and `duplicate-identity` fails in the
/// receiving project — so the unit travelled whole and still landed red, for a different reason than
/// issue252. Every project on an older engine, penumbra included, would have hit this.
///
/// The resolution is a REMOVAL of an exact-id duplicate, which is deterministic: there is no question
/// what to delete or how to merge. That is why this writes to a shared file where `import` otherwise
/// refuses to — appending content could conflict, but retiring a superseded duplicate cannot. It is
/// announced on stdout rather than done silently, because an import that edits a file the importer
/// did not name must say so.
///
/// Returns the names it retired.
fn retire_superseded_central(root: &Path, incoming_ids: &[String]) -> Vec<String> {
    let central_path = root.join(".engine/skills/skills-registry.sysml");
    let Ok(mut text) = std::fs::read_to_string(&central_path) else { return Vec::new() };
    let mut retired = Vec::new();
    for id in incoming_ids {
        // The block declaring this id, brace-matched from its `part` head.
        let Some(idpos) = text.find(&format!("\"{id}\"")) else { continue };
        let Some(head) = text[..idpos].rfind("    part ") else { continue };
        let Some(open_rel) = text[head..].find('{') else { continue };
        let open = head + open_rel;
        let mut depth = 0usize;
        let mut end = None;
        for (i, c) in text[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(open + i + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { continue };
        let name = text[head..open]
            .trim_start()
            .trim_start_matches("part ")
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();
        text = format!("{}{}", &text[..head], &text[end..]);
        retired.push(name);
    }
    if !retired.is_empty() {
        let _ = std::fs::write(&central_path, text);
    }
    retired
}

/// The element ids a bundle's per-skill registry files declare — what might already be declared
/// centrally in the receiving project.
fn incoming_registry_ids(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for entry in walk(dir) {
        if entry.file_name().and_then(|n| n.to_str()) != Some("registry.sysml") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&entry) else { continue };
        for line in text.lines() {
            if let Some(rest) = line.trim().strip_prefix(":>> id = \"") {
                if let Some(id) = rest.split('"').next() {
                    if !id.is_empty() {
                        out.push(id.to_string());
                    }
                }
            }
        }
    }
    out
}

/// Where a bundle entry is RESTORED in the receiving project (D0219) — the inverse of `bundle_rel`.
///
/// A first component starting with `.` is a REPO-relative extra (`.github/workflows/...`) and lands
/// at the project root; anything else is engine-relative and lands under `.engine/`.
///
/// Shared by the fresh-import and `--update` loops, which each had their own `.engine`-join. The
/// update path therefore wrote `.github/workflows/x.yml` to `.engine/.github/workflows/x.yml` —
/// inert, in the wrong place, while reporting "6 file(s) from upstream". Two loops with one rule
/// between them is how that happened, so there is now one function and no second opinion.
/// Where a bundle-relative unit file lands in a RECEIVING project.
///
/// Public so `adoption-check` maps a bundle onto its fixture through the SAME function `import`
/// uses. Two copies of this mapping would be the same-fact-in-two-places defect that has cost
/// this project more than any other (issue253, issue260) - and a fixture that resolved paths
/// differently from the real importer would be testing something no adopter ever runs.
#[must_use]
pub fn restore_dst(root: &Path, rel: &Path) -> PathBuf {
    let repo_relative = rel
        .components()
        .next()
        .is_some_and(|c| c.as_os_str().to_string_lossy().starts_with('.'));
    if repo_relative { root.join(rel) } else { root.join(".engine").join(rel) }
}

/// Where a unit file lands INSIDE the bundle (D0219).
///
/// An engine file stays engine-relative, so a bundle keeps its familiar `processes/` and `skills/`
/// layout. An EXTRA lives outside `.engine` (a workflow, a script), for which `strip_prefix(.engine)`
/// fails - and the old `unwrap_or(f)` fell back to the ABSOLUTE path, which `join` discards the base
/// for, so the copy target became the source file and export died copying a file onto itself. An
/// extra therefore keeps its REPO-relative path and round-trips to the same place on import.
fn bundle_rel(root: &Path, f: &Path) -> PathBuf {
    f.strip_prefix(root.join(".engine"))
        .map_or_else(|_| f.strip_prefix(root).unwrap_or(f).to_path_buf(), Path::to_path_buf)
}

/// The declared EXTRA files a unit needs to run, and the prerequisites an importer must satisfy
/// (D0219), from `.engine/contracts/unit-extras.toml`. Returns `(files, requires)`.
///
/// srPortModularProcessUnit says a unit travels "without leaving its enforcement behind", and the
/// decision-channel unit was leaving five files behind - two workflows and two scripts that ARE the
/// mechanism. Declared as data rather than Rust so a project adds its own units without a fork.
fn unit_extras(root: &Path, unit: &str) -> (Vec<String>, Vec<String>) {
    let Ok(text) = std::fs::read_to_string(root.join(".engine/contracts/unit-extras.toml")) else {
        return (Vec::new(), Vec::new());
    };
    let (mut files, mut requires) = (Vec::new(), Vec::new());
    let mut in_unit = false;
    let mut list: Option<&mut Vec<String>> = None;
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('#') {
            continue;
        }
        if let Some(name) = l.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
            in_unit = name == unit;
            list = None;
            continue;
        }
        if !in_unit {
            continue;
        }
        if l.starts_with("files") {
            list = Some(&mut files);
            continue;
        }
        if l.starts_with("requires") {
            list = Some(&mut requires);
            continue;
        }
        if l == "]" {
            list = None;
            continue;
        }
        if let Some(target) = list.as_deref_mut() {
            let v = l.trim_end_matches(',').trim().trim_matches('"');
            if !v.is_empty() {
                target.push(v.to_string());
            }
        }
    }
    (files, requires)
}

fn unit_files(root: &Path, r: &Row) -> Vec<PathBuf> {
    let e = root.join(".engine");
    let mut files = vec![e.join("processes").join(format!("{}.sysml", r.name))];
    for s in &r.skills {
        files.push(e.join("skills").join(s).join("SKILL.md"));
        // D0220: the skill's own registry declaration, so the unit carries what binds skill->process.
        files.push(e.join("skills").join(s).join("registry.sysml"));
    }
    // Rules live in shared files, so a rule is carried by NAME in the manifest rather than by
    // copying a file that also holds other processes' rules. Splitting them per process would be a
    // schema-shaped change, not an export detail — recorded rather than done silently.
    //
    // D0219: plus the declared EXTRAS — the files the unit needs to RUN. Without these the
    // decision-channel unit exported its definition and skill while both workflows and both scripts
    // stayed home, so the receiving project got a described process and no mechanism.
    for extra in unit_extras(root, &r.name).0 {
        files.push(root.join(&extra));
    }
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

/// The manifest key for one unit file: repository-relative, ALWAYS (issue301).
///
/// # Why this is a fallible function and not a `strip_prefix().unwrap_or(path)`
///
/// It was that, and the fallback was the defect. `unit_files` puts a unit's declared EXTRAS at
/// `root/<extra>` rather than under `.engine`, so stripping only the `.engine` prefix FAILS for every
/// extra — and falling back to the unstripped path wrote this machine's home directory into a
/// committed contract. Four such keys landed under the `decision-channel` unit:
/// `file.C:__SL__Users__SL__<user>__SL__claude_code__SL__...`.
///
/// That was survivable while the manifest only ever lived in one repository. Under D0250 the library
/// is a git repository other machines clone, so a key naming the exporting machine resolves to
/// nothing on the importing one — and the three-way base `--update` merges against is exactly what
/// silently stops being found. The failure lands in the one file whose entire purpose is portability.
///
/// Returns `Err` rather than absolutising: a path outside the project is a unit the receiving project
/// could not reconstruct anyway, so the honest outcome is a refusal naming the path, not a key that
/// works on one machine.
fn manifest_key(root: &Path, path: &Path) -> Result<String, String> {
    let engine = root.join(".engine");
    let rel = path
        .strip_prefix(&engine)
        .or_else(|_| path.strip_prefix(root))
        .map_err(|_| format!("{} is outside the project root {}", path.display(), root.display()))?;
    Ok(rel.to_string_lossy().replace('\\', "/"))
}


/// Do two (path, hash) sets describe the same content? Order-insensitive, because the manifest is
/// read back in file order and that is not the order the caller assembled them in.
///
/// Split out from [`install_is_current`] so the comparison is testable without a filesystem: the
/// disk read is the part that cannot be exercised in a unit test, and it is not the part that was
/// wrong.
fn hashes_match(recorded: &[(String, String)], files: &[(String, String)]) -> bool {
    if recorded.len() != files.len() {
        return false;
    }
    let mut a: Vec<_> = recorded.iter().map(|(p, h)| (p.as_str(), h.as_str())).collect();
    let mut b: Vec<_> = files.iter().map(|(p, h)| (p.as_str(), h.as_str())).collect();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

/// Every unit file as a `(repository-relative key, content hash)` pair, or `None` after reporting
/// the file that could not be made relative.
///
/// Returns `None` rather than absolutising, and reports before anything is written: a half-stamped
/// manifest is worse than a refused export (issue301).
fn manifest_hashes(root: &Path, files: &[PathBuf]) -> Option<Vec<(String, String)>> {
    let mut out = Vec::with_capacity(files.len());
    for f in files {
        match manifest_key(root, f) {
            Ok(rel) => out.push((rel, file_hash(f))),
            Err(msg) => {
                eprintln!("error: cannot record unit file — {msg}");
                eprintln!("  a unit file must live inside the project so a receiving project can reconstruct it.");
                return None;
            }
        }
    }
    Some(out)
}

/// The version to record: unchanged when the content is unchanged (issue302), else one higher.
fn next_unit_version(root: &Path, unit_id: &str, hashes: &[(String, String)]) -> u32 {
    match install_record(root, unit_id) {
        Some((v, recorded)) if hashes_match(&recorded, hashes) => v,
        Some((v, _)) => v + 1,
        None => 1,
    }
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

/// `keel process publish <name>` — export the unit into the machine-local library clone and COMMIT
/// (D0250 clause D). The LOUD direction: consuming the library is silent because its content governs
/// nothing until activated; writing to it changes what every other machine will consume, so a publish
/// is always one visible commit naming unit and version — and it NEVER pushes, because the push is
/// the human-visible act (or `keel land` run in the library clone).
///
/// An unchanged unit publishes NOTHING, stated (the issue302 semantics at library scale): a commit
/// that moves no content makes the library log useless as a review surface of what actually changed.
fn cmd_publish(args: &[String], root: &Path) -> i32 {
    let Some(name) = args.get(1) else {
        eprintln!("usage: keel process publish <name>   (exports the unit into the library clone and commits; push separately)");
        return 2;
    };
    let Some(clone) = crate::library::clone_dir().filter(|d| d.join(".git").exists()) else {
        eprintln!("library: not initialised on this machine — `keel library init <remote>` first");
        return 2;
    };
    let dst = clone.join(name);
    // Export INTO the clone. The export path already refuses an undeclared process, writes the whole
    // unit (definition + skill + rules + extras + unit.toml), and NESTS <out>/<name> itself — so it
    // is handed the clone root, one exporter and one layout, not two.
    let export_args = vec!["export".to_string(), (*name).clone(), "--out".to_string(), clone.to_string_lossy().to_string()];
    let code = cmd_export(&export_args, root);
    if code != 0 {
        return code;
    }
    // Anything to commit? `git status --porcelain -- <unit dir>` scopes the question to this unit.
    let status = crate::gitx::git()
        .arg("-C")
        .arg(&clone)
        .args(["status", "--porcelain", "--"])
        .arg(name)
        .output();
    let dirty = status.as_ref().is_ok_and(|o| !String::from_utf8_lossy(&o.stdout).trim().is_empty());
    if !dirty {
        println!("publish: `{name}` is UNCHANGED in the library — nothing to publish (a no-op version must not move, issue302).");
        return 0;
    }
    let version = std::fs::read_to_string(dst.join("unit.toml"))
        .ok()
        .and_then(|t| t.lines().find_map(|l| l.trim().strip_prefix("version = ").map(str::to_string)))
        .unwrap_or_else(|| "?".to_string());
    for step in [vec!["add", "-A", "--", name.as_str()], vec!["-c", "commit.gpgsign=false", "commit", "-q", "-m", &format!("publish {name} v{version}")]] {
        let out = crate::gitx::git().arg("-C").arg(&clone).args(&step).output();
        match out {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                eprintln!("publish: git {step:?} failed: {}", String::from_utf8_lossy(&o.stderr).trim());
                return 1;
            }
            Err(e) => {
                eprintln!("publish: git failed to run: {e}");
                return 1;
            }
        }
    }
    println!("publish: `{name}` v{version} committed to the library clone at {} — NOT pushed.", clone.display());
    println!("  Push when ready: git -C {} push   (the push is the human-visible act, D0250 clause D)", clone.display());
    0
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
                // D0219: an extra lives OUTSIDE .engine (a workflow, a script). strip_prefix then
                // FAILS, and the old `unwrap_or(f)` fell back to the ABSOLUTE path - which
                // `dst.join(..)` discards the base for, so the copy target became the SOURCE and the
                // export died with "used by another process" copying a file onto itself. Engine
                // files stay engine-relative (bundle keeps processes/, skills/); everything else
                // keeps its REPO-relative path, so `.github/...` round-trips to the same place.
                let rel_owned = bundle_rel(root, f);
                let rel = rel_owned.as_path();
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
            // The hashes are computed BEFORE the version, because the version depends on them
            // (issue302: it advances only when a byte moved). A key that cannot be made
            // repository-relative refuses here, before anything is written (issue301).
            let Some(exported_hashes) = manifest_hashes(root, &files) else { return 1 };
            let version = next_unit_version(root, &unit_id, &exported_hashes);
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
    // `--from-library <name>` resolves the machine-local cache (D0250) and then delegates to the
    // ONE import path below — the library changes where a bundle comes from, never what importing
    // it means.
    let dir = if let Some(i) = args.iter().position(|a| a == "--from-library") {
        let Some(name) = args.get(i + 1) else {
            eprintln!("usage: keel process import --from-library <name> [--update] [--degrade] [--assume-local-base]");
            return 2;
        };
        match crate::library::resolve_unit(name) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{e}");
                return 2;
            }
        }
    } else if let Some(d) = args.get(1).filter(|a| !a.starts_with("--")).map(PathBuf::from) {
        d
    } else {
        eprintln!("usage: keel process import <dir> | --from-library <name>  [--update] [--degrade] [--assume-local-base]");
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
            let dst = restore_dst(root, rel);
            if let Some(p) = dst.parent() {
                let _ = std::fs::create_dir_all(p);
            }
            if std::fs::copy(&entry, &dst).is_ok() {
                copied += 1;
                hashes.push((rel.to_string_lossy().replace('\\', "/"), file_hash(&dst)));
            }
        }
        // D0222: an incoming per-skill registry file SUPERSEDES a central declaration of the same
        // id. Retire it, or duplicate-identity fails in this project and the unit lands red.
        for n in retire_superseded_central(root, &incoming_registry_ids(&dir)) {
            println!("  retired the superseded central skills-registry declaration for `{n}` (D0222)");
        }
        if unit_id.is_empty() {
            eprintln!("note: pre-D0183 unit (no unitId) — no install record; a future --update will need --assume-local-base");
        } else if let Err(e) = write_install_record(root, &unit_id, &name, version, &hashes) {
            eprintln!("warning: install record not written ({e}) — the next --update will need --assume-local-base");
        }
        println!("imported '{name}' v{version}: {copied} file(s) (engine files under .engine/, declared extras at the project root; install record written)");
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
        let dst = restore_dst(root, rel);
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
    for n in retire_superseded_central(root, &incoming_registry_ids(&dir)) {
        println!("  retired the superseded central skills-registry declaration for `{n}` (D0222)");
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
/// `keel process audit` — would each unit land GREEN in a project that does not already have it?
/// (D0222, after issue252.)
///
/// WHY THIS IS COMPUTED RATHER THAN INSPECTED. Twice in one session a unit was declared
/// transferable and was not: the decision-channel unit left its whole mechanism behind (issue251),
/// and then left behind the registry entry binding skill to process, so `process-skill` FAILED in
/// the receiving project on its first run (issue252). Both were found by ACTUALLY adopting, not by
/// reading the export. A portability claim nobody can re-derive is exactly the class this engine
/// exists to kill, so the claim is now a computation any project can run before it promises
/// anything.
///
/// The audit answers ONE question per unit: if a project that lacks this process imports it, does
/// its gate stay green? It reports the reason when the answer is no, and never guesses — every
/// check reads the tree.
fn audit(root: &Path) -> i32 {
    let rows = rows(root);
    let central =
        std::fs::read_to_string(root.join(".engine/skills/skills-registry.sysml")).unwrap_or_default();
    let mut red = 0u32;
    println!("process audit: would each unit land GREEN in a project that lacks it? (D0222)");
    println!("  a unit TRAVELS WHOLE when it carries its definition, its deploying skill, and the");
    println!("  declaration binding them - plus every extra it declares. Anything else lands red.");
    println!();
    for r in &rows {
        let mut problems: Vec<String> = Vec::new();
        // 1. A deploying skill at all: a process nothing deploys is inert (D0059), and the receiving
        //    project's process-skill fails on arrival.
        if r.skills.is_empty() {
            problems.push("no deploying skill - process-skill fails on arrival (D0059)".to_string());
        }
        // 2. The declaration BINDING skill to process must travel. issue252: by default it lives in
        //    the shared central registry, and a shared file cannot travel with one unit.
        for s in &r.skills {
            if root.join(".engine/skills").join(s).join("registry.sysml").exists() {
                continue;
            }
            if central.contains(&format!(".engine/processes/{}.sysml", r.name)) {
                problems.push(format!(
                    "skill `{s}` is declared ONLY in the central skills-registry, which cannot travel - the receiving project's process-skill goes RED (issue252). Move the entry to .engine/skills/{s}/registry.sysml"
                ));
            } else {
                problems.push(format!("skill `{s}` has no registry declaration anywhere"));
            }
        }
        // 3. A declared extra that is absent on disk would travel silently missing.
        let (extras, requires) = unit_extras(root, &r.name);
        for x in &extras {
            if !root.join(x).exists() {
                problems.push(format!(
                    "declared extra `{x}` is absent on disk - it would travel silently missing"
                ));
            }
        }
        // 4. A carried file must not cite a path the adopter will not have. `.engine/tools/` is
        //    engine-dev-only and `init` never ships it, so a travelling file referencing it is a dead
        //    reference in THEIR tree - tool-reference then fails on arrival. Found the hard way: the
        //    per-skill registry files cited the migration transform that produced them.
        for f in unit_files(root, r) {
            let Ok(text) = std::fs::read_to_string(&f) else { continue };
            // Narrowed THREE times, each against the real corpus - and the third correction came
            // from a test rather than from me. (1) A DIRECTORY convention ("write a codemod under
            // .engine/tools/migrations/") is not a dead reference; the adopter creates it. (2) A
            // PLACEHOLDER ("<script>.py") documents a convention. (3) Two tools under .engine/tools/
            // ARE deliberately shipped (D0171), so the ship rule is `migrate::is_engine_dev_only`
            // and NOT "anything under tools/" - my first premise was simply wrong, and asking the
            // authority instead of restating it is the fix. A check that fires on a path that WOULD
            // resolve is a check that gets bypassed.
            let cites_a_tool_file = text
                .split(".engine/tools/")
                .skip(1)
                .filter_map(|rest| rest.split_whitespace().next())
                .any(|p| {
                    let p = p.trim_end_matches([',', '.', ')', '`', '"', ';']);
                    // A PLACEHOLDER (`<script>.py`) documents a convention; it is not a reference to
                    // a file that must exist. Narrowed twice now against the real corpus - first for
                    // directory conventions, then for placeholders.
                    if p.contains('<') || p.contains('>') {
                        return false;
                    }
                    let path = std::path::Path::new(p);
                    // Ask the ship rule, never guess it.
                    path.extension().is_some()
                        && crate::migrate::is_engine_dev_only(std::path::Path::new(&format!("tools/{p}")))
                });
            if cites_a_tool_file {
                let rel = f.strip_prefix(root).unwrap_or(&f).to_string_lossy().replace('\\', "/");
                problems.push(format!(
                    "carried file `{rel}` cites `.engine/tools/`, which init never ships - a dead reference in the adopter's tree (tool-reference fails on arrival)"
                ));
            }
        }
        let verdict = if problems.is_empty() { "TRAVELS-WHOLE" } else { "LANDS-RED" };
        if !problems.is_empty() {
            red += 1;
        }
        let prereq = if requires.is_empty() {
            String::new()
        } else {
            format!(", {} stated prerequisite(s)", requires.len())
        };
        println!("  [{verdict}] {}  ({} file(s){prereq})", r.name, unit_files(root, r).len());
        for p in &problems {
            println!("      - {p}");
        }
    }
    println!();
    if red == 0 {
        println!("process audit: every unit travels whole.");
    } else {
        println!(
            "process audit: {red} of {} unit(s) would land RED in a project that lacks them.",
            rows.len()
        );
        println!("  A REPORT, not a gate: a project may legitimately hold units it never exports.");
    }
    0
}

pub fn cmd(args: &[String], root: &Path) -> i32 {
    match args.first().map(String::as_str) {
        Some("audit") => audit(root),
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
            // D0219: an importer must be TOLD what the unit needs beyond its files, or it lands
            // inert and the first symptom is silence.
            let (_, requires) = unit_extras(root, &r.name);
            if !requires.is_empty() {
                println!("             the RECEIVING project must also:");
                for q in &requires {
                    println!("               - {q}");
                }
            }
            println!("             files that MOVE with it:");
            for f in unit_files(root, r) {
                println!("               {}", f.strip_prefix(root).unwrap_or(&f).display().to_string().replace('\\', "/"));
            }
            0
        }
        Some("export") => cmd_export(args, root),
        Some("publish") => cmd_publish(args, root),
        Some("import") => cmd_import(args, root),
        Some(other) => {
            eprintln!("unknown: keel process {other} (expected list | audit | search <term> | show <name> | export <name> --out <dir> | import <dir>)");
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

    /// Sprint 484 / issue301. A unit's declared EXTRAS live at `root/<extra>`, not under `.engine`,
    /// so the old `strip_prefix(".engine").unwrap_or(path)` fell through to the absolute path and
    /// wrote this machine's home directory into a committed contract.
    #[test]
    fn a_unit_file_outside_dot_engine_still_gets_a_repository_relative_key() {
        let root = Path::new("/proj");
        let engine_file = Path::new("/proj/.engine/processes/intake.sysml");
        assert_eq!(super::manifest_key(root, engine_file).unwrap(), "processes/intake.sysml");

        // The regression: an extra at the repository root, which is where D0219 extras live.
        let extra = Path::new("/proj/.github/workflows/decision-record.yml");
        let key = super::manifest_key(root, extra).unwrap();
        assert_eq!(key, ".github/workflows/decision-record.yml");
        assert!(!key.contains("proj"), "the key must not carry the exporting machine's path: {key}");
    }

    /// The other half of issue301: a path that cannot be made relative REFUSES rather than being
    /// absolutised. A receiving project could not reconstruct such a file anyway, so a key that
    /// works only on the exporting machine is worse than a stated failure.
    #[test]
    fn a_unit_file_outside_the_project_is_refused_not_absolutised() {
        let err = super::manifest_key(Path::new("/proj"), Path::new("/elsewhere/thing.yml"))
            .expect_err("a path outside the project must refuse");
        assert!(err.contains("outside the project root"), "the refusal must say why: {err}");
        assert!(err.contains("thing.yml"), "the refusal must name the path: {err}");
    }

    /// Sprint 484 / issue302. The version used to advance on every export whether or not a byte
    /// moved - the `intake` unit went 42 -> 43 in a session that edited none of its files. Under
    /// D0250 `--update` is decided by reading that number, so a version that moves for no reason
    /// makes the honest answer to "should I update" unavailable.
    #[test]
    fn identical_hashes_are_recognised_as_current_and_a_changed_one_is_not() {
        let recorded = vec![
            ("processes/x.sysml".to_string(), "aaaa".to_string()),
            ("skills/x/SKILL.md".to_string(), "bbbb".to_string()),
        ];
        // Order must not matter: the manifest is read back in file order, which is not authoring order.
        let reordered: Vec<(String, String)> = recorded.iter().rev().cloned().collect();
        assert!(super::hashes_match(&recorded, &reordered), "same set in a different order is still current");

        let mut changed = recorded.clone();
        changed[1].1 = "cccc".to_string();
        assert!(!super::hashes_match(&recorded, &changed), "one changed hash means not current");

        let mut extra = recorded.clone();
        extra.push(("extras/new.yml".to_string(), "dddd".to_string()));
        assert!(!super::hashes_match(&recorded, &extra), "an ADDED file means not current");
    }
}
