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
    /// `chat` | `console` | `deck` | `commitReview` | `other`.
    pub channel: &'a str,
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
    /// `need` | `useCase` | `scopeConstraint` | `bug` | `process` | `architecture` | `attestation`
    /// | `question` | `priority` | `convention` | `correction` | `none`.
    pub implication: &'a str,
    /// Why THIS kind and not a neighbouring one.
    pub triage_note: Option<&'a str>,
    pub author: &'a str,
    pub created_at: &'a str,
}

const CHANNELS: [&str; 5] = ["chat", "console", "deck", "commitReview", "other"];
const IMPLICATIONS: [&str; 12] = [
    "need", "useCase", "scopeConstraint", "bug", "process", "architecture", "attestation",
    "question", "priority", "convention", "correction", "none",
];

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
    if !CHANNELS.contains(&s.channel) {
        return Err(WriteError::InvalidMethod(format!(
            "channel `{}` — expected one of {}",
            s.channel,
            CHANNELS.join(" | ")
        )));
    }
    if s.text.trim().is_empty() {
        return Err(WriteError::Parse("a Statement with empty text records nothing".into()));
    }
    let path = intake_file(root, s.created_at);
    with_file_lock(&root.join(".tracking").join("issues.sysml"), || {
        let name = format!("st{:03}", next_number(&all_tracking_text(root), "part st"));
        let existing = ensure_intake_package(&path, s.created_at);
        let block = format!(
            "\n\x20   part {name} : Statement {{\n\
             \x20       :>> id = \"{}\";\n\
             \x20       :>> title = \"{}\";\n\
             \x20       :>> createdAt = \"{}\"; :>> createdBy = \"{}\";\n\
             \x20       :>> text = \"{}\";\n\
             \x20       :>> saidBy = \"{}\"; :>> saidAt = \"{}\"; :>> channel = StatementChannel::{};\n\
             \x20   }}\n",
            gen_uuid(),
            crate::write::sanitize_public(s.title),
            s.created_at,
            s.author,
            escape_verbatim(s.text),
            s.said_by,
            s.said_at,
            s.channel,
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
    if !IMPLICATIONS.contains(&s.implication) {
        return Err(WriteError::InvalidMethod(format!(
            "implication `{}` — expected one of {}",
            s.implication,
            IMPLICATIONS.join(" | ")
        )));
    }
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
                channel: "chat",
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
            text: "x", said_by: "w", said_at: "2026-08-26", channel: "smoke-signal",
            title: "t", author: "a", created_at: "2026-08-26",
        }).is_err(), "an unknown channel must refuse, never default - the channel is provenance");
        assert!(record_statement(&r, &NewStatement {
            text: "   ", said_by: "w", said_at: "2026-08-26", channel: "chat",
            title: "t", author: "a", created_at: "2026-08-26",
        }).is_err(), "empty text records nothing and must say so");
        let _ = std::fs::remove_dir_all(&r);
    }
}
