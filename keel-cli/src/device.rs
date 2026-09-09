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

/// The console's own channel citation, appended by the server to a human-recorded verdict (issue287).
pub const CONSOLE_GESTURE: &str = " [recorded by the human in the keel console]";

/// The receipt the server appends to a tap's recorded note (D0201 B, made re-verifiable by D0411).
///
/// It carries the paired device's FULL id and the HMAC the tap arrived with, so a reader holding the
/// machine-local store can recompute the signature from the record's own fields ([`reverify`]).
/// Before D0411 the tag named a truncated id and the words `HMAC-verified` - text any writer could
/// type (issue426); the words stay, the receipt is what makes them true.
#[must_use]
pub fn receipt_tag(device_id: &str, hmac_hex: &str) -> String {
    format!(" [device {} hmac={} HMAC-verified]", device_id.trim(), hmac_hex.trim().to_lowercase())
}

/// Split a recorded note into (the text the device signed, device id, hmac hex).
///
/// `None` when the note carries no receipt of the [`receipt_tag`] form at its end. The console gesture
/// the server appended between the note and the receipt is stripped too, because the tap signed the
/// note before either was added.
#[must_use]
pub fn parse_receipt(recorded: &str) -> Option<(String, String, String)> {
    let at = recorded.rfind(" [device ")?;
    let tail = recorded.get(at + " [device ".len()..)?.strip_suffix(" HMAC-verified]")?;
    let (id, hmac) = tail.split_once(" hmac=")?;
    if id.is_empty() || id.contains(' ') || hmac.is_empty() {
        return None;
    }
    let signed = recorded.get(..at)?;
    let signed = signed.strip_suffix(CONSOLE_GESTURE).unwrap_or(signed);
    Some((signed.to_string(), id.to_string(), hmac.to_string()))
}

/// Re-verify a RECORDED verdict against this machine's device store (D0411 / issue426).
///
/// `kind`, `target`, `judged_at` and `judged_by` are the record's own fields; `recorded` is the note
/// as written. `Ok(device id)` means the signature in the receipt is the one that device's key
/// produces over exactly these fields - the tag was written by the server for this tap, not typed.
///
/// # Errors
/// No receipt in the note, an unpaired device, or a signature that does not match the record.
pub fn reverify(root: &Path, kind: &str, target: &str, judged_at: &str, judged_by: &str, recorded: &str) -> Result<String, String> {
    reverify_against(&load(root), kind, target, judged_at, judged_by, recorded)
}

/// The pure check behind [`reverify`], over a given device set.
///
/// # Errors
/// As [`reverify`].
pub fn reverify_against(devices: &[Device], kind: &str, target: &str, judged_at: &str, judged_by: &str, recorded: &str) -> Result<String, String> {
    let Some((signed, id, hmac)) = parse_receipt(recorded) else {
        return Err("the record carries no device receipt - a console tap since D0411 ends in `[device <id> hmac=<hex> HMAC-verified]`, written by the server; a note that only SAYS console or device is text (issue426)".to_string());
    };
    verify_against(devices, Some(&id), Some(&hmac), &canonical(kind, target, judged_at, judged_by, &signed))
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
    use super::{canonical, hex, hmac_sha256, parse, parse_receipt, receipt_tag, reverify_against, unhex, verify_against, Device, CONSOLE_GESTURE};

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

    /// D0411 / issue426: the record the server writes for a console tap re-verifies from its own
    /// fields against the device store; a note that merely says `console` or `HMAC-verified`, or a
    /// record whose signed text was changed after the tap, does not.
    #[test]
    fn a_console_records_receipt_reverifies_and_typed_words_do_not() {
        let key = b"0123456789abcdef".to_vec();
        let devices = vec![Device { id: "browser-1234abcd".into(), key: key.clone(), enrolled_at: "2026-09-09".into(), label: "hum".into() }];
        let note = "yes, exactly this";
        let mac = hex(&hmac_sha256(&key, canonical("accept", "d0001", "2026-09-09", "hum", note).as_bytes()));
        // what api_decision_accept writes: note + console gesture + receipt
        let recorded = format!("{note}{CONSOLE_GESTURE}{}", receipt_tag("browser-1234abcd", &mac));
        assert_eq!(parse_receipt(&recorded), Some((note.to_string(), "browser-1234abcd".to_string(), mac)));
        assert_eq!(reverify_against(&devices, "accept", "d0001", "2026-09-09", "hum", &recorded), Ok("browser-1234abcd".to_string()));
        // a gate result carries no console gesture: the receipt alone
        let gate_mac = hex(&hmac_sha256(&key, canonical("gate-pass", "xGate", "2026-09-09", "hum", "").as_bytes()));
        let gate_note = receipt_tag("browser-1234abcd", &gate_mac);
        assert_eq!(reverify_against(&devices, "gate-pass", "xGate", "2026-09-09", "hum", &gate_note), Ok("browser-1234abcd".to_string()));
        // typed text: the words are there, the receipt is not
        let typed = "approved at the console [device browser-1234abcd HMAC-verified]";
        assert!(reverify_against(&devices, "accept", "d0001", "2026-09-09", "hum", typed).unwrap_err().contains("no device receipt"));
        assert!(reverify_against(&devices, "accept", "d0001", "2026-09-09", "hum", "accepted in the keel console").unwrap_err().contains("no device receipt"));
        // the signed text edited after the tap, or the record's fields changed: the signature no longer matches
        let altered = recorded.replace("exactly this", "exactly that");
        assert!(reverify_against(&devices, "accept", "d0001", "2026-09-09", "hum", &altered).unwrap_err().contains("does not match"));
        assert!(reverify_against(&devices, "accept", "d0001", "2026-09-10", "hum", &recorded).unwrap_err().contains("does not match"));
        assert!(reverify_against(&devices, "accept", "d0002", "2026-09-09", "hum", &recorded).unwrap_err().contains("does not match"));
        // a device this machine never paired
        assert!(reverify_against(&[], "accept", "d0001", "2026-09-09", "hum", &recorded).unwrap_err().contains("not paired"));
    }
}
