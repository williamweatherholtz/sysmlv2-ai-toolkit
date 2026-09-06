//! `keel onboard` — the computed answer to "has this project chosen its processes, and on what basis?"
//!
//! WHY THIS EXISTS (D0225). Adopting keel onto a second repository exposed the gap: `keel init` lays
//! down a working project with EVERY process active, and nothing ever asks the author what they are
//! building. An author with no basis to judge 24 processes either keeps all of them (and meets
//! friction they never chose — D0054, the top risk) or switches off the ones whose names they do not
//! recognise, which is the worst available selection rule.
//!
//! WHAT IT DOES NOT DO. It is a VIEW, never a gate (D0098): a project may legitimately run
//! unchartered, and refusing to work until someone answers questions is precisely the friction this
//! is meant to remove. It reports; the `project-onboarding` skill acts.
//!
//! THE APPLICABILITY FACT lives BESIDE each process, as an `// APPLIES-WHEN:` line inside its own
//! definition — not in a central table. A central table cannot travel with one unit, which is the
//! defect that made 23 of 24 units land red in a project that lacked them (issue253/D0222). It is a
//! structured comment rather than a schema field because `schema/core` is frozen (invariant 5) — the
//! same shape as D0207's `// RESEARCH:` line.
use std::collections::BTreeMap;
use std::path::Path;

/// One process, as onboarding sees it: what it is for, when it applies, and whether it is on.
pub struct Applicability {
    /// Process name, matching its definition file stem.
    pub process: String,
    /// The `// APPLIES-WHEN:` condition declared beside the process, if it declares one.
    pub when: Option<String>,
    /// `active` | `INACTIVE` | `always` (asserts no guard, so nothing to switch).
    pub state: String,
}

/// Read the `// APPLIES-WHEN:` line declared beside each process definition.
#[must_use]
pub fn applicability(root: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let dir = root.join(".engine").join("processes");
    let Ok(entries) = std::fs::read_dir(&dir) else { return out };
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|x| x.to_str()) != Some("sysml") {
            continue;
        }
        let Some(name) = p.file_stem().and_then(|s| s.to_str()) else { continue };
        let text = std::fs::read_to_string(&p).unwrap_or_default();
        if let Some(when) = text.lines().find_map(|l| l.trim().strip_prefix("// APPLIES-WHEN:")) {
            out.insert(name.to_string(), when.trim().to_string());
        }
    }
    out
}

/// The Decision that charters this project's process set, declared in `activation.toml` as
/// `charteredBy` under `[processes]`.
///
/// Absent means onboarding has not been run — NOT that the project is wrong. A project that never
/// declared an activation file runs everything by default and has violated nothing (D0138/D0164).
#[must_use]
pub fn chartered_by(root: &Path) -> Option<String> {
    let text = std::fs::read_to_string(root.join(".engine").join("contracts").join("activation.toml")).ok()?;
    text.lines()
        .find_map(|l| l.trim().strip_prefix("charteredBy"))
        .and_then(|r| r.split('=').nth(1))
        .map(|v| v.trim().trim_matches('"').to_string())
        .filter(|v| !v.is_empty())
}

/// Does `charter` (`dNNNN`) name a Decision file in THIS project's `.engine/decisions/`?
///
/// issue380 / GH#56: a v0.3.1 `migrate` wrote the engine's own `activation.toml` over a project's, and
/// `onboard` then printed `CHARTERED by d0226` - a Decision that project did not hold. A provenance claim
/// the tree cannot back is stated as such, never as fact.
#[must_use]
pub fn charter_resolves(root: &Path, charter: &str) -> bool {
    let Some(num) = charter.strip_prefix('d').filter(|n| n.len() == 4 && n.chars().all(|c| c.is_ascii_digit())) else {
        return false;
    };
    let prefix = format!("{num}-");
    std::fs::read_dir(root.join(".engine").join("decisions"))
        .is_ok_and(|rd| rd.flatten().any(|e| e.file_name().to_string_lossy().starts_with(&prefix) && e.path().extension().is_some_and(|x| x == "sysml")))
}

/// Every declared process with its applicability and current state, computed.
#[must_use]
pub fn rows(root: &Path) -> Vec<Applicability> {
    let act = crate::activation::Activation::load(root);
    let when = applicability(root);
    crate::activation::declared_processes(root)
        .into_iter()
        .map(|p| {
            let switchable = act.unit(&p).is_some_and(|u| !u.guards.is_empty());
            let state = if switchable {
                if act.is_process_active(&p) { "active" } else { "INACTIVE" }
            } else {
                "always"
            };
            Applicability { when: when.get(&p).cloned(), process: p, state: state.to_string() }
        })
        .collect()
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// `keel onboard [ROOT] [--json]`.
#[must_use]
pub fn cmd(args: &[String]) -> i32 {
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    let rows = rows(&root);
    let charter = chartered_by(&root);
    let undeclared = rows.iter().filter(|r| r.when.is_none()).count();

    if args.iter().any(|a| a == "--json") {
        let items: Vec<String> = rows
            .iter()
            .map(|r| {
                format!(
                    "{{\"process\":\"{}\",\"state\":\"{}\",\"appliesWhen\":{}}}",
                    esc(&r.process),
                    esc(&r.state),
                    r.when.as_ref().map_or_else(|| "null".to_string(), |w| format!("\"{}\"", esc(w)))
                )
            })
            .collect();
        let resolves = charter.as_deref().is_some_and(|c| charter_resolves(&root, c));
        println!(
            "{{\"chartered\":{},\"charteredBy\":{},\"charterResolves\":{resolves},\"declared\":{},\"undeclaredApplicability\":{},\"processes\":[{}]}}",
            charter.is_some() && resolves,
            charter.as_ref().map_or_else(|| "null".to_string(), |c| format!("\"{}\"", esc(c))),
            rows.len(),
            undeclared,
            items.join(",")
        );
        return 0;
    }

    if let Some(d) = &charter {
        if charter_resolves(&root, d) {
            println!("process set: CHARTERED by {d} ({} process(es) declared)", rows.len());
        } else {
            println!("process set: UNRESOLVED CHARTER {d} - activation.toml names a Decision this project does not hold, so the set is NOT chartered here ({} process(es) declared). issue380/GH#56: an engine resync can write another project's charter; re-run the project-onboarding skill or restore your own activation.toml from history.", rows.len());
        }
    } else {
        println!("process set: NOT CHARTERED - nobody has recorded WHY these processes and not others.");
        println!("  Not a violation: an undeclared project runs everything by default (D0138).");
        println!("  To charter it, run the `project-onboarding` skill - it elicits what you are building,");
        println!("  researches the practice where this engine has no process for it, recommends a set with");
        println!("  per-process reasoning you can reject, and records the choice as a Decision.");
    }
    println!();
    for r in &rows {
        println!("  [{:8}] {}", r.state, r.process);
        match &r.when {
            Some(w) => println!("             applies when: {w}"),
            None => println!("             applies when: NOT DECLARED - cannot be recommended for or against"),
        }
    }
    if undeclared > 0 {
        println!();
        println!("{undeclared} process(es) declare no APPLIES-WHEN condition, so onboarding cannot reason about them.");
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{applicability, chartered_by, cmd};

    #[test]
    fn applicability_is_declared_beside_every_process() {
        // The fact travels with the unit or it is useless to an adopting project (D0222/issue253).
        // Population asserted non-empty: a check over zero items is the false green this codebase
        // has already hit twice (issue250, and claude-surface-drift on zero skills).
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let a = applicability(&root);
        let declared = crate::activation::declared_processes(&root);
        assert!(!declared.is_empty(), "the population must be non-empty or this passes vacuously");
        let missing: Vec<&String> = declared.iter().filter(|p| !a.contains_key(*p)).collect();
        assert!(missing.is_empty(), "every declared process must say when it applies; missing: {missing:?}");
    }

    #[test]
    fn a_project_that_never_chartered_reports_so_rather_than_failing() {
        // D0098: onboarding REPORTS, never gates. An unchartered project is not a dishonest one, and
        // refusing to work until someone answers questions is the friction this exists to remove.
        let root = std::env::temp_dir().join("keel-onboard-unchartered");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".engine").join("processes")).unwrap();
        assert_eq!(chartered_by(&root), None);
        assert_eq!(cmd(&[root.to_string_lossy().to_string()]), 0, "reporting an unchartered project is not an error");
        let _ = std::fs::remove_dir_all(&root);
    }
}
