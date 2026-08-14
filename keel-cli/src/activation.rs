//! Process ACTIVATION (D0138, `srPortActivationManifest`) — what has this project actually adopted?
//!
//! Issues 089 and 090 were the same question wearing different clothes, and both were answered by
//! INFERENCE from file presence: a marker counted as adopted because some file declared it, a rule
//! counted as adopted because its file existed. Inference cannot express intent. A project could not
//! state what it had adopted, could not see what it had NOT adopted, and enforcement shifted silently
//! when a file moved. Both fixes were correct and neither addressed that.
//!
//! Here adoption is a DECLARED fact. `.engine/contracts/activation.toml` names the active processes;
//! `.engine/contracts/process-units.toml` (an engine fact) says what each process brings. A
//! process-bound guard runs only while its process is active, and when it does not run it is REPORTED
//! as not-adopted rather than silently skipped — because "this control is off" is exactly the thing a
//! project must be able to see.
//!
//! TWO INVARIANTS, both load-bearing:
//!   1. **Absent manifest means everything is active.** No existing project changes behaviour by
//!      upgrading, which is the mistake D0133 made and issue089 paid for.
//!   2. **Core guards are not deactivatable.** A guard named in no unit is core and always runs.
//!      Activation exists to stop enforcing PROCEDURES a project has not adopted; it must never become
//!      a switch that makes truthfulness optional (the issue081 all-or-nothing-bypass dynamic).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// What a process brings with it when activated.
#[derive(Debug, Default, Clone)]
pub struct Unit {
    pub skills: Vec<String>,
    pub rules: Vec<String>,
    pub guards: Vec<String>,
}

/// Whether a guard runs, and why.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GuardState {
    /// Named in no unit: model integrity, always enforced, not deactivatable.
    Core,
    /// Owned by a process that IS active.
    Active(String),
    /// Owned by a process the project has NOT activated — skipped, and reported as such.
    Inactive(String),
}

/// The project's activation state plus the engine's unit definitions.
#[derive(Debug, Default)]
pub struct Activation {
    /// `None` = no manifest declared => every process is active (invariant 1).
    active: Option<BTreeSet<String>>,
    units: BTreeMap<String, Unit>,
    /// Fail-loud problems: an unknown process or guard name, or unparseable TOML.
    pub errors: Vec<String>,
}

fn str_list(v: Option<&toml::Value>) -> Vec<String> {
    v.and_then(toml::Value::as_array)
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        .unwrap_or_default()
}

impl Activation {
    /// Read both contract files. Missing files are NOT errors — absence is a legitimate state that
    /// means "everything present is active" (invariant 1), never a violation (the issue090 lesson).
    #[must_use]
    pub fn load(root: &Path) -> Self {
        let mut out = Self::default();

        let units_path = root.join(".engine/contracts/process-units.toml");
        if let Ok(text) = std::fs::read_to_string(&units_path) {
            match text.parse::<toml::Value>() {
                Ok(v) => {
                    if let Some(units) = v.get("units").and_then(toml::Value::as_table) {
                        for (name, body) in units {
                            out.units.insert(
                                name.clone(),
                                Unit {
                                    skills: str_list(body.get("skills")),
                                    rules: str_list(body.get("rules")),
                                    guards: str_list(body.get("guards")),
                                },
                            );
                        }
                    }
                }
                Err(e) => out.errors.push(format!(
                    ".engine/contracts/process-units.toml: unparseable TOML ({e}) — every process unit it declares is inert"
                )),
            }
        }

        let act_path = root.join(".engine/contracts/activation.toml");
        if let Ok(text) = std::fs::read_to_string(&act_path) {
            match text.parse::<toml::Value>() {
                Ok(v) => {
                    // A declared-but-empty `active` list is meaningful (activate nothing) and must be
                    // distinguished from an absent key (activate everything), so match on presence.
                    if let Some(active) = v.get("processes").and_then(|p| p.get("active")) {
                        let set: BTreeSet<String> = str_list(Some(active)).into_iter().collect();
                        for p in &set {
                            if !out.units.contains_key(p) && !process_exists(root, p) {
                                out.errors.push(format!(
                                    ".engine/contracts/activation.toml: activates `{p}`, which is neither a declared process unit nor a process definition in .engine/processes/ — a typo here would silently disable a control"
                                ));
                            }
                        }
                        out.active = Some(set);
                    }
                }
                Err(e) => out.errors.push(format!(
                    ".engine/contracts/activation.toml: unparseable TOML ({e}) — the whole manifest is inert, so every control would silently revert to active"
                )),
            }
        }

        // A unit naming a guard that does not exist is a dead reference: the project believes a control
        // is bound to a process when nothing checks it. Same defect class as issue093.
        for (proc_name, unit) in &out.units {
            for g in &unit.guards {
                if !crate::guards::GUARD_NAMES.contains(&g.as_str()) {
                    out.errors.push(format!(
                        ".engine/contracts/process-units.toml: unit `{proc_name}` names guard `{g}`, which is not an enforced guard — nothing would check it"
                    ));
                }
            }
        }

        out
    }

    /// True when `p` is active. With no manifest, everything is active (invariant 1).
    #[must_use]
    pub fn is_process_active(&self, p: &str) -> bool {
        self.active.as_ref().is_none_or(|set| set.contains(p))
    }

    /// True when this project declared a manifest at all.
    #[must_use]
    pub const fn is_declared(&self) -> bool {
        self.active.is_some()
    }

    /// Classify a guard: core, or owned by an active/inactive process.
    #[must_use]
    pub fn guard_state(&self, guard: &str) -> GuardState {
        for (proc_name, unit) in &self.units {
            if unit.guards.iter().any(|g| g == guard) {
                return if self.is_process_active(proc_name) {
                    GuardState::Active(proc_name.clone())
                } else {
                    GuardState::Inactive(proc_name.clone())
                };
            }
        }
        GuardState::Core
    }

    /// Declared units the project has NOT activated, sorted.
    #[must_use]
    pub fn inactive_processes(&self) -> Vec<String> {
        self.units.keys().filter(|p| !self.is_process_active(p)).cloned().collect()
    }

    /// Every declared unit name, sorted.
    #[must_use]
    pub fn unit_names(&self) -> Vec<String> {
        self.units.keys().cloned().collect()
    }

    /// The unit for a process, if declared.
    #[must_use]
    pub fn unit(&self, p: &str) -> Option<&Unit> {
        self.units.get(p)
    }
}

/// True when `.engine/processes/<name>.sysml` exists — so a project may activate a process it authored
/// itself, with no unit declared, without that counting as a typo.
fn process_exists(root: &Path, name: &str) -> bool {
    root.join(".engine/processes").join(format!("{name}.sysml")).exists()
}

#[cfg(test)]
mod tests {
    use super::{Activation, GuardState};
    use std::path::Path;

    fn write(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, body).unwrap();
    }

    fn tmp(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("keel-activation-{tag}"));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    const UNITS: &str = r#"
[units.issue-resolution]
skills = ["issue-resolution"]
rules = ["issuesTriagedRule"]
guards = ["issues"]
"#;

    #[test]
    fn absent_manifest_activates_everything() {
        // Invariant 1: upgrading must not change an existing project's behaviour.
        let d = tmp("absent");
        write(&d, ".engine/contracts/process-units.toml", UNITS);
        let a = Activation::load(&d);
        assert!(!a.is_declared());
        assert!(a.is_process_active("issue-resolution"));
        assert_eq!(a.guard_state("issues"), GuardState::Active("issue-resolution".into()));
        assert!(a.errors.is_empty());
    }

    #[test]
    fn a_deactivated_process_disables_only_its_own_guards() {
        let d = tmp("subset");
        write(&d, ".engine/contracts/process-units.toml", UNITS);
        write(&d, ".engine/contracts/activation.toml", "[processes]\nactive = []\n");
        let a = Activation::load(&d);
        assert!(a.is_declared());
        assert_eq!(a.guard_state("issues"), GuardState::Inactive("issue-resolution".into()));
        assert_eq!(a.inactive_processes(), vec!["issue-resolution".to_string()]);
    }

    #[test]
    fn core_guards_are_never_deactivatable() {
        // Invariant 2: activation must not become a switch that makes truthfulness optional.
        let d = tmp("core");
        write(&d, ".engine/contracts/process-units.toml", UNITS);
        write(&d, ".engine/contracts/activation.toml", "[processes]\nactive = []\n");
        let a = Activation::load(&d);
        for core in ["duplicate-identity", "marker-vocabulary", "actors", "engine-lint"] {
            assert_eq!(a.guard_state(core), GuardState::Core, "{core} must stay enforced");
        }
    }

    #[test]
    fn a_typo_in_either_file_fails_loud() {
        let d = tmp("typo");
        write(
            &d,
            ".engine/contracts/process-units.toml",
            "[units.issue-resolution]\nguards = [\"isues\"]\n",
        );
        write(&d, ".engine/contracts/activation.toml", "[processes]\nactive = [\"isue-resolution\"]\n");
        let a = Activation::load(&d);
        assert_eq!(a.errors.len(), 2, "both a bad guard name and a bad process name must be reported");
        assert!(a.errors.iter().any(|e| e.contains("isues")));
        assert!(a.errors.iter().any(|e| e.contains("isue-resolution")));
    }
}
