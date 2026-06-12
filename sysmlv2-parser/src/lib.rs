//! Parser for the engine's `SysML` v2 `.sysml` dialect.
//!
//! Sprint 1 (`rustS1Lexer`) adds the tokenizer.
//! Sprint 2 (`rustS2Parser`) adds the recursive-descent parser and AST.
//! Sprint 3 (`rustS3Semantic`) adds cross-package reference resolution.
//! Sprint 4 (`rustS4SpecCompat`) adds the build-time spec-pin check.
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]
