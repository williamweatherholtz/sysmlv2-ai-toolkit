//! Parser for the engine's `SysML` v2 `.sysml` dialect.
//!
//! Sprint 1 (`rustS1Lexer`) adds the tokenizer — see [`tokenize`].
//! Sprint 2 (`rustS2Parser`) adds the recursive-descent parser — see [`parse`].
//! Sprint 3 (`rustS3Semantic`) adds cross-package reference resolution — see [`PackageRegistry`].
//! Sprint 4 (`rustS4SpecCompat`) adds the build-time spec-pin check.
#![forbid(unsafe_code)]
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]
// D0074 fail-loud: restriction lints (unwrap_used/expect_used/panic/indexing_slicing) for the
// parser are tracked as M0b (rustFailLoudLints) — the 32 existing call sites are cleaned to
// Result-based errors deliberately, not in the lint-foundation sprint, to avoid destabilizing
// the load-bearing parser.

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
