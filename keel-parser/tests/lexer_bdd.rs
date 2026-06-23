#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use cucumber::{given, then, when, World};
use keel_parser::token::TokenKind;
use keel_parser::tokenize;

#[derive(Debug, Default, World)]
pub struct LexerWorld {
    source: String,
    result: Vec<keel_parser::token::Token>,
    error_msg: Option<String>,
}

#[given(expr = "the source text {string}")]
fn given_source(world: &mut LexerWorld, source: String) {
    // Expand \n so Gherkin string params can contain newlines.
    world.source = source.replace("\\n", "\n");
}

#[when("I tokenize the source")]
fn when_tokenize(world: &mut LexerWorld) {
    match tokenize(&world.source, "test") {
        Ok(tokens) => {
            world.result = tokens;
            world.error_msg = None;
        }
        Err(e) => {
            world.result = Vec::new();
            world.error_msg = Some(e.to_string());
        }
    }
}

#[then("the first token kind is Package")]
fn then_package(world: &mut LexerWorld) {
    assert_eq!(
        world.result.first().map(|t| &t.kind),
        Some(&TokenKind::Package)
    );
}

#[then("the first token kind is ColonColon")]
fn then_coloncolon(world: &mut LexerWorld) {
    assert_eq!(
        world.result.first().map(|t| &t.kind),
        Some(&TokenKind::ColonColon)
    );
}

#[then("the first token kind is ColonGtGt")]
fn then_colongtgt(world: &mut LexerWorld) {
    assert_eq!(
        world.result.first().map(|t| &t.kind),
        Some(&TokenKind::ColonGtGt)
    );
}

#[then("the first token kind is ColonGt")]
fn then_colongt(world: &mut LexerWorld) {
    assert_eq!(
        world.result.first().map(|t| &t.kind),
        Some(&TokenKind::ColonGt)
    );
}

#[then("the first token kind is a string")]
fn then_string(world: &mut LexerWorld) {
    assert!(
        matches!(
            world.result.first().map(|t| &t.kind),
            Some(TokenKind::Str(_))
        ),
        "expected Str token, got {:?}",
        world.result.first().map(|t| &t.kind)
    );
}

#[then("the first token kind is an integer")]
fn then_integer(world: &mut LexerWorld) {
    assert!(
        matches!(
            world.result.first().map(|t| &t.kind),
            Some(TokenKind::Int(_))
        ),
        "expected Int token, got {:?}",
        world.result.first().map(|t| &t.kind)
    );
}

#[then("the first token kind is an identifier")]
fn then_ident(world: &mut LexerWorld) {
    assert!(
        matches!(
            world.result.first().map(|t| &t.kind),
            Some(TokenKind::Ident(_))
        ),
        "expected Ident token, got {:?}",
        world.result.first().map(|t| &t.kind)
    );
}

#[then("the first token kind is Hash")]
fn then_hash(world: &mut LexerWorld) {
    assert_eq!(
        world.result.first().map(|t| &t.kind),
        Some(&TokenKind::Hash)
    );
}

#[then("tokenization fails")]
fn then_fails(world: &mut LexerWorld) {
    assert!(
        world.error_msg.is_some(),
        "expected tokenization failure, but got tokens: {:?}",
        world.result
    );
}

#[tokio::main]
async fn main() {
    LexerWorld::run("tests/features/lexer.feature").await;
}
