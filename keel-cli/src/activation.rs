//! Process ACTIVATION (D0138, `srPortActivationManifest`) — what has this project actually adopted?
//!
//! Issues 089 and 090 were the same question wearing different clothes, and both were answered by
//! INFERENCE from file presence: a marker counted as adopted because some file declared it, a rule
//! counted as adopted because its file existed. Inference cannot express intent. A project could not
//! state what it had adopted, could not see what it had NOT adopted, and enforcement shifted silently
//! when a file moved. Both fixes were correct and neither addressed that.
//!
//! Here adoption is a DECLARED fact. `.engine/contracts/activation.toml` names the active processes;
//! what each process BRINGS is read from the MODEL — `assert constraint` members on the parts in its
//! `.engine/processes/` file (D0139(D)). That mapping used to live in `process-units.toml`, which no
//! `keel trace`, declared view or viewpoint could reach, so an engine fact sat outside the model. A
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
/// `active_viewpoints` mirrors `active`: `None` means every declared viewpoint is active (D0164).
#[derive(Debug, Default)]
pub struct Activation {
    /// `None` = no manifest declared => every process is active (invariant 1).
    active: Option<BTreeSet<String>>,
    units: BTreeMap<String, Unit>,
    /// Fail-loud problems: an unknown process or guard name, or unparseable TOML.
    pub errors: Vec<String>,
    /// Declared viewpoints the project has activated. `None` = all (D0164).
    pub active_viewpoints: Option<BTreeSet<String>>,
}

/// Read each process unit's guards FROM THE MODEL (D0139(D), replacing `process-units.toml`).
///
/// A unit is a FILE in `.engine/processes/`; its guards are the union of `assert constraint <m> : <C>;`
/// across every part in that file. Asserting on the specific step that enforces a control — ceremony on
/// the standup gate, retro-backlog on `retroTrack` — is strictly more informative than the flat
/// file-level list the TOML held, and unlike the TOML it is reachable by `keel trace` and every declared
/// view, which is the whole argument of D0139: an engine fact belongs in the model.
///
/// The constraint def name is the camelCase form of the guard name, so `sprintCoverage` recovers
/// `sprint-coverage` by one mechanical rule with no lookup table to drift.
fn units_from_model(root: &Path) -> BTreeMap<String, Unit> {
    let dir = root.join(".engine/processes");
    let mut units = BTreeMap::new();
    for path in crate::collect_sysml(&dir) {
        let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().to_string()) else { continue };
        let Ok(pkg) = crate::parse_pkg(&path) else { continue };
        let mut guards: Vec<String> = Vec::new();
        for item in &pkg.items {
            // A process part may be a `part` or — after the D0143 retype — a typed `action` usage.
            // Both carry the asserts, so both are read; keying on the member rather than the metaclass
            // means the retype cannot silently drop a control.
            let members: &[keel_parser::ast::MemberFeature] = match item {
                keel_parser::ast::Item::Part(p) => &p.members,
                keel_parser::ast::Item::ActionUsage(a) => &a.members,
                _ => continue,
            };
            for m in members {
                if m.kind == "assert" {
                    if let Some(t) = &m.type_name {
                        guards.push(camel_to_kebab(t));
                    }
                }
            }
        }
        if !guards.is_empty() {
            guards.sort();
            guards.dedup();
            let skills = deploying_skills(root, &stem);
            // D0184/p3aRuleAttribution: a declared rule's owning unit comes from the
            // rule-owners contract - unit.rules is no longer hardcoded empty, so export
            // carries the teeth (P4b) and the association is data a project edits, not Rust.
            let rules = rule_owners(root).into_iter().filter(|(_, owner)| owner == &stem).map(|(r, _)| r).collect();
            units.insert(stem, Unit { skills, rules, guards });
        }
    }
    units
}

/// The skill file(s) that DEPLOY `process`, from the skill registry.
///
/// A unit reported `skills: []` for every process, which made `keel process export` copy the
/// definition and leave the deploying skill behind — exactly the failure srPortModularProcessUnit
/// names ("without leaving its enforcement behind"). The correspondence already exists and is
/// already machine-checked: the `process-skill` guard requires a deploying skill's `purpose` to name
/// the `.engine/processes/<name>.sysml` it deploys, so this reads the same registry by the same
/// convention rather than inventing a second mapping that could disagree with the guard.
fn deploying_skills(root: &Path, process: &str) -> Vec<String> {
    let reg = root.join(".engine").join("skills").join("skills-registry.sysml");
    let Ok(text) = std::fs::read_to_string(reg) else { return Vec::new() };
    let needle = format!(".engine/processes/{process}.sysml");
    let mut out = Vec::new();
    // Each skill is a `part <name> : AISkill { ... }` block; the one whose purpose names this
    // process carries the `location` of its SKILL.md.
    for block in text.split("part ").skip(1) {
        if !block.contains(&needle) {
            continue;
        }
        if let Some(loc) = block.split(":>> location = \"").nth(1).and_then(|s| s.split('"').next()) {
            // Store the skill DIRECTORY name, which is what the exporter joins onto .engine/skills.
            if let Some(dir) = loc.strip_prefix(".engine/skills/").and_then(|s| s.split('/').next()) {
                out.push(dir.to_owned());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// `sprintCoverage` -> `sprint-coverage`. ASCII-only by construction: every guard name is ASCII.
fn camel_to_kebab(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn str_list(v: Option<&toml::Value>) -> Vec<String> {
    v.and_then(toml::Value::as_array)
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        .unwrap_or_default()
}

/// The declared rule -> owning-unit association (D0184).
///
/// Read from `.engine/contracts/rule-owners.toml`, lines `ruleName = "processStem"`. A rule with no
/// owner belongs to the project core (exported by nothing). Changing this file is process
/// definition - the keystone lock covers it.
#[must_use]
pub fn rule_owners(root: &std::path::Path) -> Vec<(String, String)> {
    let Ok(text) = std::fs::read_to_string(root.join(".engine").join("contracts").join("rule-owners.toml")) else {
        return Vec::new();
    };
    text.lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| {
            let (rule, owner) = l.split_once('=')?;
            let owner = owner.trim().trim_matches('"');
            (!owner.is_empty()).then(|| (rule.trim().to_string(), owner.to_string()))
        })
        .collect()
}

impl Activation {
    /// Read both contract files. Missing files are NOT errors — absence is a legitimate state that
    /// means "everything present is active" (invariant 1), never a violation (the issue090 lesson).
    #[must_use]
    pub fn load(root: &Path) -> Self {
        let mut out = Self { units: units_from_model(root), ..Self::default() };

        let act_path = root.join(".engine/contracts/activation.toml");
        if let Ok(text) = std::fs::read_to_string(&act_path) {
            match text.parse::<toml::Value>() {
                Ok(v) => {
                    // A declared-but-empty `active` list is meaningful (activate nothing) and must be
                    // distinguished from an absent key (activate everything), so match on presence.
                    // VIEWPOINT activation (D0164), the same shape and the same rule as processes: an
                    // absent key means everything is active, a declared-but-empty list means activate
                    // nothing, so presence must be matched rather than emptiness.
                    if let Some(active) = v.get("viewpoints").and_then(|p| p.get("active")) {
                        out.active_viewpoints =
                            Some(str_list(Some(active)).into_iter().collect::<BTreeSet<String>>());
                    }
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
                        ".engine/processes/{proc_name}.sysml: asserts constraint `{g}`, which is not an enforced guard — nothing would check it"
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
        // A guard is ACTIVE if ANY active unit asserts it (D0183 killing first-match-wins): two
        // units may share a guard, and the first-registered one being deactivated must not switch
        // off enforcement another active unit still owns.
        let mut inactive_owner: Option<String> = None;
        for (proc_name, unit) in &self.units {
            if unit.guards.iter().any(|g| g == guard) {
                if self.is_process_active(proc_name) {
                    return GuardState::Active(proc_name.clone());
                }
                inactive_owner.get_or_insert_with(|| proc_name.clone());
            }
        }
        inactive_owner.map_or(GuardState::Core, GuardState::Inactive)
    }

    /// True when viewpoint `v` is active. With no manifest key, every viewpoint is active (D0164).
    ///
    /// A viewpoint is a declared LENS, so deactivating it means the project does not look through it: it
    /// stops being offered on a surface and its renderer stops being required to resolve. It does NOT
    /// delete the declaration, and `concern-coverage` still reports the concern - an unwatched concern is
    /// a fact about the project, and hiding it would turn a deliberate choice into an invisible gap.
    #[must_use]
    pub fn is_viewpoint_active(&self, v: &str) -> bool {
        self.active_viewpoints.as_ref().is_none_or(|set| set.contains(v))
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

/// Every process declared on disk, sorted — INCLUDING those that assert no constraint (issue149).
///
/// A "unit" exists only for a process that asserts at least one guard, because activation switches
/// GUARDS. That is coherent, and it made `keel activation` report 6 of this repository's 18 processes
/// with nothing to indicate the other 12 existed, while `keel deactivate` refused them as "not a
/// declared process unit" — which reads as "no such process" rather than "that process carries no
/// deactivatable guards". The mechanism was right and its surface was misleading, so the surface now
/// reports the whole set and says which part of it is switchable.
#[must_use]
pub fn declared_processes(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = crate::collect_sysml(&root.join(".engine").join("processes"))
        .iter()
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
        .collect();
    out.sort();
    out
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

    /// issue149: the reported process set must be the set ON DISK, not the switchable subset. This
    /// repository has 18 process files and 6 guard-bearing units; reporting 6 told a reader it had 6
    /// processes, and refusing the other 12 as "not a declared process unit" told them those did not
    /// exist. The unit set is deliberately narrower - activation switches guards - so the two must be
    /// separately observable rather than one standing in for the other.
    #[test]
    fn declared_processes_is_every_file_not_only_the_guard_bearing_units() {
        let root = Path::new("..");
        let all = super::declared_processes(root);
        let act = Activation::load(root);
        let units = act.unit_names();
        assert!(
            all.len() > units.len(),
            "this repository has guard-less processes, so the declared set must exceed the unit set              (all={}, units={})",
            all.len(),
            units.len()
        );
        for u in &units {
            assert!(all.contains(u), "every guard-bearing unit `{u}` must also be a declared process");
        }
        assert!(
            all.iter().any(|p| p == "knowledge-graph-memory"),
            "a process with no asserted guard must still be reported as declared"
        );
    }

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

    /// A process unit expressed the way units are now declared (D0139(D)): a process file whose parts
    /// ASSERT the constraints they enforce. Replaces the `process-units.toml` fixture these tests used
    /// before the mapping moved into the model.
    const PROCESS_FILE: &str = r"
package ProbeIssueResolution {
    part issueResolution : Process {
        assert constraint enforcesIssues : issues;
    }
}
";

    fn with_unit(tag: &str) -> std::path::PathBuf {
        let d = tmp(tag);
        write(&d, ".engine/processes/issue-resolution.sysml", PROCESS_FILE);
        d
    }

    #[test]
    fn absent_manifest_activates_everything() {
        // Invariant 1: upgrading must not change an existing project's behaviour.
        let d = with_unit("absent");
        let a = Activation::load(&d);
        assert!(!a.is_declared());
        assert!(a.is_process_active("issue-resolution"));
        assert_eq!(a.guard_state("issues"), GuardState::Active("issue-resolution".into()));
        assert!(a.errors.is_empty());
    }

    #[test]
    fn unit_guards_come_from_the_model() {
        // The mapping is read from `assert constraint` in the process file, and the constraint def name
        // is the camelCase form of the guard name. Asserts that the camel->kebab rule actually runs.
        let d = tmp("camel");
        write(
            &d,
            ".engine/processes/agile-workflow.sysml",
            r"
package ProbeAgile {
    part standupGate : ProcessStep {
        assert constraint enforcesSprintCoverage : sprintCoverage;
    }
}
",
        );
        let a = Activation::load(&d);
        assert_eq!(a.guard_state("sprint-coverage"), GuardState::Active("agile-workflow".into()));
    }

    #[test]
    fn a_deactivated_process_disables_only_its_own_guards() {
        let d = with_unit("subset");
        write(&d, ".engine/contracts/activation.toml", "[processes]
active = []
");
        let a = Activation::load(&d);
        assert!(a.is_declared());
        assert_eq!(a.guard_state("issues"), GuardState::Inactive("issue-resolution".into()));
        assert_eq!(a.inactive_processes(), vec!["issue-resolution".to_string()]);
    }

    #[test]
    fn core_guards_are_never_deactivatable() {
        // Invariant 2: activation must not become a switch that makes truthfulness optional.
        let d = with_unit("core");
        write(&d, ".engine/contracts/activation.toml", "[processes]
active = []
");
        let a = Activation::load(&d);
        for core in ["duplicate-identity", "marker-vocabulary", "actors", "engine-lint"] {
            assert_eq!(a.guard_state(core), GuardState::Core, "{core} must stay enforced");
        }
    }

    #[test]
    fn a_typo_in_the_activation_manifest_fails_loud() {
        // A misspelled PROCESS name still fails loud. The other half of the old test — a misspelled
        // GUARD name in the units file — is now caught upstream as an unresolved constraint TYPE by
        // `keel validate`, which is strictly stronger than a name-list check, so it is verified there
        // rather than duplicated here.
        let d = with_unit("typo");
        write(&d, ".engine/contracts/activation.toml", "[processes]
active = [\"isue-resolution\"]
");
        let a = Activation::load(&d);
        assert_eq!(a.errors.len(), 1, "an unknown process name must be reported");
        assert!(a.errors[0].contains("isue-resolution"));
    }
}

/// The declared authority policy for one attestation class (D0146/D0129).
pub struct ClassPolicy {
    pub kinds: Vec<String>,
    pub roles: Vec<String>,
}

/// Read `.engine/contracts/attestation-policy.toml`.
///
/// AN ABSENT FILE IS THE BUILT-IN FLOOR, never "no policy". A missing contract must not silently
/// disable an authority check — absence is a legitimate state meaning the DEFAULT (the issue090
/// lesson), and the default here is human-only, which is what D0106 and D0092 require regardless of
/// whether a project has written the file.
#[must_use]
pub fn attestation_policy(root: &Path, class: &str) -> ClassPolicy {
    let default = || ClassPolicy { kinds: vec!["human".to_owned()], roles: Vec::new() };
    let path = root.join(".engine").join("contracts").join("attestation-policy.toml");
    let Ok(text) = std::fs::read_to_string(path) else { return default() };
    let Ok(v) = text.parse::<toml::Value>() else { return default() };
    let Some(tbl) = v.get(class) else { return default() };
    let list = |k: &str| -> Vec<String> {
        tbl.get(k)
            .and_then(toml::Value::as_array)
            .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect())
            .unwrap_or_default()
    };
    let kinds = list("kinds");
    ClassPolicy {
        // A class with no permitted kind could never be satisfied, so an empty list falls back
        // rather than locking the class shut — a typo in the contract must not become a deadlock.
        kinds: if kinds.is_empty() { default().kinds } else { kinds },
        roles: list("roles"),
    }
}

/// The recording delegation declared for one attestation class (D0192 OPTION A).
///
/// Names the Decision that authorizes an agent to RECORD a human's judgment, provided the record
/// quotes their words verbatim and names them as judge. `None` = not declared — the substance check
/// then demands nothing, and recording requires the human's own gesture.
#[must_use]
pub fn recording_delegation(root: &Path, class: &str) -> Option<String> {
    let path = root.join(".engine").join("contracts").join("attestation-policy.toml");
    let text = std::fs::read_to_string(path).ok()?;
    let v = text.parse::<toml::Value>().ok()?;
    v.get(class)?.get("delegatedRecording")?.as_str().map(str::to_owned)
}

/// Does `actor` satisfy `policy`? Returns the reason it does NOT, or `None` if it does.
///
/// ROLES: an empty `roles` list means ANY role, INCLUDING an actor with none recorded. That is what
/// keeps this forward-only (issue068) — actors enrolled before D0146 carry no role and must not be
/// retro-failed. Once a class names roles, an actor with no recorded role no longer satisfies it,
/// and the message says so rather than reporting a generic mismatch.
#[must_use]
pub fn authority_gap(model_kind: Option<&str>, model_role: Option<&str>, policy: &ClassPolicy) -> Option<String> {
    let kind = model_kind.unwrap_or("");
    let kind_ok = policy.kinds.iter().any(|k| kind.ends_with(k.as_str()));
    if !kind_ok {
        return Some(format!(
            "kind is '{}' and this class permits {:?}",
            if kind.is_empty() { "unrecorded" } else { kind },
            policy.kinds
        ));
    }
    if policy.roles.is_empty() {
        return None;
    }
    match model_role {
        Some(r) if policy.roles.iter().any(|p| p == r) => None,
        Some(r) => Some(format!("role is '{r}' and this class permits {:?}", policy.roles)),
        None => Some(format!(
            "no role is recorded, and this class permits only {:?} — enroll with `keel enroll --role` (D0146)",
            policy.roles
        )),
    }
}

#[cfg(test)]
mod authority_policy_tests {
    use super::{authority_gap, ClassPolicy};

    fn p(kinds: &[&str], roles: &[&str]) -> ClassPolicy {
        ClassPolicy {
            kinds: kinds.iter().map(|s| (*s).to_owned()).collect(),
            roles: roles.iter().map(|s| (*s).to_owned()).collect(),
        }
    }

    #[test]
    fn an_empty_role_list_admits_an_actor_with_no_role() {
        // FORWARD-ONLY (issue068). Every actor enrolled before D0146 carries no role, and a class
        // that names no roles must admit them — otherwise adding the attribute would retro-fail
        // every attestation in the repo's history.
        assert_eq!(authority_gap(Some("human"), None, &p(&["human"], &[])), None);
        assert_eq!(authority_gap(Some("ActorKind::human"), Some("anything"), &p(&["human"], &[])), None);
    }

    #[test]
    fn a_named_role_list_refuses_a_mismatch_and_an_absence() {
        let pol = p(&["human"], &["supervisor"]);
        assert_eq!(authority_gap(Some("human"), Some("supervisor"), &pol), None);
        let mismatch = authority_gap(Some("human"), Some("contributor"), &pol).unwrap();
        assert!(mismatch.contains("contributor") && mismatch.contains("supervisor"), "{mismatch}");
        // An UNRECORDED role is a distinct message from a wrong one: the remedy differs (enroll vs
        // re-enroll), and a generic "mismatch" would send the reader looking for a role that is not
        // there to find.
        let absent = authority_gap(Some("human"), None, &pol).unwrap();
        assert!(absent.contains("no role is recorded") && absent.contains("keel enroll --role"), "{absent}");
    }

    #[test]
    fn kind_is_checked_before_role_and_an_ai_never_passes_a_human_class() {
        let pol = p(&["human"], &["supervisor"]);
        // Even with the RIGHT role, an AI fails a human-only class — role never launders kind.
        let gap = authority_gap(Some("ActorKind::ai"), Some("supervisor"), &pol).unwrap();
        assert!(gap.contains("kind is"), "kind must be reported first: {gap}");
        // An unrecorded kind is not a pass either.
        assert!(authority_gap(None, Some("supervisor"), &pol).is_some());
    }
}
