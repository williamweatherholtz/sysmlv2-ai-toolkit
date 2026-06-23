#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use cucumber::{given, then, when, World};
use keel_parser::{ast::{Item, Value}, parse, tokenize};
use keel_parser::ast::Item::TypeDef as ItemTypeDef;

#[derive(Debug, Default, World)]
pub struct ParserWorld {
    source: String,
    package: Option<keel_parser::ast::Package>,
    error_msg: Option<String>,
}

#[given(expr = "the parser source {string}")]
fn given_source(world: &mut ParserWorld, source: String) {
    world.source = source.replace("\\n", "\n");
}

#[when("I parse the source")]
fn when_parse(world: &mut ParserWorld) {
    match tokenize(&world.source, "test") {
        Err(e) => {
            world.package = None;
            world.error_msg = Some(e.to_string());
        }
        Ok(tokens) => match parse(tokens, "test") {
            Ok(pkg) => {
                world.package = Some(pkg);
                world.error_msg = None;
            }
            Err(e) => {
                world.package = None;
                world.error_msg = Some(e.to_string());
            }
        },
    }
}

#[then("parsing succeeds")]
fn then_succeeds(world: &mut ParserWorld) {
    assert!(
        world.package.is_some(),
        "expected parse success, got error: {:?}",
        world.error_msg
    );
}

#[then("parsing fails")]
fn then_fails(world: &mut ParserWorld) {
    assert!(
        world.error_msg.is_some(),
        "expected parse error, but parsing succeeded with package: {:?}",
        world.package
    );
}

#[then(expr = "the package name is {string}")]
fn then_package_name(world: &mut ParserWorld, name: String) {
    let pkg = world.package.as_ref().expect("no package");
    assert_eq!(pkg.name, name);
}

#[then(expr = "the package has {int} items")]
fn then_item_count(world: &mut ParserWorld, count: usize) {
    let pkg = world.package.as_ref().expect("no package");
    assert_eq!(pkg.items.len(), count, "items: {:?}", pkg.items);
}

#[then(expr = "the first attribute of the first part is {string} with value {string}")]
fn then_first_part_attr(world: &mut ParserWorld, attr_name: String, expected: String) {
    let pkg = world.package.as_ref().expect("no package");
    let Item::Part(part) = pkg.items.first().expect("no items") else {
        panic!("first item is not a Part");
    };
    let attr = part.attributes.first().expect("no attributes");
    assert_eq!(attr.name, attr_name);
    let Value::Str(actual) = &attr.value else {
        panic!("attribute value is not a string: {:?}", attr.value);
    };
    assert_eq!(*actual, expected);
}

#[then(expr = "the first item is a TypeDef named {string}")]
fn then_first_is_typedef(world: &mut ParserWorld, name: String) {
    let pkg = world.package.as_ref().expect("no package");
    let item = pkg.items.first().expect("no items");
    let ItemTypeDef(td) = item else {
        panic!("first item is not a TypeDef: {:?}", item);
    };
    assert_eq!(td.name, name);
}

#[tokio::main]
async fn main() {
    ParserWorld::run("tests/features/parser.feature").await;
}
