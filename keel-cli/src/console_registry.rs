//! The machine-local console registry (D0245): which projects one console can reach, and whether a
//! console is already running.
//!
//! # Why this file exists outside every repository
//!
//! keel's rule is DISCOVERED, NOT DECLARED — `api_projects` carried the comment "A config file would be
//! a second place to keep the list true, and the filesystem already knows", and workspace discovery is
//! built on the same rule (D0234). That is right for "what projects does this repository contain" and
//! wrong for "which projects do I want in my console", because the second is an operator preference
//! that spans repositories and belongs to a person rather than to a tree. The filesystem knows which
//! directories are projects; it cannot know which ones the human cares about today.
//!
//! What kept the spirit of the rule is that this is A RECORD OF AN ACT, not a configuration to
//! maintain: an entry appears because someone ran `keel serve` there, and is dropped when the directory
//! stops existing. Nothing here is hand-edited and nothing has to be kept true, so it does not become
//! the second source of truth the original comment guarded against.
//!
//! # What it fixes
//!
//! Measured before: `keel serve` bound 127.0.0.1:7777 and hard-errored when taken, so a second project
//! either failed or was given `--port` and became another window; and the console's project selector
//! scanned the PARENT DIRECTORY for siblings, so it worked on this machine only because all eight
//! projects happen to share a parent, and a project in another repo was invisible.
//!
//! Never committed: it names filesystem paths on one machine and describes one person's working
//! arrangement.

use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};

use crate::json::Json;

/// Default console port. Deliberately rigid: silent port-hopping is the mechanism that produced the
/// window proliferation this registry exists to end (D0245 clause 6).
pub const DEFAULT_PORT: u16 = 7777;

/// One registered project.
pub struct Entry {
    pub root: PathBuf,
    /// Display label — the directory name, disambiguated by its repo when two labels collide.
    pub label: String,
    /// The git repository this project lives in, so two projects called `alpha` are tellable apart.
    pub repo: String,
    pub last_served: String,
}

/// The registry file's whole content.
pub struct Registry {
    /// The port a console was last started on, if one is believed to be running.
    pub port: Option<u16>,
    pub entries: Vec<Entry>,
}

/// `<home>/.keel/console.json` — machine-local, outside every repository.
#[must_use]
pub fn registry_path() -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    Some(Path::new(&home).join(".keel").join("console.json"))
}

fn str_field(v: &serde_json::Value, k: &str) -> String {
    v.get(k).and_then(serde_json::Value::as_str).unwrap_or_default().to_string()
}

/// Read the registry, pruning entries whose directory is gone.
///
/// D0245 clause 4 — prune dead, keep the rest. A missing or unreadable file is an EMPTY registry rather
/// than an error: the console has to start on a machine that has never run one.
#[must_use]
pub fn load() -> Registry {
    let Some(p) = registry_path() else { return Registry { port: None, entries: Vec::new() } };
    let Ok(text) = std::fs::read_to_string(&p) else { return Registry { port: None, entries: Vec::new() } };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else {
        return Registry { port: None, entries: Vec::new() };
    };
    let port = v.get("port").and_then(serde_json::Value::as_u64).and_then(|n| u16::try_from(n).ok());
    let mut entries = Vec::new();
    if let Some(arr) = v.get("projects").and_then(serde_json::Value::as_array) {
        for e in arr {
            let root = PathBuf::from(str_field(e, "root"));
            // PRUNE: the directory is gone, or it stopped being a project. Silently, because a deleted
            // project is not an error the reader needs to act on — it is just no longer listed.
            if !root.join(".engine").is_dir() || !root.join(".tracking").is_dir() {
                continue;
            }
            entries.push(Entry {
                label: str_field(e, "label"),
                repo: str_field(e, "repo"),
                last_served: str_field(e, "lastServed"),
                root,
            });
        }
    }
    Registry { port, entries }
}

fn write(reg: &Registry) -> Result<(), String> {
    let Some(p) = registry_path() else { return Err("no home directory to store the registry".into()) };
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    let rows: Vec<Json> = reg
        .entries
        .iter()
        .map(|e| {
            Json::Obj(vec![
                ("root".to_string(), Json::Str(e.root.display().to_string().replace('\\', "/"))),
                ("label".to_string(), Json::Str(e.label.clone())),
                ("repo".to_string(), Json::Str(e.repo.clone())),
                ("lastServed".to_string(), Json::Str(e.last_served.clone())),
            ])
        })
        .collect();
    let mut fields = vec![("projects".to_string(), Json::Arr(rows))];
    if let Some(port) = reg.port {
        fields.insert(0, ("port".to_string(), Json::Int(i64::from(port))));
    }
    let body = Json::Obj(fields).dump();
    std::fs::write(&p, body).map_err(|e| format!("cannot write {}: {e}", p.display()))
}

/// The git repository name containing `root`, for disambiguating two projects with the same label.
fn repo_of(root: &Path) -> String {
    crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .and_then(|t| Path::new(&t).file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_default()
}

/// Record that this project was served, and (when starting one) which port the console is on.
///
/// # Errors
/// Returns the reason the registry file could not be written.
pub fn register(root: &Path, port: Option<u16>, today: &str) -> Result<(), String> {
    let canon = crate::workspace::canon(root);
    let mut reg = load();
    let label = canon.file_name().map_or_else(|| ".".to_string(), |n| n.to_string_lossy().into_owned());
    let repo = repo_of(&canon);
    if let Some(existing) = reg.entries.iter_mut().find(|e| e.root == canon) {
        existing.last_served = today.to_string();
        existing.label = label;
        existing.repo = repo;
    } else {
        reg.entries.push(Entry { root: canon, label, repo, last_served: today.to_string() });
    }
    reg.entries.sort_by(|a, b| (&a.repo, &a.label).cmp(&(&b.repo, &b.label)));
    if port.is_some() {
        reg.port = port;
    }
    write(&reg)
}

/// Stop listing a project, without touching the project itself (D0245 clause 4).
///
/// # Errors
/// Returns the reason the registry file could not be written.
pub fn deregister(root: &Path) -> Result<bool, String> {
    let canon = crate::workspace::canon(root);
    let mut reg = load();
    let before = reg.entries.len();
    reg.entries.retain(|e| e.root != canon);
    let removed = reg.entries.len() != before;
    write(&reg)?;
    Ok(removed)
}

/// Is a KEEL CONSOLE answering on this port — not merely something holding the socket?
///
/// The distinction is the whole point of asking: a port held by an unrelated program must produce a
/// loud refusal, while a port held by our own console must produce an ATTACH. Guessing either way is
/// how a second window gets started or a stranger's server gets talked to.
#[must_use]
pub fn console_on(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    let Ok(sock) = addr.parse() else { return false };
    let timeout = std::time::Duration::from_millis(400);
    let Ok(mut stream) = std::net::TcpStream::connect_timeout(&sock, timeout) else { return false };
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));
    if stream
        .write_all(b"GET /api/version HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .is_err()
    {
        return false;
    }
    let mut buf = Vec::new();
    let mut chunk = [0_u8; 1024];
    // Bounded read: enough to see the version payload, never enough to hang on a chatty stranger.
    while let Ok(n) = stream.read(&mut chunk) {
        if n == 0 {
            break;
        }
        if let Some(read) = chunk.get(..n) {
            buf.extend_from_slice(read);
        }
        if buf.len() > 8192 {
            break;
        }
    }
    // Identify OUR console by fields only it serves, case-insensitively. The first version of this
    // looked for the literal lowercase "keel" and failed against a real running console, whose payload
    // spells it only as `viewerKeelApi` — so `serve` tried to bind, hit the occupied port, and
    // hard-errored: the exact failure this function exists to prevent, caused by the function itself.
    let text = String::from_utf8_lossy(&buf).to_lowercase();
    text.contains("200 ok") && text.contains("\"apiversion\"") && text.contains("keel")
}

/// Labels for display, disambiguated by repo only where two projects would otherwise collide — so the
/// common case stays short and the ambiguous case stays honest.
#[must_use]
pub fn display_labels(entries: &[Entry]) -> Vec<String> {
    entries
        .iter()
        .map(|e| {
            let clashes = entries.iter().filter(|o| o.label == e.label).count() > 1;
            if clashes && !e.repo.is_empty() { format!("{}/{}", e.repo, e.label) } else { e.label.clone() }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_disambiguate_only_when_they_collide() {
        let e = |label: &str, repo: &str| Entry {
            root: PathBuf::from(format!("/x/{repo}/{label}")),
            label: label.to_string(),
            repo: repo.to_string(),
            last_served: "2026-08-28".to_string(),
        };
        // Distinct labels stay short — the common case, and the one the human reads most.
        let plain = vec![e("penumbra", "penumbra"), e("passext", "passext")];
        assert_eq!(display_labels(&plain), vec!["penumbra", "passext"]);
        // A collision across repos is qualified rather than left ambiguous: two projects both called
        // `alpha` in one selector is exactly the confusion this registry exists to remove.
        let clash = vec![e("alpha", "repoA"), e("alpha", "repoB"), e("beta", "repoA")];
        assert_eq!(display_labels(&clash), vec!["repoA/alpha", "repoB/alpha", "beta"]);
    }

    #[test]
    fn a_port_nobody_is_listening_on_is_not_a_console() {
        // 0 is never bound; the check must say NO rather than hang or guess.
        assert!(!console_on(1));
    }
}
