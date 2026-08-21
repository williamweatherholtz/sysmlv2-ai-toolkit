//! Write API — append `TestResult`s and add tasks to tracking files.
//!
//! Enforces the three write-policy invariants:
//! - **ids**: every new record gets an auto-generated UUID v4.
//! - **append-only**: `append_result` always produces the next R{N}, never
//!   overwrites an existing result.
//! - **writePolicy**: `append_result` requires the task to exist; `add_task`
//!   rejects duplicate task names.

use std::fmt::Write as _;
use std::path::Path;

use keel_parser::ast::{Item, Package};

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
    /// Named ceremony gate (`verification`) not found in the file.
    GateNotFound(String),
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
            Self::GateNotFound(n) => write!(f, "gate not found: {n}"),
        }
    }
}

impl From<std::io::Error> for WriteError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// ── UUID generation ───────────────────────────────────────────────────────────

/// Generate a cryptographically-random UUID v4 (RFC 4122), 122 bits of OS entropy.
///
/// # Distributed safety (issue075 / D0129)
///
/// The previous construction mixed only clock seconds, sub-second nanos, PID and an in-process
/// counter that was 0 for the first record of every invocation. It had **no host component**, so
/// two machines were not independent sources — and `id` is the engine's identity invariant (items
/// never collide on name precisely because they are distinguished by id, CLAUDE.md §2.3), so a
/// collision corrupts identity itself. With no duplicate-id detector (issue074) it would also be
/// undetectable. Entropy now comes from the OS CSPRNG.
///
/// # Panics
///
/// If the OS CSPRNG is unavailable. That is deliberate: minting a weak identity silently is worse
/// than failing loudly (the honest-gate principle, D0098).
#[must_use]
#[allow(clippy::expect_used)] // deliberate: a weak identity minted silently is worse than a loud abort
pub fn gen_uuid() -> String {
    let mut b = [0u8; 16];
    getrandom::fill(&mut b).expect("OS CSPRNG unavailable — refusing to mint a weak identity");
    b[6] = (b[6] & 0x0f) | 0x40; // version 4
    b[8] = (b[8] & 0x3f) | 0x80; // variant RFC 4122
    let mut s = String::with_capacity(36);
    for (i, byte) in b.iter().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            s.push('-');
        }
        let _ = write!(s, "{byte:02x}");
    }
    s
}

/// The ONE lock file guarding model writes, found by walking up to the `.tracking`/`.engine` parent.
///
/// Falls back to a sibling lock when neither is found - a caller writing outside a model tree still gets
/// mutual exclusion against itself, which is the best available answer without inventing a root.
fn model_lock_path(target: &Path) -> std::path::PathBuf {
    let mut cur = target;
    while let Some(parent) = cur.parent() {
        if matches!(parent.file_name().and_then(|n| n.to_str()), Some(".tracking" | ".engine")) {
            return parent.with_file_name(".keel-write-lock");
        }
        cur = parent;
    }
    target.with_extension("keel-lock")
}

/// Hold an exclusive lock on `path` for the duration of `f` (issue185).
///
/// WHY. Every write here is a read-modify-write: read the file, splice an item in, write it all back.
/// Four concurrent `keel record issue` calls landed TWO issues - all four exited 0, the tree validated
/// clean, and two recorded facts simply vanished. Whichever writer renamed last won. Making each write
/// ATOMIC (issue184) guarantees the file is never half-written and says nothing about whether it
/// contains both writers' work.
///
/// The lock is a sibling `.keel-lock` created with `create_new`, which is atomic on every platform this
/// runs on: exactly one creator succeeds. A writer that cannot acquire it FAILS LOUDLY, because a
/// refused write is recoverable - the caller retries - whereas a lost write with a success exit code is
/// undetectable by anything, and the write API is what D0093 makes the automation substrate.
///
/// A STALE LOCK IS BREAKABLE. A process that dies holding one would otherwise block every later write
/// forever, turning a crash into a permanent outage. On the last attempt a lock older than twice the
/// retry budget is taken over - a live holder finishes far inside that.
///
/// # Errors
/// Returns [`std::io::ErrorKind::WouldBlock`] if the lock cannot be acquired, or whatever `f` returns.
pub fn with_file_lock<T, E: From<std::io::Error>>(
    path: &Path,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<T, E> {
    const ATTEMPTS: u32 = 100;
    const WAIT_MS: u64 = 20;
    // ONE MODEL-WIDE LOCK, not one per file. Two entry points - `set_attr` and `create_item` - SEARCH
    // for the file they will modify, so the target is unknown until after the read and a per-file lock
    // cannot be taken up front. A single lock beside the model root makes every writer mutually
    // exclusive, which for a text-file model with millisecond writes costs nothing measurable and
    // removes the whole class rather than most of it.
    let lock = model_lock_path(path);
    let mut held = false;
    for attempt in 0..ATTEMPTS {
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&lock) {
            Ok(_) => {
                held = true;
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                if attempt + 1 == ATTEMPTS {
                    let stale = std::fs::metadata(&lock)
                        .and_then(|m| m.modified())
                        .ok()
                        .and_then(|m| m.elapsed().ok())
                        .is_some_and(|age| {
                            age.as_millis() > u128::from(ATTEMPTS) * u128::from(WAIT_MS) * 2
                        });
                    if stale {
                        let _ = std::fs::remove_file(&lock);
                        held = std::fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&lock)
                            .is_ok();
                    }
                }
                if !held {
                    std::thread::sleep(std::time::Duration::from_millis(WAIT_MS));
                }
            }
            Err(e) => return Err(E::from(e)),
        }
    }
    if !held {
        return Err(E::from(std::io::Error::new(
            std::io::ErrorKind::WouldBlock,
            format!("another writer holds {} - refusing rather than overwriting their work", lock.display()),
        )));
    }
    let out = f();
    let _ = std::fs::remove_file(&lock);
    out
}

/// Write `content` to `path` ATOMICALLY: a sibling temp file, then a rename over the target (issue184).
///
/// `std::fs::write` truncates and then writes, so the target is momentarily EMPTY and then progressively
/// filled. A death in between - a kill, an OOM, a watchdog exit from another thread - leaves the
/// authoritative record truncated. Invariant 1 makes these files the TRUTH, and every one of the 21 write
/// sites in this crate reached that truth non-atomically.
///
/// THE DANGEROUS CASE IS NOT THE OBVIOUS ONE. A truncated file fails the parser, so the gate converts
/// corruption into a red gate. The case that survives is a PARTIAL write that still parses - these files
/// are lists of independent items, so a prefix is often syntactically complete once a closing brace
/// happens to land, and that file passes the gate with items silently missing.
///
/// The temp file is a SIBLING, not in a temp directory, because rename is only atomic within a
/// filesystem. On Windows `fs::rename` fails if the target exists, so the target is removed first - a
/// narrow window that is still strictly better than truncate-then-fill, and the temp file survives a
/// failure at that point rather than the original being gone.
///
/// # Errors
/// Returns the underlying [`std::io::Error`] if the temp write, the removal or the rename fails.
pub fn write_atomic(path: &std::path::Path, content: impl AsRef<str>) -> std::io::Result<()> {
    let tmp = path.with_extension(format!(
        "{}.keel-tmp",
        path.extension().map_or_else(String::new, |e| e.to_string_lossy().to_string())
    ));
    std::fs::write(&tmp, content.as_ref())?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    match std::fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Leave the temp file in place on failure: it holds the ONLY copy of the new content, and
            // deleting it here would turn a failed write into a lost write.
            Err(e)
        }
    }
}

// ── AST helpers ───────────────────────────────────────────────────────────────

fn task_exists_in_pkg(pkg: &Package, name: &str) -> bool {
    for item in &pkg.items {
        match item {
            Item::ActionDecl(a) if a.name == name => return true,
            Item::ActionDef(def) if def.actions.iter().any(|a| a.name == name) => return true,
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

/// True if a ceremony gate `verification <name>` exists (top-level or inside an
/// action def). Gates are `verification`s, not `action`s — distinct from `task_exists_in_pkg`.
fn gate_exists_in_pkg(pkg: &Package, name: &str) -> bool {
    pkg.items.iter().any(|item| match item {
        Item::Verification(v) => v.name == name,
        Item::ActionDef(def) => def.verifications.iter().any(|v| v.name == name),
        _ => false,
    })
}

/// Return the highest existing gate-result sequence number for `gate_name`.
/// Gate results follow `{gate}R{n}` (e.g. `rustS1CloseOutGateR1`) — NOT the
/// `{task}DoDR{n}` action convention.
fn max_gate_result_n(pkg: &Package, gate_name: &str) -> u32 {
    let pfx = format!("{gate_name}R");
    let mut max_n = 0u32;

    let scan = |name: &str, max: &mut u32| {
        if let Some(n) = name.strip_prefix(&pfx).and_then(|s| s.parse::<u32>().ok()) {
            if n > *max {
                *max = n;
            }
        }
    };

    for item in &pkg.items {
        match item {
            Item::Part(p) => scan(&p.name, &mut max_n),
            Item::ActionDef(def) => {
                for p in &def.parts {
                    scan(&p.name, &mut max_n);
                }
            }
            _ => {}
        }
    }
    max_n
}

/// Return the 0-indexed line after which to insert a new gate `TestResult`.
///
/// Prefers the last existing `part {gate}R{n}` result line; otherwise the closing
/// brace of the gate's `verification` block (tracked by brace depth, so multi-line
/// gate bodies are handled).
fn find_gate_result_insertion(lines: &[&str], gate_name: &str) -> Result<usize, WriteError> {
    let r_pfx = format!("part {gate_name}R");
    let ver_pat = format!("verification {gate_name}");

    let mut last_result = None;
    let mut ver_line = None;

    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.strip_prefix(&r_pfx)
            .is_some_and(|rest| rest.split([' ', ':']).next().unwrap_or("").parse::<u32>().is_ok())
        {
            last_result = Some(i);
        }
        if let Some(after) = t.strip_prefix(&ver_pat) {
            // Guard against `gate_name` being a prefix of a longer name: the next
            // char must terminate the identifier (` ` or `:`).
            if after.starts_with(' ') || after.starts_with(':') {
                ver_line = Some(i);
            }
        }
    }

    if let Some(r) = last_result {
        return Ok(r);
    }
    let v = ver_line.ok_or_else(|| WriteError::GateNotFound(gate_name.to_owned()))?;
    find_action_def_close(lines, v).ok_or_else(|| WriteError::GateNotFound(gate_name.to_owned()))
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
    for line in lines.get(def_start + 1..def_close).unwrap_or(&[]) {
        let trimmed = line.trim();
        if trimmed.starts_with("action ") {
            let indent_len = line.len() - line.trim_start().len();
            return " ".repeat(indent_len);
        }
    }
    "        ".to_owned() // fallback: 8 spaces
}

// ── parse helper ─────────────────────────────────────────────────────────────

/// The write-API operation table with the DECLARED human-judgment attribute (D0178).
///
/// The protected set is DERIVED from this, never hand-enumerated at call sites — `apply-review` is
/// the case a hand list misses. `true` = records a HUMAN's judgment / mutates control state and is
/// never agent-exempt.
pub const WRITE_OPS: &[(&str, bool)] = &[
    ("accept", true),
    ("apply-review", true),
    ("actor", true),  // actor-identity mutation (K7)
    ("enroll", true), // Person enrollment (K7)
    ("deactivate", true), // control weakening (K7)
    ("record", false),
    ("add-task", false),
    ("append-result", false),
    ("append-gate-result", false),
    ("new", false),
    ("mint", false),
    ("override", false),
];

/// The derived never-agent-exempt subcommand set (D0178).
#[must_use]
pub fn human_judgment_ops() -> Vec<&'static str> {
    WRITE_OPS.iter().filter(|(_, h)| *h).map(|(n, _)| *n).collect()
}

/// Record an orient-visible OBLIGATION fact (D0176/K7).
///
/// One Issue per file under `.tracking/obligations/`, so recording can never deadlock on the very
/// file being repaired (charter note 1). Returns the file written.
///
/// # Errors
/// Io on an unwritable tree — the CALLER degrades to the local ledger with a sync obligation.
pub fn record_obligation(root: &Path, slug: &str, title: &str, description: &str, actor: &str) -> Result<std::path::PathBuf, WriteError> {
    let dir = root.join(".tracking").join("obligations");
    std::fs::create_dir_all(&dir)?;
    let id = gen_uuid();
    let short = &id[..8];
    let path = dir.join(format!("{slug}-{short}.sysml"));
    let esc = |s: &str| sanitize_field(s);
    let text = format!(
        "// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded; a human review\n\
         // discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks\n\
         // on the file being repaired.\n\
         package Obligation{short} {{\n\
         \x20   private import EngineElement::*;\n\n\
         \x20   part obligation{short} : Issue {{\n\
         \x20       :>> id = \"{id}\";\n\
         \x20       :>> title = \"{}\";\n\
         \x20       :>> createdAt = \"{}\"; :>> createdBy = \"{actor}\";\n\
         \x20       :>> description = \"{}\";\n\
         \x20       :>> discoveredInField = false;\n\
         \x20       :>> severity = Severity::Low;\n\
         \x20   }}\n\
         }}\n",
        esc(title),
        crate::scaffold::today(),
        esc(description),
    );
    write_atomic(&path, text)?;
    Ok(path)
}

/// True when `{task}DoD`'s declared method is `confirmation` — searched at package level and inside
/// action defs, the two places a `DoD` verification lives.
fn dod_method_is_confirmation(pkg: &Package, task_name: &str) -> bool {
    let dod = format!("{task_name}DoD");
    let is_conf = |v: &keel_parser::ast::Verification| {
        v.name == dod
            && v.attributes.iter().any(|a| {
                a.name == "method"
                    && match &a.value {
                        keel_parser::ast::Value::EnumLit { member, .. } => member == "confirmation",
                        keel_parser::ast::Value::Str(s) | keel_parser::ast::Value::Ident(s) => s.contains("confirmation"),
                        _ => false,
                    }
            })
    };
    pkg.items.iter().any(|item| match item {
        Item::Verification(v) => is_conf(v),
        Item::ActionDef(def) => def.verifications.iter().any(is_conf),
        _ => false,
    })
}

/// The model root above `target` (the directory holding `.tracking`/`.engine`), when any.
fn model_root_of(target: &Path) -> Option<std::path::PathBuf> {
    let mut cur = target;
    while let Some(parent) = cur.parent() {
        if matches!(parent.file_name().and_then(|n| n.to_str()), Some(".tracking" | ".engine")) {
            return parent.parent().map(std::path::Path::to_path_buf);
        }
        cur = parent;
    }
    None
}

/// D0178/K6 write-layer check: refuse when `judged_by` is a registered AI-kind actor. An
/// UNREGISTERED name is refused too — an attestation by nobody is not weaker than one by an AI.
fn refuse_ai_judgment(path: &Path, judged_by: &str, what: &str) -> Result<(), WriteError> {
    let Some(root) = model_root_of(path) else {
        return Ok(()); // outside a model tree (unit-test scratch) — the actors guard owns registry integrity
    };
    if !root.join(".tracking").join("actors.sysml").exists() {
        return Ok(());
    }
    match crate::actor::kind_of(&root, judged_by).as_deref() {
        Some("human") => Ok(()),
        Some(_) => Err(WriteError::Parse(format!(
            "{what} records a HUMAN's judgment and `{judged_by}` is registered as an AI actor — refused (D0178/K6). Acceptance flows through channels the human holds."
        ))),
        None => Err(WriteError::Parse(format!(
            "{what} records a HUMAN's judgment and `{judged_by}` is not a registered actor — refused (D0178/K6)."
        ))),
    }
}

fn parse_file(path: &Path) -> Result<Package, WriteError> {
    let src = std::fs::read_to_string(path)?;
    let fname = path.to_string_lossy();
    let tokens = keel_parser::tokenize(&src, &fname)
        .map_err(|e| WriteError::Parse(e.to_string()))?;
    keel_parser::parse(tokens, &fname)
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
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_result_locked(path, task_name, sha, verdict, judged_at, judged_by))
}

fn append_result_locked(
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
    // K6 (D0178): a `method=confirmation` result IS a human attestation — the write layer refuses an
    // AI-kind judge regardless of what any hook or caller claimed.
    if dod_method_is_confirmation(&pkg, task_name) {
        refuse_ai_judgment(path, judged_by, "a method=confirmation result")?;
    }

    let n = max_result_n(&pkg, task_name) + 1;
    let uuid = gen_uuid();

    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let insert_after = find_result_insertion(&lines, task_name)?;

    // Detect indentation from surrounding context.
    let indent = lines.get(insert_after).map_or_else(String::new, |line| {
        " ".repeat(line.len() - line.trim_start().len())
    });

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

    write_atomic(path, new_content)?;
    Ok(uuid)
}

/// Inputs for [`append_critique`] — a human/independent review recorded as a linked critique
/// (D0086). The human reviewing an element IS an independent critic (D0080).
pub struct Critique<'a> {
    /// The reviewed element's part name (the `#Verify` target).
    pub element: &'a str,
    /// `"critique"` (a review judgment) or `"test"` (a downstream test result).
    pub method: &'a str,
    /// `CritiqueLens` member (critique only): correctness/completeness/ambiguity/…
    pub lens: &'a str,
    /// `CriticKind` member (critique only): human/aiModel/tool.
    pub critiqued_by: &'a str,
    /// `Severity` member, emitted on a failing critique: Critical/High/Medium/Low.
    pub severity: Option<&'a str>,
    /// Free-text rationale (sanitized into a single-line, quote-safe `procedureText`).
    pub rationale: &'a str,
    /// `"pass"` (accept) or `"fail"` (a finding — induces computed suspicion, D0086).
    pub outcome: &'a str,
    /// Commit the judgment was made against (`judgedAgainst`).
    pub sha: &'a str,
    /// ISO-8601 attestation date.
    pub judged_at: &'a str,
    /// Reviewer id (`judgedBy`).
    pub judged_by: &'a str,
}

/// Append a human/independent critique (or downstream test result) as NEW LINKED items (D0086).
///
/// Writes a `verification <element>HRev<n> : Test` + its `TestResult` + a `#Verify` edge to the
/// reviewed element, inserted before the package's closing brace. A failing outcome induces computed
/// suspicion (no element mutation, no parallel store). Reuses UUID generation; the index `<n>` is
/// the next free slot so repeated reviews never collide (append-only).
///
/// Returns the new verification name (`<element>HRev<n>`).
///
/// # Errors
/// `WriteError::InvalidVerdict` if `outcome` is not `"pass"`/`"fail"`;
/// `WriteError::InsertionPointNotFound` if the file has no package-closing brace;
/// `WriteError::Io` on filesystem errors.
pub fn append_critique(path: &Path, c: &Critique) -> Result<String, WriteError> {
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_critique_locked(path, c))
}

fn append_critique_locked(path: &Path, c: &Critique) -> Result<String, WriteError> {
    if c.outcome != "pass" && c.outcome != "fail" {
        return Err(WriteError::InvalidVerdict(c.outcome.to_owned()));
    }
    let content = std::fs::read_to_string(path)?;

    // Next free index for `<element>HRev<n>` — append-only, collision-free across re-reviews.
    let prefix = format!("{}HRev", c.element);
    let mut n = 1u32;
    while content.contains(&format!("{prefix}{n} ")) || content.contains(&format!("{prefix}{n}R")) {
        n += 1;
    }

    // Sanitize the rationale into a single-line, quote-safe string literal.
    let safe: String = c
        .rationale
        .replace('\\', "/")
        .replace('"', "'")
        .replace(['\n', '\r', '\t'], " ");

    let uuid_v = gen_uuid();
    let uuid_r = gen_uuid();
    let mut attrs = format!(":>> id = \"{uuid_v}\"; :>> method = VerificationMethod::{};", c.method);
    if c.method == "critique" {
        let _ = write!(attrs, " :>> lens = CritiqueLens::{}; :>> critiquedBy = CriticKind::{};", c.lens, c.critiqued_by);
        if c.outcome == "fail" {
            if let Some(sev) = c.severity {
                let _ = write!(attrs, " :>> severity = Severity::{sev};");
            }
        }
    }
    let _ = write!(attrs, " :>> procedureText = \"{safe}\";");

    let block = format!(
        "    verification {prefix}{n} : Test {{ {attrs} }}\n    part {prefix}{n}R1 : TestResult {{ :>> id = \"{uuid_r}\"; :>> outcome = VerdictKind::{}; :>> judgedAgainst = \"{}\"; :>> judgedAt = \"{}\"; :>> judgedBy = \"{}\"; }}\n    #Verify dependency from {prefix}{n} to {};\n",
        c.outcome, c.sha, c.judged_at, c.judged_by, c.element
    );

    let lines: Vec<&str> = content.lines().collect();
    let close = lines
        .iter()
        .rposition(|l| l.trim() == "}")
        .ok_or_else(|| WriteError::InsertionPointNotFound(c.element.to_owned()))?;

    let mut out = String::with_capacity(content.len() + block.len() + 1);
    for (i, line) in lines.iter().enumerate() {
        if i == close {
            out.push_str(&block);
        }
        out.push_str(line);
        out.push('\n');
    }
    write_atomic(path, out)?;
    Ok(format!("{prefix}{n}"))
}

/// A human's disposition of a >= Medium finding (D0092): the verdict + rationale + provenance.
pub struct Disposition<'a> {
    /// The finding Issue being dispositioned (the `#Dispositions` target).
    pub finding: &'a str,
    /// `DispositionKind` member: `act` | `acceptRisk` | `dismiss`.
    pub verdict: &'a str,
    /// Free-text rationale (sanitized into a single-line, quote-safe `procedureText`).
    pub rationale: &'a str,
    /// Commit the disposition was made against (`judgedAgainst`).
    pub sha: &'a str,
    /// ISO-8601 attestation date.
    pub judged_at: &'a str,
    /// The human who dispositioned (`judgedBy`).
    pub judged_by: &'a str,
}

/// Append a finding DISPOSITION (D0092) as NEW LINKED items.
///
/// Writes a `method=confirmation` verification carrying `disposition : DispositionKind`, its
/// `TestResult` (the human's attestation, outcome=pass), and a `#Dispositions` edge to the finding
/// Issue. Reuses the verification substrate, so the disposition inherits provenance + staleness
/// (suspect -> re-disposition). Append-only; `<n>` is the next free slot. Returns `<finding>Disp<n>`.
///
/// # Errors
/// `WriteError::InvalidVerdict` if `verdict` is not `act`/`acceptRisk`/`dismiss`;
/// `WriteError::InsertionPointNotFound` if the file has no package-closing brace;
/// `WriteError::Io` on filesystem errors.
pub fn append_disposition(path: &Path, d: &Disposition) -> Result<String, WriteError> {
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_disposition_locked(path, d))
}

fn append_disposition_locked(path: &Path, d: &Disposition) -> Result<String, WriteError> {
    if !matches!(d.verdict, "act" | "acceptRisk" | "dismiss") {
        return Err(WriteError::InvalidVerdict(d.verdict.to_owned()));
    }
    let content = std::fs::read_to_string(path)?;

    let prefix = format!("{}Disp", d.finding);
    let mut n = 1u32;
    while content.contains(&format!("{prefix}{n} ")) || content.contains(&format!("{prefix}{n}R")) {
        n += 1;
    }

    let safe: String = d.rationale.replace('\\', "/").replace('"', "'").replace(['\n', '\r', '\t'], " ");
    let uuid_v = gen_uuid();
    let uuid_r = gen_uuid();
    let block = format!(
        "    verification {prefix}{n} : Test {{ :>> id = \"{uuid_v}\"; :>> method = VerificationMethod::confirmation; :>> disposition = DispositionKind::{}; :>> procedureText = \"{safe}\"; }}\n    part {prefix}{n}R1 : TestResult {{ :>> id = \"{uuid_r}\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"{}\"; :>> judgedAt = \"{}\"; :>> judgedBy = \"{}\"; }}\n    #Dispositions dependency from {prefix}{n} to {};\n",
        d.verdict, d.sha, d.judged_at, d.judged_by, d.finding
    );

    let lines: Vec<&str> = content.lines().collect();
    let close = lines.iter().rposition(|l| l.trim() == "}").ok_or_else(|| WriteError::InsertionPointNotFound(d.finding.to_owned()))?;
    let mut out = String::with_capacity(content.len() + block.len() + 1);
    for (i, line) in lines.iter().enumerate() {
        if i == close {
            out.push_str(&block);
        }
        out.push_str(line);
        out.push('\n');
    }
    write_atomic(path, out)?;
    Ok(format!("{prefix}{n}"))
}

/// Append a `#Resolves` edge (`from` resolves `to`) before the file's package-closing brace (sr16/D0078).
///
/// Idempotent: a no-op if the exact edge already exists. Used when the console ACT-dispositions a finding
/// and attaches a resolver task (the tracked-resolver half of the critique loop).
///
/// # Errors
/// `WriteError::InsertionPointNotFound` if the file has no closing brace; `WriteError::Io`.
/// Set (or insert) a single attribute on an existing item's part, in place (D0126, `viewerEditItem`).
///
/// Finds the item's declaration across `.tracking/`, then replaces the `:>> <attr> = …;` inside its block
/// (or inserts it after the `{`). `literal` is the already-formed value (`"text"` or `EnumType::member`).
/// Owner-of-record edit (D0108) — the CALLER refuses governed types (which must supersede). Never commits.
///
/// # Errors
/// Returns [`WriteError::TaskNotFound`] if the item isn't found, or on I/O / malformed-block failure.
pub fn set_attr(root: &Path, item: &str, attr: &str, literal: &str) -> Result<String, WriteError> {
    // issue185: this SEARCHES for its target, so it locks the model rather than a file.
    with_file_lock(&root.join(".tracking"), || set_attr_locked(root, item, attr, literal))
}

fn set_attr_locked(root: &Path, item: &str, attr: &str, literal: &str) -> Result<String, WriteError> {
    let decl_kw = ["part ", "requirement ", "use case ", "action ", "verification "];
    for file in crate::collect_sysml(&root.join(".tracking")) {
        let content = std::fs::read_to_string(&file)?;
        // locate the item's declaration line: a keyword line naming `<item> :`
        let needle = format!("{item} :");
        let Some(decl_off) = content.match_indices(&needle).find_map(|(i, _)| {
            let line_start = content[..i].rfind('\n').map_or(0, |n| n + 1);
            let line = &content[line_start..i];
            decl_kw.iter().any(|k| line.trim_start().starts_with(k)).then_some(line_start)
        }) else { continue; };
        // block end: the first line that is a bare `}` at 4-space indent after the declaration
        let after = &content[decl_off..];
        let rel_end = after.find("\n    }").map_or(after.len(), |n| n + 6);
        let block_end = decl_off + rel_end;
        let block = &content[decl_off..block_end];
        let attr_pat = format!(":>> {attr} ");
        let new_attr = format!(":>> {attr} = {literal};");
        let updated_block = if let Some(ap) = block.find(&attr_pat) {
            // replace from `:>> attr` to the terminating `;`
            let semi = block[ap..].find(';').map(|s| ap + s + 1).ok_or_else(|| WriteError::InsertionPointNotFound(attr.to_owned()))?;
            format!("{}{}{}", &block[..ap], new_attr, &block[semi..])
        } else {
            // insert after the opening `{` of the declaration
            let brace = block.find('{').ok_or_else(|| WriteError::InsertionPointNotFound(item.to_owned()))?;
            format!("{}{{\n        {}{}", &block[..brace], new_attr, &block[brace + 1..])
        };
        let out = format!("{}{}{}", &content[..decl_off], updated_block, &content[block_end..]);
        write_atomic(&file, out)?;
        return Ok(file.strip_prefix(root).unwrap_or(&file).to_string_lossy().replace('\\', "/"));
    }
    Err(WriteError::TaskNotFound(item.to_owned()))
}

/// Header for the API-authored items/edges file (`.tracking/authored.sysml`), created on first write.
const AUTHORED_HEADER: &str = "// ProjectAuthored — items + edges created in-canvas via Architect Sure (POST /api/item, /api/edge; D0126).\n// New items land here with generated ids + provenance; relocate to a domain file as they mature.\npackage ProjectAuthored {\n    private import EngineElement::*;\n    private import EngineNeeds::*;\n    private import EngineRequirements::*;\n    private import EngineWork::*;\n    private import EngineVerification::*;\n    private import EngineRelationships::*;\n    private import EngineArchitecture::*;\n    private import EngineIndicator::*;\n    private import EngineComputed::*;\n    private import EngineProcess::*;\n    private import EngineSkills::*;\n\n";

/// Append a `#Resolves` marker edge (a resolver → the Issue it resolves).
///
/// # Errors
/// Returns [`WriteError`] if the file cannot be read/written or has no closing `}` insertion point.
pub fn append_resolves_edge(path: &Path, from: &str, to: &str) -> Result<(), WriteError> {
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_resolves_edge_locked(path, from, to))
}

fn append_resolves_edge_locked(path: &Path, from: &str, to: &str) -> Result<(), WriteError> {
    append_marker_edge(path, "Resolves", from, to)
}

/// The `SysML` text for a typed edge, per the closed algebra (D0126, `viewerCreateLinkage`).
/// Native forms for `satisfy`/`allocate`; the marker form (`#Kind dependency from…to…`) otherwise.
fn edge_line(kind: &str, from: &str, to: &str) -> String {
    match kind.to_lowercase().as_str() {
        "satisfy" => format!("satisfy {from} by {to};"),
        "allocate" => format!("allocate {from} to {to};"),
        _ => format!("#{kind} dependency from {from} to {to};"), // caller passes the PascalCase marker
    }
}

/// Insert `line` (indented) before the file's last `}` — append-only, idempotent (no-op if already present).
fn append_line_before_close(path: &Path, line: &str) -> Result<(), WriteError> {
    let content = std::fs::read_to_string(path)?;
    if content.contains(line) {
        return Ok(());
    }
    let lines: Vec<&str> = content.lines().collect();
    let close = lines.iter().rposition(|l| l.trim() == "}").ok_or_else(|| WriteError::InsertionPointNotFound(line.to_owned()))?;
    let mut out = String::with_capacity(content.len() + line.len() + 6);
    for (i, l) in lines.iter().enumerate() {
        if i == close {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
        out.push_str(l);
        out.push('\n');
    }
    write_atomic(path, out)?;
    Ok(())
}

/// Author a typed edge (D0126, `viewerCreateLinkage`).
///
/// Native `satisfy`/`allocate` or a marker edge, appended into `file_rel` — creating
/// `.tracking/authored.sysml` (with imports) if that is the target and absent. Additive, idempotent.
///
/// # Errors
/// Returns [`WriteError`] on I/O failure, a missing insertion point, or an absent non-authored target.
pub fn author_edge(root: &Path, file_rel: &str, kind: &str, from: &str, to: &str) -> Result<(), WriteError> {
    let line = edge_line(kind, from, to);
    let path = root.join(file_rel);
    if path.exists() {
        return append_line_before_close(&path, &line);
    }
    if file_rel.replace('\\', "/").ends_with("authored.sysml") {
        let out = format!("{AUTHORED_HEADER}    {line}\n}}\n");
        write_atomic(&path, out)?;
        return Ok(());
    }
    Err(WriteError::InsertionPointNotFound(file_rel.to_owned()))
}

/// Append a typed marker edge `#<marker> dependency from <from> to <to>;` before the file's last `}`.
///
/// Append-only, idempotent — a no-op if the exact edge already exists. The generalized primitive behind
/// `append_resolves_edge`; also carries `#Supersede` (viewerInProgramEdit, N-16 — supersede a
/// Need/Decision via a superseding Decision + this edge) and any other declared marker edge. The CALLER
/// (e.g. the serve endpoint) whitelists which markers are permitted — this helper stays generic.
///
/// # Errors
/// Returns [`WriteError`] if the file cannot be read/written, or has no closing `}` insertion point.
pub fn append_marker_edge(path: &Path, marker: &str, from: &str, to: &str) -> Result<(), WriteError> {
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_marker_edge_locked(path, marker, from, to))
}

fn append_marker_edge_locked(path: &Path, marker: &str, from: &str, to: &str) -> Result<(), WriteError> {
    append_line_before_close(path, &edge_line(marker, from, to))
}

/// Append a `Measurement` datapoint for an Indicator (D0089) as new linked items.
///
/// Writes a `part <indicator>M<n> : Measurement { value, measuredAt, source, createdBy }` + a
/// `dependency` edge to the indicator, before the package's closing brace. For pulled/manual
/// indicators (irreducible, non-recomputable observations); computed indicators store none.
///
/// Returns the new measurement's name (`<indicator>M<n>`).
///
/// # Errors
/// `WriteError::InsertionPointNotFound` if the file has no package-closing brace; `WriteError::Io`.
pub fn append_measurement(path: &Path, indicator: &str, value: &str, measured_at: &str, source: &str, by: &str) -> Result<String, WriteError> {
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_measurement_locked(path, indicator, value, measured_at, source, by))
}

fn append_measurement_locked(path: &Path, indicator: &str, value: &str, measured_at: &str, source: &str, by: &str) -> Result<String, WriteError> {
    let content = std::fs::read_to_string(path)?;
    let prefix = format!("{indicator}M");
    let mut n = 1u32;
    while content.contains(&format!("{prefix}{n} ")) {
        n += 1;
    }
    let uuid = gen_uuid();
    let sv = value.replace(['"', '\n', '\r'], "'");
    let ss = source.replace(['"', '\n', '\r'], "'");
    let block = format!(
        "    part {prefix}{n} : Measurement {{ :>> id = \"{uuid}\"; :>> value = \"{sv}\"; :>> measuredAt = \"{measured_at}\"; :>> source = \"{ss}\"; :>> createdBy = \"{by}\"; }}\n    #Measures dependency from {prefix}{n} to {indicator};\n"
    );
    let lines: Vec<&str> = content.lines().collect();
    let close = lines.iter().rposition(|l| l.trim() == "}").ok_or_else(|| WriteError::InsertionPointNotFound(indicator.to_owned()))?;
    let mut out = String::with_capacity(content.len() + block.len() + 1);
    for (i, line) in lines.iter().enumerate() {
        if i == close {
            out.push_str(&block);
        }
        out.push_str(line);
        out.push('\n');
    }
    write_atomic(path, out)?;
    Ok(format!("{prefix}{n}"))
}

/// Append a `part <gate>R<N+1> : TestResult { ... }` for a ceremony gate to `path`.
///
/// Records the result of a phase gate (refine/standup/implement/review/closeOut/retro),
/// which is a `verification`, not an `action` — so it uses the `{gate}R{n}` naming and
/// inserts after the gate's `verification` block (or after the last existing gate result).
///
/// Enforces:
/// - The gate `verification` must exist (else `GateNotFound`).
/// - `verdict` must be `"pass"` or `"fail"` (else `InvalidVerdict`).
/// - The new result index is `(max existing N) + 1` — never overwrites.
/// - A fresh UUID is auto-generated.
///
/// Returns the UUID of the newly created record.
///
/// # Errors
/// Returns `WriteError::InvalidVerdict` if `verdict` is not `"pass"` or `"fail"`.
/// Returns `WriteError::GateNotFound` if `gate_name` is not a `verification` in the file.
/// Returns `WriteError::Parse` if the file cannot be lexed or parsed.
/// Returns `WriteError::Io` on filesystem errors.
pub fn append_gate_result(
    path: &Path,
    gate_name: &str,
    sha: &str,
    verdict: &str,
    judged_at: &str,
    judged_by: &str,
    notes: Option<&str>,
) -> Result<String, WriteError> {
    // issue185: the WHOLE read-modify-write runs under the lock, not just the write.
    with_file_lock(path, || append_gate_result_locked(path, gate_name, sha, verdict, judged_at, judged_by, notes))
}

fn append_gate_result_locked(
    path: &Path,
    gate_name: &str,
    sha: &str,
    verdict: &str,
    judged_at: &str,
    judged_by: &str,
    notes: Option<&str>,
) -> Result<String, WriteError> {
    if verdict != "pass" && verdict != "fail" {
        return Err(WriteError::InvalidVerdict(verdict.to_owned()));
    }

    let pkg = parse_file(path)?;

    if !gate_exists_in_pkg(&pkg, gate_name) {
        return Err(WriteError::GateNotFound(gate_name.to_owned()));
    }

    let n = max_gate_result_n(&pkg, gate_name) + 1;
    let uuid = gen_uuid();

    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let insert_after = find_gate_result_insertion(&lines, gate_name)?;

    // Match the indentation of the line we insert after.
    let indent = lines.get(insert_after).map_or_else(String::new, |line| {
        " ".repeat(line.len() - line.trim_start().len())
    });

    let notes_attr = notes
        .map(|t| format!(" :>> notes = \"{}\";", sanitize_field(t)))
        .unwrap_or_default();
    let new_line = format!(
        "{indent}part {gate_name}R{n} : TestResult {{ :>> id = \"{uuid}\"; :>> outcome = VerdictKind::{verdict}; :>> judgedAgainst = \"{sha}\"; :>> judgedAt = \"{judged_at}\"; :>> judgedBy = \"{judged_by}\";{notes_attr} }}"
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

    write_atomic(path, new_content)?;
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
    // issue185: the whole read-modify-write under one lock.
    with_file_lock(path, || add_task_locked(path, def_name, task_name, dod_text, method))
}

fn add_task_locked(
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

    write_atomic(path, new_content)?;
    Ok(uuid)
}

// ── record verb (D0105/D0106 RMWX axis; issue054 C1) ────────────────────────────

/// Sanitize a field value for a one-line `SysML` string literal (double-quote → single, whitespace collapsed).
fn sanitize_field(v: &str) -> String {
    v.replace('"', "'").split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A request to create a new item (D0126, `viewerAuthoringEndpoints`).
pub struct NewItem<'a> {
    /// `SysML` declaration keyword matching the type's meta-kind: part, requirement, use-case, etc.
    pub keyword: &'a str,
    /// The declared type name (e.g. `Issue`, `Need`).
    pub type_name: &'a str,
    /// A hint for the element identifier (sanitized; falls back to `<type><uuid8>` if unusable).
    pub name_hint: &'a str,
    /// String-valued attributes (written as quoted, sanitized literals).
    pub string_attrs: &'a [(String, String)],
    /// Enum-valued attributes as `(attr, EnumType, member)` (written as `EnumType::member`).
    pub enum_attrs: &'a [(String, String, String)],
    /// Authoring actor (`createdBy`).
    pub author: &'a str,
    /// Authored ISO-8601 date (`createdAt`).
    pub created_at: &'a str,
}

/// Create a new item of a declared type in `.tracking/authored.sysml` (D0126, `viewerAuthoringEndpoints`).
///
/// Writes the part block with a generated UUID + provenance into the authored-items file (created with
/// broad imports if absent). String attrs are quoted+sanitized; enum attrs become `EnumType::member`
/// literals. Additive only — never mutates an existing item; the human commits (guards / `/api/check`
/// surface any incompleteness inline).
///
/// # Errors
/// Returns [`WriteError`] on I/O failure or a missing insertion point.
pub fn create_item(root: &Path, it: &NewItem) -> Result<(String, String), WriteError> {
    // issue185: this SEARCHES for its target, so it locks the model rather than a file.
    with_file_lock(&root.join(".tracking"), || create_item_locked(root, it))
}

fn create_item_locked(root: &Path, it: &NewItem) -> Result<(String, String), WriteError> {
    use std::fmt::Write as _;
    let uuid = gen_uuid();
    let ident = |s: &str| -> String { s.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect() };
    let mut name = ident(it.name_hint);
    if name.is_empty() || name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        name = format!("{}{}", it.type_name.to_lowercase(), &uuid[..8.min(uuid.len())]);
    }
    let mut lines = String::new();
    for (k, v) in it.string_attrs {
        if v.trim().is_empty() { continue; }
        let _ = writeln!(lines, "        :>> {k} = \"{}\";", sanitize_field(v));
    }
    for (k, enum_ty, v) in it.enum_attrs {
        let (val, ty) = (ident(v), ident(enum_ty));
        if val.is_empty() || ty.is_empty() { continue; }
        let _ = writeln!(lines, "        :>> {k} = {ty}::{val};");
    }
    let (keyword, type_name, author, created_at) = (it.keyword, it.type_name, it.author, it.created_at);
    let block = format!(
        "    {keyword} {name} : {type_name} {{\n        :>> id = \"{uuid}\";\n        :>> createdAt = \"{created_at}\"; :>> createdBy = \"{author}\";\n{lines}    }}\n"
    );
    let file = root.join(".tracking").join("authored.sysml");
    if file.exists() {
        let content = std::fs::read_to_string(&file)?;
        let idx = content.rfind('}').ok_or_else(|| WriteError::InsertionPointNotFound("authored.sysml".to_owned()))?;
        let mut out = String::with_capacity(content.len() + block.len() + 1);
        out.push_str(&content[..idx]);
        out.push_str(&block);
        out.push('\n');
        out.push_str(&content[idx..]);
        write_atomic(&file, out)?;
    } else {
        let mut out = String::from(AUTHORED_HEADER);
        out.push_str(&block);
        out.push_str("}\n");
        write_atomic(&file, out)?;
    }
    Ok((name, ".tracking/authored.sysml".to_owned()))
}

/// Next `NNNN` decision number in `<root>/.engine/decisions/` (highest `^\d{4}` + 1).
fn next_decision_number(decisions_dir: &Path) -> u32 {
    let mut max = 0u32;
    if let Ok(entries) = std::fs::read_dir(decisions_dir) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if let Some(n) = name.get(0..4).and_then(|s| s.parse::<u32>().ok()) {
                if n > max {
                    max = n;
                }
            }
        }
    }
    max + 1
}

/// `keel record decision` (issue054 C1 / D0105 RMWX `record` axis): scaffold a new proposed Decision file.
///
/// Auto-generates the UUID + next `NNNN` number, killing point-of-decision authoring friction (D0054).
/// Returns `(number, relative path)`. Status is `proposed` — acceptance is a separate explicit human gate
/// (D0106); this only CAPTURES the decision at the moment it is made.
///
/// # Errors
/// Returns `WriteError::Io` on filesystem errors.
#[allow(clippy::too_many_arguments)]
pub fn record_decision(
    root: &Path,
    slug: &str,
    title: &str,
    date: &str,
    author: &str,
    context: &str,
    decision: &str,
    rationale: &str,
    consequences: &str,
) -> Result<(String, String), WriteError> {
    let dir = root.join(".engine").join("decisions");
    let num = next_decision_number(&dir);
    let nnnn = format!("{num:04}");
    let uuid = gen_uuid();
    let s = sanitize_field;
    let file_text = format!(
        "// D{nnnn} (PROPOSED — NOT YET ACCEPTED) — {title_c}\n\
         // Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit\n\
         // human gate (method=confirmation, D0106): flip status + add the d{nnnn}Accept event on sign-off.\n\
         package Decision{nnnn} {{\n\
         \x20   private import EngineElement::*;\n\
         \x20   private import EngineWork::*;\n\
         \x20   private import EngineVerification::*;\n\
         \x20   private import EngineRelationships::*;\n\n\
         \x20   part d{nnnn} : Decision {{\n\
         \x20       :>> id = \"{uuid}\";\n\
         \x20       :>> title = \"{title_c}\";\n\
         \x20       :>> createdAt = \"{date_c}\";\n\
         \x20       :>> createdBy = \"{author_c}\";\n\
         \x20       :>> status = DecisionStatus::proposed;\n\
         \x20       :>> context = \"{context_c}\";\n\
         \x20       :>> decision = \"{decision_c}\";\n\
         \x20       :>> rationale = \"{rationale_c}\";\n\
         \x20       :>> consequences = \"{consequences_c}\";\n\
         \x20   }}\n\
         }}\n",
        title_c = s(title),
        date_c = s(date),
        author_c = s(author),
        context_c = s(context),
        decision_c = s(decision),
        rationale_c = s(rationale),
        consequences_c = s(consequences),
    );
    let filename = format!("{nnnn}-{slug}.sysml");
    write_atomic(&dir.join(&filename), file_text)?;
    Ok((nnnn, format!(".engine/decisions/{filename}")))
}

/// Accept a PROPOSED Decision (D0121 human review loop).
///
/// Flips `status = DecisionStatus::proposed` to `accepted` and appends the required acceptance event — a
/// `{decision}Accept : Test` (`method=confirmation`, `procedureText` = the human's note) + a passing
/// `{decision}AcceptR1 : TestResult` (`judgedBy` = the human `Person`). The note + the human's explicit
/// action ARE the attestation (D0106 — never fabricated); naming matches the attestation guard
/// (D0066 `{decision}AcceptR1`). Does NOT auto-commit.
///
/// # Errors
/// `WriteError::TaskNotFound` if the decision part or a `proposed` status is not present (already
/// accepted, or wrong file); `WriteError::Io` on filesystem errors.
pub fn accept_decision(
    path: &Path,
    decision: &str,
    sha: &str,
    judged_at: &str,
    judged_by: &str,
    note: &str,
) -> Result<String, WriteError> {
    // issue185: the whole read-modify-write under one lock.
    with_file_lock(path, || accept_decision_locked(path, decision, sha, judged_at, judged_by, note))
}

fn accept_decision_locked(
    path: &Path,
    decision: &str,
    sha: &str,
    judged_at: &str,
    judged_by: &str,
    note: &str,
) -> Result<String, WriteError> {
    refuse_ai_judgment(path, judged_by, "accepting a Decision")?;
    let content = std::fs::read_to_string(path)?;
    if !content.contains(&format!("part {decision} : Decision")) {
        return Err(WriteError::TaskNotFound(decision.to_owned()));
    }
    if !content.contains("DecisionStatus::proposed") {
        return Err(WriteError::TaskNotFound(format!("{decision} (no proposed status to accept)")));
    }
    let flipped = content.replacen("DecisionStatus::proposed", "DecisionStatus::accepted", 1);
    let close = flipped.rfind('}').ok_or_else(|| WriteError::TaskNotFound(format!("{decision} (no package close)")))?;
    let u1 = gen_uuid();
    let u2 = gen_uuid();
    let note_c = sanitize_field(note);
    let block = format!(
        "\n    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)\n\
         \x20   verification {decision}Accept : Test {{ :>> id = \"{u1}\"; :>> method = VerificationMethod::confirmation; :>> procedureText = \"{note_c}\"; }}\n\
         \x20   part {decision}AcceptR1 : TestResult {{ :>> id = \"{u2}\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"{sha}\"; :>> judgedAt = \"{judged_at}\"; :>> judgedBy = \"{judged_by}\"; }}\n",
    );
    let new_content = format!("{}{}{}", &flipped[..close], block, &flipped[close..]);
    write_atomic(path, new_content)?;
    Ok(u1)
}

/// Reject a PROPOSED Decision (D0121/D0122 human review loop).
///
/// Flips `status = DecisionStatus::proposed` to `rejected` and appends the rejection judgment — a
/// `{decision}Reject : Test` (`method=confirmation`, `procedureText` = the human's rationale) + a
/// `{decision}RejectR1 : TestResult` (`outcome=fail`, `judgedBy` = the human `Person`). The rationale +
/// the human's explicit action ARE the attestation (D0106 — never fabricated). Does NOT auto-commit.
///
/// # Errors
/// `WriteError::TaskNotFound` if the decision part or a `proposed` status is not present; `WriteError::Io`
/// on filesystem errors.
pub fn reject_decision(
    path: &Path,
    decision: &str,
    sha: &str,
    judged_at: &str,
    judged_by: &str,
    rationale: &str,
) -> Result<String, WriteError> {
    // issue185: the whole read-modify-write under one lock.
    with_file_lock(path, || reject_decision_locked(path, decision, sha, judged_at, judged_by, rationale))
}

fn reject_decision_locked(
    path: &Path,
    decision: &str,
    sha: &str,
    judged_at: &str,
    judged_by: &str,
    rationale: &str,
) -> Result<String, WriteError> {
    let content = std::fs::read_to_string(path)?;
    if !content.contains(&format!("part {decision} : Decision")) {
        return Err(WriteError::TaskNotFound(decision.to_owned()));
    }
    if !content.contains("DecisionStatus::proposed") {
        return Err(WriteError::TaskNotFound(format!("{decision} (no proposed status to reject)")));
    }
    let flipped = content.replacen("DecisionStatus::proposed", "DecisionStatus::rejected", 1);
    let close = flipped.rfind('}').ok_or_else(|| WriteError::TaskNotFound(format!("{decision} (no package close)")))?;
    let u1 = gen_uuid();
    let u2 = gen_uuid();
    let why = sanitize_field(rationale);
    let block = format!(
        "\n    // rejection judgment (D0121 review-queue; D0106 — human-judged, not fabricated)\n\
         \x20   verification {decision}Reject : Test {{ :>> id = \"{u1}\"; :>> method = VerificationMethod::confirmation; :>> procedureText = \"REJECTED: {why}\"; }}\n\
         \x20   part {decision}RejectR1 : TestResult {{ :>> id = \"{u2}\"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = \"{sha}\"; :>> judgedAt = \"{judged_at}\"; :>> judgedBy = \"{judged_by}\"; }}\n",
    );
    let new_content = format!("{}{}{}", &flipped[..close], block, &flipped[close..]);
    write_atomic(path, new_content)?;
    Ok(u1)
}

#[cfg(test)]
mod atomic_write_tests {
    use super::write_atomic;

    /// The target ends up with the NEW content and no temp file survives (issue184). The property that
    /// matters - never a PREFIX of the new content - cannot be tested without killing a process
    /// mid-write, so what is pinned here is that the mechanism is a temp-then-rename rather than a
    /// truncate, and that it cleans up after itself.
    #[test]
    fn a_write_replaces_the_target_and_leaves_no_temp_file() {
        let dir = std::env::temp_dir().join("keel-atomic-write-test");
        let _ = std::fs::create_dir_all(&dir);
        let target = dir.join("model.sysml");
        std::fs::write(&target, "old").expect("seed");
        write_atomic(&target, "new content").expect("atomic write");
        assert_eq!(std::fs::read_to_string(&target).expect("read"), "new content");
        let strays: Vec<_> = std::fs::read_dir(&dir)
            .expect("readdir")
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().contains("keel-tmp"))
            .collect();
        assert!(strays.is_empty(), "a temp file survived a successful write");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// THE CONTROL for issue185: no PUBLIC write entry point may read-modify-write outside the lock.
    ///
    /// Four concurrent `keel record issue` calls landed two issues, all four exiting 0. A concurrency
    /// test would be flaky, so what is pinned is the STRUCTURE: a `pub fn` that reads the file it is
    /// about to rewrite has not taken the lock, because the wrapped ones delegate to a private
    /// `*_locked` sibling and do no reading themselves.
    #[test]
    fn no_public_write_reads_outside_the_lock() {
        let full = std::fs::read_to_string("src/write.rs").expect("write.rs is readable");
        let src = &full[..full.find("
#[cfg(test)]").unwrap_or(full.len())];
        let mut offenders = Vec::new();
        let mut current: Option<String> = None;
        let mut public = false;
        for line in src.lines() {
            if line.starts_with("pub fn ") || line.starts_with("fn ") {
                public = line.starts_with("pub fn ");
                current = line
                    .split_whitespace()
                    .nth(usize::from(public) + 1)
                    .map(|n| n.split('(').next().unwrap_or(n).to_string());
            }
            if !line.contains("read_to_string") || line.trim_start().starts_with("//") {
                continue;
            }
            let name = current.clone().unwrap_or_default();
            // `with_file_lock` and `write_atomic` are the mechanism, not entry points.
            if public && !matches!(name.as_str(), "with_file_lock" | "write_atomic") {
                offenders.push(name);
            }
        }
        assert!(
            offenders.is_empty(),
            "public write entry point(s) reading outside the lock - concurrent writers silently lose              facts: {offenders:?}"
        );
    }

    /// No model write in this module may call `std::fs::write` directly. THE CONTROL: the defect was 21
    /// sites each individually reasonable, so the property to pin is that no new one appears.
    #[test]
    fn no_model_write_bypasses_the_atomic_helper() {
        let full = std::fs::read_to_string("src/write.rs").expect("write.rs is readable");
        // PRODUCTION code only: a test's own seed write is not a model write, and scanning the test
        // module made this test report itself.
        let src = &full[..full.find("
#[cfg(test)]").unwrap_or(full.len())];
        let offenders: Vec<String> = src
            .lines()
            .enumerate()
            .filter(|(_, l)| {
                let s = l.trim_start();
                s.contains("std::fs::write(") && !s.starts_with("//") && !s.contains("&tmp,")
            })
            .map(|(i, l)| format!("{}: {}", i + 1, l.trim()))
            .collect();
        assert!(
            offenders.is_empty(),
            "model write(s) bypassing write_atomic - a death mid-write truncates the truth: {offenders:#?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{gen_uuid, sanitize_field};
    use std::collections::HashSet;

    fn k6_root(tag: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("keel-k6-{tag}"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".tracking").join("delivery")).expect("mkdir");
        std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
        std::fs::write(
            root.join(".tracking").join("actors.sysml"),
            "package ProjectActors {\n    part hum : Person { :>> name = \"H\"; }\n    part bot : Actor { :>> name = \"B\"; :>> kind = ActorKind::ai; }\n}\n",
        )
        .expect("actors");
        root
    }

    /// K6/D0178 write layer: accepting a Decision refuses an AI-kind actor and an unregistered
    /// name; a registered Person passes. The check binds INSIDE the write, under the lock — no
    /// caller, hook, or transport can route around it.
    #[test]
    fn accept_decision_refuses_ai_and_unregistered_judges() {
        let root = k6_root("accept");
        let d = root.join(".engine").join("decisions").join("0001-t.sysml");
        let body = "package D1 {\n    part d1 : Decision { :>> id = \"e2e00000-0000-4000-8000-00000000d001\"; :>> status = DecisionStatus::proposed; }\n}\n";
        std::fs::write(&d, body).expect("decision");
        let ai = super::accept_decision(&d, "d1", "abc1234", "2026-08-21", "bot", "their words");
        assert!(ai.is_err(), "an AI-kind judge must be refused");
        assert!(format!("{}", ai.unwrap_err()).contains("AI actor"), "the refusal names the cause");
        let nobody = super::accept_decision(&d, "d1", "abc1234", "2026-08-21", "ghost", "their words");
        assert!(nobody.is_err(), "an unregistered judge must be refused");
        let human = super::accept_decision(&d, "d1", "abc1234", "2026-08-21", "hum", "their words");
        assert!(human.is_ok(), "a registered Person accepts: {human:?}");
    }

    /// K6/D0178 write layer: a `method=confirmation` result refuses an AI-kind judge; an ordinary
    /// `method=test` result does not (AI-recorded test evidence is the normal case, D0049).
    #[test]
    fn confirmation_results_are_human_only_at_the_write_layer() {
        let root = k6_root("confirm");
        let f = root.join(".tracking").join("delivery").join("x.sysml");
        let body = "package X {\n    action def Run {\n        action tconf;\n        verification tconfDoD : Test { :>> id = \"e2e00000-0000-4000-8000-00000000c001\"; :>> method = VerificationMethod::confirmation; :>> procedureText = \"human attests\"; }\n        action ttest;\n        verification ttestDoD : Test { :>> id = \"e2e00000-0000-4000-8000-00000000c002\"; :>> method = VerificationMethod::test; :>> procedureText = \"machine verifies\"; }\n    }\n}\n";
        std::fs::write(&f, body).expect("write");
        let ai_conf = super::append_result(&f, "tconf", "abc1234", "pass", "2026-08-21", "bot");
        assert!(ai_conf.is_err(), "an AI judging a confirmation must be refused");
        let hum_conf = super::append_result(&f, "tconf", "abc1234", "pass", "2026-08-21", "hum");
        assert!(hum_conf.is_ok(), "a Person judging a confirmation passes: {hum_conf:?}");
        let ai_test = super::append_result(&f, "ttest", "abc1234", "pass", "2026-08-21", "bot");
        assert!(ai_test.is_ok(), "an AI judging a method=test result is the normal case: {ai_test:?}");
    }

    /// dcMintCommand (us019): what `keel mint` prints must satisfy guard 38's OWN shape predicate,
    /// and 10000 mints contain no duplicate. The command exists so identity is never hand-authored;
    /// checking against the guard's predicate (not a re-derivation) keeps mint and guard one truth.
    #[test]
    fn minted_ids_satisfy_guard_38_and_do_not_collide_at_10k() {
        let mut seen = HashSet::new();
        for _ in 0..10_000 {
            let u = gen_uuid();
            assert!(crate::guards::uuid_shaped(&u), "guard 38 rejects a minted id: {u}");
            assert!(seen.insert(u), "duplicate within 10000 mints");
        }
    }

    #[test]
    fn sanitize_field_makes_a_safe_one_line_literal() {
        // record_decision (issue054) — field values must not break the one-line SysML string literal.
        assert_eq!(sanitize_field("has \"quotes\""), "has 'quotes'");
        assert_eq!(sanitize_field("multi\n  line\t text"), "multi line text");
        assert_eq!(sanitize_field("  trimmed  "), "trimmed");
    }

    #[test]
    fn gen_uuid_is_rfc4122_v4_shaped() {
        // issue075/D0129: identity is the engine's foundational invariant (§2.3), so the format
        // must be exactly right — 8-4-4-4-12 lowercase hex, version nibble 4, variant in [89ab].
        let u = gen_uuid();
        assert_eq!(u.len(), 36, "uuid must be 36 chars: {u}");
        let parts: Vec<&str> = u.split('-').collect();
        assert_eq!(parts.len(), 5, "uuid must have 5 groups: {u}");
        assert_eq!(
            parts.iter().map(|p| p.len()).collect::<Vec<_>>(),
            vec![8, 4, 4, 4, 12],
            "uuid group widths wrong: {u}"
        );
        assert!(
            u.chars().all(|c| c.is_ascii_hexdigit() || c == '-'),
            "uuid must be lowercase hex + dashes: {u}"
        );
        assert!(
            u.chars().filter(char::is_ascii_alphabetic).all(char::is_lowercase),
            "uuid hex must be lowercase: {u}"
        );
        assert_eq!(parts[2].as_bytes()[0], b'4', "version nibble must be 4: {u}");
        assert!(
            matches!(parts[3].as_bytes()[0], b'8' | b'9' | b'a' | b'b'),
            "variant nibble must be 8/9/a/b: {u}"
        );
    }

    #[test]
    fn gen_uuid_does_not_collide_and_is_not_clock_correlated() {
        // The pre-issue075 generator derived every id from clock + PID + an in-process counter that
        // started at 0 each invocation, so ids minted in the same instant on two machines could
        // collide — and with no duplicate-id detector (issue074) it would be silent. A CSPRNG-backed
        // v4 must show no collisions and no shared prefix across a tight loop.
        let n = 10_000;
        let ids: HashSet<String> = (0..n).map(|_| gen_uuid()).collect();
        assert_eq!(ids.len(), n, "gen_uuid produced a collision within {n} draws");

        // Clock-derived ids share a leading prefix within the same second; random ones must not.
        let first_group: HashSet<String> =
            ids.iter().map(|u| u.split('-').next().unwrap().to_owned()).collect();
        assert!(
            first_group.len() > n * 9 / 10,
            "first group is not well-distributed ({} distinct of {n}) — looks clock-derived",
            first_group.len()
        );
    }
}

// ── record issue (D0129 srDcContentionAdjudication) ───────────────────────────

/// A new `Issue`, with the triage that makes it well-formed on arrival.
pub struct NewIssue<'a> {
    pub title: &'a str,
    pub description: &'a str,
    /// `Critical` | `High` | `Medium` | `Low`.
    pub severity: &'a str,
    /// An EXISTING item that resolves it — the `#Resolves` edge is authored with the Issue.
    pub resolver: &'a str,
    pub related_task: Option<&'a str>,
    pub date: &'a str,
    pub author: &'a str,
    /// Optional engine marker prefix, e.g. `ProcessDefect`.
    pub marker: Option<&'a str>,
    /// Discovered in production use rather than by an internal check.
    pub in_field: bool,
}

/// Next free `issueNNN` name in the issues file.
fn next_issue_number(text: &str) -> u32 {
    let mut max = 0u32;
    let mut from = 0usize;
    while let Some(hit) = text[from..].find("part issue") {
        let at = from + hit + "part issue".len();
        let digits: String = text[at..].chars().take_while(char::is_ascii_digit).collect();
        if let Ok(n) = digits.parse::<u32>() {
            max = max.max(n);
        }
        from = at;
    }
    max + 1
}

/// `keel record issue` — author a TRIAGED Issue plus its `#Resolves` edge, in one call.
///
/// D0108 clause 5 MANDATES that conflicting conclusions across contributors be recorded as an Issue
/// for human adjudication, and there was no command that records an Issue at all — `record decision`
/// existed, `record issue` did not. A sanctioned path with no implementation is the friction that
/// guarantees non-compliance (D0054): the rule was reachable only by hand-editing a 1300-line file.
///
/// The `#Resolves` edge is written WITH the Issue rather than left for later, because the `issues`
/// guard fails on an untriaged Issue — so a command that produced one would hand the caller a red
/// gate as its output. Triage-on-arrival is what makes "green in one call" true.
///
/// # Errors
/// `WriteError::Io` on filesystem errors; `WriteError::TaskNotFound` if the issues file has no
/// package close to insert before.
pub fn record_issue(root: &Path, n: &NewIssue) -> Result<(String, String), WriteError> {
    // issue185: lock the file this will read-modify-write, for its whole duration.
    with_file_lock(&root.join(".tracking").join("issues.sysml"), || record_issue_locked(root, n))
}

fn record_issue_locked(root: &Path, n: &NewIssue) -> Result<(String, String), WriteError> {
    let path = root.join(".tracking").join("issues.sysml");
    let text = std::fs::read_to_string(&path)?;
    let num = next_issue_number(&text);
    let name = format!("issue{num:03}");
    let uuid = gen_uuid();
    let s = sanitize_field;
    let marker = n.marker.map_or_else(String::new, |m| format!("#{m} "));
    let related = n
        .related_task
        .map_or_else(String::new, |t| format!("        :>> relatedTask = \"{}\";\n", s(t)));
    let block = format!(
        "\n    {marker}part {name} : Issue {{\n\
         \x20       :>> id = \"{uuid}\";\n\
         \x20       :>> title = \"{title}\";\n\
         \x20       :>> createdAt = \"{date}\";\n\
         \x20       :>> createdBy = \"{author}\";\n\
         \x20       :>> description = \"{desc}\";\n\
         \x20       :>> discoveredInField = {in_field};\n\
         {related}\
         \x20       :>> severity = Severity::{sev};\n\
         \x20   }}\n\
         \x20   #Resolves dependency from {resolver} to {name};\n",
        title = s(n.title),
        date = s(n.date),
        author = s(n.author),
        desc = s(n.description),
        in_field = n.in_field,
        sev = s(n.severity),
        resolver = s(n.resolver),
    );
    let close = text
        .rfind('}')
        .ok_or_else(|| WriteError::TaskNotFound("issues.sysml (no package close)".to_owned()))?;
    let mut out = String::with_capacity(text.len() + block.len());
    out.push_str(text[..close].trim_end());
    out.push('\n');
    out.push_str(&block);
    out.push_str(&text[close..]);
    write_atomic(&path, out)?;
    Ok((name, ".tracking/issues.sysml".to_owned()))
}

#[cfg(test)]
mod issue_tests {
    use super::next_issue_number;

    #[test]
    fn issue_numbering_takes_the_max_not_the_count() {
        // Counting would collide the moment an issue is ever removed or numbered out of order, and a
        // duplicate id is its own corruption class (issue074). Max+1 is stable under both.
        assert_eq!(next_issue_number("part issue001 : Issue"), 2);
        assert_eq!(next_issue_number("part issue001\npart issue007\npart issue003"), 8);
        assert_eq!(next_issue_number("no issues here"), 1);
        // A mention inside prose must not drive the counter — the self-referential-corpus trap that
        // inflated the marker census in issue099. `part issue` is the declaration form.
        assert_eq!(next_issue_number("description = \"see issue900 for context\""), 1);
    }
}

// ── record claim (D0147 / D0129 srDcWorkClaim) ───────────────────────────────

/// Author a `Claim` — one contributor's intent to work one item.
///
/// PER-ACTOR FILE, deliberately: claims land in `.tracking/claims/<actor>.sysml` rather than a shared
/// file. That is `srDcPerActorWriteTargets` applied where it matters most — claims are the highest-
/// frequency concurrent write in the system, and routing every contributor's to the same anchor in
/// one file would make a textual conflict the NORMAL outcome of two people working at once. Distinct
/// files mean concurrent claims merge cleanly and the only contention left is the intended one: the
/// git ref update that decides who got the item.
///
/// `claimedAgainst` records the commit the claim was made against, so a reader can tell a claim made
/// against current work from one made against a tree forty commits stale.
///
/// # Errors
/// `WriteError::Io` on filesystem errors.
// @audit-hash ceRecordClaim
pub fn record_claim(root: &Path, item: &str, actor: &str) -> Result<(String, String), WriteError> {
    // issue185: lock the file this will read-modify-write, for its whole duration.
    with_file_lock(&root.join(".tracking").join("claims.sysml"), || record_claim_locked(root, item, actor))
}

fn record_claim_locked(root: &Path, item: &str, actor: &str) -> Result<(String, String), WriteError> {
    let dir = root.join(".tracking").join("claims");
    std::fs::create_dir_all(&dir)?;
    let file = dir.join(format!("{}.sysml", sanitize_name(actor)));
    let sha = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default();
    let at = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["log", "-1", "--format=%cs"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .filter(|s| s.len() == 10)
        .unwrap_or_default();
    let existing = std::fs::read_to_string(&file).unwrap_or_default();
    let n = existing.matches("part claim").count() + 1;
    let name = format!("claim{}{n:03}", sanitize_name(actor));
    let uuid = gen_uuid();
    let s = sanitize_field;
    let entry = format!(
        "    part {name} : Claim {{\n\
         \x20       :>> id = \"{uuid}\";\n\
         \x20       :>> title = \"claim on {item_c}\";\n\
         \x20       :>> createdAt = \"{at}\";\n\
         \x20       :>> createdBy = \"{actor_c}\";\n\
         \x20       :>> claimedItem = \"{item_c}\";\n\
         \x20       :>> claimedBy = \"{actor_c}\";\n\
         \x20       :>> claimedAt = \"{at}\";\n\
         \x20       :>> claimedAgainst = \"{sha}\";\n\
         \x20   }}\n",
        item_c = s(item),
        actor_c = s(actor),
    );
    let text = if existing.trim().is_empty() {
        format!(
            "// Claims authored by {actor_c} (D0147). PER-ACTOR FILE so concurrent claims by different\n\
             // contributors merge cleanly (srDcPerActorWriteTargets) — which also means BOTH claims on a\n\
             // contested item land, so who holds it is COMPUTED, never decided by the push.\n\
             //\n\
             // Liveness is NOT stored here. A claim is live if it is the earliest un-expired claim on its\n\
             // item; `keel claim --list` computes that. Nothing needs to be released.\n\
             package ProjectClaims{pkg} {{\n\
             \x20   private import EngineElement::*;\n\
             \x20   private import EngineWork::*;\n\n\
             {entry}}}\n",
            actor_c = s(actor),
            pkg = sanitize_name(actor),
        )
    } else {
        let close = existing
            .rfind('}')
            .ok_or_else(|| WriteError::TaskNotFound(format!("{} (no package close)", file.display())))?;
        format!("{}\n\n{entry}{}", existing[..close].trim_end(), &existing[close..])
    };
    write_atomic(&file, text)?;
    Ok((name, format!(".tracking/claims/{}.sysml", sanitize_name(actor))))
}

/// An actor id reduced to an identifier safe for a filename and a package name.
fn sanitize_name(v: &str) -> String {
    let mut out: String = v.chars().filter(char::is_ascii_alphanumeric).collect();
    if out.is_empty() {
        out.push_str("actor");
    }
    out
}
