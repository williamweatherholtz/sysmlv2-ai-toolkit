use std::io::Read;
use std::time::Duration;
use std::{env, process};

/// URL of the SysML v2 grammar manifest this crate pins.
/// Update together with SYSML_V2_GRAMMAR_SHA whenever upgrading to a new spec release.
const SYSML_V2_SPEC_URL: &str =
    "https://raw.githubusercontent.com/Systems-Modeling/SysML-v2-Release/master/README.md";

/// Expected SHA-256 hex digest of the file at SYSML_V2_SPEC_URL.
/// Set to all-zeros as an explicit "not yet pinned" sentinel.
/// Run with SYSML_V2_SPEC_OFFLINE=0 and let the build print the actual SHA to pin it.
const SYSML_V2_GRAMMAR_SHA: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(data))
}

fn main() {
    println!("cargo:rerun-if-env-changed=SYSML_V2_SPEC_OFFLINE");

    if env::var("SYSML_V2_SPEC_OFFLINE").as_deref() == Ok("1") {
        println!("cargo:rustc-env=SYSML_V2_GRAMMAR_VERSION=offline");
        return;
    }

    let result = ureq::get(SYSML_V2_SPEC_URL)
        .timeout(Duration::from_secs(5))
        .call();

    match result {
        Ok(resp) => {
            let mut body: Vec<u8> = Vec::new();
            if resp.into_reader().read_to_end(&mut body).is_err() {
                println!("cargo:warning=SysML v2 spec: failed to read response body. Set SYSML_V2_SPEC_OFFLINE=1 to suppress.");
                println!("cargo:rustc-env=SYSML_V2_GRAMMAR_VERSION=unavailable");
                return;
            }
            let actual = sha256_hex(&body);
            if actual != SYSML_V2_GRAMMAR_SHA {
                // Use process::exit instead of panic! so the message isn't buried in a panic frame.
                eprintln!(
                    "\nerror: SysML v2 grammar manifest SHA mismatch!\
                    \n  expected: {SYSML_V2_GRAMMAR_SHA}\
                    \n  actual:   {actual}\
                    \n\nThe spec at {SYSML_V2_SPEC_URL} has changed.\
                    \nUpdate SYSML_V2_GRAMMAR_SHA in build.rs to the actual value above,\
                    \nor set SYSML_V2_SPEC_OFFLINE=1 to skip the check.\n"
                );
                process::exit(1);
            }
            println!("cargo:rustc-env=SYSML_V2_GRAMMAR_VERSION=verified:{SYSML_V2_GRAMMAR_SHA}");
        }
        Err(e) => {
            println!(
                "cargo:warning=SysML v2 spec check skipped (network unavailable: {e}). \
                 Set SYSML_V2_SPEC_OFFLINE=1 to suppress this warning."
            );
            println!("cargo:rustc-env=SYSML_V2_GRAMMAR_VERSION=unavailable");
        }
    }
}
