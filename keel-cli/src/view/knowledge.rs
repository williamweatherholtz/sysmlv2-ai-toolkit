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

/// How many top-scoring elements SEED the graph walk (issue295). Every matching element is SCORED -
/// BM25 needs no cap - but the walk expands only from the best few, because expanding from every
/// element a common word touches reaches the whole model (the D0243 explosion) and hops then mean
/// nothing. The walk adds neighbours the words did not name; the score decides the order.
const TRAVERSAL_SEEDS: usize = 30;

/// BM25 parameters, the textbook defaults. `k1` saturates term frequency (a body that says
/// `provenance` nine times is not nine times as relevant); `b` is how fully length normalisation
/// applies (a long rationale is not more relevant for being long).
const BM25_K1: f64 = 1.2;
/// Share of an adjacent seed's score a reached element receives (before degree normalisation).
const LIFT: f64 = 0.5;
const BM25_B: f64 = 0.75;

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

/// IDF weight of a token: how much a match on it is worth (issue295).
///
/// REPLACES THE BINARY AGREEMENT VOTE, which the panel measured as inert. Agreement counted how many
/// distinct prompt tokens reached an element, capped at three by a prompt-word cap, and reached max 1
/// in 43 of 50 benchmark cases — so the primary sort key was CONSTANT and ranking degenerated to
/// (hops, name), i.e. alphabetical. That is the exact failure agreement was introduced to fix.
///
/// The graded version of the same intuition is inverse document frequency, and it is the standard one:
/// a token matching 5,000 elements says almost nothing about any of them, a token matching 6 says a
/// great deal, and co-occurrence still accumulates because scores ADD. It also treats a hit on
/// `keystone` as worth more than a hit on `paths`, which the vote could not.
fn idf(corpus: usize, hits: usize) -> f64 {
    // BM25's IDF, with the +0.5 smoothing that keeps a term matching most of the corpus from going
    // negative. `hits == 0` cannot reach here (a token with no hits is never kept as a seed).
    #[allow(clippy::cast_precision_loss)]
    let (n, df) = (corpus as f64, hits.max(1) as f64);
    ((n - df + 0.5) / (df + 0.5)).ln_1p()
}

/// The query terms of a free-form PROMPT: identifiers, and every word of three letters or more that
/// is not English filler. No rarity ceiling and no cap on count (issue295): BM25's IDF weights a common
/// word near zero instead of deleting it, and a word that matches nothing scores nothing. Returned with
/// the dropped filler so the caller can SAY what it ignored.
fn query_terms(prompt: &str) -> (Vec<(String, &'static str)>, Vec<String>) {
    let mut seen: HashSet<String> = HashSet::new();
    let mut kept: Vec<(String, &'static str)> = Vec::new();
    let mut dropped: Vec<String> = Vec::new();
    for raw in prompt.split(|c: char| !c.is_alphanumeric() && c != '-') {
        let tok = raw.trim_matches('-');
        if tok.chars().count() < 3 {
            continue;
        }
        if is_identifier_token(tok) {
            if seen.insert(tok.to_lowercase()) {
                kept.push((tok.to_string(), "identifier"));
            }
            continue;
        }
        // The index is built from SEGMENTS, so the query must be too: `issue-resolution` is the two
        // segments issue and resolution, `ProspectiveChange` is prospective and change. Measured: a
        // 14-word query from an element's own body lost 8 words as "unknown" before this, and the
        // element it was drawn from ranked below the process named `dor`.
        for seg in segments(tok) {
            if seg.chars().count() < 3 || !seen.insert(seg.clone()) {
                continue;
            }
            if is_function_word(&seg) {
                // MEASURED (D0243): `made` matched 7 elements - rare, and semantically empty. Frequency
                // cannot separate a rare DOMAIN term from a rare filler word, so language is filtered
                // as language; the list is English, not this corpus's vocabulary.
                dropped.push(seg);
            } else {
                kept.push((seg, "word"));
            }
        }
    }
    dropped.sort();
    (kept, dropped)
}

/// One element's term bags for BM25F scoring, one per field, with each field's length in segments.
///
/// PER FIELD, not one bag: normalising a whole element by its total length let a two-segment name
/// with a tiny title score 2.0 x idf - above BM25's own saturation ceiling of 1.0 - while a Decision
/// that said the word once in a long rationale scored 0.1 x idf. Measured on `rebase`: d0129 ranked
/// 29th of 30 seeds behind every short item NAMED with the word. BM25F normalises each field against
/// the average length of THAT field, weights, sums, then saturates once.
struct Doc {
    tf: [HashMap<String, f64>; 3],
    len: [f64; 3],
}

/// The fielded index over every element, with the corpus average length PER FIELD.
struct Index {
    docs: HashMap<String, Doc>,
    avg_len: [f64; 3],
}

/// Field order in the per-field arrays: name, title, body.
const FIELDS: [Field; 3] = [Field::Name, Field::Title, Field::Body];

impl Index {
    fn build(model: &Model) -> Self {
        let mut docs: HashMap<String, Doc> = HashMap::with_capacity(model.items.len());
        let mut total = [0.0f64; 3];
        for (name, info) in &model.items {
            let mut tf: [HashMap<String, f64>; 3] = Default::default();
            let mut len = [0.0f64; 3];
            let add = |bag: &mut HashMap<String, f64>, count: &mut f64, text: &str| {
                for seg in segments(text) {
                    *bag.entry(seg).or_insert(0.0) += 1.0;
                    *count += 1.0;
                }
            };
            let [tf_name, tf_title, tf_body] = &mut tf;
            let [len_name, len_title, len_body] = &mut len;
            add(tf_name, len_name, name);
            if let Some(t) = info.attrs.get("title") {
                add(tf_title, len_title, t);
            }
            for f in BODY_FIELDS {
                if let Some(v) = info.attrs.get(f) {
                    add(tf_body, len_body, v);
                }
            }
            for (t, l) in total.iter_mut().zip(len.iter()) {
                *t += l;
            }
            docs.insert(name.clone(), Doc { tf, len });
        }
        #[allow(clippy::cast_precision_loss)]
        let n = docs.len().max(1) as f64;
        let avg_len = total.map(|t| (t / n).max(1.0));
        Self { docs, avg_len }
    }

    /// Documents containing `term` in any field - BM25's document frequency.
    fn df(&self, term: &str) -> usize {
        self.docs.values().filter(|d| d.tf.iter().any(|m| m.contains_key(term))).count()
    }

    /// BM25F contribution of one term to every element that contains it: `(element, score)`. Each
    /// field's tf is normalised by that field's length ratio, weighted (name 3 > title 2 > body 1),
    /// summed, and saturated ONCE by `k1` - so no element can exceed `idf * (k1 + 1)` however short.
    fn score_term(&self, term: &str, idf: f64) -> Vec<(String, f64)> {
        self.docs
            .iter()
            .filter_map(|(name, d)| {
                let mut tfn = 0.0;
                for (((field, bag), len), avg) in FIELDS.iter().zip(d.tf.iter()).zip(d.len.iter()).zip(self.avg_len.iter()) {
                    if let Some(tf) = bag.get(term) {
                        let b_norm = 1.0 - BM25_B + BM25_B * len / avg;
                        tfn += field.weight() * tf / b_norm;
                    }
                }
                (tfn > 0.0).then(|| (name.clone(), idf * tfn * (BM25_K1 + 1.0) / (tfn + BM25_K1)))
            })
            .collect()
    }
}

/// The substantive fields a seed may match, beyond name and title (issue295).
///
/// These were ALREADY read by this module — to PRINT the top rows — and were invisible to RETRIEVAL.
/// An adversarial panel measured the consequence: a prompt about provenance and defaulted dates
/// contributed ZERO seeds for `provenance`, the most informative word in it, because the word appears
/// in dozens of bodies and in no name or title.
const BODY_FIELDS: [&str; 6] = ["decision", "description", "rationale", "actionText", "consequences", "procedureText"];

/// Where a term matched, and how much that is worth. A name is what an author chose to call a thing; a
/// title is how they summarised it; a body mention is evidence but weaker, and there is a lot more body
/// text than title text, so an unweighted body match would swamp the ranking.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Name,
    Title,
    Body,
    Alias,
}

impl Field {
    /// Field weight, in the spirit of a fielded BM25: name > title > body.
    const fn weight(self) -> f64 {
        match self {
            Self::Name | Self::Alias => 3.0,
            Self::Title => 2.0,
            Self::Body => 1.0,
        }
    }
    const fn label(self) -> &'static str {
        match self {
            Self::Name => "name match",
            Self::Title => "title match",
            Self::Body => "body match",
            Self::Alias => "alias",
        }
    }
}

/// Seed the traversal: items whose NAME or TITLE contains `term`, plus every target of an Alias
/// whose `term` matches. Returns `(seed name, how it was found)` pairs, sorted for determinism.
fn seeds_for(model: &Model, term: &str) -> Vec<(String, String)> {
    seeds_scored(model, term).into_iter().map(|(n, f, _)| (n, f.label().to_string())).collect()
}

/// Seeds with the FIELD each matched in, so a caller can weight them (issue295).
///
/// Body fields are searched here where they were not before. Name is checked first and wins, then
/// title, then bodies — one hit per element, strongest field, so an element mentioned in three bodies
/// does not outrank one named for the term.
fn seeds_scored(model: &Model, term: &str) -> Vec<(String, Field, f64)> {
    let mut out: Vec<(String, Field, f64)> = Vec::new();
    for (name, info) in &model.items {
        let field = if matches_term(name, term) {
            Some(Field::Name)
        } else if info.attrs.get("title").is_some_and(|t| matches_term(t, term)) {
            Some(Field::Title)
        } else if BODY_FIELDS
            .iter()
            .any(|f| info.attrs.get(*f).is_some_and(|v| matches_term(v, term)))
        {
            Some(Field::Body)
        } else {
            None
        };
        if let Some(f) = field {
            out.push((name.clone(), f, f.weight()));
        }
        if info.type_name == "Alias" && info.attrs.get("term").is_some_and(|t| matches_term(t, term)) {
            for e in model.edges.iter().filter(|e| e.kind == "dependency" && e.from == *name) {
                if model.items.contains_key(&e.to) {
                    out.push((e.to.clone(), Field::Alias, Field::Alias.weight()));
                }
            }
        }
    }
    out.sort_by(|a, b| (&a.0, a.1.label()).cmp(&(&b.0, b.1.label())));
    out.dedup_by(|a, b| a.0 == b.0);
    out
}

/// The targets of every Alias whose `term` matches `term` - the one lookup that must stay cheap per
/// query word: it reads ALIAS items only, never the bodies (which made a recall take 51 seconds when
/// it went through `seeds_scored`).
fn alias_targets(model: &Model, term: &str) -> Vec<(String, Field, f64)> {
    let mut out = Vec::new();
    for (name, info) in &model.items {
        if info.type_name == "Alias" && info.attrs.get("term").is_some_and(|t| matches_term(t, term)) {
            for e in model.edges.iter().filter(|e| e.kind == "dependency" && e.from == *name) {
                if model.items.contains_key(&e.to) {
                    out.push((e.to.clone(), Field::Alias, Field::Alias.weight()));
                }
            }
        }
    }
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
    score: &HashMap<String, f64>,
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
    // THE GRAPH LIFTS WHAT THE WORDS POINT AT (issue295). Before this, a neighbour reached by the walk
    // kept only its own lexical score, so the walk could only APPEND: for "why must we never rebase",
    // the requirement and DoD derived from d0129 led and d0129 itself sat 47th, although half the top
    // seeds link to it. An element that many high-scoring seeds link to is what those seeds are ABOUT.
    // Each reached element receives a share of every adjacent seed's score, divided by the square root
    // of its own degree so a hub every gate verifies does not collect the whole payload.
    let mut degree: HashMap<&str, f64> = HashMap::new();
    for e in &model.edges {
        *degree.entry(e.from.as_str()).or_default() += 1.0;
        *degree.entry(e.to.as_str()).or_default() += 1.0;
    }
    let seed_set: HashSet<&str> = seed_names.iter().map(String::as_str).collect();
    let mut lifted: HashMap<String, f64> = HashMap::new();
    for e in &model.edges {
        for (from, to) in [(&e.from, &e.to), (&e.to, &e.from)] {
            if seed_set.contains(from.as_str()) && reached.contains_key(to) && !seed_set.contains(to.as_str()) {
                *lifted.entry(to.clone()).or_insert(0.0) += score.get(from).copied().unwrap_or(0.0);
            }
        }
    }
    let final_score: HashMap<String, f64> = reached
        .keys()
        .map(|n| {
            let own = score.get(n).copied().unwrap_or(0.0);
            let lift = lifted.get(n).copied().unwrap_or(0.0) * LIFT / degree.get(n.as_str()).copied().unwrap_or(1.0).sqrt();
            (n.clone(), own + lift)
        })
        .collect();
    let score = &final_score;
    let mut sorted: Vec<(&String, &(usize, String))> = reached.iter().collect();
    sorted.sort_by(|a, b| {
        // Score DESC, then hops ASC, then name — and an element the prompt NAMED outright is pinned
        // first, because `d0239` is not a guess about relevance. Alphabetical is now only the final
        // tie-break between elements of equal score AND equal distance, where it is arbitrary but
        // harmless; before, it was the primary order in 43 of 50 cases.
        let key = |n: &String, h: usize| {
            let s = if named.contains(n.as_str()) { f64::INFINITY } else { *score.get(n).unwrap_or(&0.0) };
            (s, h, n.clone())
        };
        let (ka, kb) = (key(a.0, a.1 .0), key(b.0, b.1 .0));
        kb.0.partial_cmp(&ka.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| (ka.1, &ka.2).cmp(&(kb.1, &kb.2)))
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
    let score: HashMap<String, f64> = seeds.iter().map(|(n, _)| (n.clone(), 1.0)).collect();
    Ok(brief_from_seeds(&model, &format!("recalled for term `{term}`:"), &seeds, &score, budget))
}

/// `keel recall --prompt -` : seed from a free-form PROMPT and return a budgeted brief (D0242 part 2,
/// D0243 precision rules). ZERO model calls — this is code finding facts before the model is involved.
///
/// # Errors
/// Propagates model-build failures.
pub fn recall_for_prompt(root: &Path, prompt: &str, budget: usize) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let (terms, dropped) = query_terms(prompt);
    let index = Index::build(&model);
    let corpus = model.items.len();
    // BM25F SCORE per element, accumulated across terms (issue295). Scores add, so two terms agreeing
    // on one element still beats one - the signal agreement was reaching for - while IDF makes a rare
    // term worth more than a common one and length normalisation stops a long rationale winning for
    // being long. A term matching most of the corpus contributes almost nothing instead of being
    // deleted, so nothing is "too common" any more; a term matching nothing is reported as unknown.
    let mut score: HashMap<String, f64> = HashMap::new();
    let mut how: HashMap<String, String> = HashMap::new();
    let mut matched: Vec<String> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();
    let mut has_identifier = false;
    for (tok, kind) in &terms {
        if *kind == "identifier" {
            has_identifier = true;
            // An identifier is not a guess about relevance: weight it as the rarest possible term.
            for (n, field, fw) in seeds_scored(&model, tok) {
                *score.entry(n.clone()).or_insert(0.0) += idf(corpus, 1) * fw;
                how.entry(n).or_insert_with(|| format!("identifier `{tok}` / {}", field.label()));
            }
            matched.push(tok.clone());
            continue;
        }
        let df = index.df(tok);
        // Aliases (the human-owned vocabulary under .knowledge/) still route a term to their targets.
        let alias_hits = alias_targets(&model, tok);
        if df == 0 && alias_hits.is_empty() {
            unknown.push(tok.clone());
            continue;
        }
        matched.push(format!("{tok}({df})"));
        let w = idf(corpus, df.max(1));
        for (n, s) in index.score_term(tok, w) {
            *score.entry(n.clone()).or_insert(0.0) += s;
            how.entry(n).or_insert_with(|| format!("word `{tok}`"));
        }
        for (n, field, fw) in alias_hits {
            *score.entry(n.clone()).or_insert(0.0) += w * fw;
            how.entry(n).or_insert_with(|| format!("word `{tok}` / {}", field.label()));
        }
    }
    if score.is_empty() {
        // SAYING SO is the point: a silent empty recall is indistinguishable from a broken one.
        let mut msg = String::from("recall: no term of this prompt matches the model — nothing pushed.\n");
        if !unknown.is_empty() {
            let _ = writeln!(msg, "  unknown to the model: {}", unknown.join(", "));
        }
        if !dropped.is_empty() {
            let _ = writeln!(msg, "  ignored as filler: {}", dropped.join(", "));
        }
        return Ok(msg);
    }
    // The walk expands from the best-scoring few (and every element an identifier NAMED); everything
    // scored is still ranked, so a strong lexical match outside the seed set is not lost.
    let mut ranked: Vec<(&String, &f64)> = score.iter().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(b.0)));
    let seeds: Vec<(String, String)> = ranked
        .iter()
        .enumerate()
        .filter(|(i, (n, _))| *i < TRAVERSAL_SEEDS || how.get(*n).is_some_and(|h| h.starts_with("identifier")))
        .map(|(_, (n, _))| ((*n).clone(), how.get(*n).cloned().unwrap_or_default()))
        .collect();
    let best = ranked.first().map_or(0.0, |(_, s)| **s);
    let mut header = format!(
        "recalled on {} [corpus {corpus} items, {} scored, top score {best:.1}{}]:",
        matched.join(", "),
        score.len(),
        if has_identifier { ", named" } else { "" }
    );
    if !unknown.is_empty() {
        let _ = write!(header, " [unknown to the model: {}]", unknown.join(", "));
    }
    if !dropped.is_empty() {
        let _ = write!(header, " [ignored as filler: {}]", dropped.join(", "));
    }
    Ok(brief_from_seeds(&model, &header, &seeds, &score, budget))
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
/// Is there anything worth PUSHING unasked?
///
/// The honest answer turned out to be "does any term of the prompt occur in the model at all" (an
/// identifier, or a word with a non-zero document frequency) — so this is that check, named for
/// what it does, and nothing more.
///
/// # A confidence gate lived here and was wrong twice
///
/// Measured on eight questions with ground truth fixed in advance: `identifier | agreement>=2 |
/// (<=12 seeds AND a name match)` scored 2/8 while NO bar at all scored 5/8, and a second tuning
/// attempt also landed at 2/8. The bar suppressed more right answers than wrong ones, because seed
/// count is not a proxy for relevance — the prompts it wrongly silenced had 13 to 53 seeds, the same
/// range as the ones it admitted.
///
/// It was removed, and then the REMOVAL was the defect: `confident()` kept its four parameters,
/// discarded them all, and returned `seed_count > 0`, while three surfaces went on advertising a gate
/// — a doc comment listing it as kill switch 3, a PASSING `DoD` asserting "injection is gated on a
/// CONFIDENCE verdict", and a printed HIGH/LOW label whose LOW was unreachable whenever any row was
/// shown. An adversarial panel found all three; the label was stamping "confidence HIGH" on a payload
/// that was 14 of 17 rows off-topic. Dead code with live prose is worse than either alone, because the
/// prose is what a reader believes.
/// # Errors
/// Propagates model-build failures; the caller treats an error as "push nothing" (fail-open).
pub fn has_pushable_facts(root: &Path, prompt: &str) -> Result<bool, ViewError> {
    let model = Model::build(root)?;
    let (terms, _) = query_terms(prompt);
    if terms.iter().any(|(_, k)| *k == "identifier") {
        return Ok(true);
    }
    let index = Index::build(&model);
    Ok(terms.iter().any(|(t, _)| index.df(t) > 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A prompt of filler and words the model does not know pushes nothing, and SAYS which is which.
    /// A common word is no longer deleted as "too common" - BM25's IDF weights it near zero instead -
    /// and a four-letter domain word seeds (issue294: `hook` used to be dropped for length alone).
    #[test]
    fn filler_is_dropped_common_words_are_weighted_not_deleted_and_short_words_seed() {
        let mut model = Model { items: HashMap::new(), edges: Vec::new() };
        for i in 0..300u32 {
            let mut attrs = HashMap::new();
            attrs.insert("title".to_string(), "the ceremony ran".to_string());
            if i == 7 {
                attrs.insert("description".to_string(), "the hook fired at the ceremony".to_string());
            }
            model.items.insert(
                format!("ceremonyThing{i}"),
                ItemInfo { type_name: "Test".to_string(), attrs, marker: None, file: String::new() },
            );
        }
        let (kept, dropped) = query_terms("what about the thing at the ceremony, its closeOut and force-push");
        assert!(dropped.contains(&"thing".to_string()), "filler is dropped and named: {dropped:?}");
        assert!(kept.iter().any(|(t, _)| t == "ceremony"), "a plain word is KEPT for weighting: {kept:?}");
        // MEASURED: a dedupe on the raw token silently dropped every plain word and kept only the
        // segmented ones - a 14-word query recalled on two terms. Both shapes must survive together.
        for seg in ["force", "push", "what", "ceremony"] {
            assert!(kept.iter().any(|(t, _)| t == seg), "segment `{seg}` must be a query term: {kept:?}");
        }
        assert!(kept.contains(&("closeOut".to_string(), "identifier")), "a camelCase token is an identifier, matched by name: {kept:?}");
        assert_eq!(kept.iter().filter(|(t, _)| t == "ceremony").count(), 1, "deduped once, not dropped");
        let index = Index::build(&model);
        assert_eq!(index.df("ceremony"), 300);
        assert!(idf(300, 300) < idf(300, 1), "a word in every document is worth almost nothing");
        // `hook` appears in ONE body: it seeds, and that element is the top score.
        let scored = index.score_term("hook", idf(300, index.df("hook")));
        assert_eq!(scored.len(), 1, "four-letter domain words seed now: {scored:?}");
        assert_eq!(scored[0].0, "ceremonyThing7");
        // length normalisation: the same tf in a longer document scores lower
        let mut long = model.items.get("ceremonyThing7").cloned().expect("doc");
        long.attrs.insert("rationale".to_string(), "many many words of unrelated rationale text ".repeat(20));
        model.items.insert("ceremonyLong".to_string(), long);
        let index = Index::build(&model);
        let scored: HashMap<String, f64> = index.score_term("hook", 1.0).into_iter().collect();
        assert!(scored["ceremonyLong"] < scored["ceremonyThing7"], "longer document, same tf, lower score: {scored:?}");
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
        let score: HashMap<String, f64> = seeds.iter().map(|(n, _)| (n.clone(), 1.0)).collect();
        let out = brief_from_seeds(&model, "header:", &seeds, &score, 50);
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
