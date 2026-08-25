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

// ── the channel's deterministic text logic (D0221) ─────────────────────────────────────────────
//
// WHY IT IS HERE AND NOT IN A SCRIPT. `.github/scripts/record_decision.sh` parsed the human's
// gesture with a shell `case` and a locale-dependent `tr`, re-grepped the login table the binary
// already owns (two implementations of one rule - exactly how the workflow gate and the recorder
// drifted apart, D0219), and was unit-tested by production. A bespoke script is also invisible to
// every keel surface: no guard reads it and its behaviour cannot be re-derived from the tree.
//
// The split is by KIND: deterministic text logic here, `gh` effects in a thin workflow step, and
// what a gesture MEANS in the process definition. The binary performs NO network I/O.

/// What the human's comment asked for.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Accept, optionally choosing an option letter on a fork.
    Accept(Option<char>),
    /// Reject, carrying their reason verbatim.
    Reject(String),
    /// Nothing recognisable — the caller must say so on the thread and record NOTHING.
    Unparsed,
}

/// THE GESTURE VOCABULARY, in one place (D0221).
///
/// Reads only the FIRST line, trimmed, and is deliberately small:
///   * a single letter        -> accept, choosing that option (`B`)
///   * `accept` / `/accept`   -> accept with no option
///   * `accept B`             -> accept, choosing B
///   * `reject <why>`         -> reject, carrying the reason
///
/// Case-insensitive on the keyword, ASCII-only on the option letter. Anything else is `Unparsed`:
/// the channel records NOTHING it did not understand, because a misread gesture becomes a
/// fabricated human attestation (§4/D0106), which is the one failure this path must never have.
#[must_use]
pub fn parse_gesture(body: &str) -> Verdict {
    let first = body.lines().next().unwrap_or("").trim_end_matches('\r').trim();
    if first.is_empty() {
        return Verdict::Unparsed;
    }
    // A bare single ASCII letter is an option choice.
    let mut chars = first.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if c.is_ascii_alphabetic() {
            return Verdict::Accept(Some(c.to_ascii_uppercase()));
        }
    }
    let lower = first.to_ascii_lowercase();
    let word = lower.trim_start_matches('/');
    if let Some(rest) = word.strip_prefix("reject") {
        // `reject` alone is still a reject; the reason may be empty and that is the human's choice.
        let reason = first
            .trim_start_matches('/')
            .get("reject".len()..)
            .unwrap_or("")
            .trim()
            .to_string();
        if rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == ':') {
            return Verdict::Reject(reason);
        }
    }
    if let Some(rest) = word.strip_prefix("accept") {
        let rest = rest.trim();
        if rest.is_empty() {
            return Verdict::Accept(None);
        }
        let mut rc = rest.chars();
        if let (Some(c), None) = (rc.next(), rc.next()) {
            if c.is_ascii_alphabetic() {
                return Verdict::Accept(Some(c.to_ascii_uppercase()));
            }
        }
    }
    Verdict::Unparsed
}

/// Which decision an issue is about, from the `keel-decision: <name>` marker its body embeds.
///
/// The marker is what makes the thread addressable, and it is written by the opener — so reading it
/// back is the inverse of one authored fact, not a guess about the title.
#[must_use]
pub fn decision_of(issue_body: &str) -> Option<String> {
    for tok in issue_body.split_whitespace() {
        if let Some(rest) = tok.strip_prefix("keel-decision:") {
            if !rest.is_empty() {
                return Some(rest.trim_end_matches("-->").trim().to_string());
            }
        }
    }
    // `keel-decision: dNNNN` with a space after the colon.
    let mut it = issue_body.split_whitespace().peekable();
    while let Some(tok) = it.next() {
        if tok == "keel-decision:" {
            if let Some(next) = it.peek() {
                let v = next.trim_end_matches("-->").trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }
    None
}

/// The decision a GESTURE names, when the gesture is left on a shared thread (D0227/issue258).
///
/// The channel used to open one issue per Decision, so the decision was always the issue's. That
/// barraged the repo owner — 20 issues in a day, one per auto-accepted non-fork — and would have
/// barraged a colleague on any project that adopted the channel, which is exactly what the author
/// prohibited. Non-forks now share ONE standing override thread, so a reject has to say WHICH
/// decision it reverses: `reject d0225 too wide`.
///
/// Deliberately strict: `d` followed by exactly four digits, and nothing else attached. A loose
/// match would let a reason containing a version or an id silently retarget the reject to another
/// Decision — reversing something the human never mentioned, which is worse than not parsing.
#[must_use]
pub fn decision_in_gesture(comment_body: &str) -> Option<String> {
    let first = comment_body.lines().next().unwrap_or("");
    first
        .split(|c: char| !c.is_ascii_alphanumeric())
        .find(|tok| {
            tok.len() == 5
                && tok.starts_with('d')
                && tok[1..].chars().all(|c| c.is_ascii_digit())
        })
        .map(str::to_string)
}

/// Has this comment id already been receipted? Idempotency, decided from the comment bodies the
/// caller already fetched — so the check is deterministic here and the fetch stays in the workflow.
#[must_use]
pub fn already_receipted(comment_bodies: &str, comment_id: &str) -> bool {
    comment_bodies.contains(&format!("receipt-for-comment: {comment_id}"))
}

/// Escape a string for embedding in the JSON this command prints. The human's reason is arbitrary
/// text, so it is escaped rather than trusted — a quote in a rejection reason must not produce
/// malformed JSON that a workflow then mis-parses.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

/// `keel github-gesture` — parse the channel's inputs and print what they MEAN, as JSON.
///
/// Inputs arrive by ENV, never argv: `COMMENT_BODY`, and optionally `ISSUE_BODY` and `COMMENT_ID`
/// plus `COMMENT_BODIES`. The comment body is attacker-influenced text, so it is never interpolated
/// into a shell command anywhere in this path.
///
/// # Returns
/// 0 when the gesture parsed; 1 when it did not (the caller says so on the thread and records
/// nothing); 2 on a usage error.
#[must_use]
pub fn gesture_cmd() -> i32 {
    let Ok(body) = std::env::var("COMMENT_BODY") else {
        eprintln!("usage: COMMENT_BODY=<text> keel github-gesture [ISSUE_BODY=.. COMMENT_ID=.. COMMENT_BODIES=..]");
        eprintln!("  Inputs arrive by ENV, never argv: a comment body is attacker-influenced text.");
        return 2;
    };
    let issue_body = std::env::var("ISSUE_BODY").unwrap_or_default();
    let comment_id = std::env::var("COMMENT_ID").unwrap_or_default();
    let bodies = std::env::var("COMMENT_BODIES").unwrap_or_default();
    // A gesture on the SHARED standing thread names its own decision; one on a fork's own issue
    // inherits it from the issue body. The comment wins when it names one, because it is the more
    // specific statement of intent - and on the standing thread the issue body names none at all.
    let decision = decision_in_gesture(&body)
        .or_else(|| decision_of(&issue_body))
        .unwrap_or_default();
    let receipted = !comment_id.is_empty() && already_receipted(&bodies, &comment_id);
    let (verdict, option, reason) = match parse_gesture(&body) {
        Verdict::Accept(opt) => ("accept", opt.map(String::from).unwrap_or_default(), String::new()),
        Verdict::Reject(why) => ("reject", String::new(), why),
        Verdict::Unparsed => ("unparsed", String::new(), String::new()),
    };
    println!(
        "{{\"verdict\": \"{verdict}\", \"option\": \"{option}\", \"reason\": \"{}\", \"decision\": \"{decision}\", \"alreadyReceipted\": {receipted}}}",
        json_escape(&reason)
    );
    i32::from(verdict == "unparsed")
}

#[cfg(test)]
mod gesture_tests {
    use super::{already_receipted, decision_in_gesture, decision_of, parse_gesture, Verdict};

    #[test]
    fn a_gesture_on_the_shared_thread_names_its_own_decision() {
        // D0227: non-forks share one standing override thread, so a reject must say which decision
        // it reverses. The issue body names none, so the comment is the only source.
        assert_eq!(decision_in_gesture("reject d0225 too wide"), Some("d0225".to_string()));
        assert_eq!(decision_in_gesture("d0225 reject"), Some("d0225".to_string()));
        assert_eq!(decision_in_gesture("reject D0225 too wide"), None, "the token is lower-case `d`");
        // Only the first line, same as the verdict - a quoted mail trailer cannot retarget a reject.
        assert_eq!(decision_in_gesture("reject too wide

see d0225"), None);
        // STRICT: anything that is not exactly d + four digits must not match, or a reason mentioning
        // a version or an id would silently reverse a Decision the human never named.
        for miss in ["reject too wide", "reject d022 typo", "reject d02255", "reject xd0225", "reject d0225x"] {
            assert_eq!(decision_in_gesture(miss), None, "must not match: {miss}");
        }
    }

    #[test]
    fn the_documented_vocabulary_parses() {
        assert_eq!(parse_gesture("B"), Verdict::Accept(Some('B')));
        assert_eq!(parse_gesture("b"), Verdict::Accept(Some('B')), "case-insensitive on the letter");
        assert_eq!(parse_gesture("accept"), Verdict::Accept(None));
        assert_eq!(parse_gesture("/accept"), Verdict::Accept(None));
        assert_eq!(parse_gesture("Accept C"), Verdict::Accept(Some('C')));
        assert_eq!(parse_gesture("reject too wide"), Verdict::Reject("too wide".to_string()));
        assert_eq!(parse_gesture("reject"), Verdict::Reject(String::new()));
        // Only the FIRST line counts, so a signature or quoted mail trailer cannot change the verdict.
        assert_eq!(parse_gesture("A\n\nsent from my phone"), Verdict::Accept(Some('A')));
        assert_eq!(parse_gesture("accept\r\ntrailing"), Verdict::Accept(None));
    }

    /// THE CONTROL: anything unrecognised records NOTHING. A misread gesture becomes a fabricated
    /// human attestation (D0106), which is the one failure this path must never have.
    #[test]
    fn anything_unrecognised_is_unparsed_and_never_an_accept() {
        for body in [
            "",
            "   ",
            "looks good to me",
            "yes",
            "lgtm",
            "acceptable",         // not `accept`
            "rejecting this",     // not `reject`
            "12",
            "?",
            "accept BB",          // an option is ONE letter
            "thanks!",
        ] {
            assert_eq!(parse_gesture(body), Verdict::Unparsed, "must not read a verdict from {body:?}");
        }
    }

    #[test]
    fn the_decision_marker_is_read_back_from_the_issue_body() {
        let body = "<!-- keel-decision: d0219 -->\n\n**Why it came up:** ...";
        assert_eq!(decision_of(body).as_deref(), Some("d0219"));
        assert_eq!(decision_of("no marker here"), None);
    }

    #[test]
    fn idempotency_is_decided_from_the_comment_bodies() {
        let bodies = "recorded\nreceipt-for-comment: 12345\nthanks";
        assert!(already_receipted(bodies, "12345"));
        assert!(!already_receipted(bodies, "999"));
    }
}
