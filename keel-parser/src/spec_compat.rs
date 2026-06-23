//! Spec-compatibility constants and pure verification helpers.
//!
//! The build script (`build.rs`) fetches the manifest at [`SYSML_V2_SPEC_URL`],
//! verifies its SHA-256 against [`SYSML_V2_GRAMMAR_SHA`], and bakes the result
//! into [`SYSML_V2_GRAMMAR_VERSION`].  Set `SYSML_V2_SPEC_OFFLINE=1` to skip
//! the network check (the default in `.cargo/config.toml`).

use sha2::{Digest, Sha256};

/// The URL whose content is pinned by [`SYSML_V2_GRAMMAR_SHA`].
pub const SYSML_V2_SPEC_URL: &str =
    "https://raw.githubusercontent.com/Systems-Modeling/SysML-v2-Release/master/README.md";

/// Expected SHA-256 hex digest of the manifest at [`SYSML_V2_SPEC_URL`].
/// All-zeros means "not yet pinned".
pub const SYSML_V2_GRAMMAR_SHA: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Grammar version string baked in at compile time by `build.rs`.
///
/// Possible values:
/// - `"offline"` — `SYSML_V2_SPEC_OFFLINE=1` was set; network check skipped.
/// - `"unavailable"` — network request failed; check skipped non-fatally.
/// - `"verified:<sha>"` — manifest fetched and SHA matched.
pub const SYSML_V2_GRAMMAR_VERSION: &str = env!("SYSML_V2_GRAMMAR_VERSION");

/// Returns `true` when the `SYSML_V2_SPEC_OFFLINE` environment variable equals `"1"`.
#[must_use]
pub fn is_offline() -> bool {
    std::env::var("SYSML_V2_SPEC_OFFLINE").as_deref() == Ok("1")
}

/// Compute the SHA-256 digest of `data` and return a lowercase hex string.
#[must_use]
pub fn sha256_hex(data: &[u8]) -> String {
    hex::encode(Sha256::digest(data))
}

/// Mismatch detail returned by [`verify_sha`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaMismatch {
    /// The SHA that was expected.
    pub expected: String,
    /// The SHA that was actually computed.
    pub actual: String,
}

impl std::fmt::Display for ShaMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SHA mismatch: expected {}, actual {}",
            self.expected, self.actual
        )
    }
}

/// Verify that `sha256_hex(data) == expected_sha`.
///
/// # Errors
///
/// Returns [`ShaMismatch`] when the digests differ.
pub fn verify_sha(data: &[u8], expected_sha: &str) -> Result<(), ShaMismatch> {
    let actual = sha256_hex(data);
    if actual == expected_sha {
        Ok(())
    } else {
        Err(ShaMismatch {
            expected: expected_sha.to_owned(),
            actual,
        })
    }
}
