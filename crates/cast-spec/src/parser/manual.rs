//! Schema-side mirror of each macro parser's accepted fields.
//!
//! Each parser module owns a `pub const FIELDS: &[FieldDoc]` listing the
//! names, types, required-ness, and one-line documentation for the
//! fields its `Parse` impl accepts. `cast-watch`'s `manual` query reads
//! these consts to render the live grammar reference, so the manual
//! cannot drift from the parser without someone editing the same file
//! the match arms live in.
//!
//! The match-arm bodies still hold the actual parsing logic (each arm
//! dispatches by type, which can't be table-driven cleanly in `syn`).
//! `FIELDS` is the documentation slice; the match arms are the parsing
//! surface. A unit test in this module asserts every name in `FIELDS`
//! parses for its owning macro, which catches "added FIELDS entry,
//! forgot the match arm" or vice versa.

#[derive(Debug, Clone, Copy)]
pub struct FieldDoc {
    pub name: &'static str,
    pub kind: &'static str,
    pub required: bool,
    pub doc: &'static str,
}

/// Fields accepted by every macro through `CommonFields`. Appended to
/// each macro's `FIELDS` by the manual generator so the help output
/// shows them once per macro without per-parser duplication.
pub const COMMON_FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "tags",
        kind: "list<string>",
        required: false,
        doc: "Group labels. graph_for_tag and `query where:tag` filter on these — \
              a record matches if the requested tag is in the list. \
              Tagging concept + edges + rules with the same tag is the \
              primary cross-file glue. Use `tag: \"x\"` as a one-element \
              alias for `tags: [\"x\"]` (kept for migration; both forms \
              cannot appear in the same macro body).",
    },
    FieldDoc {
        name: "since",
        kind: "string",
        required: false,
        doc: "Free-form 'since when' marker.",
    },
    FieldDoc {
        name: "note",
        kind: "string",
        required: false,
        doc: "Free-form annotation note.",
    },
];

#[cfg(test)]
mod tests {
    //! Two checks per macro:
    //!
    //! 1. A body that exercises every name in `FIELDS` parses successfully —
    //!    catches the case where `FIELDS` lists a name the parser's match
    //!    arms don't accept (would surface as "unknown field for cast::X!").
    //! 2. Every name in `FIELDS` appears in that body — catches the case
    //!    where `FIELDS` grew an entry but the test author forgot to extend
    //!    the exercising body, which would otherwise let drift creep back in.
    //!
    //! What this does NOT catch: the parser accepting a name that `FIELDS`
    //! doesn't list (parser-superset-of-FIELDS). That direction is harder
    //! to assert without source-text inspection; review of the same file
    //! catches it in practice because match arms and `FIELDS` are now
    //! adjacent in every parser module.
    use super::*;
    use crate::parser::{
        anti_pattern, concept, continues_in, gut_check, io_continues_in,
        matrix, pipeline, policy, rule, tier,
    };
    use crate::parser::anti_pattern::AntiPattern;
    use crate::parser::compare::Compare;
    use crate::parser::concept::Concept;
    use crate::parser::continues_in::ContinuesIn;
    use crate::parser::gut_check::GutCheck;
    use crate::parser::io_continues_in::IoContinuesIn;
    use crate::parser::matrix::Matrix;
    use crate::parser::pipeline::Pipeline;
    use crate::parser::policy::Policy;
    use crate::parser::rule::Rule;
    use crate::parser::tier::Tier;
    use syn::parse_str;

    fn assert_fields_covered(body: &str, fields: &[FieldDoc], macro_label: &str) {
        for f in fields {
            assert!(
                body.contains(&format!("{}:", f.name))
                    || body.contains(&format!("{} ", f.name)) // ident-without-colon form for tier `direction` etc.
                    || body.contains(&format!("{},", f.name)),
                "{macro_label}: FIELDS entry `{}` is not exercised in the test body — \
                 extend the body to include this field, or remove the FIELDS entry if \
                 it's stale.",
                f.name
            );
        }
    }

    #[test]
    fn concept_fields_match_parser() {
        let body = r#"
            name: "x",
            summary: "y",
            anchors: [foo::bar],
            parent: "umbrella",
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Concept = parse_str(body).expect("concept body parses");
        let mut all: Vec<FieldDoc> = concept::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::concept");
    }

    #[test]
    fn rule_fields_match_parser() {
        let body = r#"
            rule: "x",
            why: "y",
            governs: [foo::bar],
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Rule = parse_str(body).expect("rule body parses");
        let mut all: Vec<FieldDoc> = rule::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::rule");
    }

    #[test]
    fn anti_pattern_fields_match_parser() {
        let body = r#"
            avoid: "wrong",
            why: "because",
            instead: "right",
            instead_at: foo::right,
            governs: [foo::bar],
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: AntiPattern = parse_str(body).expect("anti_pattern body parses");
        let mut all: Vec<FieldDoc> = anti_pattern::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::anti_pattern");
    }

    #[test]
    fn gut_check_fields_match_parser() {
        let body = r#"
            question: "is it X?",
            yes: "X",
            yes_at: foo::y,
            no: "Y",
            no_at: foo::n,
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: GutCheck = parse_str(body).expect("gut_check body parses");
        let mut all: Vec<FieldDoc> = gut_check::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::gut_check");
    }

    #[test]
    fn pipeline_fields_match_parser() {
        let body = r#"
            stages: { a @ foo::a, b @ foo::b },
            flow: [a -> b],
            cyclic: false,
            entry: a,
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Pipeline = parse_str(body).expect("pipeline body parses");
        let mut all: Vec<FieldDoc> = pipeline::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::pipeline");
    }

    #[test]
    fn tier_fields_match_parser() {
        let body = r#"
            axis: stability,
            direction: increasing,
            of: foo::Stab,
            stages: { weak @ foo::Weak: "w", strong @ foo::Strong: "s" },
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Tier = parse_str(body).expect("tier body parses");
        let mut all: Vec<FieldDoc> = tier::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::tier");
    }

    #[test]
    fn matrix_fields_match_parser() {
        let body = r#"
            columns: [a, b],
            rows: { r1 @ foo::R1: ["1", "2"], r2 @ foo::R2: ["3", "4"] },
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Matrix = parse_str(body).expect("matrix body parses");
        let mut all: Vec<FieldDoc> = matrix::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::matrix");
    }

    #[test]
    fn compare_fields_match_parser() {
        // compare's body is positional `key @ path: "desc"` entries plus
        // common fields — there is no `entries:` keyword in the surface
        // syntax. Skip the field-coverage check for the synthetic
        // `entries` schema entry; it documents the per-line shape.
        let body = r#"
            a @ foo::a: "alpha",
            b @ foo::b: "beta",
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Compare = parse_str(body).expect("compare body parses");
        // Only common fields use the `name: value` form here; verify those.
        assert_fields_covered(body, COMMON_FIELDS, "cast::compare (common fields)");
    }

    #[test]
    fn continues_in_fields_match_parser() {
        let body = r#"
            target: foo::bar,
            concept: "x",
            why: "z",
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: ContinuesIn = parse_str(body).expect("continues_in body parses");
        let mut all: Vec<FieldDoc> = continues_in::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::continues_in");
    }

    #[test]
    fn policy_fields_match_parser() {
        let body = r#"
            layout: sidecar_only,
            inline_in_rust: forbid,
            umbrella_files: ["Cast.cast"],
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: Policy = parse_str(body).expect("policy body parses");
        let mut all: Vec<FieldDoc> = policy::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::policy");
    }

    #[test]
    fn io_continues_in_fields_match_parser() {
        let body = r#"
            target: "samples/external/x.kt",
            lang: kotlin,
            anchor: "Foo",
            concept: "x",
            why: "z",
            tags: ["t"],
            since: "v0.1",
            note: "n"
        "#;
        let _: IoContinuesIn = parse_str(body).expect("io_continues_in body parses");
        let mut all: Vec<FieldDoc> = io_continues_in::FIELDS.to_vec();
        all.extend_from_slice(COMMON_FIELDS);
        assert_fields_covered(body, &all, "cast::io::continues_in");
    }
}
