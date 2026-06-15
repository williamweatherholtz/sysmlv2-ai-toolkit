/// Errors produced by the `SysML` v2 lexer.
#[derive(Debug, thiserror::Error)]
pub enum LexError {
    /// An unexpected character was encountered while scanning.
    #[error("{filename}:{line}:{col}: unexpected character {ch:?}")]
    UnexpectedChar {
        /// The offending character.
        ch: char,
        /// Source file name.
        filename: Box<str>,
        /// 1-indexed line number.
        line: u32,
        /// 1-indexed column number.
        col: u32,
    },

    /// A string literal reached end-of-input without a closing `"`.
    #[error("{filename}:{line}:{col}: unterminated string literal")]
    UnterminatedString {
        /// Source file name.
        filename: Box<str>,
        /// 1-indexed line of the opening quote.
        line: u32,
        /// 1-indexed column of the opening quote.
        col: u32,
    },

    /// An unrecognized escape sequence appeared inside a string literal.
    #[error("{filename}:{line}:{col}: invalid escape sequence '\\\\{seq}'")]
    InvalidEscape {
        /// The character following the backslash.
        seq: char,
        /// Source file name.
        filename: Box<str>,
        /// 1-indexed line number.
        line: u32,
        /// 1-indexed column number.
        col: u32,
    },
}
