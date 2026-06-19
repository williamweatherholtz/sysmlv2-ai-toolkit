use crate::ast::{
    ActionDecl, ActionDef, AllocateEdge, Attribute, DependencyAnnotation, EnumDef, Import, Item,
    Package, Part, SatisfyEdge, Succession, TypeDef, Value, Verification,
};
use crate::error::ParseError;
use crate::token::{Span, Token, TokenKind};

// ── parser state ───────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    #[allow(clippy::missing_const_for_fn)] // Vec is not const-constructible in stable Rust
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &TokenKind {
        self.tokens.get(self.pos).map_or(&TokenKind::Eof, |t| &t.kind)
    }

    fn peek_next(&self) -> &TokenKind {
        self.tokens
            .get(self.pos + 1)
            .map_or(&TokenKind::Eof, |t| &t.kind)
    }

    // INVARIANT: self.pos is always in-bounds — the lexer always appends an Eof token and
    // advance() saturates at the last index (never steps past Eof). So indexing at self.pos
    // cannot panic; clippy can't see the invariant, hence the localized allow (D0074 fail-loud:
    // documented-safe indexing, not a silent unchecked access).
    #[allow(clippy::indexing_slicing)]
    fn peek_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    #[allow(clippy::indexing_slicing)]
    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn current_span(&self) -> Span {
        self.peek_token().span
    }

    fn expect_ident(&mut self, filename: &str) -> Result<(String, Span), ParseError> {
        let tok = self.peek_token().clone();
        match &tok.kind {
            TokenKind::Ident(s) => {
                let name = s.clone();
                self.advance();
                Ok((name, tok.span))
            }
            kw if is_keyword(kw) => {
                let name = keyword_text(kw).to_owned();
                self.advance();
                Ok((name, tok.span))
            }
            other => Err(ParseError::Expected {
                expected: "identifier".into(),
                got: format!("{other:?}").into(),
                filename: filename.into(),
                line: tok.line,
                col: tok.col,
            }),
        }
    }

    fn expect(&mut self, kind: &TokenKind, filename: &str) -> Result<Span, ParseError> {
        let tok = self.peek_token().clone();
        if &tok.kind == kind {
            self.advance();
            Ok(tok.span)
        } else {
            Err(ParseError::Expected {
                expected: format!("{kind:?}").into(),
                got: format!("{:?}", tok.kind).into(),
                filename: filename.into(),
                line: tok.line,
                col: tok.col,
            })
        }
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.peek() == kind {
            self.advance();
            true
        } else {
            false
        }
    }
}

// ── helpers ────────────────────────────────────────────────────────────────

const fn is_keyword(k: &TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Package
            | TokenKind::Private
            | TokenKind::Import
            | TokenKind::Part
            | TokenKind::Action
            | TokenKind::Def
            | TokenKind::Verification
            | TokenKind::Requirement
            | TokenKind::Use
            | TokenKind::Case
            | TokenKind::Attribute
            | TokenKind::Enum
            | TokenKind::Abstract
            | TokenKind::First
            | TokenKind::Then
            | TokenKind::Satisfy
            | TokenKind::Allocate
            | TokenKind::By
            | TokenKind::To
            | TokenKind::From
            | TokenKind::Dependency
    )
}

const fn keyword_text(k: &TokenKind) -> &'static str {
    match k {
        TokenKind::Package => "package",
        TokenKind::Private => "private",
        TokenKind::Import => "import",
        TokenKind::Part => "part",
        TokenKind::Action => "action",
        TokenKind::Def => "def",
        TokenKind::Verification => "verification",
        TokenKind::Requirement => "requirement",
        TokenKind::Use => "use",
        TokenKind::Case => "case",
        TokenKind::Attribute => "attribute",
        TokenKind::Enum => "enum",
        TokenKind::Abstract => "abstract",
        TokenKind::First => "first",
        TokenKind::Then => "then",
        TokenKind::Satisfy => "satisfy",
        TokenKind::Allocate => "allocate",
        TokenKind::By => "by",
        TokenKind::To => "to",
        TokenKind::From => "from",
        TokenKind::Dependency => "dependency",
        _ => "",
    }
}

// ── value parser ──────────────────────────────────────────────────────────

fn parse_value(p: &mut Parser, filename: &str) -> Result<Value, ParseError> {
    let tok = p.peek_token().clone();
    match &tok.kind {
        TokenKind::Str(s) => {
            let mut value = s.clone();
            p.advance();
            while p.eat(&TokenKind::Plus) {
                if let TokenKind::Str(s2) = p.peek().clone() {
                    value.push_str(&s2);
                    p.advance();
                }
            }
            Ok(Value::Str(value))
        }
        TokenKind::Int(n) => {
            let v = Value::Int(*n);
            p.advance();
            Ok(v)
        }
        TokenKind::Ident(s) => {
            let name = s.clone();
            p.advance();
            if p.eat(&TokenKind::ColonColon) {
                let (member, _) = p.expect_ident(filename)?;
                Ok(Value::EnumLit { namespace: name, member })
            } else {
                Ok(Value::Ident(name))
            }
        }
        kw if is_keyword(kw) => {
            let name = keyword_text(kw).to_owned();
            p.advance();
            if p.eat(&TokenKind::ColonColon) {
                let (member, _) = p.expect_ident(filename)?;
                Ok(Value::EnumLit { namespace: name, member })
            } else {
                Ok(Value::Ident(name))
            }
        }
        other => Err(ParseError::Expected {
            expected: "value (string, integer, identifier, or enum literal)".into(),
            got: format!("{other:?}").into(),
            filename: filename.into(),
            line: tok.line,
            col: tok.col,
        }),
    }
}

// ── attribute body ─────────────────────────────────────────────────────────

fn parse_attribute_body(
    p: &mut Parser,
    filename: &str,
    start: Span,
) -> Result<(Vec<Attribute>, Span), ParseError> {
    p.expect(&TokenKind::LBrace, filename)?;
    let mut attrs = Vec::new();
    loop {
        match p.peek() {
            TokenKind::RBrace => { p.advance(); break; }
            TokenKind::Eof => {
                let tok = p.peek_token();
                return Err(ParseError::UnexpectedEof {
                    filename: filename.into(),
                    line: tok.line,
                    col: tok.col,
                });
            }
            TokenKind::ColonGtGt => {
                let attr_start = p.current_span();
                let attr_line = p.peek_token().line;
                p.advance();
                let (name, _) = p.expect_ident(filename)?;
                p.expect(&TokenKind::Eq, filename)?;
                let value = parse_value(p, filename)?;
                let end = p.current_span();
                p.expect(&TokenKind::Semicolon, filename)?;
                attrs.push(Attribute {
                    name,
                    value,
                    span: Span { start: attr_start.start, end: end.start },
                    line: attr_line,
                });
            }
            _ => { skip_item(p); }
        }
    }
    let end_span = p.current_span();
    Ok((attrs, Span { start: start.start, end: end_span.start }))
}

fn parse_typed_item_body(
    p: &mut Parser,
    filename: &str,
    item_start: Span,
) -> Result<(String, Option<String>, Vec<Attribute>, Span), ParseError> {
    let (name, _) = p.expect_ident(filename)?;
    let type_name = if p.eat(&TokenKind::Colon) {
        let (tn, _) = p.expect_ident(filename)?;
        if matches!(p.peek(), TokenKind::LBracket) {
            skip_bracket_block(p);
        }
        Some(tn)
    } else {
        None
    };
    let (attrs, span) = parse_attribute_body(p, filename, item_start)?;
    Ok((name, type_name, attrs, span))
}

// ── action def body ───────────────────────────────────────────────────────

fn parse_action_def_body(
    p: &mut Parser,
    filename: &str,
    name: String,
    start: Span,
) -> Result<ActionDef, ParseError> {
    p.expect(&TokenKind::LBrace, filename)?;
    let mut actions = Vec::new();
    let mut parts = Vec::new();
    let mut verifications = Vec::new();
    let mut successions = Vec::new();
    loop {
        match p.peek().clone() {
            TokenKind::RBrace => { p.advance(); break; }
            TokenKind::Eof => {
                let tok = p.peek_token();
                return Err(ParseError::UnexpectedEof {
                    filename: filename.into(), line: tok.line, col: tok.col,
                });
            }
            TokenKind::Action => {
                let s = p.current_span(); p.advance();
                let (n, _) = p.expect_ident(filename)?;
                if matches!(p.peek(), TokenKind::LBrace) {
                    skip_brace_block(p);
                } else {
                    p.expect(&TokenKind::Semicolon, filename)?;
                }
                actions.push(ActionDecl { name: n, span: s });
            }
            TokenKind::Part => {
                let item_line = p.peek_token().line;
                let s = p.current_span(); p.advance();
                let (n, tn, a, sp) = parse_typed_item_body(p, filename, s)?;
                parts.push(Part { name: n, type_name: tn, attributes: a, span: sp, line: item_line });
            }
            TokenKind::Verification => {
                let item_line = p.peek_token().line;
                let s = p.current_span(); p.advance();
                let (n, tn, a, sp) = parse_typed_item_body(p, filename, s)?;
                verifications.push(Verification { name: n, type_name: tn, attributes: a, span: sp, line: item_line });
            }
            TokenKind::First => {
                if let Some(s) = parse_succession(p, filename, false)? { successions.push(s); }
            }
            TokenKind::Hash => {
                p.advance();
                let (marker, _) = p.expect_ident(filename)?;
                if matches!(p.peek(), TokenKind::First) {
                    let is_oo = marker == "OrderingOnly";
                    if let Some(s) = parse_succession(p, filename, is_oo)? { successions.push(s); }
                } else {
                    skip_item(p);
                }
            }
            _ => skip_item(p),
        }
    }
    let end = p.current_span();
    Ok(ActionDef { name, actions, parts, verifications, successions,
        span: Span { start: start.start, end: end.start } })
}

fn parse_succession(p: &mut Parser, filename: &str, is_ordering_only: bool) -> Result<Option<Succession>, ParseError> {
    let s = p.current_span();
    p.advance(); // consume 'first'
    let (first, _) = p.expect_ident(filename)?;
    p.expect(&TokenKind::Then, filename)?;
    let (then, _) = p.expect_ident(filename)?;
    let end = p.current_span();
    p.expect(&TokenKind::Semicolon, filename)?;
    Ok(Some(Succession { first, then, is_ordering_only, span: Span { start: s.start, end: end.start } }))
}

/// Skip one unknown item. Stops (and consumes) on `;`; stops (without extra
/// consume) when a `{...}` block ends the construct; stops without consuming
/// on `}` or EOF (the caller's closing delimiter).
fn skip_item(p: &mut Parser) {
    loop {
        match p.peek() {
            TokenKind::Semicolon => { p.advance(); break; }
            TokenKind::RBrace | TokenKind::Eof => break,
            TokenKind::LBrace => { skip_brace_block(p); break; }
            TokenKind::LBracket => skip_bracket_block(p),
            _ => { p.advance(); }
        }
    }
}

fn skip_bracket_block(p: &mut Parser) {
    p.advance(); // consume '['
    while !matches!(p.peek(), TokenKind::RBracket | TokenKind::Eof) {
        p.advance();
    }
    p.eat(&TokenKind::RBracket);
}

fn skip_brace_block(p: &mut Parser) {
    p.advance(); // consume '{'
    let mut depth = 1usize;
    loop {
        match p.peek() {
            TokenKind::LBrace => { depth += 1; p.advance(); }
            TokenKind::RBrace => {
                depth -= 1; p.advance();
                if depth == 0 { break; }
            }
            TokenKind::Eof => break,
            _ => { p.advance(); }
        }
    }
}

/// Extract an ident-or-keyword name at the current position and advance.
/// Returns `None` (without advancing) when the current token is neither.
fn extract_ident_name(p: &mut Parser) -> Option<String> {
    match p.peek().clone() {
        TokenKind::Ident(s) => { p.advance(); Some(s) }
        kw if is_keyword(&kw) => { let n = keyword_text(&kw).to_owned(); p.advance(); Some(n) }
        _ => None,
    }
}

/// Parse an enum body `{ member1; member2; ... }` and return member names.
/// Tolerant: skips anything that is not an ident/keyword followed by `;`.
fn parse_enum_body(p: &mut Parser) -> Vec<String> {
    let mut members = Vec::new();
    if !p.eat(&TokenKind::LBrace) { return members; }
    loop {
        match p.peek().clone() {
            TokenKind::RBrace | TokenKind::Eof => {
                p.eat(&TokenKind::RBrace);
                break;
            }
            TokenKind::Semicolon => { p.advance(); }
            TokenKind::Ident(s) => {
                members.push(s);
                p.advance();
                p.eat(&TokenKind::Semicolon);
            }
            kw if is_keyword(&kw) => {
                members.push(keyword_text(&kw).to_owned());
                p.advance();
                p.eat(&TokenKind::Semicolon);
            }
            _ => { p.advance(); }
        }
    }
    members
}

// ── package-level item parsers ─────────────────────────────────────────────

fn parse_import(p: &mut Parser, filename: &str) -> Result<Import, ParseError> {
    let s = p.current_span();
    let import_line = p.peek_token().line;
    p.advance(); // consume 'private'
    p.expect(&TokenKind::Import, filename)?;
    let (ns, _) = p.expect_ident(filename)?;
    p.expect(&TokenKind::ColonColon, filename)?;
    let end = p.current_span();
    p.expect(&TokenKind::Star, filename)?;
    p.expect(&TokenKind::Semicolon, filename)?;
    Ok(Import { namespace: ns, span: Span { start: s.start, end: end.start }, line: import_line })
}

fn parse_satisfy(p: &mut Parser, filename: &str) -> Result<SatisfyEdge, ParseError> {
    let s = p.current_span(); p.advance(); // consume 'satisfy'
    let (need, _) = p.expect_ident(filename)?;
    p.expect(&TokenKind::By, filename)?;
    let (by, _) = p.expect_ident(filename)?;
    let end = p.current_span();
    p.expect(&TokenKind::Semicolon, filename)?;
    Ok(SatisfyEdge { need, by, span: Span { start: s.start, end: end.start } })
}

fn parse_allocate(p: &mut Parser, filename: &str) -> Result<AllocateEdge, ParseError> {
    let s = p.current_span(); p.advance(); // consume 'allocate'
    let (sr, _) = p.expect_ident(filename)?;
    p.expect(&TokenKind::To, filename)?;
    let (to, _) = p.expect_ident(filename)?;
    let end = p.current_span();
    p.expect(&TokenKind::Semicolon, filename)?;
    Ok(AllocateEdge { sr, to, span: Span { start: s.start, end: end.start } })
}

fn parse_hash_item(
    p: &mut Parser,
    filename: &str,
) -> Result<Option<Item>, ParseError> {
    let s = p.current_span(); p.advance(); // consume '#'
    let (marker, _) = p.expect_ident(filename)?;
    // `#Marker first X then Y;` — succession with ordering-only flag
    if matches!(p.peek(), TokenKind::First) {
        let is_oo = marker == "OrderingOnly";
        return parse_succession(p, filename, is_oo).map(|opt| opt.map(Item::Succession));
    }
    if !matches!(p.peek(), TokenKind::Dependency) {
        // `#Marker part X : T { ... }` (D0070 marker-on-a-definition, e.g. a process-change
        // Decision): parse + EMIT the underlying item so it is not silently dropped. The marker
        // annotation itself is not yet retained in the AST (tracked: a rust `process-changes`
        // view, M2, will need it). Without this, marked items vanished from every view (issue024).
        return parse_item(p, filename);
    }
    p.advance(); // consume 'dependency'
    p.expect(&TokenKind::From, filename)?;
    let (from, _) = p.expect_ident(filename)?;
    p.expect(&TokenKind::To, filename)?;
    let (to, _) = p.expect_ident(filename)?;
    let end = p.current_span();
    p.expect(&TokenKind::Semicolon, filename)?;
    Ok(Some(Item::Dependency(DependencyAnnotation {
        marker, from, to,
        span: Span { start: s.start, end: end.start },
    })))
}

fn parse_action_item(p: &mut Parser, filename: &str) -> Result<Option<Item>, ParseError> {
    let s = p.current_span(); p.advance(); // consume 'action'
    if p.eat(&TokenKind::Def) {
        let (n, _) = p.expect_ident(filename)?;
        let adef = parse_action_def_body(p, filename, n, s)?;
        Ok(Some(Item::ActionDef(adef)))
    } else {
        let (n, _) = p.expect_ident(filename)?;
        p.expect(&TokenKind::Semicolon, filename)?;
        Ok(Some(Item::ActionDecl(ActionDecl { name: n, span: s })))
    }
}

// ── top-level dispatch ─────────────────────────────────────────────────────

fn parse_item(p: &mut Parser, filename: &str) -> Result<Option<Item>, ParseError> {
    let start = p.current_span();
    let start_line = p.peek_token().line;

    // `abstract` only prefixes type definitions; consume and continue matching.
    let had_abstract = if matches!(p.peek(), TokenKind::Abstract) {
        p.advance();
        true
    } else {
        false
    };

    match p.peek().clone() {
        TokenKind::RBrace | TokenKind::Eof => Ok(None),

        TokenKind::Doc => { p.advance(); Ok(None) }

        // These items never appear after `abstract`.
        TokenKind::Private if !had_abstract =>
            parse_import(p, filename).map(|i| Some(Item::Import(i))),

        TokenKind::Action if !had_abstract => parse_action_item(p, filename),

        TokenKind::First if !had_abstract =>
            parse_succession(p, filename, false).map(|s| s.map(Item::Succession)),

        TokenKind::Satisfy if !had_abstract =>
            parse_satisfy(p, filename).map(|s| Some(Item::Satisfy(s))),

        TokenKind::Allocate if !had_abstract =>
            parse_allocate(p, filename).map(|a| Some(Item::Allocate(a))),

        TokenKind::Hash if !had_abstract => parse_hash_item(p, filename),

        // `enum def Name { member; ... }` → EnumDef (Sprint 3)
        TokenKind::Enum => {
            p.advance(); // consume 'enum'
            if matches!(p.peek(), TokenKind::Def) {
                p.advance(); // consume 'def'
                if let Some(name) = extract_ident_name(p) {
                    let members = parse_enum_body(p);
                    return Ok(Some(Item::EnumDef(EnumDef { name, members, span: start, line: start_line })));
                }
            }
            skip_item(p);
            Ok(None)
        }

        // `part def Name ...` → TypeDef; `part Name ...` → Part instance
        TokenKind::Part => {
            if matches!(p.peek_next(), TokenKind::Def) {
                p.advance(); p.advance(); // consume 'part' 'def'
                let name = extract_ident_name(p);
                skip_item(p);
                return Ok(name.map(|n| Item::TypeDef(TypeDef { name: n, span: start, line: start_line })));
            }
            if had_abstract { skip_item(p); return Ok(None); }
            let s = p.current_span(); p.advance();
            let (n, tn, a, sp) = parse_typed_item_body(p, filename, s)?;
            Ok(Some(Item::Part(Part { name: n, type_name: tn, attributes: a, span: sp, line: start_line })))
        }

        // `verification def Name ...` → TypeDef; `verification Name ...` → Verification instance
        TokenKind::Verification => {
            if matches!(p.peek_next(), TokenKind::Def) {
                p.advance(); p.advance(); // consume 'verification' 'def'
                let name = extract_ident_name(p);
                skip_item(p);
                return Ok(name.map(|n| Item::TypeDef(TypeDef { name: n, span: start, line: start_line })));
            }
            if had_abstract { skip_item(p); return Ok(None); }
            let s = p.current_span(); p.advance();
            let (n, tn, a, sp) = parse_typed_item_body(p, filename, s)?;
            Ok(Some(Item::Verification(Verification { name: n, type_name: tn, attributes: a, span: sp, line: start_line })))
        }

        // `attribute def Name ...` → TypeDef
        TokenKind::Attribute => {
            p.advance(); // consume 'attribute'
            if matches!(p.peek(), TokenKind::Def) {
                p.advance();
                let name = extract_ident_name(p);
                skip_item(p);
                return Ok(name.map(|n| Item::TypeDef(TypeDef { name: n, span: start, line: start_line })));
            }
            skip_item(p);
            Ok(None)
        }

        // `requirement def Name ...` → TypeDef
        TokenKind::Requirement => {
            p.advance();
            if matches!(p.peek(), TokenKind::Def) {
                p.advance();
                let name = extract_ident_name(p);
                skip_item(p);
                return Ok(name.map(|n| Item::TypeDef(TypeDef { name: n, span: start, line: start_line })));
            }
            skip_item(p);
            Ok(None)
        }

        // `use case def Name ...` → TypeDef
        TokenKind::Use => {
            p.advance(); // consume 'use'
            if matches!(p.peek(), TokenKind::Case) { p.advance(); } // consume 'case'
            if matches!(p.peek(), TokenKind::Def) {
                p.advance();
                let name = extract_ident_name(p);
                skip_item(p);
                return Ok(name.map(|n| Item::TypeDef(TypeDef { name: n, span: start, line: start_line })));
            }
            skip_item(p);
            Ok(None)
        }

        // Generic `<classifier-kind> def Name ...` where the kind word lexes as an
        // identifier rather than a dedicated keyword — e.g. `occurrence def TestResult`
        // (D0032 retype), `connection def`, `port def`. Register the defined name as a
        // TypeDef so `: Name` references resolve (bug fix, Sprint 17 / issue005).
        TokenKind::Ident(_) if matches!(p.peek_next(), TokenKind::Def) => {
            p.advance(); p.advance(); // consume <ident> 'def'
            let name = extract_ident_name(p);
            skip_item(p);
            Ok(name.map(|n| Item::TypeDef(TypeDef { name: n, span: start, line: start_line })))
        }

        _ => { skip_item(p); Ok(None) }
    }
}

fn parse_package(p: &mut Parser, filename: &str) -> Result<Package, ParseError> {
    let pkg_start = p.current_span();
    p.expect(&TokenKind::Package, filename)?;
    let (pkg_name, _) = p.expect_ident(filename)?;
    p.expect(&TokenKind::LBrace, filename)?;
    let mut items = Vec::new();
    loop {
        if matches!(p.peek(), TokenKind::RBrace | TokenKind::Eof) { break; }
        if let Some(item) = parse_item(p, filename)? { items.push(item); }
    }
    let end = p.expect(&TokenKind::RBrace, filename)?;
    Ok(Package { name: pkg_name, items, span: Span { start: pkg_start.start, end: end.start } })
}

// ── public entry point ─────────────────────────────────────────────────────

/// Parse a `SysML` v2 engine-dialect token stream into a [`Package`] AST.
///
/// # Errors
///
/// Returns [`ParseError`] when the token stream does not conform to the
/// engine-dialect grammar.
pub fn parse(tokens: Vec<Token>, filename: &str) -> Result<Package, ParseError> {
    let mut p = Parser::new(tokens);
    parse_package(&mut p, filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::Item, tokenize};

    fn parse_src(src: &str) -> Package {
        let tokens = tokenize(src, "test").expect("lex failed");
        parse(tokens, "test").expect("parse failed")
    }

    #[test]
    fn empty_package() {
        let pkg = parse_src("package Foo {}");
        assert_eq!(pkg.name, "Foo");
        assert!(pkg.items.is_empty());
    }

    #[test]
    fn import_item() {
        let pkg = parse_src("package P { private import Foo::*; }");
        assert_eq!(pkg.items.len(), 1);
        assert!(matches!(&pkg.items[0], Item::Import(i) if i.namespace == "Foo"));
    }

    #[test]
    fn marker_prefixed_part_is_preserved() {
        // issue024: `#Marker part X : T { ... }` (D0070 marker-on-a-definition) must EMIT the
        // part, not silently skip it. Marker is consumed; the underlying Part survives.
        let src = r#"package P {
            #ProspectiveChange part d0049 : Decision {
                :>> id = "x";
                :>> status = DecisionStatus::accepted;
            }
        }"#;
        let pkg = parse_src(src);
        assert_eq!(pkg.items.len(), 1, "marked part must not be dropped");
        let Item::Part(part) = &pkg.items[0] else { panic!("expected Part from #Marker part") };
        assert_eq!(part.name, "d0049");
        assert_eq!(part.type_name.as_deref(), Some("Decision"));
    }

    #[test]
    fn part_with_attributes() {
        let src = r#"package P {
            part myPart : Story {
                :>> id = "abc-123";
                :>> estimatedPoints = 2;
                :>> kind = WorkKind::code;
            }
        }"#;
        let pkg = parse_src(src);
        assert_eq!(pkg.items.len(), 1);
        let Item::Part(part) = &pkg.items[0] else { panic!("expected Part") };
        assert_eq!(part.name, "myPart");
        assert_eq!(part.type_name.as_deref(), Some("Story"));
        assert_eq!(part.attributes.len(), 3);
        assert_eq!(part.attributes[0].name, "id");
        assert!(matches!(&part.attributes[0].value, Value::Str(s) if s == "abc-123"));
        assert!(matches!(&part.attributes[1].value, Value::Int(2)));
        assert!(
            matches!(&part.attributes[2].value, Value::EnumLit { namespace, member }
                if namespace == "WorkKind" && member == "code")
        );
    }

    #[test]
    fn verification_item() {
        let src = r#"package P {
            verification myTest : Test {
                :>> method = VerificationMethod::test;
                :>> procedureText = "do the thing";
            }
        }"#;
        let pkg = parse_src(src);
        let Item::Verification(v) = &pkg.items[0] else { panic!("expected Verification") };
        assert_eq!(v.name, "myTest");
        assert_eq!(v.type_name.as_deref(), Some("Test"));
        assert_eq!(v.attributes.len(), 2);
    }

    #[test]
    fn succession_item() {
        let pkg = parse_src("package P { first alpha then beta; }");
        assert!(matches!(&pkg.items[0], Item::Succession(s) if s.first == "alpha" && s.then == "beta"));
    }

    #[test]
    fn satisfy_edge() {
        let pkg = parse_src("package P { satisfy n1 by sr1; }");
        assert!(matches!(&pkg.items[0], Item::Satisfy(s) if s.need == "n1" && s.by == "sr1"));
    }

    #[test]
    fn allocate_edge() {
        let pkg = parse_src("package P { allocate sr1 to comp1; }");
        assert!(matches!(&pkg.items[0], Item::Allocate(a) if a.sr == "sr1" && a.to == "comp1"));
    }

    #[test]
    fn dependency_annotation() {
        let pkg = parse_src("package P { #DependsOn dependency from a to b; }");
        let Item::Dependency(d) = &pkg.items[0] else { panic!("expected Dependency") };
        assert_eq!(d.marker, "DependsOn");
        assert_eq!(d.from, "a");
        assert_eq!(d.to, "b");
    }

    #[test]
    fn action_def_with_nested_items() {
        let src = r#"package P {
            action def MyDef {
                action taskA;
                action taskB;
                first taskA then taskB;
                verification myDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "ok"; }
                part myResult : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abc"; :>> judgedAt = "2026-01-01"; :>> judgedBy = "user"; }
            }
        }"#;
        let pkg = parse_src(src);
        let Item::ActionDef(adef) = &pkg.items[0] else { panic!("expected ActionDef") };
        assert_eq!(adef.name, "MyDef");
        assert_eq!(adef.actions.len(), 2);
        assert_eq!(adef.successions.len(), 1);
        assert_eq!(adef.verifications.len(), 1);
        assert_eq!(adef.parts.len(), 1);
    }

    #[test]
    fn line_comment_inside_package() {
        let src = "package P {\n// this is a comment\npart x : T { :>> id = \"u\"; }\n}";
        let pkg = parse_src(src);
        assert_eq!(pkg.items.len(), 1);
    }

    #[test]
    fn missing_closing_brace_error() {
        let tokens = tokenize("package P {", "test").expect("lex");
        assert!(parse(tokens, "test").is_err());
    }

    #[test]
    fn ordering_only_marker_in_action_def() {
        let src = r"package P {
            action def D {
                action a;
                action b;
                #OrderingOnly first a then b;
            }
        }";
        let pkg = parse_src(src);
        let Item::ActionDef(adef) = &pkg.items[0] else { panic!() };
        assert_eq!(adef.successions.len(), 1);
        assert_eq!(adef.successions[0].first, "a");
    }
}
