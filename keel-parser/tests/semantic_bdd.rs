#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use cucumber::{given, then, when, World};
use keel_parser::{parse, tokenize, Diagnostic, PackageRegistry};

#[derive(Debug, Default, World)]
pub struct SemanticWorld {
    registry: PackageRegistry,
    diagnostics: Vec<Diagnostic>,
}

fn parse_source(src: &str) -> keel_parser::ast::Package {
    let tokens = tokenize(src, "bdd-test").expect("lex error in test source");
    parse(tokens, "bdd-test").expect("parse error in test source")
}

#[given(expr = "the schema package {string} is registered")]
fn given_schema(world: &mut SemanticWorld, source: String) {
    let pkg = parse_source(&source);
    world.registry.register(&pkg);
}

#[when(expr = "I validate the package {string}")]
fn when_validate(world: &mut SemanticWorld, source: String) {
    let pkg = parse_source(&source);
    world.diagnostics = world.registry.validate(&pkg, "test.sysml");
}

#[then(expr = "there are {int} diagnostics")]
fn then_n_diagnostics(world: &mut SemanticWorld, count: usize) {
    assert_eq!(
        world.diagnostics.len(),
        count,
        "expected {count} diagnostics, got {}: {:#?}",
        world.diagnostics.len(),
        world.diagnostics
    );
}

#[then("there are no diagnostics")]
fn then_no_diagnostics(world: &mut SemanticWorld) {
    assert!(
        world.diagnostics.is_empty(),
        "expected no diagnostics, got: {:#?}",
        world.diagnostics
    );
}

#[then(expr = "diagnostic {int} message contains {string}")]
fn then_diag_contains(world: &mut SemanticWorld, n: usize, text: String) {
    let idx = n.saturating_sub(1);
    let diag = world.diagnostics.get(idx)
        .unwrap_or_else(|| panic!("no diagnostic at index {n}"));
    assert!(
        diag.message.contains(&text),
        "diagnostic {n} message {:?} does not contain {:?}",
        diag.message,
        text
    );
}

#[tokio::main]
async fn main() {
    SemanticWorld::run("tests/features/semantic.feature").await;
}
