//! Sprint 3: `PackageRegistry` — cross-package import resolution and semantic validation.
//!
//! Usage:
//! 1. Register every package (schema + tracking) with `registry.register(&pkg)`.
//! 2. Call `registry.validate(&pkg, "filename")` on each tracking package to collect diagnostics.
//!
//! The registry resolves `private import X::*` one level deep (no transitive closure):
//! tracking files explicitly import every namespace they need.

use std::collections::{HashMap, HashSet};

use crate::ast::{Attribute, Item, Package, Value};

// ── diagnostic ────────────────────────────────────────────────────────────────

/// A semantic diagnostic produced during registry validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// File where the problem was found.
    pub file: Box<str>,
    /// 1-indexed source line.
    pub line: u32,
    /// Human-readable description of the problem.
    pub message: Box<str>,
    /// Optional hint for fixing the problem.
    pub suggestion: Option<Box<str>>,
}

// ── per-package export table ───────────────────────────────────────────────────

#[derive(Debug, Default)]
struct PackageExports {
    /// Names introduced by `part def`, `verification def`, `attribute def`, etc.
    type_names: HashSet<String>,
    /// Names and members from `enum def`.
    enum_defs: HashMap<String, HashSet<String>>,
}

// ── registry ──────────────────────────────────────────────────────────────────

/// Holds all registered package exports and validates cross-package references.
#[derive(Debug, Default)]
pub struct PackageRegistry {
    packages: HashMap<String, PackageExports>,
}

impl PackageRegistry {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Register a package's exported names.  Call this for every file (schema + tracking).
    pub fn register(&mut self, pkg: &Package) {
        let exports = self.packages.entry(pkg.name.clone()).or_default();
        for item in &pkg.items {
            match item {
                Item::TypeDef(td) => { exports.type_names.insert(td.name.clone()); }
                Item::EnumDef(ed) => {
                    exports.enum_defs.entry(ed.name.clone())
                        .or_default()
                        .extend(ed.members.iter().cloned());
                }
                // Action defs and instances add names too (e.g. `part def TestResult` in schema)
                Item::ActionDef(adef) => { exports.type_names.insert(adef.name.clone()); }
                _ => {}
            }
        }
    }

    /// Validate cross-package references in `pkg`.  Returns one `Diagnostic` per problem.
    ///
    /// Schema packages should be registered but not validated; call this only for
    /// `.tracking/` files.
    #[must_use]
    pub fn validate(&self, pkg: &Package, file: &str) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        // Collect import namespaces and flag unknown ones.
        let mut namespaces: Vec<&str> = Vec::new();
        for item in &pkg.items {
            if let Item::Import(i) = item {
                let ns = i.namespace.as_str();
                // ScalarValues is a system-provided namespace; skip.
                if ns != "ScalarValues" && !self.packages.contains_key(ns) {
                    diags.push(Diagnostic {
                        file: file.into(),
                        line: i.line,
                        message: format!("unknown import namespace `{ns}`").into(),
                        suggestion: self.suggest_namespace(ns),
                    });
                }
                namespaces.push(ns);
            }
        }

        // Build the effective scope: everything visible through imports.
        let (available_types, available_enums) = self.resolve_scope(&namespaces);

        // Validate package-level and action-def-body items.
        for item in &pkg.items {
            match item {
                Item::Part(p_item) => {
                    Self::check_type_ref(p_item.type_name.as_deref(), p_item.line, file, &available_types, &mut diags);
                    for attr in &p_item.attributes {
                        Self::check_attr(attr, file, &available_enums, &mut diags);
                    }
                }
                Item::Verification(v) => {
                    Self::check_type_ref(v.type_name.as_deref(), v.line, file, &available_types, &mut diags);
                    for attr in &v.attributes {
                        Self::check_attr(attr, file, &available_enums, &mut diags);
                    }
                }
                Item::ActionDef(adef) => {
                    for p_item in &adef.parts {
                        Self::check_type_ref(p_item.type_name.as_deref(), p_item.line, file, &available_types, &mut diags);
                        for attr in &p_item.attributes {
                            Self::check_attr(attr, file, &available_enums, &mut diags);
                        }
                    }
                    for v in &adef.verifications {
                        Self::check_type_ref(v.type_name.as_deref(), v.line, file, &available_types, &mut diags);
                        for attr in &v.attributes {
                            Self::check_attr(attr, file, &available_enums, &mut diags);
                        }
                    }
                }
                _ => {}
            }
        }

        diags
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    fn resolve_scope<'a>(
        &'a self,
        namespaces: &[&str],
    ) -> (HashSet<&'a str>, HashMap<&'a str, &'a HashSet<String>>) {
        let mut types: HashSet<&str> = HashSet::new();
        let mut enums: HashMap<&str, &HashSet<String>> = HashMap::new();
        for ns in namespaces {
            if let Some(exports) = self.packages.get(*ns) {
                for t in &exports.type_names { types.insert(t.as_str()); }
                for (k, v) in &exports.enum_defs { enums.insert(k.as_str(), v); }
            }
        }
        (types, enums)
    }

    fn check_type_ref(
        type_name: Option<&str>,
        line: u32,
        file: &str,
        available: &HashSet<&str>,
        diags: &mut Vec<Diagnostic>,
    ) {
        let Some(t) = type_name else { return };
        if !available.contains(t) {
            diags.push(Diagnostic {
                file: file.into(),
                line,
                message: format!("unresolved type reference `{t}`").into(),
                suggestion: Self::suggest_type(t, available),
            });
        }
    }

    fn check_attr(
        attr: &Attribute,
        file: &str,
        available_enums: &HashMap<&str, &HashSet<String>>,
        diags: &mut Vec<Diagnostic>,
    ) {
        let Value::EnumLit { namespace, member } = &attr.value else { return };
        match available_enums.get(namespace.as_str()) {
            None => diags.push(Diagnostic {
                file: file.into(),
                line: attr.line,
                message: format!(
                    "unresolved enum `{namespace}` in `:>> {name} = …`",
                    name = attr.name
                ).into(),
                suggestion: Self::suggest_enum(namespace, available_enums),
            }),
            Some(members) if !members.contains(member.as_str()) => diags.push(Diagnostic {
                file: file.into(),
                line: attr.line,
                message: format!(
                    "unknown member `{member}` in enum `{namespace}`"
                ).into(),
                suggestion: Some(format!(
                    "valid members: {}",
                    {
                        let mut v: Vec<&str> = members.iter().map(String::as_str).collect();
                        v.sort_unstable();
                        v.join(", ")
                    }
                ).into()),
            }),
            _ => {}
        }
    }

    fn suggest_namespace(&self, query: &str) -> Option<Box<str>> {
        let q = query.to_lowercase();
        let mut hits: Vec<&str> = self.packages.keys()
            .filter(|k| k.to_lowercase().contains(&q))
            .map(String::as_str).collect();
        if hits.is_empty() { return None; }
        hits.sort_unstable();
        Some(format!("known namespaces include: {}", hits.join(", ")).into())
    }

    fn suggest_type(query: &str, available: &HashSet<&str>) -> Option<Box<str>> {
        let q = query.to_lowercase();
        let mut hits: Vec<&&str> = available.iter()
            .filter(|t| t.to_lowercase().contains(&q)).collect();
        if hits.is_empty() { return None; }
        hits.sort_unstable();
        Some(format!("similar types: {}", hits.iter().map(|t| **t).collect::<Vec<_>>().join(", ")).into())
    }

    fn suggest_enum(query: &str, available: &HashMap<&str, &HashSet<String>>) -> Option<Box<str>> {
        let q = query.to_lowercase();
        let mut hits: Vec<&&str> = available.keys()
            .filter(|k| k.to_lowercase().contains(&q)).collect();
        if hits.is_empty() { return None; }
        hits.sort_unstable();
        Some(format!("known enums: {}", hits.iter().map(|k| **k).collect::<Vec<_>>().join(", ")).into())
    }
}
