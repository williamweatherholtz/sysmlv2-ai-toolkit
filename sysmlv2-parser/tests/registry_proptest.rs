use proptest::prelude::*;
use sysmlv2_parser::{parse, tokenize, PackageRegistry};

fn parse_src(src: &str) -> Option<sysmlv2_parser::ast::Package> {
    let tokens = tokenize(src, "prop-reg").ok()?;
    parse(tokens, "prop-reg").ok()
}

proptest! {
    /// Registering two packages with arbitrary names never panics.
    #[test]
    fn prop_registry_merge(
        name1 in "[A-Z][a-zA-Z]{0,8}",
        type1 in "[A-Z][a-zA-Z]{0,8}",
        name2 in "[A-Z][a-zA-Z]{0,8}",
        type2 in "[A-Z][a-zA-Z]{0,8}",
    ) {
        let mut reg = PackageRegistry::new();
        if let Some(pkg) = parse_src(&format!("package {name1} {{ part def {type1}; }}")) {
            reg.register(&pkg);
        }
        if let Some(pkg) = parse_src(&format!("package {name2} {{ part def {type2}; }}")) {
            reg.register(&pkg);
        }
        // No panic is the invariant.
        prop_assert!(true);
    }

    /// A type defined in a registered package resolves with zero diagnostics.
    #[test]
    fn prop_known_type_validates_clean(
        ns in "[A-Z][a-zA-Z]{0,8}",
        type_name in "[A-Z][a-zA-Z]{0,8}",
        inst in "[a-z][a-zA-Z]{0,8}",
    ) {
        let schema_src = format!("package {ns} {{ part def {type_name}; }}");
        let usage_src  = format!("package P {{ private import {ns}::*; part {inst} : {type_name} {{}} }}");

        let Some(schema_pkg) = parse_src(&schema_src) else { return Ok(()); };
        let Some(usage_pkg)  = parse_src(&usage_src)  else { return Ok(()); };

        let mut reg = PackageRegistry::new();
        reg.register(&schema_pkg);
        let diags = reg.validate(&usage_pkg, "test.sysml");
        prop_assert!(
            diags.is_empty(),
            "expected no diagnostics for known type `{type_name}`, got: {diags:#?}"
        );
    }

    /// A type NOT defined in any registered package produces a diagnostic.
    #[test]
    fn prop_unknown_type_produces_diagnostic(
        ns in "[A-Z][a-zA-Z]{0,8}",
        known in "[A-Z][a-zA-Z]{0,8}",
        unknown in "[A-Z][a-zA-Z0-9]{0,8}X", // trailing X ensures different from known
        inst in "[a-z][a-zA-Z]{0,8}",
    ) {
        prop_assume!(known != unknown);
        let schema_src = format!("package {ns} {{ part def {known}; }}");
        let usage_src  = format!("package P {{ private import {ns}::*; part {inst} : {unknown} {{}} }}");

        let Some(schema_pkg) = parse_src(&schema_src) else { return Ok(()); };
        let Some(usage_pkg)  = parse_src(&usage_src)  else { return Ok(()); };

        let mut reg = PackageRegistry::new();
        reg.register(&schema_pkg);
        let diags = reg.validate(&usage_pkg, "test.sysml");
        prop_assert!(
            !diags.is_empty(),
            "expected a diagnostic for unknown type `{unknown}`, got none"
        );
    }

    /// A valid enum literal (registered enum + known member) produces no diagnostics.
    #[test]
    fn prop_valid_enum_lit_passes(
        ns   in "[A-Z][a-zA-Z]{0,8}",
        enum_name in "[A-Z][a-zA-Z]{0,8}",
        member in "[a-z][a-zA-Z]{0,8}",
        attr in "[a-z][a-zA-Z]{0,8}",
    ) {
        let schema_src = format!("package {ns} {{ enum def {enum_name} {{ {member}; }} }}");
        let usage_src  = format!("package P {{ private import {ns}::*; part x {{ :>> {attr} = {enum_name}::{member}; }} }}");

        let Some(schema_pkg) = parse_src(&schema_src) else { return Ok(()); };
        let Some(usage_pkg)  = parse_src(&usage_src)  else { return Ok(()); };

        let mut reg = PackageRegistry::new();
        reg.register(&schema_pkg);
        let diags = reg.validate(&usage_pkg, "test.sysml");
        prop_assert!(
            diags.is_empty(),
            "expected no diagnostics for valid enum lit `{enum_name}::{member}`, got: {diags:#?}"
        );
    }
}
