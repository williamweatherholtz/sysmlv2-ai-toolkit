//! `keel github-decider` — who may decide on this project's GitHub decision channel (D0219).
//!
//! WHY THIS EXISTS. The channel (D0205) shipped with the decider's LOGIN welded into
//! `.github/workflows/decision-record.yml` in three places, and with issue assignment pointed at
//! `github.repository_owner`. Both are wrong the moment a second project adopts it:
//!
//!   * `github.repository_owner` is not a person. For `asirobots/penumbra` it is an ORG, and
//!     `gh issue create --assignee asirobots` fails outright — the mechanism would break on the
//!     first adoption, not degrade.
//!   * A login in workflow SOURCE means every adopting project must edit the workflow, which is both
//!     a fork of the mechanism and an edit to its own frozen enforcement surface (D0209 clause 2).
//!
//! The authorization set is already committed data: `.engine/contracts/github-actors.toml` maps a
//! GitHub login to a keel actor, and `record_decision.sh` has always REFUSED an unmapped login
//! (provenance is never defaulted, issue182). So "who may decide" and "whose judgment is
//! attributable" are the SAME fact, and it has one home (D0105). This module is the single
//! implementation of that rule, so the workflow gate and the recorder cannot drift apart.

use std::collections::BTreeMap;
use std::path::Path;

/// GitHub login -> keel actor, from `.engine/contracts/github-actors.toml`.
///
/// Only HUMANS belong in the table: it exists to attribute human judgment, and mapping a bot login
/// would recreate the AI-recorded-as-human class (issue072/073) at the channel layer. An absent file
/// yields an EMPTY map, which authorises nobody — the safe direction, and distinguishable from a
/// present-but-empty table only in that both refuse.
#[must_use]
pub fn deciders(root: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(root.join(".engine/contracts/github-actors.toml")) else {
        return out;
    };
    let mut in_logins = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('#') || t.is_empty() {
            continue;
        }
        if t.starts_with('[') {
            in_logins = t == "[logins]";
            continue;
        }
        if !in_logins {
            continue;
        }
        if let Some((login, actor)) = t.split_once('=') {
            let login = login.trim();
            let actor = actor.trim().trim_matches('"');
            if !login.is_empty() && !actor.is_empty() {
                out.insert(login.to_string(), actor.to_string());
            }
        }
    }
    out
}

/// `keel github-decider [<login>]`.
///
/// With no argument: print each declared decider as `login<TAB>actor`, one per line, for a workflow
/// to consume (assignment, reporting). Exit 0 even when empty — a project may run the channel with no
/// declared decider and simply record nothing until it declares one.
///
/// With a login: exit 0 if that login may decide, 1 if not, printing the mapped actor on success so
/// the caller never has to re-derive it.
///
/// # Returns
/// Exit code as described. Never panics: an unreadable contract authorises nobody.
#[must_use]
pub fn decider_cmd(args: &[String], root: &Path) -> i32 {
    let map = deciders(root);
    let Some(login) = args.iter().find(|a| !a.starts_with("--")) else {
        for (l, a) in &map {
            println!("{l}\t{a}");
        }
        return 0;
    };
    if let Some(actor) = map.get(login.as_str()) {
        println!("{actor}");
        return 0;
    }
    let declared = if map.is_empty() {
        "(none)".to_string()
    } else {
        map.keys().cloned().collect::<Vec<_>>().join(", ")
    };
    eprintln!(
        "github-decider: `{login}` is not a declared decider on this project. Add it to \
         .engine/contracts/github-actors.toml [logins] as `{login} = \"<keelActor>\"` — an \
         unmapped login is REFUSED, never defaulted (issue182). Declared: {declared}"
    );
    1
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn this_project_declares_its_decider_and_refuses_others() {
        let root = Path::new("..");
        let map = super::deciders(root);
        assert!(!map.is_empty(), "this project runs the channel, so it must declare at least one decider");
        assert!(!map.contains_key("asirobots"), "an ORG is not a person and must never be a decider");
    }

    #[test]
    fn an_absent_contract_authorises_nobody() {
        // The safe direction: no table means no decider, never "anyone" and never "the repo owner".
        let tmp = std::env::temp_dir().join(format!("keel-gh-decider-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("tmp");
        assert!(super::deciders(&tmp).is_empty());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn only_the_logins_section_is_read() {
        let tmp = std::env::temp_dir().join(format!("keel-gh-sect-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".engine/contracts")).expect("tmp");
        std::fs::write(
            tmp.join(".engine/contracts/github-actors.toml"),
            "# comment\n[other]\nnotAdecider = \"x\"\n[logins]\nalice = \"aliceActor\"\n",
        )
        .expect("write");
        let m = super::deciders(&tmp);
        assert_eq!(m.get("alice").map(String::as_str), Some("aliceActor"));
        assert!(!m.contains_key("notAdecider"), "a login outside [logins] must not authorise");
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
