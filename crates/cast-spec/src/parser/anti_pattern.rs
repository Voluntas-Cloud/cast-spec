//! `cast::anti_pattern!` — "don't do it this way, because…" plus the
//! recommended alternative and the regions of code where the temptation
//! lives.
//!
//! ```text
//! cast::anti_pattern! {
//!     avoid:      "if (x && y && z) then do_thing",
//!     why:        "spaghetti decisions are unmaintainable",
//!     instead:    "inputs -> evaluation -> ranked options -> chosen action",
//!     instead_at: sample::decision::evaluate,
//!     governs:    [sample::decision],
//! }
//! ```
//!
//! `instead_at:` is optional but recommended: when present it pins the
//! prose alternative to a concrete implementation in the codebase.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::take::{require, step_separator, take_path, take_path_list, take_str};
use syn::{parse::Parse, parse::ParseStream, Ident, Path};

/// Schema-side mirror of the fields accepted by `AntiPattern::parse`.
/// Kept colocated with the match arms so future drift becomes a same-
/// file diff.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "avoid",
        kind: "string",
        required: true,
        doc: "Description of the wrong move.",
    },
    FieldDoc {
        name: "why",
        kind: "string",
        required: true,
        doc: "Why the wrong move is tempting and what it costs.",
    },
    FieldDoc {
        name: "instead",
        kind: "string",
        required: true,
        doc: "Prose description of the right shape.",
    },
    FieldDoc {
        name: "instead_at",
        kind: "rust_path",
        required: false,
        doc: "Optional pointer to the right shape in the codebase.",
    },
    FieldDoc {
        name: "governs",
        kind: "list<rust_path>",
        required: true,
        doc: "Items where the anti-pattern would be tempting / has been seen.",
    },
];

#[derive(Debug, Clone)]
pub struct AntiPattern {
    pub avoid: String,
    pub why: String,
    pub instead: String,
    pub instead_at: Option<Path>,
    pub governs: Vec<Path>,
    pub common: CommonFields,
}

impl Parse for AntiPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut avoid = None;
        let mut why = None;
        let mut instead = None;
        let mut instead_at = None;
        let mut governs = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "avoid" => take_str(&name, input, &mut avoid)?,
                "why" => take_str(&name, input, &mut why)?,
                "instead" => take_str(&name, input, &mut instead)?,
                "instead_at" => take_path(&name, input, &mut instead_at)?,
                "governs" => take_path_list(&name, input, &mut governs)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::anti_pattern!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let avoid = require(avoid, "avoid")?;
        let why = require(why, "why")?;
        let instead = require(instead, "instead")?;
        let governs = require(governs, "governs")?;
        if governs.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::anti_pattern!: `governs` must contain at least one path",
            ));
        }

        Ok(AntiPattern {
            avoid,
            why,
            instead,
            instead_at,
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
    fn parses_full_anti_pattern() {
        let src = indoc! {r#"
            avoid:      "if (x && y && z) then do_thing",
            why:        "spaghetti decisions are unmaintainable and unauditable",
            instead:    "inputs -> evaluation -> ranked options -> chosen action",
            instead_at: sample::decision::evaluate,
            governs:    [sample::decision]
        "#};
        let parsed: AntiPattern = parse_str(src).expect("parse");
        assert_eq!(parsed.governs.len(), 1);
        assert!(parsed.instead_at.is_some());
    }

    #[test]
    fn instead_at_is_optional() {
        let src = indoc! {r#"
            avoid: "x",
            why: "y",
            instead: "z",
            governs: [foo::bar]
        "#};
        let parsed: AntiPattern = parse_str(src).expect("parse");
        assert!(parsed.instead_at.is_none());
    }

    #[test]
    fn rejects_missing_instead() {
        let src = r#"avoid: "x", why: "y", governs: [foo]"#;
        let err = parse_str::<AntiPattern>(src).unwrap_err();
        assert!(err.to_string().contains("missing required field: instead"));
    }
}
