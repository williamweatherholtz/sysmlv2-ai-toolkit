//! Parser for the engine's `SysML` v2 `.sysml` dialect.
//!
//! Sprint 1 (`rustS1Lexer`) adds the tokenizer — see [`tokenize`].
//! Sprint 2 (`rustS2Parser`) adds the recursive-descent parser — see [`parse`].
//! Sprint 3 (`rustS3Semantic`) adds cross-package reference resolution — see [`PackageRegistry`].
//! Sprint 4 (`rustS4SpecCompat`) adds the build-time spec-pin check.
#![forbid(unsafe_code)]
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]
// D0074 fail-loud (M0b): production parser code is already unwrap/expect/panic/index-free — the
// parser's own `p.expect(&TokenKind, ..)?` is a fallible Result method, not Option::expect. Tests
// may use them freely.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing))]

pub mod ast;
pub mod error;
pub mod registry;
pub mod spec_compat;
pub mod token;
mod lexer;
mod parser;

pub use error::{LexError, ParseError};
pub use lexer::tokenize;
pub use parser::parse;
pub use registry::{Diagnostic, PackageRegistry};
