#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use cucumber::{given, then, when, World};
use sysmlv2_parser::spec_compat::{sha256_hex, verify_sha, ShaMismatch};

#[derive(Debug, Default, World)]
pub struct SpecCompatWorld {
    manifest: Vec<u8>,
    expected_sha: Option<String>,
    result: Option<Result<(), ShaMismatch>>,
}

// ── given ────────────────────────────────────────────────────────────────────

#[given(expr = "manifest bytes {string}")]
fn given_manifest(world: &mut SpecCompatWorld, content: String) {
    world.manifest = content.into_bytes();
}

#[given(expr = "expected SHA {string}")]
fn given_sha(world: &mut SpecCompatWorld, sha: String) {
    world.expected_sha = Some(sha);
}

#[given(expr = "the expected SHA is the SHA-256 of the manifest bytes")]
fn given_computed_sha(world: &mut SpecCompatWorld) {
    let sha = sha256_hex(&world.manifest);
    world.expected_sha = Some(sha);
}

// ── when ─────────────────────────────────────────────────────────────────────

#[when(expr = "I verify the manifest SHA")]
fn when_verify(world: &mut SpecCompatWorld) {
    let sha = world.expected_sha.as_deref().unwrap_or("");
    world.result = Some(verify_sha(&world.manifest, sha));
}

#[when(expr = "I read the grammar version constant")]
fn when_version(_world: &mut SpecCompatWorld) {}

// ── then ─────────────────────────────────────────────────────────────────────

#[then(expr = "the grammar version is {string}")]
fn then_version(_world: &mut SpecCompatWorld, expected: String) {
    assert_eq!(
        sysmlv2_parser::spec_compat::SYSML_V2_GRAMMAR_VERSION,
        expected.as_str(),
        "grammar version mismatch"
    );
}

#[then(expr = "verification fails")]
fn then_fails(world: &mut SpecCompatWorld) {
    assert!(
        world.result.as_ref().is_some_and(|r| r.is_err()),
        "expected verification to fail but it succeeded"
    );
}

#[then(expr = "verification succeeds")]
fn then_succeeds(world: &mut SpecCompatWorld) {
    assert!(
        world.result.as_ref().is_some_and(|r| r.is_ok()),
        "expected verification to succeed but got: {:?}",
        world.result
    );
}

#[then(expr = "the error message contains {string}")]
fn then_error_contains(world: &mut SpecCompatWorld, text: String) {
    let msg = match world.result.as_ref() {
        Some(Err(e)) => e.to_string(),
        other => panic!("expected a verification error but got: {other:?}"),
    };
    assert!(
        msg.contains(&text),
        "error message {msg:?} does not contain {text:?}"
    );
}

#[tokio::main]
async fn main() {
    SpecCompatWorld::run("tests/features/spec_compat.feature").await;
}
