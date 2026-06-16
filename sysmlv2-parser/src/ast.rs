//! Abstract syntax tree for the `SysML` v2 engine dialect.
//!
//! The AST is deliberately flat — every node carries its source span so
//! diagnostics can point to the exact location.  Semantic information
//! (package resolution, type checking) lives in the `registry` module (Sprint 3).

use crate::token::Span;

// ── value types used in attribute assignments ──────────────────────────────

/// A scalar value that can appear on the right-hand side of `:>>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// A double-quoted string literal.
    Str(String),
    /// A decimal integer.
    Int(i64),
    /// A bare identifier or keyword used as a value (e.g. `wweatherholtz`).
    Ident(String),
    /// A qualified enum literal: `Namespace::Member`.
    EnumLit {
        /// The namespace part (e.g. `VerdictKind`).
        namespace: String,
        /// The member part (e.g. `pass`).
        member: String,
    },
}

// ── per-item attribute assignment ──────────────────────────────────────────

/// A single `:>> name = value` assignment inside an item body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    /// Attribute name (e.g. `id`, `title`, `outcome`).
    pub name: String,
    /// Assigned value.
    pub value: Value,
    /// Source span of the whole `:>> name = value ;` assignment.
    pub span: Span,
    /// 1-indexed source line of the `:>>` token.
    pub line: u32,
}

// ── top-level items ────────────────────────────────────────────────────────

/// A `part name : Type { ... }` item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    /// Declared name.
    pub name: String,
    /// Optional `: Type` annotation.
    pub type_name: Option<String>,
    /// Attribute assignments in the body.
    pub attributes: Vec<Attribute>,
    pub span: Span,
    /// 1-indexed source line of the `part` keyword.
    pub line: u32,
}

/// A `verification name : Type { ... }` item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verification {
    /// Declared name.
    pub name: String,
    /// Optional `: Type` annotation.
    pub type_name: Option<String>,
    /// Attribute assignments in the body.
    pub attributes: Vec<Attribute>,
    pub span: Span,
    /// 1-indexed source line of the `verification` keyword.
    pub line: u32,
}

/// An `action name;` bare declaration (no body).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDecl {
    pub name: String,
    pub span: Span,
}

/// A `first A then B;` succession edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succession {
    pub first: String,
    pub then: String,
    /// `true` when prefixed with `#OrderingOnly` — edge orders execution but does not
    /// create a semantic dependency for suspect-propagation purposes.
    pub is_ordering_only: bool,
    pub span: Span,
}

/// A `satisfy needName by srName;` edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfyEdge {
    pub need: String,
    pub by: String,
    pub span: Span,
}

/// An `allocate srName to componentName;` edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocateEdge {
    pub sr: String,
    pub to: String,
    pub span: Span,
}

/// A `#Marker dependency from A to B;` dependency annotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyAnnotation {
    /// The marker name (e.g. `DependsOn`, `OrderingOnly`).
    pub marker: String,
    pub from: String,
    pub to: String,
    pub span: Span,
}

/// `private import Namespace::*;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub namespace: String,
    pub span: Span,
    /// 1-indexed source line of the `private` keyword.
    pub line: u32,
}

/// An `action def Name { ... }` block containing a task graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDef {
    pub name: String,
    pub actions: Vec<ActionDecl>,
    pub parts: Vec<Part>,
    pub verifications: Vec<Verification>,
    pub successions: Vec<Succession>,
    pub span: Span,
}

/// An `enum def Name { member1; member2; ... }` type definition.
/// Members are extracted for enum-literal validation (Sprint 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDef {
    pub name: String,
    pub members: Vec<String>,
    pub span: Span,
    pub line: u32,
}

/// A named type definition (`part def`, `verification def`, `attribute def`, etc.).
/// Only the name is captured; the body is skipped for tolerant parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDef {
    pub name: String,
    pub span: Span,
    pub line: u32,
}

/// Any top-level item inside a package body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Import(Import),
    ActionDef(ActionDef),
    Part(Part),
    Verification(Verification),
    ActionDecl(ActionDecl),
    Succession(Succession),
    Satisfy(SatisfyEdge),
    Allocate(AllocateEdge),
    Dependency(DependencyAnnotation),
    /// Named type definition (`part def`, `verification def`, `attribute def`, …).
    TypeDef(TypeDef),
    /// Enum type definition with extracted members.
    EnumDef(EnumDef),
}

/// A `package Name { ... }` — the root of a `.sysml` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub items: Vec<Item>,
    pub span: Span,
}
