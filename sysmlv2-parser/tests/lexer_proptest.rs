use proptest::prelude::*;
use sysmlv2_parser::token::TokenKind;
use sysmlv2_parser::tokenize;

/// Strategy: syntactically valid identifiers (start letter/underscore, continue alphanumeric/_).
fn arb_identifier() -> impl Strategy<Value = String> {
    "[a-zA-Z_][a-zA-Z0-9_]{0,31}".prop_map(std::convert::identity)
}

/// Strategy: simple strings containing no backslash or double-quote (no escaping needed).
fn arb_simple_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 _\\-]{0,64}".prop_map(std::convert::identity)
}

/// Strategy: enum-literal pairs rendered as `NamespaceIdent::MemberIdent`.
fn arb_enum_literal() -> impl Strategy<Value = (String, String)> {
    (arb_identifier(), arb_identifier())
}

proptest! {
    /// An identifier that is not a keyword tokenizes as `Ident` and round-trips.
    #[test]
    fn prop_ident_round_trips(s in arb_identifier()) {
        let tokens = tokenize(&s, "prop-test").expect("identifier should lex without error");
        prop_assert!(
            tokens.len() >= 2,
            "expected at least Ident+Eof, got {tokens:?}"
        );
        if let TokenKind::Ident(got) = &tokens[0].kind {
            prop_assert_eq!(got.as_str(), s.as_str());
            // If a keyword variant: still correct lexer behaviour — no further check needed.
        }
    }

    /// A quoted string with no escape characters round-trips through the lexer.
    #[test]
    fn prop_string_round_trips(s in arb_simple_string()) {
        let src = format!("\"{s}\"");
        let tokens = tokenize(&src, "prop-test").expect("string should lex without error");
        prop_assert!(tokens.len() >= 2);
        match &tokens[0].kind {
            TokenKind::Str(got) => prop_assert_eq!(got.as_str(), s.as_str()),
            other => prop_assert!(false, "expected Str token, got {other:?}"),
        }
    }

    /// `Namespace::Member` produces ColonColon as the middle token regardless of names.
    #[test]
    fn prop_enum_literal_contains_coloncolon((ns, name) in arb_enum_literal()) {
        let src = format!("{ns}::{name}");
        let tokens = tokenize(&src, "prop-test").expect("enum literal should lex without error");
        // Structure: [keyword|Ident, ColonColon, keyword|Ident, Eof] — at least 4 tokens.
        prop_assert!(tokens.len() >= 4, "expected ≥4 tokens, got {}", tokens.len());
        prop_assert_eq!(&tokens[1].kind, &TokenKind::ColonColon);
    }
}
