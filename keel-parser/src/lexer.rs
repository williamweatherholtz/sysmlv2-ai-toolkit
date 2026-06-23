use crate::error::LexError;
use crate::token::{Span, Token, TokenKind};

struct Lexer<'src> {
    filename: Box<str>,
    chars: std::iter::Peekable<std::str::CharIndices<'src>>,
    pos: usize,
    line: u32,
    col: u32,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str, filename: &str) -> Self {
        Self {
            filename: filename.into(),
            chars: source.char_indices().peekable(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Consume the next character and return `(byte_pos, ch, line, col)` — the
    /// location fields are the position of the consumed character (before advance).
    fn next_char(&mut self) -> Option<(usize, char, u32, u32)> {
        let line = self.line;
        let col = self.col;
        let (pos, ch) = self.chars.next()?;
        self.pos = pos + ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some((pos, ch, line, col))
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, ch)| *ch)
    }

    /// Skip whitespace and `//` line comments; return the first non-trivia
    /// character with its source location.
    fn skip_trivia(&mut self) -> Option<(usize, char, u32, u32)> {
        loop {
            let (pos, ch, line, col) = self.next_char()?;
            if ch.is_whitespace() {
                continue;
            }
            if ch == '/' {
                if self.peek_char() == Some('/') {
                    while self.peek_char().is_some_and(|c| c != '\n') {
                        self.next_char();
                    }
                    continue;
                }
                if self.peek_char() == Some('*') {
                    self.next_char(); // consume '*'
                    loop {
                        match self.next_char() {
                            None => break,
                            Some((_, '*', _, _)) if self.peek_char() == Some('/') => {
                                self.next_char(); // consume '/'
                                break;
                            }
                            _ => {}
                        }
                    }
                    continue;
                }
            }
            return Some((pos, ch, line, col));
        }
    }

    fn scan_colon(&mut self) -> TokenKind {
        if self.peek_char() == Some(':') {
            self.next_char();
            TokenKind::ColonColon
        } else if self.peek_char() == Some('>') {
            self.next_char();
            if self.peek_char() == Some('>') {
                self.next_char();
                TokenKind::ColonGtGt
            } else {
                TokenKind::ColonGt
            }
        } else {
            TokenKind::Colon
        }
    }

    fn lex_string(&mut self, start: usize, line: u32, col: u32) -> Result<Token, LexError> {
        let mut value = String::new();
        loop {
            match self.next_char() {
                None => {
                    return Err(LexError::UnterminatedString {
                        filename: self.filename.clone(),
                        line,
                        col,
                    });
                }
                Some((_, '"', _, _)) => break,
                Some((_, '\\', esc_line, esc_col)) => match self.next_char() {
                    Some((_, 'n', _, _)) => value.push('\n'),
                    Some((_, 't', _, _)) => value.push('\t'),
                    Some((_, '"', _, _)) => value.push('"'),
                    Some((_, '\\', _, _)) => value.push('\\'),
                    Some((_, seq, _, _)) => {
                        return Err(LexError::InvalidEscape {
                            seq,
                            filename: self.filename.clone(),
                            line: esc_line,
                            col: esc_col + 1,
                        });
                    }
                    None => {
                        return Err(LexError::UnterminatedString {
                            filename: self.filename.clone(),
                            line,
                            col,
                        });
                    }
                },
                Some((_, ch, _, _)) => value.push(ch),
            }
        }
        Ok(Token {
            kind: TokenKind::Str(value),
            span: Span { start, end: self.pos },
            line,
            col,
        })
    }

    fn lex_number(&mut self, first: char, start: usize, line: u32, col: u32) -> Token {
        let mut digits = String::from(first);
        loop {
            match self.peek_char() {
                Some(ch) if ch.is_ascii_digit() => {
                    self.next_char();
                    digits.push(ch);
                }
                _ => break,
            }
        }
        Token {
            kind: TokenKind::Int(digits.parse().unwrap_or(i64::MAX)),
            span: Span { start, end: self.pos },
            line,
            col,
        }
    }

    fn lex_ident(&mut self, first: char, start: usize, line: u32, col: u32) -> Token {
        let mut s = String::from(first);
        loop {
            match self.peek_char() {
                Some(ch) if ch.is_alphanumeric() || ch == '_' => {
                    self.next_char();
                    s.push(ch);
                }
                _ => break,
            }
        }
        Token {
            kind: keyword_or_ident(&s),
            span: Span { start, end: self.pos },
            line,
            col,
        }
    }

    fn scan_token(
        &mut self,
        ch: char,
        byte_pos: usize,
        line: u32,
        col: u32,
    ) -> Result<Token, LexError> {
        let kind = match ch {
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Dot,
            '*' => TokenKind::Star,
            '#' => TokenKind::Hash,
            '=' => TokenKind::Eq,
            '+' => TokenKind::Plus,
            '/' => TokenKind::Slash,
            ':' => self.scan_colon(),
            '"' => return self.lex_string(byte_pos, line, col),
            c if c.is_ascii_digit() => return Ok(self.lex_number(c, byte_pos, line, col)),
            c if c.is_alphabetic() || c == '_' => {
                return Ok(self.lex_ident(c, byte_pos, line, col));
            }
            c => {
                return Err(LexError::UnexpectedChar {
                    ch: c,
                    filename: self.filename.clone(),
                    line,
                    col,
                });
            }
        };
        Ok(Token {
            kind,
            span: Span { start: byte_pos, end: self.pos },
            line,
            col,
        })
    }

    fn run(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let Some((byte_pos, ch, line, col)) = self.skip_trivia() else {
                tokens.push(Token::eof(self.pos, self.line, self.col));
                break;
            };
            tokens.push(self.scan_token(ch, byte_pos, line, col)?);
        }
        Ok(tokens)
    }
}

fn keyword_or_ident(s: &str) -> TokenKind {
    match s {
        "package" => TokenKind::Package,
        "private" => TokenKind::Private,
        "import" => TokenKind::Import,
        "part" => TokenKind::Part,
        "action" => TokenKind::Action,
        "def" => TokenKind::Def,
        "verification" => TokenKind::Verification,
        "requirement" => TokenKind::Requirement,
        "use" => TokenKind::Use,
        "case" => TokenKind::Case,
        "attribute" => TokenKind::Attribute,
        "enum" => TokenKind::Enum,
        "abstract" => TokenKind::Abstract,
        "first" => TokenKind::First,
        "then" => TokenKind::Then,
        "satisfy" => TokenKind::Satisfy,
        "allocate" => TokenKind::Allocate,
        "by" => TokenKind::By,
        "to" => TokenKind::To,
        "from" => TokenKind::From,
        "dependency" => TokenKind::Dependency,
        "doc" => TokenKind::Doc,
        _ => TokenKind::Ident(s.to_owned()),
    }
}

/// Lex all tokens from `source`, appending a terminal [`TokenKind::Eof`].
///
/// # Errors
///
/// Returns [`LexError`] on an unexpected character, unterminated string
/// literal, or unrecognized escape sequence.
pub fn tokenize(source: &str, filename: &str) -> Result<Vec<Token>, LexError> {
    Lexer::new(source, filename).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        tokenize(src, "test")
            .expect("lexer error")
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn empty_source_gives_eof() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn keywords_round_trip() {
        let cases = [
            ("package", TokenKind::Package),
            ("private", TokenKind::Private),
            ("import", TokenKind::Import),
            ("part", TokenKind::Part),
            ("action", TokenKind::Action),
            ("def", TokenKind::Def),
            ("verification", TokenKind::Verification),
            ("requirement", TokenKind::Requirement),
            ("use", TokenKind::Use),
            ("case", TokenKind::Case),
            ("attribute", TokenKind::Attribute),
            ("enum", TokenKind::Enum),
            ("abstract", TokenKind::Abstract),
            ("first", TokenKind::First),
            ("then", TokenKind::Then),
            ("satisfy", TokenKind::Satisfy),
            ("allocate", TokenKind::Allocate),
            ("by", TokenKind::By),
            ("to", TokenKind::To),
            ("from", TokenKind::From),
            ("dependency", TokenKind::Dependency),
        ];
        for (src, expected) in cases {
            let got = kinds(src);
            assert_eq!(got[0], expected, "keyword {src:?}");
        }
    }

    #[test]
    fn operators_lex_correctly() {
        assert_eq!(kinds("::")[0], TokenKind::ColonColon);
        assert_eq!(kinds(":>>")[0], TokenKind::ColonGtGt);
        assert_eq!(kinds(":>")[0], TokenKind::ColonGt);
        assert_eq!(kinds(":")[0], TokenKind::Colon);
        assert_eq!(kinds("{")[0], TokenKind::LBrace);
        assert_eq!(kinds("}")[0], TokenKind::RBrace);
        assert_eq!(kinds(";")[0], TokenKind::Semicolon);
        assert_eq!(kinds("*")[0], TokenKind::Star);
        assert_eq!(kinds("#")[0], TokenKind::Hash);
        assert_eq!(kinds("=")[0], TokenKind::Eq);
    }

    #[test]
    fn string_literal_basic() {
        assert_eq!(kinds(r#""hello""#)[0], TokenKind::Str("hello".into()));
    }

    #[test]
    fn string_literal_escape_sequences() {
        assert_eq!(
            kinds(r#""\n\t\"\\""#)[0],
            TokenKind::Str("\n\t\"\\".into())
        );
    }

    #[test]
    fn integer_literal() {
        assert_eq!(kinds("42")[0], TokenKind::Int(42));
    }

    #[test]
    fn identifier_not_keyword() {
        assert_eq!(
            kinds("myIdent")[0],
            TokenKind::Ident("myIdent".into())
        );
    }

    #[test]
    fn line_comment_skipped() {
        assert_eq!(kinds("// skip me\npackage")[0], TokenKind::Package);
    }

    #[test]
    fn import_statement_sequence() {
        let got = kinds("private import Foo::*;");
        assert_eq!(
            got,
            vec![
                TokenKind::Private,
                TokenKind::Import,
                TokenKind::Ident("Foo".into()),
                TokenKind::ColonColon,
                TokenKind::Star,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn colon_gt_gt_in_assignment() {
        let got = kinds(":>> id = \"abc\";");
        assert_eq!(
            got,
            vec![
                TokenKind::ColonGtGt,
                TokenKind::Ident("id".into()),
                TokenKind::Eq,
                TokenKind::Str("abc".into()),
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn enum_literal_pattern() {
        let got = kinds("VerdictKind::pass");
        assert_eq!(
            got,
            vec![
                TokenKind::Ident("VerdictKind".into()),
                TokenKind::ColonColon,
                TokenKind::Ident("pass".into()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn unexpected_char_returns_error() {
        assert!(tokenize("@", "test").is_err());
    }

    #[test]
    fn unterminated_string_returns_error() {
        assert!(tokenize("\"oops", "test").is_err());
    }

    #[test]
    fn invalid_escape_returns_error() {
        assert!(tokenize(r#""\q""#, "test").is_err());
    }

    #[test]
    fn location_tracking() {
        let tokens = tokenize("package\nFoo", "test").expect("lex");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].col, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[1].col, 1);
    }
}
