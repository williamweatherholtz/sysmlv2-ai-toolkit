//! dcControlCensusByActorKind, the on-tree half of the probe pair: `keel show control-census` on this
//! repository classes the delegated-words refusal `friction` and names issue397 as the incident. The
//! fixture halves (code-read only -> hypothetical, observed -> evidenced) are unit tests beside the
//! view, because they need no tree.

use std::path::Path;
use std::process::Command;

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root")
}

#[test]
fn the_delegated_words_refusal_classes_friction_naming_issue397() {
    let out = Command::new(env!("CARGO_BIN_EXE_keel")).args(["show", "control-census", "."]).current_dir(repo()).output().expect("run keel");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let d: serde_json::Value = serde_json::from_slice(&out.stdout).expect("json");
    let rows = d["controls"].as_array().expect("controls");
    let row = rows.iter().find(|r| r["control"] == "accept:delegated-words").expect("the delegated-words check is a row");
    assert_eq!(row["evidenceClass"], "friction", "{row}");
    assert_eq!(row["binds"], "human", "{row}");
    assert!(row["basis"].as_str().is_some_and(|b| b.contains("issue397")), "{row}");
    let incident = row["incidents"].as_array().expect("incidents").iter().find(|i| i["issue"] == "issue397").expect("issue397 is an incident");
    assert_eq!(incident["class"], "observed", "{incident}");
    assert_eq!(incident["act"], "refusal-of-a-correct-act", "{incident}");
    // friction and hypothetical lead the list: the first evidenced row comes after the last candidate
    let first_evidenced = rows.iter().position(|r| r["evidenceClass"] == "evidenced").unwrap_or(rows.len());
    let last_candidate = rows.iter().rposition(|r| r["evidenceClass"] != "evidenced").expect("at least one candidate");
    assert!(last_candidate < first_evidenced, "candidates first: last candidate at {last_candidate}, first evidenced at {first_evidenced}");
    let candidates = d["removalCandidates"].as_array().expect("removalCandidates");
    assert!(candidates.iter().any(|c| c.as_str().is_some_and(|s| s.starts_with("accept:delegated-words (friction"))), "{candidates:?}");
}
