//! The knowledge store's two computed views (D0161): `keel why <term>` and
//! `keel knowledge question-coverage`. For keel's own corpus THE MODEL IS THE GRAPH — nothing here
//! stores entities, relations or answer text; the only authored inputs are the human-owned Questions
//! and Aliases under `.knowledge/`, and deleting them makes both views report NOTHING DECLARED with
//! the gate staying green (data-level removability, D0161 part 3i).

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write as _;
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

/// How many WORD tokens from one prompt may seed. Identifiers are exempt and unlimited: a prompt
/// naming three decisions means all three. Words are capped because each extra seed widens the walk,
/// and the rarest few carry nearly all the information in a sentence.
const MAX_PROMPT_WORDS: usize = 3;

/// Characters held back for the always-appended `recall:` summary and any truncation note, so the
/// declared budget is the WHOLE payload rather than the rows alone.
const FOOTER_RESERVE: usize = 160;

/// Split a name or title into lowercase WORD SEGMENTS: `dcWorkspaceDiscovery` -> dc, workspace,
/// discovery; `keel-gate.yml` -> keel, gate, yml. camelCase boundaries count, as does any
/// non-alphanumeric.
///
/// Substring matching is what made seeding explode (D0243): `gate` matched 5,806 of 6,625 items
/// because it sits inside gated, gating, keel-gate and staleGateProse. A segment is a word the author
/// actually wrote.
fn segments(hay: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut prev_lower = false;
    for c in hay.chars() {
        if c.is_alphanumeric() {
            // A lower->upper transition is a camelCase boundary.
            if prev_lower && c.is_uppercase() && !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.push(c.to_ascii_lowercase());
            prev_lower = c.is_lowercase() || c.is_numeric();
        } else {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            prev_lower = false;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Does `hay` contain `needle` as a whole SEGMENT (or equal it outright)?
fn matches_term(hay: &str, needle: &str) -> bool {
    let n = needle.trim().to_lowercase();
    if n.is_empty() {
        return false;
    }
    if hay.to_lowercase() == n {
        return true;
    }
    // A multi-word needle ("write api") is matched as a phrase against the joined segments.
    if n.contains(' ') {
        let joined = segments(hay).join(" ");
        let phrase = segments(&n).join(" ");
        return joined.contains(&phrase);
    }
    segments(hay).contains(&n)
}

/// Is this token an IDENTIFIER the corpus mints — `d0242`, `issue293`, `sprint477`, or a camelCase
/// element name? Identifier tokens are the strongest possible seed (D0243 rule 1): a prompt that
/// contains one is asking about exactly that thing, so they bypass rarity weighting entirely.
fn is_identifier_token(tok: &str) -> bool {
    let t = tok.trim();
    if t.len() < 3 {
        return false;
    }
    let numeric_tail = |prefix: &str| {
        t.strip_prefix(prefix)
            .is_some_and(|r| !r.is_empty() && r.chars().all(|c| c.is_ascii_digit()))
    };
    if numeric_tail("d") || numeric_tail("issue") || numeric_tail("sprint") || numeric_tail("D") {
        return true;
    }
    // camelCase / mixed case with an internal uppercase — how every element in this corpus is named.
    t.chars().all(char::is_alphanumeric)
        && t.chars().next().is_some_and(char::is_lowercase)
        && t.chars().skip(1).any(char::is_uppercase)
}

/// English function and filler words. Language, NOT domain vocabulary — the distinction matters:
/// domain stop-wording is DERIVED from the corpus (D0243 rule 3) because it differs per project and
/// goes stale, while "would" is a filler word in every corpus and always will be.
fn is_function_word(w: &str) -> bool {
    const WORDS: [&str; 55] = [
        "about", "above", "after", "again", "against", "already", "also", "although", "always",
        "another", "because", "been", "before", "being", "below", "between", "both", "cannot",
        "could", "does", "doing", "done", "during", "each", "either", "every", "from", "further",
        "have", "having", "here", "into", "just", "made", "make", "making", "many", "more", "most",
        "much", "must", "only", "other", "should", "some", "would",
        // The filler class that forced, and then failed to justify, a structural name-match rule:
        // rare in the corpus and empty about the subject. MEASURED: requiring a NAME match to catch
        // these also rejected "rebase" and "frozen", which are domain terms that legitimately live in
        // TITLES rather than in element names, and that cost 3 of 5 correct answers on an eight-case
        // set. Naming them as LANGUAGE is both cheaper and more honest than a rule that cannot tell a
        // rare filler word from a rare domain term.
        "anything", "nothing", "something", "stuff", "thing", "things", "today", "tomorrow",
        "yesterday",
    ];
    WORDS.contains(&w)
}

/// Tokens worth seeding on, drawn from a free-form PROMPT (D0242 part 2 / D0243).
///
/// Returns `(token, match count, kind)` for tokens that survive, and the dropped-as-common list so the
/// caller can SAY what it ignored rather than silently narrowing. Order: identifiers first, then rare
/// words by increasing frequency — the rarest token is the most informative seed.
fn prompt_tokens(model: &Model, prompt: &str) -> (Vec<(String, usize, &'static str)>, Vec<String>) {
    // A token matching more than this share of the corpus carries no information. DERIVED from the
    // tree, not authored, so it re-weights itself as the corpus grows (D0243 rule 3).
    // Tuned from the corpus, and reported beside the answer so the rule is auditable rather than a
    // magic number: at ~12k items this lands near 60, which drops `gate` (5,736) and `decision` (236)
    // while keeping `workspace` (65) and `keystone` (38). A payload can usefully show a dozen or two
    // elements, so seeding on a token that matches hundreds cannot inform the ranking.
    let ceiling = std::cmp::max(20, model.items.len() / 200);
    let mut seen: HashSet<String> = HashSet::new();
    let mut kept: Vec<(String, usize, &'static str)> = Vec::new();
    let mut dropped: Vec<String> = Vec::new();
    for raw in prompt.split(|c: char| !c.is_alphanumeric() && c != '-') {
        let tok = raw.trim_matches('-');
        if tok.chars().count() < 3 || !seen.insert(tok.to_lowercase()) {
            continue;
        }
        if is_identifier_token(tok) {
            kept.push((tok.to_string(), 0, "identifier"));
            continue;
        }
        if tok.chars().count() < 5 || is_function_word(&tok.to_lowercase()) {
            // Measured: `made` matched 7 elements — rare, and semantically empty, so it seeded an
            // unrelated Issue. Frequency alone cannot separate a rare DOMAIN term from a rare filler
            // word, so language is filtered as language. This list is ENGLISH, not this corpus's
            // vocabulary, so it stays true for any project and needs no maintenance (D0243 keeps
            // domain stop-wording derived; this is a different thing).
            continue;
        }
        let hits = model
            .items
            .iter()
            .filter(|(n, i)| {
                matches_term(n, tok) || i.attrs.get("title").is_some_and(|t| matches_term(t, tok))
            })
            .count();
        if hits == 0 {
            continue;
        }
        if hits > ceiling {
            dropped.push(format!("{tok} ({hits} matches)"));
        } else {
            kept.push((tok.to_string(), hits, "word"));
        }
    }
    kept.sort_by_key(|(tok, hits, _)| (*hits, tok.clone()));
    // Identifiers are never dropped; among WORDS keep only the rarest few, because the fifth-rarest
    // word in a sentence is close to noise and every extra seed widens the walk.
    let mut ids: Vec<(String, usize, &'static str)> =
        kept.iter().filter(|(_, _, k)| *k == "identifier").cloned().collect();
    let words: Vec<(String, usize, &'static str)> =
        kept.into_iter().filter(|(_, _, k)| *k != "identifier").take(MAX_PROMPT_WORDS).collect();
    ids.extend(words);
    dropped.sort();
    (ids, dropped)
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

/// One compact, budgeted, CONTENT-bearing recall payload — the form fit to place in front of a model
/// (D0242 part 1).
///
/// `keel why` returns JSON whose `seeds` are bare identifiers; injecting that would push 40 names and
/// no facts. This emits ranked lines carrying what a reader needs to act — type, name, title, how it
/// was reached, and the source file — plus the substantive text of the nearest few elements, under a
/// character budget so a turn's context cost is bounded and predictable.
///
/// THE BUDGET HAS EXACTLY ONE STATED EXCEPTION: the top row is always included, even when it alone
/// exceeds the budget. Two rules here would otherwise contradict each other - "the payload fits the
/// budget" and "a budget narrows the answer, never erases it" - and a budget small enough to erase the
/// single most relevant fact is a budget set wrong, not an answer worth suppressing. Every row after
/// the first is inside the budget, footer included.
fn brief_from_seeds(
    model: &Model,
    header: &str,
    seeds: &[(String, String)],
    agreement: &HashMap<String, usize>,
    budget: usize,
) -> String {
    let seed_names: Vec<String> = seeds.iter().map(|(n, _)| n.clone()).collect();
    // AGREEMENT IS THE PRIMARY RANK. Measured on a real prompt ("should we implement the knowledge
    // graph item now? why didn't we earlier?"): 81 seeds all sat at 0 hops, so the tie broke
    // ALPHABETICALLY and the payload led with a decision about the VIEWER's graph renderer plus two
    // stale critiques matching "earlier", while d0161 - the actual answer - sat unshown among 366.
    //
    // The signal that fixes it needs no vocabulary: how many DISTINCT prompt tokens reach the same
    // element. For that prompt, 46 elements matched `knowledge` alone and 23 matched `graph` alone,
    // while exactly 9 matched BOTH - d0153, d0161, d0242, dcKnowledgeGraphStore and the process
    // itself, which is the correct answer set. Two independent words agreeing on one element is
    // evidence; one word matching it is a coincidence waiting to happen.
    //
    // An element the prompt NAMED outright (an identifier) is given the maximum score: `d0239` is not
    // a guess about relevance.
    let named: HashSet<&str> = seeds
        .iter()
        .filter(|(_, how)| how.starts_with("identifier"))
        .map(|(n, _)| n.as_str())
        .collect();
    let (reached, hubs) = traverse(model, &seed_names);
    let mut sorted: Vec<(&String, &(usize, String))> = reached.iter().collect();
    sorted.sort_by(|a, b| {
        let key = |n: &String, h: usize| {
            let score = if named.contains(n.as_str()) { usize::MAX } else { *agreement.get(n).unwrap_or(&0) };
            (std::cmp::Reverse(score), h, n.clone())
        };
        key(a.0, a.1 .0).cmp(&key(b.0, b.1 .0))
    });
    let mut out = String::with_capacity(budget.min(8192));
    out.push_str(header);
    out.push('\n');
    let mut written = 0usize;
    // The nearest elements get their substantive text; the rest get one line each. Nearest-first is
    // the only defensible ranking here: hops is the graph's own measure of relevance to the seed.
    //
    // Substance is capped at 240 chars over the top TWO rows, not 400 over three: measured, three
    // 400-char rows consumed a 1,200-char budget entirely and showed 2 of 408 reached elements, so
    // the payload was all depth and no breadth.
    for (rank, (name, (hops, via))) in sorted.iter().enumerate() {
        let Some(info) = model.items.get(*name) else { continue };
        let title = info.attrs.get("title").cloned().unwrap_or_default();
        let mut line = format!(
            "- {} {} ({} hop{}, via {}) — {}\n  {}\n",
            info.type_name,
            name,
            hops,
            if *hops == 1 { "" } else { "s" },
            via,
            title,
            info.file
        );
        if rank < 2 {
            for field in ["decision", "description", "rationale", "actionText"] {
                if let Some(v) = info.attrs.get(field) {
                    let text: String = v.chars().take(240).collect();
                    let _ = writeln!(line, "  {field}: {text}");
                    break;
                }
            }
        }
        // THE TOP ROW IS ALWAYS SHOWN. Measured: with a tight budget the first row's substance
        // overflowed before anything was written, so the payload printed "41 more reached, not shown"
        // AND "(nothing reached)" — two contradictory statements and zero facts. A budget should
        // narrow the answer, never erase it.
        // RESERVE the footer. The summary line is always appended, so counting only the rows made a
        // 1,500-char budget produce 1,539 - a small overshoot, but the DoD says the payload FITS the
        // budget and a claim is either true or it is not.
        if out.len() + line.len() + FOOTER_RESERVE > budget && written > 0 {
            let _ = writeln!(
                out,
                "  ... {} more reached, not shown (budget {budget} chars)",
                sorted.len().saturating_sub(written)
            );
            break;
        }
        out.push_str(&line);
        written += 1;
    }
    if written == 0 {
        out.push_str("  (nothing reached)\n");
    }
    let _ = writeln!(
        out,
        "recall: {} seed(s), {} reached, {} shown, {} hub(s) not expanded",
        seeds.len(),
        reached.len(),
        written,
        hubs
    );
    out
}

/// `keel why <term> --brief` (D0242 part 1).
///
/// # Errors
/// Propagates model-build failures.
pub fn why_brief(root: &Path, term: &str, budget: usize) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let seeds = seeds_for(&model, term);
    // A single explicit term cannot produce agreement, so every seed scores 1 and hops decides - which
    // is correct here: the caller named the subject, so there is nothing to disambiguate.
    let agreement: HashMap<String, usize> = seeds.iter().map(|(n, _)| (n.clone(), 1)).collect();
    Ok(brief_from_seeds(&model, &format!("recalled for term `{term}`:"), &seeds, &agreement, budget))
}

/// `keel recall --prompt -` : seed from a free-form PROMPT and return a budgeted brief (D0242 part 2,
/// D0243 precision rules). ZERO model calls — this is code finding facts before the model is involved.
///
/// # Errors
/// Propagates model-build failures.
pub fn recall_for_prompt(root: &Path, prompt: &str, budget: usize) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let (tokens, dropped) = prompt_tokens(&model, prompt);
    if tokens.is_empty() {
        // SAYING SO is the point: a silent empty recall is indistinguishable from a broken one.
        let mut msg = String::from("recall: no informative term in this prompt — nothing pushed.\n");
        if !dropped.is_empty() {
            let _ = writeln!(msg, "  ignored as too common to inform: {}", dropped.join(", "));
        }
        return Ok(msg);
    }
    let mut seeds: Vec<(String, String)> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    // Count how many DISTINCT tokens reach each element - the agreement score.
    let mut agreement: HashMap<String, usize> = HashMap::new();
    for (tok, _, kind) in &tokens {
        for (n, how) in seeds_for(&model, tok) {
            *agreement.entry(n.clone()).or_insert(0) += 1;
            if seen.insert(n.clone()) {
                seeds.push((n, format!("{kind} `{tok}` / {how}")));
            }
        }
    }
    let best_agreement = agreement.values().copied().max().unwrap_or(0);
    let has_identifier = tokens.iter().any(|(_, _, k)| *k == "identifier");
    let has_name_match = seeds.iter().any(|(_, how)| how.contains("name match"));
    let named: Vec<String> = tokens.iter().map(|(t, h, k)| {
        if *k == "identifier" { t.clone() } else { format!("{t}({h})") }
    }).collect();
    let mut header = format!(
        "recalled on {} [corpus {} items, agreement {}, confidence {}]:",
        named.join(", "),
        model.items.len(),
        best_agreement,
        if confident(has_identifier, best_agreement, seeds.len(), has_name_match) { "HIGH" } else { "LOW" }
    );
    if !dropped.is_empty() {
        let _ = write!(header, " [ignored as too common: {}]", dropped.join(", "));
    }
    Ok(brief_from_seeds(&model, &header, &seeds, &agreement, budget))
}

/// Is this recall confident enough to PUSH unasked?
///
/// Measured, and this is the whole reason the verdict exists: recall quality is bimodal. A prompt whose
/// rare tokens agree on an element ("knowledge" + "graph" -> 9 of 78 candidates, all correct), or which
/// NAMES one outright (`d0239`), or whose tokens are rare enough to yield only a handful of seeds at
/// all, produces a precise payload. A prompt with one mid-frequency word and no agreement produces 81
/// seeds ranked arbitrarily, which costs the reader tokens and points at the wrong files - strictly
/// worse than silence.
///
/// THE THIRD CRITERION WAS A CORRECTION, not a design: with only the first two, the best payload of the
/// three sampled prompts - "do we need to rely on diligence... were there pre-model hooks", which
/// returned D0134 and the sprint that moved the hooks into the binary - scored LOW and would have been
/// suppressed. Its quality came from having 9 seeds, not from agreement. A rule that rejects its own
/// best example is wrong about what it is measuring.
///
/// Pulling is different: `keel why` and `keel recall` always print, because the caller asked. This
/// verdict governs only unrequested INJECTION.
/// A seed set this small is inherently focused: the payload can show most of what was found, so
/// ranking barely matters and there is nothing for an arbitrary tie-break to get wrong.
const FOCUSED_SEED_COUNT: usize = 12;

#[must_use]
pub const fn confident(
    has_identifier: bool,
    best_agreement: usize,
    seed_count: usize,
    has_name_match: bool,
) -> bool {
    // The focused-count criterion additionally requires a NAME match. Measured false positive: "what
    // about the thing we discussed yesterday" scored HIGH because `thing` matched 2 elements - inside
    // one Issue's TITLE PROSE ("the one thing a collision corrupts") - and 2 is a focused seed set.
    // A word appearing in prose is weak evidence about the subject; a word appearing in an element's
    // NAME is what an author chose to call it. Expanding the filler-word list instead would have been
    // whack-a-mole, and it would not generalise to the next corpus.
    // MEASURED, and this reversed the design. On eight fresh questions with ground truth fixed in
    // advance, the answering element appeared in the shown payload:
    //
    //   identifier | agreement>=2 | (<=12 seeds AND a name match)   ->  2/8   (4 prompts silenced)
    //   no bar at all: inject whenever anything informative is found ->  5/8   (0 silenced)
    //   identifier | agreement>=2 | <=12 seeds, no name-match req    ->  2/8   (3 silenced)
    //
    // The bar SUPPRESSED more right answers than wrong ones, and two attempts to tune it both landed
    // at 2/8. The reason is visible in the data: the good-but-silenced prompts had 13 to 53 seeds
    // (`frozen` 13, `github` 38, `obligation` 53), so any seed-count threshold low enough to exclude
    // noise also excludes them. Seed count is not a proxy for relevance, and I had assumed it was.
    //
    // So the rule is: if any informative token survived, push. The genuinely uninformative prompt is
    // already handled upstream - `prompt_tokens` returns nothing when every word is a function word or
    // too common, and `recall_for_prompt` then says so and pushes nothing. That check is about whether
    // there is anything to say; this one was about whether to trust it, and trusting it measures better.
    //
    // 4 of the 5 hits landed at position 1 or 2, so when recall is right it is right at the top. The 3
    // misses push roughly 20 non-answering rows, which is the cost being accepted - and the reason the
    // human has three kill switches.
    let _ = (has_identifier, best_agreement, has_name_match, FOCUSED_SEED_COUNT);
    seed_count > 0
}

/// The confidence verdict for a prompt, without building the payload — for the injection path.
///
/// # Errors
/// Propagates model-build failures.
pub fn recall_confidence(root: &Path, prompt: &str) -> Result<bool, ViewError> {
    let model = Model::build(root)?;
    let (tokens, _) = prompt_tokens(&model, prompt);
    if tokens.is_empty() {
        return Ok(false);
    }
    let mut agreement: HashMap<String, usize> = HashMap::new();
    let mut has_name_match = false;
    for (tok, _, _) in &tokens {
        for (n, how) in seeds_for(&model, tok) {
            has_name_match |= how.contains("name match");
            *agreement.entry(n).or_insert(0) += 1;
        }
    }
    Ok(confident(
        tokens.iter().any(|(_, _, k)| *k == "identifier"),
        agreement.values().copied().max().unwrap_or(0),
        agreement.len(),
        has_name_match,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The confidence verdict AS MEASURED, not as designed.
    ///
    /// This test was rewritten after the eight-case comparison, and the history matters more than the
    /// assertions: the original rule (identifier, or agreement >= 2, or a small seed set with a name
    /// match) scored 2/8, and removing it entirely scored 5/8. Two tuning attempts both landed at 2/8.
    /// A seed-count threshold cannot separate signal from noise here, because the prompts it wrongly
    /// silenced had 13 to 53 seeds - `frozen`, `github`, `obligation` - which is the same range as the
    /// prompts it rightly admitted.
    ///
    /// What survived is the check that asks whether there is anything to say at all, which lives
    /// upstream in `prompt_tokens`: a prompt of only function words or only corpus-common words yields
    /// no tokens, and nothing is pushed. That is a different question from "should I trust what I
    /// found", and it is the one worth asking.
    #[test]
    fn confidence_pushes_whenever_an_informative_token_survived() {
        // Anything found is pushed - the measured rule.
        assert!(confident(false, 1, 1, false), "one informative token is enough");
        assert!(confident(true, 1, 400, false), "a named element, however diffuse the rest");
        assert!(confident(false, 2, 81, true), "agreement, which still ranks first inside the payload");
        // These four are the cases the OLD rule silenced and the measurement says it should not have:
        // rebase(2 seeds, title-only), frozen(13), github(38), obligation(53).
        for seeds in [2usize, 13, 38, 53] {
            assert!(confident(false, 1, seeds, false), "{seeds} seeds was wrongly silenced at 2/8");
        }
        // Nothing found is still nothing pushed.
        assert!(!confident(false, 0, 0, false));
    }

    /// The uninformative-prompt check that DID survive, and where it actually lives: a prompt made only
    /// of function words and corpus-common words produces no seeding tokens at all.
    #[test]
    fn a_prompt_of_filler_and_common_words_yields_no_tokens() {
        let mut model = Model { items: HashMap::new(), edges: Vec::new() };
        for i in 0..300u32 {
            let mut attrs = HashMap::new();
            attrs.insert("title".to_string(), "the ceremony ran".to_string());
            model.items.insert(
                format!("ceremonyThing{i}"),
                ItemInfo { type_name: "Test".to_string(), attrs, marker: None, file: String::new() },
            );
        }
        // "thing" is filler; "ceremony" matches all 300 and is dropped as too common — and SAID.
        let (kept, dropped) = prompt_tokens(&model, "what about the thing at the ceremony");
        assert!(kept.is_empty(), "nothing informative survives: {kept:?}");
        assert!(dropped.iter().any(|d| d.starts_with("ceremony")), "it SAYS what it ignored: {dropped:?}");

        // A LIMITATION, found by this test's first premise being wrong rather than by design review:
        // a token under five characters is dropped for LENGTH before frequency is ever consulted, so
        // four-letter domain words never seed at all — `gate`, `hook`, `land`, `push`. `hooks` seeds
        // and `hook` does not, which is arbitrary from the reader's side. Recorded as issue294 rather
        // than tuned here, because lowering the floor re-admits `been`, `does` and `made` unless the
        // language list grows to compensate, and that trade needs measuring, not guessing.
        let (kept, _) = prompt_tokens(&model, "what about the hook");
        assert!(kept.is_empty(), "four-letter domain words do not seed today: {kept:?}");
    }

    /// D0243 rule 2: segments, not substrings. Every case here is measured from the real corpus.
    #[test]
    fn matching_is_on_word_segments_not_substrings() {
        assert_eq!(segments("dcWorkspaceDiscoveryIsComplete"), ["dc", "workspace", "discovery", "is", "complete"]);
        assert_eq!(segments("keel-gate.yml"), ["keel", "gate", "yml"]);
        assert_eq!(segments("d0239"), ["d0239"]);

        // The explosion this ends: `gate` matched 5,806 of the corpus by substring because it sits
        // inside all of these. None of them is a `gate`.
        assert!(!matches_term("propagated", "gate"));
        assert!(!matches_term("investigate", "gate"));
        // But a real segment still matches, in a name or a title, and camelCase counts.
        assert!(matches_term("staleGateProse", "gate"));
        assert!(matches_term("the commit gate belongs to the repository", "gate"));
        // A multi-word needle is matched as a phrase over the segments.
        assert!(matches_term("theWriteApiIsSanctioned", "write api"));
        assert!(!matches_term("theWriteApiIsSanctioned", "api write"));
    }

    /// D0243 rule 1: an identifier is the strongest seed and bypasses rarity entirely.
    #[test]
    fn identifier_tokens_are_recognised_and_plain_words_are_not() {
        for id in ["d0239", "issue293", "sprint477", "dcWorkspaceDiscoveryIsComplete", "kgPromptPathInjects"] {
            assert!(is_identifier_token(id), "{id} is an identifier this corpus mints");
        }
        for word in ["gate", "decision", "workspace", "the", "d", "DECISION"] {
            assert!(!is_identifier_token(word), "{word} is not an identifier");
        }
    }

    /// A budget must NARROW an answer, never erase it. Measured defect: with a tight budget the first
    /// row's substance overflowed before anything was written, so the payload printed "41 more
    /// reached, not shown" AND "(nothing reached)" - two contradictory claims and zero facts.
    #[test]
    fn a_tight_budget_still_shows_the_top_row() {
        let mut model = Model { items: HashMap::new(), edges: Vec::new() };
        let mut attrs = HashMap::new();
        attrs.insert("title".to_string(), "a title long enough to matter on its own".to_string());
        attrs.insert("decision".to_string(), "x".repeat(2000));
        model.items.insert(
            "d0001".to_string(),
            ItemInfo { type_name: "Decision".to_string(), attrs, marker: None, file: "f.sysml".to_string() },
        );
        let seeds = vec![("d0001".to_string(), "identifier `d0001`".to_string())];
        let agreement: HashMap<String, usize> = seeds.iter().map(|(n, _)| (n.clone(), 1)).collect();
        let out = brief_from_seeds(&model, "header:", &seeds, &agreement, 50);
        assert!(out.contains("d0001"), "the top row must survive any budget: {out}");
        assert!(!out.contains("(nothing reached)"), "must not claim nothing was reached: {out}");
    }

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
