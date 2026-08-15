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
    /// A parenthesised sequence: `:>> phases = (a, b, c)` — the multi-valued feature assignment.
    ///
    /// Valid `SysML` v2 (kernel-confirmed) that this parser previously rejected outright, which removed
    /// `ref` features from consideration for any one-to-many edge and left metadata markers looking
    /// like the only option (issue095). Elements are themselves [`Value`]s, so a sequence of enum
    /// literals or strings parses as naturally as a sequence of element references — nesting is
    /// permitted by the grammar but has no meaning in this schema, so consumers flatten or reject it
    /// explicitly rather than silently.
    Seq(Vec<Self>),
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
    /// Member features declared in the body — `assert constraint`, `ref`, `port` (issue102).
    /// Stored on every usage kind rather than only where currently needed: dropping them on the other
    /// two would make those members silently unread again, which is the defect this whole sequence fixes.
    pub members: Vec<MemberFeature>,
    /// Metadata marker applied as a `#Marker` prefix on the part (D0070, e.g. a process-change
    /// Decision's `ProspectiveChange`). `None` if unmarked. Retained for views (M2.0).
    pub marker: Option<String>,
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
    /// Member features declared in the body — `assert constraint`, `ref`, `port` (issue102).
    /// Stored on every usage kind rather than only where currently needed: dropping them on the other
    /// two would make those members silently unread again, which is the defect this whole sequence fixes.
    pub members: Vec<MemberFeature>,
    pub span: Span,
    /// 1-indexed source line of the `verification` keyword.
    pub line: u32,
}

/// A `use case name : Type { ... }` usage (issue102 construct 2/6).
///
/// Its own variant rather than reusing [`Part`]: a use case is not a structural part, and the whole
/// point of the base-first programme is that the model should say what it means. Following the
/// precedent already set by [`Verification`], which exists for exactly this reason. 56 of these were
/// skipped entirely — only `use case def` was handled — so every use case in the model was invisible to
/// type resolution and attribute validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseUsage {
    /// Declared name.
    pub name: String,
    /// Optional `: Type` annotation.
    pub type_name: Option<String>,
    /// Attribute assignments in the body.
    pub attributes: Vec<Attribute>,
    /// Member features declared in the body — `assert constraint`, `ref`, `port` (issue102).
    /// Stored on every usage kind rather than only where currently needed: dropping them on the other
    /// two would make those members silently unread again, which is the defect this whole sequence fixes.
    pub members: Vec<MemberFeature>,
    pub span: Span,
    /// 1-indexed source line of the `use` keyword.
    pub line: u32,
}

/// An `action name : Type { ... }` typed action USAGE (D0143).
///
/// Its own variant rather than reusing [`Part`]: the whole point of retyping `Process`/`ProcessStep`
/// from `part def` to `action def` is that a procedure is BEHAVIOUR, and modelling the usage as a part
/// internally would reintroduce the confusion the retype removes. Kernel-verified shape:
/// `action deploy : Proc { :>> title = "x"; assert constraint c : Ok; }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionUsage {
    /// Declared name.
    pub name: String,
    /// Optional `: Type` annotation.
    pub type_name: Option<String>,
    /// Attribute assignments in the body.
    pub attributes: Vec<Attribute>,
    /// Member features — notably `assert constraint`, which D0141 attaches to process parts and which
    /// must survive the retype.
    pub members: Vec<MemberFeature>,
    pub span: Span,
    /// 1-indexed source line of the `action` keyword.
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

/// A `flow from A.out to B.in;` item flow (issue102 construct 6/6).
///
/// All 18 in the model sit inside `action def` workflow bodies and were skipped, so the workflows
/// declared their stage-to-stage item flow and nothing could read it — the one place the repo already
/// modelled behaviour in base `SysML` v2 rather than in strings was invisible.
///
/// Endpoints are stored as their FULL dotted text (`dataArch.sysReq`). Resolving a dotted feature path
/// needs feature-level resolution the registry does not have; storing the text keeps the fact intact
/// and lets consumers take the root, which IS a known item. Storing only the root would discard the
/// port and could not be recovered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowEdge {
    /// Source endpoint as written, e.g. `brief.o`.
    pub from: String,
    /// Target endpoint as written, e.g. `personas.i`.
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
    /// The `:>` specialization target, if declared (D0143). An `action def` may specialize an abstract
    /// action base exactly as a `part def` may — needed once `Process :> TrackedAction` exists, and
    /// captured rather than discarded for the same reason as [`TypeDef::specializes`]: an unread
    /// hierarchy is an unchecked one.
    pub specializes: Option<String>,
    pub actions: Vec<ActionDecl>,
    pub parts: Vec<Part>,
    pub verifications: Vec<Verification>,
    pub successions: Vec<Succession>,
    /// `flow from A.x to B.y;` edges declared in the body (issue102).
    pub flows: Vec<FlowEdge>,
    /// Member features declared in the body — `attribute`, `ref`, `assert constraint` (D0143). An
    /// `action def` carries these exactly as a `part def` does, and retyping Process/ProcessStep made
    /// that immediately load-bearing: without this their attributes would be unread.
    pub members: Vec<MemberFeature>,
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

/// A member feature declared inside a type definition body (issue102 constructs 3-5).
///
/// `attribute id : String;` · `ref states : WorkflowState[*];` · `port p : Pt;` ·
/// `assert constraint c : Ok;` · `require constraint c : Ok;`
///
/// The body used to be discarded wholesale, so 149 member declarations in the engine's own schema
/// carried type references that nothing resolved. `ref` and `assert constraint` are the two the
/// base-first programme depends on: `ref` is the base construct that replaces a marker for a
/// one-to-many edge, and `assert constraint` is where D0139(D) puts process-to-controls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberFeature {
    /// Declaring keyword as written: `attribute`, `ref`, `port`, `assert`, `require`, or empty when
    /// the member is an untyped usage.
    pub kind: String,
    /// Member name.
    pub name: String,
    /// The `: Type` annotation, if present.
    pub type_name: Option<String>,
    /// 1-indexed source line.
    pub line: u32,
}

/// A named type definition (`part def`, `verification def`, `attribute def`, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDef {
    pub name: String,
    /// Member features declared in the body (issue102). Empty when the definition has no body.
    pub members: Vec<MemberFeature>,
    /// The `:>` specialization target, if declared (issue102/D0140).
    ///
    /// Previously skipped along with the rest of the body, so the TYPE HIERARCHY was unread: 52 such
    /// clauses exist in the engine's own schema and none was visible to any guard or view. That matters
    /// beyond tidiness because `:>` is the derivation form D0140 names as the only valid replacement for
    /// `#DerivedFrom` — a migration onto a relationship the parser cannot read would have lost every
    /// edge silently.
    ///
    /// Only the FIRST target is captured. `SysML` v2 permits multiple, but nothing in this schema
    /// declares more than one, and inventing multi-supertype support with no instance to test it against
    /// is how unused schema accumulates (the `OrderingRule` precedent, rules.sysml:63-66).
    pub specializes: Option<String>,
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
    /// `use case name : Type { ... }` usage.
    UseCase(UseCaseUsage),
    /// `action name : Type { ... }` typed usage (D0143).
    ActionUsage(ActionUsage),
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
    /// Statements the parser did not understand and skipped (issue102).
    ///
    /// The parser recognises a fixed set of statements and `skip_item`s everything else, which was
    /// SILENT: `ref e : C;`, `port p : Pt;`, `assert constraint c : Ok;` and `connect x.p to y.p;` all
    /// parse "clean" and resolve nothing. Measured with an undeclared target, each one validates clean
    /// while the control (`part x : NoSuchType`) correctly produces a diagnostic — so the content is not
    /// merely unresolved, it is invisible.
    ///
    /// That matters because those are precisely the BASE `SysML` v2 constructs the base-first programme
    /// (D0139) converts toward. A conversion landing before the reader would make ~1,500 edges parse
    /// clean and vanish while every guard reported green. Recording what was skipped turns that from
    /// silence into a reported count — the same fix issue027 applied to items dropped OUTSIDE the
    /// package, one level in.
    pub skipped: Vec<SkippedStatement>,
}

/// One statement the parser skipped, with enough detail to find and judge it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedStatement {
    /// 1-indexed source line of the statement's first token.
    pub line: u32,
    /// The leading token, as source text (e.g. `ref`, `port`, `connect`, `assert`).
    pub lead: String,
}
