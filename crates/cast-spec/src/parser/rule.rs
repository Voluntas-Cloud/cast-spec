//! `cast::rule!` — a short imperative architectural rule attached to one
//! or more code paths.
//!
//! ```text
//! cast::rule! {
//!     rule:    "Unify at the task level, ...",
//!     why:     "Mixing storage models ...",
//!     governs: [sample::auth::credential_router, ...],
//! }
//! ```
//!
//! `governs:` is what makes the rule load-bearing: every path must
//! resolve to a real item (validator's job, not the parser's). Here we
//! only enforce the *shape* — at least one path, syntactically valid.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::take::{require, step_separator, take_path_list, take_str};
use syn::{parse::Parse, parse::ParseStream, Ident, Path};

/// Schema-side mirror of the fields accepted by `Rule::parse` below.
/// Read by `cast-watch`'s `manual` query; the match arms in `parse`
/// remain the parsing surface. Keep both in sync — the smoke test in
/// `parser::manual::tests` asserts every name here parses.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "rule",
        kind: "string",
        required: true,
        doc: "The rule itself.",
    },
    FieldDoc {
        name: "why",
        kind: "string",
        required: true,
        doc: "Reason / motivation. Lets a reader judge edge cases when the rule's letter and intent diverge.",
    },
    FieldDoc {
        name: "governs",
        kind: "list<rust_path>",
        required: true,
        doc: "Items the rule applies to.",
    },
];

#[derive(Debug, Clone)]
pub struct Rule {
    pub rule: String,
    pub why: String,
    pub governs: Vec<Path>,
    pub common: CommonFields,
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rule = None;
        let mut why = None;
        let mut governs = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "rule" => take_str(&name, input, &mut rule)?,
                "why" => take_str(&name, input, &mut why)?,
                "governs" => take_path_list(&name, input, &mut governs)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::rule!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let rule = require(rule, "rule")?;
        let why = require(why, "why")?;
        let governs = require(governs, "governs")?;
        if governs.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::rule!: `governs` must contain at least one path",
            ));
        }

        Ok(Rule {
            rule,
            why,
            governs,
            common,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::parse_str;

    #[test]
    fn parses_full_rule() {
        let src = indoc! {r#"
            rule: "Unify at the task level, separate at the secret/lifecycle level",
            why:  "mixing storage models produces silent replay or breakage",
            governs: [
                sample::auth::credential_router,
                sample::auth::vault::derived,
                sample::auth::vault::stored,
            ],
            tags: ["auth_rules"]
        "#};
        let parsed: Rule = parse_str(src).expect("parse");
        assert!(parsed.rule.starts_with("Unify"));
        assert_eq!(parsed.governs.len(), 3);
        assert_eq!(parsed.common.tags.as_slice(), &[String::from("auth_rules")][..]);
    }

    #[test]
    fn rejects_missing_required_field() {
        let src = r#"rule: "x", why: "y""#;
        let err = parse_str::<Rule>(src).unwrap_err();
        assert!(err.to_string().contains("missing required field: governs"));
    }

    #[test]
    fn rejects_empty_governs() {
        let src = r#"rule: "x", why: "y", governs: []"#;
        let err = parse_str::<Rule>(src).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    #[test]
    fn rejects_duplicate_field() {
        let src = r#"rule: "a", rule: "b", why: "y", governs: [foo]"#;
        let err = parse_str::<Rule>(src).unwrap_err();
        assert!(err.to_string().contains("duplicate field: rule"));
    }
}
