//! Host/shell adaptation advisories for Bash tool calls (issue094, D0134).
//!
//! CLAUDE.md §6 names host/shell adaptation the #1 avoidable-friction class, and it kept recurring
//! anyway — three times in one sitting (issue065's class, reopened as issue094), then twice more in
//! the sprints that followed. Sprint 263's retro concluded the class was not automatable because it
//! "lives in throwaway shell commands, not committed artifacts". That reasoning assumed a control
//! must inspect ARTIFACTS. D0134's in-loop gates inspect TOOL CALLS, and a shell command is a tool
//! call — so the premise was wrong, not the conclusion unreachable.
//!
//! ADVISORY, NEVER BLOCKING, and that is a design commitment rather than caution. These checks
//! cannot be exact: whether `/c/Users/x` is wrong depends on what reads it, and no string predicate
//! settles that. A heuristic that BLOCKS shell commands is the issue076/issue081 dynamic where an
//! over-strict gate trains its actor to disable it, and this project has already paid for that once
//! with eight bypassed commits. So this prints a remedy and returns success, always.
//!
//! It must also be SILENT on the common case. A hook that comments on ordinary commands becomes
//! noise the reader learns to skip, at which point it is worse than absent — it looks like coverage.

/// Programs that are Windows-native on this host and therefore do NOT understand an MSYS path.
///
/// `git` is deliberately absent: git-for-Windows is MSYS-aware and accepts `/c/...` happily, so
/// flagging it would be a false positive on the single most common command in this repo.
const WINDOWS_NATIVE: &[&str] = &[
    "python", "python3", "python.exe", "py", "conda", "conda.exe", "java", "java.exe", "javac",
    "cargo", "cargo.exe", "rustc", "node", "npm", "npx", "dotnet", "msbuild", "pwsh", "powershell",
    "keel.exe", "jupyter", "pip", "pip.exe",
];

/// One advisory: what was spotted and the concrete fix.
pub struct Advisory {
    pub what: String,
    pub fix: String,
}

/// Does `tok` look like an MSYS/POSIX absolute path that a Windows program cannot open?
///
/// `/c/...` and `/mnt/...` are the drive forms; `/tmp/...` is the one that bites hardest, because it
/// exists inside the bash tool and does not exist for a Windows process at all.
fn is_msys_path(tok: &str) -> bool {
    let t = tok.trim_matches(|c| c == '"' || c == '\'');
    if t.starts_with("/tmp/") || t.starts_with("/mnt/") {
        return true;
    }
    // `/c/...`, `/d/...` — a single letter between slashes.
    let b = t.as_bytes();
    b.len() > 3
        && b.first() == Some(&b'/')
        && b.get(1).is_some_and(u8::is_ascii_alphabetic)
        && b.get(2) == Some(&b'/')
}

/// The program a command word invokes, stripped of any directory part.
fn program_of(word: &str) -> &str {
    let w = word.trim_matches(|c| c == '"' || c == '\'');
    w.rsplit(['/', '\\']).next().unwrap_or(w)
}

/// Split on the operators that start a new command, so `a && b` is two commands rather than one.
fn segments(cmd: &str) -> Vec<&str> {
    cmd.split("&&").flat_map(|s| s.split("||")).flat_map(|s| s.split(';')).flat_map(|s| s.split('|')).collect()
}

/// Inspect one Bash tool-call command string. Empty result means nothing to say.
#[must_use]
pub fn inspect(command: &str) -> Vec<Advisory> {
    let mut out: Vec<Advisory> = Vec::new();

    // (a) an MSYS path handed to a Windows-native program.
    for seg in segments(command) {
        let mut words = seg.split_whitespace().filter(|w| !w.is_empty());
        // Skip leading `VAR=value` assignments and `env`, which precede the real program.
        let mut prog = None;
        for w in words.by_ref() {
            if w.contains('=') && !w.starts_with('-') {
                continue;
            }
            prog = Some(program_of(w));
            break;
        }
        let Some(prog) = prog else { continue };
        if !WINDOWS_NATIVE.contains(&prog) {
            continue;
        }
        for arg in words {
            if is_msys_path(arg) {
                out.push(Advisory {
                    what: format!("`{prog}` is Windows-native and cannot open the MSYS path `{arg}`"),
                    fix: format!("convert it: `$(cygpath -w '{arg}')`, or pass a Windows path with forward slashes (C:/...). /tmp/ in particular does not exist for a Windows process."),
                });
                break; // one advisory per segment is enough to make the point
            }
        }
    }

    // (b) PowerShell-only syntax inside a bash command.
    if let Some(hit) = powershell_call_operator(command) {
        out.push(Advisory {
            what: format!("`{hit}` is the PowerShell call operator — bash reads a leading `&` as a syntax error, not as \"run this\""),
            fix: "drop the `&` (bash runs a quoted path directly), or use the PowerShell tool for this command.".to_string(),
        });
    }
    if command.contains("$env:") {
        out.push(Advisory {
            what: "`$env:NAME` is PowerShell variable syntax".to_string(),
            fix: "bash uses `$NAME` / `export NAME=...`.".to_string(),
        });
    }
    if command.contains("-ErrorAction") || command.contains("Get-ChildItem") || command.contains("Select-Object") {
        out.push(Advisory {
            what: "a PowerShell cmdlet appears in a bash command".to_string(),
            fix: "use the PowerShell tool, or the POSIX equivalent.".to_string(),
        });
    }
    out
}

/// A leading `&` used as PowerShell's call operator, e.g. `& "C:\path\prog.exe" args`.
///
/// Deliberately narrow. In bash `&` is backgrounding, `&&` is an operator and `2>&1` is a redirect,
/// all of them correct and common — so this matches only `&` at the start of a command position and
/// followed by whitespace then a quote or a drive letter, which is the call-operator shape and
/// nothing else.
fn powershell_call_operator(cmd: &str) -> Option<String> {
    let bytes = cmd.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c != b'&' {
            continue;
        }
        // Not part of `&&`, and not preceded by `>` (a redirect like `2>&1`).
        let prev = if i > 0 { bytes.get(i - 1).copied() } else { None };
        if bytes.get(i + 1) == Some(&b'&') || prev == Some(b'&') {
            continue;
        }
        if matches!(prev, Some(b'>' | b'\\')) {
            continue;
        }
        // Must be in command position: start of string, or after a newline / `;` / `(` .
        let in_cmd_pos = cmd[..i].trim_end().is_empty()
            || cmd[..i].trim_end().ends_with(';')
            || cmd[..i].trim_end().ends_with('(')
            || cmd[..i].ends_with('\n');
        if !in_cmd_pos {
            continue;
        }
        let rest = cmd[i + 1..].trim_start();
        if rest.is_empty() {
            continue;
        }
        let looks_like_path = rest.starts_with('"')
            || rest.starts_with('\'')
            || (rest.len() > 2
                && rest.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
                && rest.as_bytes().get(1) == Some(&b':'));
        if looks_like_path {
            let snippet: String = rest.chars().take(40).collect();
            return Some(format!("& {snippet}"));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::inspect;

    /// The real commands from the sitting that produced issue094, plus the two that recurred in
    /// sprints 281 and 283. A control for a recurring defect that cannot detect the actual
    /// occurrences is a control in name only.
    #[test]
    fn the_real_failures_are_detected_with_the_right_remedy() {
        let a = inspect("python .engine/tools/validate/validate_schema.py /c/Users/Bill/out.json");
        assert_eq!(a.len(), 1, "MSYS path handed to Windows python");
        assert!(a[0].fix.contains("cygpath"), "{}", a[0].fix);

        let b = inspect("conda run -n sysml python x.py --json /tmp/conf.json");
        assert_eq!(b.len(), 1, "/tmp/ does not exist for a Windows process");

        let c = inspect("& \"C:\\Users\\Bill\\miniforge3\\Scripts\\conda.exe\" run -n sysml python x.py");
        assert_eq!(c.len(), 1, "PowerShell call operator inside a bash command");
        assert!(c[0].fix.contains("PowerShell tool"), "{}", c[0].fix);
    }

    /// Silence on the common case is the whole design. A hook that comments on ordinary commands
    /// becomes noise the reader skips, at which point it is worse than absent.
    #[test]
    fn ordinary_commands_are_not_flagged() {
        for ok in [
            "cargo build --release",
            "./target/release/keel.exe guard .",
            "git add -A && git commit -F msg.txt",
            "grep -rn foo .tracking/ | head -5",
            "python -c \"import json; print(1)\"",
            // git IS MSYS-aware, so an MSYS path here is correct and must not be flagged.
            "git -C /c/Users/Bill/repo status --porcelain",
            // A genuine POSIX path to a POSIX tool.
            "cat /tmp/out.txt",
            // Backgrounding and redirects both contain `&` and are correct bash.
            "./server --port 8080 > log 2>&1 &",
            // A Windows path passed to a Windows program is right, not wrong.
            "python x.py C:/Users/Bill/out.json",
        ] {
            assert!(inspect(ok).is_empty(), "false positive on: {ok}");
        }
    }

    #[test]
    fn powershell_variable_and_cmdlet_syntax_is_named() {
        assert_eq!(inspect("echo $env:PATH").len(), 1);
        assert_eq!(inspect("Get-ChildItem -Recurse").len(), 1);
        // `&&` must never be read as the call operator.
        assert!(inspect("make && make test").is_empty());
    }
}
