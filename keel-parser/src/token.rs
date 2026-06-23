/// Byte-offset span of a token in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start byte (inclusive).
    pub start: usize,
    /// End byte (exclusive).
    pub end: usize,
}

/// The discriminant of a `SysML` v2 engine-dialect token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // ── keywords ──────────────────────────────────────────────────────────
    /// `package`
    Package,
    /// `private`
    Private,
    /// `import`
    Import,
    /// `part`
    Part,
    /// `action`
    Action,
    /// `def`
    Def,
    /// `verification`
    Verification,
    /// `requirement`
    Requirement,
    /// `use`
    Use,
    /// `case`
    Case,
    /// `attribute`
    Attribute,
    /// `enum`
    Enum,
    /// `abstract`
    Abstract,
    /// `first`
    First,
    /// `then`
    Then,
    /// `satisfy`
    Satisfy,
    /// `allocate`
    Allocate,
    /// `by`
    By,
    /// `to`
    To,
    /// `from`
    From,
    /// `dependency`
    Dependency,

    /// `doc`
    Doc,

    // ── punctuation & operators ────────────────────────────────────────────
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `;`
    Semicolon,
    /// `.`
    Dot,
    /// `*`
    Star,
    /// `#`
    Hash,
    /// `=`
    Eq,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `::`
    ColonColon,
    /// `:>>`
    ColonGtGt,
    /// `:>`
    ColonGt,
    /// `:`
    Colon,

    // ── literals ──────────────────────────────────────────────────────────
    /// A double-quoted string literal with escape sequences resolved.
    Str(String),
    /// A decimal integer literal.
    Int(i64),
    /// An identifier (non-keyword alphanumeric word).
    Ident(String),

    // ── sentinel ──────────────────────────────────────────────────────────
    /// End of input.
    Eof,
}

/// A single lexed token with location information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// Token kind and payload.
    pub kind: TokenKind,
    /// Byte span in the source text.
    pub span: Span,
    /// 1-indexed source line.
    pub line: u32,
    /// 1-indexed source column (of the first character of the token).
    pub col: u32,
}

impl Token {
    /// Constructs an EOF sentinel at the given byte offset and location.
    #[must_use]
    pub const fn eof(pos: usize, line: u32, col: u32) -> Self {
        Self {
            kind: TokenKind::Eof,
            span: Span { start: pos, end: pos },
            line,
            col,
        }
    }
}
