//! The guard receipt: a green gate answers from the tree it already judged (dcGateAnswersFromItsReceipt,
//! rank 4 of D0367, the every-turn-boundary term of issue409).
//!
//! WHY. Every turn boundary ran validate, all the guards and the declared rules over a tree that, most
//! turns, was byte-identical to the one the previous boundary had judged green - about five seconds on
//! this host to recompute an answer nothing had changed. A gate must not skip; it may recognise the same
//! question.
//!
//! WHAT MAKES THE SHORTCUT HONEST is the KEY: a digest of every input a guard can read. HEAD's full id
//! covers every tracked file that is unmodified; `git status --porcelain -z -uall` names every path that
//! is modified, staged, deleted or untracked, and each such path contributes its `(len, mtime)`; every
//! file under `.keel/` outside `metrics/` and `bin/` contributes the same (a guard reads `.keel/actor`
//! and orient reads `.keel/cache/`; `metrics/` is written by the hooks themselves at every fire and
//! `bin/` holds binaries no guard opens); and the binary contributes its build commit, length and mtime,
//! so a rebuilt engine never answers from an older engine's judgment. An equal key means the same
//! inputs, and the same inputs to a pure computation are the same answer - not a skipped one.
//!
//! THREE REFUSALS keep it that way. A key is computed BEFORE the run and AGAIN after it, and the receipt
//! is written only when the two agree - a file changed mid-run is a run whose inputs were not one tree.
//! A path whose mtime is younger than two seconds makes the key UNSETTLED (the same racy window `corpus`
//! names) and an unsettled key is neither written nor honoured. And any red run DELETES the receipt.
//!
//! WHAT IS STORED beside the key: the guards' reports (name, scanned count, warnings) so `keel guard`
//! prints the population it printed before, plus one line naming the receipt's age. Violations are
//! never stored - a receipt exists only for a green run. `--no-receipt` (or `KEEL_NO_RECEIPT=1`) forces
//! the run.

use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::guards::GuardReport;

/// A file touched within this window may still be changing; its key is not trusted (see `corpus`).
const RACY: Duration = Duration::from_secs(2);

/// The layers a receipt may vouch for. `guards` alone comes from `keel guard`; all three from the
/// turn boundary and the commit gate.
pub const VALIDATE: &str = "validate";
pub const GUARDS: &str = "guards";
pub const RULES: &str = "rules";
/// What the turn boundary and the commit gate vouch for.
pub const ALL_LAYERS: [&str; 3] = [VALIDATE, GUARDS, RULES];

/// The digest of every input a guard can read, and whether it can be trusted yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub digest: String,
    pub head: String,
    /// False when a listed path's mtime is inside [`RACY`]: neither written nor honoured.
    pub settled: bool,
}

#[derive(Serialize, Deserialize)]
struct StoredGuard {
    name: String,
    scanned: usize,
    #[serde(default)]
    warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct Stored {
    key: String,
    #[serde(default)]
    head: String,
    #[serde(default)]
    build: String,
    /// Seconds since the Unix epoch when the run finished.
    #[serde(default)]
    written: u64,
    #[serde(default)]
    covers: BTreeSet<String>,
    #[serde(default)]
    guards: Vec<StoredGuard>,
}

/// A receipt whose key equals the one the caller computed.
pub struct Receipt {
    pub age: Duration,
    pub covers: BTreeSet<String>,
    pub guards: Vec<GuardReport>,
}

impl Receipt {
    /// Whether the receipt vouches for every one of `layers`.
    #[must_use]
    pub fn covers_all(&self, layers: &[&str]) -> bool {
        layers.iter().all(|l| self.covers.contains(*l))
    }

    /// The one line a caller prints when it answers from this receipt.
    #[must_use]
    pub fn line(&self, what: &str) -> String {
        format!(
            "{what} answered from the guard receipt written {} ago - the same tree, binary and .keel/ inputs this engine judged green ({}; --no-receipt runs everything)",
            human(self.age),
            REL
        )
    }
}

const REL: &str = ".keel/metrics/guard-receipt.toml";

fn path(root: &Path) -> PathBuf {
    root.join(".keel").join("metrics").join("guard-receipt.toml")
}

fn human(d: Duration) -> String {
    let s = d.as_secs();
    if s < 60 {
        format!("{s}s")
    } else if s < 3600 {
        format!("{}m {}s", s / 60, s % 60)
    } else {
        format!("{}h {}m", s / 3600, (s % 3600) / 60)
    }
}

/// `--no-receipt` on the command line, or `KEEL_NO_RECEIPT=1` in the environment (hooks and CI have no
/// argv of their own).
#[must_use]
pub fn forced(args: &[String]) -> bool {
    args.iter().any(|a| a == "--no-receipt") || std::env::var("KEEL_NO_RECEIPT").is_ok_and(|v| v == "1")
}

fn build_id() -> String {
    let exe = std::env::current_exe().ok().and_then(|p| std::fs::metadata(p).ok());
    let (len, mtime) = exe.map_or((0, 0), |m| (m.len(), mtime_nanos(&m)));
    format!("{}+{len}+{mtime}", env!("KEEL_BUILD_COMMIT"))
}

fn mtime_nanos(m: &std::fs::Metadata) -> u128 {
    m.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos())
}

/// Hash one path's `(len, mtime)` - or its absence - and report whether its mtime is settled.
fn hash_path(p: &Path, h: &mut impl Hasher) -> bool {
    if let Ok(m) = std::fs::metadata(p) {
        m.len().hash(h);
        mtime_nanos(&m).hash(h);
        m.modified().ok().is_none_or(|t| SystemTime::now().duration_since(t).is_ok_and(|age| age >= RACY))
    } else {
        "absent".hash(h);
        true
    }
}

/// The `(XY, path)` entries `git status --porcelain -z -uall` lists, in order. For a rename or copy
/// the SOURCE field is dropped (the file is gone; the destination is listed). Entries under
/// `.keel/metrics/` and `.keel/bin/` are dropped for the reason those directories are outside the key:
/// in a project that has not ignored `.keel/`, the receipt itself would otherwise appear in status and
/// no key could ever equal the one it was written under.
fn status_entries(out: &[u8]) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let mut fields = out.split(|b| *b == 0).filter(|f| !f.is_empty());
    while let Some(f) = fields.next() {
        let Some((xy, p)) = f.split_at_checked(3) else { continue };
        let path = String::from_utf8_lossy(p).to_string();
        if matches!(xy.first(), Some(b'R' | b'C')) || matches!(xy.get(1), Some(b'R' | b'C')) {
            let _ = fields.next();
        }
        if path.starts_with(".keel/metrics/") || path.starts_with(".keel/bin/") {
            continue;
        }
        entries.push((String::from_utf8_lossy(xy.get(..2).unwrap_or(xy)).to_string(), path));
    }
    entries
}

fn walk_keel(dir: &Path, h: &mut impl Hasher, settled: &mut bool, root: &Path) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk_keel(&p, h, settled, root);
        } else {
            p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/").hash(h);
            *settled &= hash_path(&p, h);
        }
    }
}

/// Compute the key for `root`. `None` when git cannot answer - a tree whose inputs cannot be named is
/// never answered from a receipt.
#[must_use]
pub fn key(root: &Path) -> Option<Key> {
    crate::perf::phase("receipt:key", || {
        let head = crate::gitfacts::head_sha(root)?;
        let root_s = root.to_string_lossy().to_string();
        let status = crate::gitx::git()
            .args(["-C", &root_s, "status", "--porcelain", "-z", "-uall"])
            .output()
            .ok()
            .filter(|o| o.status.success())?;
        let mut h = std::collections::hash_map::DefaultHasher::new();
        let mut settled = true;
        head.hash(&mut h);
        for (xy, rel) in status_entries(&status.stdout) {
            xy.hash(&mut h);
            rel.hash(&mut h);
            settled &= hash_path(&root.join(&rel), &mut h);
        }
        let keel = root.join(".keel");
        if let Ok(rd) = std::fs::read_dir(&keel) {
            let mut tops: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
            tops.sort();
            for p in tops {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "metrics" || name == "bin" {
                    continue;
                }
                if p.is_dir() {
                    walk_keel(&p, &mut h, &mut settled, root);
                } else {
                    name.hash(&mut h);
                    settled &= hash_path(&p, &mut h);
                }
            }
        }
        build_id().hash(&mut h);
        Some(Key { digest: format!("{:016x}", h.finish()), head, settled })
    })
}

/// The receipt at `root`, if one exists and its key equals `key` and `key` is settled.
#[must_use]
pub fn read(root: &Path, key: &Key) -> Option<Receipt> {
    if !key.settled {
        return None;
    }
    let text = std::fs::read_to_string(path(root)).ok()?;
    let stored: Stored = toml::from_str(&text).ok()?;
    if stored.key != key.digest {
        return None;
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let age = Duration::from_secs(now.saturating_sub(stored.written));
    let mut guards = Vec::with_capacity(stored.guards.len());
    for g in stored.guards {
        // A name this binary does not know cannot be printed as its report; the build id in the key
        // makes this unreachable, and it fails closed if it is not.
        let name = crate::guards::GUARD_NAMES.iter().find(|n| **n == g.name)?;
        guards.push(GuardReport { name, scanned: g.scanned, warnings: g.warnings, violations: Vec::new() });
    }
    Some(Receipt { age, covers: stored.covers, guards })
}

/// Record a green run; returns whether a receipt was written.
///
/// `before` is the key computed before the run; it is recomputed now and the receipt is written only
/// when the two agree. `reports` are the guards' reports (every one green); `covers` names the layers
/// this run vouched for. An existing receipt with an EQUAL key keeps its layers - so a `keel guard`
/// after a green turn boundary does not narrow what the boundary proved.
#[must_use]
pub fn record_green(root: &Path, before: &Key, covers: &[&str], reports: &[GuardReport]) -> bool {
    if !before.settled || reports.iter().any(|r| !r.ok()) {
        return false;
    }
    let Some(after) = key(root) else { return false };
    if after != *before {
        return false;
    }
    let mut set: BTreeSet<String> = covers.iter().map(|s| (*s).to_string()).collect();
    if let Ok(text) = std::fs::read_to_string(path(root)) {
        if let Ok(prev) = toml::from_str::<Stored>(&text) {
            if prev.key == before.digest {
                set.extend(prev.covers);
            }
        }
    }
    let stored = Stored {
        key: before.digest.clone(),
        head: before.head.clone(),
        build: build_id(),
        written: SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()),
        covers: set,
        guards: reports
            .iter()
            .map(|r| StoredGuard { name: r.name.to_string(), scanned: r.scanned, warnings: r.warnings.clone() })
            .collect(),
    };
    let Ok(text) = toml::to_string(&stored) else { return false };
    let p = path(root);
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    crate::write::write_atomic(&p, &text).is_ok()
}

/// A red run leaves no receipt.
pub fn delete(root: &Path) {
    let _ = std::fs::remove_file(path(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &Path, args: &[&str]) {
        let out = crate::gitx::git().arg("-C").arg(dir).args(args).output().expect("git");
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }

    /// A scratch repository with one committed file and a `.keel/actor`, every mtime aged past the
    /// racy window so its key is settled.
    fn repo(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("keel-receipt-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join(".keel")).expect("mk");
        git(&d, &["init", "-q"]);
        git(&d, &["-c", "user.email=t@t", "-c", "user.name=t", "commit", "-q", "--allow-empty", "-m", "root"]);
        std::fs::write(d.join(".keel").join("actor"), "someone").expect("actor");
        age_all(&d);
        d
    }

    fn age(p: &Path) {
        let f = std::fs::OpenOptions::new().write(true).open(p).expect("open");
        f.set_modified(SystemTime::now() - Duration::from_mins(10)).expect("mtime");
    }

    fn age_all(d: &Path) {
        for e in std::fs::read_dir(d.join(".keel")).expect("rd").flatten() {
            if e.path().is_file() {
                age(&e.path());
            }
        }
    }

    fn green() -> Vec<GuardReport> {
        vec![GuardReport { name: crate::guards::GUARD_NAMES[0], scanned: 3, warnings: vec!["w".into()], violations: Vec::new() }]
    }

    /// dcGateAnswersFromItsReceipt: a green run writes the receipt and the next run answers from it,
    /// with the reports it stored.
    #[test]
    fn a_green_run_writes_the_receipt_and_the_next_run_answers_from_it() {
        let d = repo("green");
        let k = key(&d).expect("key");
        assert!(k.settled, "an aged scratch repo must be settled");
        assert!(read(&d, &k).is_none(), "no receipt before any run");
        assert!(record_green(&d, &k, &[GUARDS], &green()), "a green run writes");
        let r = read(&d, &k).expect("the receipt answers an equal key");
        assert!(r.covers_all(&[GUARDS]) && !r.covers_all(&[VALIDATE]));
        assert_eq!(r.guards.len(), 1);
        assert_eq!(r.guards[0].warnings, vec!["w".to_string()]);
        // A second writer with an equal key widens the layers rather than narrowing them.
        assert!(record_green(&d, &k, &[VALIDATE, RULES], &green()));
        assert!(read(&d, &k).expect("still equal").covers_all(&[VALIDATE, GUARDS, RULES]));
        let _ = std::fs::remove_dir_all(&d);
    }

    /// Each input a guard can read forces a run when it changes: a touched tracked file, a new untracked
    /// file, a changed `.keel/` file. (A different build id is the same mechanism with a different
    /// field; the binary running this test cannot change under it.)
    #[test]
    fn a_touched_tracked_file_a_new_untracked_file_and_a_changed_keel_file_each_force_a_run() {
        let d = repo("inputs");
        std::fs::write(d.join("tracked.txt"), "one").expect("w");
        git(&d, &["add", "tracked.txt"]);
        git(&d, &["-c", "user.email=t@t", "-c", "user.name=t", "commit", "-q", "-m", "tracked"]);
        let k0 = key(&d).expect("key");
        assert!(record_green(&d, &k0, &[GUARDS], &green()));
        assert!(read(&d, &k0).is_some());

        // Untracked file: it appears in status, so the key moves.
        std::fs::write(d.join("new.txt"), "n").expect("w");
        age(&d.join("new.txt"));
        let k1 = key(&d).expect("key");
        assert_ne!(k0.digest, k1.digest, "a new untracked file changes the key");
        assert!(read(&d, &k1).is_none(), "and the old receipt does not answer it");

        // Touched tracked file: modified in status, its (len, mtime) in the key.
        std::fs::remove_file(d.join("new.txt")).expect("rm");
        assert_eq!(key(&d).expect("key").digest, k0.digest, "removing it restores the key");
        std::fs::write(d.join("tracked.txt"), "two").expect("w");
        age(&d.join("tracked.txt"));
        let k2 = key(&d).expect("key");
        assert_ne!(k0.digest, k2.digest, "a modified tracked file changes the key");

        // Changed .keel/ file.
        git(&d, &["checkout", "-q", "--", "tracked.txt"]);
        std::fs::write(d.join(".keel").join("actor"), "someone-else").expect("w");
        age(&d.join(".keel").join("actor"));
        let k3 = key(&d).expect("key");
        assert_ne!(k0.digest, k3.digest, "a changed .keel/ file changes the key");
        // .keel/metrics/ is the hooks' own ledger and is NOT in the key.
        std::fs::create_dir_all(d.join(".keel").join("metrics")).expect("mk");
        std::fs::write(d.join(".keel").join("metrics").join("hooks.jsonl"), "{}").expect("w");
        assert_eq!(key(&d).expect("key").digest, k3.digest, "a metrics write leaves the key alone");
        let _ = std::fs::remove_dir_all(&d);
    }

    /// A red run leaves no receipt; a file younger than the racy window is neither written nor honoured.
    #[test]
    fn a_red_run_deletes_the_receipt_and_an_unsettled_key_is_never_trusted() {
        let d = repo("red");
        let k = key(&d).expect("key");
        assert!(record_green(&d, &k, &[GUARDS], &green()));
        let red = vec![GuardReport { name: crate::guards::GUARD_NAMES[0], scanned: 1, warnings: Vec::new(), violations: vec!["v".into()] }];
        assert!(!record_green(&d, &k, &[GUARDS], &red), "a red run never writes");
        delete(&d);
        assert!(read(&d, &k).is_none(), "a red run leaves no receipt");
        // Fresh write inside the racy window: unsettled.
        std::fs::write(d.join("fresh.txt"), "f").expect("w");
        let ku = key(&d).expect("key");
        assert!(!ku.settled, "a file written just now is inside the racy window");
        assert!(!record_green(&d, &ku, &[GUARDS], &green()), "an unsettled key is not written");
        assert!(read(&d, &ku).is_none(), "nor honoured");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn status_entries_drop_the_rename_source_and_the_receipt_dirs_and_keep_every_other_field() {
        let out = b" M a.txt\0?? b/c.txt\0R  new.txt\0old.txt\0 D gone.txt\0?? .keel/metrics/guard-receipt.toml\0?? .keel/actor\0";
        let got: Vec<String> = status_entries(out).into_iter().map(|(xy, p)| format!("{xy}|{p}")).collect();
        assert_eq!(got, vec![" M|a.txt", "??|b/c.txt", "R |new.txt", " D|gone.txt", "??|.keel/actor"]);
    }
}
