//! The knowledge store's two computed views (D0161): `keel why <term>` and
//! `keel knowledge question-coverage`. For keel's own corpus THE MODEL IS THE GRAPH — nothing here
//! stores entities, relations or answer text; the only authored inputs are the human-owned Questions
//! and Aliases under `.knowledge/`, and deleting them makes both views report NOTHING DECLARED with
//! the gate staying green (data-level removability, D0161 part 3i).

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use crate::json::Json;

#[allow(clippy::wildcard_imports)] // a view submodule: the parent's vocabulary is this file's vocabulary
use super::*;

/// How many edge hops a traversal follows. Three is the deliberate multi-hop trap's floor (D0153
/// step 7): a two-hop cap would make every trap question fail structurally rather than semantically.
const MAX_HOPS: usize = 3;

/// A node with more edges than this is a HUB (a Story every gate verifies, a Decision everything
/// charters to): it is still REACHED and reported, but never EXPANDED - expanding hubs made a
/// 3-hop walk reach effectively the whole model on the first live run, which turns the answer into
/// noise and coverage into a vacuous yes. The walk reports how many hubs it declined to expand, so
/// the bound is visible rather than a silent cap.
const HUB_DEGREE_LIMIT: usize = 25;

/// How many reached rows `keel why` prints. The full count is always reported beside it.
const MAX_REPORTED_ROWS: usize = 40;

/// Case-insensitive containment on names, titles and Alias terms — the seeding rule (D0161 part 2).
fn matches_term(hay: &str, needle: &str) -> bool {
    hay.to_lowercase().contains(&needle.to_lowercase())
}

/// Seed the traversal: items whose NAME or TITLE contains `term`, plus every target of an Alias
/// whose `term` matches. Returns `(seed name, how it was found)` pairs, sorted for determinism.
fn seeds_for(model: &Model, term: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for (name, info) in &model.items {
        if matches_term(name, term) {
            out.push((name.clone(), "name match".to_string()));
        } else if info.attrs.get("title").is_some_and(|t| matches_term(t, term)) {
            out.push((name.clone(), "title match".to_string()));
        }
        if info.type_name == "Alias" && info.attrs.get("term").is_some_and(|t| matches_term(t, term)) {
            for e in model.edges.iter().filter(|e| e.kind == "dependency" && e.from == *name) {
                if model.items.contains_key(&e.to) {
                    out.push((e.to.clone(), format!("alias '{name}'")));
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Breadth-first traversal over ALL typed edges (both directions) from `seeds`, up to [`MAX_HOPS`],
/// declining to EXPAND hub nodes past [`HUB_DEGREE_LIMIT`]. Returns
/// `(reached item -> (hops, via), hubs declined)`.
fn traverse(model: &Model, seeds: &[String]) -> (HashMap<String, (usize, String)>, usize) {
    let mut degree: HashMap<&str, usize> = HashMap::new();
    for e in &model.edges {
        *degree.entry(e.from.as_str()).or_default() += 1;
        *degree.entry(e.to.as_str()).or_default() += 1;
    }
    let mut hubs_declined = 0usize;
    let mut reached: HashMap<String, (usize, String)> = HashMap::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    for s in seeds {
        reached.entry(s.clone()).or_insert_with(|| (0, "seed".to_string()));
        queue.push_back((s.clone(), 0));
    }
    while let Some((cur, hops)) = queue.pop_front() {
        if hops >= MAX_HOPS {
            continue;
        }
        if hops > 0 && degree.get(cur.as_str()).copied().unwrap_or(0) > HUB_DEGREE_LIMIT {
            hubs_declined += 1;
            continue; // reached and reported, never expanded - the bound that keeps answers answers
        }
        for e in &model.edges {
            let (next, dir) = if e.from == cur {
                (&e.to, "->")
            } else if e.to == cur {
                (&e.from, "<-")
            } else {
                continue;
            };
            if !model.items.contains_key(next) || reached.contains_key(next) {
                continue;
            }
            reached.insert(next.clone(), (hops + 1, format!("{cur} {dir}{} {next}", e.kind)));
            queue.push_back((next.clone(), hops + 1));
        }
    }
    (reached, hubs_declined)
}

/// `keel why <term>` — answer from the model as a graph (D0161).
///
/// Seed on names, titles and aliases; traverse; return the reasoning WITH provenance and with any
/// FAILING critique against what it reached — the view never hides that a reached element
/// currently fails an examination.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be read.
pub fn why(root: &Path, term: &str) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let declared = model.items.values().filter(|i| i.type_name == "Alias" || i.type_name == "Question").count();
    let seeds = seeds_for(&model, term);
    let seed_names: Vec<String> = seeds.iter().map(|(n, _)| n.clone()).collect();
    let (reached, hubs_declined) = traverse(&model, &seed_names);
    let total_reached = reached.len();
    let mut rows: Vec<Json> = Vec::new();
    let mut sorted: Vec<(&String, &(usize, String))> = reached.iter().collect();
    sorted.sort_by(|a, b| (a.1 .0, a.0).cmp(&(b.1 .0, b.0)));
    sorted.truncate(MAX_REPORTED_ROWS);
    for (name, (hops, via)) in sorted {
        let Some(info) = model.items.get(name) else { continue };
        let failing: Vec<Json> = model
            .items
            .iter()
            .filter(|(vn, vi)| {
                vi.attrs.get("method").map(String::as_str) == Some("critique")
                    && model.edges.iter().any(|e| e.kind == "verify" && &e.from == *vn && e.to == *name)
                    && latest_result(&model, vn).is_some_and(|(o, _)| o == "fail")
            })
            .map(|(vn, _)| Json::Str(vn.clone()))
            .collect();
        rows.push(Json::Obj(vec![
            ("element".to_string(), Json::Str(name.clone())),
            ("type".to_string(), Json::Str(info.type_name.clone())),
            ("title".to_string(), Json::Str(info.attrs.get("title").cloned().unwrap_or_default())),
            ("hops".to_string(), Json::Int(i64::try_from(*hops).unwrap_or(i64::MAX))),
            ("via".to_string(), Json::Str(via.clone())),
            ("file".to_string(), Json::Str(info.file.clone())),
            ("createdBy".to_string(), Json::Str(info.attrs.get("createdBy").cloned().unwrap_or_default())),
            ("failingCritiques".to_string(), Json::Arr(failing)),
        ]));
    }
    let out = Json::Obj(vec![
        ("view".to_string(), Json::Str("why".to_string())),
        ("term".to_string(), Json::Str(term.to_string())),
        (
            "declared".to_string(),
            if declared == 0 {
                Json::Str("NOTHING DECLARED - no Questions or Aliases under .knowledge/ (D0161: the store's input is authored facts; absent means the feature is unplugged)".to_string())
            } else {
                Json::Int(i64::try_from(declared).unwrap_or(i64::MAX))
            },
        ),
        (
            "seeds".to_string(),
            Json::Arr(seeds.into_iter().map(|(n, how)| Json::Obj(vec![
                ("element".to_string(), Json::Str(n)),
                ("foundBy".to_string(), Json::Str(how)),
            ])).collect()),
        ),
        ("totalReached".to_string(), Json::Int(i64::try_from(total_reached).unwrap_or(i64::MAX))),
        ("reportedRows".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))),
        ("hubsNotExpanded".to_string(), Json::Int(i64::try_from(hubs_declined).unwrap_or(i64::MAX))),
        ("reached".to_string(), Json::Arr(rows)),
    ]);
    Ok(out.dump())
}

/// One Question's computed coverage: did seeding find anything, and did traversal reach beyond it?
fn question_row(model: &Model, qname: &str, qtext: &str) -> Json {
    // Seed on each word of the question (>= 4 chars, so articles and short verbs don't seed noise).
    let mut seed_set: HashSet<String> = HashSet::new();
    let mut seed_rows: Vec<Json> = Vec::new();
    for word in qtext.split(|c: char| !c.is_alphanumeric()).filter(|w| w.chars().count() >= 4) {
        for (n, how) in seeds_for(model, word) {
            if n != qname && seed_set.insert(n.clone()) {
                seed_rows.push(Json::Obj(vec![
                    ("element".to_string(), Json::Str(n)),
                    ("word".to_string(), Json::Str(word.to_string())),
                    ("foundBy".to_string(), Json::Str(how)),
                ]));
            }
        }
    }
    let seeds: Vec<String> = seed_set.into_iter().collect();
    let (reached, _) = traverse(model, &seeds);
    let beyond = reached.values().filter(|(h, _)| *h > 0).count();
    Json::Obj(vec![
        ("question".to_string(), Json::Str(qname.to_string())),
        ("text".to_string(), Json::Str(qtext.to_string())),
        ("seedFound".to_string(), Json::Bool(!seeds.is_empty())),
        ("seeds".to_string(), Json::Arr(seed_rows)),
        ("reachedBeyondSeed".to_string(), Json::Int(i64::try_from(beyond).unwrap_or(i64::MAX))),
        ("covered".to_string(), Json::Bool(!seeds.is_empty() && beyond > 0)),
    ])
}

/// `keel knowledge question-coverage` — for each declared Question, whether seeding finds an entity
/// and traversal reaches an answer (D0161 part 2). Never authored, recomputed per run.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be read.
pub fn question_coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut questions: Vec<(&String, &ItemInfo)> =
        model.items.iter().filter(|(_, i)| i.type_name == "Question").collect();
    questions.sort_by_key(|(n, _)| (*n).clone());
    let rows: Vec<Json> = questions
        .iter()
        .map(|(n, i)| question_row(&model, n, i.attrs.get("questionText").map_or("", String::as_str)))
        .collect();
    let covered = rows
        .iter()
        .filter(|r| matches!(r, Json::Obj(kv) if kv.iter().any(|(k, v)| k == "covered" && matches!(v, Json::Bool(true)))))
        .count();
    let out = Json::Obj(vec![
        ("view".to_string(), Json::Str("question-coverage".to_string())),
        (
            "declared".to_string(),
            if rows.is_empty() {
                Json::Str("NOTHING DECLARED - no Questions under .knowledge/ (D0161)".to_string())
            } else {
                Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))
            },
        ),
        ("covered".to_string(), Json::Int(i64::try_from(covered).unwrap_or(i64::MAX))),
        ("questions".to_string(), Json::Arr(rows)),
    ]);
    Ok(out.dump())
}

/// The `question-coverage` guard's core (D0161 part 3ii).
///
/// WELL-FORMEDNESS of declared knowledge facts, never coverage itself — gating on coverage would
/// make the cheapest fix deleting the question (the D0098 honest-state rule). A Question must
/// carry its text; an Alias must carry its term and at least one dependency edge to an element
/// that exists. Zero declared = zero scanned, green: absence is the feature unplugged.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be read.
pub fn knowledge_wellformedness(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for (name, info) in &model.items {
        match info.type_name.as_str() {
            "Question" => {
                scanned += 1;
                if info.attrs.get("questionText").is_none_or(|t| t.trim().is_empty()) {
                    violations.push(format!("{name}: Question with no questionText - a question that asks nothing cannot be covered (D0161)"));
                }
            }
            "Alias" => {
                scanned += 1;
                if info.attrs.get("term").is_none_or(|t| t.trim().is_empty()) {
                    violations.push(format!("{name}: Alias with no term - a lexicon entry that maps no word seeds nothing (D0161)"));
                }
                let has_target = model
                    .edges
                    .iter()
                    .any(|e| e.kind == "dependency" && e.from == *name && model.items.contains_key(&e.to));
                if !has_target {
                    violations.push(format!("{name}: Alias with no dependency edge to an existing element - the word maps to nothing (D0161)"));
                }
            }
            _ => {}
        }
    }
    violations.sort();
    Ok((scanned, violations))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// D0161's well-formedness boundary and its data-level removability, against a real temp model:
    /// a malformed Alias is a violation; an EMPTY store scans zero and violates nothing (the feature
    /// unplugged is a state, never a defect).
    #[test]
    fn knowledge_wellformedness_flags_malformed_and_passes_absent() {
        let dir = std::env::temp_dir().join("keel-knowledge-wf-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".knowledge")).expect("mkdir");
        std::fs::write(
            dir.join(".knowledge").join("bad.sysml"),
            "package P {\n    part aGhost : Alias { :>> id = \"aaaaaaaa-4444-4444-9444-aaaaaaaaaaaa\"; :>> term = \"ghost\"; }\n    part qEmpty : Question { :>> id = \"aaaaaaaa-5555-4555-9555-aaaaaaaaaaaa\"; :>> questionText = \"\"; }\n}\n",
        )
        .expect("write");
        let (scanned, violations) = knowledge_wellformedness(&dir).expect("model builds");
        assert_eq!(scanned, 2);
        assert_eq!(violations.len(), 2, "ghost alias (no edge) + empty question: {violations:?}");
        // Data-level removability: delete the store, nothing declared, nothing violated.
        // (new_epoch: the model memo is keyed by an explicit epoch - a test mutating files
        // directly must announce the write the way the CLI write paths do.)
        std::fs::remove_dir_all(dir.join(".knowledge")).expect("rm");
        crate::fingerprint::new_epoch();
        let (scanned, violations) = knowledge_wellformedness(&dir).expect("model builds empty");
        assert_eq!((scanned, violations.len()), (0, 0));
    }
}
