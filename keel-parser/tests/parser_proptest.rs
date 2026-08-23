use proptest::prelude::*;
use keel_parser::ast::{Item, Value};
use keel_parser::{parse, tokenize};

fn arb_identifier() -> impl Strategy<Value = String> {
    // Reserved words are not valid BARE identifiers (issue225: CI's random seed generated `doc` and
    // the parser correctly refused it). Filtered through the lexer's own predicate, never a copied
    // list, so the strategy can't drift from the grammar.
    "[a-zA-Z_][a-zA-Z0-9_]{0,31}".prop_filter("bare identifiers exclude reserved words", |s| !keel_parser::is_reserved_word(s))
}

fn arb_simple_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 _]{0,32}".prop_map(std::convert::identity)
}

fn parse_src(src: &str) -> Result<keel_parser::ast::Package, String> {
    let tokens = tokenize(src, "prop-test").map_err(|e| e.to_string())?;
    parse(tokens, "prop-test").map_err(|e| e.to_string())
}

proptest! {
    /// Any valid identifier is accepted as a package name.
    #[test]
    fn prop_package_name_roundtrip(name in arb_identifier()) {
        let src = format!("package {name} {{}}");
        let pkg = parse_src(&src).expect("should parse");
        prop_assert_eq!(&pkg.name, &name);
        prop_assert_eq!(pkg.items.len(), 0);
    }

    /// A part with a string attribute always produces one Part item whose
    /// attribute value matches the original string.
    #[test]
    fn prop_part_string_attr(
        pkg_name in arb_identifier(),
        part_name in arb_identifier(),
        attr_val in arb_simple_string()
    ) {
        let src = format!(
            "package {pkg_name} {{ part {part_name} : T {{ :>> title = \"{attr_val}\"; }} }}"
        );
        let pkg = parse_src(&src).expect("should parse");
        prop_assert_eq!(pkg.items.len(), 1);
        let Item::Part(part) = &pkg.items[0] else {
            prop_assert!(false, "expected Part item");
            return Ok(());
        };
        prop_assert_eq!(&part.attributes[0].name, "title");
        prop_assert!(
            matches!(&part.attributes[0].value, Value::Str(s) if s == &attr_val),
            "expected Str({attr_val:?}), got {:?}", part.attributes[0].value
        );
    }

    /// String concatenation with two segments produces the concatenated value.
    #[test]
    fn prop_string_concat(left in arb_simple_string(), right in arb_simple_string()) {
        let src = format!(
            "package P {{ part d : Decision {{ :>> ctx = \"{left}\" + \"{right}\"; }} }}"
        );
        let pkg = parse_src(&src).expect("should parse");
        let Item::Part(part) = &pkg.items[0] else {
            prop_assert!(false, "expected Part");
            return Ok(());
        };
        let expected = format!("{left}{right}");
        prop_assert!(
            matches!(&part.attributes[0].value, Value::Str(s) if s == &expected),
            "expected Str({expected:?}), got {:?}", part.attributes[0].value
        );
    }
}

/// issue225 pinned deterministically: a reserved word as a bare part name is a parse ERROR (the
/// behavior CI's random seed found), and the lexer's own predicate is what names the reserved set.
#[test]
fn reserved_word_is_not_a_bare_identifier() {
    assert!(keel_parser::is_reserved_word("doc"));
    assert!(keel_parser::is_reserved_word("dependency"));
    assert!(!keel_parser::is_reserved_word("docs"));
    assert!(parse_src("package P { part doc : T { } }").is_err());
}

/// issue231's second half (the ShardStore lightweight-formal-methods pattern): any composition of
/// SUPPORTED statement forms parses with ZERO skipped statements — skip-freedom over generated
/// corpora, so a parser regression that silently drops a supported form is caught by generation,
/// not by whoever next greps the census.
fn arb_supported_statement() -> impl Strategy<Value = String> {
    let ident = arb_identifier();
    prop_oneof![
        (arb_identifier(), arb_identifier(), arb_simple_string()).prop_map(|(n, t, v)| format!(
            "part {n} : {t} {{ :>> title = \"{v}\"; }}"
        )),
        (arb_identifier(), arb_identifier()).prop_map(|(n, t)| format!(
            "verification {n} : {t} {{ :>> id = \"aaaaaaaa-0000-4000-9000-aaaaaaaaaaaa\"; }}"
        )),
        (arb_identifier(), arb_identifier()).prop_map(|(a, b)| format!("dependency from {a} to {b};")),
        (arb_identifier(), arb_identifier()).prop_map(|(a, b)| format!("#Verify dependency from {a} to {b};")),
        (arb_identifier(), arb_identifier()).prop_map(|(a, b)| format!("satisfy {a} by {b};")),
        (arb_identifier(), arb_identifier()).prop_map(|(a, b)| format!("allocate {a} to {b};")),
        ident.prop_map(|n| format!("action {n};")),
    ]
}

proptest! {
    #[test]
    fn supported_statements_are_never_skipped(
        pkg_name in arb_identifier(),
        stmts in proptest::collection::vec(arb_supported_statement(), 1..12)
    ) {
        let src = format!("package {pkg_name} {{\n    {}\n}}", stmts.join("\n    "));
        let pkg = parse_src(&src).expect("supported forms must parse");
        prop_assert!(
            pkg.skipped.is_empty(),
            "supported statement silently skipped in:\n{src}\nskipped: {:?}",
            pkg.skipped.iter().map(|s| (&s.lead, s.line)).collect::<Vec<_>>()
        );
    }
}
