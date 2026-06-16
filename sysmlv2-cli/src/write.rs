//! Write API — append `TestResult`s and add tasks to tracking files.
//!
//! Enforces the three write-policy invariants:
//! - **ids**: every new record gets an auto-generated UUID v4.
//! - **append-only**: `append_result` always produces the next R{N}, never
//!   overwrites an existing result.
//! - **writePolicy**: `append_result` requires the task to exist; `add_task`
//!   rejects duplicate task names.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use sysmlv2_parser::ast::{Item, Package};

// ── error type ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum WriteError {
    Io(std::io::Error),
    Parse(String),
    /// Named task not found in the file.
    TaskNotFound(String),
    /// Task already exists — `add_task` would create a duplicate.
    TaskAlreadyExists(String),
    /// Verdict string was not "pass" or "fail".
    InvalidVerdict(String),
    /// Method string was not a known `VerificationMethod` variant.
    InvalidMethod(String),
    /// Named action def not found in the file.
    ActionDefNotFound(String),
    /// Cannot find a `DoD` verification or existing result line for the task.
    InsertionPointNotFound(String),
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Parse(s) => write!(f, "parse error: {s}"),
            Self::TaskNotFound(n) => write!(f, "task not found: {n}"),
            Self::TaskAlreadyExists(n) => write!(f, "task already exists: {n}"),
            Self::InvalidVerdict(v) => write!(f, "invalid verdict '{v}' (expected 'pass' or 'fail')"),
            Self::InvalidMethod(m) => write!(f, "invalid method '{m}'"),
            Self::ActionDefNotFound(n) => write!(f, "action def not found: {n}"),
            Self::InsertionPointNotFound(n) => write!(f, "cannot find insertion point for task: {n}"),
        }
    }
}

impl From<std::io::Error> for WriteError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// ── UUID generation ───────────────────────────────────────────────────────────

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a UUID v4 from `SystemTime` + monotone counter + process ID.
///
/// Not cryptographically random — suitable for dev-tooling identity only.
#[allow(clippy::cast_possible_truncation)]
pub fn gen_uuid() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let nanos = u64::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos(),
    );
    let c = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = u64::from(std::process::id());

    // Mix into two 64-bit halves with distinct multipliers.
    let hi = secs
        .wrapping_add(pid.wrapping_mul(0x9e37_79b9_7f4a_7c15))
        .wrapping_add(c.wrapping_mul(0x6c62_272e_07bb_0142));
    let lo = nanos
        .wrapping_add(c.wrapping_mul(0x517c_c1b7_2722_0a95))
        .wrapping_add(secs.wrapping_mul(0xb492_b66f_be98_f273));

    // Intentional truncations: extracting 16/32-bit UUID fields from 64-bit halves.
    let p1 = (hi >> 32) as u32;
    let p2 = (hi >> 16) as u16;
    let p3 = ((hi & 0x0fff) | 0x4000) as u16; // version 4
    let p4 = ((lo >> 48 & 0x3fff) | 0x8000) as u16; // variant RFC 4122
    let p5 = lo & 0x0000_ffff_ffff_ffff;

    format!("{p1:08x}-{p2:04x}-{p3:04x}-{p4:04x}-{p5:012x}")
}

// ── AST helpers ───────────────────────────────────────────────────────────────

fn task_exists_in_pkg(pkg: &Package, name: &str) -> bool {
    for item in &pkg.items {
        match item {
            Item::ActionDecl(a) if a.name == name => return true,
            Item::ActionDef(def) => {
                if def.actions.iter().any(|a| a.name == name) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn action_def_exists(pkg: &Package, name: &str) -> bool {
    pkg.items.iter().any(|item| matches!(item, Item::ActionDef(d) if d.name == name))
}

/// Return the highest existing result sequence number for `task_name`.
/// Checks both `{task}DoDR{n}` (canonical) and `{task}R{n}` (legacy) naming.
fn max_result_n(pkg: &Package, task_name: &str) -> u32 {
    let dodr = format!("{task_name}DoDR");
    let r_pfx = format!("{task_name}R");
    let mut max_n = 0u32;

    let scan_part_name = |name: &str, max: &mut u32| {
        let n = name.strip_prefix(&dodr)
            .or_else(|| name.strip_prefix(&r_pfx))
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        if n > *max {
            *max = n;
        }
    };

    for item in &pkg.items {
        match item {
            Item::Part(p) => scan_part_name(&p.name, &mut max_n),
            Item::ActionDef(def) => {
                for p in &def.parts {
                    scan_part_name(&p.name, &mut max_n);
                }
            }
            _ => {}
        }
    }
    max_n
}

// ── text insertion helpers ────────────────────────────────────────────────────

/// Return true if `trimmed` is a `TestResult` line for `task_name` (`DoDR` or R form).
fn is_result_line_for(trimmed: &str, dodr_pfx: &str, r_pfx: &str) -> bool {
    let check = |rest: &str| -> bool {
        rest.split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches(':')
            .parse::<u32>()
            .is_ok()
    };
    if let Some(rest) = trimmed.strip_prefix(dodr_pfx) {
        return check(rest);
    }
    if let Some(rest) = trimmed.strip_prefix(r_pfx) {
        return check(rest);
    }
    false
}

/// Return the 0-indexed line number after which to insert a new `TestResult`.
///
/// Prefers the last existing result line; falls back to the `DoD` verification
/// line if no result exists yet.
fn find_result_insertion(lines: &[&str], task_name: &str) -> Result<usize, WriteError> {
    let dodr_pfx = format!("part {task_name}DoDR");
    let r_pfx = format!("part {task_name}R");
    let dod_pat = format!("verification {task_name}DoD");

    let mut last_result = None;
    let mut dod_line = None;

    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if is_result_line_for(t, &dodr_pfx, &r_pfx) {
            last_result = Some(i);
        }
        if t.starts_with(&dod_pat) {
            dod_line = Some(i);
        }
    }

    last_result
        .or(dod_line)
        .ok_or_else(|| WriteError::InsertionPointNotFound(task_name.to_owned()))
}

/// Return the 0-indexed line number of the closing `}` for an action def.
///
/// Scans forward from `def_start_line`, tracking brace depth.
fn find_action_def_close(lines: &[&str], def_start_line: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, line) in lines.iter().enumerate().skip(def_start_line) {
        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
    }
    None
}

/// Detect the indentation prefix used for existing `action` lines inside an
/// action def — so new lines match the file's style.
fn detect_indent(lines: &[&str], def_start: usize, def_close: usize) -> String {
    for line in &lines[def_start + 1..def_close] {
        let trimmed = line.trim();
        if trimmed.starts_with("action ") {
            let indent_len = line.len() - line.trim_start().len();
            return " ".repeat(indent_len);
        }
    }
    "        ".to_owned() // fallback: 8 spaces
}

// ── parse helper ─────────────────────────────────────────────────────────────

fn parse_file(path: &Path) -> Result<Package, WriteError> {
    let src = std::fs::read_to_string(path)?;
    let fname = path.to_string_lossy();
    let tokens = sysmlv2_parser::tokenize(&src, &fname)
        .map_err(|e| WriteError::Parse(e.to_string()))?;
    sysmlv2_parser::parse(tokens, &fname)
        .map_err(|e| WriteError::Parse(e.to_string()))
}

// ── public API ────────────────────────────────────────────────────────────────

/// Append a `part <task>DoDR<N+1> : TestResult { ... }` to `path`.
///
/// Enforces:
/// - Task must exist in the parsed file (else `TaskNotFound`).
/// - `verdict` must be `"pass"` or `"fail"` (else `InvalidVerdict`).
/// - The new result index is `(max existing N) + 1` — never overwrites.
/// - A fresh UUID is auto-generated.
///
/// Returns the UUID of the newly created record.
///
/// # Errors
/// Returns `WriteError::InvalidVerdict` if `verdict` is not `"pass"` or `"fail"`.
/// Returns `WriteError::TaskNotFound` if `task_name` does not exist in the file.
/// Returns `WriteError::InsertionPointNotFound` if no `DoD` verification is found.
/// Returns `WriteError::Parse` if the file cannot be lexed or parsed.
/// Returns `WriteError::Io` on filesystem errors.
pub fn append_result(
    path: &Path,
    task_name: &str,
    sha: &str,
    verdict: &str,
    judged_at: &str,
    judged_by: &str,
) -> Result<String, WriteError> {
    if verdict != "pass" && verdict != "fail" {
        return Err(WriteError::InvalidVerdict(verdict.to_owned()));
    }

    let pkg = parse_file(path)?;

    if !task_exists_in_pkg(&pkg, task_name) {
        return Err(WriteError::TaskNotFound(task_name.to_owned()));
    }

    let n = max_result_n(&pkg, task_name) + 1;
    let uuid = gen_uuid();

    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let insert_after = find_result_insertion(&lines, task_name)?;

    // Detect indentation from surrounding context.
    let indent = {
        let context_line = lines[insert_after].trim_start();
        let indent_len = lines[insert_after].len() - context_line.len();
        " ".repeat(indent_len)
    };

    let new_line = format!(
        "{indent}part {task_name}DoDR{n} : TestResult {{ :>> id = \"{uuid}\"; :>> outcome = VerdictKind::{verdict}; :>> judgedAgainst = \"{sha}\"; :>> judgedAt = \"{judged_at}\"; :>> judgedBy = \"{judged_by}\"; }}"
    );

    let mut new_content = String::with_capacity(content.len() + new_line.len() + 1);
    for (i, line) in lines.iter().enumerate() {
        new_content.push_str(line);
        new_content.push('\n');
        if i == insert_after {
            new_content.push_str(&new_line);
            new_content.push('\n');
        }
    }

    std::fs::write(path, new_content)?;
    Ok(uuid)
}

/// Add a new `action` + `verification <task>DoD : Test` to an action def in `path`.
///
/// Enforces:
/// - The named action def must exist (else `ActionDefNotFound`).
/// - Task name must not already exist (else `TaskAlreadyExists`).
/// - `method` must be a known `VerificationMethod` variant (else `InvalidMethod`).
/// - A fresh UUID is auto-generated for the verification.
///
/// Returns the UUID of the newly created `verification`.
///
/// # Errors
/// Returns `WriteError::InvalidMethod` if `method` is not a known variant.
/// Returns `WriteError::ActionDefNotFound` if `def_name` is not in the file.
/// Returns `WriteError::TaskAlreadyExists` if `task_name` already exists in the file.
/// Returns `WriteError::Parse` if the file cannot be lexed or parsed.
/// Returns `WriteError::Io` on filesystem errors.
pub fn add_task(
    path: &Path,
    def_name: &str,
    task_name: &str,
    dod_text: &str,
    method: &str,
) -> Result<String, WriteError> {
    const VALID_METHODS: &[&str] = &["test", "inspect", "confirmation", "demo", "analysis"];
    if !VALID_METHODS.contains(&method) {
        return Err(WriteError::InvalidMethod(method.to_owned()));
    }

    let pkg = parse_file(path)?;

    if !action_def_exists(&pkg, def_name) {
        return Err(WriteError::ActionDefNotFound(def_name.to_owned()));
    }

    if task_exists_in_pkg(&pkg, task_name) {
        return Err(WriteError::TaskAlreadyExists(task_name.to_owned()));
    }

    let uuid = gen_uuid();

    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Find the action def start line.
    let def_start = lines
        .iter()
        .position(|l| {
            let t = l.trim();
            t == format!("action def {def_name} {{")
                || t.starts_with(&format!("action def {def_name} {{"))
                || t.starts_with(&format!("action def {def_name}"))
        })
        .ok_or_else(|| WriteError::ActionDefNotFound(def_name.to_owned()))?;

    let def_close = find_action_def_close(&lines, def_start)
        .ok_or_else(|| WriteError::ActionDefNotFound(def_name.to_owned()))?;

    let indent = detect_indent(&lines, def_start, def_close);

    let action_line = format!("{indent}action {task_name};");
    let dod_line = format!(
        "{indent}verification {task_name}DoD : Test {{ :>> id = \"{uuid}\"; :>> method = VerificationMethod::{method}; :>> procedureText = \"{dod_text}\"; }}"
    );

    // Insert both lines before the closing `}` (i.e., after def_close - 1).
    let insert_after = def_close - 1;

    let mut new_content = String::with_capacity(content.len() + action_line.len() + dod_line.len() + 4);
    for (i, line) in lines.iter().enumerate() {
        new_content.push_str(line);
        new_content.push('\n');
        if i == insert_after {
            new_content.push_str(&action_line);
            new_content.push('\n');
            new_content.push_str(&dod_line);
            new_content.push('\n');
        }
    }

    std::fs::write(path, new_content)?;
    Ok(uuid)
}
