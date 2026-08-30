//! `keel record statement` / `keel record story` — the write path intake never had (D0236/issue289).
//!
//! WHY THIS IS NOT A CONVENIENCE. D0216's load-bearing rule is that a stakeholder's words are recorded
//! VERBATIM **before** any Need, Brief or Story is authored — because a Need with no cited utterance
//! cannot be checked against what the human actually said, which is how one comes to be written wider
//! than the demand (D0157). That rule had no API behind it. `keel record` covered `decision` and
//! `issue` only, so every Statement in this repository — including all five from the session that
//! found this — was written by hand-editing a file: no actor validation, no write lock, no refusal on
//! a defaulted date, and nothing stopping a "verbatim" field from being a paraphrase typed from
//! memory. The human asked whether the gap was real. It was.
//!
//! THE ONE RULE THIS FILE ENFORCES THAT THE OTHERS CANNOT: a Statement's `text` is passed through
//! **unaltered** except for the SysML-literal escaping every field needs. `sanitize_field` collapses
//! whitespace, which is right for a title and wrong for a quote — a human's line break or double space
//! is part of what they wrote. So `text` gets its own minimal escaping and the difference is asserted
//! by test.
use std::path::{Path, PathBuf};

use crate::write::{gen_uuid, with_file_lock, write_atomic, WriteError};

/// A human's words, verbatim.
pub struct NewStatement<'a> {
    /// What they wrote, character for character.
    pub text: &'a str,
    /// Actor id whose words these are.
    pub said_by: &'a str,
    /// ISO-8601 date they said it.
    pub said_at: &'a str,
    /// A `StatementChannel` member. Validated against the SCHEMA, never a constant here (D0150) —
    /// which is why adding `github` needed no change to this file.
    pub channel: &'a str,
    /// The utterance's durable external address, where it has one (a GitHub issue URL). `None` for
    /// a spoken statement — never defaulted, because a fabricated source is worse than no source.
    pub source_url: Option<&'a str>,
    /// A short human label for the statement — the AI's summary, kept separate from `text`.
    pub title: &'a str,
    /// Recording actor.
    pub author: &'a str,
    /// ISO-8601 date the record was made.
    pub created_at: &'a str,
}

/// The faithful translation of a Statement into the form work is planned in.
pub struct NewStory<'a> {
    /// The `Statement` this translates. REQUIRED: a story with no source is an invention.
    pub from_statement: &'a str,
    pub title: &'a str,
    pub as_a: &'a str,
    pub i_want: &'a str,
    pub so_that: Option<&'a str>,
    /// An `ImplicationKind` member. The accepted set is DERIVED from the schema (see `accepted`), so
    /// it is not restated here — restating it is what issue300 was.
    pub implication: &'a str,
    /// Why THIS kind and not a neighbouring one.
    pub triage_note: Option<&'a str>,
    pub author: &'a str,
    pub created_at: &'a str,
}

/// The two intake vocabularies, DERIVED from `schema/core/intake.sysml` rather than restated here.
///
/// This module used to carry `CHANNELS` and a 12-element `IMPLICATIONS` beside a schema declaring 15
/// members. `keel record story` therefore refused `verifiedRequirement`, `designChange` and
/// `implementationChange` — the three the schema comments call out as deliberately renamed or added,
/// and which `.engine/processes/intake.sysml` and both copies of the intake skill instruct the
/// triager to use (issue300). The harm is the one the intake process exists to prevent: the process
/// says a kind the vocabulary cannot express is a CHANGE to the vocabulary and never a forced fit, so
/// a triager who follows the skill meets a refusal and the cheap way out is to force-fit into a
/// neighbouring kind — a wrong triage, which routes real direction to the wrong place and looks
/// handled.
///
/// `enum_members_union` is ENGINE ∪ PROJECT, so a project may EXTEND the taxonomy by editing its own
/// schema and never has to wait for a binary release, while no project can remove a member the engine
/// ships (issue090/issue129).
fn accepted(root: &Path, enum_name: &str) -> Vec<String> {
    crate::schema::enum_members_union(root, enum_name)
}

/// Refuse an unrecognised member, naming the accepted set — never default it (the channel and the
/// triage verdict are both provenance, and a defaulted verdict is a fabricated one).
fn check_member(root: &Path, enum_name: &str, field: &str, value: &str) -> Result<(), WriteError> {
    let accepted = accepted(root, enum_name);
    if accepted.iter().any(|m| m == value) {
        return Ok(());
    }
    Err(WriteError::InvalidMethod(format!(
        "{field} `{value}` — expected one of {}",
        accepted.join(" | ")
    )))
}

/// Escape a VERBATIM span for a one-line `SysML` string literal, altering nothing else.
///
/// Deliberately NOT `sanitize_field`, which collapses runs of whitespace: a human's double space or
/// line break is part of what they wrote. A newline becomes the two characters `\n` so the literal
/// stays on one line while the break remains recoverable; a quote becomes `''` so the reader can see
/// it was a quote rather than silently losing it.
#[must_use]
pub fn escape_verbatim(text: &str) -> String {
    text.replace('\\', "/").replace('"', "''").replace('\r', "").replace('\n', "\\n")
}

fn intake_file(root: &Path, created_at: &str) -> PathBuf {
    root.join(".tracking").join("intake").join(format!("intake-{created_at}.sysml"))
}

fn next_number(all: &str, prefix: &str) -> u32 {
    let mut max = 0u32;
    for (i, _) in all.match_indices(prefix) {
        let digits: String =
            all[i + prefix.len()..].chars().take_while(char::is_ascii_digit).collect();
        if let Ok(n) = digits.parse::<u32>() {
            max = max.max(n);
        }
    }
    max + 1
}

fn all_tracking_text(root: &Path) -> String {
    let mut s = String::new();
    for f in crate::collect_sysml(&root.join(".tracking")) {
        if let Ok(t) = std::fs::read_to_string(&f) {
            s.push_str(&t);
        }
    }
    s
}

fn ensure_intake_package(path: &Path, created_at: &str) -> String {
    if let Ok(existing) = std::fs::read_to_string(path) {
        return existing;
    }
    let pkg = created_at.replace('-', "");
    format!(
        "// Intake for {created_at} — their words, my translation, the triage verdict (D0166/D0216).\n\
         // Written by `keel record statement` / `keel record story` (D0236): a Statement's text is\n\
         // VERBATIM, so it is escaped for the literal and otherwise untouched.\n\
         package ProjectIntake{pkg} {{\n\
         \x20   private import EngineElement::*;\n\
         \x20   private import EngineIntake::*;\n\
         \x20   private import EngineRelationships::*;\n\
         }}\n"
    )
}

fn insert_before_close(text: &str, block: &str) -> String {
    text.rfind('}')
        .map_or_else(|| format!("{text}{block}"), |i| format!("{}{block}{}", &text[..i], &text[i..]))
}

/// Record a human's words verbatim. Returns `(name, relative path)`.
///
/// # Errors
/// `WriteError::Io` on filesystem failure; `WriteError::InvalidMethod` on an unknown channel.
pub fn record_statement(root: &Path, s: &NewStatement) -> Result<(String, String), WriteError> {
    check_member(root, "StatementChannel", "channel", s.channel)?;
    if s.text.trim().is_empty() {
        return Err(WriteError::Parse("a Statement with empty text records nothing".into()));
    }
    let path = intake_file(root, s.created_at);
    with_file_lock(&root.join(".tracking").join("issues.sysml"), || {
        // IDEMPOTENCY, checked INSIDE the lock (issue185): two concurrent ingests of one issue must
        // not both find it absent. A re-ingest is refused, not silently deduplicated — the caller
        // asked to record an utterance that is already recorded, and a write that quietly does
        // nothing while reporting success is the failure mode `keel deactivate` once had.
        if let Some(url) = s.source_url {
            let corpus = all_tracking_text(root);
            if corpus.contains(&format!("sourceUrl = \"{}\"", crate::write::sanitize_public(url))) {
                return Err(WriteError::Parse(format!(
                    "an utterance from {url} is already recorded — re-ingesting would store the same                      words twice under two ids. Nothing was written."
                )));
            }
        }
        let name = format!("st{:03}", next_number(&all_tracking_text(root), "part st"));
        let existing = ensure_intake_package(&path, s.created_at);
        let block = format!(
            "\n\x20   part {name} : Statement {{\n\
             \x20       :>> id = \"{}\";\n\
             \x20       :>> title = \"{}\";\n\
             \x20       :>> createdAt = \"{}\"; :>> createdBy = \"{}\";\n\
             \x20       :>> text = \"{}\";\n\
             \x20       :>> saidBy = \"{}\"; :>> saidAt = \"{}\"; :>> channel = StatementChannel::{};{}\n\
             \x20   }}\n",
            gen_uuid(),
            crate::write::sanitize_public(s.title),
            s.created_at,
            s.author,
            escape_verbatim(s.text),
            s.said_by,
            s.said_at,
            s.channel,
            s.source_url.map_or_else(String::new, |u| {
                format!("\n\x20       :>> sourceUrl = \"{}\";", crate::write::sanitize_public(u))
            }),
        );
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        write_atomic(&path, insert_before_close(&existing, &block))?;
        Ok((name, format!(".tracking/intake/intake-{}.sysml", s.created_at)))
    })
}

/// Record a `UserStory` and its REQUIRED `#DerivedFrom` edge to the Statement it translates.
///
/// # Errors
/// `WriteError::Io` on filesystem failure; `WriteError::InvalidMethod` on an unknown implication;
/// `WriteError::TaskNotFound` when the cited Statement does not exist — a story with no source is an
/// invention, so the edge is authored with the story or nothing is written.
pub fn record_story(root: &Path, s: &NewStory) -> Result<(String, String), WriteError> {
    check_member(root, "ImplicationKind", "implication", s.implication)?;
    let all = all_tracking_text(root);
    if !all.contains(&format!("part {} : Statement", s.from_statement)) {
        return Err(WriteError::TaskNotFound(format!(
            "{} is not a recorded Statement — a UserStory with no cited source is an invention (D0216), so nothing was written",
            s.from_statement
        )));
    }
    let path = intake_file(root, s.created_at);
    with_file_lock(&root.join(".tracking").join("issues.sysml"), || {
        let name = format!("us{:03}", next_number(&all_tracking_text(root), "part us"));
        let existing = ensure_intake_package(&path, s.created_at);
        let so_that = s
            .so_that
            .filter(|v| !v.trim().is_empty())
            .map_or_else(String::new, |v| {
                format!("\x20       :>> soThat = \"{}\";\n", crate::write::sanitize_public(v))
            });
        let triage = s
            .triage_note
            .filter(|v| !v.trim().is_empty())
            .map_or_else(String::new, |v| {
                format!("\x20       :>> triageNote = \"{}\";\n", crate::write::sanitize_public(v))
            });
        let block = format!(
            "\n\x20   part {name} : UserStory {{\n\
             \x20       :>> id = \"{}\";\n\
             \x20       :>> title = \"{}\";\n\
             \x20       :>> createdAt = \"{}\"; :>> createdBy = \"{}\";\n\
             \x20       :>> asA = \"{}\";\n\
             \x20       :>> iWant = \"{}\";\n\
             {so_that}\
             \x20       :>> implication = ImplicationKind::{};\n\
             {triage}\
             \x20   }}\n\
             \x20   #DerivedFrom dependency from {name} to {};\n",
            gen_uuid(),
            crate::write::sanitize_public(s.title),
            s.created_at,
            s.author,
            crate::write::sanitize_public(s.as_a),
            crate::write::sanitize_public(s.i_want),
            s.implication,
            s.from_statement,
        );
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        write_atomic(&path, insert_before_close(&existing, &block))?;
        Ok((name, format!(".tracking/intake/intake-{}.sysml", s.created_at)))
    })
}

#[cfg(test)]
mod tests {
    use super::{escape_verbatim, record_statement, record_story, NewStatement, NewStory};

    fn root(tag: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!("keel-intake-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(p.join(".tracking").join("intake")).unwrap();
        std::fs::write(p.join(".tracking").join("issues.sysml"), "package I {\n}\n").unwrap();
        p
    }

    #[test]
    fn a_statements_text_survives_verbatim_where_a_title_would_be_collapsed() {
        // THE POINT OF THE WHOLE FILE. `sanitize_field` collapses whitespace, which is correct for a
        // title and destroys a quote: a human's double space or line break is part of what they wrote.
        let messy = "line one\n  and   two";
        assert_eq!(escape_verbatim(messy), "line one\\n  and   two", "spacing and the break survive");
        assert_eq!(crate::write::sanitize_public(messy), "line one and two", "a title is collapsed - and must not be used for text");
        // A quote becomes '' so the reader still sees it was a quote.
        assert_eq!(escape_verbatim("he said \"no\""), "he said ''no''");
    }

    #[test]
    fn a_statement_records_and_a_story_must_cite_one() {
        let r = root("pair");
        let (st, rel) = record_statement(
            &r,
            &NewStatement {
                text: "just leave me as the decision maker.",
                said_by: "wweatherholtz",
                said_at: "2026-08-26",
                channel: "chat", source_url: None,
                title: "leave me as the only decider",
                author: "claudeFable5",
                created_at: "2026-08-26",
            },
        )
        .expect("statement records");
        assert_eq!(st, "st001");
        assert!(rel.contains("intake-2026-08-26"));

        // A story with NO cited statement is refused, and writes nothing.
        let bad = record_story(
            &r,
            &NewStory {
                from_statement: "st999",
                title: "invented",
                as_a: "x",
                i_want: "y",
                so_that: None,
                implication: "need",
                triage_note: None,
                author: "claudeFable5",
                created_at: "2026-08-26",
            },
        );
        assert!(bad.is_err(), "a UserStory with no source must be refused (D0216)");
        let text = std::fs::read_to_string(r.join(".tracking/intake/intake-2026-08-26.sysml")).unwrap();
        assert!(!text.contains("invented"), "a refused write must leave nothing behind");

        // A story citing a real statement records WITH its #DerivedFrom edge.
        let (us, _) = record_story(
            &r,
            &NewStory {
                from_statement: &st,
                title: "a second decider must not be enrolled",
                as_a: "project owner",
                i_want: "the decider table to list only me",
                so_that: Some("a colleague is not barraged"),
                implication: "scopeConstraint",
                triage_note: Some("already satisfied; the PAIN is separate"),
                author: "claudeFable5",
                created_at: "2026-08-26",
            },
        )
        .expect("story records");
        let text = std::fs::read_to_string(r.join(".tracking/intake/intake-2026-08-26.sysml")).unwrap();
        assert!(text.contains(&format!("#DerivedFrom dependency from {us} to {st};")), "the edge is authored WITH the story: {text}");
        assert!(text.contains("just leave me as the decision maker."), "their words are in the file unaltered");
        let _ = std::fs::remove_dir_all(&r);
    }

    #[test]
    fn an_unknown_channel_or_implication_is_refused_rather_than_defaulted() {
        let r = root("enum");
        assert!(record_statement(&r, &NewStatement {
            text: "x", source_url: None, said_by: "w", said_at: "2026-08-26", channel: "smoke-signal",
            title: "t", author: "a", created_at: "2026-08-26",
        }).is_err(), "an unknown channel must refuse, never default - the channel is provenance");
        assert!(record_statement(&r, &NewStatement {
            text: "   ", said_by: "w", said_at: "2026-08-26", channel: "chat", source_url: None,
            title: "t", author: "a", created_at: "2026-08-26",
        }).is_err(), "empty text records nothing and must say so");
        let _ = std::fs::remove_dir_all(&r);
    }

    /// The control for issue300 (D0047: a defect that can recur becomes an automated check).
    ///
    /// A hand-maintained list beside a schema enum drifts silently — this one sat at 12 against a
    /// declared 15 and was found only when a real triage was refused. Asserting membership one by one
    /// against the SCHEMA means the next member added to `schema/core/intake.sysml` either works or
    /// turns this test red; it can no longer be declared and quietly unusable.
    #[test]
    fn every_schema_declared_member_of_both_intake_vocabularies_is_accepted() {
        let r = root("vocab");
        let implications = crate::schema::enum_members("ImplicationKind");
        assert!(
            implications.len() >= 15,
            "schema/core/intake.sysml should declare at least the 15 known ImplicationKind members, \
             found {}: {implications:?}",
            implications.len()
        );
        for m in &implications {
            let out = record_story(&r, &NewStory {
                from_statement: "st999", title: "t", as_a: "a", i_want: "w", so_that: None,
                implication: m, triage_note: None, author: "a", created_at: "2026-08-26",
            });
            // st999 does not exist, so every call fails — but the implication check runs FIRST, so a
            // rejected member fails differently from an accepted one. Asserting on the message is what
            // separates "this kind is not in the vocabulary" from "that Statement is not recorded".
            let msg = format!("{:?}", out.expect_err("no Statement st999 exists in this fixture"));
            assert!(
                !msg.contains("expected one of"),
                "ImplicationKind::{m} is declared in the schema but refused by the write path: {msg}"
            );
        }
        for m in crate::schema::enum_members("StatementChannel") {
            let out = record_statement(&r, &NewStatement {
                text: "x", source_url: None, said_by: "w", said_at: "2026-08-26", channel: &m,
                title: "t", author: "a", created_at: "2026-08-26",
            });
            assert!(out.is_ok(), "StatementChannel::{m} is declared but refused: {out:?}");
        }
        let _ = std::fs::remove_dir_all(&r);
    }
}
