//! Device-bound deck taps (D0201 OPTION B, the human's choice of 2026-08-23; dcAttestationBindsToDevice).
//!
//! A tap on the console - accept, reject, a gate pass, a disposition, a test verdict - is a HUMAN
//! attestation, and until now it was whatever POST reached `keel serve` on localhost: the server
//! recorded the human's judgment because a request said so. Any process on the machine, the agent
//! included, could issue that request.
//!
//! Now a tap carries an HMAC-SHA256 over its canonical content, keyed by a DEVICE KEY the browser
//! generated and holds; the server knows the key only because the human PAIRED the browser once by
//! typing the pairing code the serving terminal printed. A tap with no device, an unknown device, or
//! a wrong HMAC is refused with nothing written.
//!
//! WHAT THIS BINDS, STATED SO IT IS NOT OVER-CLAIMED: a tap is bound to the DEVICE that paired, not to
//! a person's identity. The trust that the paired browser is the human's rests on the pairing code
//! reaching them through the serving terminal - the same assumption `keel accept` at a TTY makes
//! (D0315). An attestation that claims more than that would be worse than one that claims less.
//!
//! The store is machine-local (`.keel/devices.toml`, gitignored): a device key is a secret of one
//! machine and travels nowhere.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// One paired device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: String,
    pub key: Vec<u8>,
    pub enrolled_at: String,
    pub label: String,
}

fn store_path(root: &Path) -> PathBuf {
    root.join(".keel").join("devices.toml")
}

/// HMAC-SHA256 (RFC 2104) over `msg` with `key`, built on the `sha2` the tree already carries.
#[must_use]
pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    const BLOCK: usize = 64;
    let mut k = [0u8; BLOCK];
    if key.len() > BLOCK {
        let d = Sha256::digest(key);
        for (slot, b) in k.iter_mut().zip(d.iter()) {
            *slot = *b;
        }
    } else {
        for (slot, b) in k.iter_mut().zip(key.iter()) {
            *slot = *b;
        }
    }
    let ipad: Vec<u8> = k.iter().map(|b| b ^ 0x36).collect();
    let opad: Vec<u8> = k.iter().map(|b| b ^ 0x5c).collect();
    let inner = Sha256::new().chain_update(&ipad).chain_update(msg).finalize();
    let outer = Sha256::new().chain_update(&opad).chain_update(inner).finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&outer);
    out
}

#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// Decode hex; `None` on any non-hex character or odd length.
#[must_use]
pub fn unhex(s: &str) -> Option<Vec<u8>> {
    let s = s.trim();
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok()).collect()
}

/// The canonical text a tap signs: kind, target, when, who, and the note - the fields the record
/// carries, joined so a signature over one tap cannot be replayed as another.
#[must_use]
pub fn canonical(kind: &str, target: &str, judged_at: &str, judged_by: &str, note: &str) -> String {
    format!("{kind}|{target}|{judged_at}|{judged_by}|{note}")
}

/// A six-digit pairing code from the OS CSPRNG, printed by the serving terminal at start.
#[must_use]
pub fn pairing_code() -> String {
    let mut b = [0u8; 4];
    let _ = getrandom::fill(&mut b);
    format!("{:06}", u32::from_le_bytes(b) % 1_000_000)
}

/// Every paired device on this machine.
#[must_use]
pub fn load(root: &Path) -> Vec<Device> {
    let Ok(text) = std::fs::read_to_string(store_path(root)) else { return Vec::new() };
    parse(&text)
}

/// Pure parse of `devices.toml` (`[[device]]` tables).
#[must_use]
pub fn parse(text: &str) -> Vec<Device> {
    let mut out = Vec::new();
    let mut cur: Option<Device> = None;
    for line in text.lines() {
        let l = line.trim();
        if l == "[[device]]" {
            if let Some(d) = cur.take() {
                out.push(d);
            }
            cur = Some(Device { id: String::new(), key: Vec::new(), enrolled_at: String::new(), label: String::new() });
            continue;
        }
        let (Some(d), Some((k, v))) = (cur.as_mut(), l.split_once('=')) else { continue };
        let v = v.trim().trim_matches('"');
        match k.trim() {
            "id" => d.id = v.to_string(),
            "key" => d.key = unhex(v).unwrap_or_default(),
            "enrolled_at" => d.enrolled_at = v.to_string(),
            "label" => d.label = v.to_string(),
            _ => {}
        }
    }
    if let Some(d) = cur {
        out.push(d);
    }
    out.into_iter().filter(|d| !d.id.is_empty() && !d.key.is_empty()).collect()
}

/// Pair a device: the code must be the one this serve printed; the key is the browser's, hex.
///
/// # Errors
/// A wrong code, a malformed id or key, or a store that cannot be written.
pub fn enroll(root: &Path, expected_code: &str, code: &str, device_id: &str, key_hex: &str, label: &str, today: &str) -> Result<(), String> {
    if code.trim() != expected_code {
        return Err("pairing code does not match the one the serving terminal printed - nothing enrolled".to_string());
    }
    if device_id.len() < 8 || !device_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("device id must be at least 8 alphanumeric characters".to_string());
    }
    let key = unhex(key_hex).filter(|k| k.len() >= 16).ok_or_else(|| "device key must be hex, at least 16 bytes".to_string())?;
    let mut devices = load(root);
    devices.retain(|d| d.id != device_id);
    devices.push(Device { id: device_id.to_string(), key, enrolled_at: today.to_string(), label: label.chars().filter(|c| *c != '"' && *c != '\n').take(80).collect() });
    let mut text = String::from("# Paired console devices (D0201 B). MACHINE-LOCAL: a device key is a secret of this machine.\n# A tap must carry an HMAC-SHA256 by one of these keys or it is refused with nothing written.\n");
    for d in &devices {
        use std::fmt::Write as _;
        let _ = write!(text, "\n[[device]]\nid = \"{}\"\nkey = \"{}\"\nenrolled_at = \"{}\"\nlabel = \"{}\"\n", d.id, hex(&d.key), d.enrolled_at, d.label);
    }
    let path = store_path(root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, text).map_err(|e| e.to_string())
}

/// Does this tap carry a valid signature by a paired device? `Ok(device id)` or the reason it does not.
///
/// # Errors
/// No device named, an unknown device, or a signature that does not match the canonical text.
pub fn verify(root: &Path, device_id: Option<&str>, hmac_hex: Option<&str>, canonical_text: &str) -> Result<String, String> {
    verify_against(&load(root), device_id, hmac_hex, canonical_text)
}

/// The pure check behind [`verify`], over a given device set (unit-tested without a store).
///
/// # Errors
/// As [`verify`].
pub fn verify_against(devices: &[Device], device_id: Option<&str>, hmac_hex: Option<&str>, canonical_text: &str) -> Result<String, String> {
    let Some(id) = device_id.map(str::trim).filter(|s| !s.is_empty()) else {
        return Err("this tap names no device - a human attestation is bound to the paired device that made it (D0201 B); pair this browser with the code the serving terminal printed".to_string());
    };
    let Some(dev) = devices.iter().find(|d| d.id == id) else {
        return Err(format!("device `{id}` is not paired on this machine - pair it with the code the serving terminal printed; nothing written"));
    };
    let Some(given) = hmac_hex.and_then(unhex) else {
        return Err("this tap carries no device signature (hmac) - nothing written".to_string());
    };
    let want = hmac_sha256(&dev.key, canonical_text.as_bytes());
    // constant-time comparison: no early exit on the first differing byte
    let same = given.len() == want.len() && given.iter().zip(want.iter()).fold(0u8, |acc, (a, b)| acc | (a ^ b)) == 0;
    if same { Ok(id.to_string()) } else { Err(format!("device `{id}` signature does not match this tap's content - the tap was altered or signed by another key; nothing written")) }
}

#[cfg(test)]
mod tests {
    use super::{canonical, hex, hmac_sha256, parse, unhex, verify_against, Device};

    /// RFC 4231 test case 2: key "Jefe", data "what do ya want for nothing?".
    #[test]
    fn hmac_sha256_matches_the_rfc_vector() {
        let mac = hmac_sha256(b"Jefe", b"what do ya want for nothing?");
        assert_eq!(hex(&mac), "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843");
    }

    /// The `DoD` both directions: a valid device signature records; absent, unknown, or wrong is refused.
    #[test]
    fn a_valid_signature_verifies_and_absent_unknown_or_wrong_is_refused() {
        let devices = vec![Device { id: "browser-1234".into(), key: b"0123456789abcdef".to_vec(), enrolled_at: "2026-09-05".into(), label: "hum".into() }];
        let text = canonical("accept", "d0001", "2026-09-05", "hum", "yes, exactly this");
        let good = hex(&hmac_sha256(b"0123456789abcdef", text.as_bytes()));
        assert_eq!(verify_against(&devices, Some("browser-1234"), Some(&good), &text), Ok("browser-1234".to_string()));
        assert!(verify_against(&devices, None, Some(&good), &text).unwrap_err().contains("names no device"));
        assert!(verify_against(&devices, Some("browser-9999"), Some(&good), &text).unwrap_err().contains("not paired"));
        assert!(verify_against(&devices, Some("browser-1234"), None, &text).unwrap_err().contains("no device signature"));
        let other = hex(&hmac_sha256(b"another-key-here", text.as_bytes()));
        assert!(verify_against(&devices, Some("browser-1234"), Some(&other), &text).unwrap_err().contains("does not match"));
        // a signature over one tap does not sign another
        let altered = canonical("accept", "d0002", "2026-09-05", "hum", "yes, exactly this");
        assert!(verify_against(&devices, Some("browser-1234"), Some(&good), &altered).is_err());
    }

    #[test]
    fn the_store_round_trips_and_ignores_incomplete_entries() {
        let text = "# header\n[[device]]\nid = \"browser-1234\"\nkey = \"30313233\"\nenrolled_at = \"2026-09-05\"\nlabel = \"hum\"\n[[device]]\nid = \"half\"\n";
        let d = parse(text);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].key, b"0123".to_vec());
        assert_eq!(unhex("zz"), None);
        assert_eq!(unhex("abc"), None);
    }
}
