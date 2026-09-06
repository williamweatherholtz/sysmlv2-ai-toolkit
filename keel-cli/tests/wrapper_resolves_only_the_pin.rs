//! Gherkin for `dcKeelWrapper` (sprint 491, D0251 clause B) — WRITTEN BEFORE the wrapper exists.
//!
//! The wrapper (`keelw`, committed POSIX sh) resolves the pinned version from
//! `engine-version.toml` against a machine-local cache (`.keel/bin/<version>/`); on a miss it
//! downloads THAT VERSION AND NO OTHER from the release origin, verifies a committed SHA-256
//! (`keel-wrapper.toml`), and execs. It never falls back to a binary on PATH.
//!
//! TESTABLE WITHOUT THE NETWORK: `KEELW_BASE_URL` overrides the release origin, so these scenarios
//! serve a fake release from a local directory over `file://` — the same override a mirror or an
//! air-gapped site would use, which is why it is a feature and not a test hook.

use std::path::{Path, PathBuf};
use std::process::Command;

fn ws_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("workspace root").to_path_buf()
}

/// The wrapper's platform-asset name for the machine running the test — mirrors release.yml.
fn asset_name() -> &'static str {
    if cfg!(windows) {
        "keel-windows-x86_64.exe"
    } else if cfg!(target_os = "macos") {
        "keel-macos-aarch64"
    } else {
        "keel-linux-x86_64"
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    // The test computes the checksum with the same tool family the wrapper uses, so a disagreement
    // is a real one. `sha256sum` rides git-bash on Windows and coreutils elsewhere.
    // Unique per CALL, not per process: the test harness runs scenarios in parallel threads of one
    // process, and a pid-keyed name made two fixtures hash each other's bytes — the checksum test
    // failing because of a checksum race is almost too on-the-nose.
    static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir();
    let f = dir.join(format!("keelw-sha-{}-{n}.bin", std::process::id()));
    std::fs::write(&f, bytes).expect("write");
    let out = Command::new("sh")
        .arg("-c")
        .arg(format!("sha256sum '{}' 2>/dev/null || shasum -a 256 '{}'", f.to_string_lossy().replace('\\', "/"), f.to_string_lossy().replace('\\', "/")))
        .output()
        .expect("sha tool");
    let _ = std::fs::remove_file(&f);
    String::from_utf8_lossy(&out.stdout).split_whitespace().next().expect("hex").to_string()
}

/// A fixture: a project pinning `version`, and a fake release origin holding `binary_body` as the
/// platform asset for that version. Returns (project, origin file:// URL).
fn fixture(tag: &str, version: &str, binary_body: &[u8], sha_entry: Option<&str>) -> (PathBuf, String) {
    let base = std::env::temp_dir().join(format!("keelw-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let proj = base.join("proj");
    std::fs::create_dir_all(proj.join(".engine").join("contracts")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".keel")).expect("mkdir");
    std::fs::write(
        proj.join(".engine").join("contracts").join("engine-version.toml"),
        format!("engine = \"{version}\"\n"),
    )
    .expect("pin");
    // The committed checksum contract. A missing entry must refuse (never "trust on first use").
    let sha = sha_entry.map(str::to_string).unwrap_or_else(|| sha256_hex(binary_body));
    std::fs::write(
        proj.join("keel-wrapper.toml"),
        format!("# keel-wrapper — per-version, per-platform release checksums (D0251 clause B).\n[\"{version}\"]\n\"{}\" = \"{sha}\"\n", asset_name()),
    )
    .expect("wrapper toml");
    std::fs::copy(ws_root().join("keelw"), proj.join("keelw")).expect("wrapper script is committed");
    // The fake origin: <origin>/v<version>/<asset>.
    let origin = base.join("origin").join(format!("v{version}"));
    std::fs::create_dir_all(&origin).expect("mkdir origin");
    std::fs::write(origin.join(asset_name()), binary_body).expect("asset");
    let url = format!("file://{}", base.join("origin").to_string_lossy().replace('\\', "/"));
    (proj, url)
}

/// A tiny executable stand-in: a shell script that prints a marker and its args.
fn fake_binary(marker: &str) -> Vec<u8> {
    format!("#!/bin/sh\necho FAKE-KEEL-{marker} \"$@\"\n").into_bytes()
}

fn run_keelw(proj: &Path, origin: &str, args: &[&str]) -> (bool, String) {
    let out = Command::new("sh")
        .arg("keelw")
        .args(args)
        .env("KEELW_BASE_URL", origin)
        .current_dir(proj)
        .output()
        .expect("sh keelw runs");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

// ── Scenario 1: cache miss + good checksum → download, cache, exec ────────────────────────────────

#[test]
fn a_miss_with_a_good_checksum_downloads_caches_and_execs() {
    let (proj, origin) = fixture("good", "9.9.9", &fake_binary("GOOD"), None);
    let (ok, text) = run_keelw(&proj, &origin, &["version"]);
    assert!(ok, "wrapper must succeed on a verified download: {text}");
    assert!(text.contains("FAKE-KEEL-GOOD"), "the DOWNLOADED binary must be what runs: {text}");
    assert!(
        proj.join(".keel").join("bin").join("9.9.9").join(asset_name()).exists(),
        "the verified binary is cached under .keel/bin/<version>/"
    );
    // Second run: cache HIT — the origin can vanish and the wrapper still works.
    let (ok, text) = run_keelw(&proj, "file:///nonexistent-origin", &["version"]);
    assert!(ok && text.contains("FAKE-KEEL-GOOD"), "a cache hit must not touch the origin: {text}");
    let _ = std::fs::remove_dir_all(proj.parent().unwrap());
}

// ── Scenario 2: BAD checksum refuses loudly and caches NOTHING ────────────────────────────────────

#[test]
fn a_bad_checksum_refuses_and_caches_nothing() {
    let bad_sha = "0000000000000000000000000000000000000000000000000000000000000000";
    let (proj, origin) = fixture("bad", "9.9.8", &fake_binary("EVIL"), Some(bad_sha));
    let (ok, text) = run_keelw(&proj, &origin, &["version"]);
    assert!(!ok, "a checksum mismatch must REFUSE — a swapped asset must be loud: {text}");
    assert!(
        text.to_lowercase().contains("checksum") || text.to_lowercase().contains("sha"),
        "the refusal names the checksum failure: {text}"
    );
    assert!(!text.contains("FAKE-KEEL-EVIL"), "the unverified binary must NEVER run: {text}");
    assert!(
        !proj.join(".keel").join("bin").join("9.9.8").join(asset_name()).exists(),
        "an unverified binary must not be cached"
    );
    let _ = std::fs::remove_dir_all(proj.parent().unwrap());
}

// ── Scenario 3: unreachable origin + empty cache → refuse with instructions, never PATH ───────────

#[test]
fn an_unreachable_origin_with_an_empty_cache_refuses_and_never_falls_back_to_path() {
    let (proj, _origin) = fixture("offline", "9.9.7", &fake_binary("UNUSED"), None);
    let (ok, text) = run_keelw(&proj, "file:///nonexistent-origin", &["version"]);
    assert!(!ok, "no cache + no origin must REFUSE with instructions, not improvise: {text}");
    assert!(
        !text.contains("build commit"),
        "the real keel on PATH must NOT have been run — falling back to PATH is exactly today's defect: {text}"
    );
    assert!(
        text.contains("9.9.7"),
        "the refusal names the pinned version the operator must obtain: {text}"
    );
    let _ = std::fs::remove_dir_all(proj.parent().unwrap());
}

// ── Scenario 4: the wrapper resolves ONLY the pin, even when newer assets exist ───────────────────

#[test]
fn the_wrapper_resolves_only_the_pinned_version() {
    let (proj, origin) = fixture("only-pin", "9.9.6", &fake_binary("PINNED"), None);
    // A NEWER, shinier asset exists at the origin. The wrapper must not even look at it.
    let base = proj.parent().unwrap();
    let newer = base.join("origin").join("v9.9.9-newer");
    std::fs::create_dir_all(&newer).expect("mkdir");
    std::fs::write(newer.join(asset_name()), fake_binary("NEWER")).expect("asset");
    let (ok, text) = run_keelw(&proj, &origin, &["version"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("FAKE-KEEL-PINNED") && !text.contains("FAKE-KEEL-NEWER"),
        "the wrapper fetches the PIN and nothing else — that property is why it is not auto-update (D0251/D0175): {text}"
    );
    let _ = std::fs::remove_dir_all(base);
}

// ── Scenario 5: a missing checksum entry refuses — never trust-on-first-use ───────────────────────

#[test]
fn a_missing_checksum_entry_refuses() {
    let (proj, origin) = fixture("no-entry", "9.9.5", &fake_binary("UNPINNED"), None);
    // Remove the checksum contract entirely: the wrapper has nothing to verify against.
    std::fs::remove_file(proj.join("keel-wrapper.toml")).expect("rm");
    let (ok, text) = run_keelw(&proj, &origin, &["version"]);
    assert!(!ok, "no committed checksum means no verified download — refuse, never TOFU: {text}");
    assert!(!text.contains("FAKE-KEEL-UNPINNED"), "the unverifiable binary must not run: {text}");
    let _ = std::fs::remove_dir_all(proj.parent().unwrap());
}

// ── Scenario 6 (issue379 / GH#54): a cache HIT is verified against the committed checksum ─────────

#[test]
fn a_cached_binary_that_does_not_match_the_committed_checksum_refuses_naming_both() {
    // The committed entry is the RELEASE's checksum; the cache holds a different (seeded) binary.
    let release_sha = sha256_hex(&fake_binary("RELEASE"));
    let (proj, _origin) = fixture("seeded", "9.9.4", &fake_binary("RELEASE"), Some(&release_sha));
    let cache = proj.join(".keel").join("bin").join("9.9.4");
    std::fs::create_dir_all(&cache).expect("cache dir");
    std::fs::write(cache.join(asset_name()), fake_binary("SEEDED-DEV")).expect("seed a different binary under the pin's name");
    let (ok, text) = run_keelw(&proj, "file:///nonexistent-origin", &["version"]);
    assert!(!ok, "a cache hit whose hash is not the committed one must REFUSE (issue379): {text}");
    assert!(!text.contains("FAKE-KEEL-SEEDED-DEV"), "the unpinned binary must never run: {text}");
    assert!(
        text.contains(&release_sha) && text.contains(&sha256_hex(&fake_binary("SEEDED-DEV"))),
        "the refusal names BOTH hashes so the skew is legible: {text}"
    );
    let _ = std::fs::remove_dir_all(proj.parent().unwrap());
}

#[test]
fn a_cache_hit_with_no_committed_entry_runs_and_says_it_is_unverified() {
    let (proj, _origin) = fixture("tofu-hit", "9.9.3", &fake_binary("HIT"), None);
    std::fs::remove_file(proj.join("keel-wrapper.toml")).expect("rm - nothing to verify against");
    let cache = proj.join(".keel").join("bin").join("9.9.3");
    std::fs::create_dir_all(&cache).expect("cache dir");
    let cached = cache.join(asset_name());
    std::fs::write(&cached, fake_binary("HIT")).expect("a cached binary");
    // a cached binary is executable (the download path chmods it; a seeded one is a copied executable)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&cached, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }
    let (ok, text) = run_keelw(&proj, "file:///nonexistent-origin", &["version"]);
    assert!(ok && text.contains("FAKE-KEEL-HIT"), "with no entry the hit runs, as before: {text}");
    assert!(text.contains("UNVERIFIED"), "and SAYS it ran on trust, so the skew is visible: {text}");
    let _ = std::fs::remove_dir_all(proj.parent().unwrap());
}
