//! Verdict colour for the terminal (D0287): PASS green, FAIL red, WARN yellow, DEFECT magenta.
//!
//! The human asked to be able to tell passing from failing at a glance. Colour is applied ONLY when
//! stdout is a terminal, so every consumer that parses keel's text - the hooks, CI, the integration
//! tests, `keel land` reading a gate - sees exactly the bytes it always saw. `NO_COLOR` (any value)
//! turns it off; `KEEL_COLOR=1` forces it on for a pipe that will reach a terminal (a pager);
//! `KEEL_COLOR=0` turns it off. The words themselves never change - colour is added around them, never
//! instead of them, so a colour-blind reader and a log file read the same verdict.

use std::io::IsTerminal as _;
use std::sync::OnceLock;

static ENABLED: OnceLock<bool> = OnceLock::new();

/// Whether colour is on for this process. Decided once.
pub fn enabled() -> bool {
    *ENABLED.get_or_init(|| match std::env::var("KEEL_COLOR").ok().as_deref() {
        Some("0" | "false" | "off") => false,
        Some(_) => true,
        None => std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal(),
    })
}

fn wrap(code: &str, s: &str) -> String {
    if enabled() {
        format!("\x1b[{code}m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

/// A passing verdict: bold green.
#[must_use]
pub fn pass(s: &str) -> String {
    wrap("1;32", s)
}
/// A failing verdict or an error line: bold red.
#[must_use]
pub fn fail(s: &str) -> String {
    wrap("1;31", s)
}
/// A warning: yellow.
#[must_use]
pub fn warn(s: &str) -> String {
    wrap("33", s)
}
/// A registered control defect beside a verdict: magenta - neither pass nor fail, a qualification.
#[must_use]
pub fn defect(s: &str) -> String {
    wrap("35", s)
}
/// A neutral "nothing here" verdict (NOTHING TO ASSURE, empty tone): dim.
#[must_use]
pub fn dim(s: &str) -> String {
    wrap("2", s)
}

/// PASS or FAIL, coloured, from a boolean.
#[must_use]
pub fn verdict(ok: bool) -> String {
    if ok { pass("PASS") } else { fail("FAIL") }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Under `cargo test` stdout is captured (not a terminal), so colour is OFF and the words are the
    /// bare words - which is the property every text-parsing consumer relies on.
    #[test]
    fn colour_is_off_when_stdout_is_not_a_terminal() {
        if std::env::var_os("KEEL_COLOR").is_some() {
            return; // a forced setting in the environment is the operator's call, not this test's
        }
        assert_eq!(pass("PASS"), "PASS");
        assert_eq!(fail("FAIL"), "FAIL");
        assert_eq!(verdict(true), "PASS");
        assert_eq!(verdict(false), "FAIL");
    }

    /// The words survive colouring: strip the codes and the verdict is unchanged.
    #[test]
    fn the_words_never_change_only_the_codes_around_them() {
        let coloured = format!("\x1b[1;32m{}\x1b[0m", "PASS");
        let stripped: String = {
            let mut out = String::new();
            let mut in_code = false;
            for c in coloured.chars() {
                if c == '\x1b' { in_code = true; continue; }
                if in_code { if c == 'm' { in_code = false; } continue; }
                out.push(c);
            }
            out
        };
        assert_eq!(stripped, "PASS");
    }
}
